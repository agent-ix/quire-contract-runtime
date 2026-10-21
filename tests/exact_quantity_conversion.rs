//! RT acceptance of QSV's quantity conversion through its public owner API.
#![cfg(feature = "exact")]

use std::collections::BTreeMap;

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;

use ix_trace_rs::trace;
use quire_exact::{
    ChargePoint, IllTypedCause, Integer, LimitKind, Meter, NodeKey, Outcome, Quantity, Rational,
    ScalarLimits, UnitId,
};
use quire_semantic_value::quantity::{
    convert_quantity, evaluate_quantity, ConvertedValue, QuantityOperation, QuantityRefusal,
    QuantityTarget, QuantityUnit, UnitTable,
};
use quire_semantic_value::unit::{DimensionNode, NominalDeclaration, UnitGraph, UnitNode};

fn key(byte: u8) -> NodeKey {
    NodeKey::from_digest([byte; 32])
}

fn int(value: i64) -> Integer {
    Integer::from(value)
}

fn ratio(numerator: i64, denominator: i64) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

fn meter() -> Meter {
    Meter::new(ScalarLimits {
        integer_bits: 1024,
        decimal_digits: 1024,
        scale_expansion: 1024,
        text_input_bytes: 1024,
        text_scalars: 1024,
        normalized_scalars: 1024,
        unit_edges: 1024,
        value_occurrences: 1024,
        work_units: 1024,
        result_units: 1024,
    })
}

fn graph() -> UnitGraph {
    let dimension = |terms: Vec<([u8; 32], Integer)>| {
        DimensionNode::checked(terms, NominalDeclaration::default()).unwrap()
    };
    let dimensions = BTreeMap::from([
        (key(1), dimension(vec![])), // length
        (key(2), dimension(vec![])), // time
        (key(3), dimension(vec![])), // temperature
        (key(4), dimension(vec![])), // mass
        (
            key(5),
            dimension(vec![
                ([1; 32], int(2)),
                ([2; 32], int(-2)),
                ([4; 32], int(1)),
            ]),
        ), // energy
        (
            key(6),
            dimension(vec![
                ([1; 32], int(2)),
                ([2; 32], int(-2)),
                ([4; 32], int(1)),
            ]),
        ), // torque
        (key(7), dimension(vec![([1; 32], int(2))])), // area
        (key(8), dimension(vec![])), // distinct length-shaped dimension
    ]);
    let unit = |dimension: u8, target: Option<u8>, scale: Rational, offset: Rational| {
        UnitNode::checked(
            [dimension; 32],
            target.map(|byte| [byte; 32]),
            scale,
            offset,
            NominalDeclaration::default(),
        )
        .unwrap()
    };
    let units = BTreeMap::from([
        (key(11), unit(1, None, ratio(1, 1), ratio(0, 1))), // m
        (key(12), unit(1, Some(11), ratio(1, 100), ratio(0, 1))), // cm
        (key(13), unit(2, None, ratio(1, 1), ratio(0, 1))), // s
        (key(14), unit(3, None, ratio(1, 1), ratio(0, 1))), // K
        (key(15), unit(3, Some(14), ratio(1, 1), ratio(5463, 20))), // degC
        (key(16), unit(1, Some(11), ratio(1000, 1), ratio(0, 1))), // km
        (key(17), unit(1, Some(11), ratio(127, 5000), ratio(0, 1))), // inch
        (key(18), unit(3, Some(14), ratio(5, 9), ratio(45967, 180))), // degF
        (key(19), unit(3, Some(14), ratio(1, 1), ratio(10, 1))), // u1
        (key(20), unit(3, Some(19), ratio(1, 1), ratio(-10, 1))), // u2
        (key(21), unit(3, Some(15), ratio(1, 1), ratio(0, 1))), // u3
        (key(22), unit(4, None, ratio(1, 1), ratio(0, 1))), // kg
        (key(23), unit(6, None, ratio(1, 1), ratio(0, 1))), // N_m
        (key(24), unit(5, None, ratio(1, 1), ratio(0, 1))), // J
        (key(25), unit(7, None, ratio(1, 1), ratio(0, 1))), // m2
        (key(26), unit(7, Some(25), ratio(1, 10000), ratio(0, 1))), // cm2
        (key(27), unit(8, None, ratio(1, 1), ratio(0, 1))), // alike root
    ]);
    UnitGraph::from_checked_nodes(&dimensions, &units).unwrap()
}

