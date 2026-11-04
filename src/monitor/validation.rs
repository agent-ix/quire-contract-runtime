// SPDX-License-Identifier: AGPL-3.0-or-later
//! Executable integrity checks; source-language admission remains upstream.

use alloc::vec::Vec;

use quire_exact::{CollectionKind, Integer, MemberOwner, NodeKey, Presence, ValueType};
use quire_semantic_value::{declaration::CompositeShape, quantity::QuantityUnit};

use super::program::{
    Expression, ExpressionId, FunctionId, Operation, ProgramParts, ProgramRefusal,
    ProgramRefusalCode as Code,
};

fn refusal(code: Code, node: Option<&NodeKey>) -> ProgramRefusal {
    ProgramRefusal {
        code,
        node: node.cloned(),
        membership_failure: None,
    }
}

pub(super) fn validate(parts: &ProgramParts) -> Result<(), ProgramRefusal> {
    validate_layout(parts)?;
    let mut functions = Vec::new();
    functions
        .try_reserve_exact(parts.functions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    functions.extend(parts.functions.iter().map(|function| &function.key));
    functions.sort_unstable();
    for pair in functions.windows(2) {
        if let [left, right] = pair {
            if left == right {
                return Err(refusal(Code::DuplicateKey, Some(left)));
            }
        }
    }
    for function in &parts.functions {
        for value_type in function.parameters.iter().chain([&function.result]) {
            parts
                .types
                .check_type(value_type)
                .map_err(|_| refusal(Code::TypeMismatch, Some(&function.key)))?;
        }
        expression(parts, function.body, &function.key)?;
    }
    for value in &parts.expressions {
        parts
            .types
            .check_type(&value.value_type)
            .map_err(|_| refusal(Code::TypeMismatch, Some(&value.key)))?;
        match &value.operation {
            Operation::Literal(literal) => {
                if !parts
                    .types
                    .admits(&value.value_type, literal)
                    .map_err(|cause| ProgramRefusal {
                        membership_failure: Some(cause),
                        ..refusal(Code::Membership, Some(&value.key))
                    })?
                {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::Argument { function, position } => {
                let owner = parts
                    .functions
                    .get(function.0)
                    .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                let expected = owner
                    .parameters
                    .get(*position)
                    .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                if !parts.types.same_type(expected, &value.value_type) {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::Local(binding) => {
                if parts.functions.get(binding.function.0).is_none() {
                    return Err(refusal(Code::InvalidBinding, Some(&value.key)));
                }
            }
            Operation::Let {
                binding,
                value: initializer,
                body,
            } => {
                if parts.functions.get(binding.function.0).is_none() {
                    return Err(refusal(Code::InvalidBinding, Some(&value.key)));
                }
                expression(parts, *initializer, &value.key)?;
                expression(parts, *body, &value.key)?;
            }
            Operation::Call {
                function,
                arguments,
            } => {
                let callee = parts
                    .functions
                    .get(function.0)
                    .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                if callee.parameters.len() != arguments.len() {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
                for argument in arguments {
                    expression(parts, *argument, &value.key)?;
                }
            }
            Operation::TupleElement {
                value: receiver,
                position,
            } => {
                let receiver = expression(parts, *receiver, &value.key)?;
                let ValueType::Composite(key) = &receiver.value_type else {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                };
                let declaration = parts
                    .types
                    .composite(key.clone())
                    .ok_or_else(|| refusal(Code::TypeMismatch, Some(&value.key)))?;
                let CompositeShape::Tuple(positions) = declaration.shape() else {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                };
                let expected = positions
                    .get(*position)
                    .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                if !parts.types.same_type(expected, &value.value_type) {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::RecordField {
                value: receiver,
                member,
            } => {
                let receiver = expression(parts, *receiver, &value.key)?;
                let ValueType::Composite(key) = &receiver.value_type else {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                };
                if member.owner() != &MemberOwner::Node(key.clone()) {
                    return Err(refusal(Code::InvalidReference, Some(&value.key)));
                }
                let declaration = parts
                    .types
                    .composite(key.clone())
                    .ok_or_else(|| refusal(Code::TypeMismatch, Some(&value.key)))?;
                if !matches!(declaration.shape(), CompositeShape::Record(_)) {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::SequenceFold {
                sequence,
                initial,
                accumulator,
                item,
                body,
            } => {
                let source = expression(parts, *sequence, &value.key)?;
                if !matches!(&source.value_type, ValueType::Collection(value) if value.kind() == CollectionKind::Sequence)
                    || accumulator == item
                    || accumulator.function != item.function
                    || parts.functions.get(accumulator.function.0).is_none()
                {
                    return Err(refusal(Code::InvalidBinding, Some(&value.key)));
                }
                expression(parts, *initial, &value.key)?;
                expression(parts, *body, &value.key)?;
            }
            Operation::Equality { left, right, .. } => {
                expression(parts, *left, &value.key)?;
                expression(parts, *right, &value.key)?;
                if !parts
                    .types
                    .same_type(&value.value_type, &ValueType::Boolean)
                {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::Union { positions, .. }
            | Operation::Sequence {
                elements: positions,
            } => {
                for child in positions {
                    expression(parts, *child, &value.key)?;
                }
            }
            Operation::UnionIs {
                value: receiver,
                variant,
            }
            | Operation::UnionPayload {
                value: receiver,
                variant,
                ..
            } => {
                let receiver = expression(parts, *receiver, &value.key)?;
                let ValueType::Composite(key) = &receiver.value_type else {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                };
                let (_, member) = parts
                    .types
                    .union_member(key.clone(), variant.clone())
                    .filter(|(_, member)| member.member().is_some())
                    .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                let expected = if let Operation::UnionPayload { position, .. } = value.operation {
                    member
                        .positions()
                        .get(position)
                        .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?
                } else {
                    &ValueType::Boolean
                };
                if !parts.types.same_type(expected, &value.value_type) {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
            }
            Operation::SequenceSize { value: receiver }
            | Operation::SequenceFlatten { value: receiver } => {
                let receiver = expression(parts, *receiver, &value.key)?;
                let ValueType::Collection(collection) = &receiver.value_type else {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                };
                if collection.kind() != CollectionKind::Sequence {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
                if matches!(value.operation, Operation::SequenceSize { .. }) {
                    if !parts
                        .types
                        .same_type(&value.value_type, &ValueType::Integer)
                    {
                        return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                    }
                } else {
                    let (ValueType::Collection(inner), ValueType::Collection(result)) =
                        (collection.element(), &value.value_type)
                    else {
                        return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                    };
                    if inner.kind() != CollectionKind::Sequence
                        || result.kind() != CollectionKind::Sequence
                        || !parts.types.same_type(inner.element(), result.element())
                    {
                        return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                    }
                }
            }
            Operation::Record { fields } => {
                for (_, child) in fields {
                    expression(parts, *child, &value.key)?;
                }
            }
            Operation::Tuple { positions } => {
                for child in positions {
                    expression(parts, *child, &value.key)?;
                }
            }
            Operation::IntegerAdd { left, right }
            | Operation::IntegerSubtract { left, right }
            | Operation::IntegerMultiply { left, right }
            | Operation::IntegerOrder { left, right, .. } => {
                integer_operands(parts, value, &[*left, *right])?;
            }
            Operation::IntegerNegate { value: child } => {
                integer_operands(parts, value, &[*child])?;
            }
            Operation::BooleanNot { value: child } => {
                boolean_operands(parts, value, &[*child])?;
            }
            Operation::BooleanAnd { left, right } | Operation::BooleanOr { left, right } => {
                boolean_operands(parts, value, &[*left, *right])?;
            }
            Operation::If {
                condition,
                when_true,
                when_false,
            } => {
                let condition = expression(parts, *condition, &value.key)?;
                if !parts
                    .types
                    .same_type(&condition.value_type, &ValueType::Boolean)
                {
                    return Err(refusal(Code::TypeMismatch, Some(&value.key)));
                }
                expression(parts, *when_true, &value.key)?;
                expression(parts, *when_false, &value.key)?;
            }
        }
    }
    let owners = validate_expression_graph(parts)?;
    let mut expressions = Vec::new();
    expressions
        .try_reserve_exact(parts.expressions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    expressions.extend(
        parts
            .expressions
            .iter()
            .zip(&owners)
            .map(|(value, owner)| (&value.key, value.body_path.as_slice(), *owner)),
    );
    expressions.sort_unstable_by(|left, right| {
        (left.0, left.1, left.2.function.map(|id| id.0)).cmp(&(
            right.0,
            right.1,
            right.2.function.map(|id| id.0),
        ))
    });
    for pair in expressions.windows(2) {
        if let [left, right] = pair {
            if left.0 == right.0
                && left.1 == right.1
                && (left.2.mixed
                    || right.2.mixed
                    || left.2.function.is_none()
                    || right.2.function.is_none()
                    || left.2.function == right.2.function)
            {
                return Err(refusal(Code::DuplicateKey, Some(left.0)));
            }
        }
    }
    for (index, function) in parts.functions.iter().enumerate() {
        let owner = owners
            .get(function.body.0)
            .ok_or_else(|| refusal(Code::InvalidReference, Some(&function.key)))?;
        if owner.mixed || owner.function.is_some_and(|id| id != FunctionId(index)) {
            return Err(refusal(
                if owner.local {
                    Code::InvalidBinding
                } else {
                    Code::ForeignArgument
                },
                Some(&function.key),
            ));
        }
    }
    Ok(())
}

fn integer_operands(
    parts: &ProgramParts,
    node: &Expression,
    children: &[ExpressionId],
) -> Result<(), ProgramRefusal> {
    let order = matches!(node.operation, Operation::IntegerOrder { .. });
    if if order {
        !parts.types.same_type(&node.value_type, &ValueType::Boolean)
    } else {
        !matches!(node.value_type, ValueType::Integer | ValueType::Int(_))
    } {
        return Err(refusal(Code::TypeMismatch, Some(&node.key)));
    }
    for child in children {
        let child = expression(parts, *child, &node.key)?;
        if !matches!(child.value_type, ValueType::Integer | ValueType::Int(_)) {
            return Err(refusal(Code::TypeMismatch, Some(&node.key)));
        }
    }
    Ok(())
}

fn boolean_operands(
    parts: &ProgramParts,
    node: &Expression,
    arguments: &[ExpressionId],
) -> Result<(), ProgramRefusal> {
    if !parts.types.same_type(&ValueType::Boolean, &node.value_type) {
        return Err(refusal(Code::TypeMismatch, Some(&node.key)));
    }
    for id in arguments {
        let argument = expression(parts, *id, &node.key)?;
        if !parts
            .types
            .same_type(&ValueType::Boolean, &argument.value_type)
        {
            return Err(refusal(Code::TypeMismatch, Some(&node.key)));
        }
    }
    Ok(())
}

fn validate_layout(parts: &ProgramParts) -> Result<(), ProgramRefusal> {
    let layout = &parts.layout;
    let invalid = || refusal(Code::InvalidLayout, Some(&layout.root));
    if layout.package_lock.is_empty()
        || layout.occurrence.is_empty()
        || layout.subject.is_empty()
        || layout.profile.is_empty()
        || layout.clock_binding.is_empty()
        || layout.admitted_order.is_empty()
        || layout.progress_context.is_empty()
        || layout.counter.is_empty()
        || !(1..=64).contains(&layout.counter_width_bits)
        || u128::from(layout.max_reading_gap)
            >= 1_u128
                .checked_shl(u32::from(layout.counter_width_bits))
                .ok_or_else(invalid)?
        || layout.max_event_rate.numerator() <= &Integer::zero()
        || layout.uncertainty.numerator() < &Integer::zero()
        || layout
            .buffers
            .iter()
            .any(|buffer| buffer.capacity <= Integer::zero())
        || layout.activation_capacity < Integer::zero()
    {
        return Err(invalid());
    }
    let unit_id = parts
        .types
        .units()
        .ids()
        .find(|id| id.declared_key() == Some(&layout.tick_unit))
        .ok_or_else(invalid)?;
    let Some(QuantityUnit::Declared(unit)) = parts.types.units().get(unit_id) else {
        return Err(invalid());
    };
    if unit.dimension_node() != layout.time_dimension {
        return Err(invalid());
    }
    for pair in layout.buffers.windows(2) {
        let [left, right] = pair else {
            return Err(invalid());
        };
        if left.owner >= right.owner {
            return Err(invalid());
        }
    }
    for binding in [
        &layout.state,
        &layout.inputs,
        &layout.scope,
        &layout.results,
        &layout.step_result,
    ] {
        parts
            .types
            .check_type(&binding.value_type)
            .map_err(|_| refusal(Code::TypeMismatch, Some(&binding.key)))?;
    }
    for binding in [&layout.state, &layout.inputs, &layout.scope] {
        if binding.value_type != ValueType::Composite(binding.key.clone())
            || !matches!(
                parts
                    .types
                    .composite(binding.key.clone())
                    .map(|declaration| declaration.shape()),
                Some(CompositeShape::Record(_))
            )
        {
            return Err(refusal(Code::TypeMismatch, Some(&binding.key)));
        }
    }
    validate_state(parts)?;
    let ValueType::Collection(results) = &layout.results.value_type else {
        return Err(invalid());
    };
    if results.kind() != CollectionKind::Sequence {
        return Err(invalid());
    }
    let ValueType::Composite(tuple_key) = &layout.step_result.value_type else {
        return Err(invalid());
    };
    if tuple_key != &layout.step_result.key {
        return Err(invalid());
    }
    let Some(CompositeShape::Tuple(positions)) = parts
        .types
        .composite(tuple_key.clone())
        .map(|declaration| declaration.shape())
    else {
        return Err(invalid());
    };
    let [state_position, results_position] = positions.as_slice() else {
        return Err(invalid());
    };
    if !parts
        .types
        .same_type(state_position, &layout.state.value_type)
        || !parts
            .types
            .same_type(results_position, &layout.results.value_type)
    {
        return Err(invalid());
    }
    let state = &layout.state.value_type;
    let result = &layout.step_result.value_type;
    let entries = layout.entries;
    let signatures: [(FunctionId, &[ValueType], &ValueType); 4] = [
        (entries.init, &[], state),
        (
            entries.on_event,
            &[
                state.clone(),
                ValueType::Integer,
                layout.inputs.value_type.clone(),
            ],
            result,
        ),
        (
            entries.on_watermark,
            &[
                state.clone(),
                ValueType::Integer,
                layout.scope.value_type.clone(),
            ],
            result,
        ),
        (entries.on_gap, &[state.clone(), ValueType::Integer], result),
    ];
    for (entry, parameters, expected_result) in signatures {
        let function = parts.functions.get(entry.0).ok_or_else(invalid)?;
        if function.parameters.len() != parameters.len()
            || !function
                .parameters
                .iter()
                .zip(parameters)
                .all(|(actual, expected)| parts.types.same_type(actual, expected))
            || !parts.types.same_type(&function.result, expected_result)
        {
            return Err(refusal(Code::TypeMismatch, Some(&function.key)));
        }
    }
    Ok(())
}

fn validate_state(parts: &ProgramParts) -> Result<(), ProgramRefusal> {
    let state = &parts.layout.state;
    let invalid = || refusal(Code::TypeMismatch, Some(&state.key));
    let Some(CompositeShape::Record(fields)) = parts
        .types
        .composite(state.key.clone())
        .map(|declaration| declaration.shape())
    else {
        return Err(invalid());
    };
    let [now, history_start, buffers @ .., activations, event_position] = fields.as_slice() else {
        return Err(invalid());
    };
    if fields
        .iter()
        .any(|field| field.presence() != Presence::Required)
        || now.name() != "now"
        || history_start.name() != "history_start"
        || activations.name() != "activations"
        || event_position.name() != "event_position"
        || buffers.len() != parts.layout.buffers.len()
        || [now, history_start, event_position].iter().any(|field| {
            !parts
                .types
                .same_type(field.value_type(), &ValueType::Integer)
        })
    {
        return Err(invalid());
    }
    for (index, field) in buffers.iter().enumerate() {
        let Some(suffix) = field.name().strip_prefix("buffer_") else {
            return Err(invalid());
        };
        if suffix.is_empty()
            || !suffix.bytes().all(|byte| byte.is_ascii_digit())
            || (suffix.len() > 1 && suffix.starts_with('0'))
            || suffix.parse::<usize>().ok() != Some(index)
        {
            return Err(invalid());
        }
    }
    for field in buffers.iter().chain([activations]) {
        let ValueType::Collection(collection) = field.value_type() else {
            return Err(invalid());
        };
        if collection.kind() != CollectionKind::Sequence {
            return Err(invalid());
        }
    }
    Ok(())
}

fn expression<'a>(
    parts: &'a ProgramParts,
    id: ExpressionId,
    source: &NodeKey,
) -> Result<&'a Expression, ProgramRefusal> {
    parts
        .expressions
        .get(id.0)
        .ok_or_else(|| refusal(Code::InvalidReference, Some(source)))
}

#[derive(Clone, Copy)]
struct BindingOwner {
    function: Option<FunctionId>,
    mixed: bool,
    local: bool,
}

impl BindingOwner {
    const NONE: Self = Self {
        function: None,
        mixed: false,
        local: false,
    };
    fn one(function: FunctionId, local: bool) -> Self {
        Self {
            function: Some(function),
            mixed: false,
            local,
        }
    }
    fn merge(self, other: Self) -> Self {
        Self {
            function: self.function.or(other.function),
            mixed: self.mixed
                || other.mixed
                || matches!((self.function, other.function), (Some(a), Some(b)) if a != b),
            local: self.local || other.local,
        }
    }
}

fn operands(
    value: &Expression,
    mut visit: impl FnMut(ExpressionId) -> Result<(), ProgramRefusal>,
) -> Result<(), ProgramRefusal> {
    match &value.operation {
        Operation::Literal(_) | Operation::Argument { .. } | Operation::Local(_) => {}
        Operation::SequenceFold {
            sequence,
            initial,
            body,
            ..
        } => {
            visit(*body)?;
            visit(*initial)?;
            visit(*sequence)?;
        }
        Operation::Record { fields } => {
            for (_, child) in fields.iter().rev() {
                visit(*child)?;
            }
        }
        Operation::Tuple { positions }
        | Operation::Union { positions, .. }
        | Operation::Sequence {
            elements: positions,
        } => {
            for child in positions.iter().rev() {
                visit(*child)?;
            }
        }
        Operation::Let { value, body, .. } => {
            visit(*body)?;
            visit(*value)?;
        }
        Operation::Call { arguments, .. } => {
            for argument in arguments.iter().rev() {
                visit(*argument)?;
            }
        }
        Operation::BooleanNot { value }
        | Operation::IntegerNegate { value }
        | Operation::SequenceSize { value }
        | Operation::SequenceFlatten { value }
        | Operation::UnionIs { value, .. }
        | Operation::UnionPayload { value, .. }
        | Operation::TupleElement { value, .. }
        | Operation::RecordField { value, .. } => visit(*value)?,
        Operation::BooleanAnd { left, right }
        | Operation::BooleanOr { left, right }
        | Operation::IntegerAdd { left, right }
        | Operation::IntegerSubtract { left, right }
        | Operation::IntegerMultiply { left, right }
        | Operation::IntegerOrder { left, right, .. }
        | Operation::Equality { left, right, .. } => {
            visit(*right)?;
            visit(*left)?;
        }
        Operation::If {
            condition,
            when_true,
            when_false,
        } => {
            visit(*when_false)?;
            visit(*when_true)?;
            visit(*condition)?;
        }
    }
    Ok(())
}

fn validate_expression_graph(parts: &ProgramParts) -> Result<Vec<BindingOwner>, ProgramRefusal> {
    let mut colors = Vec::new();
    colors
        .try_reserve_exact(parts.expressions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    colors.resize(parts.expressions.len(), 0_u8);
    let mut owners = Vec::new();
    owners
        .try_reserve_exact(parts.expressions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    owners.resize(parts.expressions.len(), BindingOwner::NONE);
    let mut pending = Vec::new();
    for root in 0..parts.expressions.len() {
        if colors.get(root) == Some(&2) {
            continue;
        }
        pending
            .try_reserve(1)
            .map_err(|_| refusal(Code::Storage, None))?;
        pending.push((ExpressionId(root), false));
        while let Some((id, leaving)) = pending.pop() {
            let value = parts
                .expressions
                .get(id.0)
                .ok_or_else(|| refusal(Code::InvalidReference, None))?;
            if leaving {
                let mut binding = match &value.operation {
                    Operation::Argument { function, .. } => BindingOwner::one(*function, false),
                    Operation::SequenceFold {
                        accumulator, item, ..
                    } => BindingOwner::one(accumulator.function, true)
                        .merge(BindingOwner::one(item.function, true)),
                    Operation::Local(binding) | Operation::Let { binding, .. } => {
                        BindingOwner::one(binding.function, true)
                    }
                    _ => BindingOwner::NONE,
                };
                operands(value, |child| {
                    let owner = owners
                        .get(child.0)
                        .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
                    binding = binding.merge(*owner);
                    Ok(())
                })?;
                *owners
                    .get_mut(id.0)
                    .ok_or_else(|| refusal(Code::InvalidReference, None))? = binding;
                *colors
                    .get_mut(id.0)
                    .ok_or_else(|| refusal(Code::InvalidReference, None))? = 2;
                continue;
            }
            let color = colors
                .get_mut(id.0)
                .ok_or_else(|| refusal(Code::InvalidReference, Some(&value.key)))?;
            if *color == 2 {
                continue;
            }
            if *color == 1 {
                return Err(refusal(Code::ExpressionCycle, Some(&value.key)));
            }
            *color = 1;
            pending
                .try_reserve(1)
                .map_err(|_| refusal(Code::Storage, Some(&value.key)))?;
            pending.push((id, true));
            operands(value, |child| {
                pending
                    .try_reserve(1)
                    .map_err(|_| refusal(Code::Storage, Some(&value.key)))?;
                pending.push((child, false));
                Ok(())
            })?;
        }
    }
    Ok(owners)
}
