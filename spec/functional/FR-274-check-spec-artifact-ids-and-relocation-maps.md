---
id: FR-274
title: "Check spec-artifact identifiers and relocation maps from make spec"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-345
    type: references
---
# FR-274: Check spec-artifact identifiers and relocation maps from make spec

## Description

`make spec` SHALL run a check local to this repository that fails when two spec artifacts declare
one identifier, when one test case leads more than one matrix row, when two recorded ID blocks of
one family overlap, or when a relocation map disagrees with the tree; and, when the change adds a
relocation map and a base revision is named, fails when the identifier set or the renamed files
disagree with that map. The identifier, ID-block, collision and relocation-map rules the check
enforces are those of `ix://agent-ix/quire-contract-ir/ADR-0056`. The check is this repository's
own and behaves as `quire-contract-ir:FR-345` does over that repository's tree.

A spec artifact is a Markdown document under `spec/` outside `spec/reviews/`: the StR, FR, NFR,
interface, TC, AD, AA, AP, CAC, MP and SUR artifacts, the master-requirements document and the
test matrices. Its identifier is the first `id:` line of its YAML frontmatter; a document with no
frontmatter `id` declares no identifier. Files under `plan/`, `planning/`, `reviews/` and
`spec/reviews/` carry their own identifiers and are not read for identifiers.

## Inputs

- Every spec artifact in the working tree and its first frontmatter `id:` line.
- Every spec artifact typed `TestMatrix`, and the first cell of each data row of its
  `## Test Case Summary` table.
- The `## ID Blocks` table of the spec artifact typed `master-requirements`, when that table is
  present.
- Every file under `spec/relocations/`: a tab-separated relocation map.
- A base revision named by the `SPEC_BASE` environment variable, read from Git objects, when set.

## Outputs

- Exit status 0 and exactly one summary line on standard output when no defect is found.
- A non-zero exit status and one finding per defect on standard error, each naming the identifier
  or path concerned and every `path:line` involved.

## Behavior

- When two spec artifacts declare the same frontmatter `id`, the check SHALL report the identifier
  and the `path:line` of each declaration.
- When one `TC` ID leads more than one `Test Case Summary` row, within one matrix or across
  matrices, the check SHALL report the identifier and each `matrix:line`.
- The check SHALL NOT report a `TC` artifact file that declares the same `TC` ID as the one
  `Test Case Summary` row listing it: the file declares the case and the row indexes it.
- When two rows of one family in the `## ID Blocks` table cover a common ID, the check SHALL report
  the `path:line` of both rows.
- For each relocation map, the check SHALL require a header row naming exactly the columns
  `old_path`, `new_path`, `old_id` and `new_id` in that order; data rows sorted by `old_path` then
  `new_path`; each `old_path` other than `-` in at most one row; each `new_path` present in the
  working tree; where `new_id` is not `-`, the spec artifact at `new_path` declaring `new_id`; and,
  where `old_path` is `-`, `old_id` also `-` and `new_id` a `TM` ID.
- A relocation map is added by the change when its path is present in the working tree and absent
  at `SPEC_BASE`.
- When `SPEC_BASE` is set and the change adds at least one relocation map, the check SHALL take the
  spec-artifact identifier set at `SPEC_BASE`, replace each `old_id` by its `new_id` for every
  added-map row where both are identifiers, add every `new_id` whose `old_id` is `-`, and require
  the result to equal the spec-artifact identifier set in the working tree.
- When `SPEC_BASE` is set and the change adds at least one relocation map, the check SHALL require
  every file renamed under `spec/` between `SPEC_BASE` and the working tree to have exactly one row
  across the added maps.
- The check SHALL compare identifier sets across revisions only when `SPEC_BASE` is set and the
  change adds a relocation map.