fn quantity(unit: u8, numerator: i64, denominator: i64) -> Quantity {
    Quantity::new(ratio(numerator, denominator), UnitId::declared(key(unit)))
}

#[trace("TC-201", "FR-007-AC-20", "FR-007-AC-22")]
#[test]
fn tc_201_qsv_exact_conversion_composes_declared_and_affine_paths() {
    let graph = graph();
    let mut table = UnitTable::declared(&graph);
    for (source, target, expected) in [
        (quantity(12, 250, 1), 11, ratio(5, 2)),
        (quantity(15, 0, 1), 14, ratio(5463, 20)),
        (quantity(18, 32, 1), 14, ratio(5463, 20)),
        (quantity(20, 1, 1), 19, ratio(-9, 1)),
        (quantity(14, 5463, 20), 15, ratio(0, 1)),
        (quantity(15, 0, 1), 18, ratio(32, 1)),
    ] {
        let Outcome::Completed(conversion) = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(target))).unwrap(),
            &QuantityTarget::Exact,
            &mut meter(),
        )
        .unwrap() else {
            panic!("admitted exact conversion completes");
        };
        assert_eq!(conversion.value(), &ConvertedValue::Exact(expected));
        assert_eq!(
            conversion.unit(),
            table.get(UnitId::declared(key(target))).unwrap()
        );
    }
    let square_id = table
        .insert(QuantityUnit::Compound(
            graph.compound_unit(&[([11; 32], int(2))]).unwrap(),
        ))
        .unwrap();
    let square = Quantity::new(ratio(4, 1), square_id);
    let Outcome::Completed(converted) = convert_quantity(
        table.resolve(&square).unwrap(),
        table.get(UnitId::declared(key(25))).unwrap(),
        &QuantityTarget::Exact,
        &mut meter(),
    )
    .unwrap() else {
        panic!("compound square converts to declared area");
    };
    assert_eq!(converted.value(), &ConvertedValue::Exact(ratio(4, 1)));
}

#[trace("TC-201", "FR-007-AC-20")]
#[test]
fn tc_201_qsv_fraction_sweep_matches_independent_centimetre_oracle() {
    let table = UnitTable::declared(&graph());
    let target = table.get(UnitId::declared(key(11))).unwrap();
    for numerator in -20..=20 {
        for denominator in 1..=20 {
            let source = quantity(12, numerator, denominator);
            let Outcome::Completed(conversion) = convert_quantity(
                table.resolve(&source).unwrap(),
                target,
                &QuantityTarget::Exact,
                &mut meter(),
            )
            .unwrap() else {
                panic!("fraction sweep conversion completes");
            };
            let expected = ratio(numerator, denominator * 100);
            assert_eq!(conversion.value(), &ConvertedValue::Exact(expected));
        }
    }
}

#[trace("TC-201", "FR-007-AC-20")]
#[test]
fn tc_201_qsv_scale_and_offset_sweep_matches_independent_i128_oracle() {
    let rational = |n: i128, d: i128| Rational::new(Integer::from(n), Integer::from(d)).unwrap();
    let scales = [(-3_i128, 7_i128), (1, 2), (2, 3), (3, 1)];
    let offsets = [(-1_i128, 2_i128), (0, 1), (1, 3), (7, 10)];
    for (sn, sd) in scales {
        for (tn, td) in scales {
            for (on, od) in offsets {
                for (pn, pd) in offsets {
                    let dimensions = BTreeMap::from([(
                        key(1),
                        DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
                    )]);
                    let unit = |target: Option<u8>, scale: Rational, offset: Rational| {
                        UnitNode::checked(
                            [1; 32],
                            target.map(|byte| [byte; 32]),
                            scale,
                            offset,
                            NominalDeclaration::default(),
                        )
                        .unwrap()
                    };
                    let units = BTreeMap::from([
                        (key(11), unit(None, rational(1, 1), rational(0, 1))),
                        (key(12), unit(Some(11), rational(sn, sd), rational(on, od))),
                        (key(13), unit(Some(11), rational(tn, td), rational(pn, pd))),
                    ]);
                    let graph = UnitGraph::from_checked_nodes(&dimensions, &units).unwrap();
                    let table = UnitTable::declared(&graph);
                    for value in [-2_i128, -1, 0, 1, 2, i128::from(i64::MAX) + 1_000_003] {
                        let source = Quantity::new(rational(value, 1), UnitId::declared(key(12)));
                        let Outcome::Completed(converted) = convert_quantity(
                            table.resolve(&source).unwrap(),
                            table.get(UnitId::declared(key(13))).unwrap(),
                            &QuantityTarget::Exact,
                            &mut meter(),
                        )
                        .unwrap() else {
                            panic!("admitted scale/offset conversion completes");
                        };
                        // Inverse target path applied to source path, computed only with i128.
                        let expected_n = ((value * sn * od + on * sd) * pd - pn * sd * od) * td;
                        let expected_d = sd * od * pd * tn;
                        assert_eq!(
                            converted.value(),
                            &ConvertedValue::Exact(rational(expected_n, expected_d)),
                            "source {sn}/{sd} + {on}/{od}, target {tn}/{td} + {pn}/{pd}, value {value}"
                        );
                    }
                }
            }
        }
    }
}

