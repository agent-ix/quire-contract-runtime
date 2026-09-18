// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-193 IEEE exceptional and rounding profiles: runtime versus
//! authority.
//!
//! Every tabled vector F01–F31 is evaluated through both boundaries, followed
//! by the generated class matrix with every charge denied. Semantic admission
//! of the IEEE definition (the lock entry, repeated, revision, digest and
//! version mismatches, reserved intrinsic identities) is compiler-owned and
//! is not a runtime vector.

#[macro_use]
mod support;

use support::rt_side::*;

const EVALUATED: [&str; 33] = [
    "F01", "F02", "F02b", "F03", "F04", "F04b", "F05", "F06", "F07", "F08", "F09", "F10", "F11",
    "F12", "F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24",
    "F25", "F26", "F27", "F28", "F29", "F30", "F31",
];

/// Compiler-owned TC-193 procedure steps the runtime does not evaluate.
const ADMISSION_ONLY: [&str; 1] = ["semantic admission of the IEEE definition"];

const DIRECTIONS: [RoundingMode; 5] = [
    RoundingMode::NearestEven,
    RoundingMode::NearestAway,
    RoundingMode::TowardZero,
    RoundingMode::TowardPositive,
    RoundingMode::TowardNegative,
];

use ChargePoint::{IeeeExactIntermediate, IeeeOperands, IeeeResultRetain, IeeeRound};
const FINITE: [ChargePoint; 4] = [
    IeeeOperands,
    IeeeExactIntermediate,
    IeeeRound,
    IeeeResultRetain,
];
const CLASSIFIED: [ChargePoint; 2] = [IeeeOperands, IeeeResultRetain];
const TO_EXACT: [ChargePoint; 3] = [IeeeOperands, IeeeExactIntermediate, IeeeResultRetain];

fn bf(outcome: &Outcome<IeeeResult>) -> (u64, IeeeFlags) {
    let result = outcome.clone().completed().expect("completed bits");
    (result.value().bits(), result.flags())
}

fn not_exact(list: &[IeeeFlag]) -> Outcome<IeeeResult> {
    Outcome::Refused(Refusal::IeeeNotExact {
        would_be: flags(list),
    })
}

fn retain_denied(work: u64) -> Outcome<IeeeResult> {
    Outcome::Incomplete(work_denied(work, IeeeResultRetain))
}

