//! Observable kernel pair results consumed by RT after the value cutover.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{
    CollectionKind, CollectionType, EffectiveId, EnumMember, FieldValue, Integer, Meter, NodeKey,
    ObjectId, ObjectReference, Outcome, Refusal, ScalarLimits, UniverseId, Value, ValueType,
    VariantId,
};

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

#[trace("TC-026", "FR-008-AC-5")]
#[test]
fn kernel_distinguishes_composite_declarations_and_compares_enum_variants() {
    let fields = || Box::new([FieldValue::Present(Value::Integer(Integer::from(3_u64)))]);
    let left = quire_exact::from_admitted_slots(NodeKey::from_digest([1; 32]), fields());
    let right = quire_exact::from_admitted_slots(NodeKey::from_digest([2; 32]), fields());
    let mut meter = Meter::new(LIMITS);
    assert!(matches!(
        quire_exact::planned_equality(&left, &right, &mut meter),
        Outcome::Completed(false)
    ));

    let variant = VariantId::from_digest([3; 32]);
    let left = Value::Enum(EnumMember::new(variant, 0));
    let right = Value::Enum(EnumMember::new(variant, 1));
    let mut meter = Meter::new(LIMITS);
    assert!(matches!(
        quire_exact::planned_equality(&left, &right, &mut meter),
        Outcome::Completed(true)
    ));
}

#[trace("TC-025", "TC-026", "FR-008-AC-5")]
#[test]
fn membership_foreign_reference_names_retained_member_first() {
    let object_type = EffectiveId::from_digest([4; 32]);
    let member_universe = UniverseId::from_digest([5; 32]);
    let candidate_universe = UniverseId::from_digest([6; 32]);
    let reference = |universe| {
        Value::Reference(ObjectReference::new(
            universe,
            object_type,
            ObjectId::new("object").unwrap(),
        ))
    };
    let collection_type =
        CollectionType::new(CollectionKind::Set, ValueType::Reference(object_type), None);
    let mut meter = Meter::new(LIMITS);
    let outcome = quire_exact::form_collection(
        &collection_type,
        vec![reference(member_universe), reference(candidate_universe)],
        &mut meter,
    )
    .unwrap();
    assert!(matches!(
        outcome,
        Outcome::Refused(Refusal::ForeignReference { required, supplied })
            if required == member_universe && supplied == candidate_universe
    ));
}
