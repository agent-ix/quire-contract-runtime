//! Decimal collection representation and charge schedules through the public kernel API.
#![cfg(feature = "exact")]

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;
use ix_trace_rs::trace;
use quire_exact::{
    construct_collection, form_collection, BoundViolation, CardinalityBound, ChargePoint,
    CheckedInvariantCause, CollectionKind, CollectionType, Component, ConstructionCause,
    ConstructionRefusal, Decimal, DecimalType, Deferred, Incomplete, Integer, LimitKind, Meter,
    Outcome, Refusal, RoundingMode, ScalarLimits, Undefined, Value, ValueType,
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
                Integer::from(-10000_i128),
                Integer::from(10000_i128),
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
    [(20_i128, 1), (110, 2), (11, 1), (2, 0), (1100, 3)]
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
        assert_eq!(empty.occ(), Integer::from(1_i128));
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
        assert_eq!(target.occ(), Integer::from(3_i128));
        assert_eq!(
            meter.admitted_charges().last().unwrap().value_occurrences,
            Some(3)
        );
    }
}

const KINDS: [CollectionKind; 4] = [
    CollectionKind::Sequence,
    CollectionKind::OrderedSet,
    CollectionKind::Set,
    CollectionKind::Bag,
];

fn bounded_type(kind: CollectionKind, bound: CardinalityBound) -> CollectionType {
    CollectionType::new(kind, collection_type(kind).element().clone(), Some(bound))
}

fn assert_consumption(meter: &Meter, work: u64, occurrences: u64, results: u64) {
    for kind in LimitKind::ALL {
        let expected = match kind {
            LimitKind::WorkUnits => work,
            LimitKind::ValueOccurrences => occurrences,
            LimitKind::ResultUnits => results,
            _ => 0,
        };
        assert_eq!(meter.consumed(kind), expected, "{kind:?}");
    }
}

fn membership_prefix(kind: CollectionKind) -> Vec<ChargePoint> {
    let mut points = Vec::new();
    if kind != CollectionKind::Sequence {
        // The five supplied occurrences compare 0, 1, 2, 1, 2 times.
        for _ in 0..6 {
            points.extend([ChargePoint::CollectionMemberWalk, ChargePoint::CollectionMemberTest]);
        }
    }
    points
}

// Partial FR-008-AC-3: completed input validation precedes all formation work.
#[test]
fn late_type_mismatch_precedes_membership_bound_and_zero_work_denial() {
    for kind in KINDS {
        for work_units in [0, u64::MAX] {
            let mut values = input();
            values[3] = Value::Integer(Integer::one());
            values[4] = Value::Boolean(true);
            let mut meter = Meter::new(ScalarLimits { work_units, ..UNLIMITED });
            let ty = bounded_type(kind, CardinalityBound::new(0, 0).unwrap());
            assert_eq!(
                form_collection(&ty, values, &mut meter),
                Err(ConstructionRefusal {
                    component: Component::Element(3),
                    cause: ConstructionCause::TypeMismatch,
                }),
                "{kind:?}, budget {work_units}",
            );
            assert!(meter.admitted_charges().is_empty());
            assert_eq!(meter.admission_count(), 0);
            assert_consumption(&meter, 0, 0, 0);
        }
    }
}

