// SPDX-License-Identifier: AGPL-3.0-or-later
//! Declaration-ordered operand addresses over the single retained program.

use alloc::vec::Vec;
use quire_exact::{CollectionKind, MemberOwner, Presence, ValueType};
use quire_semantic_value::declaration::CompositeShape;

use super::{
    ExpressionId, Operation, Program, ProgramParts, ProgramRefusal, ProgramRefusalCode as Code,
};

pub(super) struct Construction {
    records: Vec<Option<Vec<ExpressionId>>>,
}

fn refusal(parts: &ProgramParts, index: usize, code: Code) -> ProgramRefusal {
    ProgramRefusal {
        code,
        node: parts.expressions.get(index).map(|node| node.key.clone()),
        membership_failure: None,
    }
}

impl Construction {
    pub(super) fn prepare(parts: &ProgramParts) -> Result<Self, ProgramRefusal> {
        let mut records = Vec::new();
        records
            .try_reserve_exact(parts.expressions.len())
            .map_err(|_| refusal(parts, 0, Code::Storage))?;
        records.resize_with(parts.expressions.len(), || None);
        for (index, node) in parts.expressions.iter().enumerate() {
            if !matches!(
                node.operation,
                Operation::Record { .. }
                    | Operation::Tuple { .. }
                    | Operation::Union { .. }
                    | Operation::Sequence { .. }
            ) {
                continue;
            }
            if matches!(node.operation, Operation::Sequence { .. }) {
                if !matches!(&node.value_type, ValueType::Collection(value) if value.kind() == CollectionKind::Sequence)
                {
                    return Err(refusal(parts, index, Code::TypeMismatch));
                }
                continue;
            }
            let ValueType::Composite(key) = &node.value_type else {
                return Err(refusal(parts, index, Code::TypeMismatch));
            };
            let declaration = parts
                .types
                .composite(key.clone())
                .ok_or_else(|| refusal(parts, index, Code::TypeMismatch))?;
            match (&node.operation, declaration.shape()) {
                (Operation::Union { variant, positions }, CompositeShape::Union(_)) => {
                    let (_, member) = parts
                        .types
                        .union_member(key.clone(), variant.clone())
                        .filter(|(_, member)| member.member().is_some())
                        .ok_or_else(|| refusal(parts, index, Code::InvalidReference))?;
                    if positions.len() != member.positions().len() {
                        return Err(refusal(parts, index, Code::TypeMismatch));
                    }
                }
                (Operation::Tuple { positions }, CompositeShape::Tuple(types)) => {
                    if positions.len() != types.len() {
                        return Err(refusal(parts, index, Code::TypeMismatch));
                    }
                }
                (Operation::Record { fields }, CompositeShape::Record(declared)) => {
                    if fields.len() != declared.len() {
                        return Err(refusal(parts, index, Code::TypeMismatch));
                    }
                    let mut ordered = Vec::new();
                    ordered
                        .try_reserve_exact(fields.len())
                        .map_err(|_| refusal(parts, index, Code::Storage))?;
                    for (member, child) in fields {
                        let MemberOwner::Node(owner) = member.owner() else {
                            return Err(refusal(parts, index, Code::InvalidReference));
                        };
                        if owner != key {
                            return Err(refusal(parts, index, Code::InvalidReference));
                        }
                        ordered.push((member.member(), *child));
                    }
                    ordered.sort_unstable_by(|left, right| left.0.cmp(right.0));
                    for pair in ordered.windows(2) {
                        if let [left, right] = pair {
                            if left.0 == right.0 {
                                return Err(refusal(parts, index, Code::InvalidReference));
                            }
                        }
                    }
                    let mut operands = Vec::new();
                    operands
                        .try_reserve_exact(declared.len())
                        .map_err(|_| refusal(parts, index, Code::Storage))?;
                    for field in declared {
                        if field.presence() != Presence::Required {
                            return Err(refusal(parts, index, Code::TypeMismatch));
                        }
                        let position = ordered
                            .binary_search_by(|entry| entry.0.cmp(field.name()))
                            .map_err(|_| refusal(parts, index, Code::InvalidReference))?;
                        let (_, child) = ordered
                            .get(position)
                            .ok_or_else(|| refusal(parts, index, Code::InvalidReference))?;
                        operands.push(*child);
                    }
                    let target = records
                        .get_mut(index)
                        .ok_or_else(|| refusal(parts, index, Code::InvalidReference))?;
                    *target = Some(operands);
                }
                _ => return Err(refusal(parts, index, Code::TypeMismatch)),
            }
        }
        Ok(Self { records })
    }
}

pub(super) fn operands(program: &Program, expression: ExpressionId) -> Option<&[ExpressionId]> {
    match &program.expression(expression)?.operation {
        Operation::Record { .. } => program.construction.records.get(expression.0)?.as_deref(),
        Operation::Tuple { positions }
        | Operation::Union { positions, .. }
        | Operation::Sequence {
            elements: positions,
        } => Some(positions),
        _ => None,
    }
}

pub(super) fn expected(
    program: &Program,
    expression: ExpressionId,
    position: usize,
) -> Option<&ValueType> {
    let node = program.expression(expression)?;
    if let Operation::Sequence { elements } = &node.operation {
        elements.get(position)?;
        let ValueType::Collection(value) = &node.value_type else {
            return None;
        };
        return Some(value.element());
    }
    let ValueType::Composite(key) = &node.value_type else {
        return None;
    };
    if let Operation::Union { variant, .. } = &node.operation {
        return program
            .types()
            .union_member(key.clone(), variant.clone())?
            .1
            .positions()
            .get(position);
    }
    match program.types().composite(key.clone())?.shape() {
        CompositeShape::Record(fields) => fields.get(position).map(|field| field.value_type()),
        CompositeShape::Tuple(positions) => positions.get(position),
        CompositeShape::Union(_) => None,
    }
}
