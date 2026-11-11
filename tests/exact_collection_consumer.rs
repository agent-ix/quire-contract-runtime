//! Decimal collection representation and charge schedules through the public kernel API.
#![cfg(feature = "exact")]

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;
use ix_trace_rs::trace;
use quire_exact::{
    compare_keys, construct_collection, form_collection, planned_equality, BoundViolation,
    CardinalityBound, ChargePoint, CheckedInvariantCause, CollectionKind, CollectionType,
    Component, ConstructionCause, ConstructionRefusal, Decimal, DecimalType, Deferred, Incomplete,
    InjectedDenial, Integer, LimitKind, Meter, OptionValue, Outcome, Refusal, RoundingMode,
    ScalarLimits, Undefined, Value, ValueType,
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

#[trace("FR-008-AC-12")]
#[test]
fn equality_planning_pairs_elements_in_stored_order() {
    let decimals = |pairs: [(i128, u32); 2]| -> Vec<Value> {
        pairs
            .into_iter()
            .map(|(c, s)| Value::Decimal(Decimal::new(Integer::from(c), s)))
            .collect()
    };
    // 2.0 sorts after 1.10 by canonical key, so the first input is descending.
    let descending = || decimals([(20, 1), (110, 2)]);
    let ascending = || decimals([(110, 2), (20, 1)]);
    assert_eq!(
        compare_keys(&descending()[0], &descending()[1]),
        Some(std::cmp::Ordering::Greater)
    );

    for (kind, expected) in [
        (CollectionKind::Sequence, false),
        (CollectionKind::OrderedSet, false),
        (CollectionKind::Set, true),
        (CollectionKind::Bag, true),
    ] {
        let left = form(kind, descending(), &mut Meter::new(UNLIMITED));
        let right = form(kind, ascending(), &mut Meter::new(UNLIMITED));
        let outcome = planned_equality(&left, &right, &mut Meter::new(UNLIMITED));
        assert!(
            matches!(outcome, Outcome::Completed(equal) if equal == expected),
            "{kind:?}: expected Completed({expected})"
        );
    }
}

// Callers explicitly enumerate
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
            points.extend([
                ChargePoint::CollectionMemberWalk,
                ChargePoint::CollectionMemberTest,
            ]);
        }
    }
    points
}

// Completed input validation precedes all formation work.
#[test]
fn late_type_mismatch_precedes_membership_bound_and_zero_work_denial() {
    for kind in KINDS {
        for work_units in [0, u64::MAX] {
            let mut values = input();
            values[3] = Value::Integer(Integer::one());
            values[4] = Value::Boolean(true);
            let mut meter = Meter::new(ScalarLimits {
                work_units,
                ..UNLIMITED
            });
            let ty = bounded_type(kind, CardinalityBound::new(0, 0).unwrap());
            assert_eq!(
                form_collection(&ty, values, &mut meter).unwrap_err(),
                ConstructionRefusal {
                    component: Component::Element(3),
                    cause: ConstructionCause::TypeMismatch,
                },
                "{kind:?}, budget {work_units}",
            );
            assert!(meter.admitted_charges().is_empty());
            assert_eq!(meter.admission_count(), 0);
            assert_consumption(&meter, 0, 0, 0);
        }
    }
}

// Cardinality uses numeric coalescence, not Decimal representation.
#[test]
fn decimal_bound_refusals_report_distinct_or_occurrence_count_after_membership() {
    let bound = CardinalityBound::new(3, 3).unwrap();
    for (kind, count, violation, cause, work) in [
        (
            CollectionKind::Sequence,
            5,
            BoundViolation::AboveMaximum,
            "above-maximum",
            1,
        ),
        (
            CollectionKind::Bag,
            5,
            BoundViolation::AboveMaximum,
            "above-maximum",
            19,
        ),
        (
            CollectionKind::Set,
            2,
            BoundViolation::BelowMinimum,
            "below-minimum",
            19,
        ),
        (
            CollectionKind::OrderedSet,
            2,
            BoundViolation::BelowMinimum,
            "below-minimum",
            19,
        ),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = form_collection(&bounded_type(kind, bound), input(), &mut meter).unwrap();
        let expected = Refusal::CardinalityOutOfBound {
            violation,
            kind,
            bound,
            count,
        };
        let Outcome::Refused(refusal) = outcome else {
            panic!("expected cardinality refusal");
        };
        assert_eq!(refusal, expected, "{kind:?}");
        assert_eq!(refusal.code(), Some("cardinality_out_of_bound"));
        assert_eq!(refusal.cause(), Some(cause));
        let mut points = membership_prefix(kind);
        points.push(ChargePoint::CollectionBound);
        assert_eq!(meter.admitted_charge_points(), points);
        assert_eq!(
            meter.admitted_charges().last().unwrap().value_occurrences,
            Some(count)
        );
        assert_consumption(&meter, work, count, 0);

        // The same numeric count succeeds at both inclusive endpoints.
        let exact_bound = CardinalityBound::new(count, count).unwrap();
        let exact_type = bounded_type(kind, exact_bound);
        let mut boundary_meter = Meter::new(UNLIMITED);
        let value = form_collection(&exact_type, input(), &mut boundary_meter)
            .unwrap()
            .completed()
            .expect("exact cardinality boundary must complete");
        let Value::Collection(collection) = &value else {
            panic!("expected collection");
        };
        assert_eq!(collection.collection_type(), &exact_type);
        assert_eq!(collection.elements().len(), usize::try_from(count).unwrap());
        assert_eq!(value.occ(), Integer::from(count + 1));
        points.push(ChargePoint::CollectionResultRetain);
        assert_eq!(boundary_meter.admitted_charge_points(), points);
        assert_consumption(&boundary_meter, work + 1, count + 1, count + 1);
    }
}

