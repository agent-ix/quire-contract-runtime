// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-185 exact decimal semantics: runtime versus authority.
//!
//! Every tabled vector D01–D21 is evaluated through both boundaries. D20 and
//! D21 meter decimal ordering, which QSpec 5d88578 introduced after the pinned
//! authority d9d5273: their values agree with the authority, and their charges
//! are checked against QSpec 5d88578 only, a named upstream lag pending
//! agent-ix/quire-spec-language#119.

#[macro_use]
mod support;

use support::rt_side::*;

const EVALUATED: [&str; 21] = [
    "D01", "D02", "D03", "D04", "D05", "D06", "D07", "D08", "D09", "D10", "D11", "D12", "D13", "D14",
    "D15", "D16", "D17", "D18", "D19", "D20", "D21",
];

/// Vectors whose value agrees with authority d9d5273 but whose charges the
/// authority does not yet meter (known upstream lag, pending
/// agent-ix/quire-spec-language#119). Their charges are checked against QSpec
/// 5d88578 only; runtime charges are never bent to the authority.
const CHARGES_PENDING_QSL_119: [&str; 2] = ["D20", "D21"];

/// Trace: TC-019, FR-007-AC-6
#[test]
fn tc_019_every_tc185_vector_is_evaluated() {
    let expected: Vec<String> = (1..=21).map(|n| format!("D{n:02}")).collect();
    assert_eq!(EVALUATED.to_vec(), expected);
    println!(
        "TC-185 agreement: {} evaluated, 0 admission-only, {} with charges pending QSL #119",
        EVALUATED.len(),
        CHARGES_PENDING_QSL_119.len()
    );
}

fn coefficient_scale(decimal: &Decimal) -> ((Integer, u32), (Integer, u32)) {
    let (repr, norm) = (decimal.representation(), decimal.normalized());
    ((repr.coefficient().clone(), repr.scale()), (norm.coefficient().clone(), norm.scale()))
}

fn loss_fields(result: &DecimalResult) -> (Integer, Integer, Integer, u32, RoundingMode) {
    let loss = result.loss().expect("a rounded result carries a loss record");
    (
        loss.exact_numerator().clone(),
        loss.exact_denominator(),
        loss.rounded_coefficient().clone(),
        loss.rounded_scale(),
        loss.mode(),
    )
}

/// `Decimal[-1000,1000;0,scale;mode]` as an index into `RoundingMode::ALL`.
const WIDE: (i64, i64) = (-1000, 1000);

/// Trace: TC-019, FR-007-AC-2, FR-007-AC-6
#[test]
fn tc_019_d01_equal_values_retain_distinct_representations() {
    let (a, b, rounded) = agree! {{
        let (a, b) = (dec(10, 1), dec(100, 2));
        let round = |d: &Decimal| {
            evaluate_decimal(DecimalOperation::Round(d), &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::Exact), &mut Meter::new(UNLIMITED))
        };
        let rounded = (round(&a), round(&b), a.numerically_equal(&b), a.compare(&b));
        (a, b, rounded)
    }};
    assert_eq!(coefficient_scale(&a), ((int(10), 1), (int(1), 0)));
    assert_eq!(coefficient_scale(&b), ((int(100), 2), (int(1), 0)));
    assert!(rounded.2);
    assert_eq!(rounded.3, std::cmp::Ordering::Equal);
}

