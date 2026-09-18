---
id: FR-010
title: "Deny one named charge through the qualification seam"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-specification/NFR-071
    type: implements
---
# FR-010: Deny one named charge through the qualification seam

## Description

When a qualification harness must observe how an oracle behaves at a denied charge it cannot
provoke by setting a limit, the runtime shall accept one `InjectedDenial` naming a charge point and
an occurrence, and shall deny exactly that charge with a record whose contents are fixed and do not
depend on the configured `ScalarLimits`.

## Inputs

- At most one `InjectedDenial { point: ChargePoint, occurrence: u64 }` per `Meter`, with
  `occurrence` at least one.
- The `ScalarLimits` the meter is otherwise metering against.

## Outputs

- `Incomplete` for the matching charge, with `limit_kind = WorkUnits`, `charge_point` the injected
  point, `limit` and `consumed` both equal to the `work_units` consumed before the denied charge,
  and `next_charge` the denied charge's own `work_units` amount as a mathematical integer.
- No change to any counter, to the admitted-charge log, or to the meter's occurrence counter.

## Behavior

- The injected denial is decided **before** any counter is inspected. When a charge would also be
  denied by a genuinely short counter, the injected denial wins and the caller sees the injected
  record, not the real one. This precedence is what makes the seam deterministic: a harness that
  injects at a point gets the same record whatever limits the request was configured with.
- Because of that precedence the reported `limit` is **not** `ScalarLimits.work_units`. It is the
  work already spent, reported as if the `work_units` limit were exactly that, so that
  `limit = consumed` and the denied amount is the charge that would have exceeded it. A consumer
  must not read an injected `Incomplete` as evidence about the configured limits.
- The occurrence is 1-based and counts **admitted** charges at the injected point. The `n`th
  occurrence is the `n`th charge at that point that reaches the counters and is admitted; a charge
  at that point that a short counter denies does not advance the occurrence counter, and neither
  does the injected denial itself.
- Exactly one charge is denied per meter. Once the injected denial has fired, the meter's
  subsequent charges are metered normally against the configured limits.
- A `Meter` carries at most one `InjectedDenial`, and it names exactly one point. Charges at every
  other point are unaffected, including charges at other points interleaved with the injected one.
- The denied charge consumes nothing and exposes no partial value: FR-011's stop discipline applies
  unchanged to an injected `Incomplete`.
- The occurrence counter is saturating. Past `u64::MAX` admitted charges at the injected point no
  further occurrence is distinguishable; the counter does not wrap and no charge is mis-denied.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-010-AC-1 | For every admitted charge point, an injected denial at occurrence 1 yields `Incomplete` naming that point with `limit_kind = WorkUnits`, every counter unchanged, and no entry appended to the admitted-charge log. | Test (TC-031) |
| FR-010-AC-2 | The injected record's `limit` and `consumed` are both the `work_units` consumed before the charge and are independent of the configured `ScalarLimits.work_units`; `next_charge` is the denied charge's own work amount. | Test (TC-031) |
| FR-010-AC-3 | With limits short enough that the same charge would be denied on a real counter, the injected record is returned, not the real-counter record. | Test (TC-031) |
| FR-010-AC-4 | An injection at occurrence `n` fires on the `n`th admitted charge at that point, counting no charge at any other point and no charge at that point that a short counter denied. | Test (TC-031) |
| FR-010-AC-5 | After the injected denial fires, further charges are metered against the configured limits and no second charge is injected-denied. | Test (TC-031) |

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md), [FR-011](./FR-011-meter-state-at-a-stop.md);
  `ix://agent-ix/quire-specification` at `7d7943a`.

## Open defects

`occurrence == 0` is malformed — the field is 1-based — and today it silently matches no charge, so
a malformed request degrades into "no fault injected". That behaviour is not specified here and is
deliberately outside this requirement's admitted input domain. Tracked as
`agent-ix/quire-contract-runtime#20`.
