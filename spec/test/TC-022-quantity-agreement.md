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

Execute every QSpec 5d88578 TC-187 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_187_quantities.rs` (`make conformance`).

## Test Procedure

1. Check the evaluated and admission-only lists together equal the TC-187 census.
2. Compare dimension algebra, ill-typed combinations, graph topology refusals (zero scale,
   duplicate root, cross-dimension target, target cycle, unknown target, missing root), decimal
   and integer targets, compound units, charges and counters on both sides.
3. Deny every named charge; sweep conversions across two unit families against an `i128` fraction
   oracle, and add/subtract/multiply/divide/compare over five units × 25 pairs.

## Expected Results

30 vectors agree exactly; U11 owner selection and stale keys are admission-only and named.
