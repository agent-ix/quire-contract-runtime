// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-192 integer division profiles: runtime versus authority.
//!
//! Admission-only vectors are compiler work and are not evaluated here:
//! DIV-03 (a `mod` closure claiming a non-Euclidean law) and DIV-09 (missing,
//! conflicting or stale division definitions) are refused by definition-lock
//! admission before any runtime operator exists.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 12] = [
    "signed-table",
    "DIV-01",
    "DIV-02",
    "DIV-04",
    "DIV-05",
    "DIV-06",
    "DIV-07",
    "DIV-08",
    "DIV-10",
    "DIV-11",
    "DIV-12",
    "DIV-13",
];
/// Compiler-owned admission vectors.
const ADMISSION_ONLY: [&str; 2] = ["DIV-03", "DIV-09"];

/// Trace: TC-018, FR-007-AC-6
#[test]
fn tc_018_every_tc192_vector_is_evaluated_or_admission_only() {
    let mut all: Vec<&str> = EVALUATED.iter().chain(&ADMISSION_ONLY).copied().collect();
    all.sort_unstable();
    let mut expected = vec!["signed-table"];
    let ids: Vec<String> = (1..=13).map(|n| format!("DIV-{n:02}")).collect();
    expected.extend(ids.iter().map(String::as_str));
    expected.sort_unstable();
    assert_eq!(all, expected);
    println!(
        "TC-192 agreement: {} evaluated, {} admission-only",
        EVALUATED.len(),
        ADMISSION_ONLY.len()
    );
}

const OPERANDS: [(i128, i128); 4] = [(7, 3), (7, -3), (-7, 3), (-7, -3)];

