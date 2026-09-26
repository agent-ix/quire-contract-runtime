//! FR-007 exact semantics the shared-corpus agreement suites do not reach:
//! decimal rounding tie-breaks, IEEE exceptional semantics, rational
//! membership and canonical form, decimal normalized-versus-retained,
//! Euclidean `mod` and quantity type-fault order, through the public `exact`
//! surface.
#![cfg(feature = "exact")]

use std::cmp::Ordering;

use quire_contract_runtime::exact::{
    convert_ieee_width, divide, evaluate_decimal, evaluate_ieee, evaluate_quantity,
    evaluate_rational_arithmetic, ieee_to_exact, modulo, order_numbers, ChargePoint, Decimal,
    DecimalOperation, DecimalType, DivisionProfile, IeeeExactLoss, IeeeExactTarget, IeeeFlag,
    IeeeOperation, IeeeValue, IeeeWidth, IllTypedCause, Integer, IntegerDomain, IntegerInterval,
    LimitKind, Meter, NodeKey, OrderedOperands, OrderingOperator, Outcome, Quantity,
    QuantityOperation, QuantityUnit, Rational, RationalArithmetic, RationalDomain, Refusal,
    RoundingMode, ScalarLimits, UnitDeclaration, UnitGraph,
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

fn consumed(meter: &Meter) -> Vec<u64> {
    LimitKind::ALL
        .iter()
        .map(|kind| meter.consumed(*kind))
        .collect()
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

fn ratio(numerator: i128, denominator: i128) -> Rational {
    Rational::new(int(numerator), int(denominator)).unwrap()
}

// ---- AC-8: decimal rounding ------------------------------------------------

/// Trace: TC-034, FR-007-AC-8
#[test]
fn tc_034_decimal_rounding_ties_all_six_modes_both_signs() {
    // `0.5` and `-0.5` rounded to scale zero are exact ties: `remainder × 2 ==
    // denominator` in the underlying reduced rational. `(mode, positive
    // result, negative result)`, `None` meaning strict `exact` refuses.
    let cases: [(RoundingMode, Option<i128>, Option<i128>); 6] = [
        (RoundingMode::Exact, None, None),
        (RoundingMode::TowardZero, Some(0), Some(0)),
        (RoundingMode::TowardPositive, Some(1), Some(0)),
        (RoundingMode::TowardNegative, Some(0), Some(-1)),
        (RoundingMode::NearestEven, Some(0), Some(0)),
        (RoundingMode::NearestAway, Some(1), Some(-1)),
    ];
    for (mode, expected_positive, expected_negative) in cases {
        let target = DecimalType::new(int(-10), int(10), 0, 0, mode).unwrap();
        for (coefficient, expected) in [(5_i128, expected_positive), (-5, expected_negative)] {
            let mut meter = Meter::new(UNLIMITED);
            let outcome = evaluate_decimal(
                DecimalOperation::Round(&Decimal::new(int(coefficient), 1)),
                &target,
                &mut meter,
            );
            match expected {
                Some(value) => {
                    let result = outcome
                        .completed()
                        .expect("tie rounds under a non-exact mode");
                    assert_eq!(result.value().representation().coefficient(), &int(value));
                    assert_eq!(result.value().representation().scale(), 0);
                }
                None => assert_eq!(outcome, Outcome::Refused(Box::new(Refusal::InexactDecimal))),
            }
        }
    }
}

/// Trace: TC-034, FR-007-AC-8
#[test]
fn tc_034_decimal_default_rounding_is_exact_and_refuses_a_discarded_digit() {
    assert_eq!(RoundingMode::default(), RoundingMode::Exact);
    // An omitted spelling is strict `exact`, not merely one member of `ALL`.
    let target = DecimalType::new(int(-10), int(10), 0, 0, RoundingMode::default()).unwrap();
    assert_eq!(target.rounding(), RoundingMode::Exact);
    // `0.1` rounded to an integer discards a nonzero digit, no tie involved.
    let outcome = evaluate_decimal(
        DecimalOperation::Round(&Decimal::new(int(1), 1)),
        &target,
        &mut Meter::new(UNLIMITED),
    );
    assert_eq!(outcome, Outcome::Refused(Box::new(Refusal::InexactDecimal)));
}

/// Trace: TC-034, FR-007-AC-8
#[test]
fn tc_034_decimal_rounding_never_reads_host_floating_point() {
    // Precedent: tests/exact_outcomes.rs
    // `tc_016_exact_sources_have_no_host_float_std_panic_or_unsafe_path` scans
    // every `src/exact` source for the same forbidden tokens; this repeats the
    // check scoped to `decimal.rs`, whose six rounding spellings this test
    // exercises.
    let source = include_str!("../src/exact/decimal.rs");
    let code: String = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    for token in ["f32", "f64"] {
        let hit = code.match_indices(token).any(|(at, _)| {
            let before = code[..at].chars().next_back();
            let after = code[at + token.len()..].chars().next();
            let word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
            !word(before) && !word(after)
        });
        assert!(!hit, "src/exact/decimal.rs contains {token}");
    }
}

// ---- AC-9: IEEE exceptional semantics --------------------------------------

const SIGN32: u32 = 1 << 31;
const EXPONENT32: u32 = 0xFF << 23;
const QUIET32: u32 = 1 << 22;

fn nan32(negative: bool, signaling: bool, payload: u32) -> IeeeValue {
    let sign = if negative { SIGN32 } else { 0 };
    let quiet = if signaling { 0 } else { QUIET32 };
    IeeeValue::binary32(sign | EXPONENT32 | quiet | payload)
}

fn zero32(negative: bool) -> IeeeValue {
    IeeeValue::binary32(if negative { SIGN32 } else { 0 })
}

fn infinity32(negative: bool) -> IeeeValue {
    let sign = if negative { SIGN32 } else { 0 };
    IeeeValue::binary32(sign | EXPONENT32)
}

fn one32(negative: bool) -> IeeeValue {
    let sign = if negative { SIGN32 } else { 0 };
    IeeeValue::binary32(sign | (127_u32 << 23))
}

const SIGN64: u64 = 1 << 63;
const EXPONENT64: u64 = 0x7FF << 52;
const QUIET64: u64 = 1 << 51;

fn nan64(negative: bool, signaling: bool, payload: u64) -> IeeeValue {
    let sign = if negative { SIGN64 } else { 0 };
    let quiet = if signaling { 0 } else { QUIET64 };
    IeeeValue::binary64(sign | EXPONENT64 | quiet | payload)
}

/// Trace: TC-034, FR-007-AC-9
#[test]
fn tc_034_ieee_nan_leftmost_wins_and_invalid_from_any_signaling_operand() {
    let quiet_left = nan32(false, false, 5);
    let signaling_right = nan32(true, true, 9);

    // Both operand positions: the leftmost operand's bits (quieted) win, and
    // `invalid` is raised because one operand is signaling, whichever
    // position it is in.
    for (left, right) in [(quiet_left, signaling_right), (signaling_right, quiet_left)] {
        let mut meter = Meter::new(UNLIMITED);
        let result = evaluate_ieee(
            IeeeOperation::Add(left, right),
            RoundingMode::NearestEven,
            &mut meter,
        )
        .unwrap()
        .completed()
        .unwrap();
        let expected_bits = left.bits() | u64::from(QUIET32);
        assert_eq!(result.value().bits(), expected_bits);
        assert!(result.flags().contains(IeeeFlag::Invalid));
    }

    // Two quiet NaNs: leftmost wins, no `invalid`.
    let other_quiet = nan32(true, false, 3);
    let mut meter = Meter::new(UNLIMITED);
    let result = evaluate_ieee(
        IeeeOperation::Add(quiet_left, other_quiet),
        RoundingMode::NearestEven,
        &mut meter,
    )
    .unwrap()
    .completed()
    .unwrap();
    assert_eq!(result.value().bits(), quiet_left.bits());
    assert!(!result.flags().contains(IeeeFlag::Invalid));

    // A later, third-position NaN never displaces the leftmost.
    let mut meter = Meter::new(UNLIMITED);
    let result = evaluate_ieee(
        IeeeOperation::FusedMultiplyAdd(quiet_left, one32(false), signaling_right),
        RoundingMode::NearestEven,
        &mut meter,
    )
    .unwrap()
    .completed()
    .unwrap();
    assert_eq!(result.value().bits(), quiet_left.bits());
    assert!(result.flags().contains(IeeeFlag::Invalid));
}

/// Trace: TC-034, FR-007-AC-9
#[test]
fn tc_034_ieee_nan_payload_too_wide_for_target_is_refused_not_truncated() {
    // binary32's quiet bit sits at `1 << 22`; a binary64 payload at or above
    // it cannot be represented and must be refused, never truncated.
    for payload in [1_u64 << 30, u64::from(QUIET32)] {
        let source = nan64(false, false, payload);
        let mut meter = Meter::new(UNLIMITED);
        let outcome = convert_ieee_width(
            source,
            IeeeWidth::Binary32,
            RoundingMode::NearestEven,
            &mut meter,
        );
        assert_eq!(
            outcome,
            Outcome::Refused(Box::new(Refusal::IeeeNanPayloadNotRepresentable))
        );
    }
}

/// Trace: TC-034, FR-007-AC-9
#[test]
fn tc_034_ieee_negative_and_positive_zero_convert_to_equal_exact_value() {
    let domain = RationalDomain::new(
        IntegerInterval::new(int(-1), int(1)).unwrap(),
        IntegerInterval::new(int(1), int(1)).unwrap(),
    )
    .unwrap();
    let exact_positive = ieee_to_exact(
        zero32(false),
        IeeeExactTarget::Rational(&domain),
        &mut Meter::new(UNLIMITED),
    )
    .unwrap()
    .completed()
    .unwrap();
    let exact_negative = ieee_to_exact(
        zero32(true),
        IeeeExactTarget::Rational(&domain),
        &mut Meter::new(UNLIMITED),
    )
    .unwrap()
    .completed()
    .unwrap();
    assert_eq!(exact_positive.value(), exact_negative.value());
    assert_eq!(
        exact_positive.value(),
        &Rational::from_integer(Integer::zero())
    );
    assert_eq!(exact_positive.loss(), None);
    assert_eq!(exact_negative.loss(), Some(IeeeExactLoss::NegativeZeroSign));
}

/// Trace: TC-034, FR-007-AC-9
#[test]
fn tc_034_ieee_total_order_key_totally_orders_zeros_and_nans() {
    // Distinguished patterns in their expected `totalOrder` position, both
    // zeros and four NaNs (quiet and signaling, both signs) included.
    let patterns: [(&str, IeeeValue); 10] = [
        ("neg-quiet-nan", nan32(true, false, 9)),
        ("neg-signaling-nan", nan32(true, true, 5)),
        ("neg-infinity", infinity32(true)),
        ("neg-one", one32(true)),
        ("neg-zero", zero32(true)),
        ("pos-zero", zero32(false)),
        ("pos-one", one32(false)),
        ("pos-infinity", infinity32(false)),
        ("pos-signaling-nan", nan32(false, true, 5)),
        ("pos-quiet-nan", nan32(false, false, 9)),
    ];
    let mut sorted = patterns;
    sorted.sort_by_key(|(_, value)| value.total_order_key());
    let sorted_names: Vec<&str> = sorted.iter().map(|(name, _)| *name).collect();
    let expected_names: Vec<&str> = patterns.iter().map(|(name, _)| *name).collect();
    assert_eq!(sorted_names, expected_names);

    // A strict total order: no two distinguished patterns collide.
    let keys: Vec<u64> = sorted
        .iter()
        .map(|(_, value)| value.total_order_key())
        .collect();
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));

    // `-0.0` orders strictly before `+0.0`, though neither is IEEE `<` the
    // other.
    assert!(zero32(true).total_order_key() < zero32(false).total_order_key());
}

