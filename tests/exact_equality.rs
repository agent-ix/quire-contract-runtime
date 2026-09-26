//! The quire-specification/FR-149 equality matrix and terminal `Reference<T>` identity, through
//! the public `exact` surface.
#![cfg(feature = "exact")]

use std::num::NonZeroU64;

use proptest::prelude::*;
use quire_contract_runtime::exact::{
    admit_text, CardinalityBound, ChargePoint, CollectionKind, CollectionType, Component,
    CompositeDeclaration, CompositeShape, ConstructionCause, ConstructionRefusal, DecimalType,
    EnumDeclaration, EqualityOperand, EqualityOperator, EqualitySchedule, FieldDeclaration,
    FieldValue, IeeeWidth, IllTyped, IllTypedCause, Incomplete, InjectedDenial, Integer,
    IntegerInterval, LimitKind, Meter, NodeKey, ObjectIdentity, ObjectReference,
    ObjectTypeDeclaration, OptionValue, Outcome, Presence, Quantity, QuantityUnit, Rational,
    Refusal, RoundingMode, ScalarLimits, Text, TextPayload, TextProfile, TextType, TypeEnvironment,
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
            occurrence: NonZeroU64::new(1).unwrap(),
        });
        let outcome = checked.evaluate(&int_value(1), &int_value(1), &mut meter);
        let Outcome::Incomplete(record) = outcome else {
            panic!("expected incomplete at {point:?}, got {outcome:?}")
        };
        let expected_limit = expected_consumed[LimitKind::WorkUnits as usize];
        assert_eq!(
            record,
            Box::new(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: expected_limit,
                consumed: expected_limit,
                next_charge,
                charge_point: point,
            }),
            "at {point:?}"
        );
        assert_eq!(consumed(&meter), expected_consumed, "at {point:?}");
    }
}

// The plan/evaluate agreement property (agent-ix/quire-contract-runtime IR-31): over generated
// composite values, `plan_equality`'s uncharged pair count and the number of `equality.pair`
// charges `CheckedEquality::evaluate` actually admits must agree. A Kani proof of this property is
// tracked as agent-ix/quire-contract-runtime IR-241 (Linear). `tc_026_p3` and `tc_026_p5` above
// additionally exercise `equality.pair` charging on fixed, hand-picked cases; this property test
// widens that coverage over generated shapes and also checks the predicted count against an
// independent, test-side count (never calling into `equality.rs`) rather than only against the
// admitted charges.
//
// `Spec` describes a shape only (`Boolean`, `Integer`, `Option<T>`, a bounded `Sequence<T>`, or a
// two-field tuple record), never a value. `annotate` assigns each `Pair` node a fresh `NodeKey`
// deterministically (a plain pre-order counter, not randomness), producing `Keyed`: the same
// shape, now with the keys `build_type` and `materialize` both read to agree on which composite
// declaration a tuple value belongs to.
#[derive(Clone, Debug)]
enum Spec {
    Bool,
    Int,
    Opt(Box<Spec>),
    List(Box<Spec>),
    Pair(Box<Spec>, Box<Spec>),
}

#[derive(Clone, Debug)]
enum Keyed {
    Bool,
    Int,
    Opt(Box<Keyed>),
    List(Box<Keyed>),
    Pair(NodeKey, Box<Keyed>, Box<Keyed>),
}

/// The list element type's declared cardinality bound, shared between
/// `build_type` and `materialize` so both sides always build the same
/// `CollectionType`.
fn list_bound() -> CardinalityBound {
    CardinalityBound::new(0, 4).unwrap()
}

fn spec_strategy() -> impl Strategy<Value = Spec> {
    let leaf = prop_oneof![Just(Spec::Bool), Just(Spec::Int)];
    leaf.prop_recursive(3, 15, 3, |inner| {
        prop_oneof![
            inner.clone().prop_map(|s| Spec::Opt(Box::new(s))),
            inner.clone().prop_map(|s| Spec::List(Box::new(s))),
            (inner.clone(), inner).prop_map(|(l, r)| Spec::Pair(Box::new(l), Box::new(r))),
        ]
    })
}

