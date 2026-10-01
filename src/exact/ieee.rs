// SPDX-License-Identifier: AGPL-3.0-or-later
//! The I13 negotiation disposition of an IEEE backend.
//!
//! Runtime-owned (FR-009, FR-275, interface-001-AC-8): QSL deleted its negotiators on purpose,
//! so this is the runtime's own code, not a port. The IEEE values, operations and conversions
//! themselves are the `quire-exact` kernel's.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use quire_exact::{IeeeOperationKind, IeeeWidth, RoundingMode};

/// A backend's IEEE capabilities offered during I13 negotiation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IeeeBackendCapabilities {
    /// Supported widths.
    pub widths: BTreeSet<IeeeWidth>,
    /// Supported operations.
    pub operations: BTreeSet<IeeeOperationKind>,
    /// Supported rounding directions, including strict `exact` when offered.
    pub roundings: BTreeSet<RoundingMode>,
    /// Whether the backend implements this profile's NaN and flag policy.
    pub exceptional_policy: bool,
    /// Whether the backend can discharge a required finite resource proof.
    pub finite_proof: bool,
}

/// One package item's IEEE requirement.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IeeeItemRequirement {
    /// Selected width.
    pub width: IeeeWidth,
    /// Required operation.
    pub operation: IeeeOperationKind,
    /// Selected rounding policy (ignored by non-rounding operations).
    pub rounding: RoundingMode,
    /// Whether finite execution needs a resource proof for this item.
    pub requires_finite_proof: bool,
}

/// What the backend lacks for an `unsupported` item.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IeeeUnsupportedCause {
    /// The selected width.
    Width(IeeeWidth),
    /// The required operation or intrinsic.
    Operation(IeeeOperationKind),
    /// The selected rounding direction or strict policy.
    Rounding(RoundingMode),
    /// The NaN/flag policy.
    ExceptionalPolicy,
}

/// The per-item I13 negotiation disposition. It is not an evaluator outcome
/// and never changes package admission or selects a substitute evaluator.
///
/// Trace: TC-195, FR-273-AC-4, FR-009-AC-5
///
/// `IeeeDisposition` converts to neither `Outcome<Value>` nor `InputRefusal`:
/// no such `From`/`Into` impl exists, so this would-be conversion is a
/// compile error, not a runtime one.
///
/// ```compile_fail
/// use quire_contract_runtime::exact::{
///     IeeeDisposition, IeeeUnsupportedCause, IeeeWidth, Outcome, Value,
/// };
/// let disposition = IeeeDisposition::Unsupported(IeeeUnsupportedCause::Width(IeeeWidth::Binary64));
/// let _: Outcome<Value> = disposition.into();
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IeeeDisposition {
    /// The backend implements the item exactly.
    Supported,
    /// `unsupported`.
    Unsupported(IeeeUnsupportedCause),
    /// `requires-bound`.
    RequiresBound,
}

/// Negotiate each item independently against `backend`.
pub fn negotiate_ieee(
    items: &[IeeeItemRequirement],
    backend: &IeeeBackendCapabilities,
) -> Vec<IeeeDisposition> {
    items
        .iter()
        .map(|item| {
            let unsupported = if !backend.widths.contains(&item.width) {
                Some(IeeeUnsupportedCause::Width(item.width))
            } else if !backend.operations.contains(&item.operation) {
                Some(IeeeUnsupportedCause::Operation(item.operation))
            } else if item.operation.rounds() && !backend.roundings.contains(&item.rounding) {
                Some(IeeeUnsupportedCause::Rounding(item.rounding))
            } else if !backend.exceptional_policy {
                Some(IeeeUnsupportedCause::ExceptionalPolicy)
            } else {
                None
            };
            match unsupported {
                Some(cause) => IeeeDisposition::Unsupported(cause),
                None if item.requires_finite_proof && !backend.finite_proof => {
                    IeeeDisposition::RequiresBound
                }
                None => IeeeDisposition::Supported,
            }
        })
        .collect()
}
