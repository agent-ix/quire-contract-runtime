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

## Ownership and evidence

Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure
when the left decides, propagating each right-operand stop unchanged with no retention, and
retaining a completed connective result exactly once. Current evidence is
`tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above
(FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires.
The already-decided truth-table test is kernel-owned; `ix://agent-ix/quire-exact/FR-362`
does not test lazy closure invocation. Steps 1–3 and 6–8 and the kernel part of step 9 leave
with the kernel copy; the quantity/environment parts of steps 4 and 9 remain RT evaluation residue
owned by open backlog IR-583 until removed; QSL-358's QSL-side extraction is Done. This amendment changes no test or implementation.

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