// ---- AC-10: rational membership and canonical form -------------------------

/// Trace: TC-034, FR-007-AC-10
#[test]
fn tc_034_rational_domain_excludes_one_third_by_denominator_interval() {
    // The numerator interval `[0, 1]` contains `1/3`'s numeric magnitude, but
    // the denominator interval `[1, 2]` excludes its reduced denominator `3`.
    let domain = RationalDomain::new(
        IntegerInterval::new(int(0), int(1)).unwrap(),
        IntegerInterval::new(int(1), int(2)).unwrap(),
    )
    .unwrap();
    assert!(!domain.contains(&ratio(1, 3)));

    let refused = evaluate_rational_arithmetic(
        RationalArithmetic::Divide(
            &Rational::from_integer(int(1)),
            &Rational::from_integer(int(3)),
        ),
        Some(&domain),
        &mut Meter::new(UNLIMITED),
    );
    assert_eq!(
        refused,
        Outcome::Refused(Box::new(Refusal::RationalOutOfDomain))
    );

    // The identical operation with no result domain performs no membership
    // decision at all and retains the result.
    let retained = evaluate_rational_arithmetic(
        RationalArithmetic::Divide(
            &Rational::from_integer(int(1)),
            &Rational::from_integer(int(3)),
        ),
        None,
        &mut Meter::new(UNLIMITED),
    );
    assert_eq!(retained, Outcome::Completed(ratio(1, 3)));
}

