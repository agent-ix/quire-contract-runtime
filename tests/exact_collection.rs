//! FR-144 collection algebra, membership metering and the canonical key,
//! through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::RefCell;
use std::rc::Rc;

use quire_contract_runtime::exact::{
    construct_collection, form_collection, BoundViolation, CardinalityBound, ChargePoint,
    CollectionKind, CollectionType, Component, CompositeDeclaration, CompositeShape,
    ConstructionCause, ConstructionRefusal, Deferred, FieldDeclaration, FieldValue, Incomplete,
    InjectedDenial, Integer, LimitKind, Meter, NodeKey, Outcome, Presence, Refusal, ScalarLimits,
    TypeEnvironment, Undefined, Value, ValueType,
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

fn key(byte: u8) -> NodeKey {
    NodeKey::from_bytes([byte; 32])
}

fn consumed(meter: &Meter) -> Vec<u64> {
    LimitKind::ALL
        .iter()
        .map(|kind| meter.consumed(*kind))
        .collect()
}

fn int_value(n: i128) -> Value {
    Value::Integer(Integer::from(n))
}

fn charges_of(meter: &Meter, point: ChargePoint) -> usize {
    meter
        .admitted_charges()
        .iter()
        .filter(|admitted| **admitted == point)
        .count()
}

/// Trace: TC-025, FR-008-AC-3
#[test]
fn tc_025_p1_construct_collection_charges_element_before_running_and_stops_at_first_stop() {
    for kind in [
        CollectionKind::Sequence,
        CollectionKind::Set,
        CollectionKind::Bag,
        CollectionKind::OrderedSet,
    ] {
        let collection_type = CollectionType::new(
            kind,
            ValueType::Integer,
            CardinalityBound::new(0, 10).unwrap(),
        );
        let ran: Rc<RefCell<Vec<i128>>> = Rc::new(RefCell::new(Vec::new()));
        let mut meter = Meter::new(UNLIMITED);
        let elements: Vec<Deferred<'_>> = (0_i128..3)
            .map(|index| {
                let ran = Rc::clone(&ran);
                Box::new(move |meter: &mut Meter| {
                    assert_eq!(
                        meter.admitted_charges().last(),
                        Some(&ChargePoint::CollectionElement),
                        "collection.element must be charged before this element runs"
                    );
                    ran.borrow_mut().push(index);
                    if index == 1 {
                        Outcome::Undefined(Undefined::DivisionByZero)
                    } else {
                        Outcome::Completed(int_value(index))
                    }
                }) as Deferred<'_>
            })
            .collect();
        let outcome = construct_collection(&collection_type, elements, &mut meter);
        assert!(matches!(
            outcome,
            Outcome::Undefined(Undefined::DivisionByZero)
        ));
        // The third element, declared after the first non-completing one, never ran.
        assert_eq!(*ran.borrow(), vec![0, 1]);
        assert_eq!(charges_of(&meter, ChargePoint::CollectionElement), 2);
    }
}

/// Trace: TC-025, FR-008-AC-3
#[test]
fn tc_025_p2_form_collection_refuses_an_ineligible_occurrence_before_any_charge() {
    let collection_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Integer,
        CardinalityBound::new(0, 10).unwrap(),
    );
    let mut meter = Meter::new(UNLIMITED);
    let refusal = form_collection(
        &collection_type,
        vec![int_value(1), Value::Boolean(true)],
        &mut meter,
    )
    .unwrap_err();
    assert_eq!(
        refusal,
        ConstructionRefusal {
            component: Component::Element(1),
            cause: ConstructionCause::TypeMismatch,
        }
    );
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: TC-025, FR-008-AC-3
#[test]
fn tc_025_p3_set_and_bag_membership_charges_and_result_retain() {
    let bound = CardinalityBound::new(0, 10).unwrap();
    let cases: [(CollectionKind, Vec<i128>); 2] = [
        (CollectionKind::Set, vec![1, 2, 3]),
        (CollectionKind::Bag, vec![1, 1, 2, 3]),
    ];
    for (kind, expected_elements) in cases {
        let collection_type = CollectionType::new(kind, ValueType::Integer, bound);
        let mut meter = Meter::new(UNLIMITED);
        // Occurrences with repeats, deliberately out of canonical order.
        let occurrences = vec![int_value(1), int_value(2), int_value(1), int_value(3)];
        let outcome = form_collection(&collection_type, occurrences, &mut meter)
            .unwrap()
            .completed()
            .unwrap();
        let Value::Collection(collection) = outcome else {
            panic!("expected a collection value")
        };
        let observed: Vec<i128> = collection
            .elements()
            .iter()
            .map(|value| match value {
                Value::Integer(integer) => i128::from(integer.to_u64().unwrap()),
                other => panic!("expected an integer element, got {other:?}"),
            })
            .collect();
        assert_eq!(observed, expected_elements, "kind {kind:?}");

        // One member-walk and one member-test per comparison against members
        // retained so far, stopping at the first equal member: 0 + 1 + 1 + 2 = 4.
        assert_eq!(charges_of(&meter, ChargePoint::CollectionMemberWalk), 4);
        assert_eq!(charges_of(&meter, ChargePoint::CollectionMemberTest), 4);

        let charges = meter.admitted_charges();
        assert_eq!(
            charges.get(charges.len() - 2),
            Some(&ChargePoint::CollectionBound)
        );
        assert_eq!(charges.last(), Some(&ChargePoint::CollectionResultRetain));
    }
}

/// Trace: TC-025, FR-008-AC-7
#[test]
fn tc_025_p4_cardinality_bound_violations_are_typed_and_distinct() {
    let bound = CardinalityBound::new(2, 3).unwrap();
    let collection_type = CollectionType::new(CollectionKind::Sequence, ValueType::Integer, bound);

    let mut meter = Meter::new(UNLIMITED);
    let outcome = form_collection(&collection_type, vec![int_value(1)], &mut meter).unwrap();
    let Outcome::Refused(refusal) = outcome else {
        panic!("expected a refusal, got {outcome:?}")
    };
    assert_eq!(refusal.code(), Some("cardinality_out_of_bound"));
    assert_eq!(refusal.cause(), Some("below-minimum"));
    match refusal {
        Refusal::CardinalityOutOfBound {
            violation,
            kind,
            bound: reported_bound,
            count,
        } => {
            assert_eq!(violation, BoundViolation::BelowMinimum);
            assert_eq!(kind, CollectionKind::Sequence);
            assert_eq!(reported_bound, bound);
            assert_eq!(count, 1);
        }
        other => panic!("expected CardinalityOutOfBound, got {other:?}"),
    }
    // `collection.bound` precedes the uncharged bound check; nothing follows it.
    assert_eq!(
        meter.admitted_charges().last(),
        Some(&ChargePoint::CollectionBound)
    );

    let mut meter = Meter::new(UNLIMITED);
    let occurrences = vec![int_value(1), int_value(2), int_value(3), int_value(4)];
    let outcome = form_collection(&collection_type, occurrences, &mut meter).unwrap();
    let Outcome::Refused(refusal) = outcome else {
        panic!("expected a refusal, got {outcome:?}")
    };
    assert_eq!(refusal.code(), Some("cardinality_out_of_bound"));
    assert_eq!(refusal.cause(), Some("above-maximum"));
    match refusal {
        Refusal::CardinalityOutOfBound {
            violation, count, ..
        } => {
            assert_eq!(violation, BoundViolation::AboveMaximum);
            assert_eq!(count, 4);
        }
        other => panic!("expected CardinalityOutOfBound, got {other:?}"),
    }
    assert_eq!(
        meter.admitted_charges().last(),
        Some(&ChargePoint::CollectionBound)
    );
}

/// Trace: TC-025, FR-008-AC-4
#[test]
fn tc_025_p5_canonical_order_ranks_absent_null_and_present_and_is_input_order_independent() {
    let wrapper = CompositeDeclaration::new(
        key(1),
        "Wrapper",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "v",
            ValueType::Integer,
            Presence::Optional,
        )]),
    );
    let env = TypeEnvironment::new([wrapper], []).unwrap();
    let present = env
        .record(key(1), vec![("v", FieldValue::Present(int_value(5)))])
        .unwrap();
    let null = env.record(key(1), vec![("v", FieldValue::Null)]).unwrap();
    let absent = env.record(key(1), Vec::new()).unwrap();

    let collection_type = CollectionType::new(
        CollectionKind::Set,
        ValueType::Composite(key(1)),
        CardinalityBound::new(0, 10).unwrap(),
    );

    let rank_of = |value: &Value| -> u8 {
        let Value::Composite(composite) = value else {
            panic!("expected a composite element")
        };
        match composite.slots().first().unwrap() {
            FieldValue::Absent => 0,
            FieldValue::Null => 1,
            FieldValue::Present(_) => 2,
        }
    };
    let ranks_of = |elements: &[Value]| -> Vec<u8> { elements.iter().map(rank_of).collect() };

    let mut meter_a = Meter::new(UNLIMITED);
    let a = form_collection(
        &collection_type,
        vec![present.clone(), null.clone(), absent.clone()],
        &mut meter_a,
    )
    .unwrap()
    .completed()
    .unwrap();
    let Value::Collection(a) = a else {
        panic!("expected a collection")
    };
    assert_eq!(ranks_of(a.elements()), vec![0, 1, 2]);

    // The same members in a different input order still sort ascending.
    let mut meter_b = Meter::new(UNLIMITED);
    let b = form_collection(&collection_type, vec![absent, present, null], &mut meter_b)
        .unwrap()
        .completed()
        .unwrap();
    let Value::Collection(b) = b else {
        panic!("expected a collection")
    };
    assert_eq!(ranks_of(b.elements()), ranks_of(a.elements()));
}

