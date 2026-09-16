// SPDX-License-Identifier: AGPL-3.0-or-later
//! Metered integer arithmetic, rational arithmetic, numeric ordering and
//! Boolean connectives (`quire.value.accounting/v1`, QSpec 7d7943a).
//!
//! Each operation charges its named family in order: operands, then the
//! arithmetic amount, then (for rationals) the unreduced intermediate, then an
//! uncharged FR-044 result-domain membership decision, then retention. Every
//! arithmetic amount is derived from the bit and digit lengths of operands that
//! are already materialized, never from the unmaterialized result, so the
//! charge strictly precedes every result allocation. The rational normalize
//! amount sizes the unreduced intermediate that the admitted arithmetic charge
//! materialized. A stopped operation exposes no value.

use core::cmp::Ordering;

use super::accounting::{Charge, ChargePoint, LimitKind, Meter};
use super::decimal::{shifted_bits, shifted_digits, Decimal, DecimalRepresentation};
use super::integer::{Integer, IntegerDomain};
use super::outcome::{Outcome, Refusal, Stop, Undefined};
use super::rational::{cross_bits, Parts, Rational, RationalArithmetic, RationalDomain};

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

    /// The `integer-arithmetic.arithmetic` amount, derived from operand bit
    /// lengths only: `bits(a)+bits(b)` for `*`, `max(bits(a),bits(b))+1` for
    /// `+` and `-`, and `bits(a)` for unary `-`.
    fn arithmetic_bits(self) -> u64 {
        // Operands are materialized, so a sum of two bit lengths stays far
        // below `u64::MAX`.
        match self {
            Self::Add(a, b) | Self::Subtract(a, b) => {
                a.magnitude_bits().max(b.magnitude_bits()).saturating_add(1)
            }
            Self::Multiply(a, b) => a.magnitude_bits().saturating_add(b.magnitude_bits()),
            Self::Negate(a) => a.magnitude_bits(),
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
/// its operand-derived amount, computes the result, decides membership without
/// a charge, then
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
    meter.charge(
        Charge::new(ChargePoint::IntegerArithmeticArithmetic)
            .size(LimitKind::IntegerBits, operation.arithmetic_bits()),
    )?;
    let result = operation.apply();
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

/// Evaluate one rational operation into `domain` (`None` for no result
/// bound).
///
/// Charges `rational-arithmetic.operands`; a zero divisor is undefined after
/// that charge. Then `rational-arithmetic.arithmetic` at its operand-derived
/// amount, materializes the unreduced intermediate, charges
/// `rational-arithmetic.normalize` at that intermediate's parts, reduces it,
/// decides membership without a charge and charges
/// `rational-arithmetic.result-retain`.
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
    let (left, binary): (Parts<'_>, Option<(RationalArithmetic, Parts<'_>)>) = match operation {
        RationalOperation::Add(a, b) => (a.parts(), Some((RationalArithmetic::Add, b.parts()))),
        RationalOperation::Subtract(a, b) => {
            (a.parts(), Some((RationalArithmetic::Subtract, b.parts())))
        }
        RationalOperation::Multiply(a, b) => {
            (a.parts(), Some((RationalArithmetic::Multiply, b.parts())))
        }
        RationalOperation::Divide(a, b) => {
            (a.parts(), Some((RationalArithmetic::Divide, b.parts())))
        }
        RationalOperation::IntegerDivide(n, m) => {
            ((n, &one), Some((RationalArithmetic::Divide, (m, &one))))
        }
        RationalOperation::Negate(a) => (a.parts(), None),
    };
    let maxparts = |(n, d): Parts<'_>| n.magnitude_bits().max(d.magnitude_bits());
    let (bits, count) = match binary {
        Some((_, right)) => (maxparts(left).max(maxparts(right)), 2),
        None => (maxparts(left), 1),
    };
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticOperands)
            .size(LimitKind::IntegerBits, bits)
            .size(LimitKind::ValueOccurrences, count),
    )?;
    // Sized from operand bit lengths, charged, and only then materialized.
    if let Some((RationalArithmetic::Divide, (divisor, _))) = binary {
        if divisor.is_zero() {
            return Err(Stop::Undefined(Undefined::DivisionByZero));
        }
    }
    // Unary `-` charges `max(bits(a), bits(b))`.
    let amount = binary.map_or(maxparts(left), |(operation, right)| {
        operation.charge_bits(left, right)
    });
    meter.charge(
        Charge::new(ChargePoint::RationalArithmeticArithmetic).size(LimitKind::IntegerBits, amount),
    )?;
    let (numerator, denominator) = match binary {
        Some((operation, right)) => operation.unreduced(left, right),
        None => (left.0.neg(), left.1.clone()),
    };
    // The unreduced intermediate is materialized and sized before reduction.
    meter.charge(Charge::new(ChargePoint::RationalArithmeticNormalize).size(
        LimitKind::IntegerBits,
        numerator.magnitude_bits().max(denominator.magnitude_bits()),
    ))?;
    // Every denominator above is a product of nonzero parts.
    let result = Rational::reduce(numerator, denominator);
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
            meter.charge(arithmetic_charge.size(
                LimitKind::IntegerBits,
                cross_bits(left.parts(), right.parts()),
            ))?;
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
            // Aligned to `s = max(s1, s2)`: `sbits` and `sdigits` of each
            // retained coefficient under its shift `s - s_i`.
            let scale = l.scale().max(r.scale());
            let shift =
                |side: &DecimalRepresentation| u64::from(scale.saturating_sub(side.scale()));
            let (l_shift, r_shift) = (shift(l), shift(r));
            meter.charge(
                arithmetic_charge
                    .size(
                        LimitKind::ScaleExpansion,
                        u64::from(l.scale().abs_diff(r.scale())),
                    )
                    .exact_size(
                        LimitKind::IntegerBits,
                        shifted_bits(l.coefficient(), l_shift)
                            .max(shifted_bits(r.coefficient(), r_shift)),
                    )
                    .exact_size(
                        LimitKind::DecimalDigits,
                        shifted_digits(l.coefficient(), l_shift)
                            .max(shifted_digits(r.coefficient(), r_shift)),
                    ),
            )?;
            left.compare(right)
        }
    };
    meter.charge(Charge::new(ChargePoint::OrderingResultRetain).results(1))?;
    Ok(operator.holds(result))
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
