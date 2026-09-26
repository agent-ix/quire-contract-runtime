// SPDX-License-Identifier: AGPL-3.0-or-later
//! Canonical exact rationals (quire-specification/AD-005, quire-specification/FR-140 loss records).

use alloc::boxed::Box;
use core::cmp::Ordering;
use core::fmt;

use super::integer::{Integer, IntegerInterval};

/// A reduced rational: positive denominator, `gcd(numerator, denominator) = 1`,
/// and zero is exactly `0/1`. Construction is the only way to obtain one.
// Fields behind one `Box` (IR-286): `Value` carries this type inline, and a
// `Value::Integer` read back from a `Vec<Value>` folds under CBMC only when
// `Integer` fills `Value`'s whole payload union, so no other inline payload
// may be larger than an `Integer`.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Rational(Box<RationalFields>);

#[derive(Clone, Eq, Hash, PartialEq)]
struct RationalFields {
    numerator: Integer,
    denominator: Integer,
}

impl fmt::Debug for Rational {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Rational")
            .field("numerator", &self.0.numerator)
            .field("denominator", &self.0.denominator)
            .finish()
    }
}

/// A rational with a zero denominator was requested.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZeroDenominator;

impl fmt::Display for ZeroDenominator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("zero rational denominator")
    }
}

impl Rational {
    fn from_parts(numerator: Integer, denominator: Integer) -> Self {
        Self(Box::new(RationalFields {
            numerator,
            denominator,
        }))
    }

    /// Normalize `numerator/denominator`, refusing a zero denominator.
    pub fn new(numerator: Integer, denominator: Integer) -> Result<Self, ZeroDenominator> {
        if denominator.is_zero() {
            return Err(ZeroDenominator);
        }
        Ok(Self::reduce(numerator, denominator))
    }

    /// Reduce a fraction whose denominator is known to be nonzero.
    pub(crate) fn reduce(numerator: Integer, denominator: Integer) -> Self {
        if numerator.is_zero() {
            return Self::from_integer(Integer::zero());
        }
        let divisor = numerator.gcd(&denominator);
        let (mut numerator, mut denominator) = (
            numerator.exact_div(&divisor),
            denominator.exact_div(&divisor),
        );
        if denominator.is_negative() {
            numerator = numerator.neg();
            denominator = denominator.neg();
        }
        Self::from_parts(numerator, denominator)
    }

    /// The exact integer `value/1`.
    pub fn from_integer(value: Integer) -> Self {
        Self::from_parts(value, Integer::one())
    }

    /// Reduced signed numerator.
    pub fn numerator(&self) -> &Integer {
        &self.0.numerator
    }

    /// Reduced positive denominator.
    pub fn denominator(&self) -> &Integer {
        &self.0.denominator
    }

    /// Whether the value is an integer.
    pub fn is_integer(&self) -> bool {
        self.0.denominator == Integer::one()
    }

    /// The exact value `self / 10^exponent`.
    pub fn divided_by_power_of_ten(&self, exponent: u64) -> Self {
        let denominator = self.0.denominator.mul(&Integer::power_of_ten(exponent));
        let divisor = self.0.numerator.gcd(&denominator);
        Self::from_parts(
            self.0.numerator.exact_div(&divisor),
            denominator.exact_div(&divisor),
        )
    }

    /// Whether the value is zero.
    pub fn is_zero(&self) -> bool {
        self.0.numerator.is_zero()
    }

    /// Exact `self + other`.
    pub(crate) fn add(&self, other: &Self) -> Self {
        Self::reduce(
            self.0
                .numerator
                .mul(&other.0.denominator)
                .add(&other.0.numerator.mul(&self.0.denominator)),
            self.0.denominator.mul(&other.0.denominator),
        )
    }

    /// Exact `self × other`.
    pub(crate) fn mul(&self, other: &Self) -> Self {
        Self::reduce(
            self.0.numerator.mul(&other.0.numerator),
            self.0.denominator.mul(&other.0.denominator),
        )
    }

    /// Exact `self / other`, or `None` for a zero divisor.
    pub(crate) fn div(&self, other: &Self) -> Option<Self> {
        (!other.is_zero()).then(|| {
            Self::reduce(
                self.0.numerator.mul(&other.0.denominator),
                self.0.denominator.mul(&other.0.numerator),
            )
        })
    }

    /// Exact `self^exponent`, or `None` for zero raised to a negative power.
    /// Callers bound the result size before calling.
    pub(crate) fn pow(&self, exponent: &Integer) -> Option<Self> {
        let (numerator, denominator) = (
            self.0.numerator.pow(exponent),
            self.0.denominator.pow(exponent),
        );
        if exponent.is_negative() {
            (!numerator.is_zero()).then(|| Self::reduce(denominator, numerator))
        } else {
            Some(Self::reduce(numerator, denominator))
        }
    }

    /// The exact value `self / 2^exponent` (IEEE exact conversions). Total: the
    /// power-of-two denominator is never zero.
    pub(crate) fn divided_by_power_of_two(&self, exponent: u64) -> Self {
        let power = Integer::from_big(num_bigint::BigInt::from(1_u8) << exponent);
        Self::reduce(self.0.numerator.clone(), self.0.denominator.mul(&power))
    }

