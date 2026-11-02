// SPDX-License-Identifier: AGPL-3.0-or-later
//! Iterative execution of ordinary retained expressions, with no source evaluator.

mod collections;
mod folding;
mod unions;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::cell::{Cell, RefCell};

use quire_exact::{
    Cancel, CollectionValue, CompositeValue, FieldValue, Integer, IntegerArithmetic, Meter,
    OrderedOperands, Outcome, UnionValue, Value, ValueType,
};
use quire_semantic_value::{
    declaration::{CompositeShape, StorageRequest, StorageUnit},
    supplied_membership::{
        InvalidMembershipCause, MembershipLocus, MembershipPart, MembershipPathArena,
        PathArenaCapacity, SuppliedAdmissionError, SuppliedMembershipBudget,
    },
};

use super::budget::{StepBudget, StepResource, StepResourceStop};
use super::{ExpressionId, FunctionId, LocalId, Operation, Program};

#[derive(Clone, Copy, Debug)]
pub(super) enum Site {
    Expression(ExpressionId),
    Function(FunctionId),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Broken {
    Address,
    ArgumentOwner,
    Arity,
    ValueStack,
    Condition,
    Binding,
    Projection,
}

pub(super) enum Failure {
    Scalar {
        site: Site,
        outcome: Box<Outcome<Value>>,
    },
    Stop(StepResourceStop),
    Invariant {
        site: Site,
        cause: Broken,
    },
    Membership(Box<MembershipFailure>),
    MembershipStop {
        stop: StepResourceStop,
        native: Box<MembershipFailure>,
    },
    PathCapacity(PathArenaCapacity),
}

pub(super) struct MembershipFailure {
    pub(super) site: Site,
    pub(super) failure: SuppliedAdmissionError,
    pub(super) arena: MembershipPathArena,
}

impl From<StepResourceStop> for Failure {
    fn from(stop: StepResourceStop) -> Self {
        Self::Stop(stop)
    }
}

fn broken(site: Site, cause: Broken) -> Failure {
    Failure::Invariant { site, cause }
}

#[derive(Clone, Copy)]
enum Operand {
    Literal(ExpressionId),
    Supplied(usize),
    // Select one immutable canonical primitive; this is not another Value type.
    BooleanConstant(bool),
    Projection(usize),
    Computed(usize),
}

enum Projection {
    Sequence {
        owner: Arc<CollectionValue>,
        position: usize,
    },
    Composite {
        owner: Arc<CompositeValue>,
        position: usize,
    },
    Union {
        owner: Arc<UnionValue>,
        position: usize,
    },
}

impl Projection {
    fn value(&self) -> Option<&Value> {
        match self {
            Self::Sequence { owner, position } => owner.elements().get(*position),
            Self::Composite { owner, position } => match owner.slots().get(*position)? {
                FieldValue::Present(value) => Some(value),
                _ => None,
            },
            Self::Union { owner, position } => owner.payload().get(*position),
        }
    }
}

#[derive(Clone, Copy)]
struct Scope {
    function: FunctionId,
    start: usize,
    len: usize,
    invocation: u64,
}

#[derive(Clone, Copy)]
struct ActiveBinding {
    invocation: u64,
    operand: Operand,
}

enum Frame {
    FoldStart {
        expression: ExpressionId,
        scope: Scope,
    },
    FoldNext(folding::FoldState),
    FoldBody(folding::FoldState),
    Unary {
        expression: ExpressionId,
    },
    Equality {
        expression: ExpressionId,
    },
    SequenceElement {
        expression: ExpressionId,
        position: usize,
        scope: Scope,
    },
    Construct {
        expression: ExpressionId,
        values: usize,
    },
    ConstructCheck {
        expression: ExpressionId,
        position: usize,
    },
    Scalar {
        expression: ExpressionId,
    },
    TupleElement {
        expression: ExpressionId,
    },
    BooleanLeft {
        expression: ExpressionId,
        scope: Scope,
    },
    BooleanRight {
        expression: ExpressionId,
    },
    Bind {
        expression: ExpressionId,
        scope: Scope,
    },
    Restore {
        expression: ExpressionId,
        slot: usize,
        previous: Option<ActiveBinding>,
        invocation: u64,
    },
    Evaluate {
        expression: ExpressionId,
        scope: Scope,
    },
    Choose {
        expression: ExpressionId,
        scope: Scope,
    },
    Check {
        expression: ExpressionId,
        edge: usize,
    },
    Apply {
        expression: ExpressionId,
        values: usize,
    },
    Return {
        function: FunctionId,
        arguments: usize,
        values: usize,
    },
}

struct Checks {
    expressions: Vec<Vec<bool>>,
    returns: Vec<bool>,
}

impl Checks {
    fn new(program: &Program) -> Result<Self, Failure> {
        let mut expressions = Vec::new();
        expressions
            .try_reserve_exact(program.parts.expressions.len())
            .map_err(|_| StepResourceStop::storage(program.parts.expressions.len()))?;
        for (index, expression) in program.parts.expressions.iter().enumerate() {
            let site = Site::Expression(ExpressionId(index));
            let mut edges = Vec::new();
            match &expression.operation {
                Operation::Literal(_)
                | Operation::Argument { .. }
                | Operation::BooleanNot { .. }
                | Operation::BooleanAnd { .. }
                | Operation::BooleanOr { .. }
                | Operation::TupleElement { .. }
                | Operation::RecordField { .. }
                | Operation::IntegerAdd { .. }
                | Operation::IntegerSubtract { .. }
                | Operation::IntegerMultiply { .. }
                | Operation::IntegerNegate { .. }
                | Operation::IntegerOrder { .. }
                | Operation::Equality { .. }
                | Operation::UnionIs { .. }
                | Operation::UnionPayload { .. }
                | Operation::SequenceSize { .. }
                | Operation::SequenceFlatten { .. } => {}
                Operation::Record { .. }
                | Operation::Tuple { .. }
                | Operation::Union { .. }
                | Operation::Sequence { .. } => {
                    let operands = super::construction::operands(program, ExpressionId(index))
                        .ok_or_else(|| broken(site, Broken::Address))?;
                    edges
                        .try_reserve_exact(operands.len())
                        .map_err(|_| StepResourceStop::storage(operands.len()))?;
                    for (position, operand) in operands.iter().enumerate() {
                        let child = program
                            .expression(*operand)
                            .ok_or_else(|| broken(site, Broken::Address))?;
                        let expected =
                            super::construction::expected(program, ExpressionId(index), position)
                                .ok_or_else(|| broken(site, Broken::Address))?;
                        edges.push(program.types().same_type(&child.value_type, expected));
                    }
                }
                Operation::Local(binding) => {
                    let (_, source) = program
                        .bindings
                        .resolve(*binding)
                        .ok_or_else(|| broken(site, Broken::Binding))?;
                    let value_type = source
                        .value_type(&program.parts)
                        .ok_or_else(|| broken(site, Broken::Address))?;
                    edges
                        .try_reserve_exact(1)
                        .map_err(|_| StepResourceStop::storage(1))?;
                    edges.push(
                        program
                            .types()
                            .same_type(value_type, &expression.value_type),
                    );
                }
                Operation::SequenceFold { initial, body, .. } => {
                    edges
                        .try_reserve_exact(2)
                        .map_err(|_| StepResourceStop::storage(2))?;
                    for child in [initial, body] {
                        let value = program
                            .expression(*child)
                            .ok_or_else(|| broken(site, Broken::Address))?;
                        edges.push(
                            program
                                .types()
                                .same_type(&value.value_type, &expression.value_type),
                        );
                    }
                }
                Operation::Let { body, .. } => {
                    let value = program
                        .expression(*body)
                        .ok_or_else(|| broken(site, Broken::Address))?;
                    edges
                        .try_reserve_exact(1)
                        .map_err(|_| StepResourceStop::storage(1))?;
                    edges.push(
                        program
                            .types()
                            .same_type(&value.value_type, &expression.value_type),
                    );
                }
                Operation::If {
                    when_true,
                    when_false,
                    ..
                } => {
                    edges
                        .try_reserve_exact(2)
                        .map_err(|_| StepResourceStop::storage(2))?;
                    for child in [when_true, when_false] {
                        let child = program
                            .expression(*child)
                            .ok_or_else(|| broken(site, Broken::Address))?;
                        edges.push(
                            program
                                .types()
                                .same_type(&child.value_type, &expression.value_type),
                        );
                    }
                }
                Operation::Call {
                    function,
                    arguments,
                } => {
                    let function = program
                        .function(*function)
                        .ok_or_else(|| broken(site, Broken::Address))?;
                    if function.parameters.len() != arguments.len() {
                        return Err(broken(site, Broken::Arity));
                    }
                    let count = arguments
                        .len()
                        .checked_add(1)
                        .ok_or_else(|| StepResourceStop::storage(usize::MAX))?;
                    edges
                        .try_reserve_exact(count)
                        .map_err(|_| StepResourceStop::storage(count))?;
                    for (argument, expected) in arguments.iter().zip(&function.parameters) {
                        let argument = program
                            .expression(*argument)
                            .ok_or_else(|| broken(site, Broken::Address))?;
                        edges.push(program.types().same_type(&argument.value_type, expected));
                    }
                    edges.push(
                        program
                            .types()
                            .same_type(&function.result, &expression.value_type),
                    );
                }
            }
            expressions.push(edges);
        }
        let mut returns = Vec::new();
        returns
            .try_reserve_exact(program.parts.functions.len())
            .map_err(|_| StepResourceStop::storage(program.parts.functions.len()))?;
        for (index, function) in program.parts.functions.iter().enumerate() {
            let body = program
                .expression(function.body)
                .ok_or_else(|| broken(Site::Function(FunctionId(index)), Broken::Address))?;
            returns.push(
                program
                    .types()
                    .same_type(&body.value_type, &function.result),
            );
        }
        Ok(Self {
            expressions,
            returns,
        })
    }
}

struct Resources {
    limits: StepBudget,
    work: u64,
    elements: Cell<usize>,
    bytes: Cell<usize>,
    denied: RefCell<Option<StepResourceStop>>,
    scalar: Meter,
    live_elements: Cell<usize>,
}

impl Resources {
    fn new(limits: StepBudget) -> Self {
        Self {
            limits,
            work: 0,
            elements: Cell::new(0),
            bytes: Cell::new(0),
            denied: RefCell::new(None),
            scalar: Meter::new(limits.scalar),
            live_elements: Cell::new(0),
        }
    }
    fn charge(&mut self) -> Result<(), StepResourceStop> {
        let next = self.work.checked_add(1);
        if next.is_none_or(|next| next > self.limits.work) {
            return Err(StepResourceStop {
                resource: StepResource::Work,
                limit: Some(u128::from(self.limits.work)),
                spent: u128::from(self.work),
                requested: Integer::one(),
            });
        }
        self.work = next.ok_or_else(|| StepResourceStop::storage(1))?;
        Ok(())
    }
    fn permit(&self, request: StorageRequest) -> bool {
        let (used, limit, resource) = match request.unit() {
            StorageUnit::AdditionalElements => (
                &self.elements,
                self.limits.scratch_elements,
                StepResource::ScratchElements,
            ),
            StorageUnit::Bytes => (
                &self.bytes,
                self.limits.scratch_bytes,
                StepResource::ScratchBytes,
            ),
        };
        match used.get().checked_add(request.amount()) {
            Some(next)
                if next
                    .checked_add(if request.unit() == StorageUnit::AdditionalElements {
                        self.live_elements.get()
                    } else {
                        0
                    })
                    .is_some_and(|total| total <= limit) =>
            {
                used.set(next);
                true
            }
            _ => {
                self.denied.replace(Some(StepResourceStop {
                    resource,
                    limit: Some(limit as u128),
                    spent: used.get().saturating_add(
                        if request.unit() == StorageUnit::AdditionalElements {
                            self.live_elements.get()
                        } else {
                            0
                        },
                    ) as u128,
                    requested: Integer::from(request.amount()),
                }));
                false
            }
        }
    }
}

/// Invocation addresses, cached native type facts and canonical computed values.
pub(super) struct Machine {
    checks: Checks,
    frames: Vec<Frame>,
    values: Vec<Operand>,
    arguments: Vec<Operand>,
    bindings: Vec<Option<ActiveBinding>>,
    touched_flags: Vec<bool>,
    touched: Vec<usize>,
    active_bindings: usize,
    active_folds: usize,
    projections: Vec<Projection>,
    computed: Vec<Value>,
}

impl Machine {
    pub(super) fn new(program: &Program, limits: StepBudget) -> Result<Self, Failure> {
        let checks = Checks::new(program)?;
        let mut frames = Vec::new();
        let mut values = Vec::new();
        let mut arguments = Vec::new();
        frames
            .try_reserve_exact(limits.stack_frames)
            .map_err(|_| StepResourceStop::storage(limits.stack_frames))?;
        values
            .try_reserve_exact(limits.scratch_elements)
            .map_err(|_| StepResourceStop::storage(limits.scratch_elements))?;
        arguments
            .try_reserve_exact(limits.scratch_elements)
            .map_err(|_| StepResourceStop::storage(limits.scratch_elements))?;
        let mut projections = Vec::new();
        if program.parts.expressions.iter().any(|node| {
            matches!(
                node.operation,
                Operation::TupleElement { .. }
                    | Operation::RecordField { .. }
                    | Operation::UnionPayload { .. }
                    | Operation::SequenceFold { .. }
            )
        }) {
            projections
                .try_reserve_exact(limits.scratch_elements)
                .map_err(|_| StepResourceStop::storage(limits.scratch_elements))?;
        }
        let mut computed = Vec::new();
        if program.parts.expressions.iter().any(|node| {
            matches!(
                node.operation,
                Operation::IntegerAdd { .. }
                    | Operation::IntegerSubtract { .. }
                    | Operation::IntegerMultiply { .. }
                    | Operation::IntegerNegate { .. }
                    | Operation::Record { .. }
                    | Operation::Tuple { .. }
                    | Operation::Union { .. }
                    | Operation::Sequence { .. }
                    | Operation::SequenceSize { .. }
                    | Operation::SequenceFlatten { .. }
            )
        }) {
            computed
                .try_reserve_exact(limits.scratch_elements)
                .map_err(|_| StepResourceStop::storage(limits.scratch_elements))?;
        }
        let count = program.bindings.len();
        let mut bindings = Vec::new();
        let mut touched_flags = Vec::new();
        let mut touched = Vec::new();
        bindings
            .try_reserve_exact(count)
            .map_err(|_| StepResourceStop::storage(count))?;
        touched_flags
            .try_reserve_exact(count)
            .map_err(|_| StepResourceStop::storage(count))?;
        touched
            .try_reserve_exact(count)
            .map_err(|_| StepResourceStop::storage(count))?;
        bindings.resize(count, None);
        touched_flags.resize(count, false);
        Ok(Self {
            bindings,
            touched_flags,
            touched,
            active_bindings: 0,
            active_folds: 0,
            projections,
            computed,
            checks,
            frames,
            values,
            arguments,
        })
    }