/// Trace: TC-018, FR-007-AC-1, FR-007-AC-6
#[test]
fn tc_018_signed_table() {
    let table = [
        (
            DivisionProfile::Truncating,
            [(2, 1), (-2, 1), (-2, -1), (2, -1)],
        ),
        (DivisionProfile::Floor, [(2, 1), (-3, -2), (-3, 2), (2, -1)]),
        (
            DivisionProfile::Euclidean,
            [(2, 1), (-2, 1), (-3, 2), (3, 2)],
        ),
    ];
    for (p, (profile, cells)) in table.into_iter().enumerate() {
        assert_eq!(DivisionProfile::ALL[p], profile);
        for ((a, b), (q, r)) in OPERANDS.into_iter().zip(cells) {
            let outcome = agree! {
                divide(DivisionProfile::ALL[p], &int(a), &int(b), &IntegerDomain::Mathematical, &mut Meter::new(UNLIMITED))
            };
            let pair = outcome.completed().unwrap();
            assert_eq!(
                (pair.quotient(), pair.remainder()),
                (&int(q), &int(r)),
                "{profile:?} ({a},{b})"
            );
            assert_eq!(a, b * q + r);
            assert!(r.abs() < b.abs());
            match profile {
                DivisionProfile::Truncating => assert!(r == 0 || (r < 0) == (a < 0)),
                DivisionProfile::Floor => assert!(r == 0 || (r < 0) == (b < 0)),
                DivisionProfile::Euclidean => assert!(r >= 0),
            }
        }
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-1, FR-007-AC-6
#[test]
fn tc_018_div_01_zero_divisors_are_undefined() {
    for p in 0..3_usize {
        for a in [-7, 0, 7] {
            let (division, modulus) = agree! {(
                metered(UNLIMITED, |m| divide(DivisionProfile::ALL[p], &int(a), &int(0), &IntegerDomain::Mathematical, m)),
                metered(UNLIMITED, |m| modulo(&int(a), &int(0), &IntegerDomain::Mathematical, m)),
            )};
            assert_eq!(division.0, Outcome::Undefined(Undefined::DivisionByZero));
            assert_eq!(modulus.0, Outcome::Undefined(Undefined::DivisionByZero));
            assert_eq!(division.2[9], 0);
        }
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-007-AC-6
#[test]
fn tc_018_div_02_mod_is_euclidean_under_every_selected_law() {
    for ((a, b), expected) in [((-7, 3), 2), ((7, -3), 1), ((-7, -3), 2), ((7, 3), 1)] {
        let outcome = agree! {
            modulo(&int(a), &int(b), &IntegerDomain::Mathematical, &mut Meter::new(UNLIMITED))
        };
        assert_eq!(outcome.completed(), Some(int(expected)));
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_018_div_04_div_05_div_06_mathematical_and_signed_64_domains() {
    let (min, max) = (i128::from(i64::MIN), i128::from(i64::MAX));
    for p in 0..3_usize {
        let outcomes = agree! {{
            let signed = IntegerDomain::Bounded(IntegerInterval::signed_twos_complement(
                std::num::NonZeroU32::new(64).unwrap(),
            ));
            let run = |a: i128, b: i128, domain: &IntegerDomain| {
                metered(UNLIMITED, |m| divide(DivisionProfile::ALL[p], &int(a), &int(b), domain, m))
            };
            [
                run(min, -1, &IntegerDomain::Mathematical),
                run(min, -1, &signed),
                run(min, 1, &signed),
                run(max, 1, &signed),
                run(min - 1, 1, &signed),
                run(max + 1, 1, &signed),
            ]
        }};
        let value = |i: usize| {
            outcomes[i]
                .0
                .clone()
                .completed()
                .map(|p| (p.quotient().clone(), p.remainder().clone()))
        };
        assert_eq!(value(0), Some((int(9_223_372_036_854_775_808), int(0))));
        let refused = Outcome::Refused(Refusal::DivisionPairOutOfDomain {
            quotient_admitted: false,
            remainder_admitted: true,
        });
        assert_eq!(outcomes[1].0, refused);
        assert_eq!(outcomes[1].2[9], 0, "no result unit for a refused pair");
        assert_eq!(value(2), Some((int(min), int(0))));
        assert_eq!(value(3), Some((int(max), int(0))));
        assert_eq!(outcomes[4].0, refused);
        assert_eq!(outcomes[5].0, refused);
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-007-AC-6
#[test]
fn tc_018_div_07_finite_consumers_require_every_bound() {
    let dispositions = agree! {{
        let signed = || Some(IntegerInterval::signed_twos_complement(std::num::NonZeroU32::new(64).unwrap()));
        let complete = IntegerDivisionBounds { operand: signed(), intermediate: signed(), result: signed() };
        let without = |missing: fn(&mut IntegerDivisionBounds)| {
            let mut bounds = complete.clone();
            missing(&mut bounds);
            IntegerDivisionConsumer::Finite(bounds)
        };
        negotiate_integer_division(&[
            without(|b| b.operand = None),
            without(|b| b.intermediate = None),
            without(|b| b.result = None),
            IntegerDivisionConsumer::Finite(complete.clone()),
            IntegerDivisionConsumer::Mathematical,
        ])
    }};
    use IntegerDivisionDisposition::{RequiresBound, Supported};
    assert_eq!(
        dispositions,
        [
            RequiresBound,
            RequiresBound,
            RequiresBound,
            Supported,
            Supported
        ]
    );
}

const DIV_08: [u64; 10] = [3, 0, 0, 0, 0, 0, 0, 2, 4, 2];
const DIV_10: [u64; 10] = [3, 0, 0, 0, 0, 0, 0, 2, 4, 1];

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_018_div_08_exact_tuple_and_named_denials() {
    let (exact, short_work, short_result, denied) = agree! {{
        let run = |m: &mut Meter| divide(DivisionProfile::Truncating, &int(7), &int(3), &IntegerDomain::Mathematical, m);
        let mut work = DIV_08;
        work[8] = 3;
        let mut result = DIV_08;
        result[9] = 1;
        (metered(limits(DIV_08), run), metered(limits(work), run), metered(limits(result), run), denials(limits(DIV_08), run))
    }};
    let pair = exact.0.completed().unwrap();
    assert_eq!((pair.quotient(), pair.remainder()), (&int(2), &int(1)));
    use ChargePoint::*;
    let charges = [
        IntegerDivisionOperands,
        IntegerDivisionArithmetic,
        IntegerDivisionDomainPair,
        IntegerDivisionResultPair,
    ];
    assert_eq!(exact.1, charges);
    assert_eq!(exact.2, [3, 0, 0, 0, 0, 0, 0, 2, 4, 2]);
    assert_eq!(
        short_work.0,
        Outcome::Incomplete(work_denied(3, IntegerDivisionResultPair))
    );
    assert_eq!(
        short_result.0,
        Outcome::Incomplete(incomplete(
            LimitKind::ResultUnits,
            1,
            0,
            int(2),
            IntegerDivisionResultPair
        ))
    );
    assert_eq!(denied.len(), 4);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-3, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_018_div_10_mod_charges_only_integer_modulus_points() {
    let (exact, denied) = agree! {{
        let run = |m: &mut Meter| modulo(&int(-7), &int(3), &IntegerDomain::Mathematical, m);
        (metered(limits(DIV_10), run), denials(limits(DIV_10), run))
    }};
    assert_eq!(exact.0.completed(), Some(int(2)));
    use ChargePoint::*;
    let points = [
        IntegerModulusOperands,
        IntegerModulusArithmetic,
        IntegerModulusDomain,
        IntegerModulusResultRetain,
    ];
    assert_eq!(exact.1, points);
    for (work, (point, _, outcome, results)) in (0_u64..).zip(denied) {
        assert_eq!(point, points[work as usize]);
        assert_eq!(outcome, Outcome::Incomplete(work_denied(work, point)));
        assert_eq!(results, 0);
    }
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_018_div_11_zero_divisors_after_the_operands_charge() {
    let runs = agree! {{
        let with = |base: [u64; 10], work: u64| { let mut v = base; v[8] = work; limits(v) };
        let div = |m: &mut Meter| divide(DivisionProfile::Truncating, &int(7), &int(0), &IntegerDomain::Mathematical, m);
        let rem = |m: &mut Meter| modulo(&int(7), &int(0), &IntegerDomain::Mathematical, m);
        (
            [metered(with(DIV_08, 1), div), metered(with(DIV_08, 0), div)],
            [metered(with(DIV_10, 1), rem), metered(with(DIV_10, 0), rem)],
        )
    }};
    let (div, rem) = runs;
    let undefined = Outcome::Undefined(Undefined::DivisionByZero);
    assert_eq!(div[0].0, undefined);
    assert_eq!(div[0].1, [ChargePoint::IntegerDivisionOperands]);
    assert_eq!(rem[0].0, Outcome::Undefined(Undefined::DivisionByZero));
    assert_eq!(rem[0].1, [ChargePoint::IntegerModulusOperands]);
    assert_eq!(
        div[1].0,
        Outcome::Incomplete(work_denied(0, ChargePoint::IntegerDivisionOperands))
    );
    assert_eq!(
        rem[1].0,
        Outcome::Incomplete(work_denied(0, ChargePoint::IntegerModulusOperands))
    );
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_018_div_12_mod_domain_refusal_precedes_retention() {
    let (refused, short) = agree! {{
        let unit = IntegerDomain::Bounded(IntegerInterval::new(int(0), int(1)).unwrap());
        let run = |m: &mut Meter| modulo(&int(-7), &int(3), &unit, m);
        let mut base = DIV_10;
        base[9] = 0;
        let mut two = base;
        two[8] = 2;
        (metered(limits(base), run), metered(limits(two), run))
    }};
    use ChargePoint::*;
    assert_eq!(refused.0, Outcome::Refused(Refusal::ModuloOutOfDomain));
    assert_eq!(
        refused.1,
        [
            IntegerModulusOperands,
            IntegerModulusArithmetic,
            IntegerModulusDomain
        ]
    );
    assert_eq!(
        short.0,
        Outcome::Incomplete(work_denied(2, IntegerModulusDomain))
    );
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-3, FR-007-AC-6
#[test]
fn tc_018_div_13_first_short_counter_in_field_order() {
    let outcome = agree! {{
        let mut v = DIV_08;
        v[0] = 2;
        v[8] = 0;
        divide(DivisionProfile::Truncating, &int(7), &int(3), &IntegerDomain::Mathematical, &mut Meter::new(limits(v)))
    }};
    assert_eq!(
        outcome,
        Outcome::Incomplete(incomplete(
            LimitKind::IntegerBits,
            2,
            0,
            int(3),
            ChargePoint::IntegerDivisionOperands
        ))
    );
}

/// Trace: TC-018, FR-007-AC-1, FR-006-AC-4, FR-007-AC-6
#[test]
fn tc_018_generated_pairs_domains_and_denials_agree() {
    let mut vectors = 0_usize;
    for p in 0..3_usize {
        for a in -20_i128..=20 {
            for b in -20_i128..=20 {
                let _ = agree! {{
                    let bounded = IntegerDomain::Bounded(IntegerInterval::new(int(-5), int(5)).unwrap());
                    let run = |m: &mut Meter| divide(DivisionProfile::ALL[p], &int(a), &int(b), &IntegerDomain::Mathematical, m);
                    (
                        metered(UNLIMITED, run),
                        metered(UNLIMITED, |m| divide(DivisionProfile::ALL[p], &int(a), &int(b), &bounded, m)),
                        metered(UNLIMITED, |m| modulo(&int(a), &int(b), &bounded, m)),
                        denials(UNLIMITED, run),
                    )
                }};
                vectors += 1;
            }
        }
    }
    assert_eq!(vectors, 3 * 41 * 41);
}
