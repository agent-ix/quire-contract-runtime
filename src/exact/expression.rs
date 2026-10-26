// SPDX-License-Identifier: AGPL-3.0-or-later
//! Function application under exact semantics (quire-specification/FR-146,
//! FR-273).
//!
//! AD-002 controls this runtime function boundary. Generated code supplies
//! resumable body states; this crate does not derive purity, termination or
//! definedness. Admission checks the declarations and upstream-discharge
//! marker before any package becomes callable or any meter is charged.
//!
//! [`plan_call`] and [`plan_evaluation`] validate inputs without a [`Meter`].
//! An admitted `function.call` consumes one work unit before its body starts.
//! The scheduler owns suspended body states in a heap vector and invokes one
//! transition at a time. A transition can request a child call or return;
//! it cannot synchronously invoke another checked function. The shared meter
//! uses a [`RefCell`] and a conflicting mutable borrow yields a typed refusal.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicUsize, Ordering};

use quire_exact::{
    Charge, ChargePoint, CheckedInvariantCause, DecimalLoss, FieldValue, IeeeExactLoss, IeeeFlags,
    IllTyped, IllTypedCause, Integer, Meter, Outcome, Refusal, Value, ValueType,
};
use quire_semantic_value::declaration::TypeEnvironment;
use quire_semantic_value::object_closure::ObjectClosure;
use quire_semantic_value::stop::{outcome_from_stop, Stop};

use crate::scalar::{IeeeItemRequirement, IntegerDivisionConsumer};

/// The authority's two checking modes, moved rather than added: the
/// authority threads a `CheckMode` through its own per-expression checking
/// entry point, so any one expression (including a standalone one) can be
/// checked under either mode independently. This port instead takes
/// `CheckMode` once, at [`PackageDeclarations::check`]'s package boundary —
/// every function in a package shares one mode — and
/// [`CheckedPackage::check_expression`] (which checks a standalone
/// [`CheckedExpression`] against an already-checked package) takes no
/// `CheckMode` parameter of its own at all, inheriting `Linked` implicitly
/// from the package it is checked against. Only [`CheckMode::Linked`] ever
/// admits a callable package (FR-273-AC-6; AD-002).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckMode {
    /// Every definedness obligation is discharged; the package is callable.
    Linked,
    /// Typing-only. Out of scope here: [`PackageDeclarations::check`] refuses
    /// it structurally, so no package checked under `Kernel` is ever
    /// callable.
    Kernel,
}

/// The source of [`CheckedPackage::id`]: a monotonic counter stamped once per
/// [`PackageDeclarations::check`] call. `thumbv7em-none-eabi` has
/// compare-and-swap, so `AtomicUsize` is available at this crate's MSRV
/// (1.98.1) on the governed target; `Ordering::Relaxed` is enough because the
/// property this counter needs is "does not race across concurrent `check`
/// calls," not any cross-thread happens-before relationship with other
/// state. `fetch_add` wraps silently on overflow rather than panicking, and
/// `usize` is 32 bits on that governed target, so this is not an unbounded
/// identity: the 2^32-nd `check` call on a process's lifetime wraps back to
/// `0` and would be indistinguishable from whichever still-live package was
/// stamped `0` first. That ceiling is far beyond any real program's package
/// count, and no test in this crate proves the wrapped case refuses, so the
/// invariant this crate actually holds is "does not repeat within a
/// realistic package count," not "never repeats."
static NEXT_PACKAGE_ID: AtomicUsize = AtomicUsize::new(0);

/// A checking node budget. Calls are bounded by work fuel, not depth.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CheckingLimits {
    nodes: u64,
}

impl CheckingLimits {
    /// Construct a checking node budget.
    pub fn new(nodes: u64) -> Self {
        Self { nodes }
    }

    /// The node-count ceiling, carried for the checker contract. A generated
    /// body has no AST this runtime can count, so it is not enforced here.
    pub fn nodes(self) -> u64 {
        self.nodes
    }
}