/// Trace: TC-025, FR-008-AC-4 (the iterative task-stack walk)
#[test]
fn tc_025_p6_canonical_key_comparison_at_depth_has_no_stack_overflow() {
    const DEPTH: u64 = 100_000;
    let node = CompositeDeclaration::new(
        key(2),
        "Node",
        CompositeShape::Record(vec![
            FieldDeclaration::new("child", ValueType::Composite(key(2)), Presence::Optional),
            FieldDeclaration::new("tag", ValueType::Integer, Presence::Required),
        ]),
    );
    let env = TypeEnvironment::new([node], []).unwrap();
    let build_chain = |base_tag: i128| -> Value {
        let mut current = env
            .record(
                key(2),
                vec![
                    ("child", FieldValue::Absent),
                    ("tag", FieldValue::Present(int_value(base_tag))),
                ],
            )
            .unwrap();
        for _ in 1..DEPTH {
            current = env
                .record(
                    key(2),
                    vec![
                        ("child", FieldValue::Present(current)),
                        ("tag", FieldValue::Present(int_value(0))),
                    ],
                )
                .unwrap();
        }
        current
    };
    // Two chains, identical at every depth except the deepest node's tag, so
    // the comparison must walk to the bottom before it finds a difference.
    let a = build_chain(0);
    let b = build_chain(1);
    let Value::Composite(a_rc) = &a else {
        panic!("expected a composite")
    };
    let Value::Composite(b_rc) = &b else {
        panic!("expected a composite")
    };
    let a_ptr = Rc::as_ptr(a_rc);
    let b_ptr = Rc::as_ptr(b_rc);

    let collection_type = CollectionType::new(
        CollectionKind::Set,
        ValueType::Composite(key(2)),
        CardinalityBound::new(0, 4).unwrap(),
    );
    let mut meter = Meter::new(UNLIMITED);
    let outcome = form_collection(&collection_type, vec![b.clone(), a.clone()], &mut meter)
        .unwrap()
        .completed()
        .unwrap();
    let Value::Collection(collection) = &outcome else {
        panic!("expected a collection")
    };
    let elements = collection.elements();
    assert_eq!(elements.len(), 2);
    let first_ptr = match &elements[0] {
        Value::Composite(rc) => Rc::as_ptr(rc),
        other => panic!("expected a composite element, got {other:?}"),
    };
    let second_ptr = match &elements[1] {
        Value::Composite(rc) => Rc::as_ptr(rc),
        other => panic!("expected a composite element, got {other:?}"),
    };
    assert_eq!(first_ptr, a_ptr, "the tag=0 chain must sort before tag=1");
    assert_eq!(second_ptr, b_ptr);

    // The canonical key's task-stack walk this exercised is iterative, but
    // `Value`'s implicit `Drop` glue recurses with the chain's nesting depth
    // (the same shape as the derived `Debug`); leak these 100,000-deep owned
    // structures rather than overflow the host stack dropping them.
    core::mem::forget(a);
    core::mem::forget(b);
    core::mem::forget(outcome);
}

