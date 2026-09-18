//! The quire-specification/FR-149 equality matrix and terminal `Reference<T>` identity, through
//! the public `exact` surface.
#![cfg(feature = "exact")]

use quire_contract_runtime::exact::{
    admit_text, CardinalityBound, ChargePoint, CollectionKind, CollectionType, Component,
    CompositeDeclaration, CompositeShape, ConstructionCause, ConstructionRefusal, DecimalType,
    EnumDeclaration, EqualityOperand, EqualityOperator, EqualitySchedule, FieldDeclaration,
    FieldValue, IeeeWidth, IllTyped, IllTypedCause, Incomplete, InjectedDenial, Integer,
    IntegerInterval, LimitKind, Meter, NodeKey, ObjectIdentity, ObjectReference,
    ObjectTypeDeclaration, Outcome, Presence, Quantity, QuantityUnit, Rational, Refusal,
    RoundingMode, ScalarLimits, Text, TextPayload, TextProfile, TextType, TypeEnvironment,
    UnitDeclaration, UnitGraph, UniverseIdentity, Value, ValueType,
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

/// Two declared units of two base dimensions: `meter`/`kilometer` (dimension
/// A) and `kilogram` (root of dimension B), so tests can pick same-dimension
/// distinct units and different-dimension units.
fn unit_fixture() -> (QuantityUnit, QuantityUnit, QuantityUnit) {
    let dims = [(key(50), Vec::new()), (key(51), Vec::new())];
    let units = [
        (
            key(60),
            UnitDeclaration {
                dimension: key(50),
                target: None,
                scale: Rational::from_integer(Integer::one()),
                offset: Rational::from_integer(Integer::zero()),
            },
        ),
        (
            key(61),
            UnitDeclaration {
                dimension: key(50),
                target: Some(key(60)),
                scale: Rational::from_integer(Integer::from(1000_i64)),
                offset: Rational::from_integer(Integer::zero()),
            },
        ),
        (
            key(70),
            UnitDeclaration {
                dimension: key(51),
                target: None,
                scale: Rational::from_integer(Integer::one()),
                offset: Rational::from_integer(Integer::zero()),
            },
        ),
    ];
    let graph = UnitGraph::admit(dims, units).unwrap();
    let meter_unit = QuantityUnit::Declared(Box::new(graph.unit(key(60)).unwrap().clone()));
    let kilometer_unit = QuantityUnit::Declared(Box::new(graph.unit(key(61)).unwrap().clone()));
    let kilogram_unit = QuantityUnit::Declared(Box::new(graph.unit(key(70)).unwrap().clone()));
    (meter_unit, kilometer_unit, kilogram_unit)
}

/// Trace: TC-026, FR-008-AC-5
#[test]
fn tc_026_p1_check_equality_refuses_before_any_charge_and_selects_schedule() {
    let env = TypeEnvironment::new(Vec::new(), Vec::new()).unwrap();

    // An unadmitted `convert<T>(e)`: `Integer` (unbounded) has no row into
    // `Text`.
    let text_type = TextType::new(0, 10, TextProfile::Nfc).unwrap();
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::converted(ValueType::Integer, ValueType::Text(text_type)),
            EqualityOperand::typed(ValueType::Text(text_type)),
        )
        .unwrap_err();
    assert_eq!(
        refusal,
        IllTyped {
            cause: IllTypedCause::TypeMismatch
        }
    );

    // No common type and no conversion at all: also `type-mismatch`.
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Boolean),
            EqualityOperand::typed(ValueType::Integer),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::TypeMismatch);

    // An IEEE-bearing operand type is `operator-ineligible`, at the top level
    // and nested inside a collection element type.
    let bound = CardinalityBound::new(0, 5).unwrap();
    let nested_float = ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Float(IeeeWidth::Binary64),
        bound,
    ));
    for ieee_type in [ValueType::Float(IeeeWidth::Binary64), nested_float] {
        let refusal = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ieee_type.clone()),
                EqualityOperand::typed(ieee_type),
            )
            .unwrap_err();
        assert_eq!(refusal.cause, IllTypedCause::OperatorIneligible);
    }

    // Distinct text profiles.
    let nfc = TextType::new(0, 10, TextProfile::Nfc).unwrap();
    let nfd = TextType::new(0, 10, TextProfile::Nfd).unwrap();
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Text(nfc)),
            EqualityOperand::typed(ValueType::Text(nfd)),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::DistinctTextProfiles);

    // Distinct enum declarations: `ValueType::Enum` names the declaration key.
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Enum(key(30))),
            EqualityOperand::typed(ValueType::Enum(key(31))),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::DistinctEnumDeclarations);

    // Incompatible quantity dimensions and mismatched units of one dimension.
    let (meter_unit, kilometer_unit, kilogram_unit) = unit_fixture();
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Quantity(meter_unit.clone())),
            EqualityOperand::typed(ValueType::Quantity(kilogram_unit)),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::IncompatibleDimensions);
    let refusal = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Quantity(meter_unit.clone())),
            EqualityOperand::typed(ValueType::Quantity(kilometer_unit)),
        )
        .unwrap_err();
    assert_eq!(refusal.cause, IllTypedCause::DistinctUnits);

    // Selected schedules from the common comparison type.
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Text(nfc)),
            EqualityOperand::typed(ValueType::Text(nfc)),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Text);

    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Enum(key(30))),
            EqualityOperand::typed(ValueType::Enum(key(30))),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Enum);

    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Quantity(meter_unit.clone())),
            EqualityOperand::typed(ValueType::Quantity(meter_unit)),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Quantity);

    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Plan);
}

