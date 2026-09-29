---
id: TC-196
title: "Fail the spec-artifact ID and relocation-map check on each seeded defect"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-274
    type: verifies
---
# TC-196: Fail the spec-artifact ID and relocation-map check on each seeded defect

## Description

Check that the repository-local identifier and relocation-map check of
[FR-274](../functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md) fails on each seeded
defect, names every `path:line` involved, passes each seeded near-miss, passes a clean tree with one
summary line, and fails `make spec` whenever it fails. Each step runs the check against a scratch
Git repository built by the test, never against this repository's own `spec/` tree.

## Test Procedure

1. Build a clean scratch tree: a master-requirements document with an `## ID Blocks` table, two
   FRs, one `TC` artifact file, and one `TestMatrix` whose `Test Case Summary` lists that `TC`.
   Run the check; expect exit 0, exactly one line on standard output and nothing on standard error
   (FR-274-AC-19).
2. Give a second FR the first FR's frontmatter `id`; expect a non-zero exit naming the identifier
   and both `path:line` locations (FR-274-AC-1).
3. Name the first FR's identifier only in another artifact's body prose; expect exit 0
   (FR-274-AC-2).
4. Declare one `id` in two files under `plan/`, in two under `planning/`, in two under `reviews/`,
   in two under `spec/reviews/`, and in one `reviews/` file plus one spec artifact; expect exit 0 in
   each case (FR-274-AC-3).
5. Repeat one `TC` ID in two `Test Case Summary` rows of one matrix, then in one row each of two
   matrices; expect a non-zero exit naming the identifier and each `matrix:line` in both cases
   (FR-274-AC-4, FR-274-AC-5).
6. Cite a listed `TC` ID in the matrix's coverage table beside its one summary row and its own `TC`
   artifact file; expect exit 0 (FR-274-AC-6).
7. Add two `ID Blocks` rows of one family sharing one ID; expect a non-zero exit naming both rows'
   `path:line` (FR-274-AC-7). Replace them with adjacent ranges of one family, and with one identical
   numeric range in two families; expect exit 0 (FR-274-AC-8).
8. Add a relocation map under `spec/relocations/` seeded, one map per run, with a wrong header, an
   out-of-order row, a repeated non-`-` `old_path`, a `new_path` absent from the tree, a `new_path`
   whose artifact declares a different `id` than `new_id`, an added-file row with a non-`-`
   `old_id`, and an added-file row whose `new_id` is not a `TM` ID; expect a non-zero exit naming
   each defect's `path:line` (FR-274-AC-9 through FR-274-AC-14).
9. Commit the clean tree as the base, set `SPEC_BASE` to it, and in the working tree move files
   while adding a relocation map that omits, in turn: a dropped identifier, an added non-`TM`
   identifier, an added `TM` identifier, a renumbered identifier and a renamed file; expect a
   non-zero exit each time naming the identifier or the renamed file's paths (FR-274-AC-15,
   FR-274-AC-16).
10. With `SPEC_BASE` set, make the same moves with a complete map: every rename, a new root index
    and new subsystem matrices with `old_path` and `old_id` `-`, and one collision renumbering as an
    `old_id`/`new_id` pair; expect exit 0 (FR-274-AC-17).
11. With `SPEC_BASE` set and no relocation map added, add one FR and one `TC`; expect exit 0
    (FR-274-AC-18).
12. Run `make spec` over a tree carrying the step-2 duplicate identifier; expect a non-zero exit
    whose output carries the check's finding (FR-274-AC-20).

## Expected Results

Every seeded defect fails the check with a non-zero exit and a finding on standard error naming the
identifier or path and every `path:line` involved; every near-miss and the clean tree exit 0, the
clean tree with exactly one summary line; and `make spec` exits non-zero whenever the check fails.
