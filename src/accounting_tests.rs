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