/// Trace: TC-034, FR-007-AC-10
#[test]
fn tc_034_rational_canonical_form_from_unreduced_negative_denominator_and_zero() {
    // Unreduced pair: the reduced form is unique.
    let unreduced = Rational::new(int(4), int(8)).unwrap();
    assert_eq!(unreduced.numerator(), &int(1));
    assert_eq!(unreduced.denominator(), &int(2));

    // Negative denominator: the sign moves into the numerator, the
    // denominator stays strictly positive.
    let negative_denominator = Rational::new(int(1), int(-2)).unwrap();
    assert_eq!(negative_denominator.numerator(), &int(-1));
    assert_eq!(negative_denominator.denominator(), &int(2));

    // Both negative: the sign cancels, still a strictly positive denominator.
    let both_negative = Rational::new(int(-4), int(-8)).unwrap();
    assert_eq!(both_negative.numerator(), &int(1));
    assert_eq!(both_negative.denominator(), &int(2));

    // Zero numerator, whatever the denominator's sign: exactly `0/1`, never
    // `-0/1` and never `0/n` for any other `n`.
    for denominator in [5_i128, -5] {
        let zero = Rational::new(int(0), int(denominator)).unwrap();
        assert_eq!(zero.numerator(), &int(0));
        assert_eq!(zero.denominator(), &int(1));
    }
}

