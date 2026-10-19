---
id: SR-13021
title: "IR-878 RT PR 132 integrity review: matrix rows and cross-references"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@300924158cacde4428d59de719497fdbd56b7b37 vs origin/main d6bcef6; spec/exact/matrix/tests.md, spec/spec.md, spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/functional/FR-278-runtime-checked-invariant-causes.md"
review_set: subset
---

## Summary

Ticket: IR-878. This review checked the structure of the matrix, the cross-references and the coverage delta. `quire validate --scope .` over the changed files and over `spec/**/*.md` exits 0. The new FRs' `ix://agent-ix/quire-exact/FR-375` and `FR-369` `depends_on` edges resolve, and spec.md's architecture paragraph now names FR-277 and FR-278. `quire coverage --scope . --strict` goes from 88/186 backed and 110 unbacked on origin/main to 88/194 and 118 unbacked at HEAD. The +8 are exactly FR-277-AC-1..6 and FR-278-AC-1/2, with 0 contradicted statuses.

The two new rows in spec/exact/matrix/tests.md come after the blank line that ends the Functional Requirement Coverage table. They are not table rows: the matrix document still parses 24 rows (13/24 backed) at both base and HEAD, so the PLANNED status cells are never read and never checked against the status vocabulary.

## Examined criteria

- `spec/exact/matrix/tests.md` FR-277 row: `| FR-277 | FR-277-AC-1 through FR-277-AC-6 | — | 🚧 PLANNED: checked-evaluator tests at the runtime owner; no TC or trace tag is credited by this spec allocation |`
- `spec/exact/matrix/tests.md` FR-278 row: `| FR-278 | FR-278-AC-1, FR-278-AC-2 | — | 🚧 PLANNED: runtime producer-cause tests; no TC or trace tag is credited by this spec allocation |`
- `spec/spec.md` Requirements Architecture: "... the planned monitor consumer FR-276, checked-evaluator owner FR-277, typed runtime producer FR-278, and quality requirements NFR-001 and NFR-002." (clean)
- FR-277/FR-278 frontmatter relationships and Dependencies links (clean; resolved by validate).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-277 and FR-278 matrix rows sit after the blank line that ends the coverage table, so they are not part of it. Coverage parses 24 rows in spec/exact/matrix/tests.md at both base and HEAD, so the PLANNED status cells are never read or vocabulary-checked; "0 contradicted" proves nothing for these rows | spec/exact/matrix/tests.md:76-79 |
| FND-002 | low | The new rows use the range form "FR-277-AC-1 through FR-277-AC-6" and an em-dash TC cell. Every other row in the table lists AC ids individually and names a TC, including the planned ones (TC-200/TC-201 in spec.md) | spec/exact/matrix/tests.md:77-78 |

## Verdict

**FAIL (changes requested)**: move the two rows inside the table, with no blank line before them, and list the AC ids individually. Then re-run `quire coverage --strict` to confirm the matrix document counts 26 rows and accepts the status wording.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new TC-203 and TC-204 are named in the coverage rows but are missing from the tests.md Test Case Summary table (which ends at TC-202) and from spec.md's TC list ("TC-202, planned TC-200 and TC-201") | spec/exact/matrix/tests.md:105; spec/spec.md:82 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (5312d30): the rows are now inside the coverage table. `quire coverage --strict` reports them as tests.md:76 (FR-277) and tests.md:77 (FR-278) functional-coverage rows, with 0 contradicted statuses ("🚧 Planned:" accepted) |
| FND-002 | fixed | Round 1: the AC ids are listed individually (AC-1..AC-7) and the rows name TC-203 and TC-204 |
| FND-003 | fixed | Round 2 (e815aca): TC-203 and TC-204 rows are in the tests.md Test Case Summary (parsed as tests.md:106/107), and spec.md lists "TC-202–TC-204" |