#[trace("TC-201", "FR-007-AC-20", "FR-007-AC-23")]
#[test]
fn tc_201_qsv_noncommuting_multiedge_paths_preserve_composition_order() {
    let dimensions = BTreeMap::from([(
        key(1),
        DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
    )]);
    let unit = |target: Option<u8>, scale: Rational, offset: Rational| {
        UnitNode::checked(
            [1; 32],
            target.map(|byte| [byte; 32]),
            scale,
            offset,
            NominalDeclaration::default(),
        )
        .unwrap()
    };
    // Source: root = 2(-3v + 5) + 3 = -6v + 13.
    // Target: root = 4(3z + 7) - 2 = 12z + 26.
    let units = BTreeMap::from([
        (key(11), unit(None, ratio(1, 1), ratio(0, 1))),
        (key(12), unit(Some(11), ratio(2, 1), ratio(3, 1))),
        (key(13), unit(Some(12), ratio(-3, 1), ratio(5, 1))),
        (key(14), unit(Some(11), ratio(4, 1), ratio(-2, 1))),
        (key(15), unit(Some(14), ratio(3, 1), ratio(7, 1))),
    ]);
    let graph = UnitGraph::from_checked_nodes(&dimensions, &units).unwrap();
    let table = UnitTable::declared(&graph);
    for (source, target, expected) in [
        (
            Quantity::new(ratio(2, 1), UnitId::declared(key(13))),
            15,
            ratio(-25, 12),
        ),
        (
            Quantity::new(ratio(-25, 12), UnitId::declared(key(15))),
            13,
            ratio(2, 1),
        ),
    ] {
        let mut meter = meter();
        let Outcome::Completed(converted) = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(target))).unwrap(),
            &QuantityTarget::Exact,
            &mut meter,
        )
        .unwrap() else {
            panic!("noncommuting two-edge paths convert exactly");
        };
        assert_eq!(converted.value(), &ConvertedValue::Exact(expected));
        assert_eq!(meter.consumed(LimitKind::UnitEdges), 4);
    }
}

#[trace("TC-201", "FR-007-AC-21")]
#[test]
fn tc_201_qsv_incompatible_dimensions_refuse_before_charge() {
    let graph = graph();
    let mut table = UnitTable::declared(&graph);
    let square_id = table
        .insert(QuantityUnit::Compound(
            graph.compound_unit(&[([11; 32], int(2))]).unwrap(),
        ))
        .unwrap();
    let cases = [
        (quantity(11, 1, 1), 13),                    // length versus time
        (quantity(11, 1, 1), 27),                    // separate but shape-alike dimension node
        (quantity(23, 1, 1), 24),                    // distinct torque and energy nodes
        (Quantity::new(ratio(4, 1), square_id), 11), // area versus length
    ];
    for (source, target) in cases {
        let mut meter = meter();
        let refusal = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(target))).unwrap(),
            &QuantityTarget::Exact,
            &mut meter,
        )
        .unwrap_err();
        assert_eq!(refusal.cause, IllTypedCause::IncompatibleDimensions);
        assert_eq!(meter.admission_count(), 0);
    }
}

