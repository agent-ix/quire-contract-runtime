---
id: FR-006
title: "Return typed exact outcomes under metered scalar accounting"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: depends_on
---
# FR-006: Return typed exact outcomes under metered scalar accounting

## Description

When a generated oracle evaluates a complete-V1 scalar operator through the optional `exact`
feature, the runtime shall return one typed outcome and meter every charge named by
`quire.value.accounting/v1` at agent-ix/quire-specification@7d7943a
(`proposals/quire-v1/definitions/value-accounting.md`). The runtime is an implementation of that
definition, not a second semantic authority: values and outcome kinds agree with the pinned
quire-spec-language authority (FR-007), and charge schedules are taken from the QSpec definition.

## Inputs

- Operands of one exact scalar family, a declared result domain where the family has one, and a
  `ScalarLimits` tuple with the ten counters in `ScalarLimitsV1` field order.
- Optionally, one injected denial naming a charge point and a 1-based occurrence.

## Outputs

- `Outcome<T>`: `Completed(T)`, `Undefined(Undefined)`, `Refused(Refusal)` or
  `Incomplete(Incomplete)`, plus the admitted charge sequence and consumed counters on the `Meter`.
- Ill-typed operand combinations are reported before evaluation, beside the outcome, as `IllTyped`.

## Behavior

- A completed `false` is a value, never a refusal. `Undefined`, `Refused` and `Incomplete` carry
  closed, typed reasons; no variant carries a message string.
- Each charge is decided before the work it pays for, and each size amount is derived before the
  value it measures is materialized. A denied charge consumes nothing and exposes no partial value.
- The meter's memory is bounded: the admitted-charge log is capped at `CHARGE_LOG_CAPACITY` and
  reports truncation, while every counter stays exact.
- Size counters are high-water marks; `work_units` and `result_units` are cumulative. The
  `Incomplete` record names the first unavailable counter in field order, its limit, the consumed
  amount before the charge, the exact denied amount (a mathematical integer) and the charge point.
- Result-domain membership is decided without a charge, after the arithmetic charges and before
  retention; an out-of-domain result is refused with no result unit.
- An injected denial at `(point, occurrence)` yields `Incomplete` on `work_units` at that point with
  counters unchanged by the denied charge.
- The `exact` surface uses no host floating point, no `std`, no `unsafe` and no intentional panic
  path, and is re-exported from private modules only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | The four outcome dispositions are distinct; completed `false` is a value; refusal, undefined and incomplete reasons are closed enums. | Test (TC-016) |
| FR-006-AC-2 | Ill-typed operand combinations are reported before evaluation with zero charges, and provenance-bearing refusals (invalid UTF-8 offset, stale identity) are typed. | Test (TC-020, TC-021, TC-022) |
| FR-006-AC-3 | Every charge point and limit kind round-trips its QSpec 7d7943a spelling; charges precede work; size counters are high-water, work/result cumulative; the first short counter in field order is reported with the exact denied amount. | Test (TC-016, TC-017) |
| FR-006-AC-4 | An injected denial at any admitted charge point yields `Incomplete` on `work_units` naming that point, with no result units and no partial value. | Test (TC-017) |
| FR-006-AC-5 | Exact sources contain no host float, `std`, `unsafe` or panic path, and the public surface equals the private-module re-export set. | Test (TC-016) |

## Dependencies

- **Upstream**: [FR-002](./FR-002-safe-operators.md);
  `ix://agent-ix/quire-specification` at `7d7943a` (`value-accounting.md`).
