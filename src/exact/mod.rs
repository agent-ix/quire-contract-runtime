// SPDX-License-Identifier: AGPL-3.0-or-later
//! Runtime-owned negotiation and remaining exact-semantic residue.
//!
//! Implements: FR-009, FR-012, FR-143, FR-144, FR-146, FR-149, FR-273.
//!
//! Exact scalar values, operations, outcomes and metering belong to `quire-exact` (FR-275).
//! Consumers import those directly from the kernel. This module still defines
//!
//! - the runtime-owned I13 negotiators ([`negotiate_integer_division`], [`negotiate_ieee`] and
//!   their types; FR-009, interface-001-AC-8), and
//! - the remaining ported QSL code, pending removal under QSL-358: quantity, unit,
//!   enumeration, expression and carried-vocabulary modules. This residue is an open
//!   FR-275 defect. It is built over kernel scalars and metering.
//!
//! Trace: interface-001-AC-2, interface-001-AC-4
//!
//! ```compile_fail
//! use quire_contract_runtime::exact::Integer;
//! ```
//!
//! Layers:
//!
//! 1. the runtime's remaining quantity and enum residue: [`EnumValue`] and [`Quantity`]
//!    over a [`UnitGraph`] ([`compare_enum`], [`evaluate_quantity`],
//!    [`compare_quantity`], [`convert_quantity`]);
//! 2. [`evaluate_boolean_short_circuit`], the stop-carrying lazy-right connective;
//! 3. the quire-specification/FR-146 function-application surface:
//!    [`PackageDeclarations::check`] and [`CheckedPackage::call`]/
//!    [`CheckedPackage::evaluate`], ported to the boundary AD-002 sets.
//!
//! Node keys, definition-lock selection, owner joins, stale keys and package
//! admission are compiler work. The runtime consumes admitted keys and
//! profiles, and carries the closed refusal vocabularies
//! through its remaining definition vocabulary and
//! `quire_semantic_value::semantic_node`, so a compiler refusal is reported
//! with its exact code. No host floating-point
//! operation, panic path or ambient effect exists on any semantic path.
//!
//! Composite, collection, equality, key, containment and reference values
//! are consumed directly from `quire-exact` and `quire-semantic-value`.
//! [`plan_call`] and [`plan_evaluation`] use the shared object closure.

mod boolean;
mod definition;
mod division;
mod enumeration;
mod expression;
mod ieee;
mod quantity;
mod stop;
mod unit;

pub use boolean::{evaluate_boolean_short_circuit, ShortCircuitConnective};
pub use definition::{PackageCause, PackageRefusal, PackageRefusalCode, SelectionRefusalCode};
pub use division::{
    negotiate_integer_division, IntegerDivisionBounds, IntegerDivisionConsumer,
    IntegerDivisionDisposition,
};
pub use enumeration::{compare_enum, EnumDeclaration, EnumValue};
pub use expression::{
    plan_call, plan_evaluation, Body, CallPlan, CheckCause, CheckMode, CheckRefusal,
    CheckedExpression, CheckedPackage, CheckingLimits, DepthAboveMaximum, Evaluation,
    EvaluationRefusal, Frame, FunctionDeclaration, InputRefusal, LocatedLoss, Location, Origin,
    PackageDeclarations, ValueLoss, MAX_CALL_DEPTH,
};
pub use ieee::{
    negotiate_ieee, IeeeBackendCapabilities, IeeeDisposition, IeeeItemRequirement,
    IeeeUnsupportedCause,
};
pub use quantity::{
    compare_quantity, convert_quantity, evaluate_quantity, Conversion, ConvertedValue, Quantity,
    QuantityOperation, QuantityTarget, QuantityUnit,
};
pub use unit::{
    CompoundUnit, CompoundUnitCause, Dimension, InvalidCompoundUnit, Unit, UnitDeclaration,
    UnitEdge, UnitGraph, COMPOUND_UNIT_DOMAIN,
};
