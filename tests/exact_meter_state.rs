//! FR-011 the determinate `Meter` state at an `Undefined`, `Refused` or
//! `Incomplete` stop, through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::Cell;
use std::num::NonZeroU64;

use quire_contract_runtime::exact::{
    divide, evaluate_boolean, evaluate_boolean_short_circuit, evaluate_integer_arithmetic,
    evaluate_quantity, order_numbers, BooleanConnective, ChargePoint, CompoundUnit,
    CompoundUnitCause, Decimal, DivisionProfile, IeeeFlag, IeeeFlags, Incomplete, InjectedDenial,
    Integer, IntegerArithmetic, IntegerDomain, IntegerInterval, InvalidCompoundUnit,
    InvalidSemanticGraph, LimitKind, Meter, NodeKey, OrderedOperands, OrderingOperator, Outcome,
    Quantity, QuantityOperation, QuantityUnit, Rational, Refusal, ScalarLimits, SemanticGraphCause,
    ShortCircuitConnective, Undefined, UnitDeclaration, UnitGraph, CHARGE_LOG_CAPACITY,
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

/// `evaluate_integer_arithmetic` takes an optional result bound rather than
/// `IntegerDomain`; `Mathematical` is no bound at all.
fn integer_bound(domain: &IntegerDomain) -> Option<&IntegerInterval> {
    match domain {
        IntegerDomain::Mathematical => None,
        IntegerDomain::Bounded(interval) => Some(interval),
        // `IntegerDomain` is `#[non_exhaustive]` (NFR-002-AC-3). A future
        // domain kind may carry no `&IntegerInterval` at all, so there is no
        // safe default here.
        _ => unreachable!("IntegerDomain gained a variant with no known bound representation"),
    }
}

/// Trace: TC-032, FR-011-AC-1
#[test]
fn tc_032_ac1_undefined_division_by_zero_retains_operands_only() {
    let mut meter = Meter::new(UNLIMITED);
    let outcome = divide(
        DivisionProfile::Truncating,
        &int(5),
        &int(0),
        &IntegerDomain::Mathematical,
        &mut meter,
    );
    assert_eq!(outcome, Outcome::Undefined(Undefined::DivisionByZero));
    assert_eq!(
        meter.admitted_charges(),
        [ChargePoint::IntegerDivisionOperands]
    );
    // `max(bits(5), bits(0)) = max(3, 1) = 3`; the arithmetic charge never
    // lands, so the counter it would have raised stays untouched.
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 3);
    assert_eq!(meter.consumed(LimitKind::ValueOccurrences), 2);
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

/// Trace: TC-032, FR-011-AC-1
#[test]
fn tc_032_ac1_refused_result_retains_arithmetic_not_result_unit() {
    let domain = IntegerDomain::Bounded(IntegerInterval::new(int(0), int(10)).unwrap());
    let mut meter = Meter::new(UNLIMITED);
    let outcome = evaluate_integer_arithmetic(
        IntegerArithmetic::Add(&int(7), &int(5)),
        integer_bound(&domain),
        &mut meter,
    );
    assert_eq!(
        outcome,
        Outcome::Refused(Box::new(Refusal::IntegerOutOfDomain))
    );
    assert_eq!(
        meter.admitted_charges(),
        [
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
        ]
    );
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
}

/// Trace: TC-032, FR-011-AC-1
#[test]
fn tc_032_ac1_incomplete_denied_charge_retains_nothing() {
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::BooleanResultRetain,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    let outcome = evaluate_boolean(BooleanConnective::Not(true), &mut meter);
    assert!(matches!(outcome, Outcome::Incomplete(_)));
    assert!(meter.admitted_charges().is_empty());
    assert!(!meter.charge_log_truncated());
    for kind in LimitKind::ALL {
        assert_eq!(meter.consumed(kind), 0);
    }
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

/// `evaluate_boolean` guarantees, for any decided operand pair on any connective kind, that the
/// terminal charge is admitted exactly once. Its operands are always plain, already-decided
/// `bool`s; `evaluate_boolean_short_circuit` (below) is the exact/metered subsystem's
/// stop-carrying connective, for a right operand that may itself stop.
///
/// Trace: TC-032, FR-011-AC-3
#[test]
fn tc_032_ac3_evaluate_boolean_retains_exactly_once() {
    let connectives = [
        (BooleanConnective::And(true, true), true),
        (BooleanConnective::And(false, true), false),
        (BooleanConnective::Or(false, false), false),
        (BooleanConnective::Or(true, false), true),
        (BooleanConnective::Implies(true, false), false),
        (BooleanConnective::Implies(false, true), true),
        (BooleanConnective::Not(true), false),
        (BooleanConnective::Not(false), true),
    ];
    for (connective, expected) in connectives {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = evaluate_boolean(connective, &mut meter);
        assert_eq!(outcome, Outcome::Completed(expected));
        assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    }
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
        Outcome::Refused(Box::new(Refusal::InexactDecimal)),
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::BooleanResultRetain,
        })),
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

