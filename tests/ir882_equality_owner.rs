//! Consumer controls for the shared exact value and checked equality owner.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{
    ChargePoint, CollectionKind, CollectionType, FieldValue, IllTypedCause, InjectedDenial,
    Integer, LimitKind, Meter, NodeKey, Outcome, Presence, ScalarLimits, Value, ValueType,
};
use quire_semantic_value::declaration::{
    CheckedEquality, CompositeDeclaration, CompositeShape, EqualityOperand, EqualityOperator,
    FieldDeclaration, TypeEnvironment,
};
use quire_semantic_value::enumeration::EnumMemberIndex;
use std::fs;
use std::num::NonZeroU64;

const LIMITS: ScalarLimits = ScalarLimits {
    integer_bits: 100,
    decimal_digits: 100,
    scale_expansion: 100,
    text_input_bytes: 100,
    text_scalars: 100,
    normalized_scalars: 100,
    unit_edges: 100,
    value_occurrences: 100,
    work_units: 100,
    result_units: 100,
};

fn admitted_operand(types: &TypeEnvironment, first: i128) -> Value {
    let key = NodeKey::from_digest([42; 32]);
    let inner_type = CollectionType::new(CollectionKind::Sequence, ValueType::Composite(key), None);
    let outer_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::collection(inner_type.clone()),
        None,
    );
    let mut construction_meter = Meter::new(LIMITS);
    const FIELD_NAMES: [&str; 6] = ["field0", "field1", "field2", "field3", "field4", "field5"];
    let inner = (0..2)
        .map(|composite_index| {
            let fields = (0..6)
                .filter(|field_index| composite_index == 0 || *field_index != 5)
                .map(|field_index| {
                    let number = if composite_index == 0 && field_index == 0 {
                        first
                    } else {
                        i128::from(composite_index * 6 + field_index)
                    };
                    (
                        FIELD_NAMES[field_index as usize],
                        FieldValue::Present(Value::Integer(Integer::from(number))),
                    )
                })
                .collect::<Vec<_>>();
            let composite = types.record(key, fields).unwrap();
            quire_exact::form_collection(&inner_type, vec![composite], &mut construction_meter)
                .unwrap()
                .completed()
                .unwrap()
        })
        .collect();
    quire_exact::form_collection(&outer_type, inner, &mut construction_meter)
        .unwrap()
        .completed()
        .unwrap()
}

fn checked_equality() -> (TypeEnvironment, CheckedEquality) {
    let key = NodeKey::from_digest([42; 32]);
    let nested = ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        ValueType::collection(CollectionType::new(
            CollectionKind::Sequence,
            ValueType::Composite(key),
            None,
        )),
        None,
    ));
    let declaration = CompositeDeclaration::new(
        key,
        "Six fields",
        CompositeShape::Record(
            (0..6)
                .map(|index| {
                    FieldDeclaration::new(
                        format!("field{index}"),
                        ValueType::Integer,
                        if index == 5 {
                            Presence::Optional
                        } else {
                            Presence::Required
                        },
                    )
                })
                .collect(),
        ),
    );
    let types = TypeEnvironment::new([declaration], []).unwrap();
    let checked = types
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(nested.clone()),
            EqualityOperand::typed(nested),
            &EnumMemberIndex::default(),
        )
        .unwrap();
    (types, checked)
}

fn admitted_points(meter: &Meter) -> Vec<ChargePoint> {
    meter
        .admitted_charges()
        .iter()
        .map(|charge| charge.point)
        .collect()
}

fn full_schedule() -> Vec<ChargePoint> {
    let mut points = vec![ChargePoint::EqualityPlanForm, ChargePoint::EqualityPlan];
    points.extend([ChargePoint::EqualityPair; 17]);
    points.push(ChargePoint::EqualityResultRetain);
    points
}

#[trace("TC-203", "FR-277-AC-8")]
#[test]
fn runtime_defines_no_local_equality_or_value_owner() {
    let module = include_str!("../src/exact/mod.rs");
    assert!(!module.contains("mod equality;"));
    assert!(!module.contains("mod composite;"));
    assert!(!module.contains("pub use equality::"));
    assert!(!module.contains("pub use composite::"));
    let source = format!("{}/src/exact", env!("CARGO_MANIFEST_DIR"));
    for entry in fs::read_dir(source).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let source = fs::read_to_string(&path).unwrap();
        for duplicate in [
            "pub enum Value {",
            "pub struct CheckedEquality {",
            "fn plan_pairs(",
            "fn planned_equality(",
            "pub type Value =",
            "pub use quire_exact::Value",
            "pub use quire_semantic_value::declaration::CheckedEquality",
        ] {
            assert!(
                !source.contains(duplicate),
                "{} defines {duplicate}",
                path.display()
            );
        }
    }
}

