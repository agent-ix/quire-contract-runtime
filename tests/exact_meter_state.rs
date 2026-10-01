//! FR-011 the determinate `Meter` state at an `Undefined`, `Refused` or
//! `Incomplete` stop, through the public `exact` surface, for the operations the runtime still
//! defines (the interim residue of FR-275): quantity arithmetic, the stop-carrying
//! short-circuit connective and the unit graph's ordering rules.
//!
//! The cases whose subject is the kernel `Meter` or a kernel scalar operation (the charge log
//! cap, the injected-denial occurrence counter, the scan order over integer, decimal and
//! division counters, IEEE flag order) left with the kernel to `quire-exact` (FR-275).
#![cfg(feature = "exact")]

use std::cell::Cell;

use quire_contract_runtime::exact::{
    evaluate_boolean_short_circuit, evaluate_quantity, ChargePoint, CompoundUnit,
    CompoundUnitCause, Incomplete, Integer, InvalidCompoundUnit, InvalidSemanticGraph, LimitKind,
    Meter, NodeKey, Outcome, Quantity, QuantityOperation, QuantityUnit, Rational, Refusal,
    ScalarLimits, SemanticGraphCause, ShortCircuitConnective, Undefined, UnitDeclaration,
    UnitGraph,
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

/// An opaque node key whose ordering is controlled by its last byte.
fn key(byte: u8) -> NodeKey {
    let mut bytes = [0_u8; 32];
    bytes[31] = byte;
    NodeKey::from_bytes(bytes)
}

/// Trace: TC-032, FR-011-AC-2
#[test]
fn tc_032_ac2_power_zero_base_negative_exponent_is_division_by_zero() {
    let base = Quantity::new(
        Rational::from_integer(Integer::zero()),
        QuantityUnit::Compound(CompoundUnit::dimensionless()),
    );
    let exponent = int(-1);
    let outcome = evaluate_quantity(
        QuantityOperation::Power(&base, &exponent),
        &mut Meter::new(UNLIMITED),
    )
    .unwrap();
    assert_eq!(outcome, Outcome::Undefined(Undefined::DivisionByZero));
}

/// `check_undefined` matches on the operation's own shape: `Divide`'s
/// zero-divisor arm and `Power`'s zero-base arm are mutually exclusive over
/// one call (an operation is one variant or the other, never both), so
/// "first match wins" is demonstrated here by both shapes reducing to the
/// identical `DivisionByZero` cause the vocabulary reserves for it (FR-011:
/// "the vocabulary carries no separate cause for it").
///
/// Trace: TC-032, FR-011-AC-2
#[test]
fn tc_032_ac2_divide_by_zero_and_power_zero_base_report_same_cause() {
    let unit = QuantityUnit::Compound(CompoundUnit::dimensionless());
    let zero = Quantity::new(Rational::from_integer(Integer::zero()), unit.clone());
    let dividend = Quantity::new(Rational::from_integer(int(4)), unit);

    let divided = evaluate_quantity(
        QuantityOperation::Divide(&dividend, &zero),
        &mut Meter::new(UNLIMITED),
    )
    .unwrap();
    assert_eq!(divided, Outcome::Undefined(Undefined::DivisionByZero));

    let powered = evaluate_quantity(
        QuantityOperation::Power(&zero, &int(-2)),
        &mut Meter::new(UNLIMITED),
    )
    .unwrap();
    assert_eq!(powered, Outcome::Undefined(Undefined::DivisionByZero));
}

/// `evaluate_boolean_short_circuit` is the exact/metered subsystem's stop-carrying connective
/// (`agent-ix/quire-contract-runtime#27`). Two halves:
///
/// - When `left` alone decides the result (`And` with `left = false`, `Or` with `left = true`,
///   `Implies` with `left = false`), `right` is never called — so a stop it could have produced
///   can never arise — and the decided result charges `boolean.result-retain` exactly once
///   (FR-011-AC-3's half).
/// - Otherwise `right()` runs. If it stops (`Undefined`, `Refused` or `Incomplete`), that stop
///   returns unchanged, with no `boolean.result-retain` charge and no result unit consumed
///   (FR-011-AC-8). If it completes, the combined result charges `boolean.result-retain` exactly
///   once.
///
/// Trace: TC-032, FR-011-AC-3, FR-011-AC-8
#[test]
fn tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once() {
    let stops = [
        Outcome::Undefined(Undefined::DivisionByZero),
        Outcome::Refused(Refusal::CheckedInvariant),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::BooleanResultRetain,
        }),
    ];

    // The right operand decides nothing: it is skipped entirely, and no stop it could have
    // produced can arise.
    let short_circuiting = [
        (ShortCircuitConnective::And, false, false),
        (ShortCircuitConnective::Or, true, true),
        (ShortCircuitConnective::Implies, false, true),
    ];
    for (connective, left, expected) in short_circuiting {
        for stop in &stops {
            let stop = stop.clone();
            let mut meter = Meter::new(UNLIMITED);
            let called = Cell::new(false);
            let outcome = evaluate_boolean_short_circuit(
                connective,
                left,
                || {
                    called.set(true);
                    stop
                },
                &mut meter,
            );
            assert!(!called.get());
            assert_eq!(outcome, Outcome::Completed(expected));
            assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
        }
    }

    // The right operand decides the result: a stop it produces returns unchanged, with no
    // boolean.result-retain charge and no result unit consumed.
    let evaluated = [
        (ShortCircuitConnective::And, true),
        (ShortCircuitConnective::Or, false),
        (ShortCircuitConnective::Implies, true),
    ];
    for (connective, left) in evaluated {
        for stop in &stops {
            let stop = stop.clone();
            let mut meter = Meter::new(UNLIMITED);
            let outcome =
                evaluate_boolean_short_circuit(connective, left, || stop.clone(), &mut meter);
            assert_eq!(outcome, stop);
            assert_eq!(meter.admitted_charges(), []);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 0);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        }
    }

    // The right operand decides the result and completes: the combined result charges
    // boolean.result-retain exactly once.
    let decided = [
        (ShortCircuitConnective::And, true, true, true),
        (ShortCircuitConnective::And, true, false, false),
        (ShortCircuitConnective::Or, false, true, true),
        (ShortCircuitConnective::Or, false, false, false),
        (ShortCircuitConnective::Implies, true, true, true),
        (ShortCircuitConnective::Implies, true, false, false),
    ];
    for (connective, left, right, expected) in decided {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = evaluate_boolean_short_circuit(
            connective,
            left,
            || Outcome::Completed(right),
            &mut meter,
        );
        assert_eq!(outcome, Outcome::Completed(expected));
        assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    }
}

