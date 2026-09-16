// SPDX-License-Identifier: AGPL-3.0-or-later
//! Mathematical integers and explicit inclusive integer domains (AD-005, FR-147).
//!
//! `Integer` is unbounded. A finite consumer never narrows it: membership in an
//! [`IntegerInterval`] is an explicit admission that either returns a
//! [`BoundedInteger`] or refuses.

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU32;
use core::str::FromStr;

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer as _;
use num_traits::{One, Signed, Zero};

/// An exact, arbitrary-precision mathematical integer.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Integer(BigInt);

impl Integer {
    /// The integer zero.
    pub fn zero() -> Self {
        Self(BigInt::zero())
    }

    /// The integer one.
    pub fn one() -> Self {
        Self(BigInt::one())
    }

    /// Whether this integer is zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Whether this integer is strictly negative.
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    /// `bits(x)` from `quire.value.accounting/v1`: the magnitude bit length,
    /// where zero has length one.
    pub fn magnitude_bits(&self) -> u64 {
        self.0.bits().max(1)
    }

    /// `digits(x)` from `quire.value.accounting/v1`: the base-ten magnitude
    /// digit count, where zero has one digit.
    ///
    /// Derived from the bit length without rendering the value: `2^(b-1) <= |x|
    /// < 2^b` places `digits(x)` between `⌊(b-1)·log10 2⌋ + 1` and
    /// `⌊b·log10 2⌋ + 1`, taken with a lower and an upper rational bound on
    /// `log10 2`. Each candidate above the lower end is corrected by an analytic
    /// comparison with `10^(d-1)`.
    pub fn decimal_digits(&self) -> u64 {
        let bits = self.magnitude_bits();
        let scaled = |bits: u64, log10_two: u128| {
            let floor = u128::from(bits).saturating_mul(log10_two) / LOG10_TWO_DENOMINATOR;
            u64::try_from(floor).unwrap_or(u64::MAX).saturating_add(1)
        };
        let lowest = scaled(bits.saturating_sub(1), LOG10_TWO_LOWER);
        let mut digits = scaled(bits, LOG10_TWO_UPPER);
        let (one, ten, zero) = (Self::one(), Self::from(10_u64), Self::zero());
        while digits > lowest {
            let exponent = Self::from(digits.saturating_sub(1));
            if Self::compare_power_product(&one, &ten, &exponent, self, &zero) != Ordering::Greater
            {
                break;
            }
            digits = digits.saturating_sub(1);
        }
        digits
    }

    /// The value as a `u64`, if it is one.
    pub fn to_u64(&self) -> Option<u64> {
        u64::try_from(&self.0).ok()
    }

    /// The magnitude `|self|`.
    pub(crate) fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// `self^|exponent|`. Callers bound the result size before calling.
    pub(crate) fn pow(&self, exponent: &Self) -> Self {
        Self(num_traits::Pow::pow(&self.0, exponent.0.magnitude()))
    }

    /// `(self / 2^k, k)` for the greatest `k <= limit` with `2^k | self`.
    /// Zero has no greatest such `k` and is returned with `k = 0`.
    pub(crate) fn split_factor_two(&self, limit: u64) -> (Self, u64) {
        match self.0.trailing_zeros() {
            None => (self.clone(), 0),
            Some(zeros) => {
                let shift = zeros.min(limit);
                // `shift <= zeros < bits(self)`, an in-memory length.
                (Self(&self.0 >> shift), shift)
            }
        }
    }

    /// `self × 2^shift`. Callers bound the result size before calling.
    pub(crate) fn shifted_left(&self, shift: u64) -> Self {
        Self(&self.0 << shift)
    }

    /// Whether this integer is even.
    pub fn is_even(&self) -> bool {
        self.0.is_even()
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        Self(&self.0 + &other.0)
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        Self(&self.0 - &other.0)
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        Self(&self.0 * &other.0)
    }

    pub(crate) fn neg(&self) -> Self {
        Self(-&self.0)
    }

