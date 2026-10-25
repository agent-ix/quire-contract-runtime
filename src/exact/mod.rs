// SPDX-License-Identifier: AGPL-3.0-or-later
//! Remaining exact-semantic expression residue.
//!
//! Implements: FR-012, FR-143, FR-144, FR-146, FR-149, FR-273.
//!
//! Exact scalar values, operations, outcomes and metering belong to `quire-exact` (FR-275).
//! Consumers import those directly from the kernel. This module still defines the
//! ported QSL expression code pending replacement by the shared evaluator. That
//! residue is an open FR-275 defect. It is built over kernel scalars and metering.
//!
//! Trace: interface-001-AC-2, interface-001-AC-4
//!
//! ```compile_fail
//! use quire_contract_runtime::exact::Integer;
//! ```
//!
//! The quire-specification/FR-146 function-application surface includes
//! [`PackageDeclarations::check`] and [`CheckedPackage::call`]/
//! [`CheckedPackage::evaluate`], ported to the boundary AD-002 sets.
//!
//! Node keys, definition-lock selection, owner joins, stale keys and package
//! admission are compiler work. The runtime consumes admitted keys and
//! profiles, and consumes the compiler's closed refusal vocabularies
//! through `quire_semantic_value::definition` and
//! `quire_semantic_value::semantic_node`, so a compiler refusal is reported
//! with its exact code. No host floating-point
//! operation, panic path or ambient effect exists on any semantic path.
//!
//! Composite, collection, equality, key, containment and reference values
//! are consumed directly from `quire-exact` and `quire-semantic-value`.
//! [`plan_call`] and [`plan_evaluation`] use the shared object closure.

mod expression;

pub use expression::{
    plan_call, plan_evaluation, Body, BodyState, BodyStep, CallPlan, CheckCause, CheckMode,
    CheckRefusal, CheckedExpression, CheckedPackage, CheckingLimits, Evaluation, EvaluationRefusal,
    Frame, FunctionDeclaration, InputRefusal, LocatedLoss, Location, Origin, PackageDeclarations,
    ValueLoss,
};
