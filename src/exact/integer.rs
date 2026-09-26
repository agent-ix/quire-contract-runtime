// SPDX-License-Identifier: AGPL-3.0-or-later
//! Mathematical integers and explicit inclusive integer domains
//! (quire-specification/AD-005, quire-specification/FR-147).
//!
//! `Integer` is unbounded. A finite consumer never narrows it: membership in an
//! [`IntegerInterval`] is an explicit admission that either returns a
//! [`BoundedInteger`] or refuses.

use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU32;
use core::str::FromStr;

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer as _;
use num_traits::{One, Signed, Zero};

/// An exact, arbitrary-precision mathematical integer.
///
/// Held inline as an `i64` whenever it fits and promoted to a boxed `BigInt`
/// only when it does not: the representation is canonical (`big` is `Some`
/// exactly when the value does not fit, and `small` is then zero), so
/// equality, hashing and ordering are those of the mathematical value, and no
/// operation here narrows, wraps or saturates. A plain struct, not an enum:
/// CBMC cannot fold a union nested in the union of an enclosing `Value` or
/// `ValueType`, but it folds a struct field there.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Integer {
    small: i64,
    big: Option<Box<BigInt>>,
}

/// A borrowed view of an [`Integer`]'s canonical form, built on the stack.
#[derive(Clone, Copy)]
enum Repr<'a> {
    Small(&'a i64),
    Big(&'a BigInt),
}

impl Integer {
    fn small(value: i64) -> Self {
        Self {
            small: value,
            big: None,
        }
    }

    /// `value` must not fit in `i64` (the canonical form's invariant).
    fn big(value: BigInt) -> Self {
        debug_assert!(
            i64::try_from(&value).is_err(),
            "Integer::big called with a value that fits in i64; every other \
             method assumes big is Some only when the value does not fit"
        );
        Self {
            small: 0,
            big: Some(Box::new(value)),
        }
    }

    fn repr(&self) -> Repr<'_> {
        match &self.big {
            None => Repr::Small(&self.small),
            Some(big) => Repr::Big(big),
        }
    }
}

impl Integer {
    /// The integer zero.
    pub fn zero() -> Self {
        Self::small(0)
    }

    /// The integer one.
    pub fn one() -> Self {
        Self::small(1)
    }

    /// Whether this integer is zero.
    pub fn is_zero(&self) -> bool {
        self.big.is_none() && self.small == 0
    }

    /// Whether this integer is strictly negative.
    pub fn is_negative(&self) -> bool {
        match self.repr() {
            Repr::Small(value) => *value < 0,
            Repr::Big(value) => value.is_negative(),
        }
    }

    /// `bits(x)` from `quire.value.accounting/v1`: the magnitude bit length,
    /// where zero has length one.
    pub fn magnitude_bits(&self) -> u64 {
        match self.repr() {
            Repr::Small(value) => u64::from(
                value
                    .unsigned_abs()
                    .checked_ilog2()
                    .map_or(1, |log| log.saturating_add(1)),
            ),
            Repr::Big(value) => value.bits().max(1),
        }
    }

    /// [`Integer::magnitude_bits`] as an `Integer`. An unpromoted value has at
    /// most 64 bits, so its length is built unpromoted without a range test
    /// (CBMC cannot prove that test's promoting branch unreachable).
    pub(crate) fn magnitude_bits_integer(&self) -> Self {
        match self.repr() {
            Repr::Small(value) => Self::small(i64::from(
                value
                    .unsigned_abs()
                    .checked_ilog2()
                    .map_or(1, |log| log.saturating_add(1)),
            )),
            Repr::Big(value) => Self::from(value.bits().max(1)),
        }
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
        match self.repr() {
            Repr::Small(value) => u64::try_from(*value).ok(),
            Repr::Big(value) => u64::try_from(value).ok(),
        }
    }

    /// The magnitude `|self|`.
    pub(crate) fn abs(&self) -> Self {
        match self.repr() {
            Repr::Small(value) => match value.checked_abs() {
                Some(magnitude) => Self::small(magnitude),
                None => Self::from_big(BigInt::from(*value).abs()),
            },
            Repr::Big(value) => Self::from_big(value.abs()),
        }
    }

