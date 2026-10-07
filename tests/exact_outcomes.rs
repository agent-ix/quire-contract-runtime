//! FR-006 typed exact outcomes and `quire.value.accounting/v1` metering of the operations the
//! runtime still defines (the interim residue of FR-275), through the public `exact` surface.
//!
//! The kernel `Outcome`, `Refusal`, `Meter` and injected-denial tests left with the kernel to the
//! `quire-exact` crate (FR-275; the matrix section "Evidence at the kernel move"). What stays is
//! what only this crate defines: the charge schedules of `CheckedEquality`, `CheckedPackage`,
//! collection and composite construction, enumeration comparison and quantity arithmetic.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;
use std::num::NonZeroU64;

use quire_contract_runtime::exact::{
    compare_enum, construct_collection, evaluate_quantity, CheckMode, CheckingLimits,
    CollectionType, CompositeDeclaration, CompositeShape, Deferred, EnumDeclaration,
    EqualityOperand, EqualityOperator, FunctionDeclaration, NodeKey, ObjectEnvironment,
    PackageDeclarations, Quantity, QuantityOperation, QuantityUnit, TypeEnvironment,
    UnitDeclaration, UnitGraph, Value, ValueType,
};
use quire_exact::{
    CardinalityBound, ChargePoint, CollectionKind, ComparisonOperator, Incomplete, InjectedDenial,
    Integer, LimitKind, Meter, Outcome, Rational, ScalarLimits,
};

const UNLIMITED: ScalarLimits = limits([u64::MAX; 10]);

const fn limits(v: [u64; 10]) -> ScalarLimits {
    ScalarLimits {
        integer_bits: v[0],
        decimal_digits: v[1],
        scale_expansion: v[2],
        text_input_bytes: v[3],
        text_scalars: v[4],
        normalized_scalars: v[5],
        unit_edges: v[6],
        value_occurrences: v[7],
        work_units: v[8],
        result_units: v[9],
    }
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

/// Unwrap an [`Outcome`] known to be [`Outcome::Incomplete`]; panics with the
/// disposition otherwise. `T` need not be `Debug`: only the non-`Incomplete`
/// dispositions, which are `Debug` regardless of `T`, are ever formatted.
fn expect_incomplete<T>(outcome: Outcome<T>) -> Incomplete {
    match outcome {
        Outcome::Incomplete(record) => record,
        Outcome::Completed(_) => panic!("expected Outcome::Incomplete, got Completed"),
        Outcome::Undefined(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
        Outcome::Refused(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
    }
}

// One driver per admitted charge point family the runtime itself charges, run under `UNLIMITED`
// so every charge before the point under test is genuinely admitted. Each returns the
// `Incomplete` an injected denial at that call's first matching point produces.

/// A same-type equality between two `Integer` operands selects the
/// occurrence-pair plan schedule, reaching `equality.plan-form`,
/// `equality.plan`, `equality.pair` and `equality.result-retain` in order.
fn drive_equality_plan(meter: &mut Meter) -> Incomplete {
    let env = TypeEnvironment::new(Vec::new(), Vec::new()).unwrap();
    let checked = env
        .check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Integer),
            EqualityOperand::typed(ValueType::Integer),
        )
        .unwrap();
    expect_incomplete(checked.evaluate(&Value::Integer(int(3)), &Value::Integer(int(3)), meter))
}

/// A one-element `Sequence[Integer]` construction: `collection.element` for
/// the element, then (a sequence never coalesces) `collection.bound` and
/// `collection.result-retain`.
fn drive_collection_basic(meter: &mut Meter) -> Incomplete {
    let collection_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Integer,
        CardinalityBound::new(0, 10).unwrap(),
    );
    let elements: Vec<Deferred<'_>> = vec![Box::new(|_meter: &mut Meter| {
        Outcome::Completed(Value::Integer(int(1)))
    })];
    expect_incomplete(construct_collection(&collection_type, elements, meter))
}

