//! Decimal collection representation and charge schedules through the public kernel API.
#![cfg(feature = "exact")]

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;
use ix_trace_rs::trace;
use quire_exact::{
    form_collection, ChargePoint, CollectionKind, CollectionType, Decimal, DecimalType, Integer,
    Meter, Outcome, RoundingMode, ScalarLimits, Value, ValueType,
};

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

fn collection_type(kind: CollectionKind) -> CollectionType {
    CollectionType::new(
        kind,
        ValueType::Decimal(
            DecimalType::new(
                Integer::from(-10000),
                Integer::from(10000),
                0,
                3,
                RoundingMode::Exact,
            )
            .unwrap(),
        ),
        None,
    )
}

fn input() -> Vec<Value> {
    [(20, 1), (110, 2), (11, 1), (2, 0), (1100, 3)]
        .into_iter()
        .map(|(coefficient, scale)| Value::Decimal(Decimal::new(Integer::from(coefficient), scale)))
        .collect()
}

fn form(kind: CollectionKind, input: Vec<Value>, meter: &mut Meter) -> Value {
    let Outcome::Completed(value) = form_collection(&collection_type(kind), input, meter).unwrap()
    else {
        panic!("collection must complete for {kind:?}");
    };
    value
}

fn assert_representations(value: &Value, expected: &[(i128, u32)]) {
    let Value::Collection(collection) = value else {
        panic!("expected collection");
    };
    assert_eq!(collection.elements().len(), expected.len());
    for (value, &(coefficient, scale)) in collection.elements().iter().zip(expected) {
        let Value::Decimal(decimal) = value else {
            panic!("expected decimal member");
        };
        assert_eq!(
            decimal.representation().coefficient(),
            &Integer::from(coefficient)
        );
        assert_eq!(decimal.representation().scale(), scale);
    }
}

#[trace("FR-008-AC-11")]
#[test]
fn retained_decimal_representations_follow_each_collection_kind() {
    for (kind, expected) in [
        (
            CollectionKind::Sequence,
            &[(20, 1), (110, 2), (11, 1), (2, 0), (1100, 3)][..],
        ),
        (CollectionKind::OrderedSet, &[(20, 1), (110, 2)][..]),
        (CollectionKind::Set, &[(110, 2), (20, 1)][..]),
        (
            CollectionKind::Bag,
            &[(110, 2), (110, 2), (110, 2), (20, 1), (20, 1)][..],
        ),
    ] {
        let value = form(kind, input(), &mut Meter::new(UNLIMITED));
        assert_representations(&value, expected);
    }
}

#[trace("FR-008-AC-10")]
#[test]
fn coalescence_charges_each_comparison_before_bound_and_retention() {
    use ChargePoint::{
        CollectionBound, CollectionMemberTest, CollectionMemberWalk, CollectionResultRetain,
    };
    // Candidate comparisons are independently 0, 1, 2, 1, 2.
    let coalescing = [
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionMemberWalk,
        CollectionMemberTest,
        CollectionBound,
        CollectionResultRetain,
    ];
    for (kind, expected_points, expected_occ) in [
        (
            CollectionKind::Sequence,
            &[CollectionBound, CollectionResultRetain][..],
            6,
        ),
        (CollectionKind::OrderedSet, &coalescing[..], 3),
        (CollectionKind::Set, &coalescing[..], 3),
        (CollectionKind::Bag, &coalescing[..], 6),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        let value = form(kind, input(), &mut meter);
        assert_eq!(meter.admitted_charge_points(), expected_points, "{kind:?}");
        assert_eq!(value.occ(), Integer::from(expected_occ));
        assert_eq!(
            meter.admitted_charges().last().unwrap().value_occurrences,
            Some(expected_occ)
        );

        let mut empty_meter = Meter::new(UNLIMITED);
        let empty = form(kind, Vec::new(), &mut empty_meter);
        assert_representations(&empty, &[]);
        assert_eq!(empty.occ(), Integer::from(1));
        assert_eq!(
            empty_meter.admitted_charge_points(),
            [CollectionBound, CollectionResultRetain]
        );
        assert_eq!(
            empty_meter
                .admitted_charges()
                .last()
                .unwrap()
                .value_occurrences,
            Some(1)
        );
    }
}

// Partial formation-visiting clause of FR-008-AC-12: callers explicitly enumerate
// the public retained slice. This does not observe equality-planner visitation.
#[test]
fn further_formation_consumes_retained_order_and_repeated_occurrences() {
    use ChargePoint::{
        CollectionBound, CollectionMemberTest, CollectionMemberWalk, CollectionResultRetain,
    };
    for (source_kind, expected, comparisons) in [
        (CollectionKind::Sequence, &[(20, 1), (110, 2)][..], 6),
        (CollectionKind::OrderedSet, &[(20, 1), (110, 2)][..], 1),
        (CollectionKind::Set, &[(110, 2), (20, 1)][..], 1),
        (CollectionKind::Bag, &[(110, 2), (20, 1)][..], 5),
    ] {
        let source = form(source_kind, input(), &mut Meter::new(UNLIMITED));
        let Value::Collection(collection) = &source else {
            panic!("expected collection");
        };
        let mut meter = Meter::new(UNLIMITED);
        let target = form(
            CollectionKind::OrderedSet,
            collection.elements().to_vec(),
            &mut meter,
        );
        assert_representations(&target, expected);
        let mut expected_points = Vec::new();
        for _ in 0..comparisons {
            expected_points.extend([CollectionMemberWalk, CollectionMemberTest]);
        }
        expected_points.extend([CollectionBound, CollectionResultRetain]);
        assert_eq!(
            meter.admitted_charge_points(),
            expected_points,
            "{source_kind:?}"
        );
        assert_eq!(target.occ(), Integer::from(3));
        assert_eq!(
            meter.admitted_charges().last().unwrap().value_occurrences,
            Some(3)
        );
    }
}
