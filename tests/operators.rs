use core::cell::Cell;

use proptest::prelude::*;
use quire_contract_runtime::operators::{
    and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem, checked_sub,
    implies_short_circuit, implies_total, index, option_copied, option_ref, or_short_circuit,
    or_total,
};

const OPERATORS_SOURCE: &str = include_str!("../src/operators.rs");

/// Exercises every `CheckedInteger` member for `$ty` against the primitive's own `checked_*`,
/// over the pairwise cross product of `$candidate`s. Callers list MIN, MAX, 0, 1, and (for signed
/// types) -1, so the cross product covers overflow, underflow, division/remainder by zero, and
/// (for signed types) MIN divided or remaindered by -1.
macro_rules! assert_checked_matches_primitive {
    ($ty:ty, [$($candidate:expr),+ $(,)?]) => {{
        let candidates: &[$ty] = &[$($candidate),+];
        for &left in candidates {
            for &right in candidates {
                assert_eq!(checked_add(left, right), left.checked_add(right));
                assert_eq!(checked_sub(left, right), left.checked_sub(right));
                assert_eq!(checked_mul(left, right), left.checked_mul(right));
                assert_eq!(checked_div(left, right), left.checked_div(right));
                assert_eq!(checked_rem(left, right), left.checked_rem(right));
            }
        }
    }};
}

/// Trace: TC-002, FR-002-AC-1, FR-002-AC-2
#[test]
fn tc_002_boolean_truth_tables() {
    for left in [false, true] {
        for right in [false, true] {
            assert_eq!(and_short_circuit(left, || right), left && right);
            assert_eq!(or_short_circuit(left, || right), left || right);
            assert_eq!(implies_short_circuit(left, || right), !left || right);
            assert_eq!(and_total(|| left, || right), left & right);
            assert_eq!(or_total(|| left, || right), left | right);
            assert_eq!(implies_total(|| left, || right), !left | right);
        }
    }
}

/// Trace: TC-002, FR-002-AC-1, FR-002-AC-2
#[test]
fn tc_002_evaluation_contracts_are_distinct_and_ordered() {
    let calls = Cell::new(0);
    assert!(!and_short_circuit(false, || {
        calls.set(calls.get() + 1);
        true
    }));
    assert_eq!(calls.get(), 0);

    assert!(or_short_circuit(true, || {
        calls.set(calls.get() + 1);
        false
    }));
    assert_eq!(calls.get(), 0);

    assert!(implies_short_circuit(false, || {
        calls.set(calls.get() + 1);
        false
    }));
    assert_eq!(calls.get(), 0);

    let sequence = Cell::new(0);
    let result = and_total(
        || {
            assert_eq!(sequence.get(), 0);
            sequence.set(1);
            false
        },
        || {
            assert_eq!(sequence.get(), 1);
            sequence.set(2);
            true
        },
    );
    assert!(!result);
    assert_eq!(sequence.get(), 2);

    sequence.set(0);
    let result = or_total(
        || {
            assert_eq!(sequence.get(), 0);
            sequence.set(1);
            true
        },
        || {
            assert_eq!(sequence.get(), 1);
            sequence.set(2);
            false
        },
    );
    assert!(result);
    assert_eq!(sequence.get(), 2);

    sequence.set(0);
    let result = implies_total(
        || {
            assert_eq!(sequence.get(), 0);
            sequence.set(1);
            false
        },
        || {
            assert_eq!(sequence.get(), 1);
            sequence.set(2);
            false
        },
    );
    assert!(result);
    assert_eq!(sequence.get(), 2);
}

/// Trace: TC-003, FR-002-AC-3, NFR-002-AC-1
#[test]
fn tc_003_definedness_boundaries_do_not_panic() {
    let values = [10_u8, 20];
    assert_eq!(option_ref(&Some(3)), Some(&3));
    assert_eq!(option_ref::<u8>(&None), None);
    assert_eq!(option_copied(Some(&7)), Some(7));
    assert_eq!(index(&values, 1), Some(&20));
    assert_eq!(index(&values, 2), None);
    assert_eq!(checked_add(u8::MAX, 1), None);
    assert_eq!(checked_sub(0_u8, 1), None);
    assert_eq!(checked_mul(u16::MAX, 2), None);
    assert_eq!(checked_div(1_i32, 0), None);
    assert_eq!(checked_div(i32::MIN, -1), None);
    assert_eq!(checked_rem(i32::MIN, -1), None);
}

/// Trace: TC-003, FR-002-AC-4
#[test]
fn tc_003_checked_integer_matches_primitive_semantics_for_all_twelve_types() {
    assert_checked_matches_primitive!(u8, [u8::MIN, u8::MAX, 0, 1]);
    assert_checked_matches_primitive!(u16, [u16::MIN, u16::MAX, 0, 1]);
    assert_checked_matches_primitive!(u32, [u32::MIN, u32::MAX, 0, 1]);
    assert_checked_matches_primitive!(u64, [u64::MIN, u64::MAX, 0, 1]);
    assert_checked_matches_primitive!(u128, [u128::MIN, u128::MAX, 0, 1]);
    assert_checked_matches_primitive!(usize, [usize::MIN, usize::MAX, 0, 1]);
    assert_checked_matches_primitive!(i8, [i8::MIN, i8::MAX, 0, 1, -1]);
    assert_checked_matches_primitive!(i16, [i16::MIN, i16::MAX, 0, 1, -1]);
    assert_checked_matches_primitive!(i32, [i32::MIN, i32::MAX, 0, 1, -1]);
    assert_checked_matches_primitive!(i64, [i64::MIN, i64::MAX, 0, 1, -1]);
    assert_checked_matches_primitive!(i128, [i128::MIN, i128::MAX, 0, 1, -1]);
    assert_checked_matches_primitive!(isize, [isize::MIN, isize::MAX, 0, 1, -1]);
}

/// Trace: TC-003, FR-002-AC-4
///
/// `CheckedInteger` cannot be implemented outside this crate because it has a private
/// supertrait (`sealed::Sealed`, in a module with no `pub`). That is a property of the trait
/// definition, not of any value this test could construct and compare, so it is inspected in the
/// source text rather than proven by compiling a positive or negative instance: this crate cannot
/// author a downstream `impl CheckedInteger for ...` inside an integration test to prove the seal
/// holds, and a doc-based `compile_fail` example belongs to the library crate's own docs (already
/// present on `CheckedInteger` in `src/operators.rs`), not to a file under `tests/`.
#[test]
fn tc_003_checked_integer_is_sealed_and_covers_exactly_twelve_types() {
    assert!(OPERATORS_SOURCE.contains("pub trait CheckedInteger: sealed::Sealed + Copy {"));
    assert!(OPERATORS_SOURCE.contains("\nmod sealed {"));
    assert!(!OPERATORS_SOURCE.contains("pub mod sealed"));
    assert!(OPERATORS_SOURCE.contains(
        "checked_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);"
    ));
}

proptest! {
    /// Trace: TC-003, FR-002-AC-3, NFR-002-AC-1
    #[test]
    fn tc_003_checked_i32_helpers_match_primitive_semantics(left: i32, right: i32) {
        prop_assert_eq!(checked_add(left, right), left.checked_add(right));
        prop_assert_eq!(checked_sub(left, right), left.checked_sub(right));
        prop_assert_eq!(checked_mul(left, right), left.checked_mul(right));
        prop_assert_eq!(checked_div(left, right), left.checked_div(right));
        prop_assert_eq!(checked_rem(left, right), left.checked_rem(right));
    }
}