// Denial at the bound charge wins over an otherwise certain refusal.
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
        let mut meter = Meter::new(ScalarLimits {
            work_units: work,
            ..UNLIMITED
        });
        assert_stopped(
            form_collection(
                &bounded_type(kind, CardinalityBound::new(3, 3).unwrap()),
                input(),
                &mut meter,
            )
            .unwrap(),
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: work,
                consumed: work,
                next_charge: Integer::one(),
                charge_point: ChargePoint::CollectionBound,
            }),
        );
        assert_eq!(meter.admitted_charge_points(), membership_prefix(kind));
        assert_consumption(&meter, work, occurrences, 0);
    }
}

// Actual public Deferred callbacks observe charge-before-run,
// first-stopped propagation, and no eager coalescence of completed prefix elements.
#[test]
fn deferred_collection_stops_before_later_callbacks_or_formation() {
    use std::cell::Cell;

    for kind in KINDS {
        for stop in [
            Outcome::Undefined(Undefined::DivisionByZero),
            // Independently supplied callback terminal payloads: these measure
            // unchanged propagation, not the operation that produces each stop.
            Outcome::Refused(Refusal::CardinalityOutOfBound {
                violation: BoundViolation::AboveMaximum,
                kind: CollectionKind::Bag,
                bound: CardinalityBound::at_most(1),
                count: 2,
            }),
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 8,
                consumed: 7,
                next_charge: Integer::from(2_i128),
                charge_point: ChargePoint::CollectionMemberWalk,
            }),
            Outcome::Completed(Value::Boolean(true)),
        ] {
            let calls = Cell::new(0);
            let first = |meter: &mut Meter| {
                assert_eq!(
                    meter.admitted_charge_points(),
                    [ChargePoint::CollectionElement]
                );
                calls.set(1);
                Outcome::Completed(Value::Decimal(Decimal::new(Integer::from(20_i128), 1)))
            };
            let second = |meter: &mut Meter| {
                assert_eq!(calls.get(), 1);
                assert_eq!(
                    meter.admitted_charge_points(),
                    [ChargePoint::CollectionElement; 2]
                );
                calls.set(2);
                Outcome::Completed(Value::Decimal(Decimal::new(Integer::from(2_i128), 0)))
            };
            let expected = if matches!(stop, Outcome::Completed(_)) {
                Outcome::Refused(Refusal::CheckedInvariant {
                    cause: CheckedInvariantCause::CollectionElementNotAdmitted,
                })
            } else {
                stop.clone()
            };
            let third = |meter: &mut Meter| {
                assert_eq!(calls.get(), 2);
                assert_eq!(
                    meter.admitted_charge_points(),
                    [ChargePoint::CollectionElement; 3]
                );
                calls.set(3);
                stop
            };
            let later = |_: &mut Meter| -> Outcome<Value> { panic!("later callback must not run") };
            let elements: Vec<Deferred<'_>> = vec![
                Box::new(first),
                Box::new(second),
                Box::new(third),
                Box::new(later),
            ];
            let mut meter = Meter::new(UNLIMITED);
            assert_stopped(
                construct_collection(&collection_type(kind), elements, &mut meter),
                expected,
            );
            assert_eq!(calls.get(), 3);
            assert_eq!(
                meter.admitted_charge_points(),
                [ChargePoint::CollectionElement; 3]
            );
            assert_consumption(&meter, 3, 0, 0);
        }
    }
}