/// A two-element `Set[Integer]` construction: the second occurrence is
/// compared against the first retained member, charging
/// `collection.member-walk` then `collection.member-test`.
fn drive_collection_membership(meter: &mut Meter) -> Incomplete {
    let collection_type = CollectionType::new(
        CollectionKind::Set,
        ValueType::Integer,
        CardinalityBound::new(0, 10).unwrap(),
    );
    let elements: Vec<Deferred<'_>> = vec![
        Box::new(|_meter: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
        Box::new(|_meter: &mut Meter| Outcome::Completed(Value::Integer(int(2)))),
    ];
    expect_incomplete(construct_collection(&collection_type, elements, meter))
}

/// A checked package of one nullary function, called through
/// `CheckedPackage::call`: the injected denial fires on the `function.call`
/// charge itself, strictly before the body (which would complete instead of
/// denying) ever runs.
fn drive_function_call(meter: &mut Meter) -> Incomplete {
    let package = PackageDeclarations {
        types: TypeEnvironment::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let evaluation = package
        .call("f", Vec::new(), &ObjectEnvironment::default(), meter)
        .unwrap();
    expect_incomplete(evaluation.outcome)
}

/// A one-position tuple construction reaches `composite.result-retain` once
/// its sole position completes.
fn drive_composite(meter: &mut Meter) -> Incomplete {
    let declaration = CompositeDeclaration::new(
        NodeKey::from_bytes([7; 32]),
        "Pair",
        CompositeShape::Tuple(vec![ValueType::Integer]),
    );
    let env = TypeEnvironment::new([declaration], Vec::new()).unwrap();
    let positions: Vec<Deferred<'_>> = vec![Box::new(|_meter: &mut Meter| {
        Outcome::Completed(Value::Integer(int(1)))
    })];
    expect_incomplete(
        env.evaluate_tuple(NodeKey::from_bytes([7; 32]), positions, meter)
            .unwrap(),
    )
}

fn drive_enum(meter: &mut Meter) -> Incomplete {
    let declaration =
        EnumDeclaration::new(NodeKey::from_bytes([9; 32]), true, &["a", "b"]).unwrap();
    let a = declaration
        .member("a", NodeKey::from_bytes([10; 32]))
        .unwrap();
    let b = declaration
        .member("b", NodeKey::from_bytes([11; 32]))
        .unwrap();
    expect_incomplete(compare_enum(ComparisonOperator::Equal, &a, &b, meter).unwrap())
}

/// `a ^ 2` of a quantity in a non-canonical (one-edge) unit: charges
/// `unit.identity-read`, `unit.edge`, `unit.rational-arithmetic` (from the
/// edge traversal and the power itself), `unit.target-domain` and
/// `unit.result-retain`, in that order.
fn drive_quantity(meter: &mut Meter) -> Incomplete {
    let dimension = NodeKey::from_bytes([1; 32]);
    let root = NodeKey::from_bytes([2; 32]);
    let derived = NodeKey::from_bytes([3; 32]);
    let graph = UnitGraph::admit(
        [(dimension, Vec::new())],
        [
            (
                root,
                UnitDeclaration {
                    dimension,
                    target: None,
                    scale: Rational::from_integer(int(1)),
                    offset: Rational::from_integer(int(0)),
                },
            ),
            (
                derived,
                UnitDeclaration {
                    dimension,
                    target: Some(root),
                    scale: Rational::new(int(1000), int(1)).unwrap(),
                    offset: Rational::from_integer(int(0)),
                },
            ),
        ],
    )
    .unwrap();
    let unit = graph.unit(derived).unwrap().clone();
    let quantity = Quantity::new(
        Rational::from_integer(int(5)),
        QuantityUnit::Declared(Box::new(unit)),
    );
    expect_incomplete(
        evaluate_quantity(QuantityOperation::Power(&quantity, &int(2)), meter).unwrap(),
    )
}

/// One `(point, driver)` entry of the AC-1 sweep below.
type Driver = fn(&mut Meter) -> Incomplete;

