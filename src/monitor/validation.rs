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

#[cfg(test)]
mod tests {
    use alloc::{collections::BTreeMap, vec};

    use ix_trace_rs::trace;
    use quire_exact::{
        CollectionKind, CollectionType, MemberId, MemberOwner, PathSegment, Rational, Value,
    };
    use quire_semantic_value::{
        declaration::{
            CompositeDeclaration, FieldDeclaration, ObjectTypeDeclaration, TypeEnvironment,
        },
        quantity::UnitTable,
        unit::{DimensionNode, NominalDeclaration, UnitGraph, UnitNode},
    };

    use super::*;
    use crate::monitor::{Entries, Function, Layout, LocalId, Program, TypeBinding};

    fn key(byte: u8) -> NodeKey {
        NodeKey::new(
            "test:rt-monitor",
            vec![PathSegment::Text(alloc::format!("node_{byte}"))],
            0,
        )
        .unwrap()
    }

    fn member(owner: &NodeKey, name: &str) -> MemberId {
        MemberId::new(MemberOwner::Node(owner.clone()), name).unwrap()
    }

    fn expression(byte: u8, value_type: ValueType, operation: Operation) -> Expression {
        Expression {
            key: key(byte),
            body_path: vec![],
            value_type,
            operation,
        }
    }

