// SPDX-License-Identifier: AGPL-3.0-or-later
//! Metered integer arithmetic, rational arithmetic, numeric ordering and
//! Boolean connectives (`quire.value.accounting/v1`, QSpec 5d88578).
//!
//! Each operation charges its named family in order: operands, then the exact
//! arithmetic amount, then (for rationals) the reduced result, then an
//! uncharged FR-044 result-domain membership decision, then retention. Any
//! intermediate materialized before its charge is bounded by operands already
//! admitted, and a stopped operation exposes no value.

use core::cmp::Ordering;

use super::accounting::{Charge, ChargePoint, LimitKind, Meter};
use super::decimal::{Decimal, DecimalRepresentation};
use super::integer::{Integer, IntegerDomain};
use super::outcome::{Outcome, Refusal, Stop, Undefined};
use super::rational::{Rational, RationalDomain};

/// An `Integer` or `Int[..]` `+`, `-`, `*` or unary `-`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerOperation<'a> {
    /// `a + b`.
    Add(&'a Integer, &'a Integer),
    /// `a - b`.
    Subtract(&'a Integer, &'a Integer),
    /// `a * b`.
    Multiply(&'a Integer, &'a Integer),
    /// `-a`.
    Negate(&'a Integer),
}

impl IntegerOperation<'_> {
    /// `(max(bits(operand_i)), operand_count)`.
    fn operands(self) -> (u64, u64) {
        match self {
            Self::Add(a, b) | Self::Subtract(a, b) | Self::Multiply(a, b) => {
                (a.magnitude_bits().max(b.magnitude_bits()), 2)
            }
            Self::Negate(a) => (a.magnitude_bits(), 1),
        }
    }

    fn apply(self) -> Integer {
        match self {
            Self::Add(a, b) => a.add(b),
            Self::Subtract(a, b) => a.sub(b),
            Self::Multiply(a, b) => a.mul(b),
            Self::Negate(a) => a.neg(),
        }
    }
}

/// Evaluate one integer operation into `domain`.
///
/// Charges `integer-arithmetic.operands`, `integer-arithmetic.arithmetic` at
/// `bits(exact result)`, decides membership without a charge, then
/// `integer-arithmetic.result-retain`.
pub fn evaluate_integer(
    operation: IntegerOperation<'_>,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Outcome<Integer> {
    Outcome::from_stop(integer(operation, domain, meter))
}

fn integer(
    operation: IntegerOperation<'_>,
    domain: &IntegerDomain,
    meter: &mut Meter,
) -> Result<Integer, Stop> {
    let (bits, count) = operation.operands();
    meter.charge(
        Charge::new(ChargePoint::IntegerArithmeticOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, count),
    )?;
    let result = operation.apply();
    meter.charge(
        Charge::new(ChargePoint::IntegerArithmeticArithmetic)
            .size(LimitKind::IntegerBits, result.magnitude_bits()),
    )?;
    if !domain.contains(&result) {
        return Err(Stop::Refused(Refusal::IntegerOutOfDomain));
    }
    meter.charge(Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))?;
    Ok(result)
}

/// A `Rational[..]` `+`, `-`, `*`, `/` or unary `-`, or an `Integer` or
/// `Int[..]` `/` producing a `Rational[..]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalOperation<'a> {
    /// `a + b`.
    Add(&'a Rational, &'a Rational),
    /// `a - b`.
    Subtract(&'a Rational, &'a Rational),
    /// `a * b`.
    Multiply(&'a Rational, &'a Rational),
    /// `a / b`.
    Divide(&'a Rational, &'a Rational),
    /// `-a`.
    Negate(&'a Rational),
    /// Integer `n / m` with a rational result; each operand is `n/1`.
    IntegerDivide(&'a Integer, &'a Integer),
}

/// `(a, b)` of an operand `a/b`.
type Parts<'a> = (&'a Integer, &'a Integer);

fn parts(value: &Rational) -> Parts<'_> {
    (value.numerator(), value.denominator())
}

/// Evaluate one rational operation into `domain` (`None` for no result
/// bound).
///
/// Charges `rational-arithmetic.operands`; a zero divisor is undefined after
/// that charge. Then `rational-arithmetic.arithmetic` at the unreduced
/// intermediate, `rational-arithmetic.normalize` at the reduced result, an
/// uncharged membership decision and `rational-arithmetic.result-retain`.
pub fn evaluate_rational(
    operation: RationalOperation<'_>,
    domain: Option<&RationalDomain>,
    meter: &mut Meter,
) -> Outcome<Rational> {
    Outcome::from_stop(rational(operation, domain, meter))
}

