// SPDX-License-Identifier: AGPL-3.0-or-later
//! The single retained executable representation, independent of compiler storage.

use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::fmt;

use quire_exact::{NodeKey, Rational, Value, ValueType};
use quire_semantic_value::declaration::{EnvironmentFailure, TypeEnvironment};

/// An address in the owned expression arena, never a source identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpressionId(pub usize);

/// An address in the owned function arena, never a source identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FunctionId(pub usize);

/// One source type and its shared executable value type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeBinding {
    /// Complete admitted source key, including its package and structural path.
    pub key: NodeKey,
    /// Shared leaf type; this is not a monitor-specific type vocabulary.
    pub value_type: ValueType,
}

/// The four resolved function entries of the admitted plan root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entries {
    /// `() -> State`.
    pub init: FunctionId,
    /// `(State, Integer delta, Inputs) -> (State, Results)`.
    pub on_event: FunctionId,
    /// `(State, Integer boundary, Scope) -> (State, Results)`.
    pub on_watermark: FunctionId,
    /// `(State, Integer gap) -> (State, Results)`.
    pub on_gap: FunctionId,
}

/// One capacity premise and its exact admitted subformula owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer {
    /// Complete source subformula key.
    pub owner: NodeKey,
    /// Fixed maximum active element count, not a derived capacity formula.
    pub capacity: usize,
}

/// Source selection, entry associations and exact premises retained by the program.
pub struct Layout {
    /// The complete identity-bearing plan-root key.
    pub root: NodeKey,
    /// The independently selected clause key.
    pub clause: NodeKey,
    /// Canonical bytes of the independently selected package lock.
    ///
    /// The compiler binds these bytes before construction; RT does not parse
    /// or replace the lock with a local package counter.
    pub package_lock: Box<[u8]>,
    /// Independently selected subject identity.
    pub subject: String,
    /// Exact source occurrence identity retained by the lowering boundary.
    pub occurrence: Box<[u8]>,
    /// The admitted timestamped-event profile identity.
    pub profile: String,
    /// Typed persistent state.
    pub state: TypeBinding,
    /// Typed event captures.
    pub inputs: TypeBinding,
    /// Typed admitted-progress scope.
    pub scope: TypeBinding,
    /// Typed sequence of obligation results.
    pub results: TypeBinding,
    /// The exact two-position `(State, Results)` entry result type.
    pub step_result: TypeBinding,
    /// Resolved entry addresses; their source keys remain in the function arena.
    pub entries: Entries,
    /// Nonempty hardware counter identity, distinct from the clause clock name.
    pub counter: String,
    /// The admitted base-Time unit's source key.
    pub tick_unit: NodeKey,
    /// Independently resolved base Time dimension key.
    ///
    /// The unit and its exact scale are retained in the shared type
    /// environment's unit table, rather than copied into another unit record.
    pub time_dimension: NodeKey,
    /// Declared counter width, from one through 64 bits.
    pub counter_width_bits: u8,
    /// Largest admitted consecutive modular difference.
    pub max_reading_gap: u64,
    /// Exact positive event rate in events per tick.
    pub max_event_rate: Rational,
    /// Exact nonnegative clock uncertainty in ticks.
    pub uncertainty: Rational,
    /// Capacity associations in the admitted subformula-key order.
    pub buffers: Vec<Buffer>,
}

/// One ordinary closed executable expression.
///
/// These operations describe lowered expression execution, not temporal
/// formula tags. Further supported operations require a coordinated capability
/// extension; an absent operation must not be replaced by a callback.
pub enum Operation {
    /// An already admitted immutable shared value.
    Literal(Value),
    /// A parameter of this exact owning function.
    Argument {
        /// Resolved owning function.
        function: FunctionId,
        /// Position in that function's declared parameter order.
        position: usize,
    },
    /// An ordinary checked call, with operands in declaration order.
    Call {
        /// Resolved callee.
        function: FunctionId,
        /// Ordered argument expressions.
        arguments: Vec<ExpressionId>,
    },
    /// A lazy ordinary conditional; only the selected branch is evaluated.
    If {
        /// Boolean condition.
        condition: ExpressionId,
        /// Result when the condition is true.
        when_true: ExpressionId,
        /// Result when the condition is false.
        when_false: ExpressionId,
    },
}