/// Trace: TC-019, FR-007-AC-2, FR-007-AC-6
#[test]
fn tc_019_d02_d04_exact_arithmetic_has_no_loss() {
    for mode in 0..6_usize {
        let (sum, quotient) = agree! {{
            let mode = RoundingMode::ALL[mode];
            let wide = |scale| decimal_type(WIDE.0, WIDE.1, 0, scale, mode);
            (
                evaluate_decimal(DecimalOperation::Add(&dec(125, 2), &dec(75, 2)), &wide(2), &mut Meter::new(UNLIMITED)),
                evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), &dec(8, 0)), &wide(3), &mut Meter::new(UNLIMITED)),
            )
        }};
        let (sum, quotient) = (sum.completed().unwrap(), quotient.completed().unwrap());
        assert_eq!(coefficient_scale(sum.value()).1, (int(2), 0));
        assert!(sum.loss().is_none());
        assert_eq!(coefficient_scale(quotient.value()).0, (int(125), 3));
        assert!(quotient.loss().is_none());
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-007-AC-6
#[test]
fn tc_019_d03_every_mode_rounds_both_signed_halves() {
    let table = [
        (RoundingMode::TowardZero, 2, -2),
        (RoundingMode::TowardPositive, 3, -2),
        (RoundingMode::TowardNegative, 2, -3),
        (RoundingMode::NearestEven, 2, -2),
        (RoundingMode::NearestAway, 3, -3),
    ];
    for coefficient in [25_i64, -25] {
        let outcomes = agree! {{
            RoundingMode::ALL.map(|mode| {
                evaluate_decimal(
                    DecimalOperation::Round(&dec(coefficient, 1)),
                    &decimal_type(WIDE.0, WIDE.1, 0, 0, mode),
                    &mut Meter::new(UNLIMITED),
                )
            })
        }};
        assert_eq!(outcomes[0], Outcome::Refused(Refusal::InexactDecimal));
        for (outcome, (mode, positive, negative)) in outcomes[1..].iter().zip(table) {
            assert_eq!(RoundingMode::ALL.iter().position(|m| *m == mode).is_some(), true);
            let expected = if coefficient > 0 { positive } else { negative };
            let result = outcome.clone().completed().unwrap();
            assert_eq!(coefficient_scale(result.value()).0, (int(expected), 0), "{mode:?}");
            assert_eq!(loss_fields(&result), (int(coefficient.signum() as i128 * 5), int(2), int(expected), 0, mode));
        }
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-007-AC-6
#[test]
fn tc_019_d05_d12_recurring_quotient_and_no_re_rounding() {
    let (exact, nearest, narrow) = agree! {{
        let run = |lower, upper, mode| {
            evaluate_decimal(
                DecimalOperation::Divide(&dec(1, 0), &dec(3, 0)),
                &decimal_type(lower, upper, 0, 2, mode),
                &mut Meter::new(UNLIMITED),
            )
        };
        (
            run(WIDE.0, WIDE.1, RoundingMode::Exact),
            run(WIDE.0, WIDE.1, RoundingMode::NearestEven),
            run(-10, 10, RoundingMode::NearestEven),
        )
    }};
    assert_eq!(exact, Outcome::Refused(Refusal::InexactDecimal));
    let result = nearest.completed().unwrap();
    assert_eq!(coefficient_scale(result.value()).0, (int(33), 2));
    assert_eq!(loss_fields(&result), (int(1), int(3), int(33), 2, RoundingMode::NearestEven));
    assert_eq!(narrow, Outcome::Refused(Refusal::DecimalOutOfDomain));
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-1, FR-007-AC-6
#[test]
fn tc_019_d06_d07_d08_zero_divisors_and_domains() {
    let (zero, rounded, admitted) = agree! {{
        let zero: Vec<_> = [dec(0, 0), dec(0, 3)]
            .iter()
            .flat_map(|z| RoundingMode::ALL.map(|mode| {
                evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), z), &decimal_type(WIDE.0, WIDE.1, 0, 2, mode), &mut Meter::new(UNLIMITED))
            }))
            .collect();
        let rounded = evaluate_decimal(
            DecimalOperation::Round(&dec(25, 1)),
            &decimal_type(-2, 2, 0, 0, RoundingMode::NearestAway),
            &mut Meter::new(UNLIMITED),
        );
        let admitted = [-3_i64, -2, 2, 3].map(|c| {
            evaluate_decimal(DecimalOperation::Round(&dec(c, 0)), &decimal_type(-2, 2, 0, 0, RoundingMode::Exact), &mut Meter::new(UNLIMITED))
        });
        (zero, rounded, admitted)
    }};
    assert!(zero.iter().all(|o| *o == Outcome::Undefined(Undefined::DivisionByZero)));
    assert_eq!(rounded, Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(admitted[0], Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(admitted[3], Outcome::Refused(Refusal::DecimalOutOfDomain));
    for (outcome, c) in [(&admitted[1], -2), (&admitted[2], 2)] {
        assert_eq!(coefficient_scale(outcome.clone().completed().unwrap().value()).1, (int(c), 0));
    }
}

const D09: [u64; 10] = [7, 3, 2, 0, 0, 0, 0, 2, 5, 1];

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_019_d09_exact_tuple_and_named_denials() {
    let (exact, short, denied) = agree! {{
        let run = |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), &dec(3, 0)), &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::NearestEven), m)
        };
        let mut four = D09;
        four[8] = 4;
        (metered(limits(D09), run), metered(limits(four), run), denials(limits(D09), run))
    }};
    use ChargePoint::*;
    assert_eq!(coefficient_scale(exact.0.completed().unwrap().value()).0, (int(33), 2));
    assert_eq!(exact.1, [DecimalOperands, DecimalScaleExpansion, DecimalArithmetic, DecimalRounding, DecimalResultRetain]);
    assert_eq!(exact.2, [7, 3, 2, 0, 0, 0, 0, 2, 5, 1]);
    assert_eq!(short.0, Outcome::Incomplete(work_denied(4, DecimalResultRetain)));
    assert_eq!(denied.len(), 5);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-1, FR-007-AC-6
