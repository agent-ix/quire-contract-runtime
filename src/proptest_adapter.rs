//! Optional adaptation to proptest's tri-state test-case result.

use alloc::format;
use proptest::test_runner::{TestCaseError, TestCaseResult};

use crate::{CampaignReport, Verdict, VerdictKind};

/// Maps a runtime verdict to proptest without converting rejected cases to success.
///
/// This stateless helper does not record campaign counts. Use [`adapt_recording`] when the result
/// contributes to campaign evidence.
///
// Implements: FR-003
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
/// An identity mismatch becomes a proptest failure that retains both identities. Otherwise the
/// result preserves the verdict's pass, failure, or rejection outcome.
///
// Implements: FR-003, FR-004
pub fn adapt_recording(report: &mut CampaignReport<'_>, verdict: &Verdict<'_>) -> TestCaseResult {
    if let Err(mismatch) = report.record_verdict(verdict) {
        return Err(TestCaseError::fail(format!(
            "contract identity mismatch: {mismatch}"
        )));
    }
    adapt(verdict)
}
