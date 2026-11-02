// SPDX-License-Identifier: AGPL-3.0-or-later
//! Compact indexes over the one retained lexical expression arena.

use alloc::vec::Vec;

use super::{ExpressionId, LocalId, Operation, ProgramParts, ProgramRefusal, ProgramRefusalCode};

pub(super) struct Bindings {
    offsets: Vec<usize>,
    values: Vec<Option<ExpressionId>>,
}

fn invalid(parts: &ProgramParts, expression: usize) -> ProgramRefusal {
    ProgramRefusal {
        code: ProgramRefusalCode::InvalidBinding,
        node: parts
            .expressions
            .get(expression)
            .map(|value| value.key.clone()),
        membership_failure: None,
    }
}

fn storage() -> ProgramRefusal {
    ProgramRefusal {
        code: ProgramRefusalCode::Storage,
        node: None,
        membership_failure: None,
    }
}

impl Bindings {
    pub(super) fn prepare(parts: &ProgramParts) -> Result<Self, ProgramRefusal> {
        let mut counts = Vec::new();
        counts
            .try_reserve_exact(parts.functions.len())
            .map_err(|_| storage())?;
        counts.resize(parts.functions.len(), 0_usize);
        for (index, node) in parts.expressions.iter().enumerate() {
            if let Operation::Let { binding, .. } = node.operation {
                let count = counts
                    .get_mut(binding.function.0)
                    .ok_or_else(|| invalid(parts, index))?;
                *count = count.checked_add(1).ok_or_else(storage)?;
            }
        }
        let mut offsets = Vec::new();
        offsets
            .try_reserve_exact(counts.len().checked_add(1).ok_or_else(storage)?)
            .map_err(|_| storage())?;
        offsets.push(0_usize);
        let mut total = 0_usize;
        for count in counts {
            total = total.checked_add(count).ok_or_else(storage)?;
            offsets.push(total);
        }
        let mut values = Vec::new();
        values.try_reserve_exact(total).map_err(|_| storage())?;
        values.resize(total, None);
        let mut result = Self { offsets, values };
        for (index, node) in parts.expressions.iter().enumerate() {
            if let Operation::Let { binding, value, .. } = node.operation {
                let slot = result.slot(binding).ok_or_else(|| invalid(parts, index))?;
                let entry = result
                    .values
                    .get_mut(slot)
                    .ok_or_else(|| invalid(parts, index))?;
                if entry.replace(value).is_some() {
                    return Err(invalid(parts, index));
                }
            }
        }
        for (index, node) in parts.expressions.iter().enumerate() {
            if let Operation::Local(binding) = node.operation {
                if result.resolve(binding).is_none() {
                    return Err(invalid(parts, index));
                }
            }
        }
        Ok(result)
    }

    fn slot(&self, binding: LocalId) -> Option<usize> {
        let start = *self.offsets.get(binding.function.0)?;
        let end = *self.offsets.get(binding.function.0.checked_add(1)?)?;
        let at = start.checked_add(binding.position)?;
        (at < end).then_some(at)
    }

    pub(super) fn resolve(&self, binding: LocalId) -> Option<(usize, ExpressionId)> {
        let slot = self.slot(binding)?;
        Some((slot, self.values.get(slot).copied().flatten()?))
    }

    pub(super) fn len(&self) -> usize {
        self.values.len()
    }
}
