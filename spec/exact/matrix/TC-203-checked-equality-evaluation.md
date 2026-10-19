---
id: TC-203
title: "Exercise checked equality through the kernel plan and runtime meter"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-277
    type: verifies
---
# TC-203: Exercise checked equality through the kernel plan and runtime meter

## Description

Verify FR-277-AC-1 through AC-6 by test and AC-8 by inspection only after `CheckedEquality::evaluate` routes through `quire_exact::planned_equality_with_plan`. FR-277-AC-7 is the separate titan:kani:plan claim.

## Test Procedure

1. Construct the admitted depth-four `quire_exact::Value` operands defined in FR-277 Inputs, with equal heads and one first-head-unequal control. Use the normal checker to construct `CheckedEquality`; evaluate each with a fresh sufficient meter.
2. Inspect the production evaluator's direct call to `quire_exact::planned_equality_with_plan` on the evaluated operands and its use of that call's relation and plan (AC-8). In the behavioral test, assert exactly 17 admitted `EqualityPair` charges in occurrence-pair order on the real evaluation path (AC-1). A separate kernel-only assertion does not satisfy either criterion. Check the ordered plan-form, plan, pair and result-retain charges for both relations.
3. Evaluate with 50 work units and other limits sufficient. Inject denial at the 17th `EqualityPair` and separately at `EqualityResultRetain`, recording each admitted prefix and typed stop.
4. Evaluate with zero result units and sufficient work and occurrence limits. Record the admitted prefix and typed stop.
5. Construct equal-shaped operands with different declared static composite types. Check construction and the unchanged meter before evaluating.

## Expected Results

1. Equal and first-head-unequal cases complete `true` and `false`, respectively. Inspection confirms the direct kernel call's 17-event plan; the real evaluator admits 17 `EqualityPair` charges in the stated order without short circuit.
2. At 50 work units, 32 plan-form units are admitted and the plan's 19-unit reservation refuses atomically with `Incomplete` at `WorkUnits`; no plan or pair charge is admitted. A denial at pair 17 admits exactly 16 pairs; a result-retain denial follows all 17 pairs. Neither returns a completed Boolean.
3. Zero result units refuse at `EqualityPlan` after plan-form, with `limit_kind: ResultUnits`, limit and consumed zero, and `next_charge: 1`. No later charge is admitted.
4. Different declared static types refuse at checked construction with `IllTypedCause::TypeMismatch`; evaluation is not called and the meter is unchanged.

## Status

PLANNED after FR-275 kernel routing and deletion of RT's distinct `Value` and pair walker. No FR-277-AC-1 test tag is credited before the real evaluator calls the kernel.
