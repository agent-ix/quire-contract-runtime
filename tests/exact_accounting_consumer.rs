// SPDX-License-Identifier: AGPL-3.0-or-later
//! RT consumer checks for the public exact meter contract.
#![cfg(feature = "exact")]

use quire_exact::{Charge, ChargePoint, Incomplete, Integer, LimitKind, Meter, ScalarLimits};

const UNLIMITED: ScalarLimits = ScalarLimits {
    integer_bits: u64::MAX,
    decimal_digits: u64::MAX,
    scale_expansion: u64::MAX,
    text_input_bytes: u64::MAX,
    text_scalars: u64::MAX,
    normalized_scalars: u64::MAX,
    unit_edges: u64::MAX,
    value_occurrences: u64::MAX,
    work_units: u64::MAX,
    result_units: u64::MAX,
};

/// The point labels these public meter requests; this test checks aggregation,
/// independent of a particular scalar operation's charge schedule.
const POINT: ChargePoint = ChargePoint::IntegerArithmeticOperands;

#[test]
fn consumed_size_counters_keep_high_water_while_work_and_results_accumulate() {
    for kind in LimitKind::ALL {
        if kind.is_cumulative() {
            continue;
        }
        let mut meter = Meter::new(UNLIMITED);
        meter.charge(Charge::new(POINT).size(kind, 7)).unwrap();
        meter.charge(Charge::new(POINT).size(kind, 3)).unwrap();
        meter.charge(Charge::new(POINT).size(kind, 9)).unwrap();
        meter.charge(Charge::new(POINT).size(kind, 4)).unwrap();
        assert_eq!(meter.consumed(kind), 9, "{kind:?} must keep its maximum");
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 4);
        assert_eq!(meter.admission_count(), 4);
    }

    let mut meter = Meter::new(UNLIMITED);
    for (work, results) in [(2_i64, 1_u64), (3, 2), (1, 1)] {
        meter
            .charge(
                Charge::new(POINT)
                    .work(Integer::from(work))
                    .results(results),
            )
            .unwrap();
    }
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 6);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 4);
    assert_eq!(meter.admission_count(), 3);
}

#[test]
fn first_short_counter_uses_field_order_and_exact_denied_amount() {
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: 128,
        value_occurrences: 1,
        work_units: 1,
        result_units: 0,
        ..UNLIMITED
    });
    meter
        .charge(Charge::new(POINT).size(LimitKind::IntegerBits, 65))
        .unwrap();

    let before = LimitKind::ALL.map(|kind| meter.consumed(kind));
    let denied = meter.charge(
        Charge::new(POINT)
            .size(LimitKind::ValueOccurrences, 2)
            .size(LimitKind::IntegerBits, 129)
            .work(Integer::from(2_i64))
            .results(1),
    );
    assert_eq!(
        denied,
        Err(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 128,
            consumed: 65,
            next_charge: Integer::from(129_i64),
            charge_point: POINT,
        })
    );
    assert_eq!(LimitKind::ALL.map(|kind| meter.consumed(kind)), before);
    assert_eq!(meter.admission_count(), 1);

    let denied = meter.charge(
        Charge::new(POINT)
            .size(LimitKind::ValueOccurrences, 2)
            .size(LimitKind::IntegerBits, 128),
    );
    assert_eq!(
        denied,
        Err(Incomplete {
            limit_kind: LimitKind::ValueOccurrences,
            limit: 1,
            consumed: 0,
            next_charge: Integer::from(2_i64),
            charge_point: POINT,
        })
    );
    assert_eq!(LimitKind::ALL.map(|kind| meter.consumed(kind)), before);
    assert_eq!(meter.admission_count(), 1);

    let denied = meter.charge(Charge::new(POINT).work(Integer::from(2_i64)).results(1));
    assert_eq!(
        denied,
        Err(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 1,
            consumed: 1,
            next_charge: Integer::from(2_i64),
            charge_point: POINT,
        })
    );
    assert_eq!(LimitKind::ALL.map(|kind| meter.consumed(kind)), before);

    let denied = meter.charge(Charge::new(POINT).results(1));
    assert_eq!(
        denied,
        Err(Incomplete {
            limit_kind: LimitKind::ResultUnits,
            limit: 0,
            consumed: 0,
            next_charge: Integer::from(1_i64),
            charge_point: POINT,
        })
    );
    assert_eq!(LimitKind::ALL.map(|kind| meter.consumed(kind)), before);
    assert_eq!(meter.admission_count(), 1);
}