/// Trace: TC-026, FR-008-AC-5
#[test]
fn tc_026_p2_plan_equality_is_uncharged_and_refuses_foreign_references_at_plan_time() {
    // `plan_equality` takes no meter: it cannot charge anything by
    // construction. Two completed scalar operands plan to exactly one pair.
    let plan = quire_contract_runtime::exact::plan_equality(&int_value(3), &int_value(3)).unwrap();
    assert_eq!(plan.pair_events(), &Integer::one());

    // A reference pair of different universes refuses by name at plan time.
    let universe_a = UniverseIdentity::new(b"universe-a").unwrap();
    let universe_b = UniverseIdentity::new(b"universe-b").unwrap();
    let object_type = key(80);
    let identity = ObjectIdentity::new(b"object-1").unwrap();
    let reference_a = Value::Reference(ObjectReference::new(
        universe_a,
        object_type,
        identity.clone(),
    ));
    let reference_b = Value::Reference(ObjectReference::new(universe_b, object_type, identity));
    let refusal =
        quire_contract_runtime::exact::plan_equality(&reference_a, &reference_b).unwrap_err();
    assert_eq!(refusal, Refusal::ForeignReference);
}

/// Trace: TC-026, FR-008-AC-5
#[test]
fn tc_026_p3_evaluate_charges_conversions_then_the_plan_schedule_in_order() {
    let env = TypeEnvironment::new(Vec::new(), Vec::new()).unwrap();

    // Left-then-right conversion charges, then the occurrence-pair plan.
    let source =
        ValueType::Int(IntegerInterval::new(Integer::zero(), Integer::from(1000_i64)).unwrap());
    let decimal_type = DecimalType::new(
        Integer::zero(),
        Integer::from(1000_i64),
        0,
        0,
        RoundingMode::Exact,
    )
    .unwrap();
    let target = ValueType::Decimal(decimal_type);
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::converted(source.clone(), target.clone()),
            EqualityOperand::converted(source, target),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Plan);
    let mut meter = Meter::new(UNLIMITED);
    let outcome = checked.evaluate(&int_value(5), &int_value(5), &mut meter);
    assert_eq!(outcome, Outcome::Completed(true));
    assert_eq!(
        meter.admitted_charges(),
        [
            ChargePoint::DecimalOperands,
            ChargePoint::DecimalScaleExpansion,
            ChargePoint::DecimalArithmetic,
            ChargePoint::DecimalResultRetain,
            ChargePoint::DecimalOperands,
            ChargePoint::DecimalScaleExpansion,
            ChargePoint::DecimalArithmetic,
            ChargePoint::DecimalResultRetain,
            ChargePoint::EqualityPlanForm,
            ChargePoint::EqualityPlan,
            ChargePoint::EqualityPair,
            ChargePoint::EqualityResultRetain,
        ]
    );

    // A composite operand pair nesting a collection: `equality.plan-form`,
    // `equality.plan`, one `equality.pair` per planned occurrence pair (the
    // composite root, field `a`, the collection field, and its two
    // elements: 5 pairs), then `equality.result-retain`.
    let bound = CardinalityBound::new(0, 10).unwrap();
    let xs_type = ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Integer,
        bound,
    ));
    let pair_declaration = CompositeDeclaration::new(
        key(2),
        "Pair",
        CompositeShape::Record(vec![
            FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
            FieldDeclaration::new("xs", xs_type, Presence::Required),
        ]),
    );
    let env = TypeEnvironment::new([pair_declaration], []).unwrap();
    let mut scratch = Meter::new(UNLIMITED);
    let xs_collection_type =
        CollectionType::new(CollectionKind::Sequence, ValueType::Integer, bound);
    let xs = quire_contract_runtime::exact::form_collection(
        &xs_collection_type,
        vec![int_value(10), int_value(20)],
        &mut scratch,
    )
    .unwrap()
    .completed()
    .unwrap();
    let left = env
        .record(
            key(2),
            vec![
                ("a", FieldValue::Present(int_value(1))),
                ("xs", FieldValue::Present(xs.clone())),
            ],
        )
        .unwrap();
    let right = env
        .record(
            key(2),
            vec![
                ("a", FieldValue::Present(int_value(1))),
                ("xs", FieldValue::Present(xs)),
            ],
        )
        .unwrap();
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Composite(key(2))),
            EqualityOperand::typed(ValueType::Composite(key(2))),
        )
        .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let outcome = checked.evaluate(&left, &right, &mut meter);
    assert_eq!(outcome, Outcome::Completed(true));
    let mut expected = vec![ChargePoint::EqualityPlanForm, ChargePoint::EqualityPlan];
    expected.extend([ChargePoint::EqualityPair; 5]);
    expected.push(ChargePoint::EqualityResultRetain);
    assert_eq!(meter.admitted_charges(), expected.as_slice());

    // A top-level text, enum and quantity pair each run their own schedule
    // instead of the occurrence-pair plan: no `equality.*` charge appears.
    let text_type = TextType::new(0, 10, TextProfile::UnicodeScalars).unwrap();
    let payload = TextPayload::from_utf8(b"hi").unwrap();
    let mut scratch = Meter::new(UNLIMITED);
    let text: Text = admit_text(&payload, &text_type, &mut scratch)
        .completed()
        .unwrap();
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Text(text_type)),
            EqualityOperand::typed(ValueType::Text(text_type)),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Text);
    let mut meter = Meter::new(UNLIMITED);
    let outcome = checked.evaluate(&Value::Text(text.clone()), &Value::Text(text), &mut meter);
    assert_eq!(outcome, Outcome::Completed(true));
    assert!(meter
        .admitted_charges()
        .iter()
        .all(|point| !point.as_str().starts_with("equality.")));

    let declaration = EnumDeclaration::new(key(31), false, &["A", "B"]).unwrap();
    let member = declaration.member("A", key(32)).unwrap();
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Enum(key(31))),
            EqualityOperand::typed(ValueType::Enum(key(31))),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Enum);
    let mut meter = Meter::new(UNLIMITED);
    let outcome = checked.evaluate(
        &Value::Enum(member.clone()),
        &Value::Enum(member),
        &mut meter,
    );
    assert_eq!(outcome, Outcome::Completed(true));
    assert!(meter
        .admitted_charges()
        .iter()
        .all(|point| !point.as_str().starts_with("equality.")));

    let (meter_unit, ..) = unit_fixture();
    let quantity = Quantity::new(
        Rational::from_integer(Integer::from(5_i64)),
        meter_unit.clone(),
    );
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Quantity(meter_unit.clone())),
            EqualityOperand::typed(ValueType::Quantity(meter_unit)),
        )
        .unwrap();
    assert_eq!(checked.schedule(), EqualitySchedule::Quantity);
    let mut meter = Meter::new(UNLIMITED);
    let outcome = checked.evaluate(
        &Value::Quantity(quantity.clone()),
        &Value::Quantity(quantity),
        &mut meter,
    );
    assert_eq!(outcome, Outcome::Completed(true));
    assert!(meter
        .admitted_charges()
        .iter()
        .all(|point| !point.as_str().starts_with("equality.")));
}

