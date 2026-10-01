// SPDX-License-Identifier: AGPL-3.0-or-later
//! The early-exit carrier the runtime's residue modules use between the kernel's `Outcome`
//! and `?`.
//!
//! `quire-exact` keeps its own carrier private, so the residue (function application, checking
//! environments, collections over the runtime's own `Value`) needs one of its own to write a
//! stop-propagating body. It is plumbing, not an exported item: it never appears in a public
//! signature, and every public operation converts it into the kernel's [`Outcome`].

use quire_exact::{Incomplete, Outcome, Refusal, Undefined};

/// A stop of any of the three non-completing kinds, carried through `?`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Stop {
    /// The operation has no mathematical value.
    Undefined(Undefined),
    /// The operation is defined but its result is not admitted.
    Refused(Refusal),
    /// A named charge was unavailable.
    Incomplete(Incomplete),
}

impl From<Incomplete> for Stop {
    fn from(record: Incomplete) -> Self {
        Self::Incomplete(record)
    }
}

/// Conversion between the kernel's [`Outcome`] and the `?`-friendly `Result<T, Stop>`.
pub(crate) trait OutcomeStop<T>: Sized {
    /// An `Outcome` from a `Result` carrying a stop.
    fn from_stop(result: Result<T, Stop>) -> Self;

    /// The completed value, or the stop the outcome carried.
    fn into_stop(self) -> Result<T, Stop>;
}

impl<T> OutcomeStop<T> for Outcome<T> {
    fn from_stop(result: Result<T, Stop>) -> Self {
        match result {
            Ok(value) => Self::Completed(value),
            Err(Stop::Undefined(reason)) => Self::Undefined(reason),
            Err(Stop::Refused(reason)) => Self::Refused(reason),
            Err(Stop::Incomplete(record)) => Self::Incomplete(record),
        }
    }

    fn into_stop(self) -> Result<T, Stop> {
        match self {
            Self::Completed(value) => Ok(value),
            Self::Undefined(reason) => Err(Stop::Undefined(reason)),
            Self::Refused(reason) => Err(Stop::Refused(reason)),
            Self::Incomplete(record) => Err(Stop::Incomplete(record)),
        }
    }
}