    /// `self^|exponent|`. Callers bound the result size before calling.
    pub(crate) fn pow(&self, exponent: &Self) -> Self {
        Self::from_big(num_traits::Pow::pow(
            &*self.as_big(),
            exponent.as_big().magnitude(),
        ))
    }

    /// `(self / 2^k, k)` for the greatest `k <= limit` with `2^k | self`.
    /// Zero has no greatest such `k` and is returned with `k = 0`.
    pub(crate) fn split_factor_two(&self, limit: u64) -> (Self, u64) {
        let value = self.as_big();
        match value.trailing_zeros() {
            None => (self.clone(), 0),
            Some(zeros) => {
                let shift = zeros.min(limit);
                // `shift <= zeros < bits(self)`, an in-memory length.
                (Self::from_big(&*value >> shift), shift)
            }
        }
    }

    /// `self × 2^shift`. Callers bound the result size before calling.
    pub(crate) fn shifted_left(&self, shift: u64) -> Self {
        Self::from_big(&*self.as_big() << shift)
    }

    /// Whether this integer is even.
    pub fn is_even(&self) -> bool {
        match self.repr() {
            Repr::Small(value) => value % 2 == 0,
            Repr::Big(value) => value.is_even(),
        }
    }

    // The `+`, `-` or `*` of two `i64` operands (and `-` of one) always fits
    // in `i128`, so the `wrapping_*` forms below never wrap and are exact, and
    // the small path never reaches `BigInt` arithmetic (CBMC cannot bound
    // `BigInt`'s digit loops on the overflow branch it cannot prove
    // unreachable).
    pub(crate) fn add(&self, other: &Self) -> Self {
        if let (Repr::Small(left), Repr::Small(right)) = (self.repr(), other.repr()) {
            return Self::from(i128::from(*left).wrapping_add(i128::from(*right)));
        }
        Self::from_big(&*self.as_big() + &*other.as_big())
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        if let (Repr::Small(left), Repr::Small(right)) = (self.repr(), other.repr()) {
            return Self::from(i128::from(*left).wrapping_sub(i128::from(*right)));
        }
        Self::from_big(&*self.as_big() - &*other.as_big())
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        if let (Repr::Small(left), Repr::Small(right)) = (self.repr(), other.repr()) {
            return Self::from(i128::from(*left).wrapping_mul(i128::from(*right)));
        }
        Self::from_big(&*self.as_big() * &*other.as_big())
    }

    pub(crate) fn neg(&self) -> Self {
        if let Repr::Small(value) = self.repr() {
            return Self::from(i128::from(*value).wrapping_neg());
        }
        Self::from_big(-&*self.as_big())
    }

    pub(crate) fn gcd(&self, other: &Self) -> Self {
        Self::from_big(self.as_big().gcd(&other.as_big()))
    }

    /// Exact quotient of a division known to be exact; `divisor` is nonzero.
    pub(crate) fn exact_div(&self, divisor: &Self) -> Self {
        Self::from_big(&*self.as_big() / &*divisor.as_big())
    }

    /// Truncating quotient/remainder; `divisor` is nonzero.
    pub(crate) fn div_rem_truncating(&self, divisor: &Self) -> (Self, Self) {
        let (quotient, remainder) = self.as_big().div_rem(&divisor.as_big());
        (Self::from_big(quotient), Self::from_big(remainder))
    }

