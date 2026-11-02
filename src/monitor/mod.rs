// SPDX-License-Identifier: AGPL-3.0-or-later
//! Owned executable monitor programs (FR-276).
//!
//! The compiler lowers an admitted program into this representation once.
//! Source keys and semantic values come from the shared leaves. This module
//! does not parse a source package or reconstruct temporal operators.

mod bindings;
mod budget;
mod executor;
mod fields;
mod program;
mod validation;
mod vm;

pub use program::{
    Buffer, Entries, Expression, ExpressionId, Function, FunctionId, Layout, LocalId, Operation,
    Program, ProgramParts, ProgramRefusal, ProgramRefusalCode, TypeBinding,
};

pub use budget::{StepBudget, StepResource, StepResourceStop};
pub use executor::{
    build, AdmissionFailure, BuildPlan, ExecutionSite, ExecutorInvariantCode,
    ExecutorInvariantFailure, Monitor, MonitorBuildFailure, MonitorBuildFailureCode,
};
