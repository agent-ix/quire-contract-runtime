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
field-order scan, the bounded log, exact over-`u64` charge requests and the exposed deterministic orders. RT evidence:
`tests/exact_meter_state.rs` and `tests/exact_quantity_conversion.rs` (`--features exact`);
kernel and QSV owner evidence supplies the removed implementation's remaining controls.

## Ownership and evidence

Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure
when the left decides, propagating each right-operand stop unchanged with no retention, and
retaining a completed connective result exactly once. Current evidence is
`tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above
(FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires.
The already-decided truth-table test is kernel-owned under
[quire-exact FR-362-AC-9](ix://agent-ix/quire-exact/FR-362), which does not test lazy closure invocation.
`FR-362-AC-11` covers scalar stop prefixes; `FR-359-AC-6` and `FR-359-AC-7` cover scan order
and consumed readers; `FR-358-AC-8` through `FR-358-AC-11` cover post-cap accounting;
`FR-359-AC-1` through `FR-359-AC-5` cover cumulative boundaries and atomicity; and
`FR-364-AC-1` covers IEEE flag order. Steps 1–3 and 6–8 and the kernel part of step 9 left
with the kernel copy. QSV owns quantity and unit graph behavior after the IR-882 cutover;
RT's step-4 integration check uses QSV directly. The lazy connective remains RT-owned.
QSL-358's QSL-side extraction is Done.

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
8. Under a cumulative limit of `u64::MAX`, admit one unit from `u64::MAX - 1` and deny the next
   unit without changing meter state. Request a quantity power charge past `u64::MAX`; check
   `Incomplete::next_charge` equals the exact requested amount, the power was not computed, and
   the denied charge admitted nothing and changed no counter or log entry. QSpec TC-187 U13
   supplies the `36893488147419103232` quantity-power vector; kernel FR-359-AC-3/4 own exact
   over-`u64` refusal and atomicity.
9. Read `consumed` for all ten `LimitKind` members; iterate IEEE flags and QSV dimension and
   compound-unit terms built in several orders; drive QSV's checked-node constructors and
   `UnitGraph::from_checked_nodes` into inputs that fail more than one check.

## Expected Results

A stop is never a rollback and never free; a denied charge is atomic; every exposed order is the
stated one for every permutation.