// ---- AC-11: decimal normalized value versus retained charges ---------------

/// Trace: TC-034, FR-007-AC-11
#[test]
fn tc_034_decimal_value_equality_is_normalized_but_charges_are_retained() {
    let retained_wide = Decimal::new(int(110), 2); // 1.10
    let retained_narrow = Decimal::new(int(11), 1); // 1.1

    // Value comparison is mathematical and normalized: the two are equal and
    // neither orders before the other.
    assert_eq!(retained_wide.compare(&retained_narrow), Ordering::Equal);
    assert!(retained_wide.numerically_equal(&retained_narrow));

    let less = order_numbers(
        OrderingOperator::Less,
        OrderedOperands::Decimals(&retained_wide, &retained_narrow),
        &mut Meter::new(UNLIMITED),
    );
    assert_eq!(less, Outcome::Completed(false));
    let greater = order_numbers(
        OrderingOperator::Greater,
        OrderedOperands::Decimals(&retained_wide, &retained_narrow),
        &mut Meter::new(UNLIMITED),
    );
    assert_eq!(greater, Outcome::Completed(false));

    // Charges are sized on the retained representation, not the shared
    // normalized value: comparing the wide-retained `1.10` against the
    // narrow-retained `1.1` costs a different amount than comparing the
    // narrow-retained value against itself, even though the values agree.
    let mut wide_vs_narrow = Meter::new(UNLIMITED);
    let _ = order_numbers(
        OrderingOperator::Less,
        OrderedOperands::Decimals(&retained_wide, &retained_narrow),
        &mut wide_vs_narrow,
    );
    let mut narrow_vs_narrow = Meter::new(UNLIMITED);
    let _ = order_numbers(
        OrderingOperator::Less,
        OrderedOperands::Decimals(&retained_narrow, &retained_narrow),
        &mut narrow_vs_narrow,
    );
    assert_ne!(consumed(&wide_vs_narrow), consumed(&narrow_vs_narrow));
    // `operands`: max(bits(110), bits(11))=7, max(digits(110)=3, digits(11)=2)=3;
    // `arithmetic`: scale expansion `|2-1|=1`, aligned bits
    // max(shifted_bits(110,0)=7, shifted_bits(11,1)=8)=8, aligned digits 3.
    assert_eq!(consumed(&wide_vs_narrow), [8, 3, 1, 0, 0, 0, 0, 2, 3, 1]);
    // Comparing the narrow retained value against itself has no scale
    // expansion and smaller bit/digit amounts.
    assert_eq!(consumed(&narrow_vs_narrow), [4, 2, 0, 0, 0, 0, 0, 2, 3, 1]);
}

