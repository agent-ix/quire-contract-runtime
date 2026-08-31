use crate::{
    operators::{
        and_short_circuit, and_total, checked_add, checked_div, checked_mul, checked_rem,
        checked_sub, implies_short_circuit, implies_total, index, option_copied, option_ref,
        or_short_circuit, or_total,
    },
    ClauseId, ClauseKind, ClauseOutcome, ContractIdentity, ExecutionPoint, FailureDetail,
    FailureKind, Observation, RequirementId, RevisionId, Verdict, VerdictContext, VerdictKind,
};

// Implements: TC-001
#[kani::proof]
fn tc_001_public_model_preserves_provenance() {
    let requirement_text = "FR-001";
    let revision_text = "revision";
    let execution_point_text = "return";
    let clause_text = "postcondition";
    let message_text = "detail";
    let requirement = RequirementId::new(requirement_text);
    let revision = RevisionId::new(revision_text);
    let identity = ContractIdentity::new(requirement, revision);
    let clause = ClauseId::new(clause_text);
    let detail = FailureDetail::new(clause, FailureKind::Postcondition, 7, Some(message_text));
    let observations = [Observation::new(
        clause,
        ClauseKind::Postcondition,
        ClauseOutcome::Failed,
        Some(detail),
    )];
    let context = VerdictContext::new(
        identity,
        ExecutionPoint::new(execution_point_text),
        &observations,
    );

    assert!(core::ptr::eq(requirement.as_str(), requirement_text));
    assert!(core::ptr::eq(revision.as_str(), revision_text));
    assert!(core::ptr::eq(
        context.identity.requirement.as_str(),
        requirement_text,
    ));
    assert!(core::ptr::eq(
        context.identity.revision.as_str(),
        revision_text,
    ));
    assert!(core::ptr::eq(
        context.execution_point.as_str(),
        execution_point_text,
    ));
    assert!(core::ptr::eq(context.observations, &observations));
    assert!(core::ptr::eq(
        context.observations[0].clause.as_str(),
        clause_text,
    ));
    assert!(matches!(
        context.observations[0].kind,
        ClauseKind::Postcondition
    ));
    assert!(matches!(
        context.observations[0].outcome,
        ClauseOutcome::Failed
    ));

    let passed = Verdict::passed(context);
    assert!(matches!(passed.kind(), VerdictKind::Passed));
    assert!(core::ptr::eq(passed.context().observations, &observations));
    assert!(passed.detail().is_none());

    let failed = Verdict::failed_postcondition(context, detail);
    assert!(matches!(failed.kind(), VerdictKind::FailedPostcondition));
    assert!(core::ptr::eq(failed.context().observations, &observations));
    let failed_detail = failed.detail().expect("failed verdict retains detail");
    assert_eq!(failed_detail.code, 7);
    assert!(matches!(failed_detail.kind, FailureKind::Postcondition));
    assert!(core::ptr::eq(
        failed_detail.message.expect("message retained"),
        message_text,
    ));

    let rejected = Verdict::rejected_precondition(context, detail);
    assert!(matches!(rejected.kind(), VerdictKind::RejectedPrecondition));
    assert!(core::ptr::eq(
        rejected.context().observations,
        &observations
    ));
    assert_eq!(
        rejected
            .detail()
            .expect("rejected verdict retains detail")
            .code,
        7,
    );
}

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
    let widened_difference = i16::from(left) - i16::from(right);
    let expected_difference =
        if widened_difference < i16::from(i8::MIN) || widened_difference > i16::from(i8::MAX) {
            None
        } else {
            Some(widened_difference as i8)
        };
    let widened_product = i16::from(left) * i16::from(right);
    let expected_product =
        if widened_product < i16::from(i8::MIN) || widened_product > i16::from(i8::MAX) {
            None
        } else {
            Some(widened_product as i8)
        };
    let undefined_division = right == 0 || (left == i8::MIN && right == -1);
    let expected_division = if undefined_division {
        None
    } else {
        Some((i16::from(left) / i16::from(right)) as i8)
    };
    let expected_remainder = if undefined_division {
        None
    } else {
        Some((i16::from(left) % i16::from(right)) as i8)
    };
    assert_eq!(checked_add(left, right), expected_sum);
    assert_eq!(checked_sub(left, right), expected_difference);
    assert_eq!(checked_mul(left, right), expected_product);
    assert_eq!(checked_div(left, right), expected_division);
    assert_eq!(checked_rem(left, right), expected_remainder);
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
