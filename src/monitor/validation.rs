// SPDX-License-Identifier: AGPL-3.0-or-later
//! Executable integrity checks; source-language admission remains upstream.

use alloc::{collections::BTreeSet, vec::Vec};

use quire_exact::{Integer, NodeKey, ValueType};

use super::program::{
    Expression, ExpressionId, FunctionId, Operation, ProgramParts, ProgramRefusal,
    ProgramRefusalCode as Code,
};

fn refusal(code: Code, node: Option<&NodeKey>) -> ProgramRefusal {
    ProgramRefusal {
        code,
        node: node.cloned(),
    }
}

pub(super) fn validate(parts: &ProgramParts) -> Result<(), ProgramRefusal> {
    validate_layout(parts)?;
    let mut functions = BTreeSet::new();
    for function in &parts.functions {
        if !functions.insert(&function.key) {
            return Err(refusal(Code::DuplicateKey, Some(&function.key)));
        }
        for value_type in function.parameters.iter().chain([&function.result]) {
            parts
                .types
                .check_type(value_type)
                .map_err(|_| refusal(Code::TypeMismatch, Some(&function.key)))?;
        }
        expression(parts, function.body, &function.key)?;
    }
    let mut expressions = BTreeSet::new();
    for value in &parts.expressions {
        if !expressions.insert((&value.key, value.body_path.as_slice())) {
            return Err(refusal(Code::DuplicateKey, Some(&value.key)));
        }
        parts
            .types
            .check_type(&value.value_type)
            .map_err(|_| refusal(Code::TypeMismatch, Some(&value.key)))?;
        match &value.operation {
            Operation::Literal(literal) => {
                if !parts
                    .types
                    .admits(&value.value_type, literal)
                    .map_err(|_| refusal(Code::Storage, Some(&value.key)))?
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
    for (index, function) in parts.functions.iter().enumerate() {
        match owners.get(function.body.0) {
            Some(ArgumentOwner::None) => {}
            Some(ArgumentOwner::One(owner)) if *owner == FunctionId(index) => {}
            _ => return Err(refusal(Code::ForeignArgument, Some(&function.key))),
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
        || layout.counter.is_empty()
        || !(1..=64).contains(&layout.counter_width_bits)
        || u128::from(layout.max_reading_gap)
            >= 1_u128
                .checked_shl(u32::from(layout.counter_width_bits))
                .ok_or_else(invalid)?
        || layout.tick_scale.numerator() <= &Integer::zero()
        || layout.max_event_rate.numerator() <= &Integer::zero()
        || layout.uncertainty.numerator() < &Integer::zero()
        || layout.buffers.iter().any(|buffer| buffer.capacity == 0)
    {
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
    let mut keys = BTreeSet::new();
    for (entry, parameters, expected_result) in signatures {
        let function = parts.functions.get(entry.0).ok_or_else(invalid)?;
        if !keys.insert(&function.key)
            || function.parameters.len() != parameters.len()
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
enum ArgumentOwner {
    None,
    One(FunctionId),
    Mixed,
}

impl ArgumentOwner {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::None, owner) | (owner, Self::None) => owner,
            (Self::One(left), Self::One(right)) if left == right => Self::One(left),
            _ => Self::Mixed,
        }
    }
}

fn operands(
    value: &Expression,
    mut visit: impl FnMut(ExpressionId) -> Result<(), ProgramRefusal>,
) -> Result<(), ProgramRefusal> {
    match &value.operation {
        Operation::Literal(_) | Operation::Argument { .. } => {}
        Operation::Call { arguments, .. } => {
            for argument in arguments.iter().rev() {
                visit(*argument)?;
            }
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

fn validate_expression_graph(parts: &ProgramParts) -> Result<Vec<ArgumentOwner>, ProgramRefusal> {
    let mut colors = Vec::new();
    colors
        .try_reserve_exact(parts.expressions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    colors.resize(parts.expressions.len(), 0_u8);
    let mut owners = Vec::new();
    owners
        .try_reserve_exact(parts.expressions.len())
        .map_err(|_| refusal(Code::Storage, None))?;
    owners.resize(parts.expressions.len(), ArgumentOwner::None);
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
                    Operation::Argument { function, .. } => ArgumentOwner::One(*function),
                    _ => ArgumentOwner::None,
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
