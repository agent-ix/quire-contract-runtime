---
id: FR-011
title: "Expose a determinate meter state at every stop"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-044
    type: implements
---
# FR-011: Expose a determinate meter state at every stop

## Description

When an evaluation stops without a completed value, the runtime shall leave the `Meter` in a state
the caller can read and rely on: FR-006 states that a *denied* charge consumes nothing, but an
oracle consumer also needs to know what the meter holds when the stop is `Undefined` or `Refused`,
and what the meter holds at all is what this requirement fixes.

## Inputs

- Any `Meter` that has admitted zero or more charges, at the moment an evaluation returns
  `Outcome::Undefined`, `Outcome::Refused` or `Outcome::Incomplete`.

## Outputs

- `Meter::consumed(kind)` for each of the ten `ScalarLimitsV1` counters.
- `Meter::admitted_charges()` and `Meter::charge_log_truncated()`.

## Behavior

- **A stop is not a rollback.** Every charge admitted before the stop stays consumed and stays in
  the admitted-charge log. The four dispositions differ only in what the *stopping* step costs:
  - `Incomplete` — the denied charge itself consumed nothing and appended nothing.
  - `Undefined` — the operation has no mathematical value, and the charges already paid for reading
    and sizing the operands stay consumed. An `Undefined` is therefore never free.
  - `Refused` — the result exists but is not admitted. Every charge up to and including the
    arithmetic that produced it stays consumed, and the result-retain charge is never admitted, so
    a refusal costs no result unit.
- **Undefinedness is detected at a stated point, not at an unstated one.** A divisor is tested for
  zero after the operation's `*.operands` charge — whose size is derived from the divisor's bit
  length, so the charge must precede the test — and before its `*.arithmetic` charge. An
  `Undefined::DivisionByZero` therefore leaves exactly the operands charge consumed.
- A quantity `power` with a zero base and a negative exponent is `Undefined::DivisionByZero`: the
  value is `1/0^|n|`, a division by zero, and the vocabulary carries no separate cause for it.
  Zero-divisor and zero-base-under-negative-exponent are tested in that order, first match wins.
- **A Boolean connective's stop propagates unchanged.** When the right operand stops, the
  connective returns that stop verbatim and admits no `boolean.result-retain` charge, so a stopped
  connective costs no result unit. The left operand enters as an already-decided `bool` and is
  charged for by whoever produced it, never by the connective.
- **One charge is all-or-nothing.** Within a single charge every semantic size is checked against
  its limit and both cumulative counters are checked for availability before any counter is
  written. A charge that fails any check writes no counter, appends no log entry and advances no
  occurrence counter.
- **The first short counter is found in `ScalarLimitsV1` field order over the whole charge.** A
  charge's own size vector is sorted by `LimitKind` field-order index before it is scanned, so the
  reported counter does not depend on the order in which the evaluator happened to attach the
  sizes, and every semantic-size counter is scanned before `work_units` and `result_units`.
- **The admitted-charge log is a bounded diagnostic, not accounting.** It holds the first
  `CHARGE_LOG_CAPACITY` admitted points in admission order, `CHARGE_LOG_CAPACITY` is 4096, and
  `charge_log_truncated()` is true once a charge was admitted that the log could not hold. Past the
  cap every counter stays exact and every limit is still enforced.
- **Cumulative counters are bounded; charge requests are exact.** `work_units` and `result_units`
  are `u64` counters. An addition that exceeds a configured limit or `u64::MAX` is denied before
  either counter changes. Amounts derived from operand shape remain mathematical integers in the
  charge request, even above `u64::MAX`; a denied charge reports that exact amount as
  `Incomplete::next_charge`, with no admission or partial meter mutation. For quantity power, the
  `unit.rational-arithmetic` request is `max(1, |exponent| × maxparts(base))` before the power is
  computed. The injected-denial occurrence counter has its separate rule in FR-010.
- **Every reader is total over its closed domain.** `Meter::consumed` answers without panic for
  every member of `LimitKind::ALL`; untouched counters read as zero, semantic-size counters as
  their admitted high-water values, and work/result counters as their admitted cumulative totals.