impl Default for CheckingLimits {
    fn default() -> Self {
        Self { nodes: u64::MAX }
    }
}

/// A generated factory for one owned, resumable invocation state.
pub type Body = Box<dyn Fn(Vec<Value>) -> Box<dyn BodyState>>;

/// One transition from a generated function body.
pub enum BodyStep {
    /// Suspend the current body and call a checked function.
    Call {
        /// Declared checked function name.
        function: String,
        /// Completed arguments in parameter order.
        arguments: Vec<Value>,
    },
    /// Finish this body with an outcome.
    Return(Outcome<Value>),
}

/// One owned function invocation, resumed after a child call completes.
pub trait BodyState {
    /// Run one transition; `completed` contains the returned child value.
    fn step(&mut self, frame: &Frame<'_, '_>, completed: Option<Value>) -> BodyStep;
}

impl<F> BodyState for F
where
    F: for<'a, 'm> FnMut(&Frame<'a, 'm>, Option<Value>) -> BodyStep,
{
    fn step(&mut self, frame: &Frame<'_, '_>, completed: Option<Value>) -> BodyStep {
        self(frame, completed)
    }
}

/// One function of a [`PackageDeclarations`], declared before checking.
pub struct FunctionDeclaration {
    /// The function's name, unique within its package.
    pub name: String,
    /// Parameter names and declared types, in call order.
    pub parameters: Vec<(String, ValueType)>,
    /// The declared result type.
    pub result: ValueType,
    /// The IEEE item requirements this function's body discharges (FR-009).
    pub ieee_requirements: Vec<IeeeItemRequirement>,
    /// The integer-division consumers this function's body discharges
    /// (FR-009).
    pub integer_division_consumers: Vec<IntegerDivisionConsumer>,
    /// Whether this function's `decreases` termination measure was
    /// discharged upstream. `check` refuses a function for which this is
    /// `false`: AD-002 keeps this crate from re-deriving that proof itself.
    pub measure_discharged: bool,
    /// The factory for this function's resumable body state.
    pub body: Body,
}

/// An unchecked package: its [`TypeEnvironment`] and its declared functions.
#[derive(Default)]
pub struct PackageDeclarations {
    /// The package's type environment.
    pub types: TypeEnvironment,
    /// The package's declared functions.
    pub functions: Vec<FunctionDeclaration>,
}

/// Where a [`CheckRefusal`] or a non-completed [`Evaluation`] originates.
/// Ported from the authority verbatim: variant names, fields and order.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Origin {
    /// The `index`-th declared function's own body.
    Body {
        /// The function's name.
        function: String,
        /// The function's index in its package.
        index: usize,
    },
    /// The `index`-th declared function's termination measure.
    Measure {
        /// The function's name.
        function: String,
        /// The function's index in its package.
        index: usize,
    },
    /// A standalone [`CheckedExpression`], not any package function.
    Expression,
}

/// An origin and the child path reached from it. Ported from the authority
/// verbatim: field names and order.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Location {
    /// The origin.
    pub origin: Origin,
    /// The child-index path from the origin.
    pub path: Vec<usize>,
}

fn location_at(origin: Origin) -> Location {
    Location {
        origin,
        path: Vec::new(),
    }
}

