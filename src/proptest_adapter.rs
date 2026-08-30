//! Optional adaptation to proptest's tri-state test-case result.

use proptest::test_runner::{TestCaseError, TestCaseResult};

use crate::{Verdict, VerdictKind};

/// Maps a runtime verdict to proptest without converting rejected cases to success.
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
