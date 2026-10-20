//! FR-007 exact rounding, division, Euclidean `mod`, and quantity semantics through the public
//! `exact` surface. Quantity evaluation remains runtime-owned during FR-275.
//!
//! IEEE exceptional, rational membership, and decimal normalization cases are kernel-owned.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    evaluate_quantity, Quantity, QuantityOperation, QuantityUnit, UnitDeclaration, UnitGraph,
};
use quire_exact::NodeKey;
use quire_exact::{
    divide, evaluate_decimal, modulo, Decimal, DecimalOperation, DecimalType, DivisionMember,
    DivisionProfile, IllTypedCause, InexactTarget, Integer, IntegerDomain, IntegerInterval, Meter,
    Outcome, Rational, Refusal, RoundingMode, ScalarLimits,
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

fn ratio(numerator: i128, denominator: i128) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

fn int_domain(lower: i128, upper: i128) -> IntegerDomain {
    IntegerDomain::Bounded(IntegerInterval::new(int(lower), int(upper)).unwrap())
}

fn decimal_target(mode: RoundingMode) -> DecimalType {
    DecimalType::new(int(-100), int(100), 0, 0, mode).unwrap()
}

fn decimal_divide(
    numerator: i128,
    denominator: i128,
    mode: RoundingMode,
) -> Outcome<quire_exact::DecimalResult> {
    let left = Decimal::new(int(numerator), 0);
    let right = Decimal::new(int(denominator), 0);
    evaluate_decimal(
        DecimalOperation::Divide(&left, &right),
        &decimal_target(mode),
        &mut Meter::new(UNLIMITED),
    )
}

#[trace("TC-034", "FR-007-AC-8")]
#[test]
fn tc_034_rounding_modes_select_signed_tie_neighbours() {
    assert_eq!(RoundingMode::ALL.len(), 6);
    // Columns follow the exhaustive mode match below: Exact, TowardZero,
    // TowardPositive, TowardNegative, NearestEven, NearestAway.
    let cases = [
        (1, [None, Some(0), Some(1), Some(0), Some(0), Some(1)]),
        (-1, [None, Some(0), Some(0), Some(-1), Some(0), Some(-1)]),
        (3, [None, Some(1), Some(2), Some(1), Some(2), Some(2)]),
        (-3, [None, Some(-1), Some(-1), Some(-2), Some(-2), Some(-2)]),
    ];
    for mode in RoundingMode::ALL {
        let column = match mode {
            RoundingMode::Exact => 0,
            RoundingMode::TowardZero => 1,
            RoundingMode::TowardPositive => 2,
            RoundingMode::TowardNegative => 3,
            RoundingMode::NearestEven => 4,
            RoundingMode::NearestAway => 5,
        };
        for (numerator, expected) in cases {
            let Some(expected) = expected[column] else {
                assert_eq!(
                    decimal_divide(numerator, 2, mode),
                    Outcome::Refused(Refusal::InexactDecimal {
                        target: InexactTarget::Decimal(Box::new(decimal_target(mode))),
                    }),
                );
                continue;
            };
            let result = decimal_divide(numerator, 2, mode)
                .completed()
                .expect("rounded tie");
            assert_eq!(
                result.value().representation().coefficient(),
                &int(expected),
                "{mode:?}, {numerator}"
            );
        }
    }
}

#[trace("TC-034", "FR-007-AC-8")]
#[test]
fn tc_034_rounding_mode_spellings_and_default() {
    assert_eq!(RoundingMode::ALL.len(), 6);
    assert_eq!(RoundingMode::default(), RoundingMode::Exact);
    for mode in RoundingMode::ALL {
        let spelling = match mode {
            RoundingMode::Exact => "exact",
            RoundingMode::TowardZero => "toward-zero",
            RoundingMode::TowardPositive => "toward-positive",
            RoundingMode::TowardNegative => "toward-negative",
            RoundingMode::NearestEven => "nearest-even",
            RoundingMode::NearestAway => "nearest-away",
        };
        assert_eq!(mode.as_str(), spelling);
        assert_eq!(RoundingMode::from_code(spelling), Some(mode));
    }
}

#[trace("TC-034", "FR-007-AC-15")]
#[test]
fn tc_034_division_rounds_by_quotient_sign_and_records_exact_loss() {
    for (a, b, mode, coefficient, exact) in [
        (-1, -3, RoundingMode::TowardZero, 0, ratio(1, 3)),
        (1, -3, RoundingMode::TowardZero, 0, ratio(-1, 3)),
        (-1, -2, RoundingMode::NearestAway, 1, ratio(1, 2)),
    ] {
        let result = decimal_divide(a, b, mode)
            .completed()
            .expect("rounded quotient");
        assert_eq!(
            result.value().representation().coefficient(),
            &int(coefficient)
        );
        assert_eq!(result.loss().expect("inexact quotient").exact(), exact);
    }
}

fn dim_length() -> NodeKey {
    NodeKey::from_digest([1; 32])
}
fn dim_temperature() -> NodeKey {
    NodeKey::from_digest([2; 32])
}
fn unit_meter() -> NodeKey {
    NodeKey::from_digest([10; 32])
}
fn unit_kilometer() -> NodeKey {
    NodeKey::from_digest([11; 32])
}
fn unit_kelvin() -> NodeKey {
    NodeKey::from_digest([20; 32])
}
fn unit_celsius() -> NodeKey {
    NodeKey::from_digest([21; 32])
}

/// A length dimension with a root `meter` and a non-root `kilometer`, and a
/// temperature dimension with a root `kelvin` and an affine `celsius`
/// (nonzero composed offset), for the quantity type-check tests.
fn graph() -> UnitGraph {
    let one = Rational::from_integer(Integer::one());
    let zero = Rational::from_integer(Integer::zero());
    UnitGraph::admit(
        [
            (dim_length(), Vec::<(NodeKey, Integer)>::new()),
            (dim_temperature(), Vec::new()),
        ],
        [
            (
                unit_meter(),
                UnitDeclaration {
                    dimension: dim_length(),
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                unit_kilometer(),
                UnitDeclaration {
                    dimension: dim_length(),
                    target: Some(unit_meter()),
                    scale: ratio(1000, 1),
                    offset: zero.clone(),
                },
            ),
            (
                unit_kelvin(),
                UnitDeclaration {
                    dimension: dim_temperature(),
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                unit_celsius(),
                UnitDeclaration {
                    dimension: dim_temperature(),
                    target: Some(unit_kelvin()),
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

#[trace("TC-034", "FR-007-AC-12")]
#[test]
fn tc_034_quantity_add_subtract_cause_order_first_failure_wins() {
    let graph = graph();
    let length = quantity(&graph, unit_meter(), 1);
    let kilometers = quantity(&graph, unit_kilometer(), 1);
    let celsius = quantity(&graph, unit_celsius(), 1);
    let kelvin = quantity(&graph, unit_kelvin(), 1);

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

#[trace("TC-034", "FR-007-AC-12")]
#[test]
fn tc_034_quantity_multiply_divide_and_power_return_canonical_results() {
    let graph = graph();
    let length = quantity(&graph, unit_kilometer(), 2);
    let kelvin = quantity(&graph, unit_kelvin(), 3);

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
                    .compound_unit(&[(unit_meter(), int(1)), (unit_kelvin(), int(1))])
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
                    .compound_unit(&[(unit_meter(), int(1)), (unit_kelvin(), int(-1))])
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
            QuantityUnit::Compound(graph.compound_unit(&[(unit_meter(), int(2))]).unwrap()),
        ))
    );

    let mut meter = Meter::new(UNLIMITED);
    let reciprocal = evaluate_quantity(QuantityOperation::Power(&length, &int(-1)), &mut meter)
        .expect("negative power is defined for a nonzero base");
    assert_eq!(
        reciprocal.completed(),
        Some(Quantity::new(
            ratio(1, 2000),
            QuantityUnit::Compound(graph.compound_unit(&[(unit_meter(), int(-1))]).unwrap()),
        ))
    );

    let length = quantity(&graph, unit_meter(), 5);
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

#[trace("TC-034", "FR-007-AC-12")]
#[test]
fn tc_034_quantity_zero_power_and_affine_refusal() {
    let graph = graph();
    let zero = quantity(&graph, unit_meter(), 0);
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

    let celsius = quantity(&graph, unit_celsius(), 1);
    let kelvin = quantity(&graph, unit_kelvin(), 1);
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

#[trace("TC-034", "FR-007-AC-16")]
#[test]
fn tc_034_affine_power_refuses_zero_and_negative_exponents_without_charges() {
    let graph = graph();
    let celsius = quantity(&graph, unit_celsius(), 1);
    for exponent in [0, -1] {
        let mut meter = Meter::new(UNLIMITED);
        let error = evaluate_quantity(
            QuantityOperation::Power(&celsius, &int(exponent)),
            &mut meter,
        )
        .unwrap_err();
        assert_eq!(error.cause, IllTypedCause::AffineUnitArithmetic);
        assert!(meter.admitted_charges().is_empty());
    }
}

#[trace("TC-034", "FR-007-AC-17")]
#[test]
fn tc_034_distinct_units_of_same_dimension_multiply_and_divide() {
    let graph = graph();
    let kilometers = quantity(&graph, unit_kilometer(), 2);
    let meters = quantity(&graph, unit_meter(), 3);
    let product = evaluate_quantity(
        QuantityOperation::Multiply(&kilometers, &meters),
        &mut Meter::new(UNLIMITED),
    )
    .expect("same-dimension product");
    assert_eq!(
        product.completed(),
        Some(Quantity::new(
            ratio(6000, 1),
            QuantityUnit::Compound(graph.compound_unit(&[(unit_meter(), int(2))]).unwrap()),
        ))
    );
    let quotient = evaluate_quantity(
        QuantityOperation::Divide(&kilometers, &meters),
        &mut Meter::new(UNLIMITED),
    )
    .expect("same-dimension quotient");
    assert_eq!(
        quotient.completed(),
        Some(Quantity::new(
            ratio(2000, 3),
            QuantityUnit::Compound(graph.compound_unit(&[]).unwrap()),
        ))
    );
}

#[trace("TC-034", "FR-007-AC-18")]
#[test]
fn tc_034_modulo_is_euclidean_in_all_sign_quadrants_and_refuses_outside_domain() {
    let domain = int_domain(0, 2);
    for (a, b, expected) in [(7, 3, 1), (-7, 3, 2), (7, -3, 1), (-7, -3, 2)] {
        assert_eq!(
            modulo(&int(a), &int(b), &domain, &mut Meter::new(UNLIMITED)),
            Outcome::Completed(int(expected)),
        );
    }
    let refused = int_domain(0, 1);
    assert_eq!(
        modulo(&int(-7), &int(3), &refused, &mut Meter::new(UNLIMITED)),
        Outcome::Refused(Refusal::ModuloOutOfDomain {
            domain: Box::new(IntegerInterval::new(int(0), int(1)).unwrap()),
        }),
    );
}

#[trace("TC-034", "FR-007-AC-19")]
#[test]
fn tc_034_division_profiles_admit_only_exposed_member() {
    assert_eq!(DivisionProfile::ALL.len(), 3);
    for profile in DivisionProfile::ALL {
        // A new profile variant must fail this exhaustive match at compile time.
        match profile {
            DivisionProfile::Truncating | DivisionProfile::Floor | DivisionProfile::Euclidean => {}
        }
        let cases = [
            (
                DivisionMember::Quotient,
                10,
                5,
                int_domain(1, 10),
                Outcome::Completed(int(2)),
            ),
            (
                DivisionMember::Remainder,
                -10,
                -1,
                int_domain(-10, 5),
                Outcome::Completed(int(0)),
            ),
            (
                DivisionMember::Quotient,
                -10,
                -1,
                int_domain(-10, 5),
                Outcome::Refused(Refusal::DivisionOutOfDomain {
                    domain: Box::new(IntegerInterval::new(int(-10), int(5)).unwrap()),
                    member: DivisionMember::Quotient,
                }),
            ),
            (
                DivisionMember::Remainder,
                9,
                10,
                int_domain(0, 5),
                Outcome::Refused(Refusal::DivisionOutOfDomain {
                    domain: Box::new(IntegerInterval::new(int(0), int(5)).unwrap()),
                    member: DivisionMember::Remainder,
                }),
            ),
        ];
        for (member, a, b, domain, expected) in cases {
            assert_eq!(
                divide(
                    profile,
                    member,
                    &int(a),
                    &int(b),
                    &domain,
                    &mut Meter::new(UNLIMITED)
                ),
                expected,
                "{profile:?}, {member:?}, {a}, {b}"
            );
        }
    }
}