/// Trace: TC-032, FR-011-AC-4
#[test]
fn tc_032_ac4_second_scanned_counter_short_writes_nothing() {
    let mut tuple = [u64::MAX; 10];
    tuple[1] = 5; // decimal_digits
    let mut meter = Meter::new(limits(tuple));
    let (a, b) = (Decimal::new(int(0), 5), Decimal::new(int(0), 0));
    let outcome = order_numbers(
        OrderingOperator::GreaterOrEqual,
        OrderedOperands::Decimals(&a, &b),
        &mut meter,
    );
    // `ordering.arithmetic`'s sorted sizes are `integer_bits = 18` (passes),
    // then `decimal_digits = 6` (fails at limit 5): the second-scanned
    // counter in field order.
    assert_eq!(
        outcome,
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::DecimalDigits,
            limit: 5,
            consumed: 1,
            next_charge: int(6),
            charge_point: ChargePoint::OrderingArithmetic,
        }))
    );
    assert_eq!(meter.admitted_charges(), [ChargePoint::OrderingOperands]);
    // The passing integer_bits amount (18) is never committed: the whole
    // charge stays at whatever the earlier operands charge left behind.
    assert_eq!(meter.consumed(LimitKind::IntegerBits), 1);
    assert_eq!(meter.consumed(LimitKind::DecimalDigits), 1);
    assert_eq!(meter.consumed(LimitKind::ScaleExpansion), 0);
}

/// `integer-arithmetic.operands` attaches `integer_bits` then
/// `value_occurrences` (already ascending field order); `unit.identity-read`
/// attaches `value_occurrences` then `integer_bits` (the reverse relative
/// order). Both report the same field-order-first short counter.
///
/// Trace: TC-032, FR-011-AC-4
#[test]
fn tc_032_ac4_field_order_scan_independent_of_attachment_order() {
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 0; // integer_bits
    tuple[7] = 0; // value_occurrences

    let ascending = evaluate_integer_arithmetic(
        IntegerArithmetic::Negate(&int(5)),
        None,
        &mut Meter::new(limits(tuple)),
    );
    assert_eq!(
        ascending,
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 0,
            consumed: 0,
            next_charge: int(3),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        }))
    );

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
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 0,
            consumed: 0,
            next_charge: int(3),
            charge_point: ChargePoint::UnitIdentityRead,
        }))
    );
}

/// Trace: TC-032, FR-011-AC-4
#[test]
fn tc_032_ac4_denied_charge_leaves_occurrence_counter_unchanged() {
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 0; // integer_bits: every attempt below is size-denied here
    let mut meter = Meter::new(limits(tuple));
    for _ in 0..3 {
        let outcome =
            evaluate_integer_arithmetic(IntegerArithmetic::Add(&int(1), &int(1)), None, &mut meter);
        assert!(matches!(
            outcome,
            Outcome::Incomplete(ref record) if matches!(
                **record,
                Incomplete {
                    limit_kind: LimitKind::IntegerBits,
                    ..
                }
            )
        ));
    }
    assert!(meter.admitted_charges().is_empty());
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 0);

    // Arm an injected denial for the *first* occurrence of the same point.
    // If any of the three size-denied attempts above had advanced the
    // point's occurrence counter, this would never fire, and the next call
    // would report the same size denial instead.
    let mut meter = meter.with_injected_denial(InjectedDenial {
        point: ChargePoint::IntegerArithmeticOperands,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    let outcome =
        evaluate_integer_arithmetic(IntegerArithmetic::Add(&int(1), &int(1)), None, &mut meter);
    assert_eq!(
        outcome,
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::IntegerArithmeticOperands,
        }))
    );
}

const NEGATE_POINTS: [ChargePoint; 3] = [
    ChargePoint::IntegerArithmeticOperands,
    ChargePoint::IntegerArithmeticArithmetic,
    ChargePoint::IntegerArithmeticResultRetain,
];

