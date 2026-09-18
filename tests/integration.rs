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

/// Trace: TC-001, FR-001-AC-4
#[test]
fn tc_001_clause_outcomes_are_distinct_through_construction_and_matching() {
    // Enumerated explicitly, with an unreachable wildcard, so a future sixth
    // variant is caught by the panic below rather than silently matching an
    // existing arm.
    fn discriminant(outcome: ClauseOutcome) -> u8 {
        match outcome {
            ClauseOutcome::Passed => 0,
            ClauseOutcome::Failed => 1,
            ClauseOutcome::Rejected => 2,
            ClauseOutcome::NotEvaluated => 3,
            ClauseOutcome::Undefined => 4,
            _ => unreachable!("ClauseOutcome grew a variant this test does not enumerate"),
        }
    }

    let outcomes = [
        ClauseOutcome::Passed,
        ClauseOutcome::Failed,
        ClauseOutcome::Rejected,
        ClauseOutcome::NotEvaluated,
        ClauseOutcome::Undefined,
    ];
    for (left_index, left) in outcomes.iter().enumerate() {
        assert_eq!(discriminant(*left), left_index as u8);
        for (right_index, right) in outcomes.iter().enumerate() {
            assert_eq!(left == right, left_index == right_index);
        }
    }

    let failed_detail = FailureDetail::new(
        ClauseId::new("post-1"),
        FailureKind::Postcondition,
        17,
        Some("expected output"),
    );
    let failed = Observation::new(
        ClauseId::new("post-1"),
        ClauseKind::Postcondition,
        ClauseOutcome::Failed,
        Some(failed_detail),
    );
    assert_eq!(failed.outcome, ClauseOutcome::Failed);
    assert_eq!(failed.detail, Some(failed_detail));
    assert_eq!(failed_detail.clause.as_str(), "post-1");
    assert_eq!(failed_detail.kind, FailureKind::Postcondition);
    assert_eq!(failed_detail.code, 17);
    assert_eq!(failed_detail.message, Some("expected output"));

    let rejected_detail =
        FailureDetail::new(ClauseId::new("pre-1"), FailureKind::Precondition, 9, None);
    let rejected = Observation::new(
        ClauseId::new("pre-1"),
        ClauseKind::Precondition,
        ClauseOutcome::Rejected,
        Some(rejected_detail),
    );
    assert_eq!(rejected.outcome, ClauseOutcome::Rejected);
    assert_eq!(rejected.detail, Some(rejected_detail));
    assert_eq!(rejected_detail.clause.as_str(), "pre-1");
    assert_eq!(rejected_detail.kind, FailureKind::Precondition);
    assert_eq!(rejected_detail.code, 9);
    // The borrowed detail message is optional; a rejection need not carry one.
    assert_eq!(rejected_detail.message, None);
}

/// Trace: TC-008, FR-001-AC-5
#[test]
fn tc_008_runtime_contract_version_matches_interface_frontmatter() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("spec/interface/interface-001-runtime-api.md");
    let contract = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("interface-001 contract is readable: {error}"));
    let version = contract
        .lines()
        .find_map(|line| line.strip_prefix("version: "))
        .expect("interface-001 contract declares a version")
        .trim();

    assert_eq!(quire_contract_runtime::RUNTIME_CONTRACT_VERSION, version);
}

/// Trace: TC-006, FR-004-AC-8
#[test]
fn tc_006_mismatched_identity_leaves_every_counter_and_snapshot_untouched() {
    let (identity, _, detail) = fixture();
    let other_requirement = ContractIdentity::new(RequirementId::new("FR-999"), identity.revision);
    let other_revision = ContractIdentity::new(identity.requirement, RevisionId::new("rev-999"));

    for mismatched in [other_requirement, other_revision] {
        let mismatched_context =
            VerdictContext::new(mismatched, ExecutionPoint::new("after-handler"), &[]);
        let verdict = Verdict::failed_postcondition(mismatched_context, detail);

        let mut report = CampaignReport::new(identity);
        let untouched = CampaignReport::new(identity);
        let snapshot_before = report.snapshot();

        let mismatch = report
            .record_verdict(&verdict)
            .expect_err("mismatched identity must be refused");

        assert_eq!(mismatch.expected(), identity);
        assert_eq!(mismatch.actual(), mismatched);
        assert_eq!(report.counts().accepted(), 0);
        assert_eq!(report.counts().rejected(), 0);
        assert_eq!(report.counts().failed(), 0);
        assert_eq!(report.counts().discarded(), 0);
        assert_eq!(report.counts(), untouched.counts());
        assert_eq!(report.snapshot(), snapshot_before);
    }
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
