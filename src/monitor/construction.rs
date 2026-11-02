// SPDX-License-Identifier: AGPL-3.0-or-later
//! Declaration-ordered operand addresses over the single retained program.

use alloc::vec::Vec;
use quire_exact::{MemberOwner, Presence, ValueType};
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
                Operation::Record { .. } | Operation::Tuple { .. }
            ) {
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
        Operation::Tuple { positions } => Some(positions),
        _ => None,
    }
}

pub(super) fn expected(
    program: &Program,
    expression: ExpressionId,
    position: usize,
) -> Option<&ValueType> {
    let node = program.expression(expression)?;
    let ValueType::Composite(key) = &node.value_type else {
        return None;
    };
    match program.types().composite(key.clone())?.shape() {
        CompositeShape::Record(fields) => fields.get(position).map(|field| field.value_type()),
        CompositeShape::Tuple(positions) => positions.get(position),
        CompositeShape::Union(_) => None,
    }
}
