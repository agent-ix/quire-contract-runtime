use super::CampaignCounts;
use crate::VerdictKind;

/// Trace: TC-006, FR-004-AC-2
#[test]
fn tc_006_private_counters_saturate() {
    let mut counts = CampaignCounts {
        accepted: u64::MAX,
        rejected: u64::MAX,
        failed: u64::MAX,
        discarded: u64::MAX,
    };

    counts.record_kind(VerdictKind::FailedPostcondition);
    counts.record_kind(VerdictKind::RejectedPrecondition);
    counts.record_discard();

    assert_eq!(counts.accepted(), u64::MAX);
    assert_eq!(counts.rejected(), u64::MAX);
    assert_eq!(counts.failed(), u64::MAX);
    assert_eq!(counts.discarded(), u64::MAX);
    assert_eq!(counts.total(), u64::MAX);
}

/// Trace: TC-015, FR-004-AC-4, FR-004-AC-6
#[test]
fn tc_015_actual_near_limit_report_preserves_capture_and_saturates() {
    use crate::{
        CampaignReport, ClauseId, ContractIdentity, ExecutionPoint, FailureDetail, FailureKind,
        RequirementId, RevisionId, Verdict, VerdictContext,
    };
    let mut report = CampaignReport {
        identity: ContractIdentity::new(RequirementId::new("FR-limit"), RevisionId::new("0")),
        counts: CampaignCounts {
            accepted: u64::MAX - 1,
            rejected: 0,
            failed: u64::MAX - 1,
            discarded: 0,
        },
    };
    let before = report.snapshot();
    assert!(!before.at_limit());
    let context = VerdictContext::new(report.identity(), ExecutionPoint::new("post"), &[]);
    let detail = FailureDetail::new(ClauseId::new("post"), FailureKind::Postcondition, 0, None);
    let verdict = Verdict::failed_postcondition(context, detail);
    assert!(report.record_verdict(&verdict).is_ok());
    let exact = report.snapshot();
    assert!(exact.at_limit());
    assert!(report.record_verdict(&verdict).is_ok());
    assert_eq!(report.snapshot(), exact);
    assert_eq!(before.counts().accepted(), u64::MAX - 1);
    assert_eq!(exact.counts().failed(), u64::MAX);
}
