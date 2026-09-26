// SPDX-License-Identifier: AGPL-3.0-or-later
//! Metered exact numeric kernels: `Integer`/`Int[..]` arithmetic,
//! `Rational[..]` arithmetic, numeric ordering and Boolean connectives, in the
//! `quire.value.accounting/v1` charge order.
//!
//! Every size amount is derived before the value it measures is retained; no
//! power of ten is allocated to measure an aligned decimal coefficient.

use core::cmp::Ordering;

use super::accounting::{Charge, ChargePoint, Incomplete, LimitKind, Meter};
use super::decimal::{shifted_bits, shifted_digits, Decimal};
use super::integer::{Integer, IntegerInterval};
use super::outcome::{Outcome, Refusal, Stop, Undefined};
use super::rational::{Rational, RationalDomain};

/// A numeric ordering operator. Equality has its own quire-specification/FR-149 schedule.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OrderingOperator {
    /// `<`.
    Less,
    /// `<=`.
    LessOrEqual,
    /// `>`.
    Greater,
    /// `>=`.
    GreaterOrEqual,
}

impl OrderingOperator {
    /// Every operator in source order.
    pub const ALL: [Self; 4] = [
        Self::Less,
        Self::LessOrEqual,
        Self::Greater,
        Self::GreaterOrEqual,
    ];

    fn holds(self, ordering: Ordering) -> bool {
        match self {
            Self::Less => ordering.is_lt(),
            Self::LessOrEqual => ordering.is_le(),
            Self::Greater => ordering.is_gt(),
            Self::GreaterOrEqual => ordering.is_ge(),
        }
    }
}

/// The two operands of one numeric ordering, both of one exact kind.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum OrderedOperands<'a> {
    /// `Integer` or `Int[..]` operands.
    Integers(&'a Integer, &'a Integer),
    /// `Rational[..]` operands.
    Rationals(&'a Rational, &'a Rational),
    /// `Decimal[..]` operands in their retained representations.
    Decimals(&'a Decimal, &'a Decimal),
}

/// Order two exact numbers: `ordering.operands`, `ordering.arithmetic`, then
/// `ordering.result-retain`.
pub fn order_numbers(
    operator: OrderingOperator,
    operands: OrderedOperands<'_>,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(order(operator, operands, meter))
}

fn order(
    operator: OrderingOperator,
    operands: OrderedOperands<'_>,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let occurrences = LimitKind::ValueOccurrences;
    let ordering = match operands {
        OrderedOperands::Integers(left, right) => {
            meter.charge(
                Charge::new(ChargePoint::OrderingOperands)
                    .size(
                        LimitKind::IntegerBits,
                        left.magnitude_bits().max(right.magnitude_bits()),
                    )
                    .size(occurrences, 2),
            )?;
            meter.charge(
                Charge::new(ChargePoint::OrderingArithmetic)
                    .exact_size(LimitKind::IntegerBits, integer_ordering_bits(left, right)),
            )?;
            left.cmp(right)
        }
        OrderedOperands::Rationals(left, right) => {
            meter.charge(
                Charge::new(ChargePoint::OrderingOperands)
                    .size(
                        LimitKind::IntegerBits,
                        left.max_part_bits().max(right.max_part_bits()),
                    )
                    .size(occurrences, 2),
            )?;
            meter.charge(
                Charge::new(ChargePoint::OrderingArithmetic)
                    .exact_size(LimitKind::IntegerBits, rational_ordering_bits(left, right)),
            )?;
            left.cmp(right)
        }
        OrderedOperands::Decimals(left, right) => {
            let (left_repr, right_repr) = (left.representation(), right.representation());
            let (left_coefficient, right_coefficient) =
                (left_repr.coefficient(), right_repr.coefficient());
            meter.charge(
                Charge::new(ChargePoint::OrderingOperands)
                    .size(
                        LimitKind::IntegerBits,
                        left_coefficient
                            .magnitude_bits()
                            .max(right_coefficient.magnitude_bits()),
                    )
                    .size(
                        LimitKind::DecimalDigits,
                        left_coefficient
                            .decimal_digits()
                            .max(right_coefficient.decimal_digits()),
                    )
                    .size(occurrences, 2),
            )?;
            // Aligned to `s = max(s1, s2)`: `shifted_bits` and
            // `shifted_digits` of each retained coefficient under its shift
            // `s - s_i`.
            let scale = left_repr.scale().max(right_repr.scale());
            let shift = |side: u32| u64::from(scale.saturating_sub(side));
            let (left_shift, right_shift) = (shift(left_repr.scale()), shift(right_repr.scale()));
            meter.charge(
                Charge::new(ChargePoint::OrderingArithmetic)
                    .size(
                        LimitKind::ScaleExpansion,
                        u64::from(left_repr.scale().abs_diff(right_repr.scale())),
                    )
                    .exact_size(
                        LimitKind::IntegerBits,
                        shifted_bits(left_coefficient, left_shift)
                            .max(shifted_bits(right_coefficient, right_shift)),
                    )
                    .exact_size(
                        LimitKind::DecimalDigits,
                        shifted_digits(left_coefficient, left_shift)
                            .max(shifted_digits(right_coefficient, right_shift)),
                    ),
            )?;
            left.compare(right)
        }
    };
    meter.charge(Charge::new(ChargePoint::OrderingResultRetain).results(1))?;
    Ok(operator.holds(ordering))
}

