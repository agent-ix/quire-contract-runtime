// SPDX-License-Identifier: AGPL-3.0-or-later
//! Operating limits, independent of the admitted plan's semantic capacities.

use quire_exact::{Integer, LimitKind, ScalarLimits};

/// Per-invocation operating limits supplied when building a monitor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StepBudget {
    /// Continuation dispatches plus shared supplied-membership events.
    pub work: u64,
    /// Explicit native scalar limits; each counter retains its shared unit.
    pub scalar: ScalarLimits,
    /// Maximum live interpreter continuations.
    pub stack_frames: usize,
    /// Live operand addresses plus cumulative native membership reservation elements.
    pub scratch_elements: usize,
    /// Cumulative byte reservations made by shared supplied membership.
    pub scratch_bytes: usize,
}

/// The operating resource that stopped an invocation before commit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepResource {
    /// A shared scalar counter, distinct from semantic monitor outcome limits.
    Scalar(LimitKind),
    /// The next interpreter or membership work event.
    Work,
    /// A continuation would exceed the explicit stack.
    Stack,
    /// Operand or native membership element storage.
    ScratchElements,
    /// Native membership byte storage.
    ScratchBytes,
    /// A representable allocation or native storage identity was unavailable.
    Storage,
}

/// A non-verdict operating stop; no transition may commit after it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepResourceStop {
    /// Resource that stopped progress.
    pub resource: StepResource,
    /// Configured ceiling, absent for an allocator/storage refusal.
    pub limit: Option<u128>,
    /// Successful use before the denied request.
    pub spent: u128,
    /// Denied additional request in that resource's native unit.
    pub requested: Integer,
}

impl StepResourceStop {
    pub(super) fn storage(requested: usize) -> Self {
        Self {
            resource: StepResource::Storage,
            limit: None,
            spent: 0,
            requested: Integer::from(requested),
        }
    }
}