/// Assign a fresh key to every `Pair` node, in a pre-order walk of `spec`.
fn annotate(spec: &Spec, next: &mut u8) -> Keyed {
    match spec {
        Spec::Bool => Keyed::Bool,
        Spec::Int => Keyed::Int,
        Spec::Opt(inner) => Keyed::Opt(Box::new(annotate(inner, next))),
        Spec::List(inner) => Keyed::List(Box::new(annotate(inner, next))),
        Spec::Pair(left, right) => {
            let node_key = key(*next);
            *next += 1;
            let left = annotate(left, next);
            let right = annotate(right, next);
            Keyed::Pair(node_key, Box::new(left), Box::new(right))
        }
    }
}

/// The `ValueType` of `spec`, appending every nested tuple declaration it
/// names to `decls` in the same pre-order. Calling this again on a subtree
/// already covered by an earlier top-level call (as `materialize` does, to
/// recover one node's own `ValueType`) reproduces the identical `ValueType`
/// -- same keys, same structure -- and simply discards the redundant decls.
fn build_type(spec: &Keyed, decls: &mut Vec<CompositeDeclaration>) -> ValueType {
    match spec {
        Keyed::Bool => ValueType::Boolean,
        Keyed::Int => ValueType::Integer,
        Keyed::Opt(inner) => ValueType::option(build_type(inner, decls)),
        Keyed::List(inner) => {
            let element = build_type(inner, decls);
            ValueType::collection(CollectionType::new(
                CollectionKind::Sequence,
                element,
                list_bound(),
            ))
        }
        Keyed::Pair(key, left, right) => {
            let left_type = build_type(left, decls);
            let right_type = build_type(right, decls);
            decls.push(CompositeDeclaration::new(
                *key,
                "Pair",
                CompositeShape::Tuple(vec![left_type, right_type]),
            ));
            ValueType::Composite(*key)
        }
    }
}

/// One generated content tree, structurally parallel to a `Keyed` spec but
/// carrying no type or key information of its own.
#[derive(Clone, Debug)]
enum RawValue {
    Bool(bool),
    Int(i64),
    Opt(Option<Box<RawValue>>),
    List(Vec<RawValue>),
    Pair(Box<RawValue>, Box<RawValue>),
}

fn raw_strategy(spec: &Keyed) -> BoxedStrategy<RawValue> {
    match spec {
        Keyed::Bool => any::<bool>().prop_map(RawValue::Bool).boxed(),
        Keyed::Int => (-1000_i64..=1000_i64).prop_map(RawValue::Int).boxed(),
        Keyed::Opt(inner) => proptest::option::of(raw_strategy(inner))
            .prop_map(|payload| RawValue::Opt(payload.map(Box::new)))
            .boxed(),
        Keyed::List(inner) => proptest::collection::vec(raw_strategy(inner), 0..=4)
            .prop_map(RawValue::List)
            .boxed(),
        Keyed::Pair(_, left, right) => (raw_strategy(left), raw_strategy(right))
            .prop_map(|(l, r)| RawValue::Pair(Box::new(l), Box::new(r)))
            .boxed(),
    }
}

/// Build `spec`'s `Value` from `raw`, constructing every nested tuple through
/// `env` by the key `spec` itself carries.
fn materialize(spec: &Keyed, raw: &RawValue, env: &TypeEnvironment) -> Value {
    match (spec, raw) {
        (Keyed::Bool, RawValue::Bool(b)) => Value::Boolean(*b),
        (Keyed::Int, RawValue::Int(n)) => Value::Integer(Integer::from(*n)),
        (Keyed::Opt(inner), RawValue::Opt(payload)) => {
            let payload_type = build_type(inner, &mut Vec::new());
            match payload {
                Some(raw_payload) => {
                    let payload = materialize(inner, raw_payload, env);
                    OptionValue::present(payload_type, payload)
                        .expect("payload was built against the same declared payload type")
                }
                None => OptionValue::none(payload_type),
            }
        }
        (Keyed::List(inner), RawValue::List(elements)) => {
            let element_type = build_type(inner, &mut Vec::new());
            let collection_type =
                CollectionType::new(CollectionKind::Sequence, element_type, list_bound());
            let elements: Vec<Value> = elements
                .iter()
                .map(|element| materialize(inner, element, env))
                .collect();
            let mut scratch = Meter::new(UNLIMITED);
            quire_contract_runtime::exact::form_collection(&collection_type, elements, &mut scratch)
                .expect("every element was built against the declared element type")
                .completed()
                .expect("an unlimited meter and a bound of 4 never blocks formation")
        }
        (Keyed::Pair(key, left_spec, right_spec), RawValue::Pair(left, right)) => {
            let left = materialize(left_spec, left, env);
            let right = materialize(right_spec, right, env);
            env.tuple(*key, vec![left, right])
                .expect("both positions were built against the declared tuple type")
        }
        _ => unreachable!("a RawValue is always generated from the Keyed spec it is paired with"),
    }
}

