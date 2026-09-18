---
id: TC-022
title: "Agree with the authority on quantity and unit vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-022: Agree with the authority on quantity and unit vectors

## Description

Execute every QSpec 7d7943a TC-187 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_187_quantities.rs` (`make conformance`).

## Test Procedure

1. Check the evaluated and admission-only lists together equal the TC-187 census.
2. Compare dimension algebra, ill-typed combinations, graph topology refusals (zero scale,
   duplicate root, cross-dimension target, target cycle, unknown target, missing root), decimal
   and integer targets, compound units and values on both sides; compare charges and counters where
   the authority meters them.
3. For U10, U13, U15, U16, U19, U20, U22–U24, U26, U28 and U29, agree with the authority on the
   value, charge schedule and consumed counters, metered under the QSpec 7d7943a limit tuple and one
   under its first short counter: `unit.rational-arithmetic` per event operands, power
   `max(1, |n| × maxparts)`, and `unit.target-domain` from the operand and target scale.
4. Deny every named charge; sweep conversions across two unit families against an `i128` fraction
   oracle, and add/subtract/multiply/divide/compare over five units × 25 pairs.

## Expected Results

30 vectors agree in value, outcome kind and charges, U10, U13, U15, U16, U19, U20, U22–U24, U26, U28
and U29 included: QSL `quire-spec-language#119` is fixed, so every vector's charges are asserted
against the authority like every other vector. U11 owner selection and stale keys are admission-only
and named.