/// Trace: TC-025, FR-008-AC-8
#[test]
fn tc_025_p7_injected_denials_leave_counters_unchanged() {
    let collection_type = CollectionType::new(
        CollectionKind::Set,
        ValueType::Integer,
        CardinalityBound::new(0, 10).unwrap(),
    );

    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::CollectionElement,
        occurrence: 1,
    });
    let before = consumed(&meter);
    let elements: Vec<Deferred<'_>> = vec![Box::new(|_meter: &mut Meter| {
        Outcome::Completed(int_value(1))
    })];
    let outcome = construct_collection(&collection_type, elements, &mut meter);
    let Outcome::Incomplete(record) = outcome else {
        panic!("expected an incomplete outcome, got {outcome:?}")
    };
    let expected = Incomplete {
        charge_point: ChargePoint::CollectionElement,
        ..record.clone()
    };
    assert_eq!(record, expected);
    assert_eq!(consumed(&meter), before);

    // Over occurrences `[1, 2]` (a Set), exactly one membership comparison
    // (`2` against the retained `1`) precedes `collection.bound` and
    // `collection.result-retain`. A denied charge itself never lands, so the
    // counters after each denied run stop at exactly what the charges
    // admitted *before* the denied one already consumed: member-walk
    // (`value_occurrences = 1`, `work_units = 2`) if reached, then
    // member-test (`work_units += 1`), then bound
    // (`value_occurrences = max(1, count=2) = 2`, `work_units += 1`).
    let expect_at = |value_occurrences: u64, work_units: u64| -> Vec<u64> {
        let mut expected = vec![0_u64; 10];
        expected[LimitKind::ValueOccurrences as usize] = value_occurrences;
        expected[LimitKind::WorkUnits as usize] = work_units;
        expected
    };
    for (point, expected) in [
        (ChargePoint::CollectionMemberWalk, expect_at(0, 0)),
        (ChargePoint::CollectionMemberTest, expect_at(1, 2)),
        (ChargePoint::CollectionBound, expect_at(1, 3)),
        (ChargePoint::CollectionResultRetain, expect_at(2, 4)),
    ] {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: 1,
        });
        let outcome = form_collection(
            &collection_type,
            vec![int_value(1), int_value(2)],
            &mut meter,
        )
        .unwrap();
        let Outcome::Incomplete(record) = outcome else {
            panic!("expected incomplete at {point:?}, got {outcome:?}")
        };
        assert_eq!(record.charge_point, point);
        assert_eq!(consumed(&meter), expected, "at {point:?}");
    }
}