/// A source-keyed expression in the flat executable arena.
pub struct Expression {
    /// Exact admitted source identity.
    pub key: NodeKey,
    /// Child path within that node's body for an inline expression.
    pub body_path: Vec<usize>,
    /// Checked shared result type.
    pub value_type: ValueType,
    /// Executable operation and its resolved operands.
    pub operation: Operation,
}

/// A source-keyed function in the retained executable closure.
pub struct Function {
    /// Exact admitted entry or helper identity.
    pub key: NodeKey,
    /// Parameter types in declaration order.
    pub parameters: Vec<ValueType>,
    /// Checked result type.
    pub result: ValueType,
    /// Root expression in the program's arena.
    pub body: ExpressionId,
}

/// Consumed construction parts of the one owned representation.
///
/// Construction moves these arenas into `Program`; this is not another wire
/// format, a source parser, or a retained mirror of the checked package.
pub struct ProgramParts {
    /// Independently admitted selection and root associations.
    pub layout: Layout,
    /// Shared admitted nominal declarations.
    pub types: TypeEnvironment,
    /// Complete retained function closure.
    pub functions: Vec<Function>,
    /// Complete retained expression closure.
    pub expressions: Vec<Expression>,
}

/// Stable construction refusal category, before any initializer executes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramRefusalCode {
    /// A required selection or exact premise is absent or out of range.
    InvalidLayout,
    /// Two retained source nodes have the same key.
    DuplicateKey,
    /// A resolved arena address does not exist.
    InvalidReference,
    /// A type or operand does not match its checked declaration.
    TypeMismatch,
    /// An argument belongs to a different function invocation.
    ForeignArgument,
    /// The expression edges contain a cycle rather than a finite expression.
    ExpressionCycle,
    /// Validation scratch could not be reserved.
    Storage,
    /// The shared value-membership walk stopped; its native cause is retained.
    Membership,
}

/// Typed construction refusal with the exact source node when available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramRefusal {
    /// Stable machine-readable cause.
    pub code: ProgramRefusalCode,
    /// Source node that failed validation.
    pub node: Option<NodeKey>,
    /// Native shared membership failure, when that walk stopped.
    pub membership_failure: Option<EnvironmentFailure>,
}

impl fmt::Display for ProgramRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "monitor program refused: {:?}", self.code)
    }
}

impl core::error::Error for ProgramRefusal {}

/// A checked immutable program retained independently of driver preparation.
pub struct Program {
    pub(super) parts: ProgramParts,
}

impl Program {
    /// Check the closed executable representation and retain its complete owner.
    ///
    /// Source selection and temporal admission belong to the compiler. This
    /// checks executable integrity, without running any entry. It does not
    /// promise recovery from arbitrary allocator exhaustion.
    pub fn try_new(parts: ProgramParts) -> Result<Rc<Self>, ProgramRefusal> {
        super::validation::validate(&parts)?;
        Ok(Rc::new(Self { parts }))
    }

    /// Inspect the exact retained selection and root premises.
    pub fn layout(&self) -> &Layout {
        &self.parts.layout
    }

    /// Inspect the retained shared declaration environment.
    pub fn types(&self) -> &TypeEnvironment {
        &self.parts.types
    }

    /// Resolve a function address without inventing a replacement function.
    pub fn function(&self, id: FunctionId) -> Option<&Function> {
        self.parts.functions.get(id.0)
    }

    /// Resolve an expression address without inventing a replacement body.
    pub fn expression(&self, id: ExpressionId) -> Option<&Expression> {
        self.parts.expressions.get(id.0)
    }
}