- When the check finds a defect, the check SHALL exit with a non-zero status.
- The check SHALL NOT report a defect as a warning.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-274-AC-1 | When two spec artifacts declare the same frontmatter `id`, the check exits non-zero and reports the identifier with the `path:line` of each declaration. | Test (TC-196) |
| FR-274-AC-2 | When an identifier is declared in the frontmatter of one spec artifact and appears only in the body prose of another, the check reports nothing for it. | Test (TC-196) |
| FR-274-AC-3 | When the same frontmatter `id` is declared by two files under `plan/`, `planning/`, `reviews/` or `spec/reviews/`, or by one such file and one spec artifact, the check reports nothing for it. | Test (TC-196) |
| FR-274-AC-4 | When one `TC` ID leads two `Test Case Summary` rows of one matrix, the check exits non-zero and reports the identifier with each `matrix:line`. | Test (TC-196) |
| FR-274-AC-5 | When one `TC` ID leads a `Test Case Summary` row in each of two matrices, the check exits non-zero and reports the identifier with each `matrix:line`. | Test (TC-196) |
| FR-274-AC-6 | When a `TC` ID leads one `Test Case Summary` row and is also cited in another table of a matrix or declared by its own `TC` artifact file, the check reports nothing for it. | Test (TC-196) |
| FR-274-AC-7 | When two `## ID Blocks` rows of one family cover a common ID, the check exits non-zero and reports the `path:line` of both rows. | Test (TC-196) |
| FR-274-AC-8 | When `## ID Blocks` rows of one family hold adjacent, non-overlapping ranges, or rows of different families hold the same numeric range, the check reports nothing for them. | Test (TC-196) |
| FR-274-AC-9 | If a relocation map's header row is not exactly `old_path`, `new_path`, `old_id`, `new_id`, then the check exits non-zero and reports the map's `path:line`. | Test (TC-196) |
| FR-274-AC-10 | If a relocation map's data rows are not sorted by `old_path` then `new_path`, then the check exits non-zero and reports the `path:line` of each out-of-order row. | Test (TC-196) |
| FR-274-AC-11 | If one relocation map repeats an `old_path` other than `-`, then the check exits non-zero and reports the `path:line` of each row carrying it. | Test (TC-196) |
| FR-274-AC-12 | If a relocation map row names a `new_path` absent from the working tree, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-13 | If a relocation map row's `new_id` is not `-` and the spec artifact at its `new_path` declares a different `id`, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-14 | If a relocation map row has `old_path` `-` and either an `old_id` other than `-` or a `new_id` that is not a `TM` ID, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-15 | While `SPEC_BASE` is set and the change adds a relocation map, when the working tree drops an identifier, adds a non-`TM` identifier, adds a `TM` identifier with no map row, or renumbers an identifier with no map row, the check exits non-zero and reports each differing identifier with the `path:line` that declares it at `SPEC_BASE` or in the working tree. | Test (TC-196) |
| FR-274-AC-16 | While `SPEC_BASE` is set and the change adds a relocation map, when a file renamed under `spec/` between `SPEC_BASE` and the working tree has no row, or more than one row, across the added maps, the check exits non-zero and reports the renamed file's old and new paths. | Test (TC-196) |
| FR-274-AC-17 | While `SPEC_BASE` is set and the change adds a relocation map listing every rename, a new root index and new subsystem matrices with `old_path` and `old_id` `-`, and a collision renumbering as one `old_id`/`new_id` pair, the check exits 0. | Test (TC-196) |
| FR-274-AC-18 | While `SPEC_BASE` is set and the change adds no relocation map, when the change adds requirement and test-case identifiers, the check exits 0. | Test (TC-196) |
| FR-274-AC-19 | When the tree holds none of the defects in FR-274-AC-1 through FR-274-AC-16, the check exits 0 and writes exactly one summary line to standard output and nothing to standard error. | Test (TC-196) |
| FR-274-AC-20 | When the check finds any defect, `make spec` exits non-zero. | Test (TC-196) |

## Dependencies

- **Decision**: `ix://agent-ix/quire-contract-ir/ADR-0056` defines identifiers, ID blocks,
  collisions, matrices and relocation maps.
- **Peer requirement**: `quire-contract-ir:FR-345` specifies the same check over that repository's
  tree; this requirement states it over this one and shares no code with it.
- **Verification**: [TC-196](../test/TC-196-spec-artifact-id-and-relocation-check.md).