/// `unit.identity-read` attaches `value_occurrences` then `integer_bits` (the reverse of
/// field order), and still reports the field-order-first short counter. The scan-order cases of
/// the kernel's own charge points left with the kernel.
///
/// Trace: TC-032, FR-011-AC-4
#[test]
fn tc_032_ac4_field_order_scan_independent_of_attachment_order() {
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 0; // integer_bits
    tuple[7] = 0; // value_occurrences

    let unit = QuantityUnit::Compound(CompoundUnit::dimensionless());
    let a = Quantity::new(Rational::from_integer(int(5)), unit.clone());
    let b = Quantity::new(Rational::from_integer(int(5)), unit);
    let reversed = evaluate_quantity(
        QuantityOperation::Add(&a, &b),
        &mut Meter::new(limits(tuple)),
    )
    .unwrap();
    assert_eq!(
        reversed,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 0,
            consumed: 0,
            next_charge: int(3),
            charge_point: ChargePoint::UnitIdentityRead,
        })
    );
}

/// Trace: TC-032, FR-011-AC-6
#[test]
fn tc_032_ac6_derived_amount_past_u64_max_is_denied_not_admitted() {
    let base = Quantity::new(
        Rational::from_integer(int(3)),
        QuantityUnit::Compound(CompoundUnit::dimensionless()),
    );
    let exponent = Integer::from(u64::MAX);
    let mut meter = Meter::new(UNLIMITED);
    let outcome =
        evaluate_quantity(QuantityOperation::Power(&base, &exponent), &mut meter).unwrap();
    match outcome {
        Outcome::Incomplete(record) => {
            assert_eq!(record.limit_kind, LimitKind::IntegerBits);
            assert_eq!(record.charge_point, ChargePoint::UnitRationalArithmetic);
            assert!(
                record.next_charge.to_u64().is_none(),
                "the denied amount must exceed u64::MAX"
            );
        }
        other => panic!("expected Incomplete, got {other:?}"),
    }
    assert_eq!(meter.admitted_charges(), [ChargePoint::UnitIdentityRead]);
}

/// Trace: TC-032, FR-011-AC-7
#[test]
fn tc_032_ac7_dimension_terms_ascend_by_node_key_for_any_construction_order() {
    let (k1, k2, k3) = (key(1), key(2), key(3));
    let graph = UnitGraph::admit(
        [
            (k1, Vec::<(NodeKey, Integer)>::new()),
            (k2, Vec::new()),
            (k3, Vec::new()),
        ],
        Vec::new(),
    )
    .unwrap();
    let (d1, d2, d3) = (
        graph.dimension(k1).unwrap(),
        graph.dimension(k2).unwrap(),
        graph.dimension(k3).unwrap(),
    );
    let combinations = [
        d1.multiply(d2).multiply(d3),
        d3.multiply(d1).multiply(d2),
        d2.multiply(d3).multiply(d1),
    ];
    for combined in combinations {
        let terms: Vec<NodeKey> = combined.exponents().map(|(key, _)| key).collect();
        assert_eq!(terms, [k1, k2, k3]);
    }
}

