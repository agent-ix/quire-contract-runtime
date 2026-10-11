//! FR-007 exact semantics of the unit graph and quantity type-fault order that the runtime still
//! defines (the interim residue of FR-275), through the public `exact` surface.
//!
//! The decimal rounding, IEEE exceptional, rational membership, Euclidean `mod` and decimal
//! normalization cases left with the kernel to `quire-exact` (FR-275).
#![cfg(feature = "exact")]

use quire_contract_runtime::exact::{
    evaluate_quantity, NodeKey, Quantity, QuantityOperation, QuantityUnit, UnitDeclaration,
    UnitGraph,
};
use quire_exact::{IllTypedCause, Integer, Meter, Rational, ScalarLimits};

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

fn ratio(numerator: i128, denominator: i128) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

const DIM_LENGTH: NodeKey = NodeKey::from_bytes([1; 32]);
const DIM_TEMPERATURE: NodeKey = NodeKey::from_bytes([2; 32]);
const UNIT_METER: NodeKey = NodeKey::from_bytes([10; 32]);
const UNIT_KILOMETER: NodeKey = NodeKey::from_bytes([11; 32]);
const UNIT_KELVIN: NodeKey = NodeKey::from_bytes([20; 32]);
const UNIT_CELSIUS: NodeKey = NodeKey::from_bytes([21; 32]);