// Arithmetic charge amounts: one function per charge point, so an amount rule
// changes in exactly one place. Every amount derives from operand sizes.

/// `bits(n)` as an unbounded amount.
fn bits(value: &Integer) -> Integer {
    Integer::from(value.magnitude_bits())
}

/// The `integer_bits` amount of `ordering.arithmetic` for integers.
fn integer_ordering_bits(left: &Integer, right: &Integer) -> Integer {
    bits(left).max(bits(right))
}

/// The `integer_bits` amount of `ordering.arithmetic` for `a/b` and `c/d`:
/// `max(bits(a)+bits(d), bits(c)+bits(b))`.
fn rational_ordering_bits(left: &Rational, right: &Rational) -> Integer {
    cross_bits(left, right).max(cross_bits(right, left))
}

/// `bits(a)+bits(d)` for `a/b` and `c/d`.
fn cross_bits(left: &Rational, right: &Rational) -> Integer {
    bits(left.numerator()).add(&bits(right.denominator()))
}

/// The `integer_bits` amount of `integer-arithmetic.arithmetic`:
/// `bits(a)+bits(b)` for `*`, `max(bits(a),bits(b))+1` for `+` and `-`, and
/// `bits(a)` for unary `-`.
fn integer_arithmetic_bits(operation: IntegerArithmetic<'_>) -> Integer {
    match operation {
        IntegerArithmetic::Add(left, right) | IntegerArithmetic::Subtract(left, right) => {
            bits(left).max(bits(right)).add(&Integer::one())
        }
        IntegerArithmetic::Multiply(left, right) => bits(left).add(&bits(right)),
        IntegerArithmetic::Negate(operand) => bits(operand),
    }
}

/// The `integer_bits` amount of `rational-arithmetic.arithmetic` for `a/b`
/// and `c/d`: `max(N,D)`, with `N` and `D` bounding the unreduced parts, and
/// `max(bits(a),bits(b))` for unary `-`. `unit.rational-arithmetic` reuses it.
pub(crate) fn rational_arithmetic_bits(operation: RationalArithmetic<'_>) -> Integer {
    let (numerator, denominator) = match operation {
        RationalArithmetic::Multiply(left, right) => (
            bits(left.numerator()).add(&bits(right.numerator())),
            bits(left.denominator()).add(&bits(right.denominator())),
        ),
        RationalArithmetic::Divide(left, right) => (
            cross_bits(left, right),
            bits(left.denominator()).add(&bits(right.numerator())),
        ),
        RationalArithmetic::Add(left, right) | RationalArithmetic::Subtract(left, right) => (
            rational_ordering_bits(left, right).add(&Integer::one()),
            bits(left.denominator()).add(&bits(right.denominator())),
        ),
        RationalArithmetic::Negate(operand) => {
            (bits(operand.numerator()), bits(operand.denominator()))
        }
    };
    numerator.max(denominator)
}

/// One `Integer` or `Int[..]` arithmetic operation.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum IntegerArithmetic<'a> {
    /// `a + b`.
    Add(&'a Integer, &'a Integer),
    /// `a - b`.
    Subtract(&'a Integer, &'a Integer),
    /// `a * b`.
    Multiply(&'a Integer, &'a Integer),
    /// `-a`.
    Negate(&'a Integer),
}