// The real element charge can stop before invoking a callback;
// the generous control reaches formation only after every callback completes.
#[test]
fn deferred_element_work_denial_preserves_completed_prefix_and_successful_control() {
    use std::cell::Cell;

    for kind in KINDS {
        for work_units in [2, u64::MAX] {
            let calls = Cell::new(0);
            let elements: Vec<Deferred<'_>> = input()
                .into_iter()
                .enumerate()
                .map(|(index, value)| {
                    let calls = &calls;
                    Box::new(move |meter: &mut Meter| {
                        assert_eq!(calls.get(), index);
                        assert_eq!(
                            meter.admitted_charge_points(),
                            vec![ChargePoint::CollectionElement; index + 1],
                        );
                        calls.set(index + 1);
                        Outcome::Completed(value)
                    }) as Deferred<'_>
                })
                .collect();
            let mut meter = Meter::new(ScalarLimits {
                work_units,
                ..UNLIMITED
            });
            let outcome = construct_collection(&collection_type(kind), elements, &mut meter);
            if work_units == 2 {
                assert_stopped(
                    outcome,
                    Outcome::Incomplete(Incomplete {
                        limit_kind: LimitKind::WorkUnits,
                        limit: 2,
                        consumed: 2,
                        next_charge: Integer::one(),
                        charge_point: ChargePoint::CollectionElement,
                    }),
                );
                assert_eq!(calls.get(), 2);
                assert_eq!(
                    meter.admitted_charge_points(),
                    [ChargePoint::CollectionElement; 2]
                );
                assert_consumption(&meter, 2, 0, 0);
            } else {
                let value = outcome
                    .completed()
                    .expect("all admitted callbacks complete");
                assert_eq!(calls.get(), 5);
                let (occurrences, work) = match kind {
                    CollectionKind::Sequence => (6_u64, 7),
                    CollectionKind::Bag => (6, 25),
                    CollectionKind::Set | CollectionKind::OrderedSet => (3, 25),
                };
                assert_eq!(value.occ(), Integer::from(occurrences));
                let mut points = vec![ChargePoint::CollectionElement; 5];
                points.extend(membership_prefix(kind));
                points.extend([
                    ChargePoint::CollectionBound,
                    ChargePoint::CollectionResultRetain,
                ]);
                assert_eq!(meter.admitted_charge_points(), points);
                assert_consumption(&meter, work, occurrences, occurrences);
            }
        }
    }
}

fn assert_stopped(actual: Outcome<Value>, expected: Outcome<Value>) {
    match (actual, expected) {
        (Outcome::Undefined(actual), Outcome::Undefined(expected)) => assert_eq!(actual, expected),
        (Outcome::Refused(actual), Outcome::Refused(expected)) => assert_eq!(actual, expected),
        (Outcome::Incomplete(actual), Outcome::Incomplete(expected)) => {
            assert_eq!(actual, expected)
        }
        (actual, expected) => panic!("expected stop {expected:?}, got {actual:?}"),
    }
}

// Late comparison denials retain only the comparisons admitted before the stop.
#[test]
fn late_membership_work_denials_preserve_exact_prefix_and_counters() {
    use ChargePoint::{CollectionMemberTest, CollectionMemberWalk};
    for kind in [
        CollectionKind::Set,
        CollectionKind::Bag,
        CollectionKind::OrderedSet,
    ] {
        for (work, point, next, prefix) in [
            (
                6,
                CollectionMemberWalk,
                2_i128,
                &[
                    CollectionMemberWalk,
                    CollectionMemberTest,
                    CollectionMemberWalk,
                    CollectionMemberTest,
                ][..],
            ),
            (
                8,
                CollectionMemberTest,
                1,
                &[
                    CollectionMemberWalk,
                    CollectionMemberTest,
                    CollectionMemberWalk,
                    CollectionMemberTest,
                    CollectionMemberWalk,
                ][..],
            ),
        ] {
            let mut meter = Meter::new(ScalarLimits {
                work_units: work,
                ..UNLIMITED
            });
            assert_stopped(
                form_collection(&collection_type(kind), input(), &mut meter).unwrap(),
                Outcome::Incomplete(Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: work,
                    consumed: work,
                    next_charge: Integer::from(next),
                    charge_point: point,
                }),
            );
            assert_eq!(
                meter.admitted_charge_points(),
                prefix,
                "{kind:?}, {point:?}"
            );
            assert_eq!(
                meter.admission_count(),
                u64::try_from(prefix.len()).unwrap()
            );
            assert_consumption(&meter, work, 1, 0);
        }
    }
}

