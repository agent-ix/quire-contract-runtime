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

Execute every QSpec 7d7943a TC-185 vector on the runtime and on quire-spec-language d9d5273.
Evidence: `conformance/qsl-agreement/tests/tc_185_exact_decimals.rs` (`make conformance`) and the
upscale allocation bound in `tests/exact_allocation.rs` (`--features exact`).

## Test Procedure

1. Check the evaluated list equals D01–D23.
2. For every vector with an authority run, compare value and outcome kind on both sides; for
   vectors whose charges the authority meters, also compare charges, counters and every injected
   denial.
3. For the vectors in `CHARGES_PENDING_QSL_119`, check the runtime charges and counters against the
   QSpec 7d7943a schedule only, at the exact limit tuple and one under the first short counter.
   D20–D21 short `integer_bits` at `ordering.arithmetic`; D22–D23 charge the result-retain upscale
   before materialization, and D23 has no authority run.
4. Check each operand-derived decimal amount exact and one under: add alignment, subtract
   cancellation, multiply, divide, negate with rounding and retain upscale.
5. Deny the digits charge of a `2^20` scale upscale and check no allocation reaches 4096 bytes.

## Expected Results

23 vectors agree in value and outcome kind. D09, D13 and D20–D23 charges are listed in
`CHARGES_PENDING_QSL_119` as a known upstream lag (quire-spec-language#119) and match QSpec.
