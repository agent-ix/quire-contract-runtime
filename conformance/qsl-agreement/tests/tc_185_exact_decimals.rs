// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-185 exact decimal semantics: runtime versus authority.
//!
//! Every tabled vector D01–D23 is evaluated. QSpec 7d7943a derives every
//! decimal and ordering amount from the operands, after the pinned authority
//! d9d5273 (which sizes some charges from results and does not meter decimal
//! ordering). Values, outcome kinds and charge schedules agree with the
//! authority; the vectors named in [`CHARGES_PENDING_QSL_119`] have their
//! consumed counters and tuple-dependent outcomes checked against QSpec
//! 7d7943a only, a named upstream lag pending agent-ix/quire-spec-language#119.

#[macro_use]
mod support;

use support::rt_side::*;

const EVALUATED: [&str; 23] = [
    "D01", "D02", "D03", "D04", "D05", "D06", "D07", "D08", "D09", "D10", "D11", "D12", "D13",
    "D14", "D15", "D16", "D17", "D18", "D19", "D20", "D21", "D22", "D23",
];

/// Vectors whose values, outcome kinds and charge schedules agree with
/// authority d9d5273 but whose charge amounts the authority does not yet derive
/// from operands (known upstream lag, pending agent-ix/quire-spec-language#119).
/// Their consumed counters and tuple-dependent outcomes are checked against
/// QSpec 7d7943a only; runtime charges are never bent to the authority. D23's
/// only outcome is its QSpec incomplete record, so it has no authority run.
const CHARGES_PENDING_QSL_119: [&str; 6] = ["D09", "D13", "D20", "D21", "D22", "D23"];

/// Trace: TC-019, FR-007-AC-6
#[test]
fn tc_019_every_tc185_vector_is_evaluated() {
    let expected: Vec<String> = (1..=23).map(|n| format!("D{n:02}")).collect();
    assert_eq!(EVALUATED.to_vec(), expected);
    println!(
        "TC-185 agreement: {} evaluated, 0 admission-only, {} with charges pending QSL #119",
        EVALUATED.len(),
        CHARGES_PENDING_QSL_119.len()
    );
}

fn coefficient_scale(decimal: &Decimal) -> ((Integer, u32), (Integer, u32)) {
    let (repr, norm) = (decimal.representation(), decimal.normalized());
    (
        (repr.coefficient().clone(), repr.scale()),
        (norm.coefficient().clone(), norm.scale()),
    )
}