    pub(crate) fn gcd(&self, other: &Self) -> Self {
        Self(self.0.gcd(&other.0))
    }

    /// Exact quotient of a division known to be exact; `divisor` is nonzero.
    pub(crate) fn exact_div(&self, divisor: &Self) -> Self {
        Self(&self.0 / &divisor.0)
    }

    /// Truncating quotient/remainder; `divisor` is nonzero.
    pub(crate) fn div_rem_truncating(&self, divisor: &Self) -> (Self, Self) {
        let (quotient, remainder) = self.0.div_rem(&divisor.0);
        (Self(quotient), Self(remainder))
    }

    /// Floor quotient/remainder; `divisor` is nonzero.
    pub(crate) fn div_mod_floor(&self, divisor: &Self) -> (Self, Self) {
        let (quotient, remainder) = self.0.div_mod_floor(&divisor.0);
        (Self(quotient), Self(remainder))
    }

    /// Exact `10^exponent`.
    pub(crate) fn power_of_ten(exponent: u64) -> Self {
        let mut result = BigInt::one();
        let mut base = BigInt::from(10_u8);
        let mut remaining = exponent;
        while remaining > 0 {
            if remaining & 1 == 1 {
                result *= &base;
            }
            remaining >>= 1;
            if remaining > 0 {
                base = &base * &base;
            }
        }
        Self(result)
    }

    /// `bits(|factor| × |base|^exponent)` for a nonnegative `exponent`, derived
    /// without materializing the power.
    ///
    /// A power-of-two base is exact by shifting. Otherwise the power is bracketed
    /// by truncated lower and upper `P`-bit mantissas under a shared binary
    /// exponent, and `P` doubles until both brackets have one bit length. The
    /// product is then not a power of two, so a finite precision separates it
    /// from the nearest power of two and the loop terminates.
    pub(crate) fn power_product_bits(factor: &Self, base: &Self, exponent: &Self) -> Self {
        let factor = factor.0.magnitude();
        let base = base.0.magnitude();
        let exponent = exponent.0.magnitude();
        let factor_bits = BigUint::from(factor.bits().max(1));
        if factor.is_zero() || base.is_zero() && !exponent.is_zero() {
            return Self::one();
        }
        if exponent.is_zero() || base.is_one() {
            return Self(BigInt::from(factor_bits));
        }
        if base.count_ones() == 1 {
            let shift = BigUint::from(base.bits().saturating_sub(1));
            return Self(BigInt::from(factor_bits + shift * exponent));
        }
        let mut precision = 64_u64;
        loop {
            let (low, high) = bracket_bits(factor, base, exponent, precision);
            if low == high {
                return Self(BigInt::from(low));
            }
            precision = precision.saturating_mul(2);
        }
    }

    /// Compare `|factor| × |base|^exponent` with `|other| × 2^shift` for a
    /// nonnegative `exponent` and a signed `shift`, without materializing the
    /// power.
    ///
    /// Unequal bit lengths decide at once. Otherwise the power is bracketed as
    /// in [`Integer::power_product_bits`] and the precision doubles until the
    /// bracket excludes the other side or collapses to the exact value.
    pub(crate) fn compare_power_product(
        factor: &Self,
        base: &Self,
        exponent: &Self,
        other: &Self,
        shift: &Self,
    ) -> Ordering {
        let left_zero = factor.is_zero() || base.is_zero() && !exponent.is_zero();
        match (left_zero, other.is_zero()) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => {}
        }
        let left_bits = Self::power_product_bits(factor, base, exponent);
        let right_bits = Self(BigInt::from(other.0.bits()) + &shift.0);
        if left_bits != right_bits {
            return left_bits.cmp(&right_bits);
        }
        let other = other.0.magnitude();
        let mut precision = 64_u64;
        loop {
            let (low, high, low_shift) = bracket(
                factor.0.magnitude(),
                base.0.magnitude(),
                exponent.0.magnitude(),
                precision,
            );
            // Equal bit lengths keep `|low_shift - shift|` within the bit
            // lengths of `high` and `other`, so the aligned sides are small.
            let gap = BigInt::from(low_shift) - &shift.0;
            let Ok(distance) = u64::try_from(gap.magnitude()) else {
                // Unreachable: equal bit lengths bound the alignment gap by
                // in-memory mantissa bit lengths. A gap beyond `u64` would put
                // the side with the smaller shift below the other.
                return if gap.is_negative() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            };
            let aligned = |mantissa: &BigUint| {
                if gap.is_negative() {
                    (mantissa.clone(), other << distance)
                } else {
                    (mantissa << distance, other.clone())
                }
            };
            let (high_side, other_side) = aligned(&high);
            if high_side < other_side {
                return Ordering::Less;
            }
            let (low_side, other_side) = aligned(&low);
            if low_side > other_side {
                return Ordering::Greater;
            }
            if low == high {
                return low_side.cmp(&other_side);
            }
            precision = precision.saturating_mul(2);
        }
    }

    /// `2^exponent`.
    fn power_of_two(exponent: u32) -> Self {
        Self(BigInt::one() << u64::from(exponent))
    }
}

