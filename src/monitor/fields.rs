// SPDX-License-Identifier: AGPL-3.0-or-later
//! Resolve only requested record declarations, retaining private slot addresses.

use alloc::vec::Vec;
use quire_exact::{MemberOwner, Presence};
use quire_semantic_value::declaration::CompositeShape;

use super::{ExpressionId, Operation, ProgramParts, ProgramRefusal, ProgramRefusalCode as Code};

pub(super) struct Fields {
    positions: Vec<Option<usize>>,
}

fn refusal(parts: &ProgramParts, index: usize, code: Code) -> ProgramRefusal {
    ProgramRefusal {
        code,
        node: parts.expressions.get(index).map(|node| node.key.clone()),
        membership_failure: None,
    }
}

impl Fields {
    pub(super) fn prepare(parts: &ProgramParts) -> Result<Self, ProgramRefusal> {
        let mut positions = Vec::new();
        let mut requests = Vec::new();
        positions
            .try_reserve_exact(parts.expressions.len())
            .map_err(|_| refusal(parts, 0, Code::Storage))?;
        positions.resize(parts.expressions.len(), None);
        requests
            .try_reserve_exact(parts.expressions.len())
            .map_err(|_| refusal(parts, 0, Code::Storage))?;
        for (index, node) in parts.expressions.iter().enumerate() {
            if let Operation::RecordField { member, .. } = &node.operation {
                requests.push((member, index));
            }
        }
        requests.sort_unstable_by(|left, right| left.0.cmp(right.0));
        let mut remaining = requests.as_slice();
        while let Some((first, index)) = remaining.first() {
            let end = remaining.partition_point(|(member, _)| member.owner() == first.owner());
            let (group, rest) = remaining.split_at(end);
            let MemberOwner::Node(owner) = first.owner() else {
                return Err(refusal(parts, *index, Code::InvalidReference));
            };
            let declaration = parts
                .types
                .composite(owner.clone())
                .ok_or_else(|| refusal(parts, *index, Code::TypeMismatch))?;
            let CompositeShape::Record(fields) = declaration.shape() else {
                return Err(refusal(parts, *index, Code::TypeMismatch));
            };
            let mut ordered = Vec::new();
            ordered
                .try_reserve_exact(fields.len())
                .map_err(|_| refusal(parts, *index, Code::Storage))?;
            ordered.extend(fields.iter().enumerate());
            ordered.sort_unstable_by(|left, right| left.1.name().cmp(right.1.name()));
            for (member, index) in group {
                let found = ordered
                    .binary_search_by(|(_, field)| field.name().cmp(member.member()))
                    .map_err(|_| refusal(parts, *index, Code::InvalidReference))?;
                let (position, field) = ordered
                    .get(found)
                    .ok_or_else(|| refusal(parts, *index, Code::InvalidReference))?;
                let node = parts
                    .expressions
                    .get(*index)
                    .ok_or_else(|| refusal(parts, *index, Code::InvalidReference))?;
                if field.presence() != Presence::Required
                    || !parts.types.same_type(field.value_type(), &node.value_type)
                {
                    return Err(refusal(parts, *index, Code::TypeMismatch));
                }
                let slot = positions
                    .get_mut(*index)
                    .ok_or_else(|| refusal(parts, *index, Code::InvalidReference))?;
                *slot = Some(*position);
            }
            remaining = rest;
        }
        Ok(Self { positions })
    }

    pub(super) fn position(&self, expression: ExpressionId) -> Option<usize> {
        self.positions.get(expression.0).copied().flatten()
    }
}
