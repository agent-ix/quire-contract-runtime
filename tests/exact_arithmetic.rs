//! Metered integer arithmetic, rational arithmetic, numeric ordering and Boolean
//! connectives under QSpec 5d88578 `quire.value.accounting/v1`.
//!
//! The pinned authority (quire-spec-language d9d5273) does not meter these
//! families yet (pending agent-ix/quire-spec-language#119), so every charge here
//! is checked against the QSpec definition rows and the TC-191 P11 and TC-190
//! Q11 atom schedules, and every value against an independent `i128` oracle.
#![cfg(feature = "exact")]

use std::cmp::Ordering;

use quire_contract_runtime::exact::{
    evaluate_connective, evaluate_integer, evaluate_not, evaluate_ordering, evaluate_rational,
    BooleanConnective, ChargePoint, Decimal, Incomplete, InjectedDenial, Integer, IntegerDomain,
    IntegerInterval, IntegerOperation, LimitKind, Meter, OrderingOperands, OrderingOperator,
    Outcome, Rational, RationalDomain, RationalOperation, Refusal, ScalarLimits, Undefined,
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

/// Every counter unlimited except `work_units`.
fn work(units: u64) -> ScalarLimits {
    let mut tuple = [u64::MAX; 10];
    tuple[8] = units;
    limits(tuple)
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

fn ratio(numerator: i128, denominator: i128) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

fn consumed(meter: &Meter) -> Vec<u64> {
    LimitKind::ALL
        .iter()
        .map(|kind| meter.consumed(*kind))
        .collect()
}

/// `bits(x)`: magnitude bit length, zero is one.
fn bits(value: i128) -> u64 {
    u64::from((128 - value.unsigned_abs().leading_zeros()).max(1))
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn work_denied(work: u64, point: ChargePoint) -> Incomplete {
    Incomplete {
        limit_kind: LimitKind::WorkUnits,
        limit: work,
        consumed: work,
        next_charge: int(1),
        charge_point: point,
    }
}

/// Deny each named charge of `run` in admission order.
fn assert_named_denials<T: std::fmt::Debug + PartialEq>(run: impl Fn(&mut Meter) -> Outcome<T>) {
    let mut meter = Meter::new(UNLIMITED);
    let _ = run(&mut meter);
    let charges = meter.admitted_charges().to_vec();
    assert!(!charges.is_empty());
    for (index, point) in charges.iter().enumerate() {
        let occurrence = charges[..=index]
            .iter()
            .filter(|seen| *seen == point)
            .count();
        let mut denied = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point: *point,
            occurrence: u64::try_from(occurrence).unwrap(),
        });
        let work = u64::try_from(index).unwrap();
        assert_eq!(
            run(&mut denied),
            Outcome::Incomplete(work_denied(work, *point))
        );
        assert_eq!(denied.consumed(LimitKind::ResultUnits), 0);
    }
}

const INTEGER: [ChargePoint; 3] = [
    ChargePoint::IntegerArithmeticOperands,
    ChargePoint::IntegerArithmeticArithmetic,
    ChargePoint::IntegerArithmeticResultRetain,
];
const RATIONAL: [ChargePoint; 4] = [
    ChargePoint::RationalArithmeticOperands,
    ChargePoint::RationalArithmeticArithmetic,
    ChargePoint::RationalArithmeticNormalize,
    ChargePoint::RationalArithmeticResultRetain,
];
const ORDERING: [ChargePoint; 3] = [
    ChargePoint::OrderingOperands,
    ChargePoint::OrderingArithmetic,
    ChargePoint::OrderingResultRetain,
];

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_p11_integer_atoms_order_and_subtract_with_exact_amounts() {
    // `n > 0` for n = 2: ordering.operands integer_bits 2, value_occurrences 2.
    let mut meter = Meter::new(UNLIMITED);
    let greater = evaluate_ordering(
        OrderingOperator::Greater,
        OrderingOperands::Integer(&int(2), &int(0)),
        &mut meter,
    );
    assert_eq!(greater, Outcome::Completed(true));
    assert_eq!(meter.admitted_charges(), ORDERING);
    assert_eq!(consumed(&meter), [2, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // `n - 1`: three integer-arithmetic charges.
    let mut meter = Meter::new(UNLIMITED);
    let difference = evaluate_integer(
        IntegerOperation::Subtract(&int(2), &int(1)),
        &IntegerDomain::Mathematical,
        &mut meter,
    );
    assert_eq!(difference, Outcome::Completed(int(1)));
    assert_eq!(meter.admitted_charges(), INTEGER);
    assert_eq!(consumed(&meter), [2, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // The scalar atoms of `down(2)`: `n > 0`, `n - 1` twice, then `0 > 0`.
    // P11's 18 work units include three `function.call` charges owned by
    // FR-146 (agent-ix/quire-spec-language#119); the scalar atoms are 15.
    let down = |meter: &mut Meter| -> Outcome<Integer> {
        let (zero, one) = (int(0), int(1));
        let mut n = int(2);
        loop {
            match evaluate_ordering(
                OrderingOperator::Greater,
                OrderingOperands::Integer(&n, &zero),
                meter,
            ) {
                Outcome::Completed(true) => {}
                Outcome::Completed(false) => return Outcome::Completed(n),
                stopped => return stopped.map_stop(),
            }
            match evaluate_integer(
                IntegerOperation::Subtract(&n, &one),
                &IntegerDomain::Mathematical,
                meter,
            ) {
                Outcome::Completed(next) => n = next,
                stopped => return stopped,
            }
        }
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(down(&mut meter), Outcome::Completed(int(0)));
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 15);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 5);
    let mut short = Meter::new(work(14));
    assert_eq!(
        down(&mut short),
        Outcome::Incomplete(work_denied(14, ChargePoint::OrderingResultRetain))
    );
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_p11_integer_division_to_rational_and_q11_fold_steps() {
    // `3/2`: operands integer_bits 2; arithmetic N = 3, D = 2 at integer_bits 2;
    // normalize integer_bits 2; retain. Four work units and one result unit.
    let run = |meter: &mut Meter| {
        evaluate_rational(
            RationalOperation::IntegerDivide(&int(3), &int(2)),
            None,
            meter,
        )
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(run(&mut meter), Outcome::Completed(ratio(3, 2)));
    assert_eq!(meter.admitted_charges(), RATIONAL);
    assert_eq!(consumed(&meter), [2, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_eq!(
        run(&mut Meter::new(work(3))),
        Outcome::Incomplete(work_denied(3, ChargePoint::RationalArithmeticResultRetain))
    );
    assert_named_denials(run);

    // Q11 `acc + x` over `1, 2`: integer-arithmetic.* per step.
    let mut meter = Meter::new(UNLIMITED);
    let mut acc = int(0);
    for x in [1, 2] {
        acc = evaluate_integer(
            IntegerOperation::Add(&acc, &int(x)),
            &IntegerDomain::Mathematical,
            &mut meter,
        )
        .completed()
        .unwrap();
    }
    assert_eq!(acc, int(3));
    assert_eq!(meter.admitted_charges(), [INTEGER, INTEGER].concat());
    assert_eq!(consumed(&meter), [2, 0, 0, 0, 0, 0, 0, 2, 6, 2]);
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_p11_implies_short_circuits_and_charges_boolean_retention() {
    let gt0 = |value: i128| {
        move |meter: &mut Meter| {
            evaluate_ordering(
                OrderingOperator::Greater,
                OrderingOperands::Integer(&int(value), &int(0)),
                meter,
            )
        }
    };
    for (a, expected_work, expected_results, right_runs) in [(3, 7, 3, true), (-1, 4, 2, false)] {
        let mut meter = Meter::new(UNLIMITED);
        let left = gt0(a)(&mut meter).completed().unwrap();
        let mut ran = false;
        let outcome = evaluate_connective(
            BooleanConnective::Implies,
            left,
            |meter| {
                ran = true;
                gt0(2)(meter)
            },
            &mut meter,
        );
        assert_eq!(outcome, Outcome::Completed(true));
        assert_eq!(ran, right_runs);
        assert_eq!(meter.consumed(LimitKind::WorkUnits), expected_work);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), expected_results);
        assert_eq!(
            meter.admitted_charges().last(),
            Some(&ChargePoint::BooleanResultRetain)
        );
    }

    // Retention is denied after a completed right operand...
    let mut meter = Meter::new(work(3));
    let denied = evaluate_connective(BooleanConnective::And, true, gt0(2), &mut meter);
    assert_eq!(
        denied,
        Outcome::Incomplete(work_denied(3, ChargePoint::BooleanResultRetain))
    );
    // ...and a stopped right operand is returned unchanged, with no retention.
    let mut meter = Meter::new(work(2));
    let stopped = evaluate_connective(BooleanConnective::And, true, gt0(2), &mut meter);
    assert_eq!(
        stopped,
        Outcome::Incomplete(work_denied(2, ChargePoint::OrderingResultRetain))
    );
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::BooleanResultRetain));
    let mut meter = Meter::new(UNLIMITED);
    let undefined = evaluate_connective(
        BooleanConnective::Or,
        false,
        |_| Outcome::Undefined(Undefined::DivisionByZero),
        &mut meter,
    );
    assert_eq!(undefined, Outcome::Undefined(Undefined::DivisionByZero));
    assert!(meter.admitted_charges().is_empty());
}

/// The same stop, retyped: a stopped ordering carries no `bool`.
trait MapStop {
    fn map_stop<T>(self) -> Outcome<T>;
}

impl MapStop for Outcome<bool> {
    fn map_stop<T>(self) -> Outcome<T> {
        match self {
            Outcome::Undefined(reason) => Outcome::Undefined(reason),
            Outcome::Refused(reason) => Outcome::Refused(reason),
            Outcome::Incomplete(record) => Outcome::Incomplete(record),
            Outcome::Completed(value) => panic!("completed ordering {value} is not a stop"),
        }
    }
}

/// Trace: TC-023, FR-007-AC-7
#[test]
fn tc_023_connective_truth_tables() {
    for connective in BooleanConnective::ALL {
        for left in [false, true] {
            for right in [false, true] {
                let expected = match connective {
                    BooleanConnective::And => left && right,
                    BooleanConnective::Or => left || right,
                    BooleanConnective::Implies => !left || right,
                };
                let mut ran = false;
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_connective(
                    connective,
                    left,
                    |_| {
                        ran = true;
                        Outcome::Completed(right)
                    },
                    &mut meter,
                );
                assert_eq!(outcome, Outcome::Completed(expected));
                let decided_by_left = matches!(
                    (connective, left),
                    (BooleanConnective::And, false)
                        | (BooleanConnective::Or, true)
                        | (BooleanConnective::Implies, false)
                );
                assert_eq!(ran, !decided_by_left);
                assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
                assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
            }
        }
    }
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(evaluate_not(false, &mut meter), Outcome::Completed(true));
    assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
    assert_eq!(
        evaluate_not(true, &mut Meter::new(work(0))),
        Outcome::Incomplete(work_denied(0, ChargePoint::BooleanResultRetain))
    );
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_rational_amounts_zero_divisors_and_domains() {
    // `1/2 + 1/2`: N = 1×2 + 1×2 = 4 and D = 4 (3 bits), reduced 1 (1 bit).
    let mut meter = Meter::new(UNLIMITED);
    let (half, zero) = (ratio(1, 2), ratio(0, 1));
    let sum = evaluate_rational(RationalOperation::Add(&half, &half), None, &mut meter);
    assert_eq!(sum, Outcome::Completed(ratio(1, 1)));
    assert_eq!(consumed(&meter), [3, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 2;
    assert_eq!(
        evaluate_rational(
            RationalOperation::Add(&half, &half),
            None,
            &mut Meter::new(limits(tuple))
        ),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 2,
            consumed: 2,
            next_charge: int(3),
            charge_point: ChargePoint::RationalArithmeticArithmetic,
        })
    );

    // A zero divisor is undefined after `rational-arithmetic.operands` only.
    for operation in [
        RationalOperation::Divide(&half, &zero),
        RationalOperation::IntegerDivide(&int(1), &int(0)),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        assert_eq!(
            evaluate_rational(operation, None, &mut meter),
            Outcome::Undefined(Undefined::DivisionByZero)
        );
        assert_eq!(
            meter.admitted_charges(),
            [ChargePoint::RationalArithmeticOperands]
        );
    }

    // Membership is decided after normalize and before retention.
    let domain = RationalDomain::new(
        IntegerInterval::new(int(-1), int(1)).unwrap(),
        IntegerInterval::new(int(1), int(2)).unwrap(),
    )
    .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let (third, two_thirds) = (ratio(1, 3), ratio(2, 3));
    assert_eq!(
        evaluate_rational(
            RationalOperation::Add(&third, &third),
            Some(&domain),
            &mut meter
        ),
        Outcome::Refused(Refusal::RationalOutOfDomain)
    );
    assert_eq!(meter.admitted_charges(), &RATIONAL[..3]);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(
        evaluate_rational(
            RationalOperation::Subtract(&two_thirds, &ratio(1, 6)),
            Some(&domain),
            &mut Meter::new(UNLIMITED)
        ),
        Outcome::Completed(ratio(1, 2))
    );
    // Unary minus: N = -a, D = b.
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(
        evaluate_rational(RationalOperation::Negate(&ratio(-5, 8)), None, &mut meter),
        Outcome::Completed(ratio(5, 8))
    );
    assert_eq!(consumed(&meter), [4, 0, 0, 0, 0, 0, 0, 1, 4, 1]);
    assert_named_denials(|meter| {
        evaluate_rational(
            RationalOperation::Multiply(&third, &two_thirds),
            Some(&domain),
            meter,
        )
    });
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_ordering_amounts_for_rationals_and_retained_decimals() {
    // `7/3 < 5/8`: operands maxparts 4; arithmetic max(bits(7×8), bits(5×3)) = 6.
    let mut meter = Meter::new(UNLIMITED);
    let (left, right) = (ratio(7, 3), ratio(5, 8));
    let less = evaluate_ordering(
        OrderingOperator::Less,
        OrderingOperands::Rational(&left, &right),
        &mut meter,
    );
    assert_eq!(less, Outcome::Completed(false));
    assert_eq!(meter.admitted_charges(), ORDERING);
    assert_eq!(consumed(&meter), [6, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // Retained, never normalized: `(0, 5) >= (0, 0)` aligns zero to one digit.
    let mut meter = Meter::new(UNLIMITED);
    let (a, b) = (Decimal::new(int(0), 5), Decimal::new(int(0), 0));
    let ge = evaluate_ordering(
        OrderingOperator::GreaterOrEqual,
        OrderingOperands::Decimal(&a, &b),
        &mut meter,
    );
    assert_eq!(ge, Outcome::Completed(true));
    assert_eq!(consumed(&meter), [1, 1, 5, 0, 0, 0, 0, 2, 3, 1]);

    // `(100, 2) <= (2, 0)`: aligned 200 (8 bits, 3 digits), scale expansion 2.
    let mut meter = Meter::new(UNLIMITED);
    let (a, b) = (Decimal::new(int(100), 2), Decimal::new(int(2), 0));
    let le = evaluate_ordering(
        OrderingOperator::LessOrEqual,
        OrderingOperands::Decimal(&a, &b),
        &mut meter,
    );
    assert_eq!(le, Outcome::Completed(true));
    assert_eq!(consumed(&meter), [8, 3, 2, 0, 0, 0, 0, 2, 3, 1]);

    // An aligned coefficient beyond u64 is sized analytically, never materialized.
    let (a, b) = (Decimal::new(int(1), 0), Decimal::new(int(1), u32::MAX));
    let mut tuple = [u64::MAX; 10];
    tuple[1] = 64;
    assert_eq!(
        evaluate_ordering(
            OrderingOperator::Less,
            OrderingOperands::Decimal(&a, &b),
            &mut Meter::new(limits(tuple))
        ),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::DecimalDigits,
            limit: 64,
            consumed: 1,
            next_charge: int(4_294_967_296),
            charge_point: ChargePoint::OrderingArithmetic,
        })
    );
    let (a, b) = (Decimal::new(int(-125), 2), Decimal::new(int(3), 0));
    assert_named_denials(|meter| {
        evaluate_ordering(
            OrderingOperator::Greater,
            OrderingOperands::Decimal(&a, &b),
            meter,
        )
    });
}

const VALUES: [i128; 12] = [
    0,
    1,
    -1,
    2,
    -3,
    7,
    255,
    -256,
    1 << 40,
    -(1 << 62),
    i64::MAX as i128,
    i64::MIN as i128,
];

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3, FR-006-AC-4
#[test]
fn tc_023_generated_integer_and_ordering_against_an_i128_oracle() {
    let bounded =
        IntegerDomain::Bounded(IntegerInterval::new(int(-(1 << 63)), int((1 << 63) - 1)).unwrap());
    let mut vectors = 0_u32;
    for a in VALUES {
        for b in VALUES {
            let (x, y) = (int(a), int(b));
            let cases: [(IntegerOperation<'_>, i128, u64); 4] = [
                (IntegerOperation::Add(&x, &y), a + b, 2),
                (IntegerOperation::Subtract(&x, &y), a - b, 2),
                (IntegerOperation::Multiply(&x, &y), a * b, 2),
                (IntegerOperation::Negate(&x), -a, 1),
            ];
            for (operation, expected, count) in cases {
                let operand_bits = if count == 2 {
                    bits(a).max(bits(b))
                } else {
                    bits(a)
                };
                let mut meter = Meter::new(UNLIMITED);
                assert_eq!(
                    evaluate_integer(operation, &IntegerDomain::Mathematical, &mut meter),
                    Outcome::Completed(int(expected))
                );
                assert_eq!(
                    consumed(&meter),
                    [
                        operand_bits.max(bits(expected)),
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        count,
                        3,
                        1
                    ]
                );
                let in_range = i64::try_from(expected).is_ok();
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_integer(operation, &bounded, &mut meter);
                if in_range {
                    assert_eq!(outcome, Outcome::Completed(int(expected)));
                } else {
                    assert_eq!(outcome, Outcome::Refused(Refusal::IntegerOutOfDomain));
                    assert_eq!(meter.admitted_charges(), &INTEGER[..2]);
                }
                assert_named_denials(|meter| {
                    evaluate_integer(operation, &IntegerDomain::Mathematical, meter)
                });
                vectors += 1;
            }
            for operator in OrderingOperator::ALL {
                let ordering = a.cmp(&b);
                let expected = match operator {
                    OrderingOperator::Less => ordering == Ordering::Less,
                    OrderingOperator::LessOrEqual => ordering != Ordering::Greater,
                    OrderingOperator::Greater => ordering == Ordering::Greater,
                    OrderingOperator::GreaterOrEqual => ordering != Ordering::Less,
                };
                let mut meter = Meter::new(UNLIMITED);
                assert_eq!(
                    evaluate_ordering(operator, OrderingOperands::Integer(&x, &y), &mut meter),
                    Outcome::Completed(expected)
                );
                assert_eq!(
                    consumed(&meter),
                    [bits(a).max(bits(b)), 0, 0, 0, 0, 0, 0, 2, 3, 1]
                );
                vectors += 1;
            }
        }
    }
    assert_eq!(vectors, 12 * 12 * 8);
}

const FRACTIONS: [(i128, i128); 10] = [
    (0, 1),
    (1, 1),
    (-1, 2),
    (3, 4),
    (-7, 3),
    (5, 8),
    (1, 1 << 20),
    (-(1 << 30), 7),
    (22, 7),
    (-355, 113),
];

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3, FR-006-AC-4
#[test]
fn tc_023_generated_rational_arithmetic_and_ordering_against_an_i128_oracle() {
    let maxparts = |(n, d): (i128, i128)| bits(n).max(bits(d));
    let reduce = |(n, d): (i128, i128)| {
        let g = gcd(n, d).max(1);
        let (n, d) = (n / g, d / g);
        if d < 0 {
            (-n, -d)
        } else {
            (n, d)
        }
    };
    let mut vectors = 0_u32;
    for (a, b) in FRACTIONS {
        for (c, d) in FRACTIONS {
            let (left, right) = (ratio(a, b), ratio(c, d));
            let unreduced = [
                (
                    RationalOperation::Add(&left, &right),
                    Some((a * d + c * b, b * d)),
                ),
                (
                    RationalOperation::Subtract(&left, &right),
                    Some((a * d - c * b, b * d)),
                ),
                (
                    RationalOperation::Multiply(&left, &right),
                    Some((a * c, b * d)),
                ),
                (
                    RationalOperation::Divide(&left, &right),
                    (c != 0).then_some((a * d, b * c)),
                ),
                (RationalOperation::Negate(&left), Some((-a, b))),
            ];
            for (index, (operation, intermediate)) in unreduced.into_iter().enumerate() {
                let operand_bits = if index == 4 {
                    maxparts((a, b))
                } else {
                    maxparts((a, b)).max(maxparts((c, d)))
                };
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_rational(operation, None, &mut meter);
                match intermediate {
                    None => {
                        assert_eq!(outcome, Outcome::Undefined(Undefined::DivisionByZero));
                        assert_eq!(meter.admitted_charges(), &RATIONAL[..1]);
                    }
                    Some((n, m)) => {
                        let (rn, rd) = reduce((n, m));
                        assert_eq!(outcome, Outcome::Completed(ratio(rn, rd)));
                        let high_water = operand_bits.max(maxparts((n, m))).max(maxparts((rn, rd)));
                        assert_eq!(consumed(&meter)[0], high_water);
                        assert_named_denials(|meter| evaluate_rational(operation, None, meter));
                    }
                }
                vectors += 1;
            }
            let ordering = (a * d).cmp(&(c * b));
            for operator in OrderingOperator::ALL {
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_ordering(
                    operator,
                    OrderingOperands::Rational(&left, &right),
                    &mut meter,
                );
                assert_eq!(outcome, Outcome::Completed(operator.holds(ordering)));
                let cross = bits(a * d).max(bits(c * b));
                assert_eq!(
                    consumed(&meter),
                    [
                        maxparts((a, b)).max(maxparts((c, d))).max(cross),
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        2,
                        3,
                        1
                    ]
                );
                vectors += 1;
            }
        }
    }
    assert_eq!(vectors, 10 * 10 * 9);
}
