---
id: TC-020
title: "Agree with the authority on IEEE profile vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-020: Agree with the authority on IEEE profile vectors

## Description

Execute every QSpec 7d7943a TC-193 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_193_ieee_profiles.rs` (`make conformance`).

## Test Procedure

1. Check the evaluated and admission-only lists together equal the TC-193 census.
2. Compare value, outcome kind, charges and counters for arithmetic, rounding, non-finite,
   NaN-payload and rational-domain vectors on both sides.
3. Deny every admitted charge and compare the records; sweep generated operands per profile.

## Expected Results

33 vectors agree exactly; semantic admission of the IEEE definition is admission-only and named.
