// SPDX-License-Identifier: AGPL-3.0-or-later
//! Function application under exact semantics (quire-specification/FR-146,
//! FR-273).
//!
//! AD-002 controls the boundary this module ports: the runtime carries the
//! *call surface* of `quire_spec_language::value::expression` only. A
//! function's body is a host callable ([`Body`]) supplied by generated code;
//! this crate never derives purity, termination or definedness itself, and
//! is not a second semantic authority for them. [`PackageDeclarations::check`]
//! therefore checks only what a generated package can carry as its own
//! admitted facts (declared types, name uniqueness, an upstream-discharged
//! termination measure) before any package becomes callable and before any
//! charge, mirroring the authority's own admit-before-call ordering.
//!
//! The plan/execute split mirrors [`super::equality`]: [`plan_call`] and
//! [`plan_evaluation`] validate a call's arguments and take no [`Meter`] at
//! all, so every [`InputRefusal`] is reachable with no charge in scope,
//! before [`CheckedPackage::call`]/[`CheckedPackage::evaluate`] charge
//! `function.call` and run the body. Unbounded host recursion is silent stack
//! corruption on the governed `thumbv7em-none-eabi` target, so every
//! re-entrant path into a checked program is bounded by
//! [`CheckingLimits::depth`] (`MAX_CALL_DEPTH`) — not only [`Frame::call`],
//! but also a direct, bypassing re-entry into [`CheckedPackage::call`] or
//! [`CheckedPackage::evaluate`] from within a running [`Body`] that holds its
//! own `Rc<CheckedPackage>` (obtainable in safe Rust: a `Frame`'s depth
//! threaded purely by value cannot see that path at all, since a fresh root
//! `Frame` built at `depth: 0` looks identical to a program's first call).
//! The bound is therefore a [`core::cell::Cell`] counter carried on
//! [`CheckedPackage`] itself and shared by every entry path, incremented on
//! the way in and decremented on the way out of `call`, `evaluate` and
//! `Frame::call` alike, rather than threaded per-`Frame`. The shared
//! [`Meter`] is reached through a [`core::cell::RefCell`] (never `unsafe`,
//! never atomics, consistent with this crate's single-threaded, `Rc`-not-
//! `Arc` stance); every access goes through `try_borrow_mut` rather than
//! `borrow_mut`, so a body that re-enters its own frame's [`Meter`] (through
//! [`Frame::meter`] or [`Frame::call`]) is refused a typed, non-panicking
//! result instead of aborting the process.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicUsize, Ordering};

use super::accounting::{Charge, ChargePoint, Meter};
use super::comparison::{IllTyped, IllTypedCause};
use super::composite::{FieldValue, TypeEnvironment, Value, ValueType};
use super::decimal::DecimalLoss;
use super::division::IntegerDivisionConsumer;
use super::ieee::{IeeeExactLoss, IeeeFlags, IeeeItemRequirement};
use super::integer::Integer;
use super::outcome::{Outcome, Refusal, Stop};
use super::reference::ObjectEnvironment;

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

/// Reuses the authority's `MAX_CHECKING_DEPTH` value, but not its invariant:
/// upstream, that constant bounds expression *nesting* during *checking*, and
/// the authority runs application itself on an explicit task stack, so
/// neither checking nesting nor call depth ever consumes a host call-stack
/// frame there. This crate has no such indirection — [`Frame::call`],
/// [`CheckedPackage::call`] and [`CheckedPackage::evaluate`] recurse on the
/// real host stack — so here `MAX_CALL_DEPTH` is the greatest re-entrant call
/// depth [`CheckingLimits::new`] admits before every one of those entry
/// points refuses rather than risk the silent stack corruption an unbounded
/// `thumbv7em-none-eabi` recursion would cause. Same number, different
/// invariant.
pub const MAX_CALL_DEPTH: u64 = 128;

/// The source of [`CheckedPackage::id`]: a monotonic counter stamped once per
/// [`PackageDeclarations::check`] call. `thumbv7em-none-eabi` has
/// compare-and-swap, so `AtomicUsize` is available at this crate's MSRV
/// (1.75) on the governed target; `Ordering::Relaxed` is enough because the
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

/// A checking budget: a node-count ceiling and a call-depth bound of at most
/// [`MAX_CALL_DEPTH`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CheckingLimits {
    nodes: u64,
    depth: u64,
}

/// `depth` exceeds [`MAX_CALL_DEPTH`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DepthAboveMaximum {
    /// The refused depth.
    pub depth: u64,
}

