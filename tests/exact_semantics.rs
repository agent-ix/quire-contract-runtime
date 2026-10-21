//! FR-007 exact rounding, division and Euclidean `mod` through the public kernel API.
//!
//! IEEE exceptional, rational membership, and decimal normalization cases are kernel-owned.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{
    divide, evaluate_decimal, modulo, Decimal, DecimalOperation, DecimalType, DivisionMember,
    DivisionProfile, InexactTarget, Integer, IntegerDomain, IntegerInterval, Meter, Outcome,
    Rational, Refusal, RoundingMode, ScalarLimits,
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