/// Every quantity operation that combines units (`*`, `/`) runs through the
/// same `pub(crate)` `CompoundUnit::multiply`/`divide`; this exercises it
/// through the public `evaluate_quantity` surface instead, over declared root
/// units admitted in distinct dimensions, associated in several orders.
///
/// Trace: TC-032, FR-011-AC-7
#[test]
fn tc_032_ac7_compound_unit_terms_ascend_by_node_key_for_any_construction_order() {
    let (d1, d2, d3) = (key(11), key(12), key(13));
    let (u1, u2, u3) = (key(21), key(22), key(23));
    let root = |dimension: NodeKey| UnitDeclaration {
        dimension,
        target: None,
        scale: Rational::from_integer(int(1)),
        offset: Rational::from_integer(int(0)),
    };
    let graph = UnitGraph::admit(
        [
            (d1, Vec::<(NodeKey, Integer)>::new()),
            (d2, Vec::new()),
            (d3, Vec::new()),
        ],
        [(u1, root(d1)), (u2, root(d2)), (u3, root(d3))],
    )
    .unwrap();
    let declared =
        |key: NodeKey| QuantityUnit::Declared(Box::new(graph.unit(key).unwrap().clone()));
    let one = |unit: QuantityUnit| Quantity::new(Rational::from_integer(int(1)), unit);
    let multiply = |a: &Quantity, b: &Quantity| -> Quantity {
        evaluate_quantity(
            QuantityOperation::Multiply(a, b),
            &mut Meter::new(UNLIMITED),
        )
        .unwrap()
        .completed()
        .unwrap()
    };
    let (a, b, c) = (one(declared(u1)), one(declared(u2)), one(declared(u3)));
    let orders = [
        multiply(&multiply(&a, &b), &c),
        multiply(&multiply(&c, &a), &b),
        multiply(&multiply(&b, &c), &a),
    ];
    for combined in orders {
        let compound = match combined.unit() {
            QuantityUnit::Compound(compound) => compound,
            QuantityUnit::Declared(_) => panic!("a compound-unit product must stay compound"),
            // `QuantityUnit` is `#[non_exhaustive]` (NFR-002).
            _ => panic!("QuantityUnit gained a variant this test does not expect"),
        };
        let terms: Vec<NodeKey> = compound.terms().map(|(key, _)| key).collect();
        assert_eq!(terms, [u1, u2, u3]);
    }
}

/// Trace: TC-032, FR-011-AC-7
#[test]
fn tc_032_ac7_unit_graph_admit_orders_well_formedness_duplicate_keys_then_topology() {
    // Well-formedness precedes the duplicate-key check for one item: `k` is
    // admitted cleanly first, and the *second* dimension entry that reuses
    // `k` is malformed (a zero exponent), so its own well-formedness check
    // fails before a duplicate-key insert is ever attempted for it.
    let k = key(1);
    let other = key(2);
    let malformed = UnitGraph::admit(
        [
            (k, Vec::<(NodeKey, Integer)>::new()),
            (k, vec![(other, Integer::zero())]),
        ],
        Vec::new(),
    );
    assert_eq!(
        malformed,
        Err(InvalidSemanticGraph {
            cause: SemanticGraphCause::ZeroExponent,
        })
    );

    // Duplicate keys precede graph topology: two dimension entries reuse `k`
    // (both well-formed on their own), and a unit elsewhere names a
    // dimension that was never admitted (an unrelated topology failure).
    // Every dimension is processed, well-formedness then duplicate, before
    // any unit's topology is checked, so the duplicate is reported.
    let unit_key = key(3);
    let duplicate_before_topology = UnitGraph::admit(
        [(k, Vec::<(NodeKey, Integer)>::new()), (k, Vec::new())],
        [(
            unit_key,
            UnitDeclaration {
                dimension: key(99), // never admitted
                target: None,
                scale: Rational::from_integer(int(1)),
                offset: Rational::from_integer(int(0)),
            },
        )],
    );
    assert_eq!(
        duplicate_before_topology,
        Err(InvalidSemanticGraph {
            cause: SemanticGraphCause::DuplicateNode,
        })
    );
}

/// Trace: TC-032, FR-011-AC-7
#[test]
fn tc_032_ac7_check_terms_refuses_zero_then_duplicate_then_unsorted() {
    let graph = UnitGraph::default();
    let (k1, k2) = (key(1), key(2));

    // A zero exponent and out-of-order terms both hold; zero is reported.
    let zero_and_unsorted = graph.compound_unit(&[(k2, Integer::zero()), (k1, int(1))]);
    assert_eq!(
        zero_and_unsorted,
        Err(InvalidCompoundUnit {
            cause: CompoundUnitCause::ZeroExponent,
        })
    );

    // A duplicate term and out-of-order terms both hold (equal keys are
    // never strictly ascending); the duplicate is reported.
    let duplicate_and_unsorted = graph.compound_unit(&[(k1, int(1)), (k1, int(1))]);
    assert_eq!(
        duplicate_and_unsorted,
        Err(InvalidCompoundUnit {
            cause: CompoundUnitCause::DuplicateTerm,
        })
    );

    // Purely out-of-order, distinct, nonzero terms: unsorted is reported.
    let unsorted = graph.compound_unit(&[(k2, int(1)), (k1, int(1))]);
    assert_eq!(
        unsorted,
        Err(InvalidCompoundUnit {
            cause: CompoundUnitCause::UnsortedTerms,
        })
    );
}