/// Trace: TC-034, FR-007-AC-11
#[test]
fn tc_034_decimal_has_no_structural_partial_eq() {
    let source = include_str!("../src/exact/decimal.rs");
    assert!(source.contains("#[derive(Clone)]\npub struct Decimal(Box<DecimalFields>);"));
    assert!(source.contains("#[derive(Clone)]\nstruct DecimalFields {"));
    // `DecimalResult` does implement `PartialEq`, on its retained
    // representation and loss record; `Decimal` itself must not.
    assert!(!source.contains("impl PartialEq for Decimal "));
}

// ---- AC-12: Euclidean `mod`, pair refusal and quantity fault order --------

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_euclidean_mod_is_law_independent_across_all_operand_signs() {
    let domain = IntegerDomain::Mathematical;
    for (a, b) in [(7_i128, 3_i128), (7, -3), (-7, 3), (-7, -3)] {
        // An independent oracle: Rust's own Euclidean remainder.
        let expected = int(a.rem_euclid(b));
        for profile in DivisionProfile::ALL {
            // A `div`/`rem` law selected earlier in the same run never
            // changes `mod`'s answer.
            let mut meter = Meter::new(UNLIMITED);
            let _ = divide(profile, &int(a), &int(b), &domain, &mut meter);
            let remainder = modulo_completed(&int(a), &int(b), &domain, &mut meter);
            assert_eq!(remainder, expected);
        }
    }
}