/// One test case: a shared shape and two value trees of that shape. The right side is, with equal
/// probability, generated fully independently of the left, an exact clone of the left, or a clone
/// of the left with exactly one leaf changed -- so equal-length lists, `Some`/`Some` options and an
/// overall `true` result (which an independently generated pair reaches only rarely, since the two
/// sides diverge at the root in the majority of cases) are all well exercised, alongside the
/// fully-independent pairs that exercise early divergence.
fn equality_case_strategy() -> impl Strategy<Value = (Keyed, RawValue, RawValue)> {
    spec_strategy().prop_flat_map(|spec| {
        let mut counter = 0_u8;
        let keyed = annotate(&spec, &mut counter);
        let left = raw_strategy(&keyed);
        (Just(keyed), left).prop_flat_map(|(keyed, left)| {
            let independent_right = raw_strategy(&keyed);
            let cloned_right = Just(left.clone());
            let perturbed_right = perturb_one_leaf_strategy(left.clone());
            let right = prop_oneof![independent_right, cloned_right, perturbed_right];
            (Just(keyed), Just(left), right)
        })
    })
}

/// The occurrence-pair count computed purely from the generated shape and values -- never calling
/// into `src/exact/equality.rs` -- so the property below has an oracle independent of
/// `plan_pairs`, not just a second caller of it. Mirrors FR-008's Behavior section: a leaf pair
/// (`Boolean`/`Integer`) is 1; an `Option` pair is `1 + inner` when both sides are present,
/// otherwise 1; a `Sequence` pair is 1 when the two sides' lengths differ, otherwise `1 + the sum
/// of its element pairs`; and a tuple/record `Pair` is always `1 + the sum of its field pairs`.
fn independent_pair_count(spec: &Keyed, left: &RawValue, right: &RawValue) -> u64 {
    match (spec, left, right) {
        (Keyed::Bool, RawValue::Bool(_), RawValue::Bool(_))
        | (Keyed::Int, RawValue::Int(_), RawValue::Int(_)) => 1,
        (Keyed::Opt(inner), RawValue::Opt(left), RawValue::Opt(right)) => match (left, right) {
            (Some(left), Some(right)) => 1 + independent_pair_count(inner, left, right),
            _ => 1,
        },
        (Keyed::List(inner), RawValue::List(left), RawValue::List(right)) => {
            if left.len() != right.len() {
                1
            } else {
                1 + left
                    .iter()
                    .zip(right)
                    .map(|(left, right)| independent_pair_count(inner, left, right))
                    .sum::<u64>()
            }
        }
        (Keyed::Pair(_, left_spec, right_spec), RawValue::Pair(ll, lr), RawValue::Pair(rl, rr)) => {
            1 + independent_pair_count(left_spec, ll, rl)
                + independent_pair_count(right_spec, lr, rr)
        }
        _ => unreachable!("a RawValue is always generated from the Keyed spec it is paired with"),
    }
}

/// One leaf of `raw`, in the same pre-order `perturb_at` walks, changed to a different value of
/// its own type; every other leaf is left untouched. `None` is a no-op count: an `Option::None`
/// payload and an empty `Sequence` contribute no leaf to perturb.
fn count_leaves(raw: &RawValue) -> usize {
    match raw {
        RawValue::Bool(_) | RawValue::Int(_) => 1,
        RawValue::Opt(None) => 0,
        RawValue::Opt(Some(inner)) => count_leaves(inner),
        RawValue::List(elements) => elements.iter().map(count_leaves).sum(),
        RawValue::Pair(left, right) => count_leaves(left) + count_leaves(right),
    }
}