/// Evaluate integer arithmetic: `integer-arithmetic.operands`,
/// `integer-arithmetic.arithmetic`, the uncharged membership of an optional
/// quire-specification/FR-044 result bound, then `integer-arithmetic.result-retain`.
pub fn evaluate_integer_arithmetic(
    operation: IntegerArithmetic<'_>,
    bound: Option<&IntegerInterval>,
    meter: &mut Meter,
) -> Outcome<Integer> {
    // Builds the `Outcome` directly rather than through `Result<Integer,
    // Stop>` (IR-286): that `Result`'s discriminant is a niche inside `Stop`,
    // and Kani writes `Ok` with nondet bytes over it, so CBMC cannot fold it.
    let (bits, count) = match operation {
        IntegerArithmetic::Add(left, right)
        | IntegerArithmetic::Subtract(left, right)
        | IntegerArithmetic::Multiply(left, right) => {
            (left.magnitude_bits().max(right.magnitude_bits()), 2)
        }
        IntegerArithmetic::Negate(operand) => (operand.magnitude_bits(), 1),
    };
    if let Err(record) = meter.charge(
        Charge::new(ChargePoint::IntegerArithmeticOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, count),
    ) {
        return Outcome::Incomplete(record);
    }
    if let Err(record) = meter.charge(
        Charge::new(ChargePoint::IntegerArithmeticArithmetic)
            .exact_size(LimitKind::IntegerBits, integer_arithmetic_bits(operation)),
    ) {
        return Outcome::Incomplete(record);
    }
    let result = match operation {
        IntegerArithmetic::Add(left, right) => left.add(right),
        IntegerArithmetic::Subtract(left, right) => left.sub(right),
        IntegerArithmetic::Negate(operand) => operand.neg(),
        IntegerArithmetic::Multiply(left, right) => left.mul(right),
    };
    if bound.is_some_and(|bound| !bound.contains(&result)) {
        return Outcome::Refused(Refusal::IntegerOutOfDomain);
    }
    if let Err(record) =
        meter.charge(Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))
    {
        return Outcome::Incomplete(record);
    }
    Outcome::Completed(result)
}

/// One `Rational[..]` arithmetic operation. An `Integer` or `Int[..]` `/`
/// producing `Rational[..]` takes each operand `n` as `n/1`
/// ([`Rational::from_integer`](super::Rational::from_integer)).
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum RationalArithmetic<'a> {
    /// `a/b + c/d`.
    Add(&'a Rational, &'a Rational),
    /// `a/b - c/d`.
    Subtract(&'a Rational, &'a Rational),
    /// `a/b * c/d`.
    Multiply(&'a Rational, &'a Rational),
    /// `(a/b) / (c/d)`.
    Divide(&'a Rational, &'a Rational),
    /// `-(a/b)`.
    Negate(&'a Rational),
}

/// Evaluate rational arithmetic: `rational-arithmetic.operands`, a zero
/// divisor as undefined, `rational-arithmetic.arithmetic` from the operands,
/// `rational-arithmetic.normalize` on the unreduced intermediate, the
/// uncharged membership of
/// an optional quire-specification/FR-044 result domain, then `rational-arithmetic.result-retain`.
pub fn evaluate_rational_arithmetic(
    operation: RationalArithmetic<'_>,
    domain: Option<&RationalDomain>,
    meter: &mut Meter,
) -> Outcome<Rational> {
    Outcome::from_stop(rational_arithmetic(operation, domain, meter))
}

