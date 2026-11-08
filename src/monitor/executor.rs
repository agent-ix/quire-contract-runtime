// SPDX-License-Identifier: AGPL-3.0-or-later
//! Monitor ownership and initializer execution. Entry dispatch shares the same machine.

use alloc::{boxed::Box, rc::Rc};
use core::fmt;

use quire_exact::{FieldValue, NodeKey, Outcome, Value};
use quire_semantic_value::supplied_membership::{
    MembershipPathArena, PathArenaCapacity, SuppliedAdmissionError,
};

use super::{vm, ExpressionId, FunctionId, Program, StepBudget, StepResourceStop};

/// Immutable admitted program and explicit operating budget for one fresh monitor.
pub struct BuildPlan {
    /// The one retained executable owner, independent of driver preparation.
    pub program: Rc<Program>,
    /// Operating limits, without changing any semantic plan premise.
    pub budget: StepBudget,
}

/// An arena address qualified by its retained program owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionSite {
    /// An expression's source key and inline path remain in the retained arena.
    Expression(ExpressionId),
    /// A function's complete source key remains in the retained arena.
    Function(FunctionId),
}

impl From<vm::Site> for ExecutionSite {
    fn from(site: vm::Site) -> Self {
        match site {
            vm::Site::Expression(id) => Self::Expression(id),
            vm::Site::Function(id) => Self::Function(id),
        }
    }
}

/// Stable class of an impossible evaluated-program condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutorInvariantCode {
    /// A retained arena address cannot be resolved.
    Address,
    /// An argument escaped its owning invocation.
    ArgumentOwner,
    /// An invocation has the wrong number of operands.
    Arity,
    /// A continuation did not leave exactly its one result.
    ValueStack,
    /// An evaluated condition or Boolean operand is not Boolean.
    Condition,
    /// A selected record or tuple slot broke its admitted projection contract.
    Projection,
    /// A native scalar operation did not complete under its admitted contract.
    Scalar,
    /// A lexical binding was inactive or belonged to a different invocation.
    Binding,
    /// A body produced a value outside its admitted signature.
    Membership,
    /// A returned State holds more buffer or activation elements than the plan admits.
    StateBound,
}

/// A terminal executor defect, distinct from resources and temporal outcomes.
pub struct ExecutorInvariantFailure {
    /// Stable machine-readable class.
    pub code: ExecutorInvariantCode,
    /// Address within the retained owner; it is not a semantic identity.
    pub site: ExecutionSite,
    program: Rc<Program>,
}

impl ExecutorInvariantFailure {
    /// Borrow the original full source identity without minting another key.
    pub fn source_node(&self) -> Option<&NodeKey> {
        source_node(&self.program, self.site)
    }
}

impl fmt::Debug for ExecutorInvariantFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutorInvariantFailure")
            .field("code", &self.code)
            .field("site", &self.site)
            .finish_non_exhaustive()
    }
}

/// Native shared value-admission failure and its still-resolvable path arena.
pub struct AdmissionFailure {
    /// Source operation whose conformance check stopped.
    pub site: ExecutionSite,
    /// Native distinction between invalid data, limits, unsupported helpers and cancellation.
    pub failure: SuppliedAdmissionError,
    /// The original arena; no diagnostic path is flattened into a string.
    pub arena: MembershipPathArena,
    program: Rc<Program>,
}

impl AdmissionFailure {
    /// Borrow the operation's original source identity.
    pub fn source_node(&self) -> Option<&NodeKey> {
        source_node(&self.program, self.site)
    }
}

impl fmt::Debug for AdmissionFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmissionFailure")
            .field("site", &self.site)
            .field("failure", &self.failure)
            .finish_non_exhaustive()
    }
}

/// Original shared scalar result and retained source evidence of a stopped operation.
///
/// Only non-completed outcomes appear here; no partial scalar value is exposed.
pub struct ScalarFailure {
    /// Source operation whose native evaluation stopped.
    pub site: ExecutionSite,
    /// Original typed refusal, undefined cause or native operating shortage.
    pub outcome: Outcome<Value>,
    program: Rc<Program>,
}

