use quire_contract_runtime::{
    CampaignReport, ClauseId, ClauseKind, ClauseOutcome, ContractIdentity, ExecutionPoint,
    FailureDetail, FailureKind, Observation, RequirementId, RevisionId, Verdict, VerdictContext,
    VerdictKind,
};

fn fixture() -> (
    ContractIdentity<'static>,
    VerdictContext<'static>,
    FailureDetail<'static>,
) {
    static OBSERVATIONS: [Observation<'static>; 1] = [Observation::new(
        ClauseId::new("post-1"),
        ClauseKind::Postcondition,
        ClauseOutcome::Failed,
        Some(FailureDetail::new(
            ClauseId::new("post-1"),
            FailureKind::Postcondition,
            17,
            Some("expected output"),
        )),
    )];
    let identity = ContractIdentity::new(RequirementId::new("FR-001"), RevisionId::new("rev-7"));
    let context = VerdictContext::new(
        identity,
        ExecutionPoint::new("after-handler"),
        &OBSERVATIONS,
    );
    (identity, context, OBSERVATIONS[0].detail.unwrap())
}

/// Trace: TC-001, FR-001-AC-1, FR-001-AC-2, StR-001-VC-1
#[test]
fn tc_001_preserves_tri_state_identity_and_observations() {
    let (identity, context, detail) = fixture();
    let verdicts = [
        Verdict::passed(context),
        Verdict::failed_postcondition(context, detail),
        Verdict::rejected_precondition(
            context,
            FailureDetail::new(
                ClauseId::new("pre-1"),
                FailureKind::Precondition,
                9,
                Some("unsupported input"),
            ),
        ),
    ];

    assert_eq!(verdicts[0].kind(), VerdictKind::Passed);
    assert_eq!(verdicts[1].kind(), VerdictKind::FailedPostcondition);
    assert_eq!(verdicts[2].kind(), VerdictKind::RejectedPrecondition);
    for verdict in verdicts {
        assert_eq!(verdict.context().identity, identity);
        assert_eq!(verdict.context().execution_point.as_str(), "after-handler");
        assert_eq!(verdict.context().observations.len(), 1);
    }
    assert!(verdicts[0].detail().is_none());
    assert_eq!(verdicts[1].detail(), Some(&detail));
}

/// Trace: TC-006, FR-004-AC-1, FR-004-AC-2
#[test]
fn tc_006_report_tracks_complete_saturating_counts() {
    let (identity, context, detail) = fixture();
    let mut report = CampaignReport::new(identity);
    assert!(report.record_verdict(&Verdict::passed(context)).is_ok());
    assert!(report
        .record_verdict(&Verdict::failed_postcondition(context, detail))
        .is_ok());
    assert!(report
        .record_verdict(&Verdict::rejected_precondition(context, detail))
        .is_ok());
    report.record_discard();

    assert_eq!(report.counts().accepted(), 2);
    assert_eq!(report.counts().failed(), 1);
    assert_eq!(report.counts().rejected(), 1);
    assert_eq!(report.counts().discarded(), 1);
    assert_eq!(report.counts().total(), 4);
    assert_eq!(
        report.to_string(),
        "requirement=FR-001 revision=rev-7 accepted=2 rejected=1 failed=1 discarded=1"
    );
}

/// Trace: TC-006, FR-004-AC-1, FR-004-AC-2
#[test]
fn tc_006_report_refuses_a_different_requirement() {
    let (identity, _, _) = fixture();
    let other = ContractIdentity::new(RequirementId::new("FR-999"), RevisionId::new("rev-7"));
    let context = VerdictContext::new(other, ExecutionPoint::new("after-handler"), &[]);
    let mut report = CampaignReport::new(identity);

    let mismatch = report
        .record_verdict(&Verdict::passed(context))
        .expect_err("mismatched identity must be refused");
    assert_eq!(mismatch.expected(), identity);
    assert_eq!(mismatch.actual(), other);
    assert_eq!(report.counts().total(), 0);
}
