---
id: SR-13020
title: "IR-878 RT PR 132 spec review: FR-277 and FR-278 owner allocation"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@300924158cacde4428d59de719497fdbd56b7b37 vs origin/main d6bcef6; spec/exact/functional/FR-277-checked-equality-evaluation.md, spec/exact/functional/FR-278-runtime-checked-invariant-causes.md, spec/exact/matrix/tests.md, spec/spec.md; originals agent-ix/quire-exact origin/main spec/functional/FR-375, FR-369"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-277
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-278
    type: references
---

## Summary

Ticket: IR-878 (item 4, owner allocation of unbacked strict-coverage criteria). PR agent-ix/quire-contract-runtime#132 re-homes quire-exact FR-375-AC-1..5 and AC-8 as RT FR-277-AC-1..6 and quire-exact FR-369-AC-3/4 as RT FR-278-AC-1/2. Every acceptance-criterion cell was compared with the quire-exact origin/main original and is byte-identical apart from the id: no weakening. Ids FR-277/FR-278 follow RT's numbering (next after FR-276), the FR archetype sections are present, and verification is `Test`. `quire validate` is clean.

The defects are in what surrounds the verbatim text. FR-277 presents RT's `CheckedEquality::evaluate` as already "consuming the quire-exact planned relation", but RT evaluates through its own `Value` and its own `plan_pairs`/`planned_equality` walker and never calls the kernel's result-bearing seam. So AC-1's "result-bearing kernel seam" clause cannot be met by a test PR alone, and the PLANNED status hides a needed code change. FR-277 also drops FR-375's definition of the bounded input. FR-278 restates obligations RT already tests under FR-273-AC-8 and FR-008-AC-5, and its status says those tests have not landed.

## Examined criteria

