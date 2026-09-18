use core::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

use proptest::prelude::*;
use quire_contract_runtime::operators::{
    and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem, checked_sub,
    implies_short_circuit, implies_total, index, option_copied, option_ref, or_short_circuit,
    or_total,
};

const OPERATORS_SOURCE: &str = include_str!("../src/operators.rs");

/// Every `.rs` file under `src/`, recursively, as (path, contents) pairs.
fn all_crate_sources() -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let source = fs::read_to_string(&path).unwrap();
                out.push((path, source));
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
    out
}

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

/// `and_short_circuit`/`or_short_circuit`/`implies_short_circuit` are generic over what `right`
/// returns (`R: From<bool>`): a plain `bool` for an ordinary generated expression, or a
/// stop-carrying type such as `exact::Outcome<bool>` when the right operand may itself be
/// undefined, refused or incomplete. `quire-contract-codegen`'s oracle renderer calls these three
/// functions by name for `BooleanOperator::ShortCircuitAnd`/`ShortCircuitOr`/`Implication`
/// (`agent-ix/quire-contract-runtime#27`), so this crate must be able to carry a stop through them
/// without the caller deciding it outside the connective.
///
/// Trace: TC-002, FR-002-AC-1, FR-002-AC-2
#[cfg(feature = "exact")]
#[test]
fn tc_002_short_circuit_is_generic_over_a_stop_carrying_right_operand() {
    use quire_contract_runtime::exact::{Outcome, Refusal};

    let stop: Outcome<bool> = Outcome::Refused(Refusal::InexactDecimal);

    // The right operand decides the result and stops: the stop returns unchanged.
    assert_eq!(and_short_circuit(true, || stop.clone()), stop);
    assert_eq!(or_short_circuit(false, || stop.clone()), stop);
    assert_eq!(implies_short_circuit(true, || stop.clone()), stop);

    // The left operand alone decides the result: the right thunk is never called, so the stop it
    // would have produced can never arise.
    let calls = Cell::new(0);
    let right = || {
        calls.set(calls.get() + 1);
        stop.clone()
    };
    assert_eq!(and_short_circuit(false, right), Outcome::Completed(false));
    assert_eq!(or_short_circuit(true, right), Outcome::Completed(true));
    assert_eq!(
        implies_short_circuit(false, right),
        Outcome::Completed(true)
    );
    assert_eq!(calls.get(), 0);

    // The right operand decides the result and completes: the plain bool it produces propagates
    // unchanged, wrapped as a completed outcome.
    assert_eq!(
        and_short_circuit(true, || Outcome::Completed(false)),
        Outcome::Completed(false)
    );
    assert_eq!(
        or_short_circuit(false, || Outcome::Completed(true)),
        Outcome::Completed(true)
    );
    assert_eq!(
        implies_short_circuit(true, || Outcome::Completed(false)),
        Outcome::Completed(false)
    );
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
/// supertrait (`sealed::Sealed`, in a module with no `pub`): a downstream crate cannot name
/// `sealed::Sealed` to satisfy the supertrait bound. That is proven at compile time by the
/// `compile_fail` doctest on `CheckedInteger` in `src/operators.rs`
/// (`cargo test --doc`), not by this test — an integration test cannot author a downstream
/// `impl CheckedInteger for ...` to prove the seal holds without failing to build itself.
///
/// What this test checks instead is what a doctest cannot: that nothing in the crate widens the
/// seal (`mod sealed` turning `pub`, or any `pub use` re-exporting out of `sealed`) or hides a
/// thirteenth implementation outside the `checked_integer!` macro invocation. Widening the seal or
/// adding a stray `impl CheckedInteger for Foo` would not change the doctest's pass/fail shape (it
/// would just make the doctest wrong to be `compile_fail` at all — caught separately by `cargo
/// test --doc` turning red) but would slip past a check that only inspects `operators.rs`'s
/// declared shape, so this scans literal occurrences across every crate source file instead.
#[test]
fn tc_003_checked_integer_is_sealed_and_covers_exactly_twelve_types() {
    assert!(OPERATORS_SOURCE.contains("pub trait CheckedInteger: sealed::Sealed + Copy {"));
    assert!(OPERATORS_SOURCE.contains("\nmod sealed {"));
    assert!(OPERATORS_SOURCE.contains(
        "checked_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);"
    ));

    let sources = all_crate_sources();

    // No file may widen `mod sealed` to `pub mod sealed`, or re-export anything out of it: either
    // would let an outside crate name `sealed::Sealed` and defeat the seal, while the checks above
    // (which only read `operators.rs`'s own declared trait and macro invocation) would not change.
    for (path, source) in &sources {
        assert!(
            !source.contains("pub mod sealed"),
            "{} widens `sealed` to `pub mod sealed`",
            path.display()
        );
        for line in source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("pub use") && trimmed.contains("sealed") {
                panic!(
                    "{} re-exports out of `sealed`, defeating CheckedInteger's seal: {trimmed}",
                    path.display()
                );
            }
        }
    }

    // The only place `impl CheckedInteger for` may textually appear in the crate is the
    // `checked_integer!` macro's own template line (it expands once per listed type, but the
    // expansion is not itself present as source text); counting literal occurrences across every
    // file, not just `operators.rs`, catches a stray hand-written thirteenth implementation
    // anywhere in the crate.
    let total_impls: usize = sources
        .iter()
        .map(|(_, source)| source.matches("impl CheckedInteger for").count())
        .sum();
    assert_eq!(
        total_impls, 1,
        "expected exactly one textual `impl CheckedInteger for` (the macro template)"
    );
    let total_sealed_impls: usize = sources
        .iter()
        .map(|(_, source)| source.matches("impl sealed::Sealed for").count())
        .sum();
    assert_eq!(
        total_sealed_impls, 1,
        "expected exactly one textual `impl sealed::Sealed for` (the macro template)"
    );
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