#[trace("TC-201", "FR-007-AC-23")]
#[test]
fn tc_201_qsv_conversion_charges_read_edge_arithmetic_target_then_retain() {
    let table = UnitTable::declared(&graph());
    let source = quantity(12, 100, 1);
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: 8,
        decimal_digits: 0,
        scale_expansion: 0,
        text_input_bytes: 0,
        text_scalars: 0,
        normalized_scalars: 0,
        unit_edges: 1,
        value_occurrences: 1,
        work_units: 6,
        result_units: 1,
    });
    let Outcome::Completed(conversion) = convert_quantity(
        table.resolve(&source).unwrap(),
        table.get(UnitId::declared(key(11))).unwrap(),
        &QuantityTarget::Exact,
        &mut meter,
    )
    .unwrap() else {
        panic!("centimetres to metres completes");
    };
    assert_eq!(conversion.value(), &ConvertedValue::Exact(ratio(1, 1)));
    assert_eq!(
        meter.admitted_charge_points(),
        [
            ChargePoint::UnitIdentityRead,
            ChargePoint::UnitEdge,
            ChargePoint::UnitRationalArithmetic,
            ChargePoint::UnitRationalArithmetic,
            ChargePoint::UnitTargetDomain,
            ChargePoint::UnitResultRetain,
        ]
    );
    assert_eq!(meter.consumed(LimitKind::UnitEdges), 1);
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 8);
    assert_eq!(meter.consumed(LimitKind::DecimalDigits), 0);
    assert_eq!(meter.consumed(LimitKind::ScaleExpansion), 0);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 1);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 6);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

#[trace("TC-201", "FR-007-AC-23")]
#[test]
fn tc_201_qsv_multiedge_and_compound_conversions_preserve_charge_schedule() {
    let graph = graph();
    let mut table = UnitTable::declared(&graph);
    let square_id = table
        .insert(QuantityUnit::Compound(
            graph.compound_unit(&[([11; 32], int(2))]).unwrap(),
        ))
        .unwrap();
    let cases = [
        (quantity(17, 1, 1), 12, ratio(127, 50), 2, 4, 15, 9),
        (
            Quantity::new(ratio(4, 1), square_id),
            26,
            ratio(40000, 1),
            1,
            2,
            17,
            6,
        ),
        (
            Quantity::new(ratio(4, 1), square_id),
            25,
            ratio(4, 1),
            0,
            0,
            3,
            3,
        ),
        (quantity(20, 1, 1), 19, ratio(-9, 1), 3, 6, 6, 12),
    ];
    for (source, target, expected, edges, arithmetic, bits, work) in cases {
        let mut meter = meter();
        let Outcome::Completed(converted) = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(target))).unwrap(),
            &QuantityTarget::Exact,
            &mut meter,
        )
        .unwrap() else {
            panic!("admitted conversion completes");
        };
        assert_eq!(converted.value(), &ConvertedValue::Exact(expected));
        let mut schedule = vec![ChargePoint::UnitIdentityRead];
        schedule.extend(std::iter::repeat_n(ChargePoint::UnitEdge, edges));
        schedule.extend(std::iter::repeat_n(
            ChargePoint::UnitRationalArithmetic,
            arithmetic,
        ));
        schedule.extend([ChargePoint::UnitTargetDomain, ChargePoint::UnitResultRetain]);
        assert_eq!(meter.admitted_charge_points(), schedule);
        assert_eq!(meter.consumed(LimitKind::IntegerBits), bits);
        assert_eq!(meter.consumed(LimitKind::UnitEdges), edges as u64);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), work);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    }
}

#[trace("TC-201", "FR-007-AC-23")]
#[test]
fn tc_201_qsv_inch_to_centimetre_exposes_the_first_two_rational_amounts() {
    let table = UnitTable::declared(&graph());
    let source = quantity(17, 1, 1);
    for (budget, consumed, next) in [(13, 1, 14), (14, 14, 15)] {
        let mut limits = *meter().limits();
        limits.integer_bits = budget;
        let Outcome::Incomplete(record) = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(12))).unwrap(),
            &QuantityTarget::Exact,
            &mut Meter::new(limits),
        )
        .unwrap() else {
            panic!("the selected arithmetic charge must deny");
        };
        assert_eq!(record.limit_kind, LimitKind::IntegerBits);
        assert_eq!(record.consumed, consumed);
        assert_eq!(record.next_charge, int(next));
        assert_eq!(record.charge_point, ChargePoint::UnitRationalArithmetic);
    }
}