impl CheckingLimits {
    /// A checking budget, refusing a `depth` above [`MAX_CALL_DEPTH`].
    pub fn new(nodes: u64, depth: u64) -> Result<Self, DepthAboveMaximum> {
        if depth > MAX_CALL_DEPTH {
            return Err(DepthAboveMaximum { depth });
        }
        Ok(Self { nodes, depth })
    }

    /// The node-count ceiling. Carried for parity with the authority's
    /// checking budget shape, but not enforced anywhere in this crate: a
    /// [`Body`] is an opaque host callable with no AST this crate can walk to
    /// count nodes against, so no admission or evaluation path here ever
    /// reads this value.
    pub fn nodes(self) -> u64 {
        self.nodes
    }

    /// The call-depth bound.
    pub fn depth(self) -> u64 {
        self.depth
    }
}

impl Default for CheckingLimits {
    fn default() -> Self {
        Self {
            nodes: u64::MAX,
            depth: MAX_CALL_DEPTH,
        }
    }
}

/// A host-supplied function body: given the [`Frame`] it runs in and its
/// bound arguments, produces the applied [`Outcome`]. `for<'f>` lets each
/// re-entrant call bind a fresh, shorter-lived frame rather than reusing one
/// lifetime for an entire call tree.
pub type Body = Box<dyn for<'f> Fn(&Frame<'f>, &[Value]) -> Outcome<Value>>;

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
    /// The host callable this function applies.
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
        limits: CheckingLimits,
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

        Ok(CheckedPackage {
            types: self.types,
            functions: self.functions,
            limits,
            depth: Cell::new(0),
            id: NEXT_PACKAGE_ID.fetch_add(1, Ordering::Relaxed),
        })
    }
}

/// A package admitted by [`PackageDeclarations::check`]: every declared
/// function is callable by name.
pub struct CheckedPackage {
    types: TypeEnvironment,
    functions: Vec<FunctionDeclaration>,
    limits: CheckingLimits,
    /// The re-entrant call depth currently active anywhere in this package's
    /// call tree, shared by every entry path (`call`, `evaluate`,
    /// `Frame::call`): see this module's own documentation for why the bound
    /// lives here rather than threaded per-`Frame`.
    depth: Cell<u64>,
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
/// [`TypeEnvironment`], with its own parameters and host body.
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
    /// supplied [`ObjectEnvironment`].
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
/// no charge and no body invoked. Mirrors [`super::equality::EqualityPlan`].
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
    objects: &ObjectEnvironment,
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
                Value::Collection(collection) => pending.extend(collection.elements()),
                Value::Boolean(_)
                | Value::Integer(_)
                | Value::Rational(_)
                | Value::Decimal(_)
                | Value::Float(_)
                | Value::Quantity(_)
                | Value::Text(_)
                | Value::Enum(_) => {}
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
    objects: &ObjectEnvironment,
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
/// outcome: Outcome::Refused(Refusal::CheckedInvariant), .. })`, decided here
/// at the plan boundary, before any charge — never by drifting into a
/// `Refused(CheckedInvariant)` deep inside a body that silently ran against
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
    objects: &ObjectEnvironment,
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

/// A held claim on [`CheckedPackage`]'s shared re-entrant call depth,
/// released on drop: [`CheckedPackage::enter`] increments the counter and
/// returns this guard; every early return (`?`, a refusal, an unwinding
/// panic elsewhere in the call tree — never here, this crate has none) still
/// runs [`Drop::drop`], so the counter is decremented exactly once per
/// successful `enter`, without `unsafe` or manual bookkeeping at each call
/// site.
struct DepthGuard<'p> {
    depth: &'p Cell<u64>,
    own_depth: u64,
}

impl DepthGuard<'_> {
    /// The re-entrant call depth this guard's holder is running at (`0` at
    /// the root).
    fn own_depth(&self) -> u64 {
        self.own_depth
    }
}

impl Drop for DepthGuard<'_> {
    fn drop(&mut self) {
        self.depth.set(self.depth.get().saturating_sub(1));
    }
}

impl CheckedPackage {
    fn function(&self, name: &str) -> Option<&FunctionDeclaration> {
        self.functions.iter().find(|function| function.name == name)
    }

    /// This package's own identity: a monotonic id stamped once at
    /// [`PackageDeclarations::check`] time, used to bind a
    /// [`CheckedExpression`] to the package that checked it. See
    /// [`CheckedPackage::id`]'s and [`CheckedExpression`]'s own documentation
    /// for why this is not derived from the package's address.
    fn identity(&self) -> usize {
        self.id
    }