    fn frame(&mut self, frame: Frame, resources: &Resources) -> Result<(), Failure> {
        if self.frames.len() >= resources.limits.stack_frames {
            return Err(StepResourceStop {
                resource: StepResource::Stack,
                limit: Some(resources.limits.stack_frames as u128),
                spent: self.frames.len() as u128,
                requested: Integer::one(),
            }
            .into());
        }
        self.frames.push(frame);
        Ok(())
    }

    fn slot(&self, resources: &Resources) -> Result<(), Failure> {
        let used = self
            .values
            .len()
            .checked_add(self.arguments.len())
            .and_then(|n| n.checked_add(self.active_bindings))
            .and_then(|n| n.checked_add(self.active_folds))
            .and_then(|n| n.checked_add(self.projections.len()))
            .and_then(|n| n.checked_add(self.computed.len()))
            .and_then(|n| n.checked_add(resources.elements.get()));
        if used.is_none_or(|n| n >= resources.limits.scratch_elements) {
            return Err(StepResourceStop {
                resource: StepResource::ScratchElements,
                limit: Some(resources.limits.scratch_elements as u128),
                spent: used.unwrap_or(usize::MAX) as u128,
                requested: Integer::one(),
            }
            .into());
        }
        Ok(())
    }

    fn value(&mut self, operand: Operand, resources: &Resources) -> Result<(), Failure> {
        self.slot(resources)?;
        self.values.push(operand);
        Ok(())
    }

