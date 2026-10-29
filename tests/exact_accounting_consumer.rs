// SPDX-License-Identifier: AGPL-3.0-or-later
//! RT consumer checks for the public exact meter contract.
#![cfg(feature = "exact")]

use quire_exact::{Charge, ChargePoint, Incomplete, Integer, LimitKind, Meter, ScalarLimits};
use quire_specification::VALUE_ACCOUNTING_LIMIT_KIND_VECTORS;

#[test]
fn consumed_kernel_limit_kinds_match_authoritative_scalar_limits_vocabulary() {
    let vectors: serde_json::Value = serde_json::from_str(VALUE_ACCOUNTING_LIMIT_KIND_VECTORS)
        .expect("QSpec limit-kind vectors are JSON");
    let declared = vectors["limit_kinds"]
        .as_array()
        .expect("QSpec limit kinds are an array");
    let declared: Vec<&str> = declared
        .iter()
        .map(|entry| entry.as_str().expect("QSpec limit kind is a string"))
        .collect();
    let kernel: Vec<&str> = LimitKind::ALL.iter().map(|kind| kind.as_str()).collect();

    assert_eq!(
        kernel, declared,
        "kernel counters must match ScalarLimitsV1 field order"
    );
    for (&kind, &spelling) in LimitKind::ALL.iter().zip(&declared) {
        assert_eq!(LimitKind::from_code(spelling), Some(kind));
    }

    let undeclared = vectors["undeclared"]
        .as_array()
        .expect("QSpec undeclared spellings are an array");
    assert!(!undeclared.is_empty(), "QSpec has negative spellings");
    for entry in undeclared {
        let spelling = entry.as_str().expect("QSpec negative spelling is a string");
        assert!(
            !declared.contains(&spelling),
            "negative spelling {spelling:?} is declared"
        );
        assert_eq!(
            LimitKind::from_code(spelling),
            None,
            "{spelling:?} must be refused"
        );
    }
}

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
