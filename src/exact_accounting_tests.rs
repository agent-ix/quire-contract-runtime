// SPDX-License-Identifier: AGPL-3.0-or-later
//! In-crate meter tests for the counter semantics no public operator can reach.
//!
//! A public `exact` operator admits one work unit and at most two result units
//! per named charge, and `Charge`'s builders are crate-private, so the
//! cumulative-counter overflow boundary of FR-011-AC-6 is reachable only from
//! inside the crate. The same crate-private access is why IR-44's
//! check-before-mutate tests live here too: proving no counter moves across an
//! injected denial needs a charge that moves every counter at once, which
//! needs `Charge`'s crate-private builders to construct.

use core::num::NonZeroU64;

use super::{
    length_amount, Charge, ChargePoint, InjectedDenial, Integer, LimitKind, Meter, ScalarLimits,
};

const fn limits(work_units: u64, result_units: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units,
        result_units,
    }
}

/// Trace: TC-032, FR-011-AC-6
#[test]
fn tc_032_cumulative_work_units_deny_at_the_limit_rather_than_wrapping() {
    let mut meter = Meter::new(limits(u64::MAX, u64::MAX));
    let mut charge = Charge::new(ChargePoint::BooleanResultRetain);
    charge.work_units = Integer::from(u64::MAX - 1);
    assert!(meter.charge(charge).is_ok());
    assert_eq!(meter.consumed(LimitKind::WorkUnits), u64::MAX - 1);

    // One more admitted charge reaches the limit exactly.
    assert!(meter
        .charge(Charge::new(ChargePoint::BooleanResultRetain))
        .is_ok());
    assert_eq!(meter.consumed(LimitKind::WorkUnits), u64::MAX);

    // The next is denied, not wrapped.
    let denied = meter
        .charge(Charge::new(ChargePoint::BooleanResultRetain))
        .expect_err("a cumulative counter at its limit denies");
    assert_eq!(denied.limit_kind, LimitKind::WorkUnits);
    assert_eq!(denied.limit, u64::MAX);
    assert_eq!(denied.consumed, u64::MAX);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), u64::MAX);
}

/// Trace: TC-032, FR-011-AC-6
#[test]
fn tc_032_cumulative_result_units_deny_at_the_limit_rather_than_wrapping() {
    let mut meter = Meter::new(limits(u64::MAX, u64::MAX));
    let mut charge = Charge::new(ChargePoint::BooleanResultRetain);
    charge.result_units = Integer::from(u64::MAX);
    assert!(meter.charge(charge).is_ok());
    assert_eq!(meter.consumed(LimitKind::ResultUnits), u64::MAX);

    let denied = meter
        .charge(Charge::new(ChargePoint::BooleanResultRetain).results(1))
        .expect_err("a cumulative counter at its limit denies");
    assert_eq!(denied.limit_kind, LimitKind::ResultUnits);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), u64::MAX);
    // The denied charge is atomic: its own work unit was not written either.
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
}

/// A charge at `point` that would move every non-cumulative counter, plus
/// `work_units` and `result_units`, by a distinct nonzero amount derived from
/// `seed` if it were ever admitted. Used to make an injected denial's
/// check-before-mutate ordering observable: if any counter moved, this is how
/// the test would see it.
fn every_counter_charge(point: ChargePoint, seed: u64) -> Charge {
    let mut charge = Charge::new(point).work(Integer::from(seed.saturating_add(1)));
    for (offset, kind) in LimitKind::ALL.into_iter().enumerate() {
        if !kind.is_cumulative() {
            charge = charge.size(
                kind,
                seed.saturating_add(length_amount(offset)).saturating_add(1),
            );
        }
    }
    charge.results(seed.saturating_add(1))
}

