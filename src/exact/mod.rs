// SPDX-License-Identifier: AGPL-3.0-or-later
//! Complete-V1 exact scalar oracle operators and typed runtime outcomes.
//!
//! Implements: FR-006, FR-007, FR-008.
//!
//! `quire-spec-language` `quire_spec_language::value` is the semantic
//! authority. This module is a conformance-gated `no_std + alloc` port of that
//! authority for generated oracles: every type, field and variant keeps the
//! authority's name and order, so a shared-corpus test compares the two by
//! their `Debug` renderings, and no operator decides anything the authority
//! does not.
//!
//! Layers:
//!
//! 1. typed values: [`Integer`], [`IntegerInterval`]/[`BoundedInteger`],
//!    [`Rational`], [`Decimal`], [`IeeeValue`], [`Text`], [`EnumValue`] and
//!    [`Quantity`] over a [`UnitGraph`];
//! 2. explicit operation tables: [`evaluate_integer_arithmetic`],
//!    [`evaluate_rational_arithmetic`], [`order_numbers`], [`evaluate_boolean`],
//!    [`evaluate_boolean_short_circuit`],
//!    [`evaluate_decimal`], [`divide`] and
//!    [`modulo`], [`evaluate_ieee`], [`compare_ieee`], [`convert_ieee_width`],
//!    [`ieee_to_exact`], [`exact_to_ieee`], [`admit_text`], [`compare_text`],
//!    [`compare_enum`], [`evaluate_quantity`], [`compare_quantity`] and
//!    [`convert_quantity`], each after its static [`IllTyped`] refusal;
//! 3. quire-specification/FR-143 composite values: [`TypeEnvironment`], [`Value`], [`ValueType`]
//!    and the [`ValueGraph`] finite-value constructor, over terminal
//!    [`ObjectReference`] identities;
//! 4. quire-specification/FR-144 collections: [`CollectionType`], [`CollectionValue`] and
//!    [`construct_collection`]/[`form_collection`], keyed by the quire-specification/FR-144
//!    canonical key that [`CheckedEquality`] and collection membership share;
//! 5. the quire-specification/FR-149 equality matrix: [`TypeEnvironment::check_equality`] and
//!    [`CheckedEquality::evaluate`];
//! 6. the distinct evaluator [`Outcome`] with typed [`Undefined`], [`Refusal`]
//!    and [`Incomplete`] reasons;
//! 7. `quire.value.accounting/v1` charge-before-work metering through
//!    [`Meter`].
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
//! governed `thumbv7em-none-eabi` target, so `make audit-panic` cannot see
//! it — which is why `Debug` and `Drop` are hand-written rather than derived,
//! the same as every other value-walking algorithm here.
//! [`Value`] shares nested composite and collection values through
//! [`alloc::rc::Rc`], never `Arc`: this crate has no concurrency and no
//! `target_has_atomic` requirement. A closed
//! `ObjectEnvironment` that resolves an [`ObjectReference`] against a bound
//! model snapshot is out of scope: it is business logic for a consumer
//! holding that snapshot, not part of this exact value/collection/equality
//! core.

mod accounting;
mod collection;
mod comparison;
mod composite;
mod containment;
mod decimal;
mod definition;
mod division;
mod enumeration;
mod equality;
mod ieee;
mod integer;
mod key;
mod node;
mod numeric;
mod outcome;
mod quantity;
mod rational;
mod reference;
mod text;
mod unit;

pub use accounting::{
    ChargePoint, Incomplete, InjectedDenial, LimitKind, Meter, ScalarLimits, CHARGE_LOG_CAPACITY,
};
pub use collection::{
    construct_collection, form_collection, CardinalityBound, CollectionKind, CollectionType,
    CollectionValue, EmptyCardinalityBound,
};
pub use comparison::{ComparisonOperator, IllTyped, IllTypedCause};
pub use composite::{
    Component, CompositeDeclaration, CompositeShape, CompositeValue, ConstructionCause,
    ConstructionRefusal, DeclarationCause, Deferred, FieldDeclaration, FieldExpression, FieldValue,
    InvalidDeclaration, ObjectTypeDeclaration, OptionValue, Presence, RecursionEdges,
    TypeEnvironment, Value, ValueType,
};
pub use containment::{GraphCause, GraphNode, GraphNodeId, GraphRefusal, GraphSlot, ValueGraph};
pub use decimal::{
    evaluate_decimal, Decimal, DecimalLoss, DecimalOperation, DecimalRepresentation, DecimalResult,
    DecimalType, RoundingMode,
};
pub use definition::{PackageCause, PackageRefusal, PackageRefusalCode, SelectionRefusalCode};
pub use division::{
    divide, modulo, negotiate_integer_division, DivisionProfile, IntegerDivisionBounds,
    IntegerDivisionConsumer, IntegerDivisionDisposition, QuotientRemainder,
};
pub use enumeration::{compare_enum, EnumDeclaration, EnumValue};
pub use equality::{
    admits_equality_conversion, plan_equality, CheckedEquality, EqualityOperand, EqualityOperator,
    EqualityPlan, EqualitySchedule,
};
pub use ieee::{
    compare_ieee, convert_ieee_width, evaluate_ieee, exact_to_ieee, ieee_intrinsic_identities,
    ieee_to_exact, negotiate_ieee, ExactScalar, IeeeBackendCapabilities, IeeeComparison,
    IeeeDisposition, IeeeExact, IeeeExactLoss, IeeeExactTarget, IeeeFlag, IeeeFlags,
    IeeeItemRequirement, IeeeOperand, IeeeOperation, IeeeOperationKind, IeeeProvenance, IeeeResult,
    IeeeUnsupportedCause, IeeeValue, IeeeWidth, IEEE_DEFINITION,
};
pub use integer::{
    BoundedInteger, EmptyInterval, Integer, IntegerDomain, IntegerInterval, NonCanonicalInteger,
    OutOfDomain,
};
pub use node::{InvalidSemanticGraph, NodeKey, SemanticGraphCause, NODE_KEY_DOMAIN};
pub use numeric::{
    evaluate_boolean, evaluate_boolean_short_circuit, evaluate_integer_arithmetic,
    evaluate_rational_arithmetic, order_numbers, BooleanConnective, IntegerArithmetic,
    OrderedOperands, OrderingOperator, RationalArithmetic, ShortCircuitConnective,
};
pub use outcome::{BoundViolation, Outcome, Refusal, Undefined};
pub use quantity::{
    compare_quantity, convert_quantity, evaluate_quantity, Conversion, ConvertedValue, Quantity,
    QuantityOperation, QuantityTarget, QuantityUnit,
};
pub use rational::{NonPositiveDenominatorBound, Rational, RationalDomain, ZeroDenominator};
pub use reference::{InvalidObjectIdentity, ObjectIdentity, ObjectReference, UniverseIdentity};
pub use text::{
    admit_text, compare_text, EmptyTextBounds, InvalidTextLiteral, InvalidUtf8, NormalizationForm,
    Text, TextPayload, TextProfile, TextProvenance, TextType, UNICODE_TEXT_DEFINITION,
    UNICODE_VERSION,
};
pub use unit::{
    CompoundUnit, CompoundUnitCause, Dimension, InvalidCompoundUnit, Unit, UnitDeclaration,
    UnitEdge, UnitGraph, COMPOUND_UNIT_DOMAIN,
};
