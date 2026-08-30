//! Per-clause outcomes and structured failure details.

use crate::ClauseId;

/// Semantic role of one contract clause.
///
/// Trace: TC-008, NFR-002-AC-3
// Implements: FR-001
///
/// ```compile_fail
/// use quire_contract_runtime::ClauseKind;
///
/// fn classify(kind: ClauseKind) -> u8 {
///     match kind {
///         ClauseKind::Precondition => 0,
///         ClauseKind::Postcondition => 1,
///         ClauseKind::Invariant => 2,
///         ClauseKind::Guard => 3,
///         ClauseKind::Consequent => 4,
///     }
/// }
/// ```
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClauseKind {
    /// Input-domain or state precondition.
    Precondition,
    /// Required result or post-state.
    Postcondition,
    /// Invariant checked at the execution point.
    Invariant,
    /// Guard controlling a case or implication.
    Guard,
    /// Consequent controlled by a guard.
    Consequent,
}

/// Observable result of evaluating one clause.
///
/// Trace: TC-008, NFR-002-AC-3
// Implements: FR-001
///
/// ```compile_fail
/// use quire_contract_runtime::ClauseOutcome;
///
/// fn classify(outcome: ClauseOutcome) -> u8 {
///     match outcome {
///         ClauseOutcome::Passed => 0,
///         ClauseOutcome::Failed => 1,
///         ClauseOutcome::Rejected => 2,
///         ClauseOutcome::NotEvaluated => 3,
///         ClauseOutcome::Undefined => 4,
///     }
/// }
/// ```
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClauseOutcome {
    /// Clause evaluated to true.
    Passed,
    /// Clause evaluated to false and constitutes a failure.
    Failed,
    /// A precondition excluded this case from the accepted input domain.
    Rejected,
    /// Semantics did not require this clause to execute.
    NotEvaluated,
    /// A partial expression was undefined and was retained explicitly.
    Undefined,
}

/// Stable category for a failure or rejection.
///
/// Trace: TC-008, NFR-002-AC-3
// Implements: FR-001
///
/// ```compile_fail
/// use quire_contract_runtime::FailureKind;
///
/// fn classify(kind: FailureKind) -> u8 {
///     match kind {
///         FailureKind::Postcondition => 0,
///         FailureKind::Precondition => 1,
///         FailureKind::Undefined => 2,
///         FailureKind::Contract => 3,
///     }
/// }
/// ```
#[non_exhaustive]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FailureKind {
    /// A required postcondition evaluated to false.
    Postcondition,
    /// A precondition rejected the generated or supplied case.
    Precondition,
    /// An expression had no defined value.
    Undefined,
    /// A generated contract-specific failure code.
    Contract,
}

/// Allocation-free structured failure information.
///
// Implements: FR-001
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FailureDetail<'a> {
    /// Clause that produced this result.
    ///
    // Implements: FR-001
    pub clause: ClauseId<'a>,
    /// Stable category interpreted independently of human-readable text.
    ///
    // Implements: FR-001
    pub kind: FailureKind,
    /// Contract-specific numeric code. Zero is permitted and has no implicit meaning.
    ///
    // Implements: FR-001
    pub code: u32,
    /// Optional borrowed diagnostic; evidence processing must not depend on its wording.
    ///
    // Implements: FR-001
    pub message: Option<&'a str>,
}

impl<'a> FailureDetail<'a> {
    /// Creates structured failure information.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn new(
        clause: ClauseId<'a>,
        kind: FailureKind,
        code: u32,
        message: Option<&'a str>,
    ) -> Self {
        Self {
            clause,
            kind,
            code,
            message,
        }
    }
}

/// One allocation-free clause observation.
///
// Implements: FR-001
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Observation<'a> {
    /// Stable clause identity.
    ///
    // Implements: FR-001
    pub clause: ClauseId<'a>,
    /// Clause semantic role.
    ///
    // Implements: FR-001
    pub kind: ClauseKind,
    /// Observable evaluation result.
    ///
    // Implements: FR-001
    pub outcome: ClauseOutcome,
    /// Structured detail for a failure, rejection, or undefined result.
    ///
    // Implements: FR-001
    pub detail: Option<FailureDetail<'a>>,
}

impl<'a> Observation<'a> {
    /// Creates an observation.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn new(
        clause: ClauseId<'a>,
        kind: ClauseKind,
        outcome: ClauseOutcome,
        detail: Option<FailureDetail<'a>>,
    ) -> Self {
        Self {
            clause,
            kind,
            outcome,
            detail,
        }
    }
}
