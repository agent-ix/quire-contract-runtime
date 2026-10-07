// SPDX-License-Identifier: AGPL-3.0-or-later
//! The I13 negotiation disposition of a finite integer-division consumer.
//!
//! Runtime-owned (FR-009, FR-275, interface-001-AC-8): QSL deleted its negotiators on purpose,
//! so this is the runtime's own code, not a port. The `div`/`rem`/`mod` laws themselves
//! (`DivisionProfile`, `divide`, `modulo`) are the `quire-exact` kernel's.

use alloc::vec::Vec;

use quire_exact::IntegerInterval;

/// The declared bounds a finite I13 consumer offers for one `div`/`rem` or
/// `mod` item. An absent member is a missing bound.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IntegerDivisionBounds {
    /// Bound on both operands.
    pub operand: Option<IntegerInterval>,
    /// Bound on the exact intermediate quotient and remainder.
    pub intermediate: Option<IntegerInterval>,
    /// Bound on the exposed results.
    pub result: Option<IntegerInterval>,
}

impl IntegerDivisionBounds {
    fn complete(&self) -> bool {
        self.operand.is_some() && self.intermediate.is_some() && self.result.is_some()
    }
}

/// The consumer of one integer-division item at I13 negotiation.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerDivisionConsumer {
    /// Unbounded mathematical integers; no bound is needed.
    Mathematical,
    /// A finite backend with its declared bounds.
    Finite(IntegerDivisionBounds),
}

/// The per-item I13 negotiation disposition. It is not an evaluator outcome:
/// negotiation never evaluates and never narrows mathematical integers.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IntegerDivisionDisposition {
    /// The consumer can execute the item.
    Supported,
    /// `requires-bound`: a finite consumer lacks an operand, intermediate or
    /// result bound.
    RequiresBound,
}

/// Negotiate each item independently, before and without evaluation.
pub fn negotiate_integer_division(
    items: &[IntegerDivisionConsumer],
) -> Vec<IntegerDivisionDisposition> {
    items
        .iter()
        .map(|consumer| match consumer {
            IntegerDivisionConsumer::Mathematical => IntegerDivisionDisposition::Supported,
            IntegerDivisionConsumer::Finite(bounds) if bounds.complete() => {
                IntegerDivisionDisposition::Supported
            }
            IntegerDivisionConsumer::Finite(_) => IntegerDivisionDisposition::RequiresBound,
        })
        .collect()
}
