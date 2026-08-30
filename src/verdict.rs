//! Tri-state terminal verdicts for contract harnesses.

use crate::{ContractIdentity, ExecutionPoint, FailureDetail, Observation};

/// Common provenance carried by every verdict.
///
// Implements: FR-001
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerdictContext<'a> {
    /// Requirement and revision under evaluation.
    ///
    // Implements: FR-001
    pub identity: ContractIdentity<'a>,
    /// Named point where the oracle observed the program.
    ///
    // Implements: FR-001
    pub execution_point: ExecutionPoint<'a>,
    /// Per-clause observations in generated contract order.
    ///
    // Implements: FR-001
    pub observations: &'a [Observation<'a>],
}

impl<'a> VerdictContext<'a> {
    /// Creates common verdict provenance without allocation.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn new(
        identity: ContractIdentity<'a>,
        execution_point: ExecutionPoint<'a>,
        observations: &'a [Observation<'a>],
    ) -> Self {
        Self {
            identity,
            execution_point,
            observations,
        }
    }
}

/// Stable terminal category for counters and adapters.
///
/// Trace: TC-008, NFR-002-AC-3
// Implements: FR-001
///
/// Downstream exhaustive matching is rejected so future terminal categories cannot be silently
/// treated as success:
///
/// ```compile_fail
/// use quire_contract_runtime::VerdictKind;
///
/// fn classify(kind: VerdictKind) -> u8 {
///     match kind {
///         VerdictKind::Passed => 0,
///         VerdictKind::FailedPostcondition => 1,
///         VerdictKind::RejectedPrecondition => 2,
///     }
/// }
/// ```
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VerdictKind {
    /// All required clauses passed.
    Passed,
    /// An accepted case violated a postcondition.
    FailedPostcondition,
    /// A precondition excluded the case.
    RejectedPrecondition,
}

/// Terminal harness result.
///
/// There is deliberately no conversion to `bool` or `Result<(), _>` in the core: downstream code
/// must choose how to retain a rejected precondition instead of accidentally treating it as success.
///
/// Trace: TC-008, FR-001-AC-3, NFR-002-AC-3
// Implements: FR-001
///
/// Downstream exhaustive matching is rejected:
///
/// ```compile_fail
/// use quire_contract_runtime::Verdict;
///
/// fn classify(verdict: Verdict<'_>) -> u8 {
///     match verdict {
///         Verdict::Passed(_) => 0,
///         Verdict::FailedPostcondition { .. } => 1,
///         Verdict::RejectedPrecondition { .. } => 2,
///     }
/// }
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verdict<'a> {
    /// All required clauses passed.
    Passed(VerdictContext<'a>),
    /// An accepted case violated a postcondition.
    FailedPostcondition {
        /// Common provenance and clause observations.
        context: VerdictContext<'a>,
        /// Structured terminal failure.
        failure: FailureDetail<'a>,
    },
    /// A precondition excluded the case from the accepted domain.
    RejectedPrecondition {
        /// Common provenance and clause observations.
        context: VerdictContext<'a>,
        /// Structured rejection reason.
        rejection: FailureDetail<'a>,
    },
}

impl<'a> Verdict<'a> {
    /// Constructs a successful verdict.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn passed(context: VerdictContext<'a>) -> Self {
        Self::Passed(context)
    }

    /// Constructs a failed-postcondition verdict.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn failed_postcondition(
        context: VerdictContext<'a>,
        failure: FailureDetail<'a>,
    ) -> Self {
        Self::FailedPostcondition { context, failure }
    }

    /// Constructs a rejected-precondition verdict.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn rejected_precondition(
        context: VerdictContext<'a>,
        rejection: FailureDetail<'a>,
    ) -> Self {
        Self::RejectedPrecondition { context, rejection }
    }

    /// Returns the terminal category without discarding rejection.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn kind(&self) -> VerdictKind {
        match self {
            Self::Passed(_) => VerdictKind::Passed,
            Self::FailedPostcondition { .. } => VerdictKind::FailedPostcondition,
            Self::RejectedPrecondition { .. } => VerdictKind::RejectedPrecondition,
        }
    }

    /// Returns common requirement, revision, execution-point, and observation provenance.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn context(&self) -> &VerdictContext<'a> {
        match self {
            Self::Passed(context)
            | Self::FailedPostcondition { context, .. }
            | Self::RejectedPrecondition { context, .. } => context,
        }
    }

    /// Returns structured terminal detail for failure or rejection.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn detail(&self) -> Option<&FailureDetail<'a>> {
        match self {
            Self::Passed(_) => None,
            Self::FailedPostcondition { failure, .. } => Some(failure),
            Self::RejectedPrecondition { rejection, .. } => Some(rejection),
        }
    }
}