fn loss_fields(result: &DecimalResult) -> (Integer, Integer, Integer, u32, RoundingMode) {
    let loss = result
        .loss()
        .expect("a rounded result carries a loss record");
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
            assert_eq!(
                RoundingMode::ALL.iter().position(|m| *m == mode).is_some(),
                true
            );
            let expected = if coefficient > 0 { positive } else { negative };
            let result = outcome.clone().completed().unwrap();
            assert_eq!(
                coefficient_scale(result.value()).0,
                (int(expected), 0),
                "{mode:?}"
            );
            assert_eq!(
                loss_fields(&result),
                (
                    int(coefficient.signum() as i128 * 5),
                    int(2),
                    int(expected),
                    0,
                    mode
                )
            );
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
    assert_eq!(
        loss_fields(&result),
        (int(1), int(3), int(33), 2, RoundingMode::NearestEven)
    );
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
    assert!(zero
        .iter()
        .all(|o| *o == Outcome::Undefined(Undefined::DivisionByZero)));
    assert_eq!(rounded, Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(admitted[0], Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(admitted[3], Outcome::Refused(Refusal::DecimalOutOfDomain));
    for (outcome, c) in [(&admitted[1], -2), (&admitted[2], 2)] {
        assert_eq!(
            coefficient_scale(outcome.clone().completed().unwrap().value()).1,
            (int(c), 0)
        );
    }
}

const D09: [u64; 10] = [8, 3, 2, 0, 0, 0, 0, 2, 5, 1];

/// D05 `1/3` into `Decimal[-1000,1000;0,2;mode]`.
fn d05(mode: RoundingMode) -> impl Fn(&mut Meter) -> Outcome<DecimalResult> {
    move |m| {
        evaluate_decimal(
            DecimalOperation::Divide(&dec(1, 0), &dec(3, 0)),
            &decimal_type(WIDE.0, WIDE.1, 0, 2, mode),
            m,
        )
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_019_d09_exact_tuple_and_named_denials() {
    use ChargePoint::*;
    let (schedule, denied) = agree! {{
        let run = |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Divide(&dec(1, 0), &dec(3, 0)), &decimal_type(WIDE.0, WIDE.1, 0, 2, RoundingMode::NearestEven), m)
        };
        (scheduled(UNLIMITED, run), denials(UNLIMITED, run))
    }};
    assert_eq!(
        coefficient_scale(schedule.0.clone().completed().unwrap().value()).0,
        (int(33), 2)
    );
    assert_eq!(
        schedule.1,
        [
            DecimalOperands,
            DecimalScaleExpansion,
            DecimalArithmetic,
            DecimalRounding,
            DecimalResultRetain
        ]
    );
    assert_eq!(denied.len(), 5);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }

    // QSpec 7d7943a only: the operand-derived tuple is exact.
    let run = d05(RoundingMode::NearestEven);
    let exact = metered(limits(D09), &run);
    assert_eq!(exact.0, schedule.0);
    assert_eq!(exact.2, D09);
    let mut four = D09;
    four[8] = 4;
    assert_eq!(
        metered(limits(four), &run).0,
        Outcome::Incomplete(work_denied(4, DecimalResultRetain))
    );
    // One under the dividend's `sbits(1,2) = 8`, which the division repeats.
    let mut seven = D09;
    seven[0] = 7;
    assert_eq!(
        metered(limits(seven), &run).0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            7,
            2,
            int(8),
            DecimalScaleExpansion
        ))
    );
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
    let malformed = Err(IllTyped {
        cause: IllTypedCause::MalformedDecimalType,
    });
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
    assert_eq!(
        membership,
        [true, true, true, true, false, true, false, true, false]
    );
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d13_discarded_zero_digits_are_not_rounding() {
    const D13: [u64; 10] = [10, 4, 0, 0, 0, 0, 0, 2, 4, 1];
    use ChargePoint::*;
    for mode in [RoundingMode::TowardZero, RoundingMode::Exact] {
        let exact_mode = mode == RoundingMode::Exact;
        let schedule = agree! {{
            let mode = if exact_mode { RoundingMode::Exact } else { RoundingMode::TowardZero };
            scheduled(UNLIMITED, |m: &mut Meter| {
                evaluate_decimal(DecimalOperation::Multiply(&dec(150, 2), &dec(2, 0)), &decimal_type(-9, 9, 0, 0, mode), m)
            })
        }};
        let result = schedule.0.clone().completed().unwrap();
        assert_eq!(coefficient_scale(result.value()).0, (int(3), 0));
        assert!(result.loss().is_none());
        assert_eq!(
            schedule.1,
            [
                DecimalOperands,
                DecimalScaleExpansion,
                DecimalArithmetic,
                DecimalResultRetain
            ]
        );

        // QSpec 7d7943a only: `bits(150) + bits(2) = 10` on the retained operand.
        let run = |m: &mut Meter| {
            evaluate_decimal(
                DecimalOperation::Multiply(&dec(150, 2), &dec(2, 0)),
                &decimal_type(-9, 9, 0, 0, mode),
                m,
            )
        };
        let exact = metered(limits(D13), run);
        assert_eq!(exact.0, schedule.0);
        assert_eq!(exact.2, D13);
        let (mut work, mut bits) = (D13, D13);
        work[8] = 3;
        bits[0] = 9;
        assert_eq!(
            metered(limits(work), run).0,
            Outcome::Incomplete(work_denied(3, DecimalResultRetain))
        );
        assert_eq!(
            metered(limits(bits), run).0,
            Outcome::Incomplete(incomplete(
                LimitKind::IntegerBits,
                9,
                8,
                int(10),
                DecimalArithmetic
            ))
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
            [scheduled(with(D09, 3), strict), scheduled(with(D09, 2), strict)],
            [scheduled(with(D09, 1), zero), scheduled(with(D09, 0), zero)],
            [scheduled(limits(d16_limits), away), scheduled(with(d16_limits, 3), away)],
        )
    }};
    use ChargePoint::*;
    assert_eq!(d14[0].0, Outcome::Refused(Refusal::InexactDecimal));
    assert_eq!(
        d14[0].1,
        [DecimalOperands, DecimalScaleExpansion, DecimalArithmetic]
    );
    assert_eq!(
        d14[1].0,
        Outcome::Incomplete(work_denied(2, DecimalArithmetic))
    );
    assert_eq!(d15[0].0, Outcome::Undefined(Undefined::DivisionByZero));
    assert_eq!(d15[0].1, [DecimalOperands]);
    assert_eq!(
        d15[1].0,
        Outcome::Incomplete(work_denied(0, DecimalOperands))
    );
    assert_eq!(d16[0].0, Outcome::Refused(Refusal::DecimalOutOfDomain));
    assert_eq!(
        d16[0].1,
        [
            DecimalOperands,
            DecimalScaleExpansion,
            DecimalArithmetic,
            DecimalRounding
        ]
    );
    assert_eq!(
        d16[1].0,
        Outcome::Incomplete(work_denied(3, DecimalRounding))
    );
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
    assert_eq!(
        loss_fields(&d18),
        (int(617), int(1000), int(6), 1, RoundingMode::NearestEven)
    );
    assert_eq!(
        d18_short,
        Outcome::Incomplete(incomplete(
            LimitKind::ScaleExpansion,
            1,
            0,
            int(2),
            ChargePoint::DecimalScaleExpansion
        ))
    );
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_019_generated_operations_and_denials_agree() {
    const COEFFICIENTS: [i64; 10] = [-25, -10, -7, -5, -1, 0, 1, 3, 8, 10];
    let mut vectors = 0_usize;
    for (ca, sa) in COEFFICIENTS
        .iter()
        .flat_map(|c| (0..3).map(move |s| (*c, s)))
    {
        for (cb, sb) in COEFFICIENTS
            .iter()
            .flat_map(|c| (0..3).map(move |s| (*c, s)))
        {
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
                            all.push((scheduled(UNLIMITED, run), denials(UNLIMITED, run)));
                        }
                    }
                }
                all
            }};
            vectors += 6 * 3 * 6;
            // The schedule agrees with the authority above; the amounts are the
            // runtime's alone, checked against an independent QSpec 7d7943a
            // oracle.
            let (a, b) = (dec(ca, sa), dec(cb, sb));
            let operations = [
                DecimalOperation::Add(&a, &b),
                DecimalOperation::Subtract(&a, &b),
                DecimalOperation::Multiply(&a, &b),
                DecimalOperation::Divide(&a, &b),
                DecimalOperation::Negate(&a),
                DecimalOperation::Round(&a),
            ];
            for (index, operation) in operations.into_iter().enumerate() {
                for scale in 0..3 {
                    for mode in RoundingMode::ALL {
                        let target = decimal_type(-40, 40, 0, scale, mode);
                        let (outcome, charges, consumed) = metered(UNLIMITED, |m| {
                            evaluate_decimal(operation, &target, m)
                        });
                        let expected = decimal_counters(
                            index,
                            (ca.into(), sa.into()),
                            (cb.into(), sb.into()),
                            scale.into(),
                            &outcome,
                            charges.len(),
                        );
                        assert_eq!(
                            consumed, expected,
                            "{index} ({ca},{sa}) ({cb},{sb}) T={scale} {mode:?}: {outcome:?}"
                        );
                    }
                }
            }
        }
    }
    assert_eq!(vectors, 30 * 30 * 108);
}

