// SPDX-License-Identifier: AGPL-3.0-or-later
//! In-crate meter tests for the counter semantics no public operator can reach.
//!
//! A public `exact` operator admits one work unit and at most two result units
//! per named charge, and `Charge`'s builders are crate-private, so the
//! cumulative-counter overflow boundary of FR-011-AC-6 is reachable only from
//! inside the crate.

use super::{Charge, ChargePoint, LimitKind, Meter, ScalarLimits};

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
