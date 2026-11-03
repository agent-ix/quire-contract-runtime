// SPDX-License-Identifier: AGPL-3.0-or-later
//! Borrowed original input associations and owned canonical step results.

use super::{MonitorFailure, MonitorFailureCode, Program};
use alloc::rc::Rc;
use quire_exact::{Integer, NodeKey, Value};

/// Original independently admitted association attached to an input.
///
/// The driver validates source authority before transfer. These canonical value
/// encodings are not document text, hashes or newly allocated semantic identities.
#[derive(Clone, Copy)]
pub struct InputContext<'a> {
    /// Original canonical selected package-lock bytes.
    pub package_lock: &'a [u8],
    /// Original selected clause key.
    pub clause: &'a NodeKey,
    /// Independent selected subject identity.
    pub subject: &'a str,
    /// Original selected occurrence value.
    pub occurrence: &'a [u8],
    /// Complete canonical admitted clock-binding value.
    pub clock_binding: &'a [u8],
    /// Hardware counter identity.
    pub counter: &'a str,
    /// Original Time unit key.
    pub tick_unit: &'a NodeKey,
    /// Declared hardware width.
    pub counter_width_bits: u8,
    /// Exact selected profile identity.
    pub profile: &'a str,
    /// Original admitted causal/sequence order value.
    pub admitted_order: &'a [u8],
}

/// One hardware reading with its independently admitted association.
#[derive(Clone, Copy)]
pub struct Reading<'a> {
    /// Original source association.
    pub context: InputContext<'a>,
    /// Raw value, never truncated to fit the declared width.
    pub value: u64,
}

/// An admitted event, distinct from merely polling a counter.
pub struct Event<'a> {
    /// Associated raw reading.
    pub reading: Reading<'a>,
    /// Original canonical captures, independently admitted for this event.
    pub inputs: &'a Value,
}

/// An already-admitted inclusive progress assertion.
///
/// Admission owns authority, coverage and FR-289 half-open mapping. EOF or a
/// bare endpoint is not an admitted Watermark.
pub struct Watermark<'a> {
    /// Original source association.
    pub context: InputContext<'a>,
    /// Stable original clock/subject/source-set/scope/execution association.
    pub progress_context: &'a [u8],
    /// Complete original admitted authority assertion, retained without parsing.
    pub assertion: &'a [u8],
    /// Inclusive coverage in the admitted unwrapped tick coordinate.
    pub inclusive_boundary: &'a Integer,
    /// Canonical source Scope value bound by the supplying admission.
    pub scope: &'a Value,
}

/// Stable input refusal before any source entry is executed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputRefusal {
    /// A source invariant has already terminated this monitor.
    Terminated,
    /// Foreign selected package lock.
    Package,
    /// Foreign selected clause.
    Clause,
    /// Foreign subject.
    Subject,
    /// Foreign occurrence.
    Occurrence,
    /// Different canonical clock-binding value.
    ClockBinding,
    /// Different hardware counter identity.
    Counter,
    /// Different tick unit.
    Unit,
    /// Different counter width.
    Width,
    /// Different profile identity.
    Profile,
    /// Different or absent admitted order value.
    Order,
    /// Raw reading exceeds its declared width.
    ReadingRange,
    /// Foreign stable progress association.
    ProgressContext,
    /// Original admitted authority assertion is absent.
    MissingProgress,
    /// The inclusive boundary regresses.
    ProgressRegression,
    /// A required canonical entry value was not admitted.
    Values,
}

/// Original source Results, qualified by the retained immutable program owner.
pub struct MonitorStep {
    pub(super) program: Rc<Program>,
    pub(super) results: Value,
}

impl MonitorStep {
    /// Original canonical Results, never a synthesized verdict sequence.
    pub fn results(&self) -> &Value {
        &self.results
    }
    /// Complete original program association; no replacement PlanId is created.
    pub fn program(&self) -> &Rc<Program> {
        &self.program
    }
}

pub(super) fn refused(code: InputRefusal) -> MonitorFailure {
    MonitorFailure {
        code: if code == InputRefusal::Terminated {
            MonitorFailureCode::Terminated
        } else {
            MonitorFailureCode::Input
        },
        input: Some(code),
        resource: None,
        scalar: None,
        invariant: None,
        admission: None,
        path_capacity: None,
    }
}

pub(super) fn context(program: &Program, value: InputContext<'_>) -> Result<(), MonitorFailure> {
    let expected = program.layout();
    let code = if value.package_lock != expected.package_lock.as_ref() {
        Some(InputRefusal::Package)
    } else if value.clause != &expected.clause {
        Some(InputRefusal::Clause)
    } else if value.subject != expected.subject {
        Some(InputRefusal::Subject)
    } else if value.occurrence != expected.occurrence.as_ref() {
        Some(InputRefusal::Occurrence)
    } else if value.clock_binding != expected.clock_binding.as_ref() {
        Some(InputRefusal::ClockBinding)
    } else if value.counter != expected.counter {
        Some(InputRefusal::Counter)
    } else if value.tick_unit != &expected.tick_unit {
        Some(InputRefusal::Unit)
    } else if value.counter_width_bits != expected.counter_width_bits {
        Some(InputRefusal::Width)
    } else if value.profile != expected.profile {
        Some(InputRefusal::Profile)
    } else if value.admitted_order != expected.admitted_order.as_ref() {
        Some(InputRefusal::Order)
    } else {
        None
    };
    match code {
        Some(code) => Err(refused(code)),
        None => Ok(()),
    }
}

pub(super) fn reading(program: &Program, value: Reading<'_>) -> Result<(), MonitorFailure> {
    context(program, value.context)?;
    let width = program.layout().counter_width_bits;
    if width < 64
        && value.value
            >= 1_u64
                .checked_shl(u32::from(width))
                .ok_or_else(|| refused(InputRefusal::Width))?
    {
        return Err(refused(InputRefusal::ReadingRange));
    }
    Ok(())
}