/// Why [`PackageDeclarations::check`] or [`CheckedPackage::check_expression`]
/// refuses, before any charge.
///
/// This crate does not derive purity, termination or definedness itself
/// (AD-002): that proof stays entirely in the authority's own
/// `Typer`/definedness/termination machinery, deliberately not ported here.
/// `check` here verifies only what a generated package can carry as its own
/// admitted fact: declared types are members of the package's
/// [`TypeEnvironment`], no two functions share a name, every function's
/// termination measure was discharged upstream, and the package is checked
/// under [`CheckMode::Linked`].
///
/// Unlike [`InputRefusal`] (ported verbatim: variant names, fields and
/// order), `CheckCause` is **narrowed, not ported**: the authority's own
/// check-refusal cause carries twelve variants plus `code()`/`cause()`
/// accessors, covering every definedness, purity and termination obligation
/// its `Typer`/checker proves. AD-002 keeps that whole proof out of this
/// crate, so only what a generated package can itself carry as an admitted
/// fact survives here — four variants, two of which this crate invented
/// rather than carried over: `UndischargedMeasure` trusts an
/// upstream-discharged flag instead of proving termination itself, and
/// `UnsupportedCheckMode` gates the [`CheckMode`] this port *moved*, not
/// added: the authority threads a `CheckMode` through its own per-expression
/// checking entry point, but this port's [`CheckedPackage::check_expression`]
/// takes no such parameter at all — it inherits `Linked` implicitly from the
/// already-checked [`CheckedPackage`] a standalone expression is checked
/// against. [`PackageDeclarations::check`] is where this port instead takes
/// an explicit `CheckMode`, once, at the package boundary (see
/// [`CheckMode`]'s own documentation for the full seam). Treat this type as
/// this port's own closed vocabulary, not as evidence of a verbatim port.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckCause {
    /// A declared parameter or result type is not admitted by the package's
    /// [`TypeEnvironment`].
    IllTyped(IllTypedCause),
    /// Two functions share one name: `call` could not tell which is meant.
    AmbiguousName {
        /// The ambiguous name.
        name: String,
        /// Every declaring locus.
        loci: Vec<Location>,
    },
    /// The function's termination measure was not discharged upstream: no
    /// termination proof backs its body, so it can never become callable.
    UndischargedMeasure,
    /// [`CheckMode::Kernel`]: out of scope (AD-002; FR-273-AC-6). Only a
    /// package checked under [`CheckMode::Linked`] is ever callable.
    UnsupportedCheckMode,
}

/// A single refusal `check` produced, located at its origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckRefusal {
    /// Where the refusal originates.
    pub location: Location,
    /// The typed cause.
    pub cause: CheckCause,
}

impl PackageDeclarations {
    /// Admit this package under `mode`, or refuse every violation found.
    /// Admission never charges: this happens once, before any package is
    /// callable (FR-273-AC-1).
    pub fn check(
        self,
        mode: CheckMode,
        _limits: CheckingLimits,
    ) -> Result<CheckedPackage, Vec<CheckRefusal>> {
        if matches!(mode, CheckMode::Kernel) {
            return Err(alloc::vec![CheckRefusal {
                location: location_at(Origin::Expression),
                cause: CheckCause::UnsupportedCheckMode,
            }]);
        }

        let mut refusals = Vec::new();
        for (index, function) in self.functions.iter().enumerate() {
            let loci: Vec<Location> = self
                .functions
                .iter()
                .enumerate()
                .filter(|(_, other)| other.name == function.name)
                .map(|(other_index, other)| {
                    location_at(Origin::Body {
                        function: other.name.clone(),
                        index: other_index,
                    })
                })
                .collect();
            if loci.len() > 1 {
                refusals.push(CheckRefusal {
                    location: location_at(Origin::Body {
                        function: function.name.clone(),
                        index,
                    }),
                    cause: CheckCause::AmbiguousName {
                        name: function.name.clone(),
                        loci,
                    },
                });
            }
        }
        if !refusals.is_empty() {
            return Err(refusals);
        }

        for (index, function) in self.functions.iter().enumerate() {
            let location = || {
                location_at(Origin::Body {
                    function: function.name.clone(),
                    index,
                })
            };
            for (_, value_type) in &function.parameters {
                if let Err(IllTyped { cause }) = self.types.check_type(value_type) {
                    refusals.push(CheckRefusal {
                        location: location(),
                        cause: CheckCause::IllTyped(cause),
                    });
                }
            }
            if let Err(IllTyped { cause }) = self.types.check_type(&function.result) {
                refusals.push(CheckRefusal {
                    location: location(),
                    cause: CheckCause::IllTyped(cause),
                });
            }
            if !function.measure_discharged {
                refusals.push(CheckRefusal {
                    location: location(),
                    cause: CheckCause::UndischargedMeasure,
                });
            }
        }
        if !refusals.is_empty() {
            return Err(refusals);
        }

        Ok(CheckedPackage(Box::new(CheckedPackageFields {
            types: self.types,
            functions: self.functions,
            active: Cell::new(false),
            id: NEXT_PACKAGE_ID.fetch_add(1, Ordering::Relaxed),
        })))
    }
}

