use crate::operators::{
    and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem, checked_sub,
    implies_short_circuit, implies_total, index, or_short_circuit, or_total,
};

#[kani::proof]
fn tc_003_checked_i8_arithmetic_matches_primitives() {
    let left: i8 = kani::any();
    let right: i8 = kani::any();
    assert_eq!(checked_add(left, right), left.checked_add(right));
    assert_eq!(checked_sub(left, right), left.checked_sub(right));
    assert_eq!(checked_mul(left, right), left.checked_mul(right));
    assert_eq!(checked_div(left, right), left.checked_div(right));
    assert_eq!(checked_rem(left, right), left.checked_rem(right));
}

#[kani::proof]
fn tc_003_i32_division_boundaries_are_undefined() {
    assert_eq!(checked_div(i32::MIN, -1), None);
    assert_eq!(checked_div(1_i32, 0), None);
    assert_eq!(checked_rem(i32::MIN, -1), None);
    assert_eq!(checked_rem(1_i32, 0), None);
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
    let at = usize::from(kani::any::<u8>());
    assert_eq!(index(&values, at).is_some(), at < values.len());
}