fn rational(
    operation: RationalOperation<'_>,
    domain: Option<&RationalDomain>,
    meter: &mut Meter,
) -> Result<Rational, Stop> {
    let one = Integer::one();
    let (left, right): (Parts<'_>, Option<Parts<'_>>) = match operation {
        RationalOperation::Add(a, b)
        | RationalOperation::Subtract(a, b)
        | RationalOperation::Multiply(a, b)
        | RationalOperation::Divide(a, b) => (parts(a), Some(parts(b))),
        RationalOperation::Negate(a) => (parts(a), None),
        RationalOperation::IntegerDivide(n, m) => ((n, &one), Some((m, &one))),
    };
    let maxparts = |(n, d): Parts<'_>| n.magnitude_bits().max(d.magnitude_bits());
    let (bits, count) = match right {
        Some(right) => (maxparts(left).max(maxparts(right)), 2),
        None => (maxparts(left), 1),
    };
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, count),
    )?;
    let ((a, b), (c, d)) = (left, right.unwrap_or((&one, &one)));
    let (numerator, denominator) = match operation {
        RationalOperation::Add(..) => (a.mul(d).add(&c.mul(b)), b.mul(d)),
        RationalOperation::Subtract(..) => (a.mul(d).sub(&c.mul(b)), b.mul(d)),
        RationalOperation::Multiply(..) => (a.mul(c), b.mul(d)),
        RationalOperation::Divide(..) | RationalOperation::IntegerDivide(..) => {
            if c.is_zero() {
                return Err(Stop::Undefined(Undefined::DivisionByZero));
            }
            (a.mul(d), b.mul(c))
        }
        RationalOperation::Negate(..) => (a.neg(), b.clone()),
    };
    meter.charge(Charge::new(ChargePoint::RationalArithmeticArithmetic).size(
        LimitKind::IntegerBits,
        numerator.magnitude_bits().max(denominator.magnitude_bits()),
    ))?;
    // Every denominator above is a product of nonzero parts.
    let result = Rational::reduce(numerator, denominator);
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticNormalize)
            .size(LimitKind::IntegerBits, result.max_part_bits()),
    )?;
    if domain.is_some_and(|domain| !domain.contains(&result)) {
        return Err(Stop::Refused(Refusal::RationalOutOfDomain));
    }
    meter.charge(Charge::new(ChargePoint::RationalArithmeticResultRetain).results(1))?;
    Ok(result)
}

/// A numeric ordering operator.
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

    /// Whether `ordering` of left against right satisfies this operator.
    pub fn holds(self, ordering: Ordering) -> bool {
        match self {
            Self::Less => ordering == Ordering::Less,
            Self::LessOrEqual => ordering != Ordering::Greater,
            Self::Greater => ordering == Ordering::Greater,
            Self::GreaterOrEqual => ordering != Ordering::Less,
        }
    }
}

/// Two operands of one ordered exact numeric kind. `Integer` and `Int[..]`
/// operands are both [`OrderingOperands::Integer`]; decimals are measured in
/// their retained representations.
#[derive(Clone, Copy, Debug)]
pub enum OrderingOperands<'a> {
    /// `Integer` or `Int[..]`.
    Integer(&'a Integer, &'a Integer),
    /// `Rational[..]`.
    Rational(&'a Rational, &'a Rational),
    /// `Decimal[..]`.
    Decimal(&'a Decimal, &'a Decimal),
}

/// Evaluate `left operator right`.
///
/// Charges `ordering.operands`, `ordering.arithmetic` and
/// `ordering.result-retain`: three work units and one result unit. Every
/// amount is derived before any product or aligned coefficient exists.
pub fn evaluate_ordering(
    operator: OrderingOperator,
    operands: OrderingOperands<'_>,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(ordering(operator, operands, meter))
}

