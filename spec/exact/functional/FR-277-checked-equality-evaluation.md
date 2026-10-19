---
id: FR-277
title: "Exercise checked equality through the runtime evaluator and kernel meter"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: depends_on
---
# FR-277: Exercise checked equality through the runtime evaluator and kernel meter

## Description

The runtime SHALL route `CheckedEquality::evaluate` in `src/exact/equality.rs` through `quire_exact::planned_equality_with_plan`, consuming one kernel plan for its relation and `Meter` effects. This removes RT-local `planned_equality`/`plan_pairs` residue as part of the FR-275 one-copy work. This requirement allocates the end-to-end controls formerly in quire-exact FR-375-AC-1 through AC-5 and AC-8 to the runtime that owns the evaluator.

## Inputs

- Two separately constructed, admitted depth-four `quire_exact::Value` sequences (`CollectionKind::Sequence`) with the same checked composite declaration identity, 16 visited value occurrences per operand and a 17-pair kernel plan; the unequal control differs at its first head. RT constructs these values through kernel constructors and does not import QSL `e21_operands`.
- A runtime meter with the stated charge limits or injected denial.

## Outputs

- The existing typed equality outcome and admitted meter charges.

## Behavior

`CheckedEquality::evaluate` currently calls RT-local `planned_equality` and `plan_pairs`, so the kernel route is a required code change before the controls can pass. RT SHALL accept `quire_exact::Value` directly, delete its distinct `Value` and local pair walker, and preserve the kernel plan's relation and complete pair schedule. The meter SHALL admit only successful charges, in the order plan-form, plan, each pair, and result retention. Checked construction SHALL reject unlike static composite declarations before evaluation. The real evaluator is owned by quire-contract-runtime. The Kani claim is FR-277-AC-7, owned by titan:kani:plan; this requirement grants no proof credit before that lane runs.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-277-AC-1 | On the bounded depth-four `Sequence` input with equal `quire_exact::Value` operands, the actual `CheckedEquality::evaluate` path completes `true` and admits exactly 17 `EqualityPair` charges in occurrence-pair order after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`. | Test (TC-203) |
| FR-277-AC-2 | On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule. | Test |
| FR-277-AC-3 | If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted. | Test |
| FR-277-AC-4 | With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. | Test |
| FR-277-AC-5 | With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned. | Test |
| FR-277-AC-6 | With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control. | Test |
| FR-277-AC-7 | The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion. | Test |
| FR-277-AC-8 | Inspection of the production `CheckedEquality::evaluate` path shows a direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and consumption of that call's relation and complete 17-event plan. A separate kernel call does not establish this criterion. | Inspection (TC-203) |

## Dependencies

- [quire-exact FR-375](ix://agent-ix/quire-exact/FR-375) owns equality planning and metering semantics.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the owner allocation. FR-275 residue removal and kernel routing must land before FR-277 tests are tagged.

## Status

PLANNED: first route `CheckedEquality::evaluate` through the result-bearing quire-exact kernel seam and remove RT-local equality-walker residue under FR-275; then add and tag TC-203 runtime controls. No test may be tagged FR-277-AC-1 before that code change. FR-277-AC-7 remains PLANNED in titan:kani:plan. FR-277-AC-8 (inspection) stays PLANNED until the direct kernel call lands.