    fn parts() -> ProgramParts {
        let state = key(10);
        let inputs = key(11);
        let scope = key(12);
        let step = key(13);
        let time = key(20);
        let tick = key(21);
        let sequence = ValueType::collection(CollectionType::new(
            CollectionKind::Sequence,
            ValueType::Boolean,
            None,
        ));
        let state_type = ValueType::Composite(state.clone());
        let inputs_type = ValueType::Composite(inputs.clone());
        let scope_type = ValueType::Composite(scope.clone());
        let step_type = ValueType::Composite(step.clone());
        let fields = vec![
            FieldDeclaration::new("now", ValueType::Integer, Presence::Required),
            FieldDeclaration::new("history_start", ValueType::Integer, Presence::Required),
            FieldDeclaration::new("activations", sequence.clone(), Presence::Required),
            FieldDeclaration::new("event_position", ValueType::Integer, Presence::Required),
        ];
        let dimensions = BTreeMap::from([(
            time.clone(),
            DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
        )]);
        let units = BTreeMap::from([(
            tick.clone(),
            UnitNode::checked(
                time.clone(),
                None,
                Rational::from_integer(Integer::one()),
                Rational::from_integer(Integer::zero()),
                NominalDeclaration::default(),
            )
            .unwrap(),
        )]);
        let graph = UnitGraph::from_checked_nodes(&dimensions, &units).unwrap();
        let types = TypeEnvironment::new(
            [
                CompositeDeclaration::new(state.clone(), "State", CompositeShape::Record(fields)),
                CompositeDeclaration::new(inputs.clone(), "Inputs", CompositeShape::Record(vec![])),
                CompositeDeclaration::new(scope.clone(), "Scope", CompositeShape::Record(vec![])),
                CompositeDeclaration::new(
                    step.clone(),
                    "Step",
                    CompositeShape::Tuple(vec![state_type.clone(), sequence.clone()]),
                ),
            ],
            core::iter::empty::<ObjectTypeDeclaration>(),
        )
        .unwrap()
        .with_units(UnitTable::declared(&graph));
        let state_fields = |now| {
            vec![
                (member(&state, "now"), now),
                (member(&state, "history_start"), ExpressionId(0)),
                (member(&state, "activations"), ExpressionId(1)),
                (member(&state, "event_position"), ExpressionId(0)),
            ]
        };
        let expressions = vec![
            expression(
                30,
                ValueType::Integer,
                Operation::Literal(Value::Integer(Integer::zero())),
            ),
            expression(
                31,
                sequence.clone(),
                Operation::Sequence { elements: vec![] },
            ),
            expression(
                32,
                state_type.clone(),
                Operation::Record {
                    fields: state_fields(ExpressionId(0)),
                },
            ),
            expression(
                40,
                state_type.clone(),
                Operation::Argument {
                    function: FunctionId(1),
                    position: 0,
                },
            ),
            expression(
                41,
                ValueType::Integer,
                Operation::Argument {
                    function: FunctionId(1),
                    position: 1,
                },
            ),
            expression(
                42,
                ValueType::Integer,
                Operation::RecordField {
                    value: ExpressionId(3),
                    member: member(&state, "now"),
                },
            ),
            expression(
                43,
                ValueType::Integer,
                Operation::IntegerAdd {
                    left: ExpressionId(5),
                    right: ExpressionId(4),
                },
            ),
            expression(
                44,
                state_type.clone(),
                Operation::Record {
                    fields: state_fields(ExpressionId(6)),
                },
            ),
            expression(
                45,
                sequence.clone(),
                Operation::Sequence { elements: vec![] },
            ),
            expression(
                46,
                step_type.clone(),
                Operation::Tuple {
                    positions: vec![ExpressionId(7), ExpressionId(8)],
                },
            ),
            expression(
                40,
                state_type.clone(),
                Operation::Argument {
                    function: FunctionId(3),
                    position: 0,
                },
            ),
            expression(
                41,
                ValueType::Integer,
                Operation::Argument {
                    function: FunctionId(3),
                    position: 1,
                },
            ),
            expression(
                47,
                ValueType::Integer,
                Operation::RecordField {
                    value: ExpressionId(10),
                    member: member(&state, "now"),
                },
            ),
            expression(
                43,
                ValueType::Integer,
                Operation::IntegerAdd {
                    left: ExpressionId(12),
                    right: ExpressionId(11),
                },
            ),
            expression(
                48,
                state_type.clone(),
                Operation::Record {
                    fields: state_fields(ExpressionId(13)),
                },
            ),
            expression(
                49,
                step_type.clone(),
                Operation::Tuple {
                    positions: vec![ExpressionId(14), ExpressionId(8)],
                },
            ),
            expression(
                50,
                state_type.clone(),
                Operation::Argument {
                    function: FunctionId(2),
                    position: 0,
                },
            ),
            expression(
                51,
                step_type.clone(),
                Operation::Tuple {
                    positions: vec![ExpressionId(16), ExpressionId(8)],
                },
            ),
        ];
        ProgramParts {
            layout: Layout {
                root: key(1),
                clause: key(2),
                package_lock: vec![1].into_boxed_slice(),
                subject: "subject".into(),
                occurrence: vec![1].into_boxed_slice(),
                profile: "profile".into(),
                clock_binding: vec![1].into_boxed_slice(),
                admitted_order: vec![1].into_boxed_slice(),
                progress_context: vec![1].into_boxed_slice(),
                state: TypeBinding {
                    key: state.clone(),
                    value_type: state_type.clone(),
                },
                inputs: TypeBinding {
                    key: inputs.clone(),
                    value_type: inputs_type.clone(),
                },
                scope: TypeBinding {
                    key: scope.clone(),
                    value_type: scope_type.clone(),
                },
                results: TypeBinding {
                    key: key(14),
                    value_type: sequence.clone(),
                },
                step_result: TypeBinding {
                    key: step.clone(),
                    value_type: step_type.clone(),
                },
                entries: Entries {
                    init: FunctionId(0),
                    on_event: FunctionId(1),
                    on_watermark: FunctionId(2),
                    on_gap: FunctionId(3),
                },
                counter: "counter".into(),
                tick_unit: tick,
                time_dimension: time,
                counter_width_bits: 8,
                max_reading_gap: 10,
                max_event_rate: Rational::from_integer(Integer::one()),
                uncertainty: Rational::from_integer(Integer::zero()),
                buffers: vec![],
                activation_capacity: Integer::zero(),
            },
            types,
            functions: vec![
                Function {
                    key: key(80),
                    parameters: vec![],
                    result: state_type.clone(),
                    body: ExpressionId(2),
                },
                Function {
                    key: key(81),
                    parameters: vec![state_type.clone(), ValueType::Integer, inputs_type],
                    result: step_type.clone(),
                    body: ExpressionId(9),
                },
                Function {
                    key: key(82),
                    parameters: vec![state_type.clone(), ValueType::Integer, scope_type],
                    result: step_type.clone(),
                    body: ExpressionId(17),
                },
                Function {
                    key: key(83),
                    parameters: vec![state_type, ValueType::Integer],
                    result: step_type,
                    body: ExpressionId(15),
                },
            ],
            expressions,
        }
    }