impl ScalarFailure {
    /// Borrow the operation's complete original source identity.
    pub fn source_node(&self) -> Option<&NodeKey> {
        source_node(&self.program, self.site)
    }
}

impl fmt::Debug for ScalarFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScalarFailure")
            .field("site", &self.site)
            .field("outcome", &self.outcome)
            .finish_non_exhaustive()
    }
}

/// Stable build or entry failure channel; no partial transition is committed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonitorFailureCode {
    /// Attached input data or its independent association was refused.
    Input,
    /// A source-body invariant has already terminated this monitor.
    Terminated,
    /// An operating budget or real storage request stopped initialization.
    Resource,
    /// An initializer broke the retained executable contract.
    Invariant,
    /// The shared conformance boundary returned its native failure.
    Admission,
}

/// A monitor failure is neither a Boolean verdict nor a temporal Incomplete/Failed.
#[derive(Debug)]
pub struct MonitorFailure {
    /// Stable category of the failed operation.
    pub code: MonitorFailureCode,
    /// Stable input-association refusal, when entry execution never began.
    pub input: Option<super::InputRefusal>,
    /// Present for an operating stop, preserving its actual request.
    pub resource: Option<Box<StepResourceStop>>,
    /// Present for a non-completed native scalar operation, retaining its original cause.
    pub scalar: Option<Box<ScalarFailure>>,
    /// Present for an evaluated-program defect.
    pub invariant: Option<Box<ExecutorInvariantFailure>>,
    /// Present for a native shared membership failure.
    pub admission: Option<Box<AdmissionFailure>>,
    /// Native path-arena identifier capacity refusal, before any arena exists.
    pub path_capacity: Option<Box<PathArenaCapacity>>,
}

impl fmt::Display for MonitorFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "monitor operation stopped: {:?}", self.code)
    }
}
impl core::error::Error for MonitorFailure {}

/// One initialized state slot bound to one immutable executable owner.
///
/// Canonical values are immutable; cloning their Arc-backed record does not
/// create shared mutable monitor state. Each build owns its own replacement slot.
pub struct Monitor {
    pub(super) program: Rc<Program>,
    pub(super) state: Value,
    pub(super) budget: StepBudget,
    pub(super) machine: vm::Machine,
    pub(super) baseline: Option<u64>,
    pub(super) progress: Option<quire_exact::Integer>,
    pub(super) terminated: bool,
}

impl Monitor {
    /// The complete canonical state returned by this monitor's initializer.
    pub fn state(&self) -> &Value {
        &self.state
    }
    /// Borrow this monitor's fixed program owner; no live rebind exists.
    pub fn program(&self) -> &Rc<Program> {
        &self.program
    }
    /// The explicitly selected operating limits.
    pub const fn step_budget(&self) -> StepBudget {
        self.budget
    }
    /// Replace operating limits atomically, preserving the program and state.
    ///
    /// A failed scratch reservation keeps the old machine and limits intact.
    pub fn set_step_budget(&mut self, budget: StepBudget) -> Result<(), MonitorFailure> {
        let machine = vm::Machine::new(&self.program, budget)
            .map_err(|failure| build_failure(&self.program, failure))?;
        self.machine = machine;
        self.budget = budget;
        Ok(())
    }
}