/// A package admitted by [`PackageDeclarations::check`]: every declared
/// function is callable by name.
pub struct CheckedPackage(Box<CheckedPackageFields>);

/// [`CheckedPackage`]'s fields, behind one `Box`: `check` returns
/// `Result<CheckedPackage, Vec<CheckRefusal>>`, and a boxed `Ok` puts that
/// `Result`'s niche in a non-null pointer CBMC can fold, rather than in
/// padding it fills with nondeterministic bytes.
struct CheckedPackageFields {
    types: TypeEnvironment,
    functions: Vec<FunctionDeclaration>,
    active: Cell<bool>,
    /// This package's identity, stamped once from [`NEXT_PACKAGE_ID`] at
    /// [`PackageDeclarations::check`] time and carried by value from then on.
    /// Not derived from the package's address: `check` returns `CheckedPackage`
    /// by value, so any move (into an `Rc`, a `Box`, or simply up the stack)
    /// after `check` would change an address-derived identity out from under
    /// every [`CheckedExpression`] already checked against it — refusing the
    /// package's own, unmoved expression as `ForeignExpression`, silently
    /// indistinguishable from a genuine invariant violation. A monotonic
    /// counter is immune to moves and, unlike an address, is never reused
    /// once a package is dropped and its allocation recycled.
    id: usize,
}

/// A standalone expression checked against an already-checked package's
/// [`TypeEnvironment`], with its own parameters and resumable body.
pub struct CheckedExpression {
    parameters: Vec<(String, ValueType)>,
    root: Body,
    /// The identity of the [`CheckedPackage`] this expression was checked
    /// against ([`CheckedPackage::identity`], a monotonic id stamped at
    /// [`PackageDeclarations::check`] time — see [`CheckedPackage`]'s own
    /// documentation) at [`CheckedPackage::check_expression`] time: it
    /// discriminates "checked against this package" from "checked against a
    /// different one" for as long as this owned, detachable value outlives
    /// the package's own lifetime, and survives the package being moved.
    /// [`plan_evaluation`] compares it against the package `evaluate` is
    /// actually called on and refuses a mismatch at the plan boundary, before
    /// any charge.
    package: usize,
}

/// Why a runtime input is refused, before any charge. Ported verbatim from
/// the authority (variant names, fields and order), minus its `thiserror`
/// derive: this crate is `no_std` and has no `std::error::Error`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputRefusal {
    /// No function of this name is declared.
    UnknownFunction(String),
    /// The supplied argument count does not match the declared parameter
    /// count.
    Arity {
        /// The declared parameter count.
        declared: usize,
        /// The supplied argument count.
        supplied: usize,
    },
    /// The argument at `parameter` is not admitted by its declared type.
    WrongValueKind {
        /// The zero-based parameter position.
        parameter: usize,
    },
    /// The argument at `parameter` contains a reference absent from the
    /// supplied [`ObjectClosure`].
    DanglingReference {
        /// The zero-based parameter position.
        parameter: usize,
    },
}

