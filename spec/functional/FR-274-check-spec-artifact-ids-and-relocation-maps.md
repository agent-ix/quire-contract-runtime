---
id: FR-274
title: "Check spec-artifact identifiers and relocation maps from make spec"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/StR-002
    type: traces_to
  - target: ix://agent-ix/quire-contract-ir/ADR-0056
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-345
    type: references
---
# FR-274: Check spec-artifact identifiers and relocation maps from make spec

## Description

`make spec` SHALL run a check local to this repository, before any other command of its recipe,
that fails when two spec artifacts declare one identifier, when one test case leads more than one
matrix row, when two recorded ID blocks of one family overlap, or when a relocation map is
malformed; and, when the change adds a relocation map, fails when the identifiers, the live test
cases or the moved paths disagree with that map. The identifier, ID-block, collision and
relocation-map rules the check enforces are those of `ix://agent-ix/quire-contract-ir/ADR-0056`.
The check reads the tree of its working directory.

A spec artifact is a Markdown document under `spec/` outside `spec/reviews/`: the StR, FR, NFR,
interface, TC, AD, AA, AP, CAC, MP and SUR artifacts, the master-requirements document and the
test matrices. Its identifier is the first `id:` line of its YAML frontmatter; a document with no
frontmatter `id` declares no identifier. Files under `plan/`, `planning/`, `reviews/` and
`spec/reviews/` carry their own identifiers and are not read for identifiers.

At a revision:

- the **frontmatter identifier set** is the set of identifiers the spec artifacts declare;
- the **live test-case set** is the set of `TC` IDs leading a `## Test Case Summary` row of a
  spec artifact typed `TestMatrix`.

The base revision is the revision named by the `SPEC_BASE` environment variable.

- When `make spec` runs without `SPEC_BASE` in its environment and the ref `origin/main` exists,
  `make spec` SHALL set `SPEC_BASE` to the merge base of `HEAD` and `origin/main`.
- When `make spec` runs without `SPEC_BASE` in its environment and the ref `origin/main` does not
  exist, `make spec` SHALL run the check with `SPEC_BASE` unset.

A relocation map is **added by the change** when `SPEC_BASE` is set and the map's path is present in
the working tree and absent at `SPEC_BASE`. When `SPEC_BASE` is not set, no relocation map is added
by the change.

A path is **moved away** when it names a file under `spec/` at `SPEC_BASE` and no file exists at
that path in the working tree, whether the file was deleted, moved within `spec/` or moved out of
`spec/`. The check decides this from paths alone and applies no content-similarity rename
detection.

## Inputs

- Every spec artifact in the working tree and its first frontmatter `id:` line.
- Every spec artifact typed `TestMatrix`, and the first cell of each data row of its
  `## Test Case Summary` table.
- The `## ID Blocks` table of the spec artifact typed `master-requirements`, when that table is
  present.
- Every file under `spec/relocations/`: a tab-separated relocation map.
- The base revision named by `SPEC_BASE`, read from Git objects, when set.

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
  `new_path`; each `old_path` other than `-` in at most one row; and, where `old_path` is `-`,
  `old_id` also `-` and `new_id` a `TM` ID.
- For each relocation map added by the change, the check SHALL require each `new_path` present in
  the working tree and, where `new_id` is not `-`, the spec artifact at `new_path` declaring
  `new_id`.
- The check SHALL NOT validate the `new_path` or `new_id` of a relocation map present at
  `SPEC_BASE`, or of any relocation map when `SPEC_BASE` is not set, against the working tree.
- When the change adds at least one relocation map, the check SHALL take the frontmatter identifier
  set at `SPEC_BASE`, replace each `old_id` by its `new_id` for every added-map row where both are
  identifiers, add every `new_id` whose `old_id` is `-`, and require the result to equal the
  frontmatter identifier set in the working tree.
- When the change adds at least one relocation map, the check SHALL require the live test-case set
  in the working tree to equal the live test-case set at `SPEC_BASE`.
- When the change adds at least one relocation map, the check SHALL require every path moved away
  to be the `old_path` of exactly one row across the added maps.