// Partial FR-008-AC-3/7: cardinality uses numeric coalescence, not Decimal representation.
#[test]
fn decimal_bound_refusals_report_distinct_or_occurrence_count_after_membership() {
    let bound = CardinalityBound::new(3, 3).unwrap();
    for (kind, count, violation, cause, work) in [
        (CollectionKind::Sequence, 5, BoundViolation::AboveMaximum, "above-maximum", 1),
        (CollectionKind::Bag, 5, BoundViolation::AboveMaximum, "above-maximum", 19),
        (CollectionKind::Set, 2, BoundViolation::BelowMinimum, "below-minimum", 19),
        (CollectionKind::OrderedSet, 2, BoundViolation::BelowMinimum, "below-minimum", 19),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = form_collection(&bounded_type(kind, bound), input(), &mut meter).unwrap();
        let expected = Refusal::CardinalityOutOfBound { violation, kind, bound, count };
        assert_eq!(outcome, Outcome::Refused(expected.clone()), "{kind:?}");
        let Outcome::Refused(refusal) = outcome else { panic!("expected cardinality refusal"); };
        assert_eq!(refusal.code(), Some("cardinality_out_of_bound"));
        assert_eq!(refusal.cause(), Some(cause));
        let mut points = membership_prefix(kind);
        points.push(ChargePoint::CollectionBound);
        assert_eq!(meter.admitted_charge_points(), points);
        assert_eq!(meter.admitted_charges().last().unwrap().value_occurrences, Some(count));
        assert_consumption(&meter, work, count, 0);

        // The same numeric count succeeds at both inclusive endpoints.
        let exact_bound = CardinalityBound::new(count, count).unwrap();
        let exact_type = bounded_type(kind, exact_bound);
        let mut boundary_meter = Meter::new(UNLIMITED);
        let value = form_collection(&exact_type, input(), &mut boundary_meter)
            .unwrap().completed().expect("exact cardinality boundary must complete");
        let Value::Collection(collection) = &value else { panic!("expected collection"); };
        assert_eq!(collection.collection_type(), &exact_type);
        assert_eq!(collection.elements().len(), usize::try_from(count).unwrap());
        assert_eq!(value.occ(), Integer::from(count + 1));
        points.push(ChargePoint::CollectionResultRetain);
        assert_eq!(boundary_meter.admitted_charge_points(), points);
        assert_consumption(&boundary_meter, work + 1, count + 1, count + 1);
    }
}

// Partial FR-008-AC-3: denial at the bound charge wins over an otherwise certain refusal.
#[test]
fn natural_bound_charge_denial_precedes_cardinality_refusal_without_retention() {
    for (kind, work, occurrences) in [
        (CollectionKind::Sequence, 0, 0),
        (CollectionKind::OrderedSet, 18, 1),
        (CollectionKind::Set, 18, 1),
        (CollectionKind::Bag, 18, 1),
    ] {
        // Each scalar comparison pays two walk units and one test unit:
        // six comparisons consume 18. The bound requests one more work unit.
        let mut meter = Meter::new(ScalarLimits { work_units: work, ..UNLIMITED });
        assert_eq!(
            form_collection(&bounded_type(kind, CardinalityBound::new(3, 3).unwrap()), input(), &mut meter).unwrap(),
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: work,
                consumed: work,
                next_charge: Integer::one(),
                charge_point: ChargePoint::CollectionBound,
            }),
            "{kind:?}",
        );
        assert_eq!(meter.admitted_charge_points(), membership_prefix(kind));
        assert_consumption(&meter, work, occurrences, 0);
    }
}

// Partial FR-008-AC-3: actual public Deferred callbacks observe charge-before-run,
// first-stopped propagation, and no eager coalescence of completed prefix elements.
#[test]
fn deferred_collection_stops_before_later_callbacks_or_formation() {
    use std::cell::Cell;

    for kind in KINDS {
        for stop in [
            Outcome::Undefined(Undefined::DivisionByZero),
            Outcome::Completed(Value::Boolean(true)),
        ] {
            let calls = Cell::new(0);
            let first = |meter: &mut Meter| {
                assert_eq!(meter.admitted_charge_points(), [ChargePoint::CollectionElement]);
                calls.set(1);
                Outcome::Completed(Value::Decimal(Decimal::new(Integer::from(20_i128), 1)))
            };
            let second = |meter: &mut Meter| {
                assert_eq!(calls.get(), 1);
                assert_eq!(meter.admitted_charge_points(), [ChargePoint::CollectionElement; 2]);
                calls.set(2);
                Outcome::Completed(Value::Decimal(Decimal::new(Integer::from(2_i128), 0)))
            };
            let expected = if matches!(stop, Outcome::Completed(_)) {
                Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::CollectionElementNotAdmitted })
            } else { stop.clone() };
            let third = |meter: &mut Meter| {
                assert_eq!(calls.get(), 2);
                assert_eq!(meter.admitted_charge_points(), [ChargePoint::CollectionElement; 3]);
                calls.set(3);
                stop
            };
            let later = |_: &mut Meter| -> Outcome<Value> { panic!("later callback must not run") };
            let elements: Vec<Deferred<'_>> = vec![Box::new(first), Box::new(second), Box::new(third), Box::new(later)];
            let mut meter = Meter::new(UNLIMITED);
            assert_eq!(construct_collection(&collection_type(kind), elements, &mut meter), expected);
            assert_eq!(calls.get(), 3);
            assert_eq!(meter.admitted_charge_points(), [ChargePoint::CollectionElement; 3]);
            assert_consumption(&meter, 3, 0, 0);
        }
    }
}