/// `log10 2` lies strictly between these numerators over
/// [`LOG10_TWO_DENOMINATOR`]. Both stay below `2^62`, so a `u64` bit length
/// times either fits in `u128`.
const LOG10_TWO_LOWER: u128 = 3_010_299_956_639_811_952;
const LOG10_TWO_UPPER: u128 = 3_010_299_956_639_811_953;
const LOG10_TWO_DENOMINATOR: u128 = 10_000_000_000_000_000_000;

/// A signed dyadic interval `[low, high] × 2^shift` enclosing one exact integer
/// expression, each mantissa kept to about `precision` bits.
///
/// It sizes a sum, difference or product before the value exists: operands
/// enter truncated (the lower end rounded down, the upper end up), and every
/// operation keeps the enclosure. With no truncation the interval is the exact
/// value, so [`exact_bits`] terminates once `precision` covers the widest
/// intermediate. That last round is the only one that can allocate as much as
/// the value itself, and only for a result within one bit of a power of two or
/// a near-total cancellation.
#[derive(Clone, Debug)]
pub(crate) struct Dyadic {
    low: BigInt,
    high: BigInt,
    shift: u64,
    precision: u64,
}

impl Dyadic {
    fn truncated(low: BigInt, high: BigInt, shift: u64, precision: u64) -> Self {
        let excess = low.bits().max(high.bits()).saturating_sub(precision);
        if excess == 0 {
            return Self {
                low,
                high,
                shift,
                precision,
            };
        }
        Self {
            low: floor_shift(&low, excess),
            high: ceil_shift(&high, excess),
            shift: shift.saturating_add(excess),
            precision,
        }
    }

    /// An enclosure of `self + other`.
    pub(crate) fn add(&self, other: &Self) -> Self {
        let shift = self.shift.max(other.shift);
        let (left, right) = (
            shift.saturating_sub(self.shift),
            shift.saturating_sub(other.shift),
        );
        Self::truncated(
            floor_shift(&self.low, left) + floor_shift(&other.low, right),
            ceil_shift(&self.high, left) + ceil_shift(&other.high, right),
            shift,
            self.precision,
        )
    }

    /// An enclosure of `self - other`.
    pub(crate) fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// An enclosure of `-self`.
    pub(crate) fn neg(&self) -> Self {
        Self {
            low: -&self.high,
            high: -&self.low,
            shift: self.shift,
            precision: self.precision,
        }
    }

    /// An enclosure of `self × other`.
    pub(crate) fn mul(&self, other: &Self) -> Self {
        let products = [
            &self.low * &other.low,
            &self.low * &other.high,
            &self.high * &other.low,
            &self.high * &other.high,
        ];
        let low = products.iter().min().cloned().unwrap_or_default();
        let high = products.iter().max().cloned().unwrap_or_default();
        Self::truncated(
            low,
            high,
            self.shift.saturating_add(other.shift),
            self.precision,
        )
    }