/// Trace: TC-026, FR-008-AC-6
#[test]
fn tc_026_p4_object_reference_identity_and_no_component_substitution() {
    let universe = UniverseIdentity::new(b"universe-a").unwrap();
    let object_type = key(81);
    let identity = ObjectIdentity::new(b"object-1").unwrap();
    let reference = ObjectReference::new(universe.clone(), object_type, identity.clone());
    // The reference carries exactly its supplied triple: no attribute of the
    // referenced object is read or required (this crate has no
    // `ObjectEnvironment` at all to read one from).
    assert_eq!(reference.universe(), &universe);
    assert_eq!(reference.object_type(), object_type);
    assert_eq!(reference.identity(), &identity);

    // No record, tuple or collection constructor accepts a component value in
    // place of a supplied reference.
    let object_type_decl = ObjectTypeDeclaration::new(object_type, "Widget", Vec::new());
    let holder = CompositeDeclaration::new(
        key(3),
        "Holder",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "ref",
            ValueType::Reference(object_type),
            Presence::Required,
        )]),
    );
    let tuple_holder = CompositeDeclaration::new(
        key(4),
        "TupleHolder",
        CompositeShape::Tuple(vec![ValueType::Reference(object_type)]),
    );
    let env = TypeEnvironment::new([holder, tuple_holder], [object_type_decl]).unwrap();

    let refusal = env
        .record(key(3), vec![("ref", FieldValue::Present(int_value(1)))])
        .unwrap_err();
    assert_eq!(
        refusal,
        ConstructionRefusal {
            component: Component::Field(String::from("ref")),
            cause: ConstructionCause::TypeMismatch,
        }
    );
    let refusal = env.tuple(key(4), vec![int_value(1)]).unwrap_err();
    assert_eq!(
        refusal,
        ConstructionRefusal {
            component: Component::Position(0),
            cause: ConstructionCause::TypeMismatch,
        }
    );

    let bound = CardinalityBound::new(0, 5).unwrap();
    let collection_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Reference(object_type),
        bound,
    );
    let mut meter = Meter::new(UNLIMITED);
    let refusal = quire_contract_runtime::exact::form_collection(
        &collection_type,
        vec![int_value(1)],
        &mut meter,
    )
    .unwrap_err();
    assert_eq!(
        refusal,
        ConstructionRefusal {
            component: Component::Element(0),
            cause: ConstructionCause::TypeMismatch,
        }
    );

    // A genuinely-typed reference is accepted by all three.
    let value = Value::Reference(reference);
    env.record(key(3), vec![("ref", FieldValue::Present(value.clone()))])
        .unwrap();
    env.tuple(key(4), vec![value.clone()]).unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let outcome =
        quire_contract_runtime::exact::form_collection(&collection_type, vec![value], &mut meter)
            .unwrap();
    assert!(matches!(outcome, Outcome::Completed(Value::Collection(_))));
}

