---
id: "SR-625"
title: "IR-322 spec integrity analysis: matrix split and id preservation (PR 91)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-runtime@b6514fa2b55d64fe7d1320f4d80dbe392f2d3bbe; spec/core/matrix/tests.md, spec/accounting/matrix/tests.md, spec/proptest_adapter/matrix/tests.md, spec/exact/matrix/tests.md, spec/tests.md, every spec/** frontmatter id and AC reference; base spec/test-matrix.md at 75e5ed04d415f47a99bca2db994d1add888495e9"
relationships: []
---

# SR-625: integrity of the RT matrix split

## Summary

Ticket: IR-322. PR: agent-ix/quire-contract-runtime#91 at b6514fa. The old `spec/test-matrix.md`
(TM-001) was moved with `git mv` to `spec/exact/matrix/tests.md`. Its rows were split into the
core (TM-002), accounting (TM-003) and proptest_adapter (TM-004) matrices, and `spec/tests.md`
(TM-005) indexes them. This review checks that the split lost, duplicated and altered nothing,
and that every id survived.

## Method

- **Rows.** Collected every data row of the old matrix and of the four new ones, excluding
  separator lines, as sorted multisets, and diffed them.
- **Prose.** Did the same for every line outside the tables, covering notes and Evidence
  Locations bullets.
- **Ids.** Diffed the frontmatter `id:` set and the distinct `<ID>-AC-<n>` reference set across
  spec/ at base and head.
- **Placement.** Checked each row's subsystem against the directory that holds its FR and TC
  files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | One sentence of the old shared note was narrowed to the core matrix only. It read: "Every other row is backed by a `tc_NNN` Rust test, a Kani harness or a `compile_fail` doctest; executable semantic claims retain direct acceptance-criterion trace tags." It used to qualify every row of TM-001, but now appears only in TM-002. TM-001 (exact) and TM-003 (accounting) no longer state what backs their rows. The text is not lost, but its scope changed silently. | spec/core/matrix/tests.md:36-38, spec/exact/matrix/tests.md:64-71, spec/accounting/matrix/tests.md:20-24 |

## Verdict

**The split is clean.** There is one low finding about the scope of a note.

- **Every row appears exactly once.** All 66 data rows of the old matrix (39 Functional
  Requirement Coverage, 1 Interface Requirement Coverage, 26 Test Case Summary) appear exactly
  once across the four new matrices and are byte-identical. The only extra lines are the
  repeated table headers, 3 each for the two tables.
- **Rows sit in the right subsystem.**
  - core: FR-001, FR-002, interface-001, and TC-001, TC-002, TC-003, TC-007.
  - accounting: FR-004, TC-006, TC-015.
  - proptest_adapter: FR-003, TC-004.
  - exact: FR-006 to FR-012, FR-273, TC-016 to TC-035, TC-194, TC-195.
  - Each TC file sits beside the matrix that declares it.
- **The prose diff has only the expected edits.**
  - Each new matrix got its own frontmatter and title.
  - The old note "FR-003-AC-2 and NFR-002-AC-3 by `compile_fail` doctests" was split into core
    ("NFR-002-AC-3 by a `compile_fail` doctest") and proptest_adapter ("FR-003-AC-2 is verified
    by a `compile_fail` doctest and has no test case").
  - The bullet "TC-001 and TC-006: `tests/integration.rs`" was split into a core TC-001 bullet
    and an accounting TC-006 bullet.
  - Every other bullet, including the long FR-273-AC-5 Evidence Locations text and the
    FR-010-AC-5 note, is unchanged and appears once.
- **Ids are preserved.**
  - The frontmatter ids are the same 45 at base and head, plus TM-002 to TM-005, with no
    duplicates.
  - The 96 distinct AC references are the same set.
  - TM-001 keeps its id at its new path.
  - interface-001 still has no frontmatter `id`, as on main. That is the known validate error.
- **The root index follows ADR-0056.**
  - TM-005 is typed TestMatrixIndex and declares no TC.
  - Its Requirements column lists exactly the requirement ids in each subsystem directory.
  - Its Status cells start with ✅ or 🚧 and repeat the matrices' own status claims. The
    proptest_adapter ✅ inherits the existing unbacked FR-003-AC-2 row, which SR-624 FND-004
    covers. It is not a new claim.