/// A length dimension with a root `meter` and a non-root `kilometer`, and a
/// temperature dimension with a root `kelvin` and an affine `celsius`
/// (nonzero composed offset), for the quantity type-check tests.
fn graph() -> UnitGraph {
    let one = Rational::from_integer(Integer::one());
    let zero = Rational::from_integer(Integer::zero());
    UnitGraph::admit(
        [
            (DIM_LENGTH, Vec::<(NodeKey, Integer)>::new()),
            (DIM_TEMPERATURE, Vec::new()),
        ],
        [
            (
                UNIT_METER,
                UnitDeclaration {
                    dimension: DIM_LENGTH,
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_KILOMETER,
                UnitDeclaration {
                    dimension: DIM_LENGTH,
                    target: Some(UNIT_METER),
                    scale: ratio(1000, 1),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_KELVIN,
                UnitDeclaration {
                    dimension: DIM_TEMPERATURE,
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_CELSIUS,
                UnitDeclaration {
                    dimension: DIM_TEMPERATURE,
                    target: Some(UNIT_KELVIN),
                    scale: one,
                    offset: ratio(273, 1),
                },
            ),
        ],
    )
    .unwrap()
}

fn quantity(graph: &UnitGraph, unit_key: NodeKey, value: i128) -> Quantity {
    let unit = graph.unit(unit_key).unwrap().clone();
    Quantity::new(ratio(value, 1), QuantityUnit::Declared(Box::new(unit)))
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_quantity_add_subtract_cause_order_first_failure_wins() {
    let graph = graph();
    let length = quantity(&graph, UNIT_METER, 1);
    let kilometers = quantity(&graph, UNIT_KILOMETER, 1);
    let celsius = quantity(&graph, UNIT_CELSIUS, 1);
    let kelvin = quantity(&graph, UNIT_KELVIN, 1);

    // Incompatible dimensions and an affine operand both fail; the dimension
    // check wins because it runs first.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&length, &celsius), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::IncompatibleDimensions);
    assert!(meter.admitted_charges().is_empty());

    // Subtract checks in the identical order.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Subtract(&length, &celsius), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::IncompatibleDimensions);
    assert!(meter.admitted_charges().is_empty());

    // Compatible dimension, but an affine operand and distinct units both
    // fail; the affine check wins because it runs before the units check.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&celsius, &kelvin), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::AffineUnitArithmetic);
    assert!(meter.admitted_charges().is_empty());

    // Compatible dimension, neither operand affine, distinct units: the
    // units check is reached last.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&length, &kilometers), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::DistinctUnits);
    assert!(meter.admitted_charges().is_empty());

    // Every one of these is `ill_typed` with zero charges, reported beside
    // the outcome rather than inside a result: `evaluate_quantity` returns
    // `Err` before `Ok(Outcome::..)`.
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_quantity_multiply_divide_and_power_return_canonical_results() {
    let graph = graph();
    let length = quantity(&graph, UNIT_KILOMETER, 2);
    let kelvin = quantity(&graph, UNIT_KELVIN, 3);

    // Cross-dimension operands would fail `Add`/`Subtract`'s incompatible-
    // dimensions check. Arithmetic converts declared units to canonical roots.
    let mut meter = Meter::new(UNLIMITED);
    let product = evaluate_quantity(QuantityOperation::Multiply(&length, &kelvin), &mut meter)
        .expect("multiply is well-typed across dimensions");
    assert_eq!(
        product.completed(),
        Some(Quantity::new(
            ratio(6000, 1),
            QuantityUnit::Compound(
                graph
                    .compound_unit(&[(UNIT_METER, int(1)), (UNIT_KELVIN, int(1))])
                    .unwrap(),
            ),
        ))
    );

    let mut meter = Meter::new(UNLIMITED);
    let quotient = evaluate_quantity(QuantityOperation::Divide(&length, &kelvin), &mut meter)
        .expect("divide is well-typed across dimensions");
    assert_eq!(
        quotient.completed(),
        Some(Quantity::new(
            ratio(2000, 3),
            QuantityUnit::Compound(
                graph
                    .compound_unit(&[(UNIT_METER, int(1)), (UNIT_KELVIN, int(-1))])
                    .unwrap(),
            ),
        ))
    );

    let mut meter = Meter::new(UNLIMITED);
    let power = evaluate_quantity(QuantityOperation::Power(&length, &int(2)), &mut meter)
        .expect("power is well-typed for a zero-offset unit");
    assert_eq!(
        power.completed(),
        Some(Quantity::new(
            ratio(4_000_000, 1),
            QuantityUnit::Compound(graph.compound_unit(&[(UNIT_METER, int(2))]).unwrap()),
        ))
    );

    let mut meter = Meter::new(UNLIMITED);
    let reciprocal = evaluate_quantity(QuantityOperation::Power(&length, &int(-1)), &mut meter)
        .expect("negative power is defined for a nonzero base");
    assert_eq!(
        reciprocal.completed(),
        Some(Quantity::new(
            ratio(1, 2000),
            QuantityUnit::Compound(graph.compound_unit(&[(UNIT_METER, int(-1))]).unwrap()),
        ))
    );

    let length = quantity(&graph, UNIT_METER, 5);
    let mut meter = Meter::new(UNLIMITED);
    let cancellation = evaluate_quantity(QuantityOperation::Divide(&length, &length), &mut meter)
        .expect("equal units cancel");
    assert_eq!(
        cancellation.completed(),
        Some(Quantity::new(
            ratio(1, 1),
            QuantityUnit::Compound(graph.compound_unit(&[]).unwrap()),
        ))
    );
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_quantity_zero_power_and_affine_refusal() {
    let graph = graph();
    let zero = quantity(&graph, UNIT_METER, 0);
    let mut meter = Meter::new(UNLIMITED);
    let zero_power = evaluate_quantity(QuantityOperation::Power(&zero, &int(0)), &mut meter)
        .expect("zero to the zero power is defined");
    assert_eq!(
        zero_power.completed(),
        Some(Quantity::new(
            ratio(1, 1),
            QuantityUnit::Compound(graph.compound_unit(&[]).unwrap()),
        ))
    );

    let mut meter = Meter::new(UNLIMITED);
    let undefined = evaluate_quantity(QuantityOperation::Power(&zero, &int(-1)), &mut meter)
        .expect("zero under a negative exponent is a runtime stop");
    assert_eq!(
        undefined,
        quire_exact::Outcome::Undefined(quire_exact::Undefined::DivisionByZero)
    );

    let celsius = quantity(&graph, UNIT_CELSIUS, 1);
    let kelvin = quantity(&graph, UNIT_KELVIN, 1);
    for operation in [
        QuantityOperation::Multiply(&celsius, &kelvin),
        QuantityOperation::Multiply(&kelvin, &celsius),
        QuantityOperation::Divide(&celsius, &kelvin),
        QuantityOperation::Divide(&kelvin, &celsius),
        QuantityOperation::Power(&celsius, &int(2)),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        let error = evaluate_quantity(operation, &mut meter).unwrap_err();
        assert_eq!(error.cause, IllTypedCause::AffineUnitArithmetic);
        assert!(meter.admitted_charges().is_empty());
    }
}