impl InputRefusal {
    /// The authority returns a `crate::diagnostic::Code` this crate does not
    /// have; this is that port, as the stable string the authority's `Code`
    /// itself renders.
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnknownFunction(_) => "missing_declaration",
            Self::Arity { .. } | Self::WrongValueKind { .. } => "invalid_runtime_input",
            Self::DanglingReference { .. } => "dangling_reference",
        }
    }

    /// The stable machine-readable cause tag.
    pub fn cause(&self) -> &'static str {
        match self {
            Self::UnknownFunction(_) => "missing-name",
            Self::Arity { .. } | Self::WrongValueKind { .. } => "wrong-value-kind",
            Self::DanglingReference { .. } => "absent-target-in-complete-population",
        }
    }
}

/// What a completed application discarded. Ported verbatim from the
/// authority: variant names and order.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValueLoss {
    /// A rounded decimal operation or conversion (FR-140).
    Decimal(DecimalLoss),
    /// An IEEE-to-exact conversion (FR-148).
    IeeeExact(IeeeExactLoss),
    /// The non-empty flag set an IEEE operation raised (FR-148).
    IeeeFlags(IeeeFlags),
}

/// A loss record and the expression that produced it. Ported verbatim from
/// the authority: field names and order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocatedLoss {
    /// The producing location.
    pub location: Location,
    /// What was discarded.
    pub loss: ValueLoss,
}

/// A completed, undefined, refused or incomplete application. Ported
/// verbatim from the authority: field names and order.
///
/// This crate has no AST to walk, so no producer here ever populates
/// `location` or `losses`: a host body reports neither through [`Frame`]'s
/// public surface. Both fields keep the authority's shape so a future
/// codegen-emitted location path (tracked separately from this issue) is an
/// additive change, not a breaking one; today `location` is always `None`
/// and `losses` is always empty.
#[derive(Clone, Debug)]
pub struct Evaluation {
    /// The outcome.
    pub outcome: Outcome<Value>,
    /// Where a non-completed outcome originated; `None` when completed, and
    /// today always `None` (see the struct's own documentation).
    pub location: Option<Location>,
    /// The loss records of the operations a completed evaluation performed,
    /// in evaluation order; today always empty (see the struct's own
    /// documentation).
    pub losses: Vec<LocatedLoss>,
}

/// The events a planned call or evaluation contributes at its own root, with
/// no charge and no body invoked. Mirrors [`quire_exact::EqualityPlan`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallPlan {
    call_events: Integer,
}

impl CallPlan {
    /// The number of `function.call` charges this plan's own root
    /// contributes: exactly one for [`plan_call`] (the call itself), and
    /// exactly zero for [`plan_evaluation`] (an expression's root is not
    /// itself an application; FR-273's "resolved" ordering note).
    pub fn call_events(&self) -> &Integer {
        &self.call_events
    }
}

fn validate_arguments(
    parameters: &[(String, ValueType)],
    arguments: &[Value],
    objects: &ObjectClosure,
) -> Result<(), InputRefusal> {
    if parameters.len() != arguments.len() {
        return Err(InputRefusal::Arity {
            declared: parameters.len(),
            supplied: arguments.len(),
        });
    }
    for (parameter, ((_, value_type), argument)) in parameters.iter().zip(arguments).enumerate() {
        if !value_type.admits(argument) {
            return Err(InputRefusal::WrongValueKind { parameter });
        }
        let mut pending: Vec<&Value> = alloc::vec![argument];
        while let Some(value) = pending.pop() {
            match value {
                Value::Reference(reference) => {
                    if !objects.contains(reference) {
                        return Err(InputRefusal::DanglingReference { parameter });
                    }
                }
                Value::Option(option) => pending.extend(option.payload()),
                Value::Composite(composite) => {
                    pending.extend(composite.slots().iter().filter_map(|slot| match slot {
                        FieldValue::Present(value) => Some(value),
                        FieldValue::Absent | FieldValue::Null => None,
                    }));
                }
                Value::Union(union) => pending.extend(union.payload()),
                Value::Collection(collection) => pending.extend(collection.elements()),
                Value::Boolean(_)
                | Value::Integer(_)
                | Value::Rational(_)
                | Value::Decimal(_)
                | Value::Float(_)
                | Value::Quantity(_)
                | Value::Text(_)
                | Value::String(_)
                | Value::Uuid(_)
                | Value::Timestamp(_)
                | Value::Enum(_) => {}
                Value::Population(_) => {}
            }
        }
    }
    Ok(())
}

