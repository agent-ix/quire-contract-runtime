//! Per-clause outcomes and structured failure details.

use crate::ClauseId;

/// Semantic role of one contract clause.
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FailureDetail<'a> {
    /// Clause that produced this result.
    pub clause: ClauseId<'a>,
    /// Stable category interpreted independently of human-readable text.
    pub kind: FailureKind,
    /// Contract-specific numeric code. Zero is permitted and has no implicit meaning.
    pub code: u32,
    /// Optional borrowed diagnostic; evidence processing must not depend on its wording.
    pub message: Option<&'a str>,
}

impl<'a> FailureDetail<'a> {
    /// Creates structured failure information.
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Observation<'a> {
    /// Stable clause identity.
    pub clause: ClauseId<'a>,
    /// Clause semantic role.
    pub kind: ClauseKind,
    /// Observable evaluation result.
    pub outcome: ClauseOutcome,
    /// Structured detail for a failure, rejection, or undefined result.
    pub detail: Option<FailureDetail<'a>>,
}

impl<'a> Observation<'a> {
    /// Creates an observation.
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