fn ordering(
    operator: OrderingOperator,
    operands: OrderingOperands<'_>,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let operands_charge = Charge::new(ChargePoint::OrderingOperands);
    let arithmetic_charge = Charge::new(ChargePoint::OrderingArithmetic);
    let result = match operands {
        OrderingOperands::Integer(a, b) => {
            let bits = a.magnitude_bits().max(b.magnitude_bits());
            meter.charge(
                operands_charge
                    .size(LimitKind::IntegerBits, bits)
                    .size(LimitKind::ValueOccurrences, 2),
            )?;
            meter.charge(arithmetic_charge.size(LimitKind::IntegerBits, bits))?;
            a.cmp(b)
        }
        OrderingOperands::Rational(left, right) => {
            meter.charge(
                operands_charge
                    .size(
                        LimitKind::IntegerBits,
                        left.max_part_bits().max(right.max_part_bits()),
                    )
                    .size(LimitKind::ValueOccurrences, 2),
            )?;
            // `bits(a×d)` and `bits(c×b)` for `a/b` and `c/d`.
            let one = Integer::one();
            let cross = |x: &Integer, y: &Integer| Integer::power_product_bits(x, y, &one);
            let bits = cross(left.numerator(), right.denominator())
                .max(cross(right.numerator(), left.denominator()));
            meter.charge(arithmetic_charge.exact_size(LimitKind::IntegerBits, bits))?;
            left.cmp(right)
        }
        OrderingOperands::Decimal(left, right) => {
            let (l, r) = (left.representation(), right.representation());
            meter.charge(
                operands_charge
                    .size(
                        LimitKind::IntegerBits,
                        l.coefficient()
                            .magnitude_bits()
                            .max(r.coefficient().magnitude_bits()),
                    )
                    .size(
                        LimitKind::DecimalDigits,
                        l.coefficient()
                            .decimal_digits()
                            .max(r.coefficient().decimal_digits()),
                    )
                    .size(LimitKind::ValueOccurrences, 2),
            )?;
            let scale = l.scale().max(r.scale());
            let (l_bits, l_digits) = aligned(l, scale);
            let (r_bits, r_digits) = aligned(r, scale);
            meter.charge(
                arithmetic_charge
                    .size(
                        LimitKind::ScaleExpansion,
                        u64::from(l.scale().abs_diff(r.scale())),
                    )
                    .exact_size(LimitKind::IntegerBits, l_bits.max(r_bits))
                    .exact_size(LimitKind::DecimalDigits, l_digits.max(r_digits)),
            )?;
            left.compare(right)
        }
    };
    meter.charge(Charge::new(ChargePoint::OrderingResultRetain).results(1))?;
    Ok(operator.holds(result))
}

/// `(bits, digits)` of the retained coefficient aligned to `scale`, derived
/// without materializing it.
fn aligned(representation: &DecimalRepresentation, scale: u32) -> (Integer, Integer) {
    let shift = Integer::from(u64::from(scale.saturating_sub(representation.scale())));
    let coefficient = representation.coefficient();
    let bits = Integer::power_product_bits(coefficient, &Integer::from(10_i64), &shift);
    let digits = if coefficient.is_zero() {
        Integer::one()
    } else {
        Integer::from(coefficient.decimal_digits()).add(&shift)
    };
    (bits, digits)
}

/// A binary Boolean connective.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BooleanConnective {
    /// `and`.
    And,
    /// `or`.
    Or,
    /// `implies`.
    Implies,
}

impl BooleanConnective {
    /// Every connective in source order.
    pub const ALL: [Self; 3] = [Self::And, Self::Or, Self::Implies];
}

/// Evaluate `left connective right`.
///
/// The right operand is evaluated only when `left` does not decide the
/// result; `boolean.result-retain` is charged once the result is decided.
/// A stopped right operand is returned unchanged with no retention.
pub fn evaluate_connective(
    connective: BooleanConnective,
    left: bool,
    right: impl FnOnce(&mut Meter) -> Outcome<bool>,
    meter: &mut Meter,
) -> Outcome<bool> {
    let decided = match (connective, left) {
        (BooleanConnective::And, false) => Some(false),
        (BooleanConnective::Or, true) | (BooleanConnective::Implies, false) => Some(true),
        (BooleanConnective::And | BooleanConnective::Or | BooleanConnective::Implies, _) => None,
    };
    let result = match decided {
        Some(result) => result,
        None => match right(meter) {
            Outcome::Completed(result) => result,
            stopped => return stopped,
        },
    };
    Outcome::from_stop(retain_boolean(result, meter))
}

/// Evaluate `not operand`, charging `boolean.result-retain`.
pub fn evaluate_not(operand: bool, meter: &mut Meter) -> Outcome<bool> {
    Outcome::from_stop(retain_boolean(!operand, meter))
}

fn retain_boolean(result: bool, meter: &mut Meter) -> Result<bool, Stop> {
    meter.charge(Charge::new(ChargePoint::BooleanResultRetain).results(1))?;
    Ok(result)
}