/// Trace: TC-032, FR-011-AC-5
#[test]
fn tc_032_ac5_log_holds_first_4096_in_admission_order() {
    let iterations = 1400_u64;
    let mut meter = Meter::new(UNLIMITED);
    for _ in 0..iterations {
        assert!(
            evaluate_integer_arithmetic(IntegerArithmetic::Negate(&int(3)), None, &mut meter)
                .completed()
                .is_some()
        );
    }
    let total_charges = iterations * 3;
    assert!(total_charges > u64::try_from(CHARGE_LOG_CAPACITY).unwrap());
    assert_eq!(meter.admitted_charges().len(), CHARGE_LOG_CAPACITY);
    assert!(meter.charge_log_truncated());
    for (index, point) in meter.admitted_charges().iter().enumerate() {
        assert_eq!(*point, NEGATE_POINTS[index % 3]);
    }
    assert_eq!(meter.consumed(LimitKind::WorkUnits), total_charges);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), iterations);
}

/// Trace: TC-032, FR-011-AC-5
#[test]
fn tc_032_ac5_limits_still_enforced_past_the_cap() {
    let iterations = 1400_u64;
    let total_charges = iterations * 3;
    assert!(total_charges > u64::try_from(CHARGE_LOG_CAPACITY).unwrap());
    let mut tuple = [u64::MAX; 10];
    tuple[8] = total_charges - 1; // work_units: one short of every charge admitted
    let mut meter = Meter::new(limits(tuple));
    for _ in 0..iterations - 1 {
        assert!(
            evaluate_integer_arithmetic(IntegerArithmetic::Negate(&int(3)), None, &mut meter)
                .completed()
                .is_some()
        );
    }
    let last = evaluate_integer_arithmetic(IntegerArithmetic::Negate(&int(3)), None, &mut meter);
    assert_eq!(
        last,
        Outcome::Incomplete(Box::new(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: total_charges - 1,
            consumed: total_charges - 1,
            next_charge: int(1),
            charge_point: ChargePoint::IntegerArithmeticResultRetain,
        }))
    );
    assert_eq!(meter.consumed(LimitKind::WorkUnits), total_charges - 1);
    assert_eq!(meter.admitted_charges().len(), CHARGE_LOG_CAPACITY);
    assert!(meter.charge_log_truncated());
}

/// `quire.value.accounting/v1` derived amounts saturate toward denial rather
/// than wrap. `Quantity` `power`'s bits amount is an unbounded `Integer` (a
/// unit integer power can require more than `u64::MAX`, per
/// [`Incomplete::next_charge`]'s own doc), so a modest exponent whose *own*
/// magnitude is unremarkable can still produce a derived amount that no
/// longer fits `u64` once multiplied by the base's part width, and
/// `Meter::charge` denies it outright rather than admitting it.
///
/// FR-011-AC-6 also asks for a *cumulative* counter (`work_units` or
/// `result_units`) driven to `u64::MAX - 1` and shown to deny rather than
/// wrap. That half is not reachable through the public surface: every named
/// charge point in this crate admits exactly one work unit (or a small fixed
/// result-unit amount) per call, `Charge`'s builders are `pub(crate)`, and
/// `Meter` exposes no way to preset its counters — reaching `u64::MAX - 1`
/// through real charges would take on the order of 10^19 operations. This
/// test backs only the derived-amount half of AC-6.
///
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
fn tc_032_ac7_consumed_is_total_for_every_limit_kind() {
    let fresh = Meter::new(UNLIMITED);
    for kind in LimitKind::ALL {
        assert_eq!(fresh.consumed(kind), 0);
    }
    let mut meter = Meter::new(UNLIMITED);
    assert!(evaluate_boolean(BooleanConnective::Not(true), &mut meter)
        .completed()
        .is_some());
    for kind in LimitKind::ALL {
        let _ = meter.consumed(kind);
    }
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
}

/// Trace: TC-032, FR-011-AC-7
#[test]
fn tc_032_ac7_ieee_flags_iterate_in_all_order() {
    let permutations = [
        [IeeeFlag::Inexact, IeeeFlag::Invalid, IeeeFlag::Overflow],
        [IeeeFlag::Overflow, IeeeFlag::Inexact, IeeeFlag::Invalid],
        [IeeeFlag::Invalid, IeeeFlag::Overflow, IeeeFlag::Inexact],
    ];
    for permutation in permutations {
        let flags: IeeeFlags = permutation.into_iter().collect();
        let order: Vec<IeeeFlag> = flags.iter().collect();
        assert_eq!(
            order,
            [IeeeFlag::Invalid, IeeeFlag::Overflow, IeeeFlag::Inexact]
        );
    }
    let reversed: IeeeFlags = IeeeFlag::ALL.into_iter().rev().collect();
    assert_eq!(reversed.iter().collect::<Vec<_>>(), IeeeFlag::ALL.to_vec());
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
            // `QuantityUnit` is `#[non_exhaustive]` (NFR-002-AC-3).
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