// A successful bound charge remains admitted when result retention cannot run.
#[test]
fn result_retention_denials_keep_bound_count_without_a_partial_value() {
    for (kind, work, count, retain_occ) in [
        (CollectionKind::Sequence, 1, 5, 6_u64),
        (CollectionKind::OrderedSet, 19, 2, 3),
        (CollectionKind::Set, 19, 2, 3),
        (CollectionKind::Bag, 19, 5, 6),
    ] {
        for (limits, limit_kind, limit, consumed, next) in [
            (
                ScalarLimits {
                    work_units: work,
                    ..UNLIMITED
                },
                LimitKind::WorkUnits,
                work,
                work,
                1,
            ),
            (
                ScalarLimits {
                    result_units: 0,
                    ..UNLIMITED
                },
                LimitKind::ResultUnits,
                0,
                0,
                retain_occ,
            ),
        ] {
            let mut meter = Meter::new(limits);
            let ty = bounded_type(kind, CardinalityBound::new(count, count).unwrap());
            assert_stopped(
                form_collection(&ty, input(), &mut meter).unwrap(),
                Outcome::Incomplete(Incomplete {
                    limit_kind,
                    limit,
                    consumed,
                    next_charge: Integer::from(next),
                    charge_point: ChargePoint::CollectionResultRetain,
                }),
            );
            let mut prefix = membership_prefix(kind);
            prefix.push(ChargePoint::CollectionBound);
            assert_eq!(meter.admitted_charge_points(), prefix);
            assert_eq!(
                meter.admission_count(),
                u64::try_from(prefix.len()).unwrap()
            );
            assert_eq!(
                meter.admitted_charges().last().unwrap().value_occurrences,
                Some(count)
            );
            // A denied mixed charge must not raise the size high-water to retain_occ,
            // even when only its later-scanned result-units counter is unavailable.
            assert_consumption(&meter, work, count, 0);
        }
    }
}

fn construct_decimal_input(
    kind: CollectionKind,
    meter: &mut Meter,
    calls: &std::cell::Cell<usize>,
) -> Outcome<Value> {
    let elements: Vec<Deferred<'_>> = input()
        .into_iter()
        .map(|value| {
            Box::new(move |_: &mut Meter| {
                calls.set(calls.get() + 1);
                Outcome::Completed(value)
            }) as Deferred<'_>
        })
        .collect();
    construct_collection(&collection_type(kind), elements, meter)
}

// The real test-support seam denies a named occurrence. Independent counter and
// point expectations are also checked against a successful run's complete charge records.
#[test]
fn injected_collection_denials_admit_only_the_generous_controls_prefix() {
    use std::{cell::Cell, num::NonZeroU64};
    use ChargePoint::{
        CollectionBound, CollectionElement, CollectionMemberTest, CollectionMemberWalk,
        CollectionResultRetain,
    };

    for kind in KINDS {
        let calls = Cell::new(0);
        let mut generous = Meter::new(UNLIMITED);
        let value = construct_decimal_input(kind, &mut generous, &calls)
            .completed()
            .expect("generous control completes");
        let (retained_occ, bound_count, full_work) = match kind {
            CollectionKind::Sequence => (6_u64, 5, 7),
            CollectionKind::Bag => (6, 5, 25),
            CollectionKind::Set | CollectionKind::OrderedSet => (3, 2, 25),
        };
        assert_eq!(value.occ(), Integer::from(retained_occ));
        assert_eq!(calls.get(), 5);
        let mut complete_points = vec![CollectionElement; 5];
        complete_points.extend(membership_prefix(kind));
        complete_points.extend([CollectionBound, CollectionResultRetain]);
        assert_eq!(generous.admitted_charge_points(), complete_points);
        assert_consumption(&generous, full_work, retained_occ, retained_occ);

        let (bound_prefix, bound_work, bound_occ) = if kind == CollectionKind::Sequence {
            (5, 5, 0)
        } else {
            (17, 23, 1)
        };
        for (point, occurrence, prefix_len, work, occurrences, next, expected_calls) in [
            (CollectionElement, 3, 2, 2, 0, 1_i128, 2),
            (CollectionMemberWalk, 3, 9, 11, 1, 2, 5),
            (CollectionMemberTest, 3, 10, 13, 1, 1, 5),
            (
                CollectionBound,
                1,
                bound_prefix,
                bound_work,
                bound_occ,
                1,
                5,
            ),
            (
                CollectionResultRetain,
                1,
                bound_prefix + 1,
                bound_work + 1,
                bound_count,
                1,
                5,
            ),
        ] {
            if kind == CollectionKind::Sequence
                && matches!(point, CollectionMemberWalk | CollectionMemberTest)
            {
                continue; // Sequences perform no membership comparisons.
            }
            calls.set(0);
            let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
                point,
                occurrence: NonZeroU64::new(occurrence).unwrap(),
            });
            assert_stopped(
                construct_decimal_input(kind, &mut meter, &calls),
                Outcome::Incomplete(Incomplete {
                    limit_kind: LimitKind::WorkUnits,
                    limit: work,
                    consumed: work,
                    next_charge: Integer::from(next),
                    charge_point: point,
                }),
            );
            assert_eq!(calls.get(), expected_calls, "{kind:?}, {point:?}");
            assert_eq!(
                meter.admitted_charge_points(),
                complete_points[..prefix_len]
            );
            assert_eq!(
                meter.admitted_charges(),
                &generous.admitted_charges()[..prefix_len]
            );
            assert_eq!(meter.admission_count(), u64::try_from(prefix_len).unwrap());
            assert_consumption(&meter, work, occurrences, 0);
        }
    }
}