- The check SHALL compare revisions only when the change adds a relocation map.
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
| FR-274-AC-12 | If a row of a relocation map added by the change names a `new_path` absent from the working tree, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-13 | If a row of a relocation map added by the change has a `new_id` that is not `-` and the spec artifact at its `new_path` declares a different `id`, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-14 | If a relocation map row has `old_path` `-` and either an `old_id` other than `-` or a `new_id` that is not a `TM` ID, then the check exits non-zero and reports the row's `path:line`. | Test (TC-196) |
| FR-274-AC-15 | While the change adds a relocation map, when the working tree's frontmatter identifier set drops an identifier, adds a non-`TM` identifier, adds a `TM` identifier with no map row, or renumbers an identifier with no map row, the check exits non-zero and reports each differing identifier with the `path:line` that declares it at `SPEC_BASE` or in the working tree. | Test (TC-196) |
| FR-274-AC-16 | While the change adds a relocation map, when a path moved away is the `old_path` of no row, or of rows in two added maps, the check exits non-zero and reports the moved-away path and the `path:line` of each row naming it. | Test (TC-196) |
| FR-274-AC-17 | While the change adds a relocation map listing every path moved away, a new root index and new subsystem matrices with `old_path` and `old_id` `-`, and a requirement-identifier collision renumbering as one `old_id`/`new_id` pair, with the live test-case set unchanged, the check exits 0. | Test (TC-196) |
| FR-274-AC-18 | While `SPEC_BASE` is set and the change adds no relocation map, when the change adds requirement and test-case identifiers, the check exits 0. | Test (TC-196) |
| FR-274-AC-19 | When the tree holds none of the defects this table names, the check exits 0 and writes exactly one summary line to standard output and nothing to standard error. | Test (TC-196) |
| FR-274-AC-20 | When the check finds a defect, `make spec` runs the check before `quire validate`, exits non-zero, and its output carries the check's finding. | Test (TC-196) |
| FR-274-AC-21 | While the change adds a relocation map, when a `TC` ID leading a `Test Case Summary` row at `SPEC_BASE` leads no such row in the working tree, the check exits non-zero and reports the identifier, even when that `TC`'s artifact file remains. | Test (TC-196) |
| FR-274-AC-22 | When a relocation map present at `SPEC_BASE` names a `new_path` absent from the working tree or a `new_id` its `new_path` no longer declares, the check reports nothing for that map. | Test (TC-196) |
| FR-274-AC-23 | While `SPEC_BASE` is not set, when the working tree holds a relocation map whose `new_path` is absent from the working tree or whose `new_id` its `new_path` does not declare, the check reports nothing for that map, compares no revisions, and exits 0. | Test (TC-196) |
| FR-274-AC-24 | When the check finds a defect, it exits non-zero and labels no finding as a warning. | Test (TC-196) |
| FR-274-AC-25 | When `make spec` runs without `SPEC_BASE` in its environment in a repository where the ref `origin/main` exists, it runs the check with `SPEC_BASE` set to the merge base of `HEAD` and `origin/main`. | Test (TC-196) |
| FR-274-AC-26 | When `make spec` runs without `SPEC_BASE` in its environment in a repository with no ref `origin/main`, it runs the check with `SPEC_BASE` unset. | Test (TC-196) |

## Dependencies

- **Upstream**: [StR-002](../stakeholder/StR-002-machine-checkable-specification.md).
- **Decision**: `ix://agent-ix/quire-contract-ir/ADR-0056` defines identifiers, ID blocks,
  collisions, matrices and relocation maps.
- **Peer requirement**: `quire-contract-ir:FR-345` specifies that repository's own check under the
  same decision. This requirement governs this repository's check and differs from FR-345's text:
  it validates `new_path` and `new_id` only for relocation maps added by the change; it excludes
  `planning/`; it compares the live test-case set as well as the frontmatter identifier set; it
  reports `path:line` for every finding; it defines a moved-away path without rename detection; and
  `make spec` supplies `SPEC_BASE` from `origin/main`. The two checks share no code.
- **Verification**: [TC-196](../test/TC-196-spec-artifact-id-and-relocation-check.md).