- **Every exposed order is deterministic and stated.** IEEE flags iterate in `IeeeFlag::ALL`
  vocabulary order; `Dimension` and `CompoundUnit` terms iterate ascending by node key;
  `UnitGraph::admit` checks node well-formedness, then duplicate keys, then graph topology, and its
  refusal names the first failed check; `check_terms` refuses zero exponent, then duplicate term,
  then unsorted terms, in that order. No exposed order depends on a hash, an address or an
  insertion order the caller cannot see.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-011-AC-1 | For each of `Undefined`, `Refused` and `Incomplete`, the meter after the stop holds exactly the charges admitted before it: an `Undefined` division by zero retains the operands charge and no arithmetic charge; a refused result retains the arithmetic charge and no result unit; a denied charge retains neither. | Test (TC-032) |
| FR-011-AC-2 | A quantity `power` with zero base and negative exponent is `Undefined::DivisionByZero`, and a divide by zero is reported in preference to it when both hold. | Test (TC-032) |
| FR-011-AC-3 | A lazy connective skips its right operand when the left decides the result; for every connective kind and every decided operand pair, its completed result admits exactly one `boolean.result-retain` charge. | Test (TC-032) |
| FR-011-AC-4 | A charge whose second-scanned counter is short writes no counter, appends no log entry and advances no occurrence counter; and a charge presented with its size vector in either order reports the same first short counter in `ScalarLimitsV1` field order. | Test (TC-032) |
| FR-011-AC-5 | Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first 4096 points in admission order, `charge_log_truncated()` is true, and the counters are still exact and still enforced. | Test (TC-032) |
| FR-011-AC-6 | With a cumulative limit of `u64::MAX`, a counter at `u64::MAX - 1` admits one unit to reach the limit and denies the next unit without wrapping or changing any meter state. A quantity power whose `unit.rational-arithmetic` request exceeds `u64::MAX` returns `Incomplete` before computing the power; `next_charge` equals the exact request, and the denied charge changes no counter, admission count or admitted-charge log. | Test (TC-032) |
| FR-011-AC-7 | `Meter::consumed` answers without panic for every member of `LimitKind::ALL`, returning zero for untouched counters, admitted high-water values for semantic sizes and admitted cumulative totals for work/results; IEEE flag iteration, dimension and compound-unit term iteration, and the `UnitGraph::admit` and `check_terms` refusal orders are the stated ones for every input permutation. | Test (TC-032) |
| FR-011-AC-8 | A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit. | Test (TC-032) |

## Kernel ownership

The behaviour above that `quire-exact` implements (interface-001, "Exact kernel surface") is the
behaviour of that one kernel, which the runtime consumes and does not copy ([FR-275](./FR-275-single-exact-kernel.md)).
This requirement stays in force and binds the `exact` feature as a whole; where its evidence
moves to the kernel's repository the matrix says so and keeps the row, without deleting any
acceptance criterion.

`ix://agent-ix/quire-exact/FR-362-AC-11` covers scalar stop prefixes;
`FR-359-AC-6` and `FR-359-AC-7` cover meter scan order and consumed readers;
`FR-358-AC-8` through `FR-358-AC-11` cover the bounded test-support log after its cap;
`FR-359-AC-1` through `FR-359-AC-5` cover cumulative boundaries, exact over-`u64` charge
requests and atomicity; and
`FR-364-AC-1` covers IEEE flag order. These do not cover the RT quantity stop in AC-2,
the lazy invocation and stop propagation in AC-3/AC-8, or RT dimension, compound-unit and
graph orders in AC-7. The RT portions remain required and retain their own evidence in TC-032.
In particular, kernel `FR-359-AC-3`/`FR-359-AC-4` require exact over-`u64` `next_charge`
values and `FR-359-AC-7` covers `consumed` over `LimitKind::ALL`. QSpec
`ix://agent-ix/quire-specification/TC-187` vector U13 requires the exact
`next_charge = 36893488147419103232` for quantity power before computation. RT's retained
`tests/exact_meter_state.rs` over-`u64` case checks the quantity stop, a non-`u64` amount and the
admitted-charge prefix; `src/exact/quantity.rs` constructs the exact request.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification`.