/// `bits(x)`, where zero has length one.
fn bits(value: i128) -> u64 {
    u64::from(128 - value.unsigned_abs().leading_zeros()).max(1)
}

fn digits(value: i128) -> u64 {
    value.unsigned_abs().to_string().len() as u64
}

/// `sbits(c,k)`: `bits(c)` when `k = 0`, else `bits(c) + bits(10^k)`.
fn sbits(coefficient: i128, shift: u64) -> u64 {
    if shift == 0 {
        bits(coefficient)
    } else {
        bits(coefficient) + bits(10_i128.pow(u32::try_from(shift).unwrap()))
    }
}

/// `sdigits(c,k) = digits(c) + k`.
fn sdigits(coefficient: i128, shift: u64) -> u64 {
    digits(coefficient) + shift
}

/// The QSpec 7d7943a consumed counters of one unlimited decimal operation
/// (index into add, subtract, multiply, divide, negate, round) on `(c, s)`
/// operands at target scale `T`, derived from the operands and, for
/// `decimal.result-retain`, the retained coefficient. The work count is the
/// schedule length, which agrees with the authority.
fn decimal_counters(
    operation: usize,
    (ca, sa): (i128, u64),
    (cb, sb): (i128, u64),
    target: u64,
    outcome: &Outcome<DecimalResult>,
    work: usize,
) -> Vec<u64> {
    let unary = operation >= 4;
    let (mut high_bits, mut high_digits) = if unary {
        (bits(ca), digits(ca))
    } else {
        (bits(ca).max(bits(cb)), digits(ca).max(digits(cb)))
    };
    let mut high_scale = 0;
    let occurrences = if unary { 1 } else { 2 };
    let counters = |bits, digits, scale, results| {
        vec![
            bits,
            digits,
            scale,
            0,
            0,
            0,
            0,
            occurrences,
            work as u64,
            results,
        ]
    };
    if matches!(outcome, Outcome::Undefined(Undefined::DivisionByZero)) {
        assert_eq!((operation, cb), (3, 0));
        return counters(high_bits, high_digits, 0, 0);
    }
    // (expanded side, arithmetic (bits, digits), working scale)
    let (expanded, arithmetic, working) = match operation {
        0 | 1 => {
            let scale = sa.max(sb);
            let (ka, kb) = (scale - sa, scale - sb);
            let expanded = if ka > 0 {
                Some((ca, ka))
            } else if kb > 0 {
                Some((cb, kb))
            } else {
                None
            };
            let arithmetic = (
                sbits(ca, ka).max(sbits(cb, kb)) + 1,
                sdigits(ca, ka).max(sdigits(cb, kb)) + 1,
            );
            (expanded, arithmetic, scale)
        }
        2 => (
            None,
            (bits(ca) + bits(cb), digits(ca) + digits(cb)),
            sa + sb,
        ),
        3 => {
            // N = ca × 10^max(0, T+sb-sa), D = cb × 10^max(0, sa-sb-T).
            let up = target + sb;
            let (kn, kd) = (up.saturating_sub(sa), sa.saturating_sub(up));
            let expanded = if kn > 0 {
                Some((ca, kn))
            } else if kd > 0 {
                Some((cb, kd))
            } else {
                None
            };
            let arithmetic = (
                sbits(ca, kn).max(sbits(cb, kd)),
                sdigits(ca, kn).max(sdigits(cb, kd)),
            );
            (expanded, arithmetic, target)
        }
        _ => (None, (bits(ca), digits(ca)), sa),
    };
    if let Some((coefficient, shift)) = expanded {
        high_bits = high_bits.max(sbits(coefficient, shift));
        high_digits = high_digits.max(sdigits(coefficient, shift));
        high_scale = shift;
    }
    // `decimal.rounding` repeats the arithmetic amounts.
    high_bits = high_bits.max(arithmetic.0);
    high_digits = high_digits.max(arithmetic.1);
    match outcome {
        Outcome::Refused(Refusal::InexactDecimal | Refusal::DecimalOutOfDomain) => {
            counters(high_bits, high_digits, high_scale, 0)
        }
        Outcome::Completed(result) => {
            // Retained at `T`: the placed coefficient at `min(s, T)` upscaled
            // by `k = T - min(s, T)`.
            let upscale = target - working.min(target);
            let retained: i128 = result
                .value()
                .representation()
                .coefficient()
                .to_string()
                .parse()
                .unwrap();
            let placed = retained / 10_i128.pow(u32::try_from(upscale).unwrap());
            assert_eq!(placed * 10_i128.pow(u32::try_from(upscale).unwrap()), retained);
            counters(
                high_bits.max(sbits(placed, upscale)),
                high_digits.max(sdigits(placed, upscale)),
                high_scale.max(upscale),
                1,
            )
        }
        other => panic!("unexpected {other:?}"),
    }
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
        assert_eq!(
            value,
            OrderingOperator::Less.holds(authority),
            "value disagrees with the authority"
        );
    }
    run
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d20_d21_ordering_meters_the_retained_representation() {
    use ChargePoint::{OrderingArithmetic, OrderingOperands as Operands, OrderingResultRetain};
    // `sbits(2,1) = bits(2) + bits(10) = 6`.
    let d20 = [6, 2, 1, 0, 0, 0, 0, 2, 3, 1];
    let run = ordered((15, 1), (2, 0), d20);
    assert_eq!(run.0, Outcome::Completed(true));
    assert_eq!(run.1, [Operands, OrderingArithmetic, OrderingResultRetain]);
    assert_eq!(run.2, d20);
    let mut short = d20;
    short[0] = 5;
    assert_eq!(
        ordered((15, 1), (2, 0), short).0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            5,
            4,
            int(6),
            OrderingArithmetic
        ))
    );

    // `sbits(2,2) = bits(2) + bits(100) = 9`.
    let d21 = [9, 3, 2, 0, 0, 0, 0, 2, 3, 1];
    let run = ordered((100, 2), (2, 0), d21);
    assert_eq!(run.0, Outcome::Completed(true));
    assert_eq!(run.1, [Operands, OrderingArithmetic, OrderingResultRetain]);
    assert_eq!(run.2, d21);
    let mut short = d21;
    short[0] = 8;
    assert_eq!(
        ordered((100, 2), (2, 0), short).0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            8,
            7,
            int(9),
            OrderingArithmetic
        ))
    );
}