/// Validate a would-be [`CheckedPackage::call`] with no [`Meter`] in scope:
/// every [`InputRefusal`] is reachable here, before any charge (FR-273-AC-2,
/// FR-273-AC-3).
pub fn plan_call(
    package: &CheckedPackage,
    function: &str,
    arguments: &[Value],
    objects: &ObjectClosure,
) -> Result<CallPlan, InputRefusal> {
    let declaration = package
        .function(function)
        .ok_or_else(|| InputRefusal::UnknownFunction(function.to_string()))?;
    validate_arguments(&declaration.parameters, arguments, objects)?;
    Ok(CallPlan {
        call_events: Integer::one(),
    })
}

/// Why [`plan_evaluation`] refuses a would-be [`CheckedPackage::evaluate`],
/// before any charge: either the same argument validation [`plan_call`]
/// shares, or a [`CheckedExpression`] checked against a different
/// [`CheckedPackage`] than the one `evaluate` is actually called on.
///
/// The authority's checker ties an expression to the package it type-checked
/// it against structurally (through the AST it holds in place); this port's
/// [`CheckedExpression`] is an owned, detachable value with no such tie, so
/// the foreign-expression case is this port's own addition — not part of
/// [`InputRefusal`]'s verbatim-ported vocabulary. [`CheckedPackage::evaluate`]
/// itself still returns `Result<Evaluation, InputRefusal>` exactly as FR-273
/// specifies: a foreign expression surfaces there as `Ok(Evaluation {
/// outcome: Outcome::Refused(Refusal::CheckedInvariant {
///     cause: CheckedInvariantCause::ForeignCheckedExpression,
/// }), .. })`, decided here
/// at the plan boundary, before any charge — never by drifting into a
/// `Refused(CheckedInvariant { cause: ForeignCheckedExpression })` deep inside a body that silently ran against
/// the wrong package's function table.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvaluationRefusal {
    /// Shared with [`plan_call`]: the same [`InputRefusal`] argument
    /// validation.
    Input(InputRefusal),
    /// The [`CheckedExpression`] was checked against a different
    /// [`CheckedPackage`] than the one `evaluate`/`plan_evaluation` was
    /// called on.
    ForeignExpression,
}

impl From<InputRefusal> for EvaluationRefusal {
    fn from(refusal: InputRefusal) -> Self {
        Self::Input(refusal)
    }
}

/// Validate a would-be [`CheckedPackage::evaluate`] with no [`Meter`] in
/// scope: the same argument validation [`plan_call`] shares, plus the
/// [`CheckedExpression`]-to-`package` identity check `plan_call` does not
/// need (a named `call` already proves that binding by looking `function` up
/// in `package`'s own table; a standalone `expression` carries no name to
/// look up).
pub fn plan_evaluation(
    package: &CheckedPackage,
    expression: &CheckedExpression,
    arguments: &[Value],
    objects: &ObjectClosure,
) -> Result<CallPlan, EvaluationRefusal> {
    if expression.package != package.identity() {
        return Err(EvaluationRefusal::ForeignExpression);
    }
    validate_arguments(&expression.parameters, arguments, objects)?;
    Ok(CallPlan {
        call_events: Integer::zero(),
    })
}

fn charge_call(meter: &mut Meter) -> Result<(), Stop> {
    meter.charge(Charge::new(ChargePoint::FunctionCall))?;
    Ok(())
}