    /// The caller already admitted each root argument against this exact signature.
    pub(super) fn invoke(
        &mut self,
        program: &Program,
        function: FunctionId,
        supplied: &[&Value],
        limits: StepBudget,
    ) -> Result<Value, Failure> {
        self.frames.clear();
        self.values.clear();
        self.arguments.clear();
        let mut resources = Resources::new(limits);
        let result = self
            .run(program, function, supplied, &mut resources)
            .and_then(|operand| {
                let site = Site::Function(function);
                match resolve(
                    program,
                    supplied,
                    &self.projections,
                    &self.computed,
                    operand,
                    site,
                )? {
                    Value::Composite(value) => Ok(Value::Composite(Arc::clone(value))),
                    _ => Err(broken(site, Broken::ValueStack)),
                }
            });
        self.frames.clear();
        self.values.clear();
        self.arguments.clear();
        self.clear_bindings();
        self.active_folds = 0;
        self.projections.clear();
        self.computed.clear();
        result
    }

    fn run(
        &mut self,
        program: &Program,
        function: FunctionId,
        supplied: &[&Value],
        resources: &mut Resources,
    ) -> Result<Operand, Failure> {
        let declared = program
            .function(function)
            .ok_or_else(|| broken(Site::Function(function), Broken::Address))?;
        if declared.parameters.len() != supplied.len() {
            return Err(broken(Site::Function(function), Broken::Arity));
        }
        for index in 0..supplied.len() {
            self.slot(resources)?;
            self.arguments.push(Operand::Supplied(index));
        }
        let scope = Scope {
            function,
            start: 0,
            len: supplied.len(),
            invocation: 0,
        };
        self.frame(
            Frame::Return {
                function,
                arguments: 0,
                values: 0,
            },
            resources,
        )?;
        self.frame(
            Frame::Evaluate {
                expression: declared.body,
                scope,
            },
            resources,
        )?;
        while let Some(frame) = self.frames.pop() {
            resources.charge()?;
            match frame {
                Frame::FoldStart { expression, scope } => {
                    self.fold_start(program, supplied, expression, scope, resources)?
                }
                Frame::FoldNext(state) => self.fold_next(program, supplied, state, resources)?,
                Frame::FoldBody(state) => self.fold_body(program, supplied, state, resources)?,
                Frame::Equality { expression } => {
                    self.equality(program, supplied, expression, resources)?
                }
                Frame::Unary { expression } => {
                    self.unary(program, supplied, expression, resources)?
                }
                Frame::SequenceElement {
                    expression,
                    position,
                    scope,
                } => self.sequence_element(program, expression, position, scope, resources)?,
                Frame::Construct { expression, values } => {
                    self.construct(program, supplied, expression, values, resources)?
                }
                Frame::ConstructCheck {
                    expression,
                    position,
                } => self.construct_check(program, supplied, expression, position, resources)?,
                Frame::Scalar { expression } => {
                    self.scalar(program, supplied, expression, resources)?
                }
                Frame::TupleElement { expression } => {
                    self.tuple_element(program, supplied, expression, resources)?
                }
                Frame::BooleanLeft { expression, scope } => {
                    self.boolean_left(program, supplied, expression, scope, resources)?
                }
                Frame::BooleanRight { expression } => {
                    self.boolean_right(program, supplied, expression, resources)?
                }
                Frame::Bind { expression, scope } => {
                    self.bind(program, expression, scope, resources)?
                }
                Frame::Restore {
                    expression,
                    slot,
                    previous,
                    invocation,
                } => self.restore(expression, slot, previous, invocation)?,
                Frame::Evaluate { expression, scope } => {
                    self.evaluate(program, supplied, expression, scope, resources)?
                }
                Frame::Choose { expression, scope } => {
                    self.choose(program, supplied, expression, scope, resources)?
                }
                Frame::Apply { expression, values } => {
                    self.apply(program, supplied, expression, values, resources)?
                }
                Frame::Check { expression, edge } => {
                    self.check(program, supplied, expression, edge, resources)?
                }
                Frame::Return {
                    function,
                    arguments,
                    values,
                } => {
                    let site = Site::Function(function);
                    let operand = self
                        .values
                        .pop()
                        .ok_or_else(|| broken(site, Broken::ValueStack))?;
                    if self.values.len() != values {
                        return Err(broken(site, Broken::ValueStack));
                    }
                    let equal = *self
                        .checks
                        .returns
                        .get(function.0)
                        .ok_or_else(|| broken(site, Broken::Address))?;
                    if !equal {
                        let expected = &program
                            .function(function)
                            .ok_or_else(|| broken(site, Broken::Address))?
                            .result;
                        self.admit(program, supplied, operand, expected, site, resources)?;
                    }
                    self.arguments.truncate(arguments);
                    self.value(operand, resources)?;
                }
            }
        }
        let [operand] = self.values.as_slice() else {
            return Err(broken(Site::Function(function), Broken::ValueStack));
        };
        Ok(*operand)
    }

