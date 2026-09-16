---
id: TC-019
title: "Agree with the authority on exact decimal vectors"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: verifies
---
# TC-019: Agree with the authority on exact decimal vectors

## Description

Execute every QSpec 5d88578 TC-185 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_185_exact_decimals.rs` (`make conformance`).

## Test Procedure

1. Check the evaluated list equals D01–D21.
2. For D01–D19, compare value, outcome kind, charges and counters on both sides, including every
   rounding mode and every denial.
3. For D20–D21, compare the ordering value against the authority's decimal comparison, and check
   the charges and counters against the QSpec 5d88578 `ordering.*` schedule only, including the
   `integer_bits` short at `ordering.arithmetic`.

## Expected Results

21 vectors agree in value and outcome kind. D20 and D21 charges are listed in
`CHARGES_PENDING_QSL_119` as a known upstream lag (quire-spec-language#119) and match QSpec.
