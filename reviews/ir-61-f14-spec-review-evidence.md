---
id: SR-11585
title: "IR-61 F14 FR-007 correction: evidence analysis"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-runtime@2cde819ac7520f31b9b43dab5b25a158edbd4691 (base 40425522c8b3f962c543969d283e54acbccf00e3); spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-61. This analysis checks whether each new or changed criterion's verification
method fits its obligation, and whether the matrix rows are honest about their evidence.

## Verdict

**FAIL: one medium finding and one low finding.**

`Test` fits AC-15, AC-16 and AC-17, each of which has an executable vector that was confirmed
for this review. The matrix rows are honestly marked planned or unbacked (🚧), and the AC-12
overclaim is removed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-18's single `Test (TC-034)` method covers an admission clause that no RT test can execute (see SR-11584 FND-001). Because of the table split in SR-11580 FND-001, quire also records the method as the uncatalogued `b`. The method should be split: `Test` for the two `mod` vectors, and a delegated method for the admission refusal, such as compiler/QSpec TC-192 DIV-03, or Inspection of the RT vocabulary. | spec/exact/functional/FR-007-exact-scalar-families.md:117; spec/exact/matrix/tests.md:45 |
| FND-002 | low | Rows AC-12, AC-15 and AC-18, and the TC-034 row, say "planned ... against the kernel" or "no RT-local binder" without naming owner evidence that already exists. Neighbouring rows AC-8 to AC-11 do name theirs. The existing owner evidence is in quire-exact: `only_the_exposed_member_must_be_in_domain` (FR-357-AC-2) uses exactly AC-12's two vectors under every profile; `exposed_member_outside_domain_refuses_with_its_cause` is FR-357-AC-3; `modulo_is_euclidean_after_every_division_profile` (FR-357-AC-7) covers all four `mod` sign quadrants; and FR-365-AC-1 covers all six modes on positive and negative ties. Failure: a reader concludes these behaviours have no evidence anywhere and re-derives them. | spec/exact/matrix/tests.md:40; :42; :45; :88 |

## Coverage

Examined:

- the Verification cells of FR-007-AC-8, AC-12, AC-15, AC-16, AC-17 and AC-18
- matrix rows 36, 40, 42-45 and 88, and the disposition row at 136

Read as context only:

- quire-exact `src/division.rs` tests
- quire-exact `tests/ir673_core.rs` and `tests/ir673_decimal.rs`
- the `quire coverage --strict` output at base and head

## New findings (disposition pass 1)

These were found while checking the round 1 fixes at
f7e1e02615e1069743f55801a5d3bd11f5fa2693.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The new FR-007-AC-19 is verified as `Test (owner ix://agent-ix/quire-specification/FR-147-AC-5, TC-192 DIV-03)`, yet its own text says "RT ... neither produces nor tests this admission refusal". The spec-artifacts-process `verification_catalog` has no delegated or owner method: its classes are Test, Analysis, Demonstration and Inspection. quire therefore treats this as an RT `Test` with no binder. It adds two permanently unbacked rows that no RT work can close: `FR-007-AC-19 [verification]` and `FR-007 (tests.md:46) [functional-coverage]`, the matrix row whose TC cell is the free text "owner QSpec TC-192 DIV-03". That takes `--strict` from 113 to 114. The matrix row discloses the delegation honestly, but the criterion contradicts its own method. Under this catalog a delegated verification should not be an RT criterion at all. Fix: remove AC-19 and keep the compiler-admission sentence as Behavior context with its QSpec link. Alternatively, restate it as an RT `Inspection` that `PackageCause` carries `incompatible-definition`, or get a delegated method catalogued upstream first. | spec/exact/functional/FR-007-exact-scalar-families.md:122; spec/exact/matrix/tests.md:46 |

## Dispositions

Round 1 checks head f7e1e02615e1069743f55801a5d3bd11f5fa2693 against `git diff 2cde819..f7e1e02`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-18 now holds only executable `mod` behaviour: four quadrant vectors and the `ModuloOutOfDomain` refusal. Its Verification cell parses as `Test (TC-034)`, and TC-034 step 9 tests it. The admission clause moved to AC-19, and how AC-19 is verified is recorded as new FND-003. |
| FND-002 | fixed | The rows now cite existing owner evidence: AC-12 cites FR-357-AC-2 (`only_the_exposed_member_must_be_in_domain`) and FR-357-AC-3. AC-15 cites FR-365-AC-1 and says plainly that it does not cover these division vectors. AC-18 cites FR-357-AC-7. The TC-034 row cites FR-357-AC-2, AC-3 and AC-7 and FR-362, 363, 365 and 366. Every cited id exists and is traced in quire-exact origin/main d18927e. |
| FND-003 | fixed | Round 2, checked at 71f0cb7b69f4690acda358cb2860890cd112da6d. The delegated admission criterion is removed, and no verification method was invented. Admission now appears only as FR-007 Behavior context ("as specified by `ix://agent-ix/quire-specification/FR-147-AC-5` and QSpec TC-192 DIV-03. RT carries the refusal vocabulary but neither produces nor tests this admission refusal") and as a TC-034 note. Its matrix rows are gone. `quire coverage --strict` reports 112 unbacked rows, the same as base. The id FR-007-AC-19 is now the division-exposure criterion, `Test (TC-034)`. AC-19 has no row in the strict list, although `quire matrix` marks it untagged, because the strict `[verification]` and `[traces-to]` checks work at test-case level. TC-034 has backing symbols: `tests/exact_semantics.rs:104,149,223` are tagged `TC-034`. So every criterion verified by TC-034 passes that check whether or not it is tagged itself. The same holds for the untagged AC-8 to AC-11 and AC-15 to AC-18, before and after this change. AC-19 is therefore not silently unreported: it is counted at criterion level by `quire matrix` (72/173 backed at base, 72/178 at this head) but not listed by `--strict`. Equal strict counts at base and head do not mean no new unbacked criteria; there are five untagged, planned new criteria (AC-15 to AC-19). |
