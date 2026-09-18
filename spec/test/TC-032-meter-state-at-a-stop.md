---
id: TC-032
title: "Read a determinate meter state at every stop"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-011
    type: verifies
---
# TC-032: Read a determinate meter state at every stop

## Description

Check what the meter holds at `Undefined`, `Refused` and `Incomplete` stops, charge atomicity, the
field-order scan, the bounded log, saturation and the exposed deterministic orders. Evidence:
`tests/exact_meter_state.rs` (`--features exact`).

## Test Procedure

1. Divide by zero; check the operands charge is consumed and the arithmetic charge is not.
2. Produce an out-of-domain result; check the arithmetic charge is consumed and no result unit is.
3. Deny a charge; check no counter, log entry or occurrence counter moved.
4. Evaluate a quantity `power` with zero base and negative exponent, and a divide by zero that also
   satisfies it; check the reported cause and its precedence.
5. Evaluate each connective with a stopping right operand and with a short-circuiting left operand;
   check the retain charge is admitted exactly when the result is decided by the connective.
6. Present one charge's size vector in both orders under two short counters; check the same
   `ScalarLimitsV1`-field-order counter is reported and that nothing was written.
7. Admit more than `CHARGE_LOG_CAPACITY` charges; check the log holds the first 4096 in order, is
   marked truncated, and the counters stay exact and enforced.
8. Drive a cumulative counter to `u64::MAX - 1` and a derived amount past `u64::MAX`; check the
   charge is denied, not wrapped.
9. Read `consumed` for all ten `LimitKind` members; iterate IEEE flags, dimension terms and
   compound-unit terms built in several orders; drive `UnitGraph::admit` and `check_terms` into
   inputs that fail more than one check.

## Expected Results

A stop is never a rollback and never free; a denied charge is atomic; every exposed order is the
stated one for every permutation.
