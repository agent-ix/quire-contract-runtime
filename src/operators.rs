//! Safe operator families used by generated oracles.

/// Short-circuit conjunction. The right operand is skipped when `left` is false.
///
/// `R` carries whatever `right` returns: plain `bool` for an ordinary generated expression, or a
/// stop-carrying type such as `exact::Outcome<bool>` (via its `From<bool>` impl) when the right
/// operand may itself be undefined, refused or incomplete. When `left` alone decides the result,
/// `right` is never called, so a stop it could have produced can never arise.
// Implements: FR-002
#[inline]
pub fn and_short_circuit<R: From<bool>>(left: bool, right: impl FnOnce() -> R) -> R {
    if left {
        right()
    } else {
        R::from(false)
    }
}

/// Short-circuit disjunction. The right operand is skipped when `left` is true.
///
/// See [`and_short_circuit`] for what `R` carries and the short-circuit-implies-unreached
/// guarantee.
// Implements: FR-002
#[inline]
pub fn or_short_circuit<R: From<bool>>(left: bool, right: impl FnOnce() -> R) -> R {
    if left {
        R::from(true)
    } else {
        right()
    }
}

/// Short-circuit implication. The consequent is skipped when the antecedent is false.
///
/// See [`and_short_circuit`] for what `R` carries and the short-circuit-implies-unreached
/// guarantee.
// Implements: FR-002
#[inline]
pub fn implies_short_circuit<R: From<bool>>(antecedent: bool, consequent: impl FnOnce() -> R) -> R {
    if antecedent {
        consequent()
    } else {
        R::from(true)
    }
}

/// Total conjunction. Evaluates each operand exactly once, from left to right.
// Implements: FR-002
#[inline]
pub fn and_total(left: impl FnOnce() -> bool, right: impl FnOnce() -> bool) -> bool {
    let left_value = left();
    let right_value = right();
    left_value & right_value
}

/// Total disjunction. Evaluates each operand exactly once, from left to right.
// Implements: FR-002
#[inline]
pub fn or_total(left: impl FnOnce() -> bool, right: impl FnOnce() -> bool) -> bool {
    let left_value = left();
    let right_value = right();
    left_value | right_value
}

/// Total implication. Evaluates antecedent then consequent exactly once each.
// Implements: FR-002
#[inline]
pub fn implies_total(antecedent: impl FnOnce() -> bool, consequent: impl FnOnce() -> bool) -> bool {
    let antecedent_value = antecedent();
    let consequent_value = consequent();
    !antecedent_value | consequent_value
}

/// Returns a borrowed value only when an option is defined.
// Implements: FR-002
#[inline]
#[must_use]
pub const fn option_ref<T>(value: &Option<T>) -> Option<&T> {
    value.as_ref()
}

/// Returns a copied value only when an option is defined.
// Implements: FR-002
#[inline]
#[must_use]
pub const fn option_copied<T: Copy>(value: Option<&T>) -> Option<T> {
    match value {
        Some(inner) => Some(*inner),
        None => None,
    }
}

/// Returns a borrowed slice element only when the index is in bounds.
// Implements: FR-002
#[inline]
#[must_use]
pub fn index<T>(values: &[T], at: usize) -> Option<&T> {
    values.get(at)
}

mod sealed {
    pub trait Sealed {}
}

/// Integer operations with Rust's defined checked semantics.
///
/// This trait is sealed so generated code cannot provide a panicking implementation:
/// `sealed::Sealed` lives in a module with no `pub`, so a downstream crate cannot name it to
/// satisfy this trait's supertrait bound.
///
/// ```compile_fail
/// #[derive(Clone, Copy)]
/// struct Custom;
///
/// impl quire_contract_runtime::operators::CheckedInteger for Custom {
///     fn checked_add(self, _right: Self) -> Option<Self> { None }
///     fn checked_sub(self, _right: Self) -> Option<Self> { None }
///     fn checked_mul(self, _right: Self) -> Option<Self> { None }
///     fn checked_div(self, _right: Self) -> Option<Self> { None }
///     fn checked_rem(self, _right: Self) -> Option<Self> { None }
/// }
/// ```
// Implements: FR-002
pub trait CheckedInteger: sealed::Sealed + Copy {
    /// Checked addition.
    // Implements: FR-002
    fn checked_add(self, right: Self) -> Option<Self>;
    /// Checked subtraction.
    // Implements: FR-002
    fn checked_sub(self, right: Self) -> Option<Self>;
    /// Checked multiplication.
    // Implements: FR-002
    fn checked_mul(self, right: Self) -> Option<Self>;
    /// Checked division, including zero and signed overflow checks.
    // Implements: FR-002
    fn checked_div(self, right: Self) -> Option<Self>;
    /// Checked remainder, including zero and signed overflow checks.
    // Implements: FR-002
    fn checked_rem(self, right: Self) -> Option<Self>;
}

macro_rules! checked_integer {
    ($($integer:ty),+ $(,)?) => {
        $(
            impl sealed::Sealed for $integer {}

            impl CheckedInteger for $integer {
                #[inline]
                fn checked_add(self, right: Self) -> Option<Self> {
                    <$integer>::checked_add(self, right)
                }

                #[inline]
                fn checked_sub(self, right: Self) -> Option<Self> {
                    <$integer>::checked_sub(self, right)
                }

                #[inline]
                fn checked_mul(self, right: Self) -> Option<Self> {
                    <$integer>::checked_mul(self, right)
                }

                #[inline]
                fn checked_div(self, right: Self) -> Option<Self> {
                    <$integer>::checked_div(self, right)
                }

                #[inline]
                fn checked_rem(self, right: Self) -> Option<Self> {
                    <$integer>::checked_rem(self, right)
                }
            }
        )+
    };
}

checked_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// Returns the sum when representable.
// Implements: FR-002
#[inline]
#[must_use]
pub fn checked_add<T: CheckedInteger>(left: T, right: T) -> Option<T> {
    left.checked_add(right)
}

/// Returns the difference when representable.
// Implements: FR-002
#[inline]
#[must_use]
pub fn checked_sub<T: CheckedInteger>(left: T, right: T) -> Option<T> {
    left.checked_sub(right)
}

/// Returns the product when representable.
// Implements: FR-002
#[inline]
#[must_use]
pub fn checked_mul<T: CheckedInteger>(left: T, right: T) -> Option<T> {
    left.checked_mul(right)
}

/// Returns the quotient when division is defined and representable.
// Implements: FR-002
#[inline]
#[must_use]
pub fn checked_div<T: CheckedInteger>(left: T, right: T) -> Option<T> {
    left.checked_div(right)
}

/// Returns the remainder when division is defined and representable.
// Implements: FR-002
#[inline]
#[must_use]
pub fn checked_rem<T: CheckedInteger>(left: T, right: T) -> Option<T> {
    left.checked_rem(right)
}