    /// `maxparts(r)` from `quire.value.accounting/v1`.
    pub fn max_part_bits(&self) -> u64 {
        self.0
            .numerator
            .magnitude_bits()
            .max(self.0.denominator.magnitude_bits())
    }

    /// `(a, b)` of this value `a/b`.
    pub(crate) fn parts(&self) -> Parts<'_> {
        (&self.0.numerator, &self.0.denominator)
    }
}

/// `(a, b)` of an operand `a/b`; an integer `n` enters as `(n, 1)`.
pub(crate) type Parts<'a> = (&'a Integer, &'a Integer);

/// A binary operation of the `rational-arithmetic.arithmetic` row of
/// `quire.value.accounting/v1`, shared by rational arithmetic and every
/// `unit.rational-arithmetic` event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RationalArithmetic {
    /// `a/b + c/d`.
    Add,
    /// `a/b - c/d`.
    Subtract,
    /// `a/b × c/d`.
    Multiply,
    /// `a/b ÷ c/d`.
    Divide,
}

impl RationalArithmetic {
    /// The row's `integer_bits = max(N, D)`, derived from operand bit lengths
    /// only: `N = bits(a)+bits(c)`, `D = bits(b)+bits(d)` for `×`;
    /// `N = bits(a)+bits(d)`, `D = bits(b)+bits(c)` for `÷`; and
    /// `N = max(bits(a)+bits(d), bits(c)+bits(b)) + 1`, `D = bits(b)+bits(d)`
    /// for `+` and `-`.
    pub(crate) fn charge_bits(self, (a, b): Parts<'_>, (c, d): Parts<'_>) -> u64 {
        let [a, b, c, d] = [a, b, c, d].map(Integer::magnitude_bits);
        // Every operand is materialized, so each sum of two bit lengths is far
        // below `u64::MAX`.
        let (numerator, denominator) = match self {
            Self::Add | Self::Subtract => (
                a.saturating_add(d)
                    .max(c.saturating_add(b))
                    .saturating_add(1),
                b.saturating_add(d),
            ),
            Self::Multiply => (a.saturating_add(c), b.saturating_add(d)),
            Self::Divide => (a.saturating_add(d), b.saturating_add(c)),
        };
        numerator.max(denominator)
    }

    /// The unreduced intermediate `N/D`: `(a×d ± c×b) / (b×d)`, `(a×c) /
    /// (b×d)` or `(a×d) / (b×c)`. Materialize only after the row is charged.
    pub(crate) fn unreduced(self, (a, b): Parts<'_>, (c, d): Parts<'_>) -> (Integer, Integer) {
        match self {
            Self::Add => (a.mul(d).add(&c.mul(b)), b.mul(d)),
            Self::Subtract => (a.mul(d).sub(&c.mul(b)), b.mul(d)),
            Self::Multiply => (a.mul(c), b.mul(d)),
            Self::Divide => (a.mul(d), b.mul(c)),
        }
    }

    /// The reduced exact result, or `None` for a zero divisor. Materialize only
    /// after the row is charged.
    pub(crate) fn apply(self, left: &Rational, right: &Rational) -> Option<Rational> {
        if self == Self::Divide && right.is_zero() {
            return None;
        }
        let (numerator, denominator) = self.unreduced(left.parts(), right.parts());
        // Every denominator is a product of nonzero parts.
        Some(Rational::reduce(numerator, denominator))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        // Denominators are positive, so cross multiplication preserves order.
        self.0
            .numerator
            .mul(&other.0.denominator)
            .cmp(&other.0.numerator.mul(&self.0.denominator))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.0.numerator, self.0.denominator)
    }
}

/// A grammar-named `Rational[lo, hi; dmin, dmax]` domain: the reduced
/// numerator lies in `[lo, hi]` and the positive denominator in
/// `[dmin, dmax]`.
// Fields behind one `Box` (IR-286), for the reason `CompoundUnit` gives.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct RationalDomain(Box<RationalDomainFields>);

#[derive(Clone, Eq, Hash, PartialEq)]
struct RationalDomainFields {
    numerator: IntegerInterval,
    denominator: IntegerInterval,
}

impl fmt::Debug for RationalDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RationalDomain")
            .field("numerator", &self.0.numerator)
            .field("denominator", &self.0.denominator)
            .finish()
    }
}

/// A rational domain's denominator interval admits a denominator below one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonPositiveDenominatorBound;

impl fmt::Display for NonPositiveDenominatorBound {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("rational denominator bound below one")
    }
}

impl RationalDomain {
    /// Construct the domain, refusing a denominator interval that reaches
    /// below one, since a reduced denominator is always positive.
    pub fn new(
        numerator: IntegerInterval,
        denominator: IntegerInterval,
    ) -> Result<Self, NonPositiveDenominatorBound> {
        if denominator.lower() < &Integer::one() {
            return Err(NonPositiveDenominatorBound);
        }
        Ok(Self(Box::new(RationalDomainFields {
            numerator,
            denominator,
        })))
    }

    /// The reduced-numerator interval.
    pub fn numerator(&self) -> &IntegerInterval {
        &self.0.numerator
    }

    /// The positive-denominator interval.
    pub fn denominator(&self) -> &IntegerInterval {
        &self.0.denominator
    }

    /// Whether the reduced `value` is a member.
    pub fn contains(&self, value: &Rational) -> bool {
        self.0.numerator.contains(value.numerator())
            && self.0.denominator.contains(value.denominator())
    }
}