/// Trace: TC-020, FR-007-AC-6
#[test]
fn tc_020_every_tc193_vector_is_evaluated() {
    let mut expected: Vec<String> = (1..=31).map(|n| format!("F{n:02}")).collect();
    expected.insert(2, "F02b".to_owned());
    expected.insert(5, "F04b".to_owned());
    assert_eq!(EVALUATED.to_vec(), expected);
    println!(
        "TC-193 agreement: {} evaluated, {} admission-only",
        EVALUATED.len(),
        ADMISSION_ONLY.len()
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f01_f02_f02b_zeros_and_nans_separate_equality_order_and_identity() {
    let results = agree! {{
        let cmp = |c: IeeeComparison, l: IeeeValue, r: IeeeValue| {
            compare_ieee(c, l, r, &mut Meter::new(UNLIMITED)).unwrap().completed().unwrap()
        };
        let (p0, n0) = (f32v(0x0000_0000), f32v(0x8000_0000));
        let (nan1, nan2) = (f32v(0x7fc0_0001), f32v(0x7fc0_0002));
        let (neg2, neg1) = (f32v(0xffc0_0002), f32v(0xffc0_0001));
        let (sig, quiet) = (f32v(0x7f80_0001), f32v(0x7fc0_0001));
        [
            cmp(IeeeComparison::NumericEqual, p0, n0),
            cmp(IeeeComparison::BitIdentical, p0, n0),
            cmp(IeeeComparison::TotalOrder, n0, p0),
            cmp(IeeeComparison::TotalOrder, p0, n0),
            cmp(IeeeComparison::NumericEqual, nan1, nan1),
            cmp(IeeeComparison::NumericEqual, nan2, nan2),
            cmp(IeeeComparison::BitIdentical, nan1, nan1),
            cmp(IeeeComparison::BitIdentical, nan2, nan2),
            cmp(IeeeComparison::TotalOrder, nan1, nan2),
            cmp(IeeeComparison::TotalOrder, nan2, nan1),
            cmp(IeeeComparison::TotalOrder, neg2, neg1),
            cmp(IeeeComparison::TotalOrder, neg1, neg2),
            cmp(IeeeComparison::TotalOrder, sig, quiet),
            cmp(IeeeComparison::TotalOrder, quiet, sig),
        ]
    }};
    assert_eq!(
        results,
        [
            true, false, true, false, false, false, true, true, true, false, true, false, true,
            false
        ]
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f03_f04_f04b_leftmost_nan_is_retained_and_signaling_raises_invalid() {
    let results = agree! {{
        let run = |operation: IeeeOperation, mode| {
            evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap()
        };
        let even = RoundingMode::NearestEven;
        [
            run(IeeeOperation::Add(f32v(0xffc0_0021), f32v(0x7fc0_0012)), even),
            run(IeeeOperation::Add(f32v(0x7f80_0021), f32v(0x7fc0_0012)), even),
            run(IeeeOperation::Add(f32v(0x7fc0_0012), f32v(0xff80_0021)), even),
            run(IeeeOperation::FusedMultiplyAdd(f32v(0x3f80_0000), f32v(0x7fc0_0012), f32v(0xff80_0021)), even),
            run(IeeeOperation::Add(f32v(0x7f80_0021), f32v(0x7fc0_0012)), RoundingMode::Exact),
        ]
    }};
    let invalid = flags(&[IeeeFlag::Invalid]);
    let observed: Vec<_> = results.iter().map(bf).collect();
    assert_eq!(
        observed,
        [
            (0xffc0_0021, IeeeFlags::EMPTY),
            (0x7fc0_0021, invalid),
            (0x7fc0_0012, invalid),
            (0x7fc0_0012, invalid),
            (0x7fc0_0021, invalid),
        ]
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f05_infinities_and_finite_extrema_follow_value_order() {
    const ORDERED: [u32; 8] = [
        0xff80_0000,
        0xff7f_ffff,
        0x8000_0001,
        0x8000_0000,
        0x0000_0000,
        0x0000_0001,
        0x7f7f_ffff,
        0x7f80_0000,
    ];
    let table = agree! {{
        let mut table = Vec::new();
        for left in ORDERED {
            for right in ORDERED {
                let cmp = |c| {
                    compare_ieee(c, f32v(left), f32v(right), &mut Meter::new(UNLIMITED))
                        .unwrap()
                        .completed()
                        .unwrap()
                };
                table.push(IeeeComparison::ALL.map(cmp));
            }
        }
        table.push(IeeeComparison::ALL.map(|c| {
            let inf = f64v(0x7ff0_0000_0000_0000);
            compare_ieee(c, inf, inf, &mut Meter::new(UNLIMITED)).unwrap().completed().unwrap()
        }));
        table
    }};
    for (index, row) in table[..64].iter().enumerate() {
        let (i, j) = (index / 8, index % 8);
        let zeros = matches!((i, j), (3, 4) | (4, 3));
        // `IeeeComparison::ALL` is numeric-equal, total-order, bit-identical.
        assert_eq!(*row, [i == j || zeros, i <= j, i == j], "{i} {j}");
    }
    assert_eq!(table[64], [true, true, true]);
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-1, FR-007-AC-6
#[test]
fn tc_020_f06_f20_cross_width_and_exact_operands_are_ill_typed_before_any_charge() {
    let (comparisons, attempts, converted) = agree! {{
        let (narrow, wide) = (f32v(0x3f80_0000), f64v(0x3ff0_0000_0000_0000));
        let zero = ieee_limits(0, 0, 0, 0);
        let comparisons = IeeeComparison::ALL.map(|c| {
            metered(zero, |m| compare_ieee(c, narrow, wide, m))
        });
        let one = Integer::from(1_i64);
        let decimal = decimal_type(0, 10, 0, 0, RoundingMode::NearestEven);
        let bit = IntegerInterval::new(Integer::from(0_i64), Integer::from(1_i64)).unwrap();
        let attempts = [
            metered(zero, |m| evaluate_ieee(IeeeOperation::Add(narrow, wide), RoundingMode::NearestEven, m).map(|_| ())),
            metered(zero, |m| {
                evaluate_ieee(
                    IeeeOperation::Add(IeeeOperand::Ieee(narrow), IeeeOperand::Exact(ExactScalar::from(&one))),
                    RoundingMode::NearestEven,
                    m,
                )
                .map(|_| ())
            }),
            metered(zero, |m| compare_ieee(IeeeComparison::NumericEqual, narrow, ExactScalar::from(&one), m).map(|_| ())),
            metered(zero, |m| ieee_to_exact(narrow, IeeeExactTarget::Decimal(&decimal), m).map(|_| ())),
            metered(zero, |m| ieee_to_exact(narrow, IeeeExactTarget::BoundedInteger(&bit), m).map(|_| ())),
            metered(zero, |m| ieee_to_exact(narrow, IeeeExactTarget::Integer, m).map(|_| ())),
        ];
        let converted = convert_ieee_width(narrow, IeeeWidth::Binary64, RoundingMode::Exact, &mut Meter::new(UNLIMITED));
        let identical = IeeeComparison::ALL.map(|c| {
            let value = converted.clone().completed().unwrap().value();
            compare_ieee(c, value, wide, &mut Meter::new(UNLIMITED)).unwrap().completed().unwrap()
        });
        (comparisons, attempts, (converted, identical))
    }};
    let ill = |cause| -> Result<(), IllTyped> { Err(IllTyped { cause }) };
    for (outcome, charges, consumed) in &comparisons {
        assert_eq!(
            outcome.clone().map(|_| ()),
            ill(IllTypedCause::DistinctIeeeWidths)
        );
        assert!(charges.is_empty() && consumed.iter().all(|c| *c == 0));
    }
    let causes: Vec<_> = attempts
        .iter()
        .map(|(outcome, _, _)| outcome.clone())
        .collect();
    assert_eq!(
        causes,
        [
            ill(IllTypedCause::DistinctIeeeWidths),
            ill(IllTypedCause::IeeeWithExactOperand),
            ill(IllTypedCause::IeeeWithExactOperand),
            ill(IllTypedCause::IeeeToNonRationalExact),
            ill(IllTypedCause::IeeeToNonRationalExact),
            ill(IllTypedCause::IeeeToNonRationalExact),
        ]
    );
    assert!(attempts
        .iter()
        .all(|(_, charges, consumed)| { charges.is_empty() && consumed.iter().all(|c| *c == 0) }));
    assert_eq!(bf(&converted.0), (0x3ff0_0000_0000_0000, IeeeFlags::EMPTY));
    assert_eq!(converted.1, [true, true, true]);
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f07_f08_f10_each_direction_rounds_once_and_changes_provenance() {
    let tables = agree! {{
        [
            IeeeOperation::Add(f32v(0x3f80_0000), f32v(0x3380_0000)),
            IeeeOperation::Add(f32v(0xbf80_0000), f32v(0xb380_0000)),
            IeeeOperation::Add(f64v(0x3ff0_0000_0000_0000), f64v(0x3ca0_0000_0000_0000)),
        ]
        .map(|operation| {
            [RoundingMode::NearestEven, RoundingMode::NearestAway, RoundingMode::TowardZero, RoundingMode::TowardPositive, RoundingMode::TowardNegative].map(|mode| {
                let result = evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap();
                let provenance = result.clone().completed().map(|r| {
                    (r.provenance().width(), r.provenance().rounding(), r.provenance().definition())
                });
                (result, provenance)
            })
        })
    }};
    let inexact = flags(&[IeeeFlag::Inexact]);
    use RoundingMode::{NearestAway, TowardNegative, TowardPositive};
    let expected: [(u64, u64, [RoundingMode; 2], IeeeWidth); 3] = [
        (
            0x3f80_0000,
            0x3f80_0001,
            [NearestAway, TowardPositive],
            IeeeWidth::Binary32,
        ),
        (
            0xbf80_0000,
            0xbf80_0001,
            [NearestAway, TowardNegative],
            IeeeWidth::Binary32,
        ),
        (
            0x3ff0_0000_0000_0000,
            0x3ff0_0000_0000_0001,
            [NearestAway, TowardPositive],
            IeeeWidth::Binary64,
        ),
    ];
    for (table, (down, up, up_modes, width)) in tables.iter().zip(expected) {
        for ((outcome, provenance), mode) in table.iter().zip(DIRECTIONS) {
            let bits = if up_modes.contains(&mode) { up } else { down };
            assert_eq!(bf(outcome), (bits, inexact), "{mode:?}");
            assert_eq!(*provenance, Some((width, mode, IEEE_DEFINITION)));
        }
    }
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f09_f14_f23_strict_exact_refuses_with_would_be_flags_and_no_bits() {
    let results = agree! {{
        let run = |operation: IeeeOperation, mode| {
            evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap()
        };
        let f09 = IeeeOperation::Add(f32v(0x3f80_0000), f32v(0x3380_0000));
        let f14 = IeeeOperation::Add(f32v(0x7f7f_ffff), f32v(0x7f7f_ffff));
        let f23 = IeeeOperation::Add(f32v(0x7f7f_ffff), f32v(0x7300_0000));
        [
            run(f09, RoundingMode::Exact),
            run(IeeeOperation::Add(f32v(0x3f80_0000), f32v(0x3400_0000)), RoundingMode::Exact),
            run(f14, RoundingMode::NearestEven),
            run(f14, RoundingMode::Exact),
            run(f23, RoundingMode::NearestEven),
            run(f23, RoundingMode::TowardZero),
            run(f23, RoundingMode::Exact),
        ]
    }};
    let overflow = [IeeeFlag::Overflow, IeeeFlag::Inexact];
    assert_eq!(results[0], not_exact(&[IeeeFlag::Inexact]));
    assert_eq!(bf(&results[1]), (0x3f80_0001, IeeeFlags::EMPTY));
    assert_eq!(
        results[1]
            .clone()
            .completed()
            .unwrap()
            .provenance()
            .rounding(),
        RoundingMode::Exact
    );
    assert_eq!(bf(&results[2]), (0x7f80_0000, flags(&overflow)));
    assert_eq!(results[3], not_exact(&overflow));
    assert_eq!(bf(&results[4]), (0x7f80_0000, flags(&overflow)));
    assert_eq!(bf(&results[5]), (0x7f7f_ffff, flags(&[IeeeFlag::Inexact])));
    assert_eq!(results[6], not_exact(&overflow));
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f11_fused_multiply_add_rounds_once_unlike_multiply_then_add() {
    let (fused, product, sum) = agree! {{
        let (a, b, c) = (f32v(0x3f80_0001), f32v(0x3f7f_fffe), f32v(0xbf80_0000));
        let even = RoundingMode::NearestEven;
        let run = |operation: IeeeOperation| evaluate_ieee(operation, even, &mut Meter::new(UNLIMITED)).unwrap();
        let product = run(IeeeOperation::Multiply(a, b));
        let sum = run(IeeeOperation::Add(product.clone().completed().unwrap().value(), c));
        (run(IeeeOperation::FusedMultiplyAdd(a, b, c)), product, sum)
    }};
    assert_eq!(bf(&fused), (0xa880_0000, IeeeFlags::EMPTY));
    assert_eq!(bf(&product).1, flags(&[IeeeFlag::Inexact]));
    assert_eq!(bf(&sum), (0x0000_0000, IeeeFlags::EMPTY));
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f12_f13_invalid_and_divide_by_zero_are_operation_local_under_every_policy() {
    for mode in 0..RoundingMode::ALL.len() {
        let (invalid, divided) = agree! {{
            let mode = RoundingMode::ALL[mode];
            let invalid = [
                IeeeOperation::Multiply(f32v(0x0000_0000), f32v(0x7f80_0000)),
                IeeeOperation::SquareRoot(f32v(0xbf80_0000)),
            ]
            .map(|operation| evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap());
            let divided = [
                (0x3f80_0000, 0x0000_0000),
                (0x3f80_0000, 0x8000_0000),
                (0xbf80_0000, 0x0000_0000),
                (0xbf80_0000, 0x8000_0000),
            ]
            .map(|(dividend, divisor)| {
                let mut meter = Meter::new(UNLIMITED);
                let quotient = evaluate_ieee(IeeeOperation::Divide(f32v(dividend), f32v(divisor)), mode, &mut meter).unwrap();
                let next = evaluate_ieee(
                    IeeeOperation::Add(f32v(0x3f80_0000), f32v(0x3400_0000)),
                    RoundingMode::Exact,
                    &mut meter,
                )
                .unwrap();
                (quotient, next)
            });
            (invalid, divided)
        }};
        for outcome in &invalid {
            assert_eq!(bf(outcome), (0x7fc0_0000, flags(&[IeeeFlag::Invalid])));
        }
        let expected = [0x7f80_0000, 0xff80_0000, 0xff80_0000, 0x7f80_0000];
        for ((quotient, next), bits) in divided.iter().zip(expected) {
            assert_eq!(bf(quotient), (bits, flags(&[IeeeFlag::DivideByZero])));
            assert_eq!(bf(next), (0x3f80_0001, IeeeFlags::EMPTY));
        }
    }
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f15_f16_f29_subnormal_and_extreme_flags() {
    let results = agree! {{
        let run = |operation: IeeeOperation, mode| {
            evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap()
        };
        let (even, exact) = (RoundingMode::NearestEven, RoundingMode::Exact);
        let f15 = IeeeOperation::Multiply(f32v(0x0080_0000), f32v(0x3f00_0000));
        let f16 = IeeeOperation::Multiply(f32v(0x0000_0001), f32v(0x3f00_0000));
        let f29a = IeeeOperation::Add(f32v(0x7f7f_ffff), f32v(0x7280_0000));
        let f29b = IeeeOperation::Multiply(f32v(0x007f_ffff), f32v(0x3f80_0001));
        [
            run(f15, even),
            run(f15, exact),
            run(f16, even),
            run(f29a, even),
            run(f29a, exact),
            run(f29b, even),
            run(f29b, exact),
        ]
    }};
    let inexact = flags(&[IeeeFlag::Inexact]);
    assert_eq!(bf(&results[0]), (0x0040_0000, IeeeFlags::EMPTY));
    assert_eq!(bf(&results[1]), (0x0040_0000, IeeeFlags::EMPTY));
    assert_eq!(
        bf(&results[2]),
        (0, flags(&[IeeeFlag::Underflow, IeeeFlag::Inexact]))
    );
    assert_eq!(bf(&results[3]), (0x7f7f_ffff, inexact));
    assert_eq!(results[4], not_exact(&[IeeeFlag::Inexact]));
    assert_eq!(bf(&results[5]), (0x0080_0000, inexact));
    assert_eq!(results[6], not_exact(&[IeeeFlag::Inexact]));
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_020_f17_f18_classified_paths_charge_only_operands_and_retention() {
    const F17: [u64; 10] = [32, 0, 0, 0, 0, 0, 0, 2, 2, 1];
    let (equal, equal_denials, equal_short, invalid, invalid_denials, no_result) = agree! {{
        let nan_equal = |m: &mut Meter| {
            compare_ieee(IeeeComparison::NumericEqual, f32v(0x7fc0_0001), f32v(0x3f80_0000), m).unwrap()
        };
        let invalid = |m: &mut Meter| {
            evaluate_ieee(IeeeOperation::Multiply(f32v(0x0000_0000), f32v(0x7f80_0000)), RoundingMode::NearestEven, m).unwrap()
        };
        let mut short = F17;
        short[8] = 1;
        let mut no_result = F17;
        no_result[9] = 0;
        (
            metered(limits(F17), nan_equal),
            denials(limits(F17), nan_equal),
            metered(limits(short), nan_equal),
            metered(limits(F17), invalid),
            denials(limits(F17), invalid),
            metered(limits(no_result), invalid),
        )
    }};
    assert_eq!(equal.0, Outcome::Completed(false));
    assert_eq!(equal.1, CLASSIFIED);
    assert_eq!(equal.2, [32, 0, 0, 0, 0, 0, 0, 2, 2, 1]);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(equal_denials) {
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }
    assert_eq!(
        equal_short.0,
        Outcome::Incomplete(work_denied(1, IeeeResultRetain))
    );
    assert_eq!(bf(&invalid.0), (0x7fc0_0000, flags(&[IeeeFlag::Invalid])));
    assert_eq!(invalid.1, CLASSIFIED);
    assert_eq!(invalid_denials[1].2, retain_denied(1));
    assert_eq!(
        no_result.0,
        Outcome::Incomplete(incomplete(
            LimitKind::ResultUnits,
            0,
            0,
            int(1),
            IeeeResultRetain
        ))
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_020_f19_irrational_square_root_charges_the_fixed_width_allowance() {
    let (root, denied) = agree! {{
        let run = |m: &mut Meter| {
            evaluate_ieee(IeeeOperation::SquareRoot(f32v(0x4000_0000)), RoundingMode::NearestEven, m).unwrap()
        };
        let f19 = limits([32, 0, 0, 0, 0, 0, 0, 1, 4, 1]);
        (metered(f19, run), denials(f19, run))
    }};
    assert_eq!(bf(&root.0), (0x3fb5_04f3, flags(&[IeeeFlag::Inexact])));
    assert_eq!(root.1, FINITE);
    assert_eq!(root.2, [32, 0, 0, 0, 0, 0, 0, 1, 4, 1]);
    assert_eq!(denied.len(), 4);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_f10_binary64_limit_tuple_and_its_denials() {
    const F10: [u64; 10] = [64, 0, 0, 0, 0, 0, 0, 2, 4, 1];
    let (exact, denied, short_work, short_bits, sibling) = agree! {{
        let run = |mode| {
            move |m: &mut Meter| {
                evaluate_ieee(
                    IeeeOperation::Add(f64v(0x3ff0_0000_0000_0000), f64v(0x3ca0_0000_0000_0000)),
                    mode,
                    m,
                )
                .unwrap()
            }
        };
        let even = run(RoundingMode::NearestEven);
        let (mut work, mut bits) = (F10, F10);
        work[8] = 3;
        bits[0] = 63;
        (
            metered(limits(F10), even),
            denials(limits(F10), even),
            metered(limits(work), even),
            metered(limits(bits), even),
            metered(UNLIMITED, run(RoundingMode::TowardPositive)),
        )
    }};
    assert_eq!(
        bf(&exact.0),
        (0x3ff0_0000_0000_0000, flags(&[IeeeFlag::Inexact]))
    );
    assert_eq!(exact.1, FINITE);
    assert_eq!(exact.2, [64, 0, 0, 0, 0, 0, 0, 2, 4, 1]);
    assert_eq!(denied[3].2, retain_denied(3));
    assert_eq!(short_work.0, retain_denied(3));
    assert_eq!(
        short_bits.0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            63,
            0,
            int(64),
            IeeeOperands
        ))
    );
    assert_eq!(
        bf(&sibling.0),
        (0x3ff0_0000_0000_0001, flags(&[IeeeFlag::Inexact]))
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_f21_f22_f30_zero_operands_and_strict_exact_charge_positions() {
    let results = agree! {{
        let run = |operation: IeeeOperation, mode, l: ScalarLimits| {
            metered(l, |m| evaluate_ieee(operation, mode, m).unwrap())
        };
        let even = RoundingMode::NearestEven;
        let f09 = IeeeOperation::Add(f32v(0x3f80_0000), f32v(0x3380_0000));
        let zero_sum = IeeeOperation::Add(f32v(0x0000_0000), f32v(0x0000_0000));
        let zero_product = IeeeOperation::Multiply(f32v(0x8000_0000), f32v(0x3f80_0000));
        let root = IeeeOperation::SquareRoot(f32v(0x8000_0000));
        [
            run(zero_sum, even, ieee_limits(32, 2, 4, 1)),
            run(zero_sum, even, ieee_limits(32, 2, 3, 1)),
            run(zero_product, even, ieee_limits(32, 2, 4, 1)),
            run(zero_product, even, ieee_limits(32, 2, 3, 1)),
            run(f09, RoundingMode::Exact, ieee_limits(32, 2, 3, 1)),
            run(f09, even, ieee_limits(32, 2, 3, 1)),
            run(f09, RoundingMode::Exact, ieee_limits(32, 2, 2, 1)),
            run(root, even, ieee_limits(32, 1, 4, 1)),
            run(root, even, ieee_limits(32, 1, 3, 1)),
        ]
    }};
    assert_eq!(
        (bf(&results[0].0), results[0].1.as_slice()),
        ((0, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(results[1].0, retain_denied(3));
    assert_eq!(
        (bf(&results[2].0), results[2].1.as_slice()),
        ((0x8000_0000, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(results[3].0, retain_denied(3));
    assert_eq!(results[4].0, not_exact(&[IeeeFlag::Inexact]));
    assert_eq!(results[4].1, FINITE[..3]);
    assert_eq!(results[5].0, retain_denied(3));
    assert_eq!(results[6].0, Outcome::Incomplete(work_denied(2, IeeeRound)));
    assert_eq!(
        (bf(&results[7].0), results[7].1.as_slice()),
        ((0x8000_0000, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(results[8].0, retain_denied(3));
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_f24_f28_conversions_charge_at_their_stated_widths_and_positions() {
    let (widths, thirds, halves, decimals, largest) = agree! {{
        let even = RoundingMode::NearestEven;
        let widths = [
            metered(ieee_limits(64, 1, 4, 1), |m| convert_ieee_width(f32v(0x3f80_0000), IeeeWidth::Binary64, even, m)),
            metered(ieee_limits(32, 1, 4, 1), |m| convert_ieee_width(f32v(0x3f80_0000), IeeeWidth::Binary64, even, m)),
            metered(ieee_limits(32, 1, 4, 1), |m| convert_ieee_width(f64v(0x3ff0_0000_0000_0000), IeeeWidth::Binary32, even, m)),
        ];
        let third = ratio(1, 3);
        let thirds = [
            metered(ieee_limits(32, 1, 4, 1), |m| exact_to_ieee(&third, IeeeWidth::Binary32, even, m)),
            metered(ieee_limits(2, 1, 4, 1), |m| exact_to_ieee(&third, IeeeWidth::Binary32, even, m)),
        ];
        let unit_halves = rational_type("0", "1", "1", "2");
        let halves = (
            metered(ieee_limits(32, 1, 3, 1), |m| {
                ieee_to_exact(f32v(0x3f00_0000), IeeeExactTarget::Rational(&unit_halves), m).unwrap()
            }),
            metered(ieee_limits(32, 1, 2, 1), |m| {
                ieee_to_exact(f32v(0x7fc0_0000), IeeeExactTarget::Rational(&unit_halves), m).unwrap()
            }),
        );
        let hundred = dec(100, 2);
        let decimals = [
            metered(ieee_limits(32, 1, 4, 1), |m| exact_to_ieee(&hundred, IeeeWidth::Binary32, even, m)),
            metered(ieee_limits(6, 1, 4, 1), |m| exact_to_ieee(&hundred, IeeeWidth::Binary32, even, m)),
        ];
        let source = Decimal::new(Integer::from(1_i64), u32::MAX);
        let largest = metered(ieee_limits(64, 1, 4, 1), |m| exact_to_ieee(&source, IeeeWidth::Binary64, even, m));
        (widths, thirds, halves, decimals, largest)
    }};
    let bits = LimitKind::IntegerBits;
    assert_eq!(
        (bf(&widths[0].0), widths[0].1.as_slice()),
        ((0x3ff0_0000_0000_0000, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(
        widths[1].0,
        Outcome::Incomplete(incomplete(bits, 32, 32, int(64), IeeeExactIntermediate))
    );
    assert_eq!(
        widths[2].0,
        Outcome::Incomplete(incomplete(bits, 32, 0, int(64), IeeeOperands))
    );
    assert_eq!(
        (bf(&thirds[0].0), thirds[0].1.as_slice()),
        ((0x3eaa_aaab, flags(&[IeeeFlag::Inexact])), &FINITE[..])
    );
    assert_eq!(
        thirds[1].0,
        Outcome::Incomplete(incomplete(bits, 2, 2, int(32), IeeeExactIntermediate))
    );
    let half = halves.0 .0.clone().completed().unwrap();
    assert_eq!(
        (half.value(), half.value().max_part_bits()),
        (&ratio(1, 2), 2)
    );
    assert_eq!(halves.0 .1, TO_EXACT);
    assert_eq!(halves.1 .0, Outcome::Undefined(Undefined::IeeeNotFinite));
    assert_eq!(halves.1 .1, [IeeeOperands]);
    assert_eq!(
        (bf(&decimals[0].0), decimals[0].1.as_slice()),
        ((0x3f80_0000, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(
        decimals[1].0,
        Outcome::Incomplete(incomplete(bits, 6, 0, int(7), IeeeOperands))
    );
    assert_eq!(
        largest.0,
        Outcome::Incomplete(incomplete(bits, 64, 0, int(14_267_572_524), IeeeOperands))
    );
    assert!(largest.1.is_empty());
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-2, FR-007-AC-6
#[test]
fn tc_020_f25_nan_width_conversion_keeps_sign_and_payload_or_refuses() {
    let results = agree! {{
        let even = RoundingMode::NearestEven;
        let convert = |value: IeeeValue, target, l| metered(l, |m| convert_ieee_width(value, target, even, m));
        [
            convert(f64v(0x7ff0_0000_0000_0001), IeeeWidth::Binary32, ieee_limits(64, 1, 2, 1)),
            convert(f64v(0xfff8_0000_0000_0003), IeeeWidth::Binary32, ieee_limits(64, 1, 2, 1)),
            convert(f32v(0x7fc0_0001), IeeeWidth::Binary64, ieee_limits(32, 1, 2, 1)),
            convert(f64v(0x7ff8_0000_0040_0000), IeeeWidth::Binary32, ieee_limits(64, 1, 1, 0)),
            convert(f64v(0x7ff8_0000_0040_0000), IeeeWidth::Binary32, ieee_limits(64, 1, 0, 0)),
            convert(f64v(0x7ff0_0000_0040_0000), IeeeWidth::Binary32, ieee_limits(64, 1, 1, 0)),
        ]
    }};
    let expected = [
        (
            0x7fc0_0001,
            flags(&[IeeeFlag::Invalid]),
            IeeeWidth::Binary32,
        ),
        (0xffc0_0003, IeeeFlags::EMPTY, IeeeWidth::Binary32),
        (0x7ff8_0000_0000_0001, IeeeFlags::EMPTY, IeeeWidth::Binary64),
    ];
    for ((outcome, charges, _), (bits, raised, width)) in results[..3].iter().zip(expected) {
        assert_eq!(bf(outcome), (bits, raised));
        assert_eq!(outcome.clone().completed().unwrap().value().width(), width);
        assert_eq!(*charges, CLASSIFIED);
    }
    let refused = Outcome::Refused(Refusal::IeeeNanPayloadNotRepresentable);
    assert_eq!(
        (&results[3].0, results[3].1.as_slice()),
        (&refused, &[IeeeOperands][..])
    );
    assert_eq!(
        results[4].0,
        Outcome::Incomplete(work_denied(0, IeeeOperands))
    );
    assert_eq!(
        (&results[5].0, results[5].1.as_slice()),
        (&refused, &[IeeeOperands][..])
    );
    assert_eq!(
        Refusal::IeeeNanPayloadNotRepresentable.code(),
        Some("ieee_nan_payload_not_representable")
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_f26_zero_signs_survive_width_conversion_sums_and_differences() {
    let (widened, table) = agree! {{
        let even = RoundingMode::NearestEven;
        let widened = [
            metered(ieee_limits(64, 1, 4, 1), |m| convert_ieee_width(f32v(0x8000_0000), IeeeWidth::Binary64, even, m)),
            metered(ieee_limits(64, 1, 3, 1), |m| convert_ieee_width(f32v(0x8000_0000), IeeeWidth::Binary64, even, m)),
        ];
        let (nz, pz, one, minus_one) = (f32v(0x8000_0000), f32v(0), f32v(0x3f80_0000), f32v(0xbf80_0000));
        let table = RoundingMode::ALL.map(|mode| {
            [
                IeeeOperation::Add(nz, nz),
                IeeeOperation::Add(one, minus_one),
                IeeeOperation::Subtract(nz, pz),
                IeeeOperation::Subtract(pz, pz),
                IeeeOperation::Subtract(one, one),
            ]
            .map(|operation| evaluate_ieee(operation, mode, &mut Meter::new(UNLIMITED)).unwrap())
        });
        (widened, table)
    }};
    assert_eq!(
        (bf(&widened[0].0), widened[0].1.as_slice()),
        ((0x8000_0000_0000_0000, IeeeFlags::EMPTY), &FINITE[..])
    );
    assert_eq!(widened[1].0, retain_denied(3));
    for (row, mode) in table.iter().zip(RoundingMode::ALL) {
        let cancelled = if mode == RoundingMode::TowardNegative {
            0x8000_0000
        } else {
            0
        };
        let expected = [0x8000_0000, cancelled, 0x8000_0000, cancelled, cancelled];
        let observed: Vec<_> = row.iter().map(bf).collect();
        let expected: Vec<_> = expected
            .iter()
            .map(|bits| (*bits, IeeeFlags::EMPTY))
            .collect();
        assert_eq!(observed, expected, "{mode:?}");
    }
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_fused_multiply_add_exact_zero_takes_the_sum_sign_rule() {
    let table = agree! {{
        let (nz, pz, one, minus_one) = (f32v(0x8000_0000), f32v(0), f32v(0x3f80_0000), f32v(0xbf80_0000));
        RoundingMode::ALL.map(|mode| {
            [(nz, one, nz), (nz, minus_one, pz), (pz, minus_one, nz), (nz, one, pz), (pz, one, nz), (one, one, minus_one), (minus_one, one, one)]
                .map(|(x, y, z)| {
                    evaluate_ieee(IeeeOperation::FusedMultiplyAdd(x, y, z), mode, &mut Meter::new(UNLIMITED)).unwrap()
                })
        })
    }};
    for (row, mode) in table.iter().zip(RoundingMode::ALL) {
        let opposite = if mode == RoundingMode::TowardNegative {
            0x8000_0000
        } else {
            0
        };
        let expected = [
            0x8000_0000,
            0,
            0x8000_0000,
            opposite,
            opposite,
            opposite,
            opposite,
        ];
        let observed: Vec<_> = row.iter().map(|o| bf(o).0).collect();
        assert_eq!(observed, expected, "{mode:?}");
    }
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-2, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_f27_ieee_to_rational_sizes_maxparts_and_admits_membership_before_retention() {
    let smallest = support::pow2(1074);
    let results = agree! {{
        let convert = |value: IeeeValue, domain: &RationalDomain, l| {
            metered(l, |m| ieee_to_exact(value, IeeeExactTarget::Rational(domain), m).unwrap())
        };
        let tiny = rational_type("0", "1", "1", &smallest);
        let zero = rational_type("0", "0", "1", "1");
        let halves = rational_type("0", "1", "1", "2");
        let integers = rational_type("0", "1", "1", "1");
        [
            convert(f64v(1), &tiny, ieee_limits(1075, 1, 3, 1)),
            convert(f64v(1), &tiny, ieee_limits(1074, 1, 3, 1)),
            convert(f32v(0x8000_0000), &zero, ieee_limits(32, 1, 3, 1)),
            convert(f32v(0x8000_0000), &zero, ieee_limits(32, 1, 2, 1)),
            convert(f32v(0x7f80_0000), &halves, ieee_limits(32, 1, 1, 0)),
            convert(f32v(0x7f80_0000), &halves, ieee_limits(32, 1, 0, 0)),
            convert(f32v(0x3f00_0000), &integers, ieee_limits(32, 1, 2, 0)),
            convert(f32v(0x3f00_0000), &integers, ieee_limits(32, 1, 1, 0)),
        ]
    }};
    let tiny = results[0].0.clone().completed().unwrap();
    assert_eq!(
        tiny.value(),
        &Rational::new(int(1), big(&smallest)).unwrap()
    );
    assert_eq!(results[0].1, TO_EXACT);
    assert_eq!(
        results[1].0,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            1074,
            64,
            int(1075),
            IeeeExactIntermediate
        ))
    );
    let zero = results[2].0.clone().completed().unwrap();
    assert_eq!(
        (zero.value(), zero.loss()),
        (&ratio(0, 1), Some(IeeeExactLoss::NegativeZeroSign))
    );
    assert_eq!(results[2].1, TO_EXACT);
    assert_eq!(
        results[3].0,
        Outcome::Incomplete(work_denied(2, IeeeResultRetain))
    );
    assert_eq!(
        (&results[4].0, results[4].1.as_slice()),
        (
            &Outcome::Undefined(Undefined::IeeeNotFinite),
            &[IeeeOperands][..]
        )
    );
    assert_eq!(
        results[5].0,
        Outcome::Incomplete(work_denied(0, IeeeOperands))
    );
    assert_eq!(
        (&results[6].0, results[6].1.as_slice()),
        (
            &Outcome::Refused(Refusal::IeeeRationalOutOfDomain),
            &TO_EXACT[..2]
        )
    );
    assert_eq!(
        results[7].0,
        Outcome::Incomplete(work_denied(1, IeeeExactIntermediate))
    );
    assert_eq!(
        Refusal::IeeeRationalOutOfDomain.code(),
        Some("ieee_rational_out_of_domain")
    );
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_f31_narrowing_conversion_rounds_overflows_and_underflows_once() {
    let results = agree! {{
        let narrow = |bits: u64, mode, l| {
            metered(l, |m| convert_ieee_width(f64v(bits), IeeeWidth::Binary32, mode, m))
        };
        let (even, exact) = (RoundingMode::NearestEven, RoundingMode::Exact);
        let (tenth, halfway, smallest) = (0x3fb9_9999_9999_999a, 0x47ef_ffff_f000_0000, 1);
        [
            narrow(0xfff0_0000_0000_0000, even, ieee_limits(64, 1, 2, 1)),
            narrow(0xfff0_0000_0000_0000, even, ieee_limits(64, 1, 1, 1)),
            narrow(tenth, even, ieee_limits(64, 1, 4, 1)),
            narrow(tenth, exact, ieee_limits(64, 1, 3, 1)),
            narrow(tenth, exact, ieee_limits(64, 1, 2, 1)),
            narrow(0x47ef_ffff_e000_0000, even, ieee_limits(64, 1, 4, 1)),
            narrow(halfway, even, ieee_limits(64, 1, 4, 1)),
            narrow(halfway, even, ieee_limits(64, 1, 3, 1)),
            narrow(halfway, RoundingMode::TowardZero, ieee_limits(64, 1, 4, 1)),
            narrow(smallest, even, ieee_limits(64, 1, 4, 1)),
            narrow(smallest, even, ieee_limits(64, 1, 3, 1)),
        ]
    }};
    let inexact = flags(&[IeeeFlag::Inexact]);
    assert_eq!(
        (bf(&results[0].0), results[0].1.as_slice()),
        ((0xff80_0000, IeeeFlags::EMPTY), &CLASSIFIED[..])
    );
    assert_eq!(results[1].0, retain_denied(1));
    assert_eq!(
        (bf(&results[2].0), results[2].1.as_slice()),
        ((0x3dcc_cccd, inexact), &FINITE[..])
    );
    assert_eq!(
        (&results[3].0, results[3].1.as_slice()),
        (&not_exact(&[IeeeFlag::Inexact]), &FINITE[..3])
    );
    assert_eq!(results[4].0, Outcome::Incomplete(work_denied(2, IeeeRound)));
    assert_eq!(bf(&results[5].0), (0x7f7f_ffff, IeeeFlags::EMPTY));
    assert_eq!(
        bf(&results[6].0),
        (0x7f80_0000, flags(&[IeeeFlag::Overflow, IeeeFlag::Inexact]))
    );
    assert_eq!(results[7].0, retain_denied(3));
    assert_eq!(bf(&results[8].0), (0x7f7f_ffff, inexact));
    assert_eq!(
        bf(&results[9].0),
        (0, flags(&[IeeeFlag::Underflow, IeeeFlag::Inexact]))
    );
    assert_eq!(results[10].0, retain_denied(3));
}

/// Trace: TC-020, FR-007-AC-3, FR-007-AC-6
#[test]
fn tc_020_i13_negotiation_is_per_item() {
    let dispositions = agree! {{
        let full = IeeeBackendCapabilities {
            widths: IeeeWidth::ALL.into_iter().collect(),
            operations: IeeeOperationKind::ALL.into_iter().collect(),
            roundings: RoundingMode::ALL.into_iter().collect(),
            exceptional_policy: true,
            finite_proof: true,
        };
        let item = |width, operation, rounding, requires_finite_proof| IeeeItemRequirement {
            width,
            operation,
            rounding,
            requires_finite_proof,
        };
        let items = [
            item(IeeeWidth::Binary64, IeeeOperationKind::Add, RoundingMode::NearestEven, true),
            item(IeeeWidth::Binary32, IeeeOperationKind::FusedMultiplyAdd, RoundingMode::TowardNegative, false),
            item(IeeeWidth::Binary32, IeeeOperationKind::TotalOrder, RoundingMode::TowardNegative, false),
        ];
        let backends = [
            full.clone(),
            IeeeBackendCapabilities { widths: [IeeeWidth::Binary32].into_iter().collect(), ..full.clone() },
            IeeeBackendCapabilities {
                operations: IeeeOperationKind::ALL.into_iter().filter(|k| *k != IeeeOperationKind::FusedMultiplyAdd).collect(),
                ..full.clone()
            },
            IeeeBackendCapabilities {
                roundings: [RoundingMode::NearestEven, RoundingMode::Exact].into_iter().collect(),
                ..full.clone()
            },
            IeeeBackendCapabilities { exceptional_policy: false, ..full.clone() },
            IeeeBackendCapabilities { finite_proof: false, ..full.clone() },
        ];
        backends.map(|backend| negotiate_ieee(&items, &backend).to_vec())
    }};
    use IeeeDisposition::{RequiresBound, Supported, Unsupported};
    assert_eq!(
        dispositions,
        [
            vec![Supported; 3],
            vec![
                Unsupported(IeeeUnsupportedCause::Width(IeeeWidth::Binary64)),
                Supported,
                Supported
            ],
            vec![
                Supported,
                Unsupported(IeeeUnsupportedCause::Operation(
                    IeeeOperationKind::FusedMultiplyAdd
                )),
                Supported
            ],
            vec![
                Supported,
                Unsupported(IeeeUnsupportedCause::Rounding(RoundingMode::TowardNegative)),
                Supported
            ],
            vec![Unsupported(IeeeUnsupportedCause::ExceptionalPolicy); 3],
            vec![RequiresBound, Supported, Supported],
        ]
    );
}

// ---- generated class matrix ----------------------------------------------------------

/// Signs, zeros, subnormal and normal extrema, an odd significand, the
/// infinities and signaling/quiet NaNs of both signs, as raw patterns of
/// `IeeeWidth::ALL[width]`.
fn classes(width: usize) -> Vec<u64> {
    let (exponent_bits, fraction_bits): (u32, u32) = if width == 0 { (8, 23) } else { (11, 52) };
    let sign = 1_u64 << (exponent_bits + fraction_bits);
    let exponent = ((1_u64 << exponent_bits) - 1) << fraction_bits;
    let fraction = (1_u64 << fraction_bits) - 1;
    let quiet = 1_u64 << (fraction_bits - 1);
    let one = ((1_u64 << (exponent_bits - 1)) - 1) << fraction_bits;
    let three = one + (1 << fraction_bits) + quiet;
    let max = exponent - (1 << fraction_bits) + fraction;
    [
        0,
        1,
        fraction,
        1 << fraction_bits,
        max,
        one,
        three,
        exponent,
    ]
    .into_iter()
    .flat_map(|bits| [bits, bits | sign])
    .chain([
        exponent | 1,
        sign | exponent | 2,
        exponent | quiet | 3,
        sign | exponent | quiet,
    ])
    .collect()
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_020_generated_class_matrix_and_every_denial_agree() {
    let mut vectors = 0_usize;
    for width in 0..2_usize {
        let classes = classes(width);
        for mode in 0..RoundingMode::ALL.len() {
            for a in &classes {
                for b in &classes {
                    let a = *a;
                    let b = *b;
                    let _ = agree! {{
                        let mode = RoundingMode::ALL[mode];
                        let (x, y) = (ieee(width, a), ieee(width, b));
                        let binary = [
                            IeeeOperation::Add(x, y),
                            IeeeOperation::Subtract(x, y),
                            IeeeOperation::Multiply(x, y),
                            IeeeOperation::Divide(x, y),
                        ];
                        let mut all = Vec::new();
                        for operation in binary.into_iter().chain([IeeeOperation::SquareRoot(x)]) {
                            let run = |m: &mut Meter| evaluate_ieee(operation, mode, m).unwrap();
                            all.push((metered(UNLIMITED, run), denials(UNLIMITED, run)));
                        }
                        let comparisons = IeeeComparison::ALL.map(|c| {
                            let run = |m: &mut Meter| compare_ieee(c, x, y, m).unwrap();
                            (metered(UNLIMITED, run), denials(UNLIMITED, run))
                        });
                        let converted = IeeeWidth::ALL.map(|target| {
                            let run = |m: &mut Meter| convert_ieee_width(x, target, mode, m);
                            (metered(UNLIMITED, run), denials(UNLIMITED, run))
                        });
                        (all, comparisons, converted)
                    }};
                    vectors += 5 + 3 + 2;
                    let deny_fma = mode == 0;
                    let fma = agree! {{
                        let mode = RoundingMode::ALL[mode];
                        let (x, y) = (ieee(width, a), ieee(width, b));
                        let mut all = Vec::new();
                        for c in &classes {
                            let run = |m: &mut Meter| {
                                evaluate_ieee(IeeeOperation::FusedMultiplyAdd(x, y, ieee(width, *c)), mode, m).unwrap()
                            };
                            let denied = if deny_fma { denials(UNLIMITED, run) } else { Vec::new() };
                            all.push((metered(UNLIMITED, run), denied));
                        }
                        all
                    }};
                    vectors += fma.len();
                }
            }
        }
    }
    // 2 widths × 6 policies × 20² pairs × (10 + 20 FMA) vectors.
    assert_eq!(vectors, 2 * 6 * 400 * 30);
}

/// Trace: TC-020, FR-007-AC-3, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_020_generated_finite_classes_round_trip_through_rational_and_every_denial_agrees() {
    let (numerator, denominator) = (support::pow2(1024), support::pow2(1074));
    let mut finite = 0_usize;
    for width in 0..2_usize {
        for bits in classes(width) {
            let (to_exact, back) = agree! {{
                let every = rational_type(&format!("-{numerator}"), &numerator, "1", &denominator);
                let value = ieee(width, bits);
                let run = |m: &mut Meter| ieee_to_exact(value, IeeeExactTarget::Rational(&every), m).unwrap();
                let to_exact = (metered(UNLIMITED, run), denials(UNLIMITED, run));
                let back = to_exact.0 .0.clone().completed().map(|exact| {
                    let target = IeeeWidth::ALL[width];
                    let run = |mode| {
                        let source = exact.value().clone();
                        move |m: &mut Meter| exact_to_ieee(&source, target, mode, m)
                    };
                    (
                        metered(UNLIMITED, run(RoundingMode::Exact)),
                        denials(UNLIMITED, run(RoundingMode::NearestEven)),
                        exact.loss().is_some(),
                    )
                });
                (to_exact, back)
            }};
            match (&to_exact.0 .0, back) {
                (Outcome::Completed(_), Some((round_trip, _, discarded))) => {
                    finite += 1;
                    assert_eq!(to_exact.0 .1, TO_EXACT);
                    let expected = if discarded { 0 } else { bits };
                    assert_eq!(bf(&round_trip.0), (expected, IeeeFlags::EMPTY), "{bits:#x}");
                    assert_eq!(round_trip.1, FINITE);
                }
                (Outcome::Undefined(Undefined::IeeeNotFinite), None) => {}
                (other, _) => panic!("{bits:#x}: {other:?}"),
            }
        }
    }
    // Fourteen finite patterns (seven of each sign) per width.
    assert_eq!(finite, 28);
}
