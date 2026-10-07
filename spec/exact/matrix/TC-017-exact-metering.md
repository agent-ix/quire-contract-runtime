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
the public meter. The former RT-local kernel tests left with the copied implementation.

## Ownership and evidence

Kernel-owned after IR-349 removes the local copy. `ix://agent-ix/quire-exact/FR-358-AC-1` and `FR-358-AC-2` own
named denial subsets; `FR-358-AC-8` through `FR-358-AC-11` own the post-cap test-support log;
`FR-359-AC-1` through `FR-359-AC-6` own cumulative-boundary and first-short-counter checks;
`FR-368-AC-1` and `FR-368-AC-2` own the spelling census.
The RT residue charge-point driver remains in `tests/exact_outcomes.rs` under TC-031.
No reference establishes the untested FR-010-AC-2/3 amount and precedence details.

## Test Procedure

1. Multiply `2^64 × 2^64` under `integer_bits` 128; expect `Incomplete` at
   `integer-arithmetic.arithmetic` with consumed 65 and denied amount 129.
2. Run two operations on one meter; check size counters keep the high-water mark and
   `work_units`/`result_units` accumulate.
3. Exhaust two counters at once; check the first in `ScalarLimitsV1` field order is reported.
4. Evaluate an out-of-domain result; check the refusal precedes retention and no result unit is
   charged.
5. Inject a denial at each admitted point; check the record and that counters are unchanged.
6. Admit more charges than `CHARGE_LOG_CAPACITY`; check the log is capped and marked truncated,
   counters stay exact, and an injected denial past the cap still fires.

## Expected Results

Every `Incomplete` record names the exact counter, limit, consumed amount, denied amount and
point; no denied charge consumes anything.