#[trace("TC-201", "FR-007-AC-24")]
#[test]
fn tc_201_qsv_affine_arithmetic_refuses_without_charge() {
    let table = UnitTable::declared(&graph());
    let one = quantity(15, 1, 1);
    let two = quantity(15, 2, 1);
    let c1 = table.resolve(&one).unwrap();
    let c2 = table.resolve(&two).unwrap();
    let u1 = quantity(19, 1, 1);
    let u3 = quantity(21, 1, 1);
    let metre = quantity(11, 1, 1);
    for operation in [
        QuantityOperation::Add(c1, c2),
        QuantityOperation::Subtract(c1, c2),
        QuantityOperation::Multiply(c1, c2),
        QuantityOperation::Divide(c1, c2),
        QuantityOperation::Power(c1, &int(2)),
        QuantityOperation::Add(table.resolve(&u1).unwrap(), table.resolve(&u1).unwrap()),
        QuantityOperation::Multiply(table.resolve(&u3).unwrap(), table.resolve(&metre).unwrap()),
    ] {
        let mut meter = meter();
        let refusal = evaluate_quantity(operation, &mut meter).unwrap_err();
        assert_eq!(
            refusal,
            QuantityRefusal::IllTyped(quire_exact::IllTyped {
                cause: IllTypedCause::AffineUnitArithmetic,
            })
        );
        assert_eq!(meter.admission_count(), 0);
    }
    let u2_one = quantity(20, 1, 1);
    let u2_two = quantity(20, 2, 1);
    let Outcome::Completed(sum) = evaluate_quantity(
        QuantityOperation::Add(
            table.resolve(&u2_one).unwrap(),
            table.resolve(&u2_two).unwrap(),
        ),
        &mut meter(),
    )
    .unwrap() else {
        panic!("u2 is non-affine and addition completes");
    };
    assert_eq!(sum.magnitude(), &ratio(3, 1));
    assert_eq!(sum.unit(), UnitId::declared(key(20)));
}

#[trace("TC-034", "FR-007-AC-16")]
#[test]
fn tc_034_qsv_affine_power_refuses_zero_and_negative_exponents_without_charge() {
    let table = UnitTable::declared(&graph());
    let celsius = quantity(15, 1, 1);
    for exponent in [0, -1] {
        let mut meter = meter();
        let refusal = evaluate_quantity(
            QuantityOperation::Power(table.resolve(&celsius).unwrap(), &int(exponent)),
            &mut meter,
        )
        .unwrap_err();
        assert_eq!(
            refusal,
            QuantityRefusal::IllTyped(quire_exact::IllTyped {
                cause: IllTypedCause::AffineUnitArithmetic,
            })
        );
        assert_eq!(meter.admission_count(), 0);
    }
}

#[trace("TC-034", "FR-007-AC-17")]
#[test]
fn tc_034_qsv_distinct_length_units_multiply_and_divide_in_canonical_units() {
    let graph = graph();
    let table = UnitTable::declared(&graph);
    let kilometres = quantity(16, 2, 1);
    let metres = quantity(11, 3, 1);
    let left = table.resolve(&kilometres).unwrap();
    let right = table.resolve(&metres).unwrap();
    let Outcome::Completed(product) =
        evaluate_quantity(QuantityOperation::Multiply(left, right), &mut meter()).unwrap()
    else {
        panic!("product completes");
    };
    assert_eq!(product.magnitude(), &ratio(6000, 1));
    let square_id = graph
        .compound_unit(&[([11; 32], int(2))])
        .unwrap()
        .id()
        .unwrap();
    assert_eq!(product.unit(), square_id);

    let Outcome::Completed(quotient) =
        evaluate_quantity(QuantityOperation::Divide(left, right), &mut meter()).unwrap()
    else {
        panic!("quotient completes");
    };
    assert_eq!(quotient.magnitude(), &ratio(2000, 3));
    let dimensionless_id = graph.compound_unit(&[]).unwrap().id().unwrap();
    assert_eq!(quotient.unit(), dimensionless_id);
}