fn rational_arithmetic(
    operation: RationalArithmetic<'_>,
    domain: Option<&RationalDomain>,
    meter: &mut Meter,
) -> Result<Rational, Stop> {
    let (bits, count) = match operation {
        RationalArithmetic::Add(left, right)
        | RationalArithmetic::Subtract(left, right)
        | RationalArithmetic::Multiply(left, right)
        | RationalArithmetic::Divide(left, right) => {
            (left.max_part_bits().max(right.max_part_bits()), 2)
        }
        RationalArithmetic::Negate(operand) => (operand.max_part_bits(), 1),
    };
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, count),
    )?;
    if let RationalArithmetic::Divide(_, divisor) = operation {
        if divisor.is_zero() {
            return Err(Stop::Undefined(Undefined::DivisionByZero));
        }
    }
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticArithmetic)
            .exact_size(LimitKind::IntegerBits, rational_arithmetic_bits(operation)),
    )?;
    let (numerator, denominator) = match operation {
        RationalArithmetic::Add(left, right) => (
            left.numerator()
                .mul(right.denominator())
                .add(&right.numerator().mul(left.denominator())),
            left.denominator().mul(right.denominator()),
        ),
        RationalArithmetic::Subtract(left, right) => (
            left.numerator()
                .mul(right.denominator())
                .sub(&right.numerator().mul(left.denominator())),
            left.denominator().mul(right.denominator()),
        ),
        RationalArithmetic::Multiply(left, right) => (
            left.numerator().mul(right.numerator()),
            left.denominator().mul(right.denominator()),
        ),
        RationalArithmetic::Divide(left, right) => (
            left.numerator().mul(right.denominator()),
            left.denominator().mul(right.numerator()),
        ),
        RationalArithmetic::Negate(operand) => {
            (operand.numerator().neg(), operand.denominator().clone())
        }
    };
    meter.charge(Charge::new(ChargePoint::RationalArithmeticNormalize).size(
        LimitKind::IntegerBits,
        numerator.magnitude_bits().max(denominator.magnitude_bits()),
    ))?;
    let result = Rational::new(numerator, denominator)
        .map_err(|_| Stop::Undefined(Undefined::DivisionByZero))?;
    if domain.is_some_and(|domain| !domain.contains(&result)) {
        return Err(Stop::Refused(Refusal::RationalOutOfDomain));
    }
    meter.charge(Charge::new(ChargePoint::RationalArithmeticResultRetain).results(1))?;
    Ok(result)
}

/// One Boolean connective over decided operands.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BooleanConnective {
    /// `a and b`.
    And(bool, bool),
    /// `a or b`.
    Or(bool, bool),
    /// `a implies b`.
    Implies(bool, bool),
    /// `not a`.
    Not(bool),
}

/// Decide a connective, then charge `boolean.result-retain`.
pub fn evaluate_boolean(connective: BooleanConnective, meter: &mut Meter) -> Outcome<bool> {
    let result = match connective {
        BooleanConnective::And(left, right) => left && right,
        BooleanConnective::Or(left, right) => left || right,
        BooleanConnective::Implies(left, right) => !left || right,
        BooleanConnective::Not(operand) => !operand,
    };
    Outcome::from_stop(retain_boolean(result, meter).map_err(Stop::from))
}

/// A short-circuiting Boolean connective kind: `and`, `or` or `implies`.
///
/// Unlike [`BooleanConnective`], the right operand is evaluated lazily and may
/// itself stop; see [`evaluate_boolean_short_circuit`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ShortCircuitConnective {
    /// `a and b`.
    And,
    /// `a or b`.
    Or,
    /// `a implies b`.
    Implies,
}

/// Decide a short-circuiting connective whose right operand may stop.
///
/// `left` is always decided. When `left` alone determines the result (`false and _`, `true or _`,
/// `false implies _`), `right` is never called, no stop it could produce can arise, and the
/// decided result charges `boolean.result-retain` exactly once. Otherwise `right()` runs: if it
/// stops, that stop is returned unchanged and no `boolean.result-retain` charge is admitted; if it
/// completes, the combined result charges `boolean.result-retain` exactly once.
pub fn evaluate_boolean_short_circuit(
    connective: ShortCircuitConnective,
    left: bool,
    right: impl FnOnce() -> Outcome<bool>,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(short_circuit(connective, left, right, meter))
}

fn short_circuit(
    connective: ShortCircuitConnective,
    left: bool,
    right: impl FnOnce() -> Outcome<bool>,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let decided = match connective {
        ShortCircuitConnective::And if !left => Some(false),
        ShortCircuitConnective::Or if left => Some(true),
        ShortCircuitConnective::Implies if !left => Some(true),
        ShortCircuitConnective::And
        | ShortCircuitConnective::Or
        | ShortCircuitConnective::Implies => None,
    };
    let result = match decided {
        Some(result) => result,
        None => {
            let right_value = right().into_stop()?;
            match connective {
                ShortCircuitConnective::And => left && right_value,
                ShortCircuitConnective::Or => left || right_value,
                ShortCircuitConnective::Implies => right_value,
            }
        }
    };
    retain_boolean(result, meter).map_err(Stop::from)
}

/// Charge `boolean.result-retain` for a decided connective result.
pub(crate) fn retain_boolean(result: bool, meter: &mut Meter) -> Result<bool, Incomplete> {
    meter.charge(Charge::new(ChargePoint::BooleanResultRetain).results(1))?;
    Ok(result)
}
