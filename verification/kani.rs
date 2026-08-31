use crate::{
    operators::{
        and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem,
        checked_sub, implies_short_circuit, implies_total, index, option_copied, option_ref,
        or_short_circuit, or_total,
    },
    ClauseId, ContractIdentity, ExecutionPoint, FailureDetail, FailureKind, RequirementId,
    RevisionId, Verdict, VerdictContext,
};

// Implements: TC-003
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

// Implements: TC-003
#[kani::proof]
fn tc_003_i32_division_boundaries_are_undefined() {
    let left: i32 = kani::any();
    let right: i32 = kani::any();
    kani::assume(right == 0 || (left == i32::MIN && right == -1));
    assert_eq!(checked_div(left, right), None);
    assert_eq!(checked_rem(left, right), None);
}

// Implements: TC-002
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

// Implements: TC-003
#[kani::proof]
fn tc_003_slice_index_is_defined_exactly_in_bounds() {
    let values: [u8; 4] = kani::any();
    let at: usize = kani::any();
    assert_eq!(index(&values, at).is_some(), at < values.len());
}

// Implements: TC-003
#[kani::proof]
fn tc_003_campaign_accounting_saturates() {
    let accepted: u64 = kani::any();
    let rejected: u64 = kani::any();
    let failed: u64 = kani::any();
    let discarded: u64 = kani::any();
    let identity = ContractIdentity::new(RequirementId::new("FR-004"), RevisionId::new("1"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("proof"), &[]);
    let detail = FailureDetail::new(ClauseId::new("accounting"), FailureKind::Contract, 1, None);

    let mut passed =
        crate::CampaignReport::from_proof_counts(identity, accepted, rejected, failed, discarded);
    assert!(passed.record_verdict(&Verdict::passed(context)).is_ok());
    assert_eq!(passed.counts().accepted(), accepted.saturating_add(1));
    assert_eq!(passed.counts().failed(), failed);

    let mut failed_postcondition =
        crate::CampaignReport::from_proof_counts(identity, accepted, rejected, failed, discarded);
    assert!(failed_postcondition
        .record_verdict(&Verdict::failed_postcondition(context, detail))
        .is_ok());
    assert_eq!(
        failed_postcondition.counts().accepted(),
        accepted.saturating_add(1)
    );
    assert_eq!(
        failed_postcondition.counts().failed(),
        failed.saturating_add(1)
    );

    let mut rejected_precondition =
        crate::CampaignReport::from_proof_counts(identity, accepted, rejected, failed, discarded);
    assert!(rejected_precondition
        .record_verdict(&Verdict::rejected_precondition(context, detail))
        .is_ok());
    assert_eq!(
        rejected_precondition.counts().rejected(),
        rejected.saturating_add(1)
    );

    let mut discarded_report =
        crate::CampaignReport::from_proof_counts(identity, accepted, rejected, failed, discarded);
    discarded_report.record_discard();
    assert_eq!(
        discarded_report.counts().discarded(),
        discarded.saturating_add(1)
    );
    assert_eq!(
        discarded_report.counts().total(),
        accepted
            .saturating_add(rejected)
            .saturating_add(discarded.saturating_add(1))
    );
}

// Implements: TC-003
#[kani::proof]
fn tc_003_option_helpers_preserve_definedness() {
    let value: Option<u8> = kani::any();
    assert_eq!(option_copied(option_ref(&value)), value);
}