    fn evaluate(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        match &node.operation {
            Operation::Literal(_) => self.value(Operand::Literal(expression), resources),
            Operation::Local(binding) => {
                self.read_local(program, supplied, expression, *binding, scope, resources)
            }
            Operation::Let { value, .. } => {
                self.frame(Frame::Bind { expression, scope }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *value,
                        scope,
                    },
                    resources,
                )
            }
            Operation::Argument { function, position } => {
                if *function != scope.function || *position >= scope.len {
                    return Err(broken(site, Broken::ArgumentOwner));
                }
                let index = scope
                    .start
                    .checked_add(*position)
                    .ok_or_else(|| broken(site, Broken::Address))?;
                let operand = *self
                    .arguments
                    .get(index)
                    .ok_or_else(|| broken(site, Broken::Address))?;
                self.value(operand, resources)
            }
            Operation::TupleElement { value, .. } | Operation::RecordField { value, .. } => {
                self.frame(Frame::TupleElement { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *value,
                        scope,
                    },
                    resources,
                )
            }
            Operation::SequenceFold {
                sequence, initial, ..
            } => {
                self.frame(Frame::FoldStart { expression, scope }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *initial,
                        scope,
                    },
                    resources,
                )?;
                self.frame(
                    Frame::Evaluate {
                        expression: *sequence,
                        scope,
                    },
                    resources,
                )
            }
            Operation::Equality { left, right, .. } => {
                self.frame(Frame::Equality { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *right,
                        scope,
                    },
                    resources,
                )?;
                self.frame(
                    Frame::Evaluate {
                        expression: *left,
                        scope,
                    },
                    resources,
                )
            }
            Operation::UnionIs { value, .. }
            | Operation::UnionPayload { value, .. }
            | Operation::SequenceSize { value }
            | Operation::SequenceFlatten { value } => {
                self.frame(Frame::Unary { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *value,
                        scope,
                    },
                    resources,
                )
            }
            Operation::Sequence { elements } => {
                self.frame(
                    Frame::Construct {
                        expression,
                        values: self.values.len(),
                    },
                    resources,
                )?;
                for position in (0..elements.len()).rev() {
                    self.frame(
                        Frame::SequenceElement {
                            expression,
                            position,
                            scope,
                        },
                        resources,
                    )?;
                }
                Ok(())
            }
            Operation::Record { .. } | Operation::Tuple { .. } | Operation::Union { .. } => {
                let operands = super::construction::operands(program, expression)
                    .ok_or_else(|| broken(site, Broken::Address))?;
                self.frame(
                    Frame::Construct {
                        expression,
                        values: self.values.len(),
                    },
                    resources,
                )?;
                for (position, child) in operands.iter().enumerate().rev() {
                    self.frame(
                        Frame::ConstructCheck {
                            expression,
                            position,
                        },
                        resources,
                    )?;
                    self.frame(
                        Frame::Evaluate {
                            expression: *child,
                            scope,
                        },
                        resources,
                    )?;
                }
                Ok(())
            }
            Operation::IntegerAdd { left, right }
            | Operation::IntegerSubtract { left, right }
            | Operation::IntegerMultiply { left, right }
            | Operation::IntegerOrder { left, right, .. } => {
                self.frame(Frame::Scalar { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *right,
                        scope,
                    },
                    resources,
                )?;
                self.frame(
                    Frame::Evaluate {
                        expression: *left,
                        scope,
                    },
                    resources,
                )
            }
            Operation::IntegerNegate { value } => {
                self.frame(Frame::Scalar { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *value,
                        scope,
                    },
                    resources,
                )
            }
            Operation::BooleanNot { value } => {
                self.frame(Frame::BooleanLeft { expression, scope }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *value,
                        scope,
                    },
                    resources,
                )
            }
            Operation::BooleanAnd { left, .. } | Operation::BooleanOr { left, .. } => {
                self.frame(Frame::BooleanLeft { expression, scope }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *left,
                        scope,
                    },
                    resources,
                )
            }
            Operation::If { condition, .. } => {
                self.frame(Frame::Choose { expression, scope }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *condition,
                        scope,
                    },
                    resources,
                )
            }
            Operation::Call { arguments, .. } => {
                self.frame(
                    Frame::Apply {
                        expression,
                        values: self.values.len(),
                    },
                    resources,
                )?;
                for argument in arguments.iter().rev() {
                    self.frame(
                        Frame::Evaluate {
                            expression: *argument,
                            scope,
                        },
                        resources,
                    )?;
                }
                Ok(())
            }
        }
    }

    fn construct_check(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        position: usize,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let equal = self
            .checks
            .expressions
            .get(expression.0)
            .and_then(|edges| edges.get(position))
            .copied()
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !equal {
            let operand = self
                .values
                .last()
                .copied()
                .ok_or_else(|| broken(site, Broken::ValueStack))?;
            let expected = super::construction::expected(program, expression, position)
                .ok_or_else(|| broken(site, Broken::Address))?;
            let child = super::construction::operands(program, expression)
                .and_then(|values| values.get(position))
                .copied()
                .ok_or_else(|| broken(site, Broken::Address))?;
            self.admit(
                program,
                supplied,
                operand,
                expected,
                Site::Expression(child),
                resources,
            )?;
        }
        Ok(())
    }

    fn construct(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        base: usize,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let operands = super::construction::operands(program, expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let count = operands.len();
        if base.checked_add(count) != Some(self.values.len()) {
            return Err(broken(site, Broken::Arity));
        }
        if matches!(
            program.expression(expression).map(|node| &node.operation),
            Some(Operation::Sequence { .. } | Operation::Union { .. })
        ) {
            return self.construct_values(program, supplied, expression, base, count, resources);
        }
        let live = self
            .values
            .len()
            .checked_add(self.arguments.len())
            .and_then(|n| n.checked_add(self.active_bindings))
            .and_then(|n| n.checked_add(self.active_folds))
            .and_then(|n| n.checked_add(self.projections.len()))
            .and_then(|n| n.checked_add(self.computed.len()))
            .ok_or_else(|| StepResourceStop::storage(count))?;
        resources.live_elements.set(live);
        let bytes = count
            .checked_mul(core::mem::size_of::<FieldValue>())
            .ok_or_else(|| StepResourceStop::storage(count))?;
        for request in [
            StorageRequest::new(count, StorageUnit::AdditionalElements),
            StorageRequest::new(bytes, StorageUnit::Bytes),
        ] {
            if !resources.permit(request) {
                return Err(Failure::Stop(
                    resources
                        .denied
                        .take()
                        .unwrap_or_else(|| StepResourceStop::storage(request.amount())),
                ));
            }
        }
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(count)
            .map_err(|_| StepResourceStop::storage(count))?;
        for operand in self
            .values
            .get(base..)
            .ok_or_else(|| broken(site, Broken::ValueStack))?
        {
            resources.charge()?;
            let value = resolve(
                program,
                supplied,
                &self.projections,
                &self.computed,
                *operand,
                site,
            )?;
            slots.push(FieldValue::Present(value.clone()));
        }
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let ValueType::Composite(key) = &node.value_type else {
            return Err(broken(site, Broken::Projection));
        };
        // Native type facts or bounded membership checked every slot before materialization.
        let value = quire_exact::from_admitted_slots(key.clone(), slots.into_boxed_slice());
        self.values.truncate(base);
        let outcome = quire_exact::retain_composite(value, &mut resources.scalar);
        self.computed_result(outcome, site, resources)
    }

    fn computed_result(
        &mut self,
        outcome: Outcome<Value>,
        site: Site,
        resources: &Resources,
    ) -> Result<(), Failure> {
        match outcome {
            Outcome::Completed(Value::Boolean(value)) => {
                self.value(Operand::BooleanConstant(value), resources)
            }
            Outcome::Completed(value) => {
                self.slot(resources)?;
                let index = self.computed.len();
                self.computed.push(value);
                self.value(Operand::Computed(index), resources)
            }
            outcome @ (Outcome::Incomplete(_) | Outcome::Refused(_) | Outcome::Undefined(_)) => {
                Err(Failure::Scalar {
                    site,
                    outcome: Box::new(outcome),
                })
            }
        }
    }

    fn scalar(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let right = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let unary = matches!(node.operation, Operation::IntegerNegate { .. });
        let left = if unary {
            right
        } else {
            self.values
                .pop()
                .ok_or_else(|| broken(site, Broken::ValueStack))?
        };
        let left = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            left,
            site,
        )?;
        let right = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            right,
            site,
        )?;
        let (Value::Integer(left), Value::Integer(right)) = (left, right) else {
            return Err(broken(site, Broken::ValueStack));
        };
        let bound = match &node.value_type {
            ValueType::Int(bound) => Some(bound),
            _ => None,
        };
        let arithmetic = match node.operation {
            Operation::IntegerAdd { .. } => Some(IntegerArithmetic::Add(left, right)),
            Operation::IntegerSubtract { .. } => Some(IntegerArithmetic::Subtract(left, right)),
            Operation::IntegerMultiply { .. } => Some(IntegerArithmetic::Multiply(left, right)),
            Operation::IntegerNegate { .. } => Some(IntegerArithmetic::Negate(left)),
            _ => None,
        };
        let outcome = if let Some(operation) = arithmetic {
            match quire_exact::evaluate_integer_arithmetic(operation, bound, &mut resources.scalar)
            {
                Outcome::Completed(value) => Outcome::Completed(Value::Integer(value)),
                Outcome::Incomplete(cause) => Outcome::Incomplete(cause),
                Outcome::Refused(cause) => Outcome::Refused(cause),
                Outcome::Undefined(cause) => Outcome::Undefined(cause),
            }
        } else if let Operation::IntegerOrder { operator, .. } = node.operation {
            match quire_exact::order_numbers(
                operator,
                OrderedOperands::Integers(left, right),
                &mut resources.scalar,
            ) {
                Outcome::Completed(value) => Outcome::Completed(Value::Boolean(value)),
                Outcome::Incomplete(cause) => Outcome::Incomplete(cause),
                Outcome::Refused(cause) => Outcome::Refused(cause),
                Outcome::Undefined(cause) => Outcome::Undefined(cause),
            }
        } else {
            return Err(broken(site, Broken::Address));
        };
        self.computed_result(outcome, site, resources)
    }

    fn tuple_element(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let (value, declared_position, member) = match &node.operation {
            Operation::TupleElement { value, position } => (*value, *position, None),
            Operation::RecordField { value, member } => (
                *value,
                program
                    .fields
                    .position(expression)
                    .ok_or_else(|| broken(site, Broken::Projection))?,
                Some(member),
            ),
            _ => return Err(broken(site, Broken::Address)),
        };
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Composite(owner) = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            operand,
            site,
        )?
        else {
            return Err(broken(site, Broken::Projection));
        };
        let receiver = program
            .expression(value)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let ValueType::Composite(expected) = &receiver.value_type else {
            return Err(broken(site, Broken::Projection));
        };
        let exact = owner.declaration() == expected;
        if !exact
            && !program
                .types()
                .composite_conforms(owner.declaration().clone(), expected.clone())
        {
            return Err(broken(site, Broken::Projection));
        }
        let declaration = program
            .types()
            .composite(owner.declaration().clone())
            .ok_or_else(|| broken(site, Broken::Projection))?;
        let (position, actual) = match (declaration.shape(), member) {
            (CompositeShape::Tuple(positions), None) => (
                declared_position,
                positions
                    .get(declared_position)
                    .ok_or_else(|| broken(site, Broken::Projection))?,
            ),
            (CompositeShape::Record(fields), Some(member)) => {
                let position = if exact {
                    declared_position
                } else {
                    let mut found = None;
                    for (position, field) in fields.iter().enumerate() {
                        resources.charge()?;
                        if field.name() == member.member() {
                            found = Some(position);
                            break;
                        }
                    }
                    found.ok_or_else(|| broken(site, Broken::Projection))?
                };
                let field = fields
                    .get(position)
                    .ok_or_else(|| broken(site, Broken::Projection))?;
                (position, field.value_type())
            }
            _ => return Err(broken(site, Broken::Projection)),
        };
        if !matches!(owner.slots().get(position), Some(FieldValue::Present(_))) {
            return Err(broken(site, Broken::Projection));
        }
        let trusted = exact || native_type_fact(program, actual, &node.value_type);
        let owner = Arc::clone(owner);
        self.slot(resources)?;
        let index = self.projections.len();
        self.projections
            .push(Projection::Composite { owner, position });
        let result = Operand::Projection(index);
        if !trusted {
            self.admit(program, supplied, result, &node.value_type, site, resources)?;
        }
        self.value(result, resources)
    }

    fn boolean_left(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Boolean(left) = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            operand,
            site,
        )?
        else {
            return Err(broken(site, Broken::Condition));
        };
        let left = *left;
        match program.expression(expression).map(|node| &node.operation) {
            Some(Operation::BooleanNot { .. }) => self.boolean_result(!left, site, resources),
            Some(Operation::BooleanAnd { .. }) if !left => {
                self.boolean_result(left, site, resources)
            }
            Some(Operation::BooleanOr { .. }) if left => self.boolean_result(left, site, resources),
            Some(Operation::BooleanAnd { right, .. } | Operation::BooleanOr { right, .. }) => {
                self.frame(Frame::BooleanRight { expression }, resources)?;
                self.frame(
                    Frame::Evaluate {
                        expression: *right,
                        scope,
                    },
                    resources,
                )
            }
            _ => Err(broken(site, Broken::Address)),
        }
    }

    fn boolean_right(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Boolean(value) = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            operand,
            site,
        )?
        else {
            return Err(broken(site, Broken::Condition));
        };
        let value = *value;
        self.boolean_result(value, site, resources)
    }

    fn boolean_result(
        &mut self,
        value: bool,
        site: Site,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let outcome = match quire_exact::retain_boolean(value, &mut resources.scalar) {
            Ok(value) => Outcome::Completed(Value::Boolean(value)),
            Err(cause) => Outcome::Incomplete(cause),
        };
        self.computed_result(outcome, site, resources)
    }

    fn read_local(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        binding: LocalId,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        if binding.function != scope.function {
            return Err(broken(site, Broken::Binding));
        }
        let (slot, _) = program
            .bindings
            .resolve(binding)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        let active = self
            .bindings
            .get(slot)
            .copied()
            .flatten()
            .ok_or_else(|| broken(site, Broken::Binding))?;
        if active.invocation != scope.invocation {
            return Err(broken(site, Broken::Binding));
        }
        let equal = self
            .checks
            .expressions
            .get(expression.0)
            .and_then(|edges| edges.first())
            .copied()
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !equal {
            let expected = &program
                .expression(expression)
                .ok_or_else(|| broken(site, Broken::Address))?
                .value_type;
            self.admit(program, supplied, active.operand, expected, site, resources)?;
        }
        self.value(active.operand, resources)
    }

    fn bind(
        &mut self,
        program: &Program,
        expression: ExpressionId,
        scope: Scope,
        resources: &Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let Some(Operation::Let { binding, body, .. }) =
            program.expression(expression).map(|value| &value.operation)
        else {
            return Err(broken(site, Broken::Address));
        };
        if binding.function != scope.function {
            return Err(broken(site, Broken::Binding));
        }
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        self.slot(resources)?;
        let (slot, _) = program
            .bindings
            .resolve(*binding)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        let previous = self.activate(slot, operand, scope.invocation, site)?;
        self.frame(
            Frame::Check {
                expression,
                edge: 0,
            },
            resources,
        )?;
        self.frame(
            Frame::Restore {
                expression,
                slot,
                previous,
                invocation: scope.invocation,
            },
            resources,
        )?;
        self.frame(
            Frame::Evaluate {
                expression: *body,
                scope,
            },
            resources,
        )
    }

    fn activate(
        &mut self,
        slot: usize,
        operand: Operand,
        invocation: u64,
        site: Site,
    ) -> Result<Option<ActiveBinding>, Failure> {
        let entry = self
            .bindings
            .get_mut(slot)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let previous = entry.replace(ActiveBinding {
            invocation,
            operand,
        });
        let touched = self
            .touched_flags
            .get_mut(slot)
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !*touched {
            *touched = true;
            self.touched.push(slot);
        }
        self.active_bindings = self
            .active_bindings
            .checked_add(1)
            .ok_or_else(|| StepResourceStop::storage(1))?;
        Ok(previous)
    }

    fn restore(
        &mut self,
        expression: ExpressionId,
        slot: usize,
        previous: Option<ActiveBinding>,
        invocation: u64,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let entry = self
            .bindings
            .get_mut(slot)
            .ok_or_else(|| broken(site, Broken::Address))?;
        if entry
            .as_ref()
            .is_none_or(|active| active.invocation != invocation)
        {
            return Err(broken(site, Broken::Binding));
        }
        *entry = previous;
        self.active_bindings = self
            .active_bindings
            .checked_sub(1)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        Ok(())
    }

    fn clear_bindings(&mut self) {
        for slot in self.touched.drain(..) {
            if let Some(entry) = self.bindings.get_mut(slot) {
                *entry = None;
            }
            if let Some(flag) = self.touched_flags.get_mut(slot) {
                *flag = false;
            }
        }
        self.active_bindings = 0;
    }

    fn choose(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        scope: Scope,
        resources: &Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Boolean(condition) = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            operand,
            site,
        )?
        else {
            return Err(broken(site, Broken::Condition));
        };
        let Some(Operation::If {
            when_true,
            when_false,
            ..
        }) = program.expression(expression).map(|node| &node.operation)
        else {
            return Err(broken(site, Broken::Address));
        };
        let (branch, edge) = if *condition {
            (*when_true, 0)
        } else {
            (*when_false, 1)
        };
        self.frame(Frame::Check { expression, edge }, resources)?;
        self.frame(
            Frame::Evaluate {
                expression: branch,
                scope,
            },
            resources,
        )
    }

    fn check(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        edge: usize,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let equal = *self
            .checks
            .expressions
            .get(expression.0)
            .and_then(|edges| edges.get(edge))
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !equal {
            let operand = *self
                .values
                .last()
                .ok_or_else(|| broken(site, Broken::ValueStack))?;
            let expected = &program
                .expression(expression)
                .ok_or_else(|| broken(site, Broken::Address))?
                .value_type;
            self.admit(program, supplied, operand, expected, site, resources)?;
        }
        Ok(())
    }

    fn admit(
        &self,
        program: &Program,
        supplied: &[&Value],
        operand: Operand,
        expected: &ValueType,
        site: Site,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let live = self
            .values
            .len()
            .checked_add(self.arguments.len())
            .and_then(|count| count.checked_add(self.active_bindings))
            .and_then(|count| count.checked_add(self.active_folds))
            .and_then(|count| count.checked_add(self.projections.len()))
            .and_then(|count| count.checked_add(self.computed.len()))
            .ok_or_else(|| StepResourceStop::storage(usize::MAX))?;
        resources.live_elements.set(live);
        let value = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            operand,
            site,
        )?;
        admit_value(program.types(), value, expected, site, resources)
    }

    fn apply(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        mark: usize,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let Some(Operation::Call {
            function,
            arguments,
        }) = program.expression(expression).map(|node| &node.operation)
        else {
            return Err(broken(site, Broken::Address));
        };
        let callee = program
            .function(*function)
            .ok_or_else(|| broken(site, Broken::Address))?;
        if self.values.len().checked_sub(mark) != Some(arguments.len())
            || callee.parameters.len() != arguments.len()
        {
            return Err(broken(site, Broken::Arity));
        }
        let start = self.arguments.len();
        for (index, expected) in callee.parameters.iter().enumerate() {
            let at = mark
                .checked_add(index)
                .ok_or_else(|| broken(site, Broken::Address))?;
            let operand = *self
                .values
                .get(at)
                .ok_or_else(|| broken(site, Broken::ValueStack))?;
            let equal = *self
                .checks
                .expressions
                .get(expression.0)
                .and_then(|edges| edges.get(index))
                .ok_or_else(|| broken(site, Broken::Address))?;
            if !equal {
                self.admit(program, supplied, operand, expected, site, resources)?;
            }
            self.slot(resources)?;
            self.arguments.push(operand);
        }
        self.values.truncate(mark);
        self.frame(
            Frame::Check {
                expression,
                edge: arguments.len(),
            },
            resources,
        )?;
        self.frame(
            Frame::Return {
                function: *function,
                arguments: start,
                values: mark,
            },
            resources,
        )?;
        self.frame(
            Frame::Evaluate {
                expression: callee.body,
                scope: Scope {
                    function: *function,
                    start,
                    len: arguments.len(),
                    invocation: resources.work,
                },
            },
            resources,
        )
    }
}

