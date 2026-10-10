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

When a qualification harness must observe how an RT exact-feature oracle behaves at a denied
charge it cannot provoke by setting a limit, the runtime shall use the `quire-exact` meter's
`InjectedDenial` seam. The kernel owns the injected record and ordinary-limit precedence under
`ix://agent-ix/quire-exact/FR-358-AC-12` and `ix://agent-ix/quire-exact/FR-358-AC-13`;
RT owns the charge-point drivers for its remaining operations.

## Inputs

- At most one `InjectedDenial { point: ChargePoint, occurrence: NonZeroU64 }` per `Meter`.
  `occurrence` is 1-based; `NonZeroU64` makes the malformed 0-based request unrepresentable.
- The `ScalarLimits` the meter is otherwise metering against.

## Outputs

- The kernel's `Incomplete` for the matching charge, as specified by
  `ix://agent-ix/quire-exact/FR-358-AC-12`.
- No change to any counter, to the admitted-charge log, or to the meter's occurrence counter.

## Behavior

- The kernel decides injected-versus-ordinary-limit precedence under
  `ix://agent-ix/quire-exact/FR-358-AC-13`. RT passes its charge requests to that meter.
- The occurrence is 1-based and counts **admitted** charges at the injected point. The `n`th
  occurrence is the `n`th charge at that point that reaches the counters and is admitted; a charge
  at that point that a short counter denies does not advance the occurrence counter, and neither
  does the injected denial itself.
- Exactly one charge is denied per meter. Once the injected denial has fired, the meter's
  subsequent charges are metered normally against the configured limits.
- A `Meter` carries at most one `InjectedDenial`, and it names exactly one point. Charges at every
  other point are unaffected, including charges at other points interleaved with the injected one.
- The `equality.plan` reservation recorded by an injected denial is defined by
  `ix://agent-ix/quire-exact/FR-358-AC-12`.
- The denied charge consumes nothing and exposes no partial value: FR-011's stop discipline applies
  unchanged to an injected `Incomplete`.
- The occurrence counter is saturating. Past `u64::MAX` admitted charges at the injected point no
  further occurrence is distinguishable; the counter does not wrap and no charge is mis-denied.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-010-AC-1 | For every charge point that an RT-owned residue operation can issue through the `quire-exact` meter, an occurrence-1 injected denial returns `Incomplete` with `WorkUnits` and that point; the observed work consumption equals the record's `consumed`, result consumption is zero, and the denied point is absent from the admitted-charge log. `ix://agent-ix/quire-exact/FR-358-AC-1` owns full meter-state atomicity. | Test (TC-031); Test (owner TC-906) |
| FR-010-AC-2 | A denial reached through an RT exact-feature charge-point driver returns the `quire-exact` record specified and directly tested by `ix://agent-ix/quire-exact/FR-358-AC-12`; RT adds no second record definition. | Test (owner TC-906); Inspection (RT dependency) |
| FR-010-AC-3 | An RT exact-feature charge-point driver uses the `quire-exact` injection-versus-ordinary-limit precedence specified and directly tested by `ix://agent-ix/quire-exact/FR-358-AC-13`; RT adds no second precedence rule. | Test (owner TC-906); Inspection (RT dependency) |
| FR-010-AC-4 | An injection at occurrence `n` fires on the `n`th admitted charge at that point, counting no charge at any other point and no charge at that point that a short counter denied. | Test (TC-031) |
| FR-010-AC-5 | After the injected denial fires, further charges are metered against the configured limits and no second charge is injected-denied. | Test (TC-031) |
| FR-010-AC-6 | A 0-based `occurrence` cannot be constructed, so an injected denial can never silently match no charge and degrade into "no fault injected". | Test (TC-031) |

## Kernel ownership

The behaviour above that `quire-exact` implements (interface-001, "Exact kernel surface") is the
behaviour of that one kernel, which the runtime consumes and does not copy ([FR-275](./FR-275-single-exact-kernel.md)).
This requirement stays in force and binds the `exact` feature as a whole; where its evidence
moves to the kernel's repository the matrix says so and keeps the row, without deleting any
acceptance criterion.

`ix://agent-ix/quire-exact/FR-358-AC-1` tests named admitted occurrences and one-shot
behavior; `ix://agent-ix/quire-exact/FR-358-AC-2` tests denial and retry at `equality.plan`;
`ix://agent-ix/quire-exact/FR-358-AC-3` fixes the nonzero occurrence type.
`ix://agent-ix/quire-exact/FR-358-AC-12` and `ix://agent-ix/quire-exact/FR-358-AC-13`
directly test the complete injected record and ordinary-limit precedence at the public meter.
RT's retained TC-031 driver checks its residue charge points against that meter. The RT lockfile
still resolves an older `quire-exact` revision; refreshing that dependency is separate from this
spec transfer and is required before claiming current RT consumes the newly evidenced owner
revision.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md), [FR-011](./FR-011-meter-state-at-a-stop.md);
  `ix://agent-ix/quire-specification`.
