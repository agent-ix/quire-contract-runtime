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

/// `adapt` records nothing because it has no `CampaignReport` to record into: its signature takes
/// only a verdict. That is a fact about the function's type, not a value this test could
/// construct and compare (a `report` never passed to `adapt` trivially stays unchanged, which the
/// compiler already guarantees and no runtime assertion could falsify), so it is checked in the
/// source text, the same way `CheckedInteger`'s seal and FR-012's re-export set are.
///
/// Trace: TC-004, FR-003-AC-3
#[test]
fn tc_004_adapt_signature_takes_no_report() {
    let source = include_str!("../src/proptest_adapter.rs");
    assert!(source.contains("pub fn adapt(verdict: &Verdict<'_>) -> TestCaseResult {"));
    assert!(source.contains(
        "pub fn adapt_recording(report: &mut CampaignReport<'_>, verdict: &Verdict<'_>) -> TestCaseResult {"
    ));
}

/// Trace: TC-004, FR-003-AC-3
#[test]
fn tc_004_recording_adapter_maps_matching_verdicts_to_pinned_results_and_records_exactly_once() {
    let identity = ContractIdentity::new(RequirementId::new("FR-003"), RevisionId::new("rev-1"));
    let context = VerdictContext::new(identity, ExecutionPoint::new("property"), &[]);
    let detail = FailureDetail::new(ClauseId::new("clause"), FailureKind::Contract, 1, None);

    // Passed: `Ok(())`, exactly one accepted case and no other counter moves.
    let mut report = CampaignReport::new(identity);
    let result = proptest_adapter::adapt_recording(&mut report, &Verdict::passed(context));
    assert!(result.is_ok());
    assert_eq!(report.counts().accepted(), 1);
    assert_eq!(report.counts().failed(), 0);
    assert_eq!(report.counts().rejected(), 0);
    assert_eq!(report.counts().discarded(), 0);

    // Failed postcondition: the pinned failure message, one accepted case that is also failed.
    let mut report = CampaignReport::new(identity);
    let result = proptest_adapter::adapt_recording(
        &mut report,
        &Verdict::failed_postcondition(context, detail),
    );
    match result {
        Err(TestCaseError::Fail(message)) => {
            assert_eq!(message.to_string(), "contract postcondition failed");
        }
        other => panic!("expected a postcondition failure, observed {other:?}"),
    }
    assert_eq!(report.counts().accepted(), 1);
    assert_eq!(report.counts().failed(), 1);
    assert_eq!(report.counts().rejected(), 0);
    assert_eq!(report.counts().discarded(), 0);

    // Rejected precondition: the pinned rejection message, one rejected case and nothing else.
    let mut report = CampaignReport::new(identity);
    let result = proptest_adapter::adapt_recording(
        &mut report,
        &Verdict::rejected_precondition(context, detail),
    );
    match result {
        Err(TestCaseError::Reject(message)) => {
            assert_eq!(message.to_string(), "contract precondition rejected case");
        }
        other => panic!("expected a precondition rejection, observed {other:?}"),
    }
    assert_eq!(report.counts().accepted(), 0);
    assert_eq!(report.counts().failed(), 0);
    assert_eq!(report.counts().rejected(), 1);
    assert_eq!(report.counts().discarded(), 0);
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
