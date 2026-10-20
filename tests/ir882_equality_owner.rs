//! Consumer controls for the shared exact value and checked equality owner.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{
    CollectionKind, CollectionType, FieldValue, Integer, Meter, NodeKey, Outcome, Presence,
    ScalarLimits, Value, ValueType,
};
use quire_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, EqualityOperand, EqualityOperator, FieldDeclaration,
    TypeEnvironment,
};
use quire_semantic_value::enumeration::EnumMemberIndex;

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

fn admitted_operand(first: i128) -> Value {
    let key = NodeKey::from_digest([42; 32]);
    let inner_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Composite(key),
        None,
    );
    let outer_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::collection(inner_type.clone()),
        None,
    );
    let mut construction_meter = Meter::new(LIMITS);
    let inner = (0..2)
        .map(|composite_index| {
            let fields = (0..6)
                .map(|field_index| {
                    if composite_index == 1 && field_index == 5 {
                        FieldValue::Absent
                    } else {
                        let number = if composite_index == 0 && field_index == 0 {
                            first
                        } else {
                            i128::from(composite_index * 6 + field_index)
                        };
                        FieldValue::Present(Value::Integer(Integer::from(number)))
                    }
                })
                .collect::<Vec<_>>();
            let composite = quire_exact::from_admitted_slots(key, fields.into_boxed_slice());
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

#[trace("TC-203")]
#[test]
fn runtime_defines_no_local_equality_or_value_owner() {
    let module = include_str!("../src/exact/mod.rs");
    assert!(!module.contains("mod equality;"));
    assert!(!module.contains("mod composite;"));
    assert!(!module.contains("pub use equality::"));
    assert!(!module.contains("pub use composite::"));
}

#[trace("TC-203")]
#[test]
fn qsv_equality_consumes_kernel_values_and_full_pair_schedule() {
    let left = admitted_operand(0);
    let equal_right = admitted_operand(0);
    let unequal_right = admitted_operand(99);
    assert_eq!(left.occ(), Integer::from(16_u64));
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
                        if index == 5 { Presence::Optional } else { Presence::Required },
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
    for (right, expected) in [(&equal_right, true), (&unequal_right, false)] {
        let mut meter = Meter::new(LIMITS);
        assert!(matches!(
            checked.evaluate(&types, &left, right, &mut meter),
            Outcome::Completed(value) if value == expected
        ));
        assert_eq!(
            meter.admitted_charges()
                .iter()
                .filter(|charge| charge.point == quire_exact::ChargePoint::EqualityPair)
                .count(),
            17
        );
    }
}