#[trace("TC-034", "FR-007-AC-12")]
#[test]
fn tc_034_qsv_quantity_type_checks_report_the_first_cause_without_charge() {
    let table = UnitTable::declared(&graph());
    let metre = quantity(11, 1, 1);
    let kilometre = quantity(16, 1, 1);
    let kelvin = quantity(14, 1, 1);
    let celsius = quantity(15, 1, 1);
    let m = table.resolve(&metre).unwrap();
    let km = table.resolve(&kilometre).unwrap();
    let k = table.resolve(&kelvin).unwrap();
    let c = table.resolve(&celsius).unwrap();
    for (operation, expected) in [
        (
            QuantityOperation::Add(m, c),
            IllTypedCause::IncompatibleDimensions,
        ),
        (
            QuantityOperation::Subtract(m, c),
            IllTypedCause::IncompatibleDimensions,
        ),
        (
            QuantityOperation::Add(c, k),
            IllTypedCause::AffineUnitArithmetic,
        ),
        (QuantityOperation::Add(m, km), IllTypedCause::DistinctUnits),
        (
            QuantityOperation::Subtract(m, km),
            IllTypedCause::DistinctUnits,
        ),
        (
            QuantityOperation::Multiply(c, k),
            IllTypedCause::AffineUnitArithmetic,
        ),
        (
            QuantityOperation::Divide(k, c),
            IllTypedCause::AffineUnitArithmetic,
        ),
        (
            QuantityOperation::Power(c, &int(2)),
            IllTypedCause::AffineUnitArithmetic,
        ),
    ] {
        let mut meter = meter();
        let refusal = evaluate_quantity(operation, &mut meter).unwrap_err();
        assert_eq!(
            refusal,
            QuantityRefusal::IllTyped(quire_exact::IllTyped { cause: expected })
        );
        assert_eq!(meter.admission_count(), 0);
    }
    for operation in [
        QuantityOperation::Multiply(km, k),
        QuantityOperation::Divide(km, k),
        QuantityOperation::Power(km, &int(2)),
    ] {
        assert!(matches!(
            evaluate_quantity(operation, &mut meter()),
            Ok(Outcome::Completed(_))
        ));
    }
}

#[trace("TC-032", "FR-011-AC-2")]
#[test]
fn tc_032_qsv_zero_divisor_and_negative_power_have_one_undefined_cause() {
    let graph = graph();
    let mut table = UnitTable::declared(&graph);
    let unit_id = table
        .insert(QuantityUnit::Compound(graph.compound_unit(&[]).unwrap()))
        .unwrap();
    let zero = Quantity::new(ratio(0, 1), unit_id);
    let four = Quantity::new(ratio(4, 1), unit_id);
    let stopped = Outcome::Undefined(quire_exact::Undefined::DivisionByZero);
    assert_eq!(
        evaluate_quantity(
            QuantityOperation::Power(table.resolve(&zero).unwrap(), &int(-1)),
            &mut meter(),
        )
        .unwrap(),
        stopped
    );
    assert_eq!(
        evaluate_quantity(
            QuantityOperation::Divide(
                table.resolve(&four).unwrap(),
                table.resolve(&zero).unwrap(),
            ),
            &mut meter(),
        )
        .unwrap(),
        stopped
    );
}

#[trace("TC-032", "FR-011-AC-4")]
#[test]
fn tc_032_qsv_identity_read_checks_integer_bits_before_value_occurrences() {
    let table = UnitTable::declared(&graph());
    let one = quantity(11, 5, 1);
    let two = quantity(11, 5, 1);
    let mut limits = *meter().limits();
    limits.integer_bits = 0;
    limits.value_occurrences = 0;
    let mut meter = Meter::new(limits);
    let Outcome::Incomplete(record) = evaluate_quantity(
        QuantityOperation::Add(table.resolve(&one).unwrap(), table.resolve(&two).unwrap()),
        &mut meter,
    )
    .unwrap() else {
        panic!("identity read is denied");
    };
    assert_eq!(record.limit_kind, LimitKind::IntegerBits);
    assert_eq!(record.limit, 0);
    assert_eq!(record.consumed, 0);
    assert_eq!(record.next_charge, int(3));
    assert_eq!(record.charge_point, ChargePoint::UnitIdentityRead);
    assert_eq!(meter.admission_count(), 0);
}

