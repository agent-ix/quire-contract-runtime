#![cfg(kani)]

use quire_contract_runtime::operators::{
    and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem, checked_sub,
    implies_short_circuit, implies_total, index, or_short_circuit, or_total,
};

#[kani::proof]
fn tc_003_checked_i32_arithmetic_matches_primitives() {
    let left: i32 = kani::any();
    let right: i32 = kani::any();
    assert_eq!(checked_add(left, right), left.checked_add(right));
    assert_eq!(checked_sub(left, right), left.checked_sub(right));
    assert_eq!(checked_mul(left, right), left.checked_mul(right));
    assert_eq!(checked_div(left, right), left.checked_div(right));
    assert_eq!(checked_rem(left, right), left.checked_rem(right));
}

#[kani::proof]
fn tc_002_boolean_truth_tables() {
    let left: bool = kani::any();
    let right: bool = kani::any();
    assert_eq!(and_short_circuit(left, || right), left & right);
    assert_eq!(or_short_circuit(left, || right), left | right);
    assert_eq!(implies_short_circuit(left, || right), !left | right);
    assert_eq!(and_total(|| left, || right), left & right);
    assert_eq!(or_total(|| left, || right), left | right);
    assert_eq!(implies_total(|| left, || right), !left | right);
}

#[kani::proof]
fn tc_003_slice_index_is_defined_exactly_in_bounds() {
    let values: [u8; 4] = kani::any();
    let at: usize = kani::any();
    assert_eq!(index(&values, at).is_some(), at < values.len());
}
