// SPDX-License-Identifier: AGPL-3.0-or-later
//! Complete-V1 exact scalar oracle operators and typed runtime outcomes.
//!
//! Implements: FR-006, FR-007.
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
//! 2. explicit operation tables: [`evaluate_integer`], [`evaluate_rational`],
//!    [`evaluate_ordering`], [`evaluate_connective`], [`evaluate_not`],
//!    [`evaluate_decimal`], [`divide`] and
//!    [`modulo`], [`evaluate_ieee`], [`compare_ieee`], [`convert_ieee_width`],
//!    [`ieee_to_exact`], [`exact_to_ieee`], [`admit_text`], [`compare_text`],
//!    [`compare_enum`], [`evaluate_quantity`], [`compare_quantity`] and
//!    [`convert_quantity`], each after its static [`IllTyped`] refusal;
//! 3. the distinct evaluator [`Outcome`] with typed [`Undefined`], [`Refusal`]
//!    and [`Incomplete`] reasons;
//! 4. `quire.value.accounting/v1` charge-before-work metering through
//!    [`Meter`].
//!
//! Node keys, definition-lock selection, owner joins, stale keys and package
//! admission are compiler work. The runtime consumes admitted keys and
//! profiles, and carries the closed refusal vocabularies
//! ([`SelectionRefusalCode`], [`PackageRefusal`], [`SemanticGraphCause`]) so a
//! compiler refusal is reported with its exact code. No host floating-point
//! operation, panic path or ambient effect exists on any semantic path.

mod accounting;
mod arithmetic;
mod comparison;
mod decimal;
mod definition;
mod division;
mod enumeration;
mod ieee;
mod integer;
mod node;
mod outcome;
mod quantity;
mod rational;
mod text;
mod unit;

pub use accounting::{ChargePoint, Incomplete, InjectedDenial, LimitKind, Meter, ScalarLimits};
pub use arithmetic::{
    evaluate_connective, evaluate_integer, evaluate_not, evaluate_ordering, evaluate_rational,
    BooleanConnective, IntegerOperation, OrderingOperands, OrderingOperator, RationalOperation,
};
pub use comparison::{ComparisonOperator, IllTyped, IllTypedCause};
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
pub use ieee::{
    compare_ieee, convert_ieee_width, evaluate_ieee, exact_to_ieee, ieee_intrinsic_identities,
    ieee_to_exact, negotiate_ieee, ExactScalar, IeeeBackendCapabilities, IeeeComparison,
    IeeeDisposition, IeeeExact, IeeeExactTarget, IeeeFlag, IeeeFlags, IeeeItemRequirement,
    IeeeOperand, IeeeOperation, IeeeOperationKind, IeeeProvenance, IeeeResult,
    IeeeUnsupportedCause, IeeeValue, IeeeWidth, IEEE_DEFINITION,
};
pub use integer::{
    BoundedInteger, EmptyInterval, Integer, IntegerDomain, IntegerInterval, NonCanonicalInteger,
    OutOfDomain,
};
pub use node::{InvalidSemanticGraph, NodeKey, SemanticGraphCause, NODE_KEY_DOMAIN};
pub use outcome::{Outcome, Refusal, Undefined};
pub use quantity::{
    compare_quantity, convert_quantity, evaluate_quantity, Conversion, ConvertedValue, Quantity,
    QuantityOperation, QuantityTarget, QuantityUnit,
};
pub use rational::{NonPositiveDenominatorBound, Rational, RationalDomain, ZeroDenominator};
pub use text::{
    admit_text, compare_text, EmptyTextBounds, InvalidTextLiteral, InvalidUtf8, NormalizationForm,
    Text, TextPayload, TextProfile, TextProvenance, TextType, UNICODE_TEXT_DEFINITION,
    UNICODE_VERSION,
};
pub use unit::{
    CompoundUnit, CompoundUnitCause, Dimension, InvalidCompoundUnit, Unit, UnitDeclaration,
    UnitEdge, UnitGraph, COMPOUND_UNIT_DOMAIN,
};