- `FR-277-AC-1` (spec/exact/functional/FR-277-checked-equality-evaluation.md): On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`.
- `FR-277-AC-2`: On the same bounded shape with the first head unequal, the actual path completes `false` and still admits the complete 17-pair schedule and result-retain charge.
- `FR-277-AC-3`: A test meter denying the 17th `EqualityPair` yields typed `Incomplete` at `EqualityPair` with 16 pair charges admitted and no `EqualityResultRetain`.
- `FR-277-AC-4`: Equal-shaped operands with differing static composite types refuse at checked construction with `IllTypedCause::TypeMismatch`, before evaluation.
- `FR-277-AC-5`: `work_units = 50` admits plan-form 32 then refuses `equality.plan` (needs 19, 18 remain), `Incomplete` at `WorkUnits`, limit 50, consumed 32, next_charge 19.
- `FR-277-AC-6`: `result_units = 0` admits plan-form then refuses `equality.plan` atomically with `Incomplete { ResultUnits, 0, 0, 1 }`.
- `FR-278-AC-1` (spec/exact/functional/FR-278-runtime-checked-invariant-causes.md): RT checked-package/function producers return `UnknownCheckedFunction`, `ForeignCheckedExpression`, `MeterBorrowConflict`; no `CallDepthExceeded`; fuel exhaustion is `Incomplete` with `work_units`.
- `FR-278-AC-2`: RT collection, composite, equality and quantity producers return their table causes, including all six `operand_value` failures and the retained `IllTypedCause`.
- Context only: quire-exact FR-375 Behavior and FR-369 tables (origin/main); RT FR-273-AC-8, FR-008-AC-5, FR-275 residue list; spec/spec.md Requirements Architecture paragraph (examined, clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-277 says RT's `CheckedEquality::evaluate` consumes the quire-exact planned relation, and AC-1 requires the result-bearing kernel seam to report 17 pair events. RT's evaluator uses its own `Value` and its own `plan_pairs`/`planned_equality` and never calls `quire_exact::planned_equality_with_plan`. A test-only PR cannot meet AC-1 as written, or would credit it against RT's duplicate walker. The PLANNED status names only missing tests, not the code change needed | spec/exact/functional/FR-277-checked-equality-evaluation.md:15,32,46; src/exact/equality.rs:160-240 |
| FND-002 | medium | FR-277 does not define "the bounded depth-four `List` input". FR-375 defines it as QSL's `e21_operands` (occ 16 each, 17 pair events), which RT cannot import. RT has no `List` kind (kernel `CollectionKind::Sequence`), so AC-1..3 and AC-5 have no RT-resolvable fixture | spec/exact/functional/FR-277-checked-equality-evaluation.md:17,32 |
| FND-003 | medium | FR-278-AC-1's three producer causes restate RT FR-273-AC-8, and AC-2's six `operand_value` causes restate FR-008-AC-5. Both already have tagged RT tests (TC-194, TC-026). FR-278's status "runtime producer tests have not landed" is inaccurate, and the duplicate criteria do not say how they relate to the existing ones | spec/exact/functional/FR-278-runtime-checked-invariant-causes.md:31-32,41; spec/exact/functional/FR-273-exact-function-application.md:145; spec/exact/functional/FR-008-composite-collection-and-equality.md:164 |
| FND-004 | low | FR-278-AC-1's clause "the final carrier has no `CallDepthExceeded` variant" is about quire-exact's `CheckedInvariantCause` enum (kernel src/outcome.rs), not an RT code path; under the owner-allocation rule that clause is kernel-owned, though carried verbatim | spec/exact/functional/FR-278-runtime-checked-invariant-causes.md:31 |

## Verdict

**FAIL (changes requested)**: the re-homed AC text is faithful and the files are well formed. FND-001 needs a spec decision: either FR-277 states the RT code change (route `CheckedEquality::evaluate` through the kernel seam, tied to FR-275 residue removal) and its status says so, or the kernel-seam clause stays with quire-exact. FND-002 and FND-003 should be fixed in the same PR.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | FR-277-AC-7 is not byte-equal to quire-exact origin/main FR-375-AC-6. It inserts "The real evaluator is owned by quire-contract-runtime;" before "a plan-only equality or duplicated QSL walker does not satisfy this criterion" (origin/main: "A plan-only equality ..."). The clause adds an ownership statement and weakens nothing, but quire-exact #56 (c58ab92) calls the move "verbatim" | spec/exact/functional/FR-277-checked-equality-evaluation.md:37 |
| FND-006 | low | The FR-277-AC-7 row sits between AC-5 and AC-6 in the acceptance-criteria table, so the ids are out of order | spec/exact/functional/FR-277-checked-equality-evaluation.md:36-38 |
| FND-007 | low | FR-278 Behavior and TC-204 step 2 put the six `operand_value` cause assertions in TC-026. They are actually the in-crate unit tests in src/exact/equality.rs (lines 636, 686, 723 and the `equality_operand_failures_keep_distinct_causes` test), tagged FR-008-AC-5 only, not TC-026. The review pass's FND-003 wording repeated the same imprecision | spec/exact/functional/FR-278-runtime-checked-invariant-causes.md:25; spec/exact/matrix/TC-204-runtime-checked-invariant-causes.md:18; src/exact/equality.rs:636 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (5312d30): the Description now requires routing `CheckedEquality::evaluate` through `quire_exact::planned_equality_with_plan` and removing the RT-local walker under FR-275. Behavior and Status say the code change comes first and that no test may be tagged FR-277-AC-1 before it |
| FND-002 | fixed | Round 1: Inputs now define RT-constructed depth-four `CollectionKind::Sequence` operands (16 occurrences each, a 17-pair plan, the first-head-unequal control) and say QSL `e21_operands` is not imported |
| FND-003 | fixed | Round 1: Behavior and Status state the overlap with FR-273-AC-8/TC-194 and FR-008-AC-5 and say only the FR-278 tags and the missing controls are pending. The test-location imprecision is new finding FND-007 |
| FND-004 | fixed | Round 1: Behavior says the no-`CallDepthExceeded` carrier shape is owned by quire-exact FR-369-AC-1, gated on IR-497 |
| FND-005 | fixed | Round 2 (e815aca): the FR-277-AC-7 criteria cell is byte-identical (`cmp`) to the quire-exact origin/main FR-375-AC-6 cell. The ownership sentence moved to FR-277 Behavior |
| FND-006 | fixed | Round 2: the AC-7 row now follows AC-6; the table order is AC-1..AC-7 |
| FND-007 | fixed | Round 2: FR-278 Behavior and Status and TC-204 Description, procedure step 2 and Status now place the six `operand_value` assertions in the `src/exact/equality.rs` unit tests tagged FR-008-AC-5 (naming `equality_operand_failures_keep_distinct_causes`) and say they are not TC-026 tests |
