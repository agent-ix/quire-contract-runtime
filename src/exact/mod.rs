// SPDX-License-Identifier: AGPL-3.0-or-later
//! Runtime-owned negotiation and temporary exact-semantic residue.
//!
//! Implements: FR-009, FR-012, FR-143, FR-144, FR-146, FR-149, FR-273.
//!
//! Exact scalar values, operations, outcomes and metering belong to `quire-exact` (FR-275).
//! Consumers import those directly from the kernel. This module still defines
//!
//! - the runtime-owned I13 negotiators ([`negotiate_integer_division`], [`negotiate_ieee`] and
//!   their types; FR-009, interface-001-AC-8), and
//! - the remaining ported QSL code, pending removal under QSL-358: the composite, collection,
//!   equality, quantity, unit, enumeration, containment, expression and carried-vocabulary
//!   modules. This residue is an open FR-275 defect, not an approved exception. It is built over
//!   kernel scalars and metering and is consumed by [`Value`].
//!
//! Trace: interface-001-AC-2, interface-001-AC-4
//!
//! ```compile_fail
//! use quire_contract_runtime::exact::Integer;
//! ```
//!
//! Layers:
//!
//! 1. the runtime's residue over kernel scalars: [`EnumValue`] and [`Quantity`] over a [`UnitGraph`]
//!    ([`compare_enum`], [`evaluate_quantity`], [`compare_quantity`], [`convert_quantity`]),
//!    and [`evaluate_boolean_short_circuit`], the stop-carrying lazy-right connective the
//!    kernel does not export;
//! 2. quire-specification/FR-143 composite values: [`TypeEnvironment`], [`Value`], [`ValueType`]
//!    and the [`ValueGraph`] finite-value constructor, over terminal
//!    [`ObjectReference`] identities;
//! 3. quire-specification/FR-144 collections: [`CollectionType`], [`CollectionValue`] and
//!    [`construct_collection`]/[`form_collection`], keyed by the quire-specification/FR-144
//!    canonical key that [`CheckedEquality`] and collection membership share;
//! 4. the quire-specification/FR-149 equality matrix: [`TypeEnvironment::check_equality`] and
//!    [`CheckedEquality::evaluate`];
//! 5. the quire-specification/FR-146 function-application surface:
//!    [`PackageDeclarations::check`] and [`CheckedPackage::call`]/
//!    [`CheckedPackage::evaluate`], ported to the boundary AD-002 sets.
//!
//! Node keys, definition-lock selection, owner joins, stale keys and package
//! admission are compiler work. The runtime consumes admitted keys and
//! profiles, and carries the closed refusal vocabularies
//! ([`SelectionRefusalCode`], [`PackageRefusal`], [`SemanticGraphCause`]) so a
//! compiler refusal is reported with its exact code. No host floating-point
//! operation, panic path or ambient effect exists on any semantic path.
//!
//! Every algorithm that walks a [`Value`] (the canonical key, the equality
//! occurrence-pair plan, collection membership and coalescing, containment
//! graph construction, and [`Value`]'s own hand-written
//! [`Debug`](core::fmt::Debug) and `Drop`) is iterative over an explicit
//! worklist, so value depth never reaches the host stack. In practice, depth
//! is bounded only by the deployment's `value_occurrences` limit; a generous
//! limit admits a nesting deep enough that a *recursive* walk would overflow
//! the host stack, which is silent corruption rather than a panic on the
//! governed `thumbv7em-none-eabi` target — which is why `Debug` and `Drop` are hand-written rather than derived,
//! the same as every other value-walking algorithm here.
//! [`Value`] shares nested composite and collection values through
//! [`alloc::rc::Rc`], never `Arc`: this crate has no concurrency and no
//! `target_has_atomic` requirement. [`ObjectEnvironment`] closes a set of
//! objects against a bound model snapshot's references, and is what
//! [`plan_call`]/[`plan_evaluation`] check every function argument's
//! references against.

mod boolean;
mod collection;
mod composite;
mod containment;
mod definition;
mod division;
mod enumeration;
mod equality;
mod expression;
mod ieee;
mod key;
mod node;
mod quantity;
mod reference;
mod stop;
mod unit;

pub use boolean::{evaluate_boolean_short_circuit, ShortCircuitConnective};
pub use collection::{construct_collection, form_collection, CollectionType, CollectionValue};
pub use composite::{
    Component, CompositeDeclaration, CompositeShape, CompositeValue, ConstructionCause,
    ConstructionRefusal, DeclarationCause, Deferred, FieldDeclaration, FieldExpression, FieldValue,
    InvalidDeclaration, ObjectTypeDeclaration, OptionValue, Presence, RecursionEdges,
    TypeEnvironment, Value, ValueType,
};
pub use containment::{GraphCause, GraphNode, GraphNodeId, GraphRefusal, GraphSlot, ValueGraph};
pub use definition::{PackageCause, PackageRefusal, PackageRefusalCode, SelectionRefusalCode};
pub use division::{
    negotiate_integer_division, IntegerDivisionBounds, IntegerDivisionConsumer,
    IntegerDivisionDisposition,
};
pub use enumeration::{compare_enum, EnumDeclaration, EnumValue};
pub use equality::{
    admits_equality_conversion, plan_equality, CheckedEquality, EqualityOperand, EqualityOperator,
    EqualityPlan, EqualitySchedule,
};
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
pub use node::{InvalidSemanticGraph, NodeKey, SemanticGraphCause, NODE_KEY_DOMAIN};
pub use quantity::{
    compare_quantity, convert_quantity, evaluate_quantity, Conversion, ConvertedValue, Quantity,
    QuantityOperation, QuantityTarget, QuantityUnit,
};
pub use reference::{
    InvalidObjectIdentity, ObjectEnvironment, ObjectEnvironmentCause, ObjectEnvironmentRefusal,
    ObjectIdentity, ObjectReference,
};
pub use unit::{
    CompoundUnit, CompoundUnitCause, Dimension, InvalidCompoundUnit, Unit, UnitDeclaration,
    UnitEdge, UnitGraph, COMPOUND_UNIT_DOMAIN,
};