#[trace("TC-203", "FR-277-AC-1", "FR-277-AC-2")]
#[test]
fn qsv_equality_consumes_kernel_values_and_full_pair_schedule() {
    let (types, checked) = checked_equality();
    let left = admitted_operand(&types, 0);
    let equal_right = admitted_operand(&types, 0);
    let unequal_right = admitted_operand(&types, 99);
    assert_eq!(left.occ(), Integer::from(16_u64));
    for (right, expected) in [(&equal_right, true), (&unequal_right, false)] {
        let mut meter = Meter::new(LIMITS);
        assert!(matches!(
            checked.evaluate(&types, &left, right, &mut meter),
            Outcome::Completed(value) if value == expected
        ));
        assert_eq!(admitted_points(&meter), full_schedule());
    }
}

#[trace("TC-203", "FR-277-AC-3")]
#[test]
fn late_pair_and_retention_denials_keep_only_the_admitted_prefix() {
    let (types, checked) = checked_equality();
    let left = admitted_operand(&types, 0);
    let right = admitted_operand(&types, 0);
    let mut meter = Meter::new(LIMITS).with_injected_denial(InjectedDenial {
        point: ChargePoint::EqualityPair,
        occurrence: NonZeroU64::new(17).unwrap(),
    });
    let Outcome::Incomplete(refusal) = checked.evaluate(&types, &left, &right, &mut meter) else {
        panic!("the seventeenth pair must be denied");
    };
    assert_eq!(refusal.charge_point, ChargePoint::EqualityPair);
    let expected = full_schedule();
    assert_eq!(admitted_points(&meter), expected[..18]);

    let mut meter = Meter::new(LIMITS).with_injected_denial(InjectedDenial {
        point: ChargePoint::EqualityResultRetain,
        occurrence: NonZeroU64::MIN,
    });
    let Outcome::Incomplete(refusal) = checked.evaluate(&types, &left, &right, &mut meter) else {
        panic!("result retention must be denied");
    };
    assert_eq!(refusal.charge_point, ChargePoint::EqualityResultRetain);
    assert_eq!(admitted_points(&meter), expected[..19]);
}

#[trace("TC-203", "FR-277-AC-4")]
#[test]
fn distinct_static_composite_types_refuse_before_evaluation() {
    let first = NodeKey::from_digest([42; 32]);
    let second = NodeKey::from_digest([43; 32]);
    let shape = CompositeShape::Tuple(vec![ValueType::Integer]);
    let types = TypeEnvironment::new(
        [
            CompositeDeclaration::new(first, "First", shape.clone()),
            CompositeDeclaration::new(second, "Second", shape),
        ],
        [],
    )
    .unwrap();
    // The checker takes no meter; construction cannot admit an equality charge.
    let refusal = types
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Composite(first)),
            EqualityOperand::typed(ValueType::Composite(second)),
            &EnumMemberIndex::default(),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::TypeMismatch);
}

#[trace("TC-203", "FR-277-AC-5")]
#[test]
fn equality_plan_reservation_refuses_atomically_at_fifty_work_units() {
    let (types, checked) = checked_equality();
    let left = admitted_operand(&types, 0);
    let right = admitted_operand(&types, 0);
    let mut meter = Meter::new(ScalarLimits {
        work_units: 50,
        ..LIMITS
    });
    let Outcome::Incomplete(refusal) = checked.evaluate(&types, &left, &right, &mut meter) else {
        panic!("the plan reservation must be denied");
    };
    assert_eq!(refusal.charge_point, ChargePoint::EqualityPlan);
    assert_eq!(refusal.limit_kind, LimitKind::WorkUnits);
    assert_eq!(refusal.limit, 50);
    assert_eq!(refusal.consumed, 32);
    assert_eq!(refusal.next_charge, Integer::from(19_u64));
    assert_eq!(admitted_points(&meter), [ChargePoint::EqualityPlanForm]);
}

#[trace("TC-203", "FR-277-AC-6")]
#[test]
fn equality_plan_checks_result_capacity_before_pair_charges() {
    let (types, checked) = checked_equality();
    let left = admitted_operand(&types, 0);
    let right = admitted_operand(&types, 0);
    let mut meter = Meter::new(ScalarLimits {
        result_units: 0,
        ..LIMITS
    });
    let Outcome::Incomplete(refusal) = checked.evaluate(&types, &left, &right, &mut meter) else {
        panic!("the result reservation must be denied");
    };
    assert_eq!(refusal.charge_point, ChargePoint::EqualityPlan);
    assert_eq!(refusal.limit_kind, LimitKind::ResultUnits);
    assert_eq!(refusal.limit, 0);
    assert_eq!(refusal.consumed, 0);
    assert_eq!(refusal.next_charge, Integer::one());
    assert_eq!(admitted_points(&meter), [ChargePoint::EqualityPlanForm]);
}
