//! Complete, saturating per-requirement campaign accounting.

use crate::{ContractIdentity, Verdict, VerdictKind};

/// Complete campaign counters. No constructor or formatter can omit a metric.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CampaignCounts {
    /// Accepted cases, including both passed and failed postconditions.
    pub accepted: u64,
    /// Cases excluded by a contract precondition.
    pub rejected: u64,
    /// Accepted cases with a failed postcondition.
    pub failed: u64,
    /// Cases discarded by an external campaign framework before a verdict.
    pub discarded: u64,
}

impl CampaignCounts {
    /// Creates an all-zero, complete counter set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            accepted: 0,
            rejected: 0,
            failed: 0,
            discarded: 0,
        }
    }

    /// Records one terminal verdict using saturating arithmetic.
    pub const fn record_kind(&mut self, kind: VerdictKind) {
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

    /// Records one external framework discard using saturating arithmetic.
    pub const fn record_discard(&mut self) {
        self.discarded = self.discarded.saturating_add(1);
    }

    /// Returns the total number of observed campaign cases, saturating on overflow.
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

/// Complete campaign report for exactly one requirement revision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CampaignReport<'a> {
    /// Requirement/revision identity for every counter.
    pub identity: ContractIdentity<'a>,
    /// Indivisible complete counter set.
    pub counts: CampaignCounts,
}

impl<'a> CampaignReport<'a> {
    /// Creates an empty report that already contains every required metric.
    #[must_use]
    pub const fn new(identity: ContractIdentity<'a>) -> Self {
        Self {
            identity,
            counts: CampaignCounts::new(),
        }
    }

    /// Records one verdict if its requirement and revision match this report.
    #[must_use]
    pub fn record_verdict(&mut self, verdict: &Verdict<'_>) -> bool {
        if self.identity.requirement.as_str() != verdict.context().identity.requirement.as_str()
            || self.identity.revision.as_str() != verdict.context().identity.revision.as_str()
        {
            return false;
        }
        self.counts.record_kind(verdict.kind());
        true
    }

    /// Records one external framework discard.
    pub const fn record_discard(&mut self) {
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