/// Trace: TC-026, FR-008-AC-8
#[test]
fn tc_026_p5_injected_denials_at_each_equality_charge_point() {
    let env = TypeEnvironment::new(Vec::new(), Vec::new()).unwrap();
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
        )
        .unwrap();

    // Over one planned pair (two `Integer` leaves), `equality.plan-form`
    // charges first (`value_occurrences = 1`, `work_units += 2`), then
    // `equality.plan` (`work_units += 1`), then one `equality.pair`
    // (`work_units += 1`), then `equality.result-retain`
    // (`work_units += 1`). A denied charge itself never lands, so the
    // counters after each denied run stop at exactly what the charges
    // admitted *before* the denied one already consumed.
    let expect_at = |value_occurrences: u64, work_units: u64| -> Vec<u64> {
        let mut expected = vec![0_u64; 10];
        expected[LimitKind::ValueOccurrences as usize] = value_occurrences;
        expected[LimitKind::WorkUnits as usize] = work_units;
        expected
    };
    let cases = [
        (
            ChargePoint::EqualityPlanForm,
            expect_at(0, 0),
            Integer::from(2_i64),
        ),
        (
            ChargePoint::EqualityPlan,
            expect_at(1, 2),
            Integer::from(3_i64),
        ),
        (ChargePoint::EqualityPair, expect_at(1, 3), Integer::one()),
        (
            ChargePoint::EqualityResultRetain,
            expect_at(1, 4),
            Integer::one(),
        ),
    ];
    for (point, expected_consumed, next_charge) in cases {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: 1,
        });
        let outcome = checked.evaluate(&int_value(1), &int_value(1), &mut meter);
        let Outcome::Incomplete(record) = outcome else {
            panic!("expected incomplete at {point:?}, got {outcome:?}")
        };
        let expected_limit = expected_consumed[LimitKind::WorkUnits as usize];
        assert_eq!(
            record,
            Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: expected_limit,
                consumed: expected_limit,
                next_charge,
                charge_point: point,
            },
            "at {point:?}"
        );
        assert_eq!(consumed(&meter), expected_consumed, "at {point:?}");
    }
}