/// Change the `target`-th leaf (0-based, pre-order) of `raw` to a different value of the same
/// type; `usize::MAX` is the "already consumed" sentinel carried after that leaf is found, so every
/// later leaf passes through unchanged.
fn perturb_at(raw: &RawValue, target: usize) -> (RawValue, usize) {
    if target == usize::MAX {
        return (raw.clone(), usize::MAX);
    }
    match raw {
        RawValue::Bool(b) => {
            if target == 0 {
                (RawValue::Bool(!b), usize::MAX)
            } else {
                (raw.clone(), target - 1)
            }
        }
        RawValue::Int(n) => {
            if target == 0 {
                let bumped = if *n == 1000 { n - 1 } else { n + 1 };
                (RawValue::Int(bumped), usize::MAX)
            } else {
                (raw.clone(), target - 1)
            }
        }
        RawValue::Opt(None) => (RawValue::Opt(None), target),
        RawValue::Opt(Some(inner)) => {
            let (inner, remaining) = perturb_at(inner, target);
            (RawValue::Opt(Some(Box::new(inner))), remaining)
        }
        RawValue::List(elements) => {
            let mut perturbed = Vec::with_capacity(elements.len());
            let mut remaining = target;
            for element in elements {
                let (element, next_remaining) = perturb_at(element, remaining);
                perturbed.push(element);
                remaining = next_remaining;
            }
            (RawValue::List(perturbed), remaining)
        }
        RawValue::Pair(left, right) => {
            let (left, remaining) = perturb_at(left, target);
            let (right, remaining) = perturb_at(right, remaining);
            (RawValue::Pair(Box::new(left), Box::new(right)), remaining)
        }
    }
}

/// `left` with one generated leaf index (chosen uniformly over its leaf count) perturbed; a
/// no-op when `left` has no leaf to perturb (an all-`None`/all-empty tree).
fn perturb_one_leaf_strategy(left: RawValue) -> impl Strategy<Value = RawValue> {
    let leaves = count_leaves(&left).max(1);
    (0..leaves).prop_map(move |index| perturb_at(&left, index).0)
}

proptest! {
    /// `plan_equality`'s occurrence-pair count and the number of `equality.pair` charges
    /// `CheckedEquality::evaluate` admits must always agree, over generated `Boolean`, `Integer`,
    /// `Option`, bounded `Sequence` and tuple-record composite values, run under an unlimited meter
    /// so every evaluation completes. This is the "otherwise" schedule FR-008's Behavior section
    /// describes (`equality.plan-form`, `equality.plan`, one `equality.pair` per planned pair,
    /// `equality.result-retain`); FR-008-AC-5 is what traces this test to that requirement. Both
    /// the planned count and the admitted-charge count are also checked against
    /// `independent_pair_count`, computed from the generated shape and values alone, so a counting
    /// bug shared by `plan_equality` and `evaluate`'s own planning (both call `plan_pairs`) cannot
    /// pass by agreeing with itself.
    ///
    /// Trace: TC-026, FR-008-AC-5
    #[test]
    fn tc_026_evaluate_admits_exactly_the_pairs_plan_equality_predicts(
        (spec, raw_left, raw_right) in equality_case_strategy()
    ) {
        let mut decls = Vec::new();
        let value_type = build_type(&spec, &mut decls);
        let env = TypeEnvironment::new(decls, Vec::new())
            .expect("freshly keyed, acyclic tuple declarations always admit");

        let left = materialize(&spec, &raw_left, &env);
        let right = materialize(&spec, &raw_right, &env);

        let plan = quire_contract_runtime::exact::plan_equality(&left, &right)
            .expect("no Value::Reference appears in this generator, so ForeignReference cannot fire");
        let predicted_pairs = plan
            .pair_events()
            .to_u64()
            .expect("a depth-3, width-4-bounded tree never plans more than u64::MAX pairs");

        let independent_pairs = independent_pair_count(&spec, &raw_left, &raw_right);
        prop_assert_eq!(
            predicted_pairs, independent_pairs,
            "plan_equality's predicted pair count disagreed with the independent count"
        );

        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(value_type.clone()),
                EqualityOperand::typed(value_type),
            )
            .expect("two operands of the same type always admit an equality schedule");
        prop_assert_eq!(checked.schedule(), EqualitySchedule::Plan);

        let mut meter = Meter::new(UNLIMITED);
        let outcome = checked.evaluate(&left, &right, &mut meter);
        prop_assert!(matches!(outcome, Outcome::Completed(_)), "unlimited meter, got {outcome:?}");
        prop_assert!(
            !meter.charge_log_truncated(),
            "the charge log capacity assumption below (every admitted equality.pair charge is \
             actually in admitted_charges) does not hold for this case"
        );

        let admitted_pairs = meter
            .admitted_charges()
            .iter()
            .filter(|point| **point == ChargePoint::EqualityPair)
            .count() as u64;

        prop_assert_eq!(admitted_pairs, predicted_pairs);
    }
}