    /// Claim one more level of this package's shared re-entrant call depth,
    /// shared by every entry path (`call`, `evaluate`, [`Frame::call`]):
    /// see this module's own documentation for why the bound lives here
    /// rather than threaded per-[`Frame`]. Refuses with
    /// [`Refusal::CheckedInvariant`] at [`CheckingLimits::depth`] before any
    /// charge: `check` admits no recursion without a discharged termination
    /// measure, so reaching the bound means a checked-program invariant was
    /// violated.
    fn enter(&self) -> Result<DepthGuard<'_>, Stop> {
        let own_depth = self.depth.get();
        if own_depth >= self.limits.depth() {
            return Err(Stop::Refused(Refusal::CheckedInvariant));
        }
        self.depth.set(own_depth.saturating_add(1));
        Ok(DepthGuard {
            depth: &self.depth,
            own_depth,
        })
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
            if let Err(IllTyped { cause }) = self.types.check_type(value_type) {
                return Err(refuse(CheckCause::IllTyped(cause)));
            }
        }
        if let Err(IllTyped { cause }) = self.types.check_type(&result) {
            return Err(refuse(CheckCause::IllTyped(cause)));
        }
        Ok(CheckedExpression {
            parameters,
            root,
            package: self.identity(),
        })
    }

    /// Apply the named function: validate (no charge), charge one
    /// `function.call` for its own body, then run it in a fresh root
    /// [`Frame`]. `evaluate` does not charge for its own root; every
    /// application it reaches charges through [`Frame::call`].
    pub fn call(
        &self,
        function: &str,
        arguments: Vec<Value>,
        objects: &ObjectEnvironment,
        meter: &mut Meter,
    ) -> Result<Evaluation, InputRefusal> {
        plan_call(self, function, &arguments, objects)?;
        let outcome = self.run_call(function, &arguments, objects, meter);
        Ok(Evaluation {
            outcome,
            location: None,
            losses: Vec::new(),
        })
    }

    /// Returns the body's own [`Outcome`] as is, never round-tripping it
    /// through `Result<Value, Stop>` (IR-286: each wrap of a `Value` in
    /// another enum is a byte-level union update CBMC cannot afford).
    fn run_call(
        &self,
        function: &str,
        arguments: &[Value],
        objects: &ObjectEnvironment,
        meter: &mut Meter,
    ) -> Outcome<Value> {
        let guard = match self.enter() {
            Ok(guard) => guard,
            Err(stop) => return Outcome::from_stop(Err(stop)),
        };
        if let Err(stop) = charge_call(meter) {
            return Outcome::from_stop(Err(stop));
        }
        let Some(declaration) = self.function(function) else {
            return Outcome::Refused(Refusal::CheckedInvariant);
        };
        let cell = RefCell::new(meter);
        let frame = Frame {
            package: self,
            objects,
            meter: cell,
            depth: guard.own_depth(),
        };
        (declaration.body)(&frame, arguments)
    }

    /// Evaluate a [`CheckedExpression`] against this package: validate (no
    /// charge, including that `expression` was checked against this same
    /// package), then run its root in a fresh root [`Frame`] with no charge
    /// for the root itself.
    pub fn evaluate(
        &self,
        expression: &CheckedExpression,
        arguments: Vec<Value>,
        objects: &ObjectEnvironment,
        meter: &mut Meter,
    ) -> Result<Evaluation, InputRefusal> {
        match plan_evaluation(self, expression, &arguments, objects) {
            Ok(_) => {}
            Err(EvaluationRefusal::Input(refusal)) => return Err(refusal),
            Err(EvaluationRefusal::ForeignExpression) => {
                return Ok(Evaluation {
                    outcome: Outcome::Refused(Refusal::CheckedInvariant),
                    location: None,
                    losses: Vec::new(),
                });
            }
        }
        let outcome = self.run_evaluate(expression, &arguments, objects, meter);
        Ok(Evaluation {
            outcome,
            location: None,
            losses: Vec::new(),
        })
    }

    fn run_evaluate(
        &self,
        expression: &CheckedExpression,
        arguments: &[Value],
        objects: &ObjectEnvironment,
        meter: &mut Meter,
    ) -> Outcome<Value> {
        let guard = match self.enter() {
            Ok(guard) => guard,
            Err(stop) => return Outcome::from_stop(Err(stop)),
        };
        let cell = RefCell::new(meter);
        let frame = Frame {
            package: self,
            objects,
            meter: cell,
            depth: guard.own_depth(),
        };
        (expression.root)(&frame, arguments)
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

/// The re-entrant call surface a running [`Body`] sees: the applications it
/// makes, and the [`Meter`] and [`ObjectEnvironment`] it runs against.
///
/// `depth` is informational only — read it back through [`Frame::depth`],
/// but the bound itself is not enforced here. A `Frame`'s own field is
/// threaded purely by value, and a fresh root `Frame` built at `depth: 0`
/// looks identical to a program's first call, so a running [`Body`] that
/// holds its own `Rc<CheckedPackage>` and re-enters [`CheckedPackage::call`]
/// or [`CheckedPackage::evaluate`] directly (obtainable in safe Rust) would
/// bypass a per-`Frame` check entirely. The actual bound lives on
/// [`CheckedPackage`] itself, as a [`Cell`] counter every entry path shares
/// (see this module's own documentation and the private `enter`), so
/// re-entry through a given checked package is bounded alike on every path.
///
/// The [`Meter`] is reached through a [`RefCell`] because many `Frame`s
/// across one call tree share it; `RefCell`, never `unsafe` or atomics.
/// Every access goes through `try_borrow_mut` rather than `borrow_mut`, so a
/// body that re-enters its own frame's `Meter` (through [`Frame::meter`] or
/// [`Frame::call`], both re-entrant from within the very closure holding the
/// outer borrow) is refused a typed, non-panicking result instead of
/// aborting the process.
pub struct Frame<'a> {
    package: &'a CheckedPackage,
    objects: &'a ObjectEnvironment,
    meter: RefCell<&'a mut Meter>,
    depth: u64,
}