    /// `bits(value)`, when the enclosure decides it.
    fn bits(&self) -> Option<u64> {
        let same = |small: &BigInt, large: &BigInt| {
            (small.bits() == large.bits()).then(|| small.bits().saturating_add(self.shift))
        };
        match (self.low.sign(), self.high.sign()) {
            (Sign::NoSign, Sign::NoSign) => Some(1),
            (Sign::Plus, Sign::Plus) => same(&self.low, &self.high),
            (Sign::Minus, Sign::Minus) => same(&self.high, &self.low),
            _ => None,
        }
    }
}

/// `⌊value / 2^shift⌋`.
fn floor_shift(value: &BigInt, shift: u64) -> BigInt {
    value >> shift
}

/// `⌈value / 2^shift⌉`, without negating the full value.
fn ceil_shift(value: &BigInt, shift: u64) -> BigInt {
    let floor = value >> shift;
    if value.trailing_zeros().is_some_and(|zeros| zeros < shift) {
        floor + 1_u8
    } else {
        floor
    }
}

/// The exact `bits` of the expression `enclose` builds at a given precision,
/// derived before the expression's value is materialized. Precision starts at
/// 64 bits and doubles until the enclosure decides.
pub(crate) fn exact_bits(enclose: impl Fn(u64) -> Dyadic) -> u64 {
    let mut precision = 64_u64;
    loop {
        if let Some(bits) = enclose(precision).bits() {
            return bits;
        }
        precision = precision.saturating_mul(2);
    }
}

impl Integer {
    /// This integer entered into a [`Dyadic`] enclosure at `precision`.
    pub(crate) fn dyadic(&self, precision: u64) -> Dyadic {
        let excess = self.0.bits().saturating_sub(precision);
        Dyadic::truncated(
            floor_shift(&self.0, excess),
            ceil_shift(&self.0, excess),
            excess,
            precision,
        )
    }
}

/// Bit lengths of a lower and upper bound of `factor × base^exponent`, each kept
/// to at most `precision` mantissa bits.
fn bracket_bits(
    factor: &BigUint,
    base: &BigUint,
    exponent: &BigUint,
    precision: u64,
) -> (BigUint, BigUint) {
    let (low, high, shift) = bracket(factor, base, exponent, precision);
    (
        BigUint::from(low.bits()) + &shift,
        BigUint::from(high.bits()) + shift,
    )
}

/// `(low, high, shift)` with `low × 2^shift <= factor × base^exponent <=
/// high × 2^shift`, the power's mantissas kept to at most `precision` bits.
/// With no truncation `low == high` is the exact value and `shift` is zero.
fn bracket(
    factor: &BigUint,
    base: &BigUint,
    exponent: &BigUint,
    precision: u64,
) -> (BigUint, BigUint, BigUint) {
    let mut low = BigUint::one();
    let mut high = BigUint::one();
    let mut shift = BigUint::zero();
    let truncate = |low: &mut BigUint, high: &mut BigUint, shift: &mut BigUint| {
        let excess = high.bits().saturating_sub(precision);
        if excess > 0 {
            *low >>= excess;
            *high = (&*high + ((BigUint::one() << excess) - 1_u8)) >> excess;
            *shift += excess;
        }
    };
    for bit in (0..exponent.bits()).rev() {
        low = &low * &low;
        high = &high * &high;
        shift = &shift << 1_u8;
        truncate(&mut low, &mut high, &mut shift);
        if exponent.bit(bit) {
            low *= base;
            high *= base;
            truncate(&mut low, &mut high, &mut shift);
        }
    }
    low *= factor;
    high *= factor;
    (low, high, shift)
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self(BigInt::from(value))
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        Self(BigInt::from(value))
    }
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Self(BigInt::from(value))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

/// The canonical integer spelling `^(0|-?[1-9][0-9]*)$` was not supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonCanonicalInteger;