/// Retain the program, prepare bounded interpreter storage, and execute init once.
///
/// Build consumes no event and produces no obligation result. It does not promise
/// recovery from arbitrary allocator exhaustion. No driver borrow or host callback
/// completes the executable program.
pub fn build(plan: BuildPlan) -> Result<Monitor, MonitorFailure> {
    let mut machine = vm::Machine::new(&plan.program, plan.budget)
        .map_err(|failure| build_failure(&plan.program, failure))?;
    let value = machine
        .invoke(
            &plan.program,
            plan.program.layout().entries.init,
            &[],
            plan.budget,
        )
        .map_err(|failure| build_failure(&plan.program, failure))?;
    let Value::Composite(_) = value else {
        return Err(build_failure(
            &plan.program,
            vm::Failure::Invariant {
                site: vm::Site::Function(plan.program.layout().entries.init),
                cause: vm::Broken::ValueStack,
            },
        ));
    };
    let layout = plan.program.layout();
    if let Some(cause) = state_defect(layout, &value) {
        return Err(build_failure(
            &plan.program,
            vm::Failure::Invariant {
                site: vm::Site::Function(layout.entries.init),
                cause,
            },
        ));
    }
    let state = value;
    Ok(Monitor {
        program: plan.program,
        state,
        budget: plan.budget,
        machine,
        baseline: None,
        progress: None,
        terminated: false,
    })
}

/// Compare sequence lengths with the plan capacities.
///
/// Each buffer may hold its capacity plus the one summary cell of an unbounded
/// past operator; activations hold at most the activation capacity.
pub(super) fn lengths_within_bounds<'a>(
    mut buffers: impl Iterator<Item = (usize, &'a quire_exact::Integer)>,
    activations: (usize, &quire_exact::Integer),
) -> bool {
    buffers.all(|(len, capacity)| quire_exact::Integer::from(len.saturating_sub(1)) <= *capacity)
        && quire_exact::Integer::from(activations.0) <= *activations.1
}

/// Read a returned State's sequence lengths against the plan capacities.
///
/// `None` means the State lacks a planned sequence slot (a shape defect, not a
/// bound violation). Entries are never interpreted.
pub(super) fn state_within_bounds(
    buffer_plan: &[super::Buffer],
    activation_capacity: &quire_exact::Integer,
    state: &Value,
) -> Option<bool> {
    let Value::Composite(record) = state else {
        return None;
    };
    let slots = record.slots();
    // Slots: now, history_start, buffer_0.., activations, event_position.
    let length = |index: usize| match slots.get(index.saturating_add(2)) {
        Some(FieldValue::Present(Value::Collection(sequence))) => Some(sequence.elements().len()),
        _ => None,
    };
    let mut lengths = alloc::vec::Vec::new();
    lengths.try_reserve_exact(buffer_plan.len()).ok()?;
    for (index, buffer) in buffer_plan.iter().enumerate() {
        lengths.push((length(index)?, &buffer.capacity));
    }
    let activations = length(buffer_plan.len())?;
    Some(lengths_within_bounds(
        lengths.into_iter(),
        (activations, activation_capacity),
    ))
}

/// The invariant a returned State breaks, if any: a missing planned sequence
/// is a projection defect and only a length violation is `StateBound`.
pub(super) fn state_defect(layout: &super::Layout, state: &Value) -> Option<vm::Broken> {
    match state_within_bounds(&layout.buffers, &layout.activation_capacity, state) {
        Some(true) => None,
        Some(false) => Some(vm::Broken::StateBound),
        None => Some(vm::Broken::Projection),
    }
}

fn source_node(program: &Program, site: ExecutionSite) -> Option<&NodeKey> {
    match site {
        ExecutionSite::Expression(id) => program.expression(id).map(|node| &node.key),
        ExecutionSite::Function(id) => program.function(id).map(|node| &node.key),
    }
}

