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
feature, the runtime shall consume the authoritative `quire-exact` outcome and metering
contracts for every charge named by
`quire.value.accounting/v1` in agent-ix/quire-specification
(`proposals/quire-v1/definitions/value-accounting.md`). The runtime consumes the kernel
implementation of that definition: values and outcome kinds agree with the
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
- A `Refusal` carries a normative `refused { code }` spelling exactly where the language
  defines one. The language defines four — `ieee_nan_payload_not_representable`,
  `ieee_rational_out_of_domain`, `foreign_reference` (FR-149) and `cardinality_out_of_bound`
  (FR-272) — and `Refusal::code()` is `Some` for exactly those four variants and `None` for every
  other. `None` means "the language names no code for this refusal", not "this refusal has no
  reason": the typed variant is always the reason. An oracle that must emit `refused { code }` for
  a variant the language does not spell shall report the absence rather than invent a code, and
  the normative vocabulary remains an obligation when kernel evidence leaves RT.
- Each charge is decided before the work it pays for, and each size amount is derived before the
  value it measures is materialized. A denied charge consumes nothing and exposes no partial value.
- The kernel owns meter storage and accounting. Its bounded diagnostic charge log is exposed
  under `quire-exact`'s `test-support` feature. Until IR-349 removes the local meter, its
  admitted-charge sequence and `CHARGE_LOG_CAPACITY` remain governed by FR-011-AC-5. After
  that removal, RT shall consume the kernel's test-support diagnostic log rather than require
  a production log or define a local capacity constant. Consumed counters remain exact.
- Size counters are high-water marks; `work_units` and `result_units` are cumulative. The
  `Incomplete` record names the first unavailable counter in field order, its limit, the consumed
  amount before the charge, the exact denied amount (a mathematical integer) and the charge point.
- Result-domain membership is decided without a charge, after the arithmetic charges and before
  retention; an out-of-domain result is refused with no result unit.
- An injected denial at `(point, occurrence)` yields `Incomplete` on `work_units` at that point with
  counters unchanged by the denied charge. Its precedence over a real short counter, the contents
  of its record and how occurrences are counted are FR-010.
- What the meter holds at an `Undefined` or `Refused` stop, the atomicity of one charge, the
  field-order scan over a charge's own size vector, the value and truncation behavior of
  `CHARGE_LOG_CAPACITY`, and which derived amounts saturate are FR-011.
- The `exact` surface uses no host floating point, no `std`, no `unsafe` and no intentional panic
  path, and is re-exported from private modules only.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-006-AC-1 | The four outcome dispositions are distinct; completed `false` is a value; refusal, undefined and incomplete reasons are closed enums. | Test (TC-016) |
| FR-006-AC-2 | Ill-typed operand combinations are reported before evaluation with zero charges, and provenance-bearing refusals (invalid UTF-8 offset, stale identity) are typed. | Test (TC-020, TC-021, TC-022) |
| FR-006-AC-3 | Every charge point and limit kind round-trips its QSpec spelling; charges precede work; size counters are high-water, work/result cumulative; the first short counter in field order is reported with the exact denied amount. | Test (TC-016, TC-017) |
| FR-006-AC-4 | An injected denial at any admitted charge point yields `Incomplete` on `work_units` naming that point, with no result units and no partial value. | Test (TC-017) |
| FR-006-AC-6 | `Refusal::code()` is `Some` for exactly `IeeeNanPayloadNotRepresentable`, `IeeeRationalOutOfDomain`, `ForeignReference` and `CardinalityOutOfBound` with their normative spellings, and `None` for all nine other variants. | Test (TC-016) |

## Kernel ownership

The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar
operations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).
The criteria above remain obligations on that consumed behavior. They do not require RT-local
kernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately
from its disposition after the implementation removes the copy.

Owner references cover only matching subsets: `ix://agent-ix/quire-exact/FR-358` owns one-shot
injected denial and the bounded test-support log; `ix://agent-ix/quire-exact/FR-359` owns
cumulative-counter boundary refusal and atomicity; `ix://agent-ix/quire-exact/FR-361` owns the
specified allocation bounds on denied large work. These references do not establish the full
outcome/reason vocabulary, every charge spelling, or the ill-typed agreement claim. Unmapped
obligations remain explicit in TC-016/TC-017; no upstream verification status is inferred.

This amendment deletes no implementation or test. The remaining local kernel copy is IR-349
work; remaining RT evaluation-residue deletion is IR-583 (open backlog) work. QSL-358 is
Done and covered QSL-side extraction, not the remaining RT deletion. Neither copy is RT-owned
by remaining present. IR-430 is Done and covered removal of RT agreement tests; the remaining
quire-integration agreement work has **UNRESOLVED OPEN OWNER — planner ticket ID pending**.

## Dependencies

- **Upstream**: [FR-002](../../core/functional/FR-002-safe-operators.md);
  `ix://agent-ix/quire-specification` (`value-accounting.md`).
