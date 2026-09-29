---
id: TC-196
title: "Fail the spec-artifact ID and relocation-map check on each seeded defect"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-274
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/StR-002
    type: verifies
---
# TC-196: Fail the spec-artifact ID and relocation-map check on each seeded defect

## Description

Check that the repository-local identifier and relocation-map check of
[FR-274](../functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md) fails on each seeded
defect, names every `path:line` involved, passes each seeded near-miss, passes a clean tree with one
summary line, and that `make spec` runs it first, supplies `SPEC_BASE` and fails whenever it fails.
Each step runs against a scratch Git repository built by the test, never against this repository's
own `spec/` tree. The check runs with the scratch repository as its working directory; `make spec`
runs as `make -C <scratch repository> -f <this repository's Makefile> spec`, so every recipe line
runs in the scratch repository.

## Test Procedure

1. Build a clean scratch tree: a master-requirements document with an `## ID Blocks` table, two
   FRs, one `TC` artifact file, and one `TestMatrix` whose `Test Case Summary` lists that `TC`.
   Run the check with `SPEC_BASE` unset; expect exit 0, exactly one line on standard output and
   nothing on standard error (FR-274-AC-19).
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
   out-of-order row, a repeated non-`-` `old_path`, an added-file row with a non-`-` `old_id`, and
   an added-file row whose `new_id` is not a `TM` ID; expect a non-zero exit naming each defect's
   `path:line` (FR-274-AC-9, FR-274-AC-10, FR-274-AC-11, FR-274-AC-14).
9. Commit the clean tree as the base and set `SPEC_BASE` to it. Add a relocation map whose row names
   a `new_path` absent from the tree, then one whose `new_path` artifact declares a different `id`
   than `new_id`; expect a non-zero exit naming the row's `path:line` each time (FR-274-AC-12,
   FR-274-AC-13).
10. Commit to the base a relocation map whose rows were valid when committed, then move or renumber
    the files it names so its `new_path` is absent and its `new_id` is no longer declared; with
    `SPEC_BASE` set to that base, expect no finding for that map (FR-274-AC-22). Run the same tree
    with `SPEC_BASE` unset; expect no finding for that map and exit 0 (FR-274-AC-23).
11. With `SPEC_BASE` set, move files in the working tree while adding a relocation map that omits,
    in turn: a dropped identifier, an added non-`TM` identifier, an added `TM` identifier, a
    renumbered identifier and a path moved away; expect a non-zero exit each time naming the
    identifier or the moved-away path (FR-274-AC-15, FR-274-AC-16). Repeat the path-moved-away
    omission once for a file deleted, once for a file moved within `spec/` whose link targets were
    edited, and once for a file moved out of `spec/`.
12. With `SPEC_BASE` set, move one file and list that move in each of two added relocation maps;
    expect a non-zero exit naming the moved-away path and both rows' `path:line` (FR-274-AC-16).
13. With `SPEC_BASE` set and a relocation map added, remove one `TC`'s `Test Case Summary` row while
    keeping its `TC` artifact file; expect a non-zero exit naming that `TC` (FR-274-AC-21).
14. With `SPEC_BASE` set, make the same moves with a complete map: every path moved away, a new root
    index and new subsystem matrices with `old_path` and `old_id` `-`, and one FR collision
    renumbering as an `old_id`/`new_id` pair, leaving every `Test Case Summary` row in place; expect
    exit 0 (FR-274-AC-17).
15. With `SPEC_BASE` set and no relocation map added, add one FR and one `TC`; expect exit 0
    (FR-274-AC-18).
16. For every run in steps 2 to 13 that exits non-zero, check that no finding is labelled a warning;
    and for every seeded defect, check that no run exits 0 (FR-274-AC-24).
17. Run `make spec` over a scratch tree carrying the step-2 duplicate identifier; expect the check's
    output to precede any `quire validate` output, a non-zero exit, and the check's finding in the
    output (FR-274-AC-20).
18. In a scratch repository holding a ref `origin/main` at the base commit, move one FR with an added
    relocation map that omits its row, and run `make spec` with `SPEC_BASE` absent from the
    environment; expect the check to run with `SPEC_BASE` equal to the merge base of `HEAD` and
    `origin/main`, and `make spec` to exit non-zero naming the lost path (FR-274-AC-25,
    StR-002-VC-1).
19. Delete the ref `origin/main` from the step-18 repository and run `make spec` with `SPEC_BASE`
    absent from the environment; expect the check to run with `SPEC_BASE` unset and report no
    cross-revision finding (FR-274-AC-26).

## Expected Results

Every seeded defect fails the check with a non-zero exit and a finding on standard error, never
labelled a warning, naming the identifier or path and every `path:line` involved; every near-miss
and the clean tree exit 0, the clean tree with exactly one summary line; a relocation map already at
the base revision, or any map while `SPEC_BASE` is unset, is not validated against the moved tree;
and `make spec` runs the check first, supplies `SPEC_BASE` from `origin/main` when present, and
exits non-zero whenever the check fails.