pub(super) fn build_failure(program: &Rc<Program>, failure: vm::Failure) -> MonitorFailure {
    let mut result = MonitorFailure {
        code: MonitorFailureCode::Resource,
        input: None,
        resource: None,
        scalar: None,
        invariant: None,
        admission: None,
        path_capacity: None,
    };
    match failure {
        vm::Failure::Input(native) => {
            let mut failure = build_failure(program, *native);
            if failure.code != MonitorFailureCode::Resource {
                failure.code = MonitorFailureCode::Input;
                failure.input = Some(super::InputRefusal::Values);
                failure.invariant = None;
            }
            return failure;
        }

        vm::Failure::Scalar { site, outcome } => {
            if let Outcome::Incomplete(cause) = outcome.as_ref() {
                result.resource = Some(Box::new(StepResourceStop {
                    resource: super::StepResource::Scalar(cause.limit_kind),
                    limit: Some(u128::from(cause.limit)),
                    spent: u128::from(cause.consumed),
                    requested: cause.next_charge.clone(),
                }));
            } else {
                result.code = MonitorFailureCode::Invariant;
                result.invariant = Some(Box::new(ExecutorInvariantFailure {
                    code: ExecutorInvariantCode::Scalar,
                    site: site.into(),
                    program: Rc::clone(program),
                }));
            }
            result.scalar = Some(Box::new(ScalarFailure {
                site: site.into(),
                outcome: *outcome,
                program: Rc::clone(program),
            }));
        }

        vm::Failure::Stop(stop) => result.resource = Some(Box::new(stop)),
        vm::Failure::PathCapacity(capacity) => {
            result.resource = Some(Box::new(StepResourceStop::storage(capacity.requested)));
            result.path_capacity = Some(Box::new(capacity));
        }
        vm::Failure::MembershipStop { stop, native } => {
            result.resource = Some(Box::new(stop));
            result.admission = Some(Box::new(AdmissionFailure {
                site: native.site.into(),
                failure: native.failure,
                arena: native.arena,
                program: Rc::clone(program),
            }));
        }
        vm::Failure::Invariant { site, cause } => {
            result.code = MonitorFailureCode::Invariant;
            let code = match cause {
                vm::Broken::Address => ExecutorInvariantCode::Address,
                vm::Broken::ArgumentOwner => ExecutorInvariantCode::ArgumentOwner,
                vm::Broken::Arity => ExecutorInvariantCode::Arity,
                vm::Broken::ValueStack => ExecutorInvariantCode::ValueStack,
                vm::Broken::Condition => ExecutorInvariantCode::Condition,
                vm::Broken::Binding => ExecutorInvariantCode::Binding,
                vm::Broken::Projection => ExecutorInvariantCode::Projection,
                vm::Broken::StateBound => ExecutorInvariantCode::StateBound,
            };
            result.invariant = Some(Box::new(ExecutorInvariantFailure {
                code,
                site: site.into(),
                program: Rc::clone(program),
            }));
        }
        vm::Failure::Membership(native) => {
            let vm::MembershipFailure {
                site,
                failure,
                arena,
            } = *native;
            result.code = MonitorFailureCode::Admission;
            if matches!(failure, SuppliedAdmissionError::Invalid { .. }) {
                result.code = MonitorFailureCode::Invariant;
                result.invariant = Some(Box::new(ExecutorInvariantFailure {
                    code: ExecutorInvariantCode::Membership,
                    site: site.into(),
                    program: Rc::clone(program),
                }));
            }
            result.admission = Some(Box::new(AdmissionFailure {
                site: site.into(),
                failure,
                arena,
                program: Rc::clone(program),
            }));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_exact::Integer;

    fn within(buffer: usize, activations: usize) -> bool {
        let capacity = Integer::from(22_u64);
        lengths_within_bounds(
            core::iter::once((buffer, &capacity)),
            (activations, &Integer::from(2_u64)),
        )
    }

    #[trace("TC-200", "FR-276-AC-12")]
    #[test]
    fn state_bound_admits_capacity_plus_summary_cell_and_activation_capacity() {
        assert!(within(0, 0));
        assert!(within(22, 2));
        assert!(within(23, 2));
    }

    #[trace("TC-200", "FR-276-AC-12")]
    #[test]
    fn state_bound_refuses_one_buffer_element_or_activation_beyond() {
        assert!(!within(24, 2));
        assert!(!within(23, 3));
    }
}
