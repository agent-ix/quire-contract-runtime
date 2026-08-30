//! Complete, saturating per-requirement campaign accounting.

use crate::{ContractIdentity, Verdict, VerdictKind};

/// Complete campaign counters. No constructor or formatter can omit a metric.
///
/// Implements: FR-004
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CampaignCounts {
    accepted: u64,
    rejected: u64,
    failed: u64,
    discarded: u64,
}

impl CampaignCounts {
    /// Creates an all-zero, complete counter set.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn new() -> Self {
        Self {
            accepted: 0,
            rejected: 0,
            failed: 0,
            discarded: 0,
        }
    }

    fn record_kind(&mut self, kind: VerdictKind) {
        match kind {
            VerdictKind::Passed => self.accepted = self.accepted.saturating_add(1),
            VerdictKind::FailedPostcondition => {
                self.accepted = self.accepted.saturating_add(1);
                self.failed = self.failed.saturating_add(1);
            }
            VerdictKind::RejectedPrecondition => {
                self.rejected = self.rejected.saturating_add(1);
            }
        }
    }

    fn record_discard(&mut self) {
        self.discarded = self.discarded.saturating_add(1);
    }

    /// Returns accepted cases, including passed and failed postconditions.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn accepted(self) -> u64 {
        self.accepted
    }

    /// Returns cases excluded by a contract precondition.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn rejected(self) -> u64 {
        self.rejected
    }

    /// Returns accepted cases with a failed postcondition.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn failed(self) -> u64 {
        self.failed
    }

    /// Returns cases discarded by an external campaign framework before a verdict.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn discarded(self) -> u64 {
        self.discarded
    }

    /// Returns the total number of observed campaign cases, saturating on overflow.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn total(self) -> u64 {
        self.accepted
            .saturating_add(self.rejected)
            .saturating_add(self.discarded)
    }
}

impl core::fmt::Display for CampaignCounts {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "accepted={} rejected={} failed={} discarded={}",
            self.accepted, self.rejected, self.failed, self.discarded
        )
    }
}

/// Describes a verdict whose requirement or revision does not match its report.
///
/// Implements: FR-004
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IdentityMismatch<'expected, 'actual> {
    expected: ContractIdentity<'expected>,
    actual: ContractIdentity<'actual>,
}

impl<'expected, 'actual> IdentityMismatch<'expected, 'actual> {
    /// Returns the report identity that was required.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn expected(&self) -> ContractIdentity<'expected> {
        self.expected
    }

    /// Returns the identity carried by the refused verdict.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn actual(&self) -> ContractIdentity<'actual> {
        self.actual
    }
}

/// Complete campaign report for exactly one requirement revision.
///
/// Implements: FR-004
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CampaignReport<'a> {
    identity: ContractIdentity<'a>,
    counts: CampaignCounts,
}

impl<'a> CampaignReport<'a> {
    /// Creates an empty report that already contains every required metric.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn new(identity: ContractIdentity<'a>) -> Self {
        Self {
            identity,
            counts: CampaignCounts::new(),
        }
    }

    /// Returns the requirement/revision identity shared by every counter.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn identity(&self) -> ContractIdentity<'a> {
        self.identity
    }

    /// Returns the indivisible complete counter set.
    ///
    /// Implements: FR-004
    #[must_use]
    pub const fn counts(&self) -> CampaignCounts {
        self.counts
    }

    /// Records one verdict if its requirement and revision match this report.
    ///
    /// Implements: FR-004
    pub fn record_verdict<'verdict>(
        &mut self,
        verdict: &Verdict<'verdict>,
    ) -> Result<(), IdentityMismatch<'a, 'verdict>> {
        if self.identity.requirement.as_str() != verdict.context().identity.requirement.as_str()
            || self.identity.revision.as_str() != verdict.context().identity.revision.as_str()
        {
            return Err(IdentityMismatch {
                expected: self.identity,
                actual: verdict.context().identity,
            });
        }
        self.counts.record_kind(verdict.kind());
        Ok(())
    }

    /// Records one external framework discard.
    ///
    /// Implements: FR-004
    pub fn record_discard(&mut self) {
        self.counts.record_discard();
    }
}

impl core::fmt::Display for CampaignReport<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "requirement={} revision={} {}",
            self.identity.requirement, self.identity.revision, self.counts
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CampaignCounts;
    use crate::VerdictKind;

    /// Trace: TC-006, FR-004-AC-2
    #[test]
    fn tc_006_private_counters_saturate() -> Result<(), &'static str> {
        let mut counts = CampaignCounts {
            accepted: u64::MAX,
            rejected: u64::MAX,
            failed: u64::MAX,
            discarded: u64::MAX,
        };

        counts.record_kind(VerdictKind::FailedPostcondition);
        counts.record_kind(VerdictKind::RejectedPrecondition);
        counts.record_discard();

        if counts.accepted() == u64::MAX
            && counts.rejected() == u64::MAX
            && counts.failed() == u64::MAX
            && counts.discarded() == u64::MAX
            && counts.total() == u64::MAX
        {
            Ok(())
        } else {
            Err("private counters did not saturate")
        }
    }
}