// Only shared native facts discharge a nominal edge; other cases use bounded admission.
fn native_type_fact(program: &Program, actual: &ValueType, expected: &ValueType) -> bool {
    if program.types().same_type(actual, expected) {
        return true;
    }
    match (actual, expected) {
        (ValueType::Composite(actual), ValueType::Composite(expected)) => program
            .types()
            .composite_conforms(actual.clone(), expected.clone()),
        (ValueType::Reference(actual), ValueType::Reference(expected)) => {
            program.types().conforms(actual.clone(), expected.clone())
        }
        _ => false,
    }
}

// Canonical scalar constants require no allocation or semantic identity.
static BOOLEAN_TRUE: Value = Value::Boolean(true);
static BOOLEAN_FALSE: Value = Value::Boolean(false);

fn resolve<'a>(
    program: &'a Program,
    supplied: &[&'a Value],
    projections: &'a [Projection],
    computed: &'a [Value],
    operand: Operand,
    site: Site,
) -> Result<&'a Value, Failure> {
    match operand {
        Operand::Literal(id) => match program.expression(id).map(|node| &node.operation) {
            Some(Operation::Literal(value)) => Ok(value),
            _ => Err(broken(site, Broken::Address)),
        },
        Operand::Computed(index) => computed
            .get(index)
            .ok_or_else(|| broken(site, Broken::Address)),
        Operand::Projection(index) => {
            let place = projections
                .get(index)
                .ok_or_else(|| broken(site, Broken::Address))?;
            place
                .value()
                .ok_or_else(|| broken(site, Broken::Projection))
        }
        Operand::BooleanConstant(value) => Ok(if value { &BOOLEAN_TRUE } else { &BOOLEAN_FALSE }),
        Operand::Supplied(index) => supplied
            .get(index)
            .copied()
            .ok_or_else(|| broken(site, Broken::Address)),
    }
}

