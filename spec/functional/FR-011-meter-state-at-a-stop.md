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
- **Connective short-circuit orchestration is out of this crate's scope.** Deciding whether the
  right operand of `and`/`or`/`implies` runs at all, and propagating a right-operand stop, is
  `quire-contract-codegen`'s work over the short-circuit/total connective distinction
  `quire-contract-ir` represents in its model (`ix://agent-ix/quire-contract-ir/FR-014-AC-2`).
  `evaluate_boolean` never receives an operand that has not already stopped or decided: both
  operands arrive as plain, already-decided `bool`s, and the caller that produced each one pays for
  it. What this crate guarantees is narrower: `evaluate_boolean` admits `boolean.result-retain`
  exactly once per call, for any decided operand pair.
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
- **Cumulative counters are exact; derived amounts saturate.** `work_units` and `result_units` are
  added with a checked addition and a charge that would overflow one is denied, never wrapped. By
  contrast the amounts *derived* from operand shape — a `usize` length widened to `u64`, an
  exponent sum, a bit-length sum, and the injected-denial occurrence counter — saturate at
  `u64::MAX`. Saturation can only make a charge more conservative, i.e. more likely to be denied,
  and never admits a charge that the exact amount would have denied.
- **Every reader is total.** `Meter::consumed` answers for every `LimitKind` without a panic path;
  it is a total function over the enum and reports zero rather than failing for an index outside the
  ten counters, which the closed `LimitKind` vocabulary cannot produce.
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
| FR-011-AC-3 | `evaluate_boolean` admits exactly one `boolean.result-retain` charge per call, for every connective kind and every decided operand pair; orchestrating which operand runs and propagating a right-operand stop is `quire-contract-codegen`'s scope, not this crate's. | Test (TC-032) |
| FR-011-AC-4 | A charge whose second-scanned counter is short writes no counter, appends no log entry and advances no occurrence counter; and a charge presented with its size vector in either order reports the same first short counter in `ScalarLimitsV1` field order. | Test (TC-032) |
| FR-011-AC-5 | Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first 4096 points in admission order, `charge_log_truncated()` is true, and the counters are still exact and still enforced. | Test (TC-032) |
| FR-011-AC-6 | A cumulative counter at `u64::MAX - 1` denies rather than wraps; a derived amount that exceeds `u64::MAX` saturates and the resulting charge is denied rather than admitted. | Test (TC-032) |
| FR-011-AC-7 | `Meter::consumed` answers for all ten `LimitKind` members with no panic path, and IEEE flag iteration, dimension and compound-unit term iteration, and the `UnitGraph::admit` and `check_terms` refusal orders are the stated ones for every input permutation. | Test (TC-032) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  `ix://agent-ix/quire-specification` at `7d7943a`.
