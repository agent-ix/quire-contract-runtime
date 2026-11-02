// SPDX-License-Identifier: AGPL-3.0-or-later
//! Monitor ownership and initializer execution. Entry dispatch shares the same machine.

use alloc::{boxed::Box, rc::Rc};
use core::fmt;

use quire_exact::{NodeKey, Value};
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
    /// An evaluated condition is not Boolean.
    Condition,
    /// A body produced a value outside its admitted signature.
    Membership,
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

/// Stable initialization failure channel; no partial monitor is returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MonitorBuildFailureCode {
    /// An operating budget or real storage request stopped initialization.
    Resource,
    /// An initializer broke the retained executable contract.
    Invariant,
    /// The shared conformance boundary returned its native failure.
    Admission,
}

/// A build refusal is neither a Boolean verdict nor a temporal Incomplete/Failed.
#[derive(Debug)]
pub struct MonitorBuildFailure {
    /// Stable category of the failed build.
    pub code: MonitorBuildFailureCode,
    /// Present for an operating stop, preserving its actual request.
    pub resource: Option<StepResourceStop>,
    /// Present for an evaluated-program defect.
    pub invariant: Option<Box<ExecutorInvariantFailure>>,
    /// Present for a native shared membership failure.
    pub admission: Option<Box<AdmissionFailure>>,
    /// Native path-arena identifier capacity refusal, before any arena exists.
    pub path_capacity: Option<Box<PathArenaCapacity>>,
}

impl fmt::Display for MonitorBuildFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "monitor build stopped: {:?}", self.code)
    }
}
impl core::error::Error for MonitorBuildFailure {}

/// One initialized state slot bound to one immutable executable owner.
///
/// Canonical values are immutable; cloning their Arc-backed record does not
/// create shared mutable monitor state. Each build owns its own replacement slot.
pub struct Monitor {
    program: Rc<Program>,
    state: Value,
    budget: StepBudget,
    machine: vm::Machine,
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
    pub fn set_step_budget(&mut self, budget: StepBudget) -> Result<(), MonitorBuildFailure> {
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
pub fn build(plan: BuildPlan) -> Result<Monitor, MonitorBuildFailure> {
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
    let state = value.clone();
    Ok(Monitor {
        program: plan.program,
        state,
        budget: plan.budget,
        machine,
    })
}

fn source_node(program: &Program, site: ExecutionSite) -> Option<&NodeKey> {
    match site {
        ExecutionSite::Expression(id) => program.expression(id).map(|node| &node.key),
        ExecutionSite::Function(id) => program.function(id).map(|node| &node.key),
    }
}

fn build_failure(program: &Rc<Program>, failure: vm::Failure) -> MonitorBuildFailure {
    let mut result = MonitorBuildFailure {
        code: MonitorBuildFailureCode::Resource,
        resource: None,
        invariant: None,
        admission: None,
        path_capacity: None,
    };
    match failure {
        vm::Failure::Stop(stop) => result.resource = Some(stop),
        vm::Failure::PathCapacity(capacity) => {
            result.resource = Some(StepResourceStop::storage(capacity.requested));
            result.path_capacity = Some(Box::new(capacity));
        }
        vm::Failure::MembershipStop { stop, native } => {
            result.resource = Some(stop);
            result.admission = Some(Box::new(AdmissionFailure {
                site: native.site.into(),
                failure: native.failure,
                arena: native.arena,
                program: Rc::clone(program),
            }));
        }
        vm::Failure::Invariant { site, cause } => {
            result.code = MonitorBuildFailureCode::Invariant;
            let code = match cause {
                vm::Broken::Address => ExecutorInvariantCode::Address,
                vm::Broken::ArgumentOwner => ExecutorInvariantCode::ArgumentOwner,
                vm::Broken::Arity => ExecutorInvariantCode::Arity,
                vm::Broken::ValueStack => ExecutorInvariantCode::ValueStack,
                vm::Broken::Condition => ExecutorInvariantCode::Condition,
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
            result.code = MonitorBuildFailureCode::Admission;
            if matches!(failure, SuppliedAdmissionError::Invalid { .. }) {
                result.code = MonitorBuildFailureCode::Invariant;
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