    fn refused(parts: ProgramParts, code: Code, node: NodeKey) {
        match Program::try_new(parts) {
            Err(failure) => {
                assert_eq!(failure.code, code);
                assert_eq!(failure.node, Some(node));
            }
            Ok(_) => panic!("malformed source location was admitted"),
        }
    }

    #[trace("TC-209", "interface-002-AC-5")]
    #[test]
    fn shared_source_nodes_keep_original_locations_in_distinct_functions() {
        let program = Program::try_new(parts()).unwrap();
        for (left, right) in [(3, 10), (4, 11), (6, 13)] {
            let first = program.expression(ExpressionId(left)).unwrap();
            let second = program.expression(ExpressionId(right)).unwrap();
            assert_eq!(first.key, second.key);
            assert_eq!(first.body_path, second.body_path);
        }
        assert_eq!(program.function(FunctionId(1)).unwrap().key, key(81));
        assert_eq!(program.function(FunctionId(3)).unwrap().key, key(83));
        assert!(matches!(
            program.expression(ExpressionId(6)).unwrap().operation,
            Operation::IntegerAdd {
                left: ExpressionId(5),
                right: ExpressionId(4)
            }
        ));
        assert!(matches!(
            program.expression(ExpressionId(13)).unwrap().operation,
            Operation::IntegerAdd {
                left: ExpressionId(12),
                right: ExpressionId(11)
            }
        ));
    }

    #[trace("TC-209", "interface-002-AC-6")]
    #[test]
    fn source_location_copies_without_distinct_single_owners_refuse() {
        let mut same = parts();
        same.expressions.push(expression(
            43,
            ValueType::Integer,
            Operation::IntegerAdd {
                left: ExpressionId(5),
                right: ExpressionId(4),
            },
        ));
        refused(same, Code::DuplicateKey, key(43));

        let mut neutral = parts();
        neutral.expressions.push(expression(
            30,
            ValueType::Integer,
            Operation::Literal(Value::Integer(Integer::one())),
        ));
        refused(neutral, Code::DuplicateKey, key(30));

        let mut neutral_and_bound = parts();
        neutral_and_bound.expressions[4].key = key(30);
        refused(neutral_and_bound, Code::DuplicateKey, key(30));

        let mut mixed = parts();
        mixed.expressions.push(expression(
            43,
            ValueType::Integer,
            Operation::IntegerAdd {
                left: ExpressionId(4),
                right: ExpressionId(11),
            },
        ));
        refused(mixed, Code::DuplicateKey, key(43));
    }

    #[trace("TC-209", "interface-002-AC-6", "interface-002-AC-3")]
    #[test]
    fn foreign_argument_and_local_keep_their_typed_refusals() {
        let mut argument = parts();
        argument.expressions[13].key = key(60);
        argument.expressions[13].operation = Operation::IntegerAdd {
            left: ExpressionId(5),
            right: ExpressionId(11),
        };
        refused(argument, Code::ForeignArgument, key(83));

        let mut local = parts();
        local.expressions.push(expression(
            61,
            ValueType::Integer,
            Operation::Local(LocalId {
                function: FunctionId(1),
                position: 0,
            }),
        ));
        local.expressions[13].key = key(60);
        local.expressions[13].operation = Operation::IntegerAdd {
            left: ExpressionId(18),
            right: ExpressionId(11),
        };
        refused(local, Code::InvalidBinding, key(83));
    }
}