fn modulo_completed(
    dividend: &Integer,
    divisor: &Integer,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Integer {
    modulo(dividend, divisor, domain, meter)
        .completed()
        .unwrap()
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_division_pair_refused_names_admitted_members_per_combination() {
    let domain = IntegerDomain::Bounded(IntegerInterval::new(int(-3), int(3)).unwrap());
    let combinations = [
        // (dividend, divisor, quotient_admitted, remainder_admitted)
        (19_i128, 5_i128, true, false),
        (20, 5, false, true),
        (24, 5, false, false),
    ];
    for (dividend, divisor, quotient_admitted, remainder_admitted) in combinations {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = divide(
            DivisionProfile::Truncating,
            &int(dividend),
            &int(divisor),
            &domain,
            &mut meter,
        );
        assert_eq!(
            outcome,
            Outcome::Refused(Box::new(Refusal::DivisionPairOutOfDomain {
                quotient_admitted,
                remainder_admitted,
            }))
        );
        // Neither member is exposed: no result is retained.
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        assert_eq!(
            meter.admitted_charges(),
            [
                ChargePoint::IntegerDivisionOperands,
                ChargePoint::IntegerDivisionArithmetic,
                ChargePoint::IntegerDivisionDomainPair,
            ]
        );
    }
}

const DIM_LENGTH: NodeKey = NodeKey::from_bytes([1; 32]);
const DIM_TEMPERATURE: NodeKey = NodeKey::from_bytes([2; 32]);
const UNIT_METER: NodeKey = NodeKey::from_bytes([10; 32]);
const UNIT_KILOMETER: NodeKey = NodeKey::from_bytes([11; 32]);
const UNIT_KELVIN: NodeKey = NodeKey::from_bytes([20; 32]);
const UNIT_CELSIUS: NodeKey = NodeKey::from_bytes([21; 32]);

/// A length dimension with a root `meter` and a non-root `kilometer`, and a
/// temperature dimension with a root `kelvin` and an affine `celsius`
/// (nonzero composed offset), for the quantity type-check tests.
fn graph() -> UnitGraph {
    let one = Rational::from_integer(Integer::one());
    let zero = Rational::from_integer(Integer::zero());
    UnitGraph::admit(
        [
            (DIM_LENGTH, Vec::<(NodeKey, Integer)>::new()),
            (DIM_TEMPERATURE, Vec::new()),
        ],
        [
            (
                UNIT_METER,
                UnitDeclaration {
                    dimension: DIM_LENGTH,
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_KILOMETER,
                UnitDeclaration {
                    dimension: DIM_LENGTH,
                    target: Some(UNIT_METER),
                    scale: ratio(1000, 1),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_KELVIN,
                UnitDeclaration {
                    dimension: DIM_TEMPERATURE,
                    target: None,
                    scale: one.clone(),
                    offset: zero.clone(),
                },
            ),
            (
                UNIT_CELSIUS,
                UnitDeclaration {
                    dimension: DIM_TEMPERATURE,
                    target: Some(UNIT_KELVIN),
                    scale: one,
                    offset: ratio(273, 1),
                },
            ),
        ],
    )
    .unwrap()
}

fn quantity(graph: &UnitGraph, unit_key: NodeKey, value: i128) -> Quantity {
    let unit = graph.unit(unit_key).unwrap().clone();
    Quantity::new(ratio(value, 1), QuantityUnit::Declared(Box::new(unit)))
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_quantity_add_subtract_cause_order_first_failure_wins() {
    let graph = graph();
    let length = quantity(&graph, UNIT_METER, 1);
    let kilometers = quantity(&graph, UNIT_KILOMETER, 1);
    let celsius = quantity(&graph, UNIT_CELSIUS, 1);
    let kelvin = quantity(&graph, UNIT_KELVIN, 1);

    // Incompatible dimensions and an affine operand both fail; the dimension
    // check wins because it runs first.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&length, &celsius), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::IncompatibleDimensions);
    assert!(meter.admitted_charges().is_empty());

    // Subtract checks in the identical order.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Subtract(&length, &celsius), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::IncompatibleDimensions);
    assert!(meter.admitted_charges().is_empty());

    // Compatible dimension, but an affine operand and distinct units both
    // fail; the affine check wins because it runs before the units check.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&celsius, &kelvin), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::AffineUnitArithmetic);
    assert!(meter.admitted_charges().is_empty());

    // Compatible dimension, neither operand affine, distinct units: the
    // units check is reached last.
    let mut meter = Meter::new(UNLIMITED);
    let error =
        evaluate_quantity(QuantityOperation::Add(&length, &kilometers), &mut meter).unwrap_err();
    assert_eq!(error.cause, IllTypedCause::DistinctUnits);
    assert!(meter.admitted_charges().is_empty());

    // Every one of these is `ill_typed` with zero charges, reported beside
    // the outcome rather than inside a result: `evaluate_quantity` returns
    // `Err` before `Ok(Outcome::..)`.
}

/// Trace: TC-034, FR-007-AC-12
#[test]
fn tc_034_quantity_multiply_and_divide_raise_no_dimension_fault() {
    let graph = graph();
    let length = quantity(&graph, UNIT_METER, 2);
    let kelvin = quantity(&graph, UNIT_KELVIN, 3);

    // Cross-dimension operands would fail `Add`/`Subtract`'s incompatible-
    // dimensions check; `Multiply` and `Divide` never check dimension
    // compatibility, since their dimensions combine rather than match.
    let mut meter = Meter::new(UNLIMITED);
    let product = evaluate_quantity(QuantityOperation::Multiply(&length, &kelvin), &mut meter)
        .expect("multiply is well-typed across dimensions");
    assert!(product.completed().is_some());

    let mut meter = Meter::new(UNLIMITED);
    let quotient = evaluate_quantity(QuantityOperation::Divide(&length, &kelvin), &mut meter)
        .expect("divide is well-typed across dimensions");
    assert!(quotient.completed().is_some());
}