/// `(1,0) × (1,0)` into `Decimal[0,1;0,T;exact]`.
fn unit_square(scale: u64) -> impl Fn(&mut Meter) -> Outcome<DecimalResult> {
    move |m| {
        evaluate_decimal(
            DecimalOperation::Multiply(&dec(1, 0), &dec(1, 0)),
            &decimal_type(0, 1, 0, scale, RoundingMode::Exact),
            m,
        )
    }
}

/// Trace: TC-019, FR-007-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_d22_d23_result_retain_upscale_is_charged_before_materialization() {
    use ChargePoint::*;
    let schedule = agree! {{
        scheduled(UNLIMITED, |m: &mut Meter| {
            evaluate_decimal(DecimalOperation::Multiply(&dec(1, 0), &dec(1, 0)), &decimal_type(0, 1, 0, 1000, RoundingMode::Exact), m)
        })
    }};
    let power = big(&format!("1{}", "0".repeat(1000)));
    let result = schedule.0.clone().completed().unwrap();
    assert_eq!(
        coefficient_scale(result.value()),
        ((power, 1000), (int(1), 0))
    );
    assert!(result.loss().is_none());
    assert_eq!(
        schedule.1,
        [
            DecimalOperands,
            DecimalScaleExpansion,
            DecimalArithmetic,
            DecimalResultRetain
        ]
    );

    // QSpec 7d7943a only: `sbits(1,1000) = 3323`, `sdigits(1,1000) = 1001`.
    const D22: [u64; 10] = [3323, 1001, 1000, 0, 0, 0, 0, 2, 4, 1];
    let d22 = unit_square(1000);
    let exact = metered(limits(D22), &d22);
    assert_eq!(exact.0, schedule.0);
    assert_eq!(exact.2, D22);
    let (mut bits, mut shift) = (D22, D22);
    bits[0] = 3322;
    shift[2] = 999;
    assert_eq!(
        metered(limits(bits), &d22).0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            3322,
            2,
            int(3323),
            DecimalResultRetain
        ))
    );
    assert_eq!(
        metered(limits(shift), &d22).0,
        Outcome::Incomplete(incomplete(
            LimitKind::ScaleExpansion,
            999,
            0,
            int(1000),
            DecimalResultRetain
        ))
    );

    // D23: `sdigits(1,4294967295) = 4294967296` is denied without the power.
    let d23 = [u64::MAX, 64, 4_294_967_295, 0, 0, 0, 0, 2, 4, 1];
    let denied = metered(limits(d23), unit_square(4_294_967_295));
    assert_eq!(
        denied.0,
        Outcome::Incomplete(incomplete(
            LimitKind::DecimalDigits,
            64,
            2,
            int(4_294_967_296),
            DecimalResultRetain
        ))
    );
    assert_eq!(denied.2[8], 3);
}

