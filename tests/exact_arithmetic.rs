//! Metered integer arithmetic, rational arithmetic, numeric ordering and Boolean
//! connectives under QSpec 7d7943a `quire.value.accounting/v1`, whose every
//! arithmetic amount is derived from operand bit lengths.
//!
//! The pinned authority (quire-spec-language d9d5273) does not meter these
//! families yet (pending agent-ix/quire-spec-language#119), so every charge here
//! is checked against the QSpec definition rows and the TC-191 P11 and TC-190
//! Q11 atom schedules, and every value against an independent `i128` oracle.
#![cfg(feature = "exact")]

use std::cmp::Ordering;
use std::num::NonZeroU64;

use quire_contract_runtime::exact::{
    evaluate_boolean, evaluate_integer_arithmetic, evaluate_rational_arithmetic, order_numbers,
    BooleanConnective, ChargePoint, Decimal, Incomplete, InjectedDenial, Integer,
    IntegerArithmetic, IntegerInterval, LimitKind, Meter, OrderedOperands, OrderingOperator,
    Outcome, Rational, RationalArithmetic, RationalDomain, Refusal, ScalarLimits, Undefined,
    CHARGE_LOG_CAPACITY,
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

/// An expected `Integer` built independently of `Integer::from(i128)`: through
/// `Integer`'s decimal-string `FromStr`, which promotes via `from_big` rather
/// than the fast `i128`-to-`Integer` conversion under test.
fn oracle(value: i128) -> Integer {
    value.to_string().parse().unwrap()
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
            occurrence: NonZeroU64::new(u64::try_from(occurrence).unwrap()).unwrap(),
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
    let greater = order_numbers(
        OrderingOperator::Greater,
        OrderedOperands::Integers(&int(2), &int(0)),
        &mut meter,
    );
    assert_eq!(greater, Outcome::Completed(true));
    assert_eq!(meter.admitted_charges(), ORDERING);
    assert_eq!(consumed(&meter), [2, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // `n - 1`: three integer-arithmetic charges; `max(bits(2), bits(1)) + 1`.
    let mut meter = Meter::new(UNLIMITED);
    let difference = evaluate_integer_arithmetic(
        IntegerArithmetic::Subtract(&int(2), &int(1)),
        None,
        &mut meter,
    );
    assert_eq!(difference, Outcome::Completed(int(1)));
    assert_eq!(meter.admitted_charges(), INTEGER);
    assert_eq!(consumed(&meter), [3, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // The scalar atoms of `down(2)`: `n > 0`, `n - 1` twice, then `0 > 0`.
    // P11's 18 work units include three `function.call` charges owned by
    // quire-specification/FR-146 (agent-ix/quire-spec-language#119); the scalar atoms are 15.
    let down = |meter: &mut Meter| -> Outcome<Integer> {
        let (zero, one) = (int(0), int(1));
        let mut n = int(2);
        loop {
            match order_numbers(
                OrderingOperator::Greater,
                OrderedOperands::Integers(&n, &zero),
                meter,
            ) {
                Outcome::Completed(true) => {}
                Outcome::Completed(false) => return Outcome::Completed(n),
                stopped => return stopped.map_stop(),
            }
            match evaluate_integer_arithmetic(IntegerArithmetic::Subtract(&n, &one), None, meter) {
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
    // `3/2`: operands integer_bits 2; arithmetic `N = bits(3) + bits(1) = 3`,
    // `D = bits(1) + bits(2) = 3`; normalize integer_bits 2; retain. Four work
    // units and one result unit. Integer `/` producing a rational enters `3`
    // and `2` as `3/1` and `2/1` (`Rational::from_integer`), per
    // `RationalArithmetic::Divide`'s doc.
    let run = |meter: &mut Meter| {
        evaluate_rational_arithmetic(
            RationalArithmetic::Divide(
                &Rational::from_integer(int(3)),
                &Rational::from_integer(int(2)),
            ),
            None,
            meter,
        )
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(run(&mut meter), Outcome::Completed(ratio(3, 2)));
    assert_eq!(meter.admitted_charges(), RATIONAL);
    assert_eq!(consumed(&meter), [3, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_eq!(
        run(&mut Meter::new(work(3))),
        Outcome::Incomplete(work_denied(3, ChargePoint::RationalArithmeticResultRetain))
    );
    assert_named_denials(run);

    // Q11 `acc + x` over `1, 2`: integer-arithmetic.* per step; the second
    // add charges `max(bits(1), bits(2)) + 1 = 3`.
    let mut meter = Meter::new(UNLIMITED);
    let mut acc = int(0);
    for x in [1, 2] {
        acc = evaluate_integer_arithmetic(IntegerArithmetic::Add(&acc, &int(x)), None, &mut meter)
            .completed()
            .unwrap();
    }
    assert_eq!(acc, int(3));
    assert_eq!(meter.admitted_charges(), [INTEGER, INTEGER].concat());
    assert_eq!(consumed(&meter), [3, 0, 0, 0, 0, 0, 0, 2, 6, 2]);
}

/// Short-circuit choice (Boolean `and`/`or`/`implies` skipping the right
/// operand) is the quire-specification/FR-145 expression machine's business, out of scope for
/// this crate: `evaluate_boolean` takes both operands already decided. This
/// test plays the caller's role directly, deciding whether to evaluate the
/// right operand before calling `evaluate_boolean`, and checks that the
/// resulting charge totals match the P11 short-circuit schedule.
///
/// Trace: TC-023, FR-007-AC-7
#[test]
fn tc_023_p11_implies_short_circuits_and_charges_boolean_retention() {
    let gt0 = |value: i128| {
        move |meter: &mut Meter| {
            order_numbers(
                OrderingOperator::Greater,
                OrderedOperands::Integers(&int(value), &int(0)),
                meter,
            )
        }
    };
    for (a, expected_work, expected_results, right_runs) in [(3, 7, 3, true), (-1, 4, 2, false)] {
        let mut meter = Meter::new(UNLIMITED);
        let left = gt0(a)(&mut meter).completed().unwrap();
        let mut ran = false;
        // `a implies b` is decided without `b` when `a` is false.
        let right = if left {
            ran = true;
            gt0(2)(&mut meter).completed().unwrap()
        } else {
            false
        };
        let outcome = evaluate_boolean(BooleanConnective::Implies(left, right), &mut meter);
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
    let right = gt0(2)(&mut meter).completed().unwrap();
    let denied = evaluate_boolean(BooleanConnective::And(true, right), &mut meter);
    assert_eq!(
        denied,
        Outcome::Incomplete(work_denied(3, ChargePoint::BooleanResultRetain))
    );
    // ...and a stopped right operand is returned unchanged, with no retention.
    let mut meter = Meter::new(work(2));
    let stopped = gt0(2)(&mut meter);
    assert_eq!(
        stopped,
        Outcome::Incomplete(work_denied(2, ChargePoint::OrderingResultRetain))
    );
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::BooleanResultRetain));

    // A stopped right operand never reaches `evaluate_boolean`.
    let mut meter = Meter::new(UNLIMITED);
    let right_outcome: Outcome<bool> = Outcome::Undefined(Undefined::DivisionByZero);
    let undefined = match right_outcome {
        Outcome::Completed(right) => {
            evaluate_boolean(BooleanConnective::Or(false, right), &mut meter)
        }
        stopped => stopped,
    };
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
            // `Outcome` is `#[non_exhaustive]` (NFR-002-AC-3); no safe retyping
            // exists for a variant this helper does not know about.
            _ => unreachable!("Outcome gained a variant `map_stop` does not know how to retype"),
        }
    }
}

/// Trace: TC-023, FR-007-AC-7
#[test]
fn tc_023_connective_truth_tables() {
    for left in [false, true] {
        for right in [false, true] {
            let cases = [
                (BooleanConnective::And(left, right), left && right),
                (BooleanConnective::Or(left, right), left || right),
                (BooleanConnective::Implies(left, right), !left || right),
            ];
            for (connective, expected) in cases {
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_boolean(connective, &mut meter);
                assert_eq!(outcome, Outcome::Completed(expected));
                assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
                assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
            }
        }
    }
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(
        evaluate_boolean(BooleanConnective::Not(false), &mut meter),
        Outcome::Completed(true)
    );
    assert_eq!(meter.admitted_charges(), [ChargePoint::BooleanResultRetain]);
    assert_eq!(
        evaluate_boolean(BooleanConnective::Not(true), &mut Meter::new(work(0))),
        Outcome::Incomplete(work_denied(0, ChargePoint::BooleanResultRetain))
    );
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_rational_amounts_zero_divisors_and_domains() {
    // `1/2 + 1/2`: `N = max(1 + 2, 1 + 2) + 1 = 4`, `D = 2 + 2 = 4`; the
    // unreduced `4/4` normalizes at 3 bits to `1`.
    let mut meter = Meter::new(UNLIMITED);
    let (half, zero) = (ratio(1, 2), ratio(0, 1));
    let sum = evaluate_rational_arithmetic(RationalArithmetic::Add(&half, &half), None, &mut meter);
    assert_eq!(sum, Outcome::Completed(ratio(1, 1)));
    assert_eq!(consumed(&meter), [4, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    let mut tuple = [u64::MAX; 10];
    tuple[0] = 3;
    assert_eq!(
        evaluate_rational_arithmetic(
            RationalArithmetic::Add(&half, &half),
            None,
            &mut Meter::new(limits(tuple))
        ),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: 3,
            consumed: 2,
            next_charge: int(4),
            charge_point: ChargePoint::RationalArithmeticArithmetic,
        })
    );

    // A zero divisor is undefined after `rational-arithmetic.operands` only.
    // Integer `/` producing a rational enters each operand as `n/1`.
    let (one_over_one, zero_over_one) = (
        Rational::from_integer(int(1)),
        Rational::from_integer(int(0)),
    );
    for operation in [
        RationalArithmetic::Divide(&half, &zero),
        RationalArithmetic::Divide(&one_over_one, &zero_over_one),
    ] {
        let mut meter = Meter::new(UNLIMITED);
        assert_eq!(
            evaluate_rational_arithmetic(operation, None, &mut meter),
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
        evaluate_rational_arithmetic(
            RationalArithmetic::Add(&third, &third),
            Some(&domain),
            &mut meter
        ),
        Outcome::Refused(Refusal::RationalOutOfDomain)
    );
    assert_eq!(meter.admitted_charges(), &RATIONAL[..3]);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    assert_eq!(
        evaluate_rational_arithmetic(
            RationalArithmetic::Subtract(&two_thirds, &ratio(1, 6)),
            Some(&domain),
            &mut Meter::new(UNLIMITED)
        ),
        Outcome::Completed(ratio(1, 2))
    );
    // Unary minus: `max(bits(a), bits(b))`, then `N = -a`, `D = b`.
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(
        evaluate_rational_arithmetic(RationalArithmetic::Negate(&ratio(-5, 8)), None, &mut meter),
        Outcome::Completed(ratio(5, 8))
    );
    assert_eq!(consumed(&meter), [4, 0, 0, 0, 0, 0, 0, 1, 4, 1]);
    assert_named_denials(|meter| {
        evaluate_rational_arithmetic(
            RationalArithmetic::Multiply(&third, &two_thirds),
            Some(&domain),
            meter,
        )
    });
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_ordering_amounts_for_rationals_and_retained_decimals() {
    // `7/3 < 5/8`: operands maxparts 4; arithmetic
    // `max(bits(7) + bits(8), bits(5) + bits(3)) = max(7, 5) = 7`.
    let mut meter = Meter::new(UNLIMITED);
    let (left, right) = (ratio(7, 3), ratio(5, 8));
    let less = order_numbers(
        OrderingOperator::Less,
        OrderedOperands::Rationals(&left, &right),
        &mut meter,
    );
    assert_eq!(less, Outcome::Completed(false));
    assert_eq!(meter.admitted_charges(), ORDERING);
    assert_eq!(consumed(&meter), [7, 0, 0, 0, 0, 0, 0, 2, 3, 1]);

    // Retained, never normalized: `(0, 5) >= (0, 0)` shifts zero by 5:
    // `sbits(0,5) = 1 + bits(10^5) = 18`, `sdigits(0,5) = 1 + 5 = 6`.
    let mut meter = Meter::new(UNLIMITED);
    let (a, b) = (Decimal::new(int(0), 5), Decimal::new(int(0), 0));
    let ge = order_numbers(
        OrderingOperator::GreaterOrEqual,
        OrderedOperands::Decimals(&a, &b),
        &mut meter,
    );
    assert_eq!(ge, Outcome::Completed(true));
    assert_eq!(consumed(&meter), [18, 6, 5, 0, 0, 0, 0, 2, 3, 1]);

    // `(100, 2) <= (2, 0)`: `sbits(2,2) = bits(2) + bits(100) = 9` and
    // `sdigits(2,2) = 3`, scale expansion 2.
    let mut meter = Meter::new(UNLIMITED);
    let (a, b) = (Decimal::new(int(100), 2), Decimal::new(int(2), 0));
    let le = order_numbers(
        OrderingOperator::LessOrEqual,
        OrderedOperands::Decimals(&a, &b),
        &mut meter,
    );
    assert_eq!(le, Outcome::Completed(true));
    assert_eq!(consumed(&meter), [9, 3, 2, 0, 0, 0, 0, 2, 3, 1]);

    // An aligned coefficient beyond u64 is sized analytically, never materialized.
    let (a, b) = (Decimal::new(int(1), 0), Decimal::new(int(1), u32::MAX));
    let mut tuple = [u64::MAX; 10];
    tuple[1] = 64;
    assert_eq!(
        order_numbers(
            OrderingOperator::Less,
            OrderedOperands::Decimals(&a, &b),
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
        order_numbers(
            OrderingOperator::Greater,
            OrderedOperands::Decimals(&a, &b),
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
    let bounded = IntegerInterval::new(int(-(1 << 63)), int((1 << 63) - 1)).unwrap();
    let mut vectors = 0_u32;
    for a in VALUES {
        for b in VALUES {
            let (x, y) = (int(a), int(b));
            // `(operation, value, operand count, operand-derived amount)`.
            let cases: [(IntegerArithmetic<'_>, i128, u64, u64); 4] = [
                (
                    IntegerArithmetic::Add(&x, &y),
                    a + b,
                    2,
                    bits(a).max(bits(b)) + 1,
                ),
                (
                    IntegerArithmetic::Subtract(&x, &y),
                    a - b,
                    2,
                    bits(a).max(bits(b)) + 1,
                ),
                (
                    IntegerArithmetic::Multiply(&x, &y),
                    a * b,
                    2,
                    bits(a) + bits(b),
                ),
                (IntegerArithmetic::Negate(&x), -a, 1, bits(a)),
            ];
            for (operation, expected, count, amount) in cases {
                // The amount never falls below the operands' or the result's bits.
                assert!(amount >= bits(a).max(bits(expected)));
                let mut meter = Meter::new(UNLIMITED);
                assert_eq!(
                    evaluate_integer_arithmetic(operation, None, &mut meter),
                    Outcome::Completed(oracle(expected))
                );
                assert_eq!(consumed(&meter), [amount, 0, 0, 0, 0, 0, 0, count, 3, 1]);
                let in_range = i64::try_from(expected).is_ok();
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_integer_arithmetic(operation, Some(&bounded), &mut meter);
                if in_range {
                    assert_eq!(outcome, Outcome::Completed(oracle(expected)));
                } else {
                    assert_eq!(outcome, Outcome::Refused(Refusal::IntegerOutOfDomain));
                    assert_eq!(meter.admitted_charges(), &INTEGER[..2]);
                }
                assert_named_denials(|meter| evaluate_integer_arithmetic(operation, None, meter));
                vectors += 1;
            }
            for operator in OrderingOperator::ALL {
                let ordering = a.cmp(&b);
                let expected = match operator {
                    OrderingOperator::Less => ordering == Ordering::Less,
                    OrderingOperator::LessOrEqual => ordering != Ordering::Greater,
                    OrderingOperator::Greater => ordering == Ordering::Greater,
                    OrderingOperator::GreaterOrEqual => ordering != Ordering::Less,
                    // `OrderingOperator` is `#[non_exhaustive]` (NFR-002-AC-3).
                    // `operator` is drawn only from `OrderingOperator::ALL`
                    // above, so this arm is unreachable unless `ALL` grows
                    // without this match being updated to match.
                    _ => unreachable!("OrderingOperator::ALL grew a variant this match omits"),
                };
                let mut meter = Meter::new(UNLIMITED);
                assert_eq!(
                    order_numbers(operator, OrderedOperands::Integers(&x, &y), &mut meter),
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
                    RationalArithmetic::Add(&left, &right),
                    Some((a * d + c * b, b * d)),
                ),
                (
                    RationalArithmetic::Subtract(&left, &right),
                    Some((a * d - c * b, b * d)),
                ),
                (
                    RationalArithmetic::Multiply(&left, &right),
                    Some((a * c, b * d)),
                ),
                (
                    RationalArithmetic::Divide(&left, &right),
                    (c != 0).then_some((a * d, b * c)),
                ),
                (RationalArithmetic::Negate(&left), Some((-a, b))),
            ];
            // The `rational-arithmetic.arithmetic` amounts of `+`, `-`, `*`,
            // `/` and unary `-`, from operand bit lengths only.
            let sum = (bits(a) + bits(d)).max(bits(c) + bits(b)) + 1;
            let amounts = [
                sum.max(bits(b) + bits(d)),
                sum.max(bits(b) + bits(d)),
                (bits(a) + bits(c)).max(bits(b) + bits(d)),
                (bits(a) + bits(d)).max(bits(b) + bits(c)),
                maxparts((a, b)),
            ];
            for (index, (operation, intermediate)) in unreduced.into_iter().enumerate() {
                let operand_bits = if index == 4 {
                    maxparts((a, b))
                } else {
                    maxparts((a, b)).max(maxparts((c, d)))
                };
                let mut meter = Meter::new(UNLIMITED);
                let outcome = evaluate_rational_arithmetic(operation, None, &mut meter);
                match intermediate {
                    None => {
                        assert_eq!(outcome, Outcome::Undefined(Undefined::DivisionByZero));
                        assert_eq!(meter.admitted_charges(), &RATIONAL[..1]);
                    }
                    Some((n, m)) => {
                        let (rn, rd) = reduce((n, m));
                        assert_eq!(outcome, Outcome::Completed(ratio(rn, rd)));
                        // The amount bounds the unreduced intermediate, whose
                        // normalize charge never raises the high-water mark.
                        assert!(amounts[index] >= operand_bits.max(maxparts((n, m))));
                        assert_eq!(consumed(&meter)[0], amounts[index]);
                        assert_named_denials(|meter| {
                            evaluate_rational_arithmetic(operation, None, meter)
                        });
                    }
                }
                vectors += 1;
            }
            let ordering = (a * d).cmp(&(c * b));
            for operator in OrderingOperator::ALL {
                let mut meter = Meter::new(UNLIMITED);
                let outcome = order_numbers(
                    operator,
                    OrderedOperands::Rationals(&left, &right),
                    &mut meter,
                );
                let expected = match operator {
                    OrderingOperator::Less => ordering.is_lt(),
                    OrderingOperator::LessOrEqual => ordering.is_le(),
                    OrderingOperator::Greater => ordering.is_gt(),
                    OrderingOperator::GreaterOrEqual => ordering.is_ge(),
                    // `OrderingOperator` is `#[non_exhaustive]` (NFR-002-AC-3).
                    // `operator` is drawn only from `OrderingOperator::ALL`
                    // above, so this arm is unreachable unless `ALL` grows
                    // without this match being updated to match.
                    _ => unreachable!("OrderingOperator::ALL grew a variant this match omits"),
                };
                assert_eq!(outcome, Outcome::Completed(expected));
                let cross = (bits(a) + bits(d)).max(bits(c) + bits(b));
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

fn bits_under(bits: u64) -> ScalarLimits {
    let mut tuple = [u64::MAX; 10];
    tuple[0] = bits;
    limits(tuple)
}

fn bits_denied(limit: u64, consumed: u64, next: u64, point: ChargePoint) -> Incomplete {
    Incomplete {
        limit_kind: LimitKind::IntegerBits,
        limit,
        consumed,
        next_charge: int(i128::from(next)),
        charge_point: point,
    }
}

/// `run` completes with `limit` at `exact`, and one under is denied as `denied`.
fn assert_exact_and_one_under<T: std::fmt::Debug + PartialEq>(
    run: impl Fn(&mut Meter) -> Outcome<T>,
    exact: u64,
    denied: Incomplete,
) {
    let mut meter = Meter::new(bits_under(exact));
    assert!(run(&mut meter).completed().is_some());
    assert_eq!(meter.consumed(LimitKind::IntegerBits), exact);
    assert_eq!(
        run(&mut Meter::new(bits_under(exact - 1))),
        Outcome::Incomplete(denied)
    );
}

/// Every charge of `run`, in order, admitted with exactly as many work units as
/// precede it and denied one under.
fn assert_work_exact_and_one_under<T: std::fmt::Debug + PartialEq>(
    run: impl Fn(&mut Meter) -> Outcome<T>,
    points: &[ChargePoint],
) {
    for (index, point) in points.iter().enumerate() {
        let before = u64::try_from(index).unwrap();
        assert_eq!(
            run(&mut Meter::new(work(before))),
            Outcome::Incomplete(work_denied(before, *point))
        );
        let mut meter = Meter::new(work(before + 1));
        let outcome = run(&mut meter);
        assert_eq!(meter.admitted_charges(), &points[..=index]);
        if index + 1 == points.len() {
            assert!(outcome.completed().is_some());
        }
    }
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_arithmetic_and_normalize_charges_at_exact_and_one_under_limits() {
    // `2/3 × 3/2`: operands 2 bits; `N = bits(2) + bits(3) = 4`,
    // `D = bits(3) + bits(2) = 4`; the unreduced `6/6` normalizes at 3 bits.
    let (two_thirds, three_halves) = (ratio(2, 3), ratio(3, 2));
    let product = |meter: &mut Meter| {
        evaluate_rational_arithmetic(
            RationalArithmetic::Multiply(&two_thirds, &three_halves),
            None,
            meter,
        )
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(product(&mut meter), Outcome::Completed(ratio(1, 1)));
    assert_eq!(consumed(&meter), [4, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_exact_and_one_under(
        product,
        4,
        bits_denied(3, 2, 4, ChargePoint::RationalArithmeticArithmetic),
    );
    // The normalize amount never exceeds the admitted arithmetic amount, so
    // its one-under limit is on work: three charges admitted, the fourth denied.
    assert_work_exact_and_one_under(product, &RATIONAL);

    // `3/4 ÷ 5/7`: `N = bits(3) + bits(7) = 5`, `D = bits(4) + bits(5) = 6`;
    // the unreduced `21/20` normalizes at 5 bits.
    let (three_quarters, five_sevenths) = (ratio(3, 4), ratio(5, 7));
    let quotient = |meter: &mut Meter| {
        evaluate_rational_arithmetic(
            RationalArithmetic::Divide(&three_quarters, &five_sevenths),
            None,
            meter,
        )
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(quotient(&mut meter), Outcome::Completed(ratio(21, 20)));
    assert_eq!(consumed(&meter), [6, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_exact_and_one_under(
        quotient,
        6,
        bits_denied(5, 3, 6, ChargePoint::RationalArithmeticArithmetic),
    );
    assert_work_exact_and_one_under(quotient, &RATIONAL);

    // Integer `n / m` as a rational: `3 / 2` enters as `3/1 ÷ 2/1`.
    let (three_over_one, two_over_one) = (
        Rational::from_integer(int(3)),
        Rational::from_integer(int(2)),
    );
    let halves = |meter: &mut Meter| {
        evaluate_rational_arithmetic(
            RationalArithmetic::Divide(&three_over_one, &two_over_one),
            None,
            meter,
        )
    };
    assert_exact_and_one_under(
        halves,
        3,
        bits_denied(2, 2, 3, ChargePoint::RationalArithmeticArithmetic),
    );

    // `5/7 - 4/7`: operands 3 bits; `N = max(3 + 3, 3 + 3) + 1 = 7`,
    // `D = 3 + 3 = 6`; the unreduced `7/49` normalizes at 6 bits to `1/7`.
    let (five, four) = (ratio(5, 7), ratio(4, 7));
    let difference = |meter: &mut Meter| {
        evaluate_rational_arithmetic(RationalArithmetic::Subtract(&five, &four), None, meter)
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(difference(&mut meter), Outcome::Completed(ratio(1, 7)));
    assert_eq!(consumed(&meter), [7, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_exact_and_one_under(
        difference,
        7,
        bits_denied(6, 3, 7, ChargePoint::RationalArithmeticArithmetic),
    );
    assert_work_exact_and_one_under(difference, &RATIONAL);

    // Rational unary `-`: `max(bits(-5), bits(8)) = 4` equals the operands
    // amount, so one under stops at the operands.
    let negative = ratio(-5, 8);
    let negated = |meter: &mut Meter| {
        evaluate_rational_arithmetic(RationalArithmetic::Negate(&negative), None, meter)
    };
    assert_exact_and_one_under(
        negated,
        4,
        bits_denied(3, 0, 4, ChargePoint::RationalArithmeticOperands),
    );
    assert_work_exact_and_one_under(negated, &RATIONAL);

    // Integer `1000 - 999 = 1`: `max(10, 10) + 1 = 11`, whatever the result.
    let (thousand, nines) = (int(1000), int(999));
    let small = |meter: &mut Meter| {
        evaluate_integer_arithmetic(IntegerArithmetic::Subtract(&thousand, &nines), None, meter)
    };
    let mut meter = Meter::new(UNLIMITED);
    assert_eq!(small(&mut meter), Outcome::Completed(int(1)));
    assert_eq!(consumed(&meter), [11, 0, 0, 0, 0, 0, 0, 2, 3, 1]);
    assert_exact_and_one_under(
        small,
        11,
        bits_denied(10, 10, 11, ChargePoint::IntegerArithmeticArithmetic),
    );
    assert_work_exact_and_one_under(small, &INTEGER);

    // Integer `255 × 255 = 65025`: operands 8 bits, arithmetic `8 + 8 = 16`.
    let byte = int(255);
    let square = |meter: &mut Meter| {
        evaluate_integer_arithmetic(IntegerArithmetic::Multiply(&byte, &byte), None, meter)
    };
    assert_exact_and_one_under(
        square,
        16,
        bits_denied(15, 8, 16, ChargePoint::IntegerArithmeticArithmetic),
    );
    assert_work_exact_and_one_under(square, &INTEGER);

    // Integer unary `-`: `bits(-256) = 9` equals the operands amount.
    let floor = int(-256);
    let negated = |meter: &mut Meter| {
        evaluate_integer_arithmetic(IntegerArithmetic::Negate(&floor), None, meter)
    };
    assert_exact_and_one_under(
        negated,
        9,
        bits_denied(8, 0, 9, ChargePoint::IntegerArithmeticOperands),
    );
    assert_work_exact_and_one_under(negated, &INTEGER);

    // Ordering `7/3 < 5/8` at its cross amount 7, over operands of 4.
    let (seven_thirds, five_eighths) = (ratio(7, 3), ratio(5, 8));
    let less = |meter: &mut Meter| {
        order_numbers(
            OrderingOperator::Less,
            OrderedOperands::Rationals(&seven_thirds, &five_eighths),
            meter,
        )
    };
    assert_exact_and_one_under(
        less,
        7,
        bits_denied(6, 4, 7, ChargePoint::OrderingArithmetic),
    );
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_result_sizes_at_power_of_two_edges_and_cancellation() {
    let power = |exponent: u32| -> Integer {
        let mut value = int(1);
        for _ in 0..exponent {
            value = evaluate_integer_arithmetic(
                IntegerArithmetic::Add(&value, &value),
                None,
                &mut Meter::new(UNLIMITED),
            )
            .completed()
            .unwrap();
        }
        value
    };
    // The consumed amount of `operation`, which must bound the result it
    // computes, and that result's bit length.
    let charged = |operation: IntegerArithmetic<'_>| {
        let mut meter = Meter::new(UNLIMITED);
        let result = evaluate_integer_arithmetic(operation, None, &mut meter)
            .completed()
            .unwrap();
        let amount = meter.consumed(LimitKind::IntegerBits);
        assert!(amount >= result.magnitude_bits());
        (amount, result.magnitude_bits())
    };
    let one = int(1);
    for exponent in [63_u32, 64, 65, 127, 128, 200, 511] {
        let two_k = power(exponent);
        let below = evaluate_integer_arithmetic(
            IntegerArithmetic::Subtract(&two_k, &one),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
        let above = evaluate_integer_arithmetic(
            IntegerArithmetic::Add(&two_k, &one),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
        let k = u64::from(exponent);
        // (2^k - 1)(2^k + 1) = 2^2k - 1 has 2k bits; the charge is k + (k + 1).
        assert_eq!(
            charged(IntegerArithmetic::Multiply(&below, &above)),
            (2 * k + 1, 2 * k)
        );
        // 2^k × 2^k = 2^2k has 2k + 1 bits; the charge is (k + 1) + (k + 1).
        assert_eq!(
            charged(IntegerArithmetic::Multiply(&two_k, &two_k)),
            (2 * k + 2, 2 * k + 1)
        );
        assert_eq!(
            charged(IntegerArithmetic::Add(&below, &one)),
            (k + 1, k + 1)
        );
        // Cancellation leaves the charge at the operands' `max + 1`.
        assert_eq!(
            charged(IntegerArithmetic::Subtract(&above, &two_k)),
            (k + 2, 1)
        );
        assert_eq!(
            charged(IntegerArithmetic::Subtract(&two_k, &two_k)),
            (k + 2, 1)
        );
        assert_eq!(
            charged(IntegerArithmetic::Subtract(&one, &above)),
            (k + 2, k + 1)
        );
        let negative = evaluate_integer_arithmetic(
            IntegerArithmetic::Negate(&below),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
        assert_eq!(
            charged(IntegerArithmetic::Add(&negative, &above)),
            (k + 2, 2)
        );
        assert_eq!(
            charged(IntegerArithmetic::Multiply(&negative, &negative)),
            (2 * k, 2 * k)
        );
        assert_eq!(charged(IntegerArithmetic::Negate(&above)), (k + 1, k + 1));
    }
}

/// Trace: TC-016, FR-006-AC-3
#[test]
fn tc_016_decimal_digits_at_every_power_of_ten_boundary() {
    let mut value = int(1);
    let ten = int(10);
    let one = int(1);
    for exponent in 0..120 {
        let below = evaluate_integer_arithmetic(
            IntegerArithmetic::Subtract(&value, &one),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
        let above = evaluate_integer_arithmetic(
            IntegerArithmetic::Add(&value, &one),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
        for candidate in [&below, &value, &above] {
            let rendered = candidate.to_string();
            let expected = u64::try_from(rendered.trim_start_matches('-').len()).unwrap();
            assert_eq!(
                candidate.decimal_digits(),
                expected,
                "{rendered} at 10^{exponent}"
            );
        }
        value = evaluate_integer_arithmetic(
            IntegerArithmetic::Multiply(&value, &ten),
            None,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
    }
    for value in VALUES {
        let expected = u64::try_from(value.unsigned_abs().to_string().len()).unwrap();
        assert_eq!(int(value).decimal_digits(), expected);
    }
}

/// Trace: TC-017, FR-006-AC-3
#[test]
fn tc_017_charge_log_is_capped_and_counters_stay_exact() {
    let extra = 3_u64;
    let total = u64::try_from(CHARGE_LOG_CAPACITY).unwrap() + extra;
    let mut meter = Meter::new(UNLIMITED);
    for _ in 0..total {
        assert_eq!(
            evaluate_boolean(BooleanConnective::Not(true), &mut meter),
            Outcome::Completed(false)
        );
    }
    assert_eq!(meter.admitted_charges().len(), CHARGE_LOG_CAPACITY);
    assert!(meter.charge_log_truncated());
    assert_eq!(meter.consumed(LimitKind::WorkUnits), total);
    assert_eq!(meter.consumed(LimitKind::ResultUnits), total);

    // An injected denial still counts occurrences past the log.
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::BooleanResultRetain,
        occurrence: NonZeroU64::new(total).unwrap(),
    });
    for _ in 1..total {
        assert!(evaluate_boolean(BooleanConnective::Not(false), &mut meter)
            .completed()
            .is_some());
    }
    assert_eq!(
        evaluate_boolean(BooleanConnective::Not(false), &mut meter),
        Outcome::Incomplete(work_denied(total - 1, ChargePoint::BooleanResultRetain))
    );
    let mut fresh = Meter::new(UNLIMITED);
    assert!(!fresh.charge_log_truncated());
    assert!(evaluate_boolean(BooleanConnective::Not(false), &mut fresh)
        .completed()
        .is_some());
    assert!(!fresh.charge_log_truncated());
}

/// `evaluate_integer_arithmetic` builds its `Outcome` directly rather than
/// through an inner `Result<Integer, Stop>` round-trip -- a Kani-provability
/// rule with no owning FR or NFR (see AD-002's Risks section). A regression
/// back to a `Result`-returning helper wrapped in `Outcome::from_stop` would
/// behave identically and compile cleanly, so this is a source check rather
/// than a behavioural one.
///
/// Trace: AD-002
#[test]
fn tc_kani_layout_integer_arithmetic_builds_outcome_directly() {
    let source = include_str!("../src/exact/numeric.rs");
    let start = source
        .find("pub fn evaluate_integer_arithmetic(")
        .expect("evaluate_integer_arithmetic exists in src/exact/numeric.rs");
    let after_signature = &source[start..];
    // The function's own closing brace is unindented; every brace inside its
    // body (match arms, if-let blocks) is indented, so this is the first
    // unindented one after the signature.
    let end = after_signature
        .find("\n}\n")
        .expect("evaluate_integer_arithmetic has a closing brace")
        + "\n}".len();
    let function = &after_signature[..end];
    assert!(
        !function.contains("Outcome::from_stop"),
        "evaluate_integer_arithmetic wraps a Result in Outcome::from_stop again"
    );
    assert!(
        function.contains("Outcome::Completed(result)"),
        "evaluate_integer_arithmetic no longer builds its Outcome directly"
    );
}