fn admit_value(
    types: &quire_semantic_value::declaration::TypeEnvironment,
    value: &Value,
    expected: &ValueType,
    site: Site,
    resources: &mut Resources,
) -> Result<(), Failure> {
    let mut arena = MembershipPathArena::new().map_err(Failure::PathCapacity)?;
    let at = MembershipLocus {
        path: arena.root(0),
        part: MembershipPart::Value,
    };
    let remaining = resources
        .limits
        .work
        .checked_sub(resources.work)
        .ok_or_else(|| broken(site, Broken::ValueStack))?;
    let mut work = SuppliedMembershipBudget::new(remaining);
    let result = types.admit_supplied_with_reservation::<InvalidMembershipCause>(
        expected,
        value,
        &mut arena,
        at,
        &mut work,
        &Cancel::new(),
        &|request| resources.permit(request),
    );
    resources.work = resources
        .work
        .checked_add(work.spent())
        .ok_or_else(|| broken(site, Broken::ValueStack))?;
    let Err(failure) = result else {
        return Ok(());
    };
    let stop = match failure {
        SuppliedAdmissionError::Limit { requested, .. } => Some(StepResourceStop {
            resource: StepResource::Work,
            limit: Some(u128::from(resources.limits.work)),
            spent: u128::from(resources.work),
            requested: Integer::from(requested),
        }),
        SuppliedAdmissionError::Allocation { request, .. }
        | SuppliedAdmissionError::Capacity { request, .. } => Some(
            resources
                .denied
                .take()
                .unwrap_or_else(|| StepResourceStop::storage(request.amount())),
        ),
        _ => None,
    };
    let native = Box::new(MembershipFailure {
        site,
        failure,
        arena,
    });
    match stop {
        Some(stop) => Err(Failure::MembershipStop { stop, native }),
        None => Err(Failure::Membership(native)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_semantic_value::declaration::{
        CompositeDeclaration, ObjectTypeDeclaration, TypeEnvironment,
    };

    fn resources() -> Resources {
        Resources::new(StepBudget {
            work: 100,
            scalar: quire_exact::ScalarLimits {
                integer_bits: 100,
                decimal_digits: 100,
                scale_expansion: 100,
                text_input_bytes: 100,
                text_scalars: 100,
                normalized_scalars: 100,
                unit_edges: 100,
                value_occurrences: 100,
                work_units: 100,
                result_units: 100,
            },
            stack_frames: 10,
            scratch_elements: 100,
            scratch_bytes: 100_000,
        })
    }

    fn check(resources: &mut Resources) -> Result<(), Failure> {
        let types = TypeEnvironment::new(
            core::iter::empty::<CompositeDeclaration>(),
            core::iter::empty::<ObjectTypeDeclaration>(),
        )
        .unwrap();
        admit_value(
            &types,
            &quire_exact::OptionValue::from_admitted(
                ValueType::Boolean,
                Some(Value::Boolean(true)),
            ),
            &ValueType::option(ValueType::Boolean),
            Site::Function(FunctionId(0)),
            resources,
        )
    }

    #[trace("interface-003-AC-3")]
    #[test]
    fn native_membership_work_stop_keeps_its_event_and_arena() {
        let mut resources = resources();
        resources.work = 100;
        match check(&mut resources) {
            Err(Failure::MembershipStop { stop, native }) => {
                assert_eq!(stop.resource, StepResource::Work);
                assert_eq!((stop.spent, stop.requested), (100, Integer::one()));
                let SuppliedAdmissionError::Limit {
                    ceiling,
                    spent,
                    requested,
                    locus,
                    ..
                } = native.failure
                else {
                    panic!("native work refusal");
                };
                assert_eq!((ceiling, spent, requested), (0, 0, 1));
                assert!(native.arena.resolve(locus).is_some());
            }
            _ => panic!("membership must stop before its first work event"),
        }
    }

    #[trace("interface-003-AC-3")]
    #[test]
    fn native_membership_element_stop_counts_live_operand_addresses() {
        let mut resources = resources();
        resources.limits.scratch_elements = 1;
        resources.live_elements.set(1);
        match check(&mut resources) {
            Err(Failure::MembershipStop { stop, native }) => {
                assert_eq!(stop.resource, StepResource::ScratchElements);
                assert_eq!(
                    (stop.limit, stop.spent, stop.requested),
                    (Some(1), 1, Integer::one())
                );
                let SuppliedAdmissionError::Allocation { request, locus } = native.failure else {
                    panic!("native allocation refusal");
                };
                assert_eq!(
                    request,
                    StorageRequest::new(1, StorageUnit::AdditionalElements)
                );
                assert!(native.arena.resolve(locus).is_some());
            }
            _ => panic!("native reservation must include the still-live operand slot"),
        }
    }

    #[trace("interface-003-AC-3")]
    #[test]
    fn byte_reservation_stop_preserves_measured_units_and_cumulative_use() {
        let mut resources = resources();
        let amount = core::mem::size_of::<usize>();
        resources.limits.scratch_bytes = amount;
        let request = StorageRequest::new(amount, StorageUnit::Bytes);
        assert!(resources.permit(request));
        assert!(!resources.permit(request));
        let stop = resources
            .denied
            .take()
            .expect("second measured request exceeds the byte ceiling");
        assert_eq!(stop.resource, StepResource::ScratchBytes);
        assert_eq!(
            (stop.limit, stop.spent, stop.requested),
            (Some(amount as u128), amount as u128, Integer::from(amount))
        );
        assert_eq!(
            resources.bytes.get(),
            amount,
            "denied bytes are not committed"
        );
        assert_eq!(
            resources.elements.get(),
            0,
            "bytes do not consume element units"
        );
    }
}
