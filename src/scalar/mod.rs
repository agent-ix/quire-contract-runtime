// SPDX-License-Identifier: AGPL-3.0-or-later
//! Runtime-owned scalar negotiation and lazy Boolean evaluation.
//!
//! Implements: FR-006, FR-009, FR-275, interface-001-AC-8.
//!
//! The exact value and operation kernel lives in `quire-exact`. This module owns
//! the I13 backend negotiators and the lazy, stop-carrying Boolean connective.

mod boolean;
mod division;
mod ieee;

pub use boolean::{evaluate_boolean_short_circuit, ShortCircuitConnective};
pub use division::{
    negotiate_integer_division, IntegerDivisionBounds, IntegerDivisionConsumer,
    IntegerDivisionDisposition,
};
pub use ieee::{
    negotiate_ieee, IeeeBackendCapabilities, IeeeDisposition, IeeeItemRequirement,
    IeeeUnsupportedCause,
};