/// Trace: TC-031, FR-010-AC-1 (IR-44).
///
/// `check_injected` decides an injected denial, and `charge`/`charge_plan`
/// return via `?` on it, strictly before either inspects or mutates any
/// counter. A consumer holding only the returned `Incomplete` cannot observe
/// this ordering directly from the `work_units` comparison alone:
/// `Incomplete.consumed` and any later `Meter::consumed(WorkUnits)` call read
/// the same array slot through the same accessor with nothing mutating
/// between them, so comparing `denied.consumed` against a reconstructed
/// pre-charge `work_units` value is `x == x` regardless of whether the seam
/// is check-before-mutate or mutate-then-check.
///
/// This test instead reads the meter's own counters directly, itself,
/// immediately before making the call that will be injected-denied, and
/// again immediately after — for every `LimitKind`, not only `work_units` —
/// so it can actually tell the two orderings apart. It also proves the
/// assertion is not vacuously true: after the single-shot seam has fired,
/// the same charge is admitted, and every counter it touches really does
/// move, so "unchanged before" is a property the charge could actually have
/// broken.
#[test]
fn tc_031_injected_denial_never_mutates_any_counter_before_returning() {
    let generous = limits(u64::MAX, u64::MAX);

    for point in ChargePoint::ALL {
        for prior_count in 0_u64..3 {
            let occurrence = NonZeroU64::MIN.saturating_add(prior_count);
            let mut meter =
                Meter::new(generous).with_injected_denial(InjectedDenial { point, occurrence });

            // Admit `prior_count` charges at `point` first, each moving every
            // counter, so the injected denial fires on exactly the
            // `occurrence`th charge at `point` (with every counter already
            // nonzero going into it once prior_count > 0).
            for prior in 0..prior_count {
                assert!(
                    meter.charge(every_counter_charge(point, prior)).is_ok(),
                    "generous limits admit every prior charge"
                );
            }
            let admitted_before = meter.admitted_charges().len();

            // The property under test, captured by the TEST ITSELF, by
            // reading the meter's counters directly — not recovered from an
            // `Incomplete` this call has not returned yet.
            let before: [u64; 10] = LimitKind::ALL.map(|kind| meter.consumed(kind));
            let work_before = meter.consumed(LimitKind::WorkUnits);

            let denied = meter
                .charge(every_counter_charge(point, 1_000))
                .expect_err("the injected denial fires on this charge");
            assert_eq!(denied.charge_point, point);
            assert_eq!(denied.limit_kind, LimitKind::WorkUnits);
            assert_eq!(denied.consumed, work_before);

            for (kind, expected) in LimitKind::ALL.into_iter().zip(before) {
                assert_eq!(
                    meter.consumed(kind),
                    expected,
                    "counter {kind:?} moved across an injected denial at {point:?} \
                     (occurrence {occurrence}); check_injected must return before any \
                     counter is inspected or mutated"
                );
            }
            assert_eq!(
                meter.admitted_charges().len(),
                admitted_before,
                "an injected denial must not append to the admitted-charge log"
            );

            // Control: the seam is single-shot, so the identical charge is now
            // admitted, and every counter it touches really does move. Without
            // this, the assertions above could pass vacuously if the stimulus
            // ever stopped being able to move these counters at all.
            assert!(
                meter.charge(every_counter_charge(point, 1_000)).is_ok(),
                "control: the same charge must be admitted once the single-shot seam has fired"
            );
            for (kind, before_value) in LimitKind::ALL.into_iter().zip(before) {
                assert_ne!(
                    meter.consumed(kind),
                    before_value,
                    "control: {kind:?} must be movable by this charge at {point:?}"
                );
            }
        }
    }
}

/// Trace: TC-031, FR-010-AC-1 (IR-44).
///
/// `charge_plan` (the FR-149 `equality.plan` charge) calls `check_injected`
/// through its own code path, separate from the generic `charge` exercised
/// above, and writes `value_occurrences` itself rather than through
/// `every_counter_charge`. That write is otherwise unguarded against an
/// injected denial anywhere in the tree: the generic-`charge`-based test
/// above drives `ChargePoint::EqualityPlan` through `charge`, not
/// `charge_plan`, so it never exercises this method's own ordering.
#[test]
fn tc_031_charge_plan_injected_denial_never_mutates_any_counter_before_returning() {
    let generous = limits(u64::MAX, u64::MAX);
    let point = ChargePoint::EqualityPlan;
    let mut meter = Meter::new(generous).with_injected_denial(InjectedDenial {
        point,
        occurrence: NonZeroU64::MIN,
    });

    let before: [u64; 10] = LimitKind::ALL.map(|kind| meter.consumed(kind));
    let work_before = meter.consumed(LimitKind::WorkUnits);
    let value_occurrences_before = meter.consumed(LimitKind::ValueOccurrences);

    let denied = meter
        .charge_plan(&Integer::from(1_000_u64))
        .expect_err("the injected denial fires on this charge_plan call");
    assert_eq!(denied.charge_point, point);
    assert_eq!(denied.limit_kind, LimitKind::WorkUnits);
    assert_eq!(denied.consumed, work_before);

    for (kind, expected) in LimitKind::ALL.into_iter().zip(before) {
        assert_eq!(
            meter.consumed(kind),
            expected,
            "counter {kind:?} moved across an injected denial in charge_plan; \
             check_injected must return before any counter is inspected or mutated"
        );
    }

    // Control: the seam is single-shot, so the identical call is now
    // admitted, and value_occurrences really does move.
    assert!(
        meter.charge_plan(&Integer::from(1_000_u64)).is_ok(),
        "control: the same charge_plan call must be admitted once the seam has fired"
    );
    assert_ne!(
        meter.consumed(LimitKind::ValueOccurrences),
        value_occurrences_before,
        "control: value_occurrences must be movable by this charge_plan call"
    );
}