fn checked_invariant(cause: CheckedInvariantCause) -> Outcome<Value> {
    Outcome::Refused(Refusal::CheckedInvariant { cause })
}

struct ExecutionGuard<'a>(&'a Cell<bool>);

impl Drop for ExecutionGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

impl CheckedPackage {
    fn begin(&self) -> Option<ExecutionGuard<'_>> {
        if self.0.active.replace(true) {
            None
        } else {
            Some(ExecutionGuard(&self.0.active))
        }
    }

    fn function(&self, name: &str) -> Option<&FunctionDeclaration> {
        self.0
            .functions
            .iter()
            .find(|function| function.name == name)
    }

    /// This package's own identity: a monotonic id stamped once at
    /// [`PackageDeclarations::check`] time, used to bind a
    /// [`CheckedExpression`] to the package that checked it. See
    /// [`CheckedPackage::id`]'s and [`CheckedExpression`]'s own documentation
    /// for why this is not derived from the package's address.
    fn identity(&self) -> usize {
        self.0.id
    }

    /// Check a standalone expression's parameters and result type against
    /// this package's [`TypeEnvironment`].
    pub fn check_expression(
        &self,
        parameters: Vec<(String, ValueType)>,
        result: ValueType,
        root: Body,
    ) -> Result<CheckedExpression, CheckRefusal> {
        let refuse = |cause| CheckRefusal {
            location: location_at(Origin::Expression),
            cause,
        };
        for (_, value_type) in &parameters {
            if let Err(IllTyped { cause }) = self.0.types.check_type(value_type) {
                return Err(refuse(CheckCause::IllTyped(cause)));
            }
        }
        if let Err(IllTyped { cause }) = self.0.types.check_type(&result) {
            return Err(refuse(CheckCause::IllTyped(cause)));
        }
        Ok(CheckedExpression {
            parameters,
            root,
            package: self.identity(),
        })
    }

    /// Validate a named call, charge `function.call`, then drive its body.
    pub fn call(
        &self,
        function: &str,
        arguments: Vec<Value>,
        objects: &ObjectClosure,
        meter: &mut Meter,
    ) -> Result<Evaluation, InputRefusal> {
        plan_call(self, function, &arguments, objects)?;
        let outcome = match self.begin() {
            Some(_execution) => match charge_call(meter) {
                Ok(()) => match self.function(function) {
                    Some(declaration) => self.run((declaration.body)(arguments), objects, meter),
                    None => checked_invariant(CheckedInvariantCause::UnknownCheckedFunction),
                },
                Err(stop) => outcome_from_stop(Err(stop)),
            },
            None => checked_invariant(CheckedInvariantCause::PackageReentry),
        };
        Ok(Evaluation {
            outcome,
            location: None,
            losses: Vec::new(),
        })
    }

    /// Evaluate a standalone checked expression. Its root is not charged;
    /// child calls requested by its body are charged in order.
    pub fn evaluate(
        &self,
        expression: &CheckedExpression,
        arguments: Vec<Value>,
        objects: &ObjectClosure,
        meter: &mut Meter,
    ) -> Result<Evaluation, InputRefusal> {
        match plan_evaluation(self, expression, &arguments, objects) {
            Ok(_) => {}
            Err(EvaluationRefusal::Input(refusal)) => return Err(refusal),
            Err(EvaluationRefusal::ForeignExpression) => {
                return Ok(Evaluation {
                    outcome: Outcome::Refused(Refusal::CheckedInvariant {
                        cause: CheckedInvariantCause::ForeignCheckedExpression,
                    }),
                    location: None,
                    losses: Vec::new(),
                });
            }
        }
        let outcome = match self.begin() {
            Some(_execution) => self.run((expression.root)(arguments), objects, meter),
            None => checked_invariant(CheckedInvariantCause::PackageReentry),
        };
        Ok(Evaluation {
            outcome,
            location: None,
            losses: Vec::new(),
        })
    }

    /// Drive one generated transition at a time with suspended parents owned
    /// by an explicit heap stack. No transition invokes another transition.
    fn run(
        &self,
        root: Box<dyn BodyState>,
        objects: &ObjectClosure,
        meter: &mut Meter,
    ) -> Outcome<Value> {
        let shared_meter = RefCell::new(meter);
        let mut states = alloc::vec![root];
        let mut child_result = None;
        loop {
            let Some(current) = states.last_mut() else {
                return checked_invariant(CheckedInvariantCause::GeneratedUnexpectedOutcome);
            };
            let context = Frame {
                objects,
                meter: &shared_meter,
            };
            let next = current.step(&context, child_result.take());
            match next {
                BodyStep::Call {
                    function,
                    arguments,
                } => {
                    let Some(callee) = self.function(&function) else {
                        return checked_invariant(CheckedInvariantCause::UnknownCheckedFunction);
                    };
                    if validate_arguments(&callee.parameters, &arguments, objects).is_err() {
                        return checked_invariant(
                            CheckedInvariantCause::GeneratedArgumentShapeMismatch,
                        );
                    }
                    let Ok(mut active_meter) = shared_meter.try_borrow_mut() else {
                        return checked_invariant(CheckedInvariantCause::MeterBorrowConflict);
                    };
                    if let Err(stop) = charge_call(&mut active_meter) {
                        return outcome_from_stop(Err(stop));
                    }
                    drop(active_meter);
                    states.push((callee.body)(arguments));
                }
                BodyStep::Return(Outcome::Completed(value)) => {
                    states.pop();
                    if states.is_empty() {
                        return Outcome::Completed(value);
                    }
                    child_result = Some(value);
                }
                BodyStep::Return(Outcome::Undefined(reason)) => return Outcome::Undefined(reason),
                BodyStep::Return(Outcome::Refused(reason)) => return Outcome::Refused(reason),
                BodyStep::Return(Outcome::Incomplete(record)) => {
                    return Outcome::Incomplete(record)
                }
            }
        }
    }

    /// The IEEE item requirements the named function's body discharges
    /// (FR-009), or `None` if no such function is declared.
    pub fn ieee_requirements(&self, function: &str) -> Option<&[IeeeItemRequirement]> {
        self.function(function)
            .map(|declaration| declaration.ieee_requirements.as_slice())
    }

    /// The integer-division consumers the named function's body discharges
    /// (FR-009), or `None` if no such function is declared.
    pub fn integer_division_consumers(&self, function: &str) -> Option<&[IntegerDivisionConsumer]> {
        self.function(function)
            .map(|declaration| declaration.integer_division_consumers.as_slice())
    }
}

/// Shared context supplied to one generated body transition.
pub struct Frame<'a, 'm> {
    objects: &'a ObjectClosure,
    meter: &'a RefCell<&'m mut Meter>,
}

impl Frame<'_, '_> {
    /// Run `run` against the shared [`Meter`] this frame's whole call tree
    /// charges through. `Err(Refusal::CheckedInvariant { cause: MeterBorrowConflict })` if this frame's
    /// `Meter` is already mutably borrowed by this transition, rather than
    /// panicking on the double borrow. Returning `Result` keeps the refusal
    /// visible to the generated body.
    /// Propagate with `?`, then fold a returned `Err(r)` into
    /// `Outcome::Refused(r)` to keep returning the body's own declared
    /// `Outcome<Value>`.
    pub fn meter<R>(&self, run: impl FnOnce(&mut Meter) -> R) -> Result<R, Refusal> {
        let mut guard = self
            .meter
            .try_borrow_mut()
            .map_err(|_| Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::MeterBorrowConflict,
            })?;
        Ok(run(&mut guard))
    }

    /// The [`ObjectClosure`] this call was made against.
    pub fn objects(&self) -> &ObjectClosure {
        self.objects
    }
}