#[trace("TC-032", "FR-011-AC-6")]
#[test]
fn tc_032_qsv_huge_power_denies_before_computation_with_exact_amount() {
    let table = UnitTable::declared(&graph());
    let base = quantity(11, 2, 1);
    let exponent = Integer::from(18_446_744_073_709_551_616_i128);
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: 64,
        decimal_digits: 0,
        scale_expansion: 0,
        text_input_bytes: 0,
        text_scalars: 0,
        normalized_scalars: 0,
        unit_edges: 0,
        value_occurrences: 1,
        work_units: 5,
        result_units: 1,
    });
    let Outcome::Incomplete(record) = evaluate_quantity(
        QuantityOperation::Power(table.resolve(&base).unwrap(), &exponent),
        &mut meter,
    )
    .unwrap() else {
        panic!("the derived amount must be denied before powering");
    };
    assert_eq!(record.limit_kind, LimitKind::IntegerBits);
    assert_eq!(record.limit, 64);
    assert_eq!(record.consumed, 2);
    assert_eq!(record.charge_point, ChargePoint::UnitRationalArithmetic);
    assert_eq!(record.next_charge.to_string(), "36893488147419103232");
    assert_eq!(
        meter.admitted_charge_points(),
        [ChargePoint::UnitIdentityRead]
    );
}

#[trace("TC-201", "FR-007-AC-25")]
#[test]
fn tc_201_qsv_denied_retention_returns_incomplete_without_conversion() {
    let table = UnitTable::declared(&graph());
    let source = quantity(12, 100, 1);
    let mut meter = Meter::new(ScalarLimits {
        result_units: 0,
        ..*meter().limits()
    });
    let Outcome::Incomplete(record) = convert_quantity(
        table.resolve(&source).unwrap(),
        table.get(UnitId::declared(key(11))).unwrap(),
        &QuantityTarget::Exact,
        &mut meter,
    )
    .unwrap() else {
        panic!("retention must be denied");
    };
    assert_eq!(record.limit_kind, LimitKind::ResultUnits);
    assert_eq!(record.limit, 0);
    assert_eq!(record.consumed, 0);
    assert_eq!(record.next_charge, int(1));
    assert_eq!(record.charge_point, ChargePoint::UnitResultRetain);
}

#[trace("TC-201", "FR-007-AC-25")]
#[test]
fn tc_201_qsv_denied_edge_and_arithmetic_charges_report_exact_prefix() {
    let graph = graph();
    let mut table = UnitTable::declared(&graph);
    let square_id = table
        .insert(QuantityUnit::Compound(
            graph.compound_unit(&[([11; 32], int(2))]).unwrap(),
        ))
        .unwrap();
    let cases = [
        (
            quantity(17, 1, 1),
            12,
            LimitKind::UnitEdges,
            1,
            1,
            2,
            ChargePoint::UnitEdge,
        ),
        (
            Quantity::new(ratio(4, 1), square_id),
            26,
            LimitKind::IntegerBits,
            16,
            5,
            17,
            ChargePoint::UnitRationalArithmetic,
        ),
        (
            quantity(20, 1, 1),
            19,
            LimitKind::UnitEdges,
            2,
            2,
            3,
            ChargePoint::UnitEdge,
        ),
    ];
    for (source, target, kind, limit, consumed, next, point) in cases {
        let mut limits = *meter().limits();
        match kind {
            LimitKind::UnitEdges => limits.unit_edges = limit,
            LimitKind::IntegerBits => limits.integer_bits = limit,
            _ => unreachable!("the listed cases use only edge and arithmetic limits"),
        }
        let mut meter = Meter::new(limits);
        let Outcome::Incomplete(record) = convert_quantity(
            table.resolve(&source).unwrap(),
            table.get(UnitId::declared(key(target))).unwrap(),
            &QuantityTarget::Exact,
            &mut meter,
        )
        .unwrap() else {
            panic!("listed charge must be denied");
        };
        assert_eq!(record.limit_kind, kind);
        assert_eq!(record.limit, limit);
        assert_eq!(record.consumed, consumed);
        assert_eq!(record.next_charge, int(next as i64));
        assert_eq!(record.charge_point, point);
    }
}
