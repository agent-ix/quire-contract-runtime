#![cfg(feature = "proptest")]

use proptest::test_runner::TestCaseError;
use quire_contract_runtime::{
    proptest_adapter, CampaignReport, ClauseId, ContractIdentity, ExecutionPoint, FailureDetail,
    FailureKind, RequirementId, RevisionId, Verdict, VerdictContext,
};

/// Trace: TC-004, FR-003-AC-1, StR-001-VC-1
#[test]
fn tc_004_adapter_preserves_all_three_outcomes() {
    let context = VerdictContext::new(
        ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1")),
        ExecutionPoint::new("property"),
        &[],
    );
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);

    assert!(proptest_adapter::adapt(&Verdict::passed(context)).is_ok());
    assert!(matches!(
        proptest_adapter::adapt(&Verdict::failed_postcondition(context, detail)),
        Err(TestCaseError::Fail(_))
    ));
    assert!(matches!(
        proptest_adapter::adapt(&Verdict::rejected_precondition(context, detail)),
        Err(TestCaseError::Reject(_))
    ));
}

/// Trace: TC-004, FR-003-AC-1, FR-004-AC-1, StR-001-VC-1
#[test]
fn tc_004_recording_adapter_preserves_the_campaign_census() {
    let identity = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("property"), &[]);
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);
    let mut report = CampaignReport::new(identity);

    assert!(proptest_adapter::adapt_recording(&mut report, &Verdict::passed(context)).is_ok());
    assert!(matches!(
        proptest_adapter::adapt_recording(
            &mut report,
            &Verdict::failed_postcondition(context, detail)
        ),
        Err(TestCaseError::Fail(_))
    ));
    assert!(matches!(
        proptest_adapter::adapt_recording(
            &mut report,
            &Verdict::rejected_precondition(context, detail)
        ),
        Err(TestCaseError::Reject(_))
    ));

    assert_eq!(report.counts().accepted(), 2);
    assert_eq!(report.counts().failed(), 1);
    assert_eq!(report.counts().rejected(), 1);
    assert_eq!(report.counts().discarded(), 0);
}

/// Trace: TC-004, FR-003-AC-3
#[test]
fn tc_004_adapt_is_stateless_and_never_records() {
    let identity = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("property"), &[]);
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);
    let report = CampaignReport::new(identity);
    let counts_before = report.counts();

    for verdict in [
        Verdict::passed(context),
        Verdict::failed_postcondition(context, detail),
        Verdict::rejected_precondition(context, detail),
    ] {
        // `adapt` takes no report at all, so there is nothing for it to record into; this
        // confirms a report that was never passed to it stays exactly as it was.
        let _ = proptest_adapter::adapt(&verdict);
        assert_eq!(report.counts(), counts_before);
    }
}

/// Trace: TC-004, FR-003-AC-3
#[test]
fn tc_004_recording_adapter_maps_matching_verdicts_exactly_as_adapt_does() {
    let identity = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("property"), &[]);
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);

    for verdict in [
        Verdict::passed(context),
        Verdict::failed_postcondition(context, detail),
        Verdict::rejected_precondition(context, detail),
    ] {
        let mut report = CampaignReport::new(identity);
        let counts_before = report.counts();

        let stateless = proptest_adapter::adapt(&verdict);
        let recorded = proptest_adapter::adapt_recording(&mut report, &verdict);

        assert_eq!(format!("{stateless:?}"), format!("{recorded:?}"));
        assert_ne!(
            report.counts(),
            counts_before,
            "a matching verdict must be recorded exactly once"
        );
    }
}

/// Trace: TC-004, FR-003-AC-3
#[test]
fn tc_004_recording_adapter_rejects_a_revision_only_mismatch() {
    let expected = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let actual = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-2"));
    let context = VerdictContext::new(actual, ExecutionPoint::new("property"), &[]);
    let mut report = CampaignReport::new(expected);

    let result = proptest_adapter::adapt_recording(&mut report, &Verdict::passed(context));
    let message = match result {
        Err(TestCaseError::Fail(message)) => message,
        other => panic!("expected identity mismatch failure, observed {other:?}"),
    };
    let message = message.to_string();

    assert!(message.contains("expected requirement=FR-003 revision=rev-1"));
    assert!(message.contains("observed requirement=FR-003 revision=rev-2"));
    assert_eq!(report.counts(), Default::default());
}

/// Trace: TC-004, FR-003-AC-1, FR-004-AC-1
#[test]
fn tc_004_recording_adapter_turns_identity_mismatch_into_failure() {
    let expected = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let actual = ContractIdentity::new(RequirementId::new("FR-004"), RevisionId::new("rev-2"));
    let context = VerdictContext::new(actual, ExecutionPoint::new("property"), &[]);
    let mut report = CampaignReport::new(expected);

    let result = proptest_adapter::adapt_recording(&mut report, &Verdict::passed(context));
    let message = match result {
        Err(TestCaseError::Fail(message)) => message,
        other => panic!("expected identity mismatch failure, observed {other:?}"),
    };
    let message = message.to_string();

    assert!(message.contains("expected requirement=FR-003 revision=rev-1"));
    assert!(message.contains("observed requirement=FR-004 revision=rev-2"));
    assert_eq!(report.counts(), Default::default());
}