    /// Floor quotient/remainder; `divisor` is nonzero.
    pub(crate) fn div_mod_floor(&self, divisor: &Self) -> (Self, Self) {
        let (quotient, remainder) = self.as_big().div_mod_floor(&divisor.as_big());
        (Self::from_big(quotient), Self::from_big(remainder))
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
        Self::from_big(result)
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
        let (factor, base, exponent) = (factor.as_big(), base.as_big(), exponent.as_big());
        let factor = factor.magnitude();
        let base = base.magnitude();
        let exponent = exponent.magnitude();
        let factor_bits = BigUint::from(factor.bits().max(1));
        if factor.is_zero() || base.is_zero() && !exponent.is_zero() {
            return Self::one();
        }
        if exponent.is_zero() || base.is_one() {
            return Self::from_big(BigInt::from(factor_bits));
        }
        if base.count_ones() == 1 {
            let shift = BigUint::from(base.bits().saturating_sub(1));
            return Self::from_big(BigInt::from(factor_bits + shift * exponent));
        }
        let mut precision = 64_u64;
        loop {
            let (low, high) = bracket_bits(factor, base, exponent, precision);
            if low == high {
                return Self::from_big(BigInt::from(low));
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
        let (other, shift) = (other.as_big(), shift.as_big());
        let right_bits = Self::from_big(BigInt::from(other.bits()) + &*shift);
        if left_bits != right_bits {
            return left_bits.cmp(&right_bits);
        }
        let other = other.magnitude();
        let (factor, base, exponent) = (factor.as_big(), base.as_big(), exponent.as_big());
        let mut precision = 64_u64;
        loop {
            let (low, high, low_shift) = bracket(
                factor.magnitude(),
                base.magnitude(),
                exponent.magnitude(),
                precision,
            );
            // Equal bit lengths keep `|low_shift - shift|` within the bit
            // lengths of `high` and `other`, so the aligned sides are small.
            let gap = BigInt::from(low_shift) - &*shift;
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
        Self::from_big(BigInt::one() << u64::from(exponent))
    }
}

/// `log10 2` lies strictly between these numerators over
/// [`LOG10_TWO_DENOMINATOR`]. Both stay below `2^62`, so a `u64` bit length
/// times either fits in `u128`.
const LOG10_TWO_LOWER: u128 = 3_010_299_956_639_811_952;
const LOG10_TWO_UPPER: u128 = 3_010_299_956_639_811_953;
const LOG10_TWO_DENOMINATOR: u128 = 10_000_000_000_000_000_000;

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
        Self::small(value)
    }
}

impl From<i128> for Integer {
    fn from(value: i128) -> Self {
        match i64::try_from(value) {
            Ok(small) => Self::small(small),
            Err(_) => Self::big(big_from_i128(value)),
        }
    }
}

/// `BigInt::from(value)` over four fixed 32-bit digits:
/// `BigUint::from(u128)` loops while the remaining value is nonzero, and CBMC
/// cannot bound that loop for a symbolic `value` on a promoting branch it
/// cannot prove unreachable.
fn big_from_i128(value: i128) -> BigInt {
    let magnitude = value.unsigned_abs();
    let digits = [0_u32, 32, 64, 96].map(|shift| (magnitude >> shift) as u32);
    let sign = if value < 0 { Sign::Minus } else { Sign::Plus };
    BigInt::from_biguint(sign, BigUint::from_slice(&digits))
}

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        match i64::try_from(value) {
            Ok(small) => Self::small(small),
            Err(_) => Self::big(BigInt::from(value)),
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self::zero()
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        // The form is canonical, so a promoted value lies outside `i64` and its
        // sign alone orders it against an unpromoted one: no digit comparison
        // is needed.
        match (self.repr(), other.repr()) {
            (Repr::Small(left), Repr::Small(right)) => left.cmp(right),
            (Repr::Small(_), Repr::Big(right)) => {
                if right.is_negative() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Repr::Big(left), Repr::Small(_)) => {
                if left.is_negative() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Repr::Big(left), Repr::Big(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `Integer(<decimal>)`, exactly as the former derived `BigInt`-backed form
/// rendered.
impl fmt::Debug for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Integer")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr() {
            Repr::Small(value) => fmt::Display::fmt(value, formatter),
            Repr::Big(value) => fmt::Display::fmt(value, formatter),
        }
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
            .map(Self::from_big)
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
#[non_exhaustive]
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
    /// Wrap an arbitrary-precision integer, held inline when it fits in
    /// `i64` (the representation's canonical form).
    pub(crate) fn from_big(value: BigInt) -> Self {
        match i64::try_from(&value) {
            Ok(small) => Self::small(small),
            Err(_) => Self::big(value),
        }
    }

    /// The arbitrary-precision integer, borrowed when already promoted.
    pub(crate) fn as_big(&self) -> Cow<'_, BigInt> {
        match self.repr() {
            Repr::Small(value) => Cow::Owned(BigInt::from(*value)),
            Repr::Big(value) => Cow::Borrowed(value),
        }
    }
}

#[cfg(test)]
#[path = "../exact_integer_tests.rs"]
mod tests;
