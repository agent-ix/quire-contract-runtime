// SPDX-License-Identifier: AGPL-3.0-or-later
//! Complete-V1 exact scalar oracle operators and typed runtime outcomes.
//!
//! Implements: FR-006, FR-007, FR-008, FR-273, FR-275.
//!
//! The exact scalar kernel, the kernel `Outcome` and the charge-before-work `Meter` are the
//! `quire-exact` crate's (FR-275): this module re-exports them at the path generated oracles
//! already import and defines none of them. What this module still defines is
//!
//! - the runtime-owned I13 negotiators ([`negotiate_integer_division`], [`negotiate_ieee`] and
//!   their types; FR-009, interface-001-AC-8), and
//! - the interim residue, kept only under the temporary exception FR-275 records (expiry:
//!   QSL-358 phase 2 merged): the composite, collection, equality, quantity, unit, enumeration,
//!   containment, expression and carried-vocabulary modules, which are built over the kernel
//!   scalars and `Meter`. They are consumed by [`Value`], and the kernel's own `Value` is not
//!   yet adopted (IR-349 part 1b).
//!
//! Layers:
//!
//! 1. kernel scalars (from `quire-exact`): [`Integer`], [`IntegerInterval`]/[`BoundedInteger`],
//!    [`Rational`], [`Decimal`], [`IeeeValue`], [`Text`], and the explicit operation tables
//!    [`evaluate_integer_arithmetic`], [`evaluate_rational_arithmetic`], [`order_numbers`],
//!    [`evaluate_boolean`], [`evaluate_decimal`], [`divide`], [`modulo`], [`evaluate_ieee`],
//!    [`compare_ieee`], [`convert_ieee_width`], [`ieee_to_exact`], [`exact_to_ieee`],
//!    [`admit_text`] and [`compare_text`], each after its static [`IllTyped`] refusal;
//! 2. the runtime's residue over them: [`EnumValue`] and [`Quantity`] over a [`UnitGraph`]
//!    ([`compare_enum`], [`evaluate_quantity`], [`compare_quantity`], [`convert_quantity`]),
//!    and [`evaluate_boolean_short_circuit`], the stop-carrying lazy-right connective the
//!    kernel does not export;
//! 3. quire-specification/FR-143 composite values: [`TypeEnvironment`], [`Value`], [`ValueType`]
//!    and the [`ValueGraph`] finite-value constructor, over terminal
//!    [`ObjectReference`] identities;
//! 4. quire-specification/FR-144 collections: [`CollectionType`], [`CollectionValue`] and
//!    [`construct_collection`]/[`form_collection`], keyed by the quire-specification/FR-144
//!    canonical key that [`CheckedEquality`] and collection membership share;
//! 5. the quire-specification/FR-149 equality matrix: [`TypeEnvironment::check_equality`] and
//!    [`CheckedEquality::evaluate`];
//! 6. the distinct evaluator [`Outcome`] (the kernel's) with typed [`Undefined`], [`Refusal`]
//!    and [`Incomplete`] reasons;
//! 7. `quire.value.accounting/v1` charge-before-work metering through the kernel's
//!    [`Meter`];
//! 8. the quire-specification/FR-146 function-application surface:
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

// The kernel items this module no longer defines (FR-275): re-exported unchanged at the path
// generated oracles import (interface-001-AC-1, AC-2, AC-4). Not an alias layer: these are
// `quire-exact`'s own definitions and the runtime owns no copy of any of them.
pub use quire_exact::{
    admit_text, compare_ieee, compare_text, convert_ieee_width, divide, evaluate_boolean,
    evaluate_decimal, evaluate_ieee, evaluate_integer_arithmetic, evaluate_rational_arithmetic,
    exact_to_ieee, ieee_intrinsic_identities, ieee_to_exact, modulo, order_numbers,
    BooleanConnective, BoundViolation, BoundedInteger, CardinalityBound, ChargePoint,
    CollectionKind, ComparisonOperator, Decimal, DecimalLoss, DecimalOperation,
    DecimalRepresentation, DecimalResult, DecimalType, DivisionProfile, EmptyCardinalityBound,
    EmptyInterval, EmptyTextBounds, ExactScalar, IeeeComparison, IeeeExact, IeeeExactLoss,
    IeeeExactTarget, IeeeFlag, IeeeFlags, IeeeOperand, IeeeOperation, IeeeOperationKind,
    IeeeProvenance, IeeeResult, IeeeValue, IeeeWidth, IllTyped, IllTypedCause, Incomplete,
    InexactTarget, InjectedDenial, Integer, IntegerArithmetic, IntegerDomain, IntegerInterval,
    InvalidUtf8, LimitKind, Meter, NonCanonicalInteger, NonPositiveDenominatorBound,
    NormalizationForm, OrderedOperands, OrderingOperator, OutOfDomain, Outcome, QuotientRemainder,
    Rational, RationalArithmetic, RationalDomain, Refusal, RoundingMode, ScalarLimits, Text,
    TextPayload, TextProfile, TextProvenance, TextType, Undefined, UniverseId, ZeroDenominator,
    IEEE_DEFINITION, UNICODE_TEXT_DEFINITION, UNICODE_VERSION,
};