fn integer_option(payload: Option<i128>) -> Value {
    match payload {
        None => OptionValue::none(ValueType::Integer),
        Some(value) => {
            OptionValue::present(ValueType::Integer, Value::Integer(Integer::from(value))).unwrap()
        }
    }
}

fn wrap_singleton(value: Value) -> Value {
    let ty = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::option(ValueType::Integer),
        None,
    );
    form_collection(&ty, vec![value], &mut Meter::new(UNLIMITED))
        .unwrap()
        .completed()
        .expect("singleton option sequence completes")
}

// This ranks None/Present, including payload order, without claiming a composite Null slot.
#[test]
fn option_keys_rank_none_before_present_directly_and_inside_sequences() {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let expected = [
        [Equal, Less, Less],
        [Greater, Equal, Less],
        [Greater, Greater, Equal],
    ];
    for nested in [false, true] {
        let values: Vec<Value> = [None, Some(-1), Some(2)]
            .into_iter()
            .map(|payload| {
                let value = integer_option(payload);
                if nested {
                    wrap_singleton(value)
                } else {
                    value
                }
            })
            .collect();
        for (left_index, left) in values.iter().enumerate() {
            for (right_index, right) in values.iter().enumerate() {
                assert_eq!(
                    compare_keys(left, right),
                    Some(expected[left_index][right_index]),
                    "nested={nested}, pair={left_index}/{right_index}"
                );
            }
        }
    }
}

#[test]
fn set_and_bag_sort_direct_and_nested_options_with_literal_multiplicities() {
    for nested in [false, true] {
        for (kind, expected) in [
            (CollectionKind::Set, &[None, Some(-1_i128), Some(2)][..]),
            (
                CollectionKind::Bag,
                &[None, None, Some(-1), Some(2), Some(2)][..],
            ),
        ] {
            let options = [Some(2), None, Some(-1), None, Some(2)];
            let values = options
                .into_iter()
                .map(|payload| {
                    let value = integer_option(payload);
                    if nested {
                        wrap_singleton(value)
                    } else {
                        value
                    }
                })
                .collect();
            let element = ValueType::option(ValueType::Integer);
            let element = if nested {
                ValueType::collection(CollectionType::new(CollectionKind::Sequence, element, None))
            } else {
                element
            };
            let ty = CollectionType::new(kind, element, None);
            let value = form_collection(&ty, values, &mut Meter::new(UNLIMITED))
                .unwrap()
                .completed()
                .expect("option collection completes");
            let Value::Collection(collection) = value else {
                panic!("expected outer collection");
            };
            assert_eq!(collection.elements().len(), expected.len());
            for (member, expected_payload) in collection.elements().iter().zip(expected) {
                let member = if nested {
                    let Value::Collection(singleton) = member else {
                        panic!("expected singleton sequence");
                    };
                    assert_eq!(singleton.collection_type().kind(), CollectionKind::Sequence);
                    assert_eq!(singleton.elements().len(), 1);
                    &singleton.elements()[0]
                } else {
                    member
                };
                let Value::Option(option) = member else {
                    panic!("expected option member");
                };
                assert_eq!(option.payload_type(), &ValueType::Integer);
                match (option.payload(), expected_payload) {
                    (None, None) => {}
                    (Some(Value::Integer(actual)), Some(expected)) => {
                        assert_eq!(actual, &Integer::from(*expected))
                    }
                    (actual, expected) => {
                        panic!("expected option payload {expected:?}, got {actual:?}")
                    }
                }
            }
        }
    }
}