/// Trace: TC-031, FR-010-AC-1
///
/// An injected denial at occurrence one of every charge point the runtime's own operations
/// charge names exactly that point: `function.call` (through `CheckedPackage::call`/
/// `Frame::call`, FR-273), the `equality.*` occurrence-pair plan schedule, the `collection.*`
/// and `composite.result-retain` family, the enumeration `enum.*` points and the quantity
/// `unit.*` points. The kernel's own scalar points are the kernel's evidence, not this crate's.
#[test]
fn tc_031_injected_denial_at_occurrence_one_names_every_residue_charge_point() {
    let drivers: Vec<(ChargePoint, Driver)> = vec![
        (ChargePoint::FunctionCall, drive_function_call),
        (ChargePoint::EnumIdentityRead, drive_enum),
        (ChargePoint::EnumResultRetain, drive_enum),
        (ChargePoint::UnitIdentityRead, drive_quantity),
        (ChargePoint::UnitEdge, drive_quantity),
        (ChargePoint::UnitRationalArithmetic, drive_quantity),
        (ChargePoint::UnitTargetDomain, drive_quantity),
        (ChargePoint::UnitResultRetain, drive_quantity),
        (ChargePoint::EqualityPlanForm, drive_equality_plan),
        (ChargePoint::EqualityPlan, drive_equality_plan),
        (ChargePoint::EqualityPair, drive_equality_plan),
        (ChargePoint::EqualityResultRetain, drive_equality_plan),
        (ChargePoint::CollectionElement, drive_collection_basic),
        (ChargePoint::CollectionBound, drive_collection_basic),
        (ChargePoint::CollectionResultRetain, drive_collection_basic),
        (
            ChargePoint::CollectionMemberWalk,
            drive_collection_membership,
        ),
        (
            ChargePoint::CollectionMemberTest,
            drive_collection_membership,
        ),
        (ChargePoint::CompositeResultRetain, drive_composite),
    ];

    let covered: BTreeSet<ChargePoint> = drivers.iter().map(|(point, _)| *point).collect();
    assert_eq!(
        covered.len(),
        drivers.len(),
        "a charge point is driven twice"
    );

    for (point, drive) in drivers {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::MIN,
        });
        let record = drive(&mut meter);
        assert_eq!(record.limit_kind, LimitKind::WorkUnits, "{point:?}");
        assert_eq!(record.charge_point, point);
        assert_eq!(record.limit, record.consumed, "{point:?}");
        // FR-010's `equality.plan` bullet states the rule these per-point amounts follow:
        // `next_charge` is the amount each point's availability check was made against, which is
        // the charge's own work amount everywhere except `equality.plan` (`pairs + 2`,
        // reservation, not commit) and the two occurrence-derived points below (`occ(left) +
        // occ(right)`, at the drivers' minimal one-occurrence-per-side operands).
        let expected_next_charge = match point {
            ChargePoint::EqualityPlanForm | ChargePoint::CollectionMemberWalk => int(2),
            ChargePoint::EqualityPlan => int(3),
            _ => int(1),
        };
        assert_eq!(record.next_charge, expected_next_charge, "{point:?}");
        // The denied charge itself changed nothing: `work_units` after the call is exactly the
        // charges admitted before it (one work unit each), no result unit was retained, and the
        // point was not logged. The occurrence-derived-work points break this 1:1
        // correspondence for whatever comes after them in a driver's own chain:
        // `equality.plan-form` and `collection.member-walk` each admit for 2 work units instead
        // of 1 (`occ(left) + occ(right)` at 2 scalar operands), so a point immediately
        // downstream of one of them has consumed exactly one more work unit than it has prior
        // admitted charges. `equality.plan` (`Meter::charge_plan`) only *reserves* `pairs + 2`
        // to decide availability; on success it still commits a single work unit like any other
        // charge, so it adds no further drift once past `equality.plan-form`.
        assert_eq!(
            meter.consumed(LimitKind::WorkUnits),
            record.consumed,
            "{point:?}"
        );
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0, "{point:?}");
        let extra_prior_work = match point {
            ChargePoint::EqualityPlan
            | ChargePoint::EqualityPair
            | ChargePoint::EqualityResultRetain
            | ChargePoint::CollectionMemberTest => 1,
            _ => 0,
        };
        assert_eq!(
            meter.admitted_charges().len() + extra_prior_work,
            usize::try_from(record.consumed).unwrap(),
            "{point:?}"
        );
        assert!(!meter.admitted_charges().contains(&point), "{point:?}");
    }
}
