use core::cell::Cell;

use proptest::prelude::*;
use quire_contract_runtime::operators::{
    and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem, checked_sub,
    implies_short_circuit, implies_total, index, option_copied, option_ref, or_short_circuit,
    or_total,
};

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

proptest! {
    #[test]
    fn tc_003_checked_i32_helpers_match_primitive_semantics(left: i32, right: i32) {
        prop_assert_eq!(checked_add(left, right), left.checked_add(right));
        prop_assert_eq!(checked_sub(left, right), left.checked_sub(right));
        prop_assert_eq!(checked_mul(left, right), left.checked_mul(right));
        prop_assert_eq!(checked_div(left, right), left.checked_div(right));
        prop_assert_eq!(checked_rem(left, right), left.checked_rem(right));
    }
}