impl<'a> Frame<'a> {
    /// Apply the named function from within a running body: charge one
    /// `function.call`, then run it in a frame one deeper than this one.
    /// Beyond [`CheckingLimits::depth`], refuses with
    /// [`Refusal::CheckedInvariant`] before any charge: `check` admits no
    /// recursion without a discharged termination measure, so reaching the
    /// bound means a checked-program invariant was violated. Also refuses
    /// with [`Refusal::CheckedInvariant`] — never panics — if this frame's
    /// shared [`Meter`] is already mutably borrowed by an enclosing call on
    /// the same re-entrant chain.
    pub fn call(&self, function: &str, arguments: &[Value]) -> Outcome<Value> {
        let guard = match self.package.enter() {
            Ok(guard) => guard,
            Err(stop) => return Outcome::from_stop(Err(stop)),
        };
        let Some(declaration) = self.package.function(function) else {
            return Outcome::Refused(Refusal::CheckedInvariant);
        };
        {
            let Ok(mut meter) = self.meter.try_borrow_mut() else {
                return Outcome::Refused(Refusal::CheckedInvariant);
            };
            if let Err(stop) = charge_call(&mut meter) {
                return Outcome::from_stop(Err(stop));
            }
        }
        let Ok(mut meter) = self.meter.try_borrow_mut() else {
            return Outcome::Refused(Refusal::CheckedInvariant);
        };
        let child = Frame {
            package: self.package,
            objects: self.objects,
            meter: RefCell::new(&mut meter),
            depth: guard.own_depth(),
        };
        (declaration.body)(&child, arguments)
    }

    /// Run `run` against the shared [`Meter`] this frame's whole call tree
    /// charges through. `Err(Refusal::CheckedInvariant)` if this frame's
    /// `Meter` is already mutably borrowed by an enclosing call on the same
    /// re-entrant chain, rather than panicking on the double borrow.
    /// Returning `Result` rather than `Option` matters here: a body that
    /// needed to charge through this access and silently swallowed a `None`
    /// would keep running unmetered with no signal, exactly the condition
    /// [`Frame::call`] itself maps to a refusal rather than an absent value.
    /// Propagate with `?`, then fold a returned `Err(r)` into
    /// `Outcome::Refused(r)` to keep returning the body's own declared
    /// `Outcome<Value>`.
    pub fn meter<R>(&self, run: impl FnOnce(&mut Meter) -> R) -> Result<R, Refusal> {
        let mut guard = self
            .meter
            .try_borrow_mut()
            .map_err(|_| Refusal::CheckedInvariant)?;
        Ok(run(&mut guard))
    }

    /// The [`ObjectEnvironment`] this call was made against.
    pub fn objects(&self) -> &ObjectEnvironment {
        self.objects
    }

    /// This frame's re-entrant call depth: the value of [`CheckedPackage`]'s
    /// shared depth counter at the instant this frame was built. `0` only for
    /// a program's actual first call into an otherwise-idle package; a root
    /// [`Frame`] built by a direct, bypassing re-entry into
    /// [`CheckedPackage::call`] or [`CheckedPackage::evaluate`] (from within
    /// a running [`Body`] holding its own `Rc<CheckedPackage>`) reports the
    /// depth already active on that package, which is non-zero.
    pub fn depth(&self) -> u64 {
        self.depth
    }
}
