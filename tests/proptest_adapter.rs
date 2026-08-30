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

    assert!(
        proptest_adapter::adapt_recording(&mut report, &Verdict::passed(context))
            .expect("identity matches")
            .is_ok()
    );
    assert!(matches!(
        proptest_adapter::adapt_recording(
            &mut report,
            &Verdict::failed_postcondition(context, detail)
        )
        .expect("identity matches"),
        Err(TestCaseError::Fail(_))
    ));
    assert!(matches!(
        proptest_adapter::adapt_recording(
            &mut report,
            &Verdict::rejected_precondition(context, detail)
        )
        .expect("identity matches"),
        Err(TestCaseError::Reject(_))
    ));

    assert_eq!(report.counts().accepted(), 2);
    assert_eq!(report.counts().failed(), 1);
    assert_eq!(report.counts().rejected(), 1);
    assert_eq!(report.counts().discarded(), 0);
}
