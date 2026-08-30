//! Optional adaptation to proptest's tri-state test-case result.

use proptest::test_runner::{TestCaseError, TestCaseResult};

use crate::{CampaignReport, IdentityMismatch, Verdict, VerdictKind};

/// Maps a runtime verdict to proptest without converting rejected cases to success.
///
/// This stateless helper does not record campaign counts. Use [`adapt_recording`] when the result
/// contributes to campaign evidence.
///
/// Implements: FR-003
pub fn adapt(verdict: &Verdict<'_>) -> TestCaseResult {
    match verdict.kind() {
        VerdictKind::Passed => Ok(()),
        VerdictKind::FailedPostcondition => {
            Err(TestCaseError::fail("contract postcondition failed"))
        }
        VerdictKind::RejectedPrecondition => {
            Err(TestCaseError::reject("contract precondition rejected case"))
        }
    }
}

/// Records a verdict in its matching report before adapting it to proptest.
///
/// The outer result reports an identity mismatch. The inner [`TestCaseResult`] preserves proptest's
/// pass, failure, or rejection result.
///
/// Implements: FR-003, FR-004
pub fn adapt_recording<'report, 'verdict>(
    report: &mut CampaignReport<'report>,
    verdict: &Verdict<'verdict>,
) -> Result<TestCaseResult, IdentityMismatch<'report, 'verdict>> {
    report.record_verdict(verdict)?;
    Ok(adapt(verdict))
}