#[test]
fn tc_019_d10_malformed_decimal_types_are_ill_typed() {
    let declared = agree! {{
        [(3, 2, 0, 0), (0, 9, 2, 1), (0, 9, 0, 4_294_967_296), (2, 2, 1, 1), (0, 9, 0, 4_294_967_295)]
            .map(|(lo, hi, min, max): (i64, i64, u64, u64)| {
                DecimalType::new(Integer::from(lo), Integer::from(hi), min, max, RoundingMode::Exact).map(|_| ())
            })
    }};
    let malformed = Err(IllTyped { cause: IllTypedCause::MalformedDecimalType });
    assert_eq!(declared[..3], [malformed; 3]);
    assert_eq!(declared[3..], [Ok(()), Ok(())]);
}

/// Trace: TC-019, FR-007-AC-2, FR-007-AC-6
#[test]
fn tc_019_d11_d19_membership_lifts_to_the_minimum_scale() {
    let membership = agree! {{
        let t = |lo, hi, min, max| decimal_type(lo, hi, min, max, RoundingMode::Exact);
        let wide = t(0, 10_000, 2, 2);
        let narrow = t(0, 9999, 2, 2);
        let largest = t(0, 10, 4_294_967_295, 4_294_967_295);
        [
            wide.contains(&dec(1, 0)),
            wide.contains(&dec(11, 1)),
            wide.contains(&dec(1000, 3)),
            wide.contains(&dec(9999, 2)),
            narrow.contains(&dec(100, 0)),
            narrow.contains(&dec(9999, 2)),
            t(-9, 9, 0, 2).contains(&dec(1, 3)),
            largest.contains(&dec(0, 0)),
            largest.contains(&dec(1, 0)),
        ]
    }};
    assert_eq!(membership, [true, true, true, true, false, true, false, true, false]);
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d13_discarded_zero_digits_are_not_rounding() {
    const D13: [u64; 10] = [9, 3, 0, 0, 0, 0, 0, 2, 4, 1];
    for mode in [RoundingMode::TowardZero, RoundingMode::Exact] {
        let exact_mode = mode == RoundingMode::Exact;
        let (exact, short_work, short_bits) = agree! {{
            let mode = if exact_mode { RoundingMode::Exact } else { RoundingMode::TowardZero };
            let run = |m: &mut Meter| {
                evaluate_decimal(DecimalOperation::Multiply(&dec(150, 2), &dec(2, 0)), &decimal_type(-9, 9, 0, 0, mode), m)
            };
            let (mut work, mut bits) = (D13, D13);
            work[8] = 3;
            bits[0] = 8;
            (metered(limits(D13), run), metered(limits(work), run), metered(limits(bits), run))
        }};
        use ChargePoint::*;
        let result = exact.0.completed().unwrap();
        assert_eq!(coefficient_scale(result.value()).0, (int(3), 0));
        assert!(result.loss().is_none());
        assert_eq!(exact.1, [DecimalOperands, DecimalScaleExpansion, DecimalArithmetic, DecimalResultRetain]);
        assert_eq!(short_work.0, Outcome::Incomplete(work_denied(3, DecimalResultRetain)));
        assert_eq!(
            short_bits.0,
            Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 8, 8, int(9), DecimalArithmetic))
        );
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d14_d15_d16_refusal_and_undefined_precede_later_charges() {
    let (d14, d15, d16) = agree! {{
        let with = |base: [u64; 10], work: u64| { let mut v = base; v[8] = work; limits(v) };
        let strict = |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), &dec(3, 0)), &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::Exact), m)
        };
        let zero = |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), &dec(0, 3)), &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::NearestEven), m)
        };
        let d16_limits = [5, 2, 0, 0, 0, 0, 0, 1, 4, 0];
        let away = |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Round(&dec(25, 1)), &decimal_type(-2, 2, 0, 0, RoundingMode::NearestAway), m)
        };
        (
            [metered(with(D09, 3), strict), metered(with(D09, 2), strict)],
            [metered(with(D09, 1), zero), metered(with(D09, 0), zero)],
            [metered(limits(d16_limits), away), metered(with(d16_limits, 3), away)],
        )
    }};
    use ChargePoint::*;
    assert_eq!(d14[0].0, Outcome::Refused(Refusal::InexactDecimal));
    assert_eq!(d14[0].1, [DecimalOperands, DecimalScaleExpansion, DecimalArithmetic]);
    assert_eq!(d14[1].0, Outcome::Incomplete(work_denied(2, DecimalArithmetic)));
    assert_eq!(d15[0].0, Outcome::Undefined(Undefined::DivisionByZero));
    assert_eq!(d15[0].1, [DecimalOperands]);
    assert_eq!(d15[1].0, Outcome::Incomplete(work_denied(0, DecimalOperands)));
    assert_eq!(d16[0].0, Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(d16[0].1, [DecimalOperands, DecimalScaleExpansion, DecimalArithmetic, DecimalRounding]);
    assert_eq!(d16[1].0, Outcome::Incomplete(work_denied(3, DecimalRounding)));
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d17_d18_scale_expansion_uses_the_retained_representation() {
    let (d17, d18, d18_short) = agree! {{
        let d17 = evaluate_decimal(
            DecimalOperation::Divide(&dec(100, 2), &dec(1, 0)),
            &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::Exact),
            &mut Meter::new(limits([7, 3, 1, 0, 0, 0, 0, 2, 4, 1])),
        );
        let run = |v: [u64; 10]| {
            evaluate_decimal(
                DecimalOperation::Divide(&dec(1234, 3), &dec(2, 0)),
                &decimal_type(WIDE.0, WIDE.1, 0, 1, RoundingMode::NearestEven),
                &mut Meter::new(limits(v)),
            )
        };
        (d17, run([11, 4, 2, 0, 0, 0, 0, 2, 5, 1]), run([11, 4, 1, 0, 0, 0, 0, 2, 5, 1]))
    }};
    let d17 = d17.completed().unwrap();
    assert_eq!(coefficient_scale(d17.value()), ((int(100), 2), (int(1), 0)));
    assert!(d17.loss().is_none());
    let d18 = d18.completed().unwrap();
    assert_eq!(coefficient_scale(d18.value()).0, (int(6), 1));
    assert_eq!(loss_fields(&d18), (int(617), int(1000), int(6), 1, RoundingMode::NearestEven));
    assert_eq!(
        d18_short,
        Outcome::Incomplete(incomplete(LimitKind::ScaleExpansion, 1, 0, int(2), ChargePoint::DecimalScaleExpansion))
    );
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_019_generated_operations_and_denials_agree() {
    const COEFFICIENTS: [i64; 10] = [-25, -10, -7, -5, -1, 0, 1, 3, 8, 10];
    let mut vectors = 0_usize;
    for (ca, sa) in COEFFICIENTS.iter().flat_map(|c| (0..3).map(move |s| (*c, s))) {
        for (cb, sb) in COEFFICIENTS.iter().flat_map(|c| (0..3).map(move |s| (*c, s))) {
            let _ = agree! {{
                let (a, b) = (dec(ca, sa), dec(cb, sb));
                let operations = [
                    DecimalOperation::Add(&a, &b),
                    DecimalOperation::Subtract(&a, &b),
                    DecimalOperation::Multiply(&a, &b),
                    DecimalOperation::Divide(&a, &b),
                    DecimalOperation::Negate(&a),
                    DecimalOperation::Round(&a),
                ];
                let mut all = Vec::new();
                for operation in operations {
                    for scale in 0..3 {
                        for mode in RoundingMode::ALL {
                            let target = decimal_type(-40, 40, 0, scale, mode);
                            let run = |m: &mut Meter| evaluate_decimal(operation, &target, m);
                            all.push((metered(UNLIMITED, run), denials(UNLIMITED, run)));
                        }
                    }
                }
                all
            }};
            vectors += 6 * 3 * 6;
        }
    }
    assert_eq!(vectors, 30 * 30 * 108);
}

/// D20/D21: `(c1,s1) < (c2,s2)` metered by the runtime, with the authority's
/// unmetered ordering for the value.
fn ordered(
    left: (i64, u32),
    right: (i64, u32),
    tuple: [u64; 10],
) -> (Outcome<bool>, Vec<ChargePoint>, Vec<u64>) {
    let authority = agree! { dec(left.0, left.1).compare(&dec(right.0, right.1)) };
    let run = metered(limits(tuple), |m| {
        evaluate_ordering(
            OrderingOperator::Less,
            OrderingOperands::Decimal(&dec(left.0, left.1), &dec(right.0, right.1)),
            m,
        )
    });
    if let Outcome::Completed(value) = run.0 {
        assert_eq!(value, OrderingOperator::Less.holds(authority), "value disagrees with the authority");
    }
    run
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d20_d21_ordering_meters_the_retained_representation() {
    use ChargePoint::{OrderingArithmetic, OrderingOperands as Operands, OrderingResultRetain};
    let d20 = [5, 2, 1, 0, 0, 0, 0, 2, 3, 1];
    let run = ordered((15, 1), (2, 0), d20);
    assert_eq!(run.0, Outcome::Completed(true));
    assert_eq!(run.1, [Operands, OrderingArithmetic, OrderingResultRetain]);
    assert_eq!(run.2, [5, 2, 1, 0, 0, 0, 0, 2, 3, 1]);
    let mut short = d20;
    short[0] = 4;
    assert_eq!(
        ordered((15, 1), (2, 0), short).0,
        Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 4, 4, int(5), OrderingArithmetic))
    );

    let d21 = [8, 3, 2, 0, 0, 0, 0, 2, 3, 1];
    let run = ordered((100, 2), (2, 0), d21);
    assert_eq!(run.0, Outcome::Completed(true));
    assert_eq!(run.1, [Operands, OrderingArithmetic, OrderingResultRetain]);
    assert_eq!(run.2, [8, 3, 2, 0, 0, 0, 0, 2, 3, 1]);
    let mut short = d21;
    short[0] = 7;
    assert_eq!(
        ordered((100, 2), (2, 0), short).0,
        Outcome::Incomplete(incomplete(LimitKind::IntegerBits, 7, 7, int(8), OrderingArithmetic))
    );
}
