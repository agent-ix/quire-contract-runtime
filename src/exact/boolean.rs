// SPDX-License-Identifier: AGPL-3.0-or-later
//! The stop-carrying short-circuit Boolean connective.
//!
//! `quire-exact` exports the eager connectives (`evaluate_boolean`) and `retain_boolean`, but no
//! connective whose right operand is lazy and may itself stop. Generated oracles call this one
//! (FR-014-AC-35 of the code generator), so it is a runtime-owned `exact` item that stays under
//! FR-275's `scalar` surface after the ported residue is removed.

use quire_exact::{retain_boolean, Meter, Outcome};

use super::stop::{OutcomeStop, Stop};

/// A short-circuiting Boolean connective kind: `and`, `or` or `implies`.
///
/// Unlike `quire_exact::BooleanConnective`, the right operand is evaluated lazily and may
/// itself stop; see [`evaluate_boolean_short_circuit`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ShortCircuitConnective {
    /// `a and b`.
    And,
    /// `a or b`.
    Or,
    /// `a implies b`.
    Implies,
}

/// Decide a short-circuiting connective whose right operand may stop.
///
/// `left` is always decided. When `left` alone determines the result (`false and _`, `true or _`,
/// `false implies _`), `right` is never called, no stop it could produce can arise, and the
/// decided result charges `boolean.result-retain` exactly once. Otherwise `right()` runs: if it
/// stops, that stop is returned unchanged and no `boolean.result-retain` charge is admitted; if it
/// completes, the combined result charges `boolean.result-retain` exactly once.
pub fn evaluate_boolean_short_circuit(
    connective: ShortCircuitConnective,
    left: bool,
    right: impl FnOnce() -> Outcome<bool>,
    meter: &mut Meter,
) -> Outcome<bool> {
    Outcome::from_stop(short_circuit(connective, left, right, meter))
}

fn short_circuit(
    connective: ShortCircuitConnective,
    left: bool,
    right: impl FnOnce() -> Outcome<bool>,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    let decided = match connective {
        ShortCircuitConnective::And if !left => Some(false),
        ShortCircuitConnective::Or if left => Some(true),
        ShortCircuitConnective::Implies if !left => Some(true),
        ShortCircuitConnective::And
        | ShortCircuitConnective::Or
        | ShortCircuitConnective::Implies => None,
    };
    let result = match decided {
        Some(result) => result,
        None => {
            let right_value = right().into_stop()?;
            match connective {
                ShortCircuitConnective::And => left && right_value,
                ShortCircuitConnective::Or => left || right_value,
                ShortCircuitConnective::Implies => right_value,
            }
        }
    };
    retain_boolean(result, meter).map_err(Stop::from)
}
