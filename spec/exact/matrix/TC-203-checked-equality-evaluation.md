---
id: TC-203
title: "Inspect checked equality ownership"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-277
    type: verifies
---
# TC-203: Inspect checked equality ownership

## Description

Inspect RT's ownership boundary for checked equality after the runtime has removed its duplicate
evaluator. The remaining behavioral Kani proof is tracked by FR-277-AC-7.

## Test Procedure

1. Inspect the locked QSV evaluator's direct call to `quire_exact::planned_equality_with_plan` on
the evaluated operands and its use of that call's Boolean relation. Confirm RT has no local
`CheckedEquality` or pair walker (FR-277-AC-8).
2. Run the separate titan Kani proof for FR-277-AC-7; TC-203 grants no proof credit before that
lane supplies its evidence.

## Expected Results

Inspection confirms QSV directly consumes the kernel equality plan and RT defines no local
checked-equality evaluator or pair walker. FR-277-AC-7 remains owned by the titan Kani lane.
