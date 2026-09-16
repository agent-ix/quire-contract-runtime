---
id: TC-017
title: "Meter charges before work and deny them without effect"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: verifies
---
# TC-017: Meter charges before work and deny them without effect

## Description

Check charge ordering, counter semantics, first-short-counter reporting and injected denials on
the public meter. Evidence: `tests/exact_outcomes.rs` (`--features exact`).

## Test Procedure

1. Multiply `2^64 × 2^64` under `integer_bits` 128; expect `Incomplete` at
   `integer-arithmetic.arithmetic` with consumed 65 and denied amount 129.
2. Run two operations on one meter; check size counters keep the high-water mark and
   `work_units`/`result_units` accumulate.
3. Exhaust two counters at once; check the first in `ScalarLimitsV1` field order is reported.
4. Evaluate an out-of-domain result; check the refusal precedes retention and no result unit is
   charged.
5. Inject a denial at each admitted point; check the record and that counters are unchanged.

## Expected Results

Every `Incomplete` record names the exact counter, limit, consumed amount, denied amount and
point; no denied charge consumes anything.