/// Trace: TC-019, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_019_operand_derived_amounts_at_exact_and_one_under_limits() {
    use ChargePoint::*;
    // Each `(operation, target scale, mode, counter, exact amount, consumed
    // before the denied charge, denied charge)`, checked against QSpec 7d7943a.
    let one_under = |run: &dyn Fn(&mut Meter) -> Outcome<DecimalResult>,
                     kind: LimitKind,
                     amount: u64,
                     before: u64,
                     point: ChargePoint| {
        let index = LimitKind::ALL.iter().position(|k| *k == kind).unwrap();
        let mut tuple = [u64::MAX; 10];
        tuple[index] = amount;
        let exact = metered(limits(tuple), run);
        assert!(
            exact.0.clone().completed().is_some(),
            "{kind:?} {amount}: {:?}",
            exact.0
        );
        assert_eq!(exact.2[index], amount);
        tuple[index] = amount - 1;
        assert_eq!(
            metered(limits(tuple), run).0,
            Outcome::Incomplete(incomplete(
                kind,
                amount - 1,
                before,
                int(i128::from(amount)),
                point
            ))
        );
    };
    let mode = RoundingMode::NearestEven;
    // Alignment then addition: `(125,2) + (3,0)` expands `sbits(3,2) = 9`,
    // `sdigits(3,2) = 3`; the sum charges `max(7, 9) + 1 = 10` and `3 + 1 = 4`.
    let add = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Add(&dec(125, 2), &dec(3, 0)),
            &decimal_type(-1000, 1000, 0, 2, mode),
            m,
        )
    };
    one_under(&add, LimitKind::IntegerBits, 10, 9, DecimalArithmetic);
    one_under(&add, LimitKind::DecimalDigits, 4, 3, DecimalArithmetic);
    // Subtraction cancels to zero and still charges `max(A) + 1`.
    let cancel = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Subtract(&dec(255, 0), &dec(255, 0)),
            &decimal_type(-1000, 1000, 0, 0, mode),
            m,
        )
    };
    one_under(&cancel, LimitKind::IntegerBits, 9, 8, DecimalArithmetic);
    // Multiplication: `bits(-7) + bits(8) = 7`, `digits(-7) + digits(8) = 2`.
    let product = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Multiply(&dec(-7, 1), &dec(8, 1)),
            &decimal_type(-1000, 1000, 0, 2, mode),
            m,
        )
    };
    one_under(&product, LimitKind::IntegerBits, 7, 4, DecimalArithmetic);
    one_under(&product, LimitKind::DecimalDigits, 2, 1, DecimalArithmetic);
    // Division sizes `max(A)`, never the quotient: `(255,0) / (1,0)` at `T = 0`
    // repeats the operands' 8 bits, then retains `255` at 8 bits.
    let quotient = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Divide(&dec(255, 0), &dec(1, 0)),
            &decimal_type(-1000, 1000, 0, 0, mode),
            m,
        )
    };
    one_under(&quotient, LimitKind::IntegerBits, 8, 0, DecimalOperands);
    // Rounding repeats the arithmetic amounts: `(-25,1)` negated into scale 0
    // charges `bits(25) = 5` at arithmetic and at rounding, retaining `2`.
    let negated = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Negate(&dec(-25, 1)),
            &decimal_type(-1000, 1000, 0, 0, mode),
            m,
        )
    };
    let run = metered(UNLIMITED, &negated);
    assert_eq!(
        run.1,
        [
            DecimalOperands,
            DecimalScaleExpansion,
            DecimalArithmetic,
            DecimalRounding,
            DecimalResultRetain
        ]
    );
    assert_eq!(run.2[..2], [5, 2]);
    one_under(&negated, LimitKind::IntegerBits, 5, 0, DecimalOperands);
    // The result upscale `k = T - s`: `(3,0) × (1,0)` into scale 3 retains
    // `(3000, 3)`: `scale_expansion` 3, `sbits(3,3) = 2 + 10 = 12`,
    // `sdigits(3,3) = 4`.
    let upscale = |m: &mut Meter| {
        evaluate_decimal(
            DecimalOperation::Multiply(&dec(3, 0), &dec(1, 0)),
            &decimal_type(-9000, 9000, 0, 3, mode),
            m,
        )
    };
    one_under(
        &upscale,
        LimitKind::ScaleExpansion,
        3,
        0,
        DecimalResultRetain,
    );
    one_under(&upscale, LimitKind::IntegerBits, 12, 3, DecimalResultRetain);
    one_under(
        &upscale,
        LimitKind::DecimalDigits,
        4,
        2,
        DecimalResultRetain,
    );
}
