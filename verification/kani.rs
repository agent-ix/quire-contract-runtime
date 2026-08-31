use crate::{
    accounting::CampaignCounts,
    operators::{
        and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem,
        checked_sub, implies_short_circuit, implies_total, index, option_copied, option_ref,
        or_short_circuit, or_total,
    },
};

#[kani::proof]
fn tc_003_checked_i8_arithmetic_matches_primitives() {
    let left: i8 = kani::any();
    let right: i8 = kani::any();
    let widened_sum = i16::from(left) + i16::from(right);
    let expected_sum = if widened_sum < i16::from(i8::MIN) || widened_sum > i16::from(i8::MAX) {
        None
    } else {
        Some(widened_sum as i8)
    };
    assert_eq!(checked_add(left, right), expected_sum);
    assert_eq!(checked_sub(left, right), left.checked_sub(right));
    assert_eq!(checked_mul(left, right), left.checked_mul(right));
    assert_eq!(checked_div(left, right), left.checked_div(right));
    assert_eq!(checked_rem(left, right), left.checked_rem(right));
}

#[kani::proof]
fn tc_003_i32_division_boundaries_are_undefined() {
    let left: i32 = kani::any();
    let right: i32 = kani::any();
    kani::assume(right == 0 || (left == i32::MIN && right == -1));
    assert_eq!(checked_div(left, right), None);
    assert_eq!(checked_rem(left, right), None);
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

#[kani::proof]
fn tc_003_campaign_counts_total_saturates() {
    let accepted: u64 = kani::any();
    let rejected: u64 = kani::any();
    let failed: u64 = kani::any();
    let discarded: u64 = kani::any();
    let counts = CampaignCounts::from_proof_counts(accepted, rejected, failed, discarded);
    let expected = counts
        .accepted()
        .saturating_add(counts.rejected())
        .saturating_add(counts.discarded());
    assert_eq!(counts.total(), expected);
}

#[kani::proof]
fn tc_003_option_helpers_preserve_definedness() {
    let value: Option<u8> = kani::any();
    assert_eq!(option_copied(option_ref(&value)), value);
}