impl fmt::Display for NonCanonicalInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("non-canonical integer spelling")
    }
}

impl FromStr for Integer {
    type Err = NonCanonicalInteger;

    /// Parse the canonical wire spelling used by complete-V1 schemas.
    fn from_str(spelling: &str) -> Result<Self, Self::Err> {
        let digits = spelling.strip_prefix('-').unwrap_or(spelling);
        let canonical = match digits.as_bytes() {
            [] => false,
            [b'0'] => digits.len() == spelling.len(),
            [first, rest @ ..] => {
                (b'1'..=b'9').contains(first) && rest.iter().all(u8::is_ascii_digit)
            }
        };
        if !canonical {
            return Err(NonCanonicalInteger);
        }
        BigInt::from_str(spelling)
            .map(Self)
            .map_err(|_| NonCanonicalInteger)
    }
}

/// A nonempty inclusive integer domain `[lower, upper]`.
///
/// This is the finite domain descriptor consumed by bounded backends; it is
/// never inferred from a host integer width.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerInterval {
    lower: Integer,
    upper: Integer,
}

/// An interval's lower bound exceeds its upper bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmptyInterval;

impl fmt::Display for EmptyInterval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("empty integer interval")
    }
}

impl IntegerInterval {
    /// Construct `[lower, upper]`, refusing an empty interval.
    pub fn new(lower: Integer, upper: Integer) -> Result<Self, EmptyInterval> {
        if lower > upper {
            return Err(EmptyInterval);
        }
        Ok(Self { lower, upper })
    }

    /// The two's-complement signed domain of `width` bits,
    /// `[-(2^(width-1)), 2^(width-1) - 1]`.
    pub fn signed_twos_complement(width: NonZeroU32) -> Self {
        let magnitude = Integer::power_of_two(width.get().saturating_sub(1));
        Self {
            lower: magnitude.neg(),
            upper: magnitude.sub(&Integer::one()),
        }
    }

    /// Inclusive lower bound.
    pub fn lower(&self) -> &Integer {
        &self.lower
    }

    /// Inclusive upper bound.
    pub fn upper(&self) -> &Integer {
        &self.upper
    }

    /// Whether `value` is a member.
    pub fn contains(&self, value: &Integer) -> bool {
        &self.lower <= value && value <= &self.upper
    }

    /// Admit `value` into this domain without narrowing or saturation.
    pub fn admit(&self, value: Integer) -> Result<BoundedInteger, OutOfDomain> {
        if self.contains(&value) {
            Ok(BoundedInteger {
                value,
                domain: self.clone(),
            })
        } else {
            Err(OutOfDomain)
        }
    }
}

/// A value is outside its declared domain. The value is not retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfDomain;

impl fmt::Display for OutOfDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("integer outside declared domain")
    }
}

/// An integer admitted into an explicit inclusive domain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedInteger {
    value: Integer,
    domain: IntegerInterval,
}

impl BoundedInteger {
    /// The admitted mathematical value.
    pub fn value(&self) -> &Integer {
        &self.value
    }

    /// The domain that admitted it.
    pub fn domain(&self) -> &IntegerInterval {
        &self.domain
    }
}

/// The domain of an integer consumer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerDomain {
    /// Unbounded mathematical integers; evaluation has no overflow.
    Mathematical,
    /// A finite consumer that must prove membership of every exposed result.
    Bounded(IntegerInterval),
}

impl IntegerDomain {
    /// Whether `value` belongs to this domain.
    pub fn contains(&self, value: &Integer) -> bool {
        match self {
            Self::Mathematical => true,
            Self::Bounded(interval) => interval.contains(value),
        }
    }
}

impl Integer {
    /// Wrap an arbitrary-precision integer (IEEE exact conversions).
    pub(crate) fn from_big(value: BigInt) -> Self {
        Self(value)
    }

    /// The arbitrary-precision integer (IEEE exact conversions).
    pub(crate) fn as_big(&self) -> &BigInt {
        &self.0
    }
}
