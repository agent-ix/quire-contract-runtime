// SPDX-License-Identifier: AGPL-3.0-or-later
//! Verified native union access and original checked equality execution.

use super::{
    broken, resolve, Broken, ExpressionId, Failure, Machine, Operand, Operation, Outcome, Program,
    Projection, Resources, Site, Value,
};
use alloc::sync::Arc;

impl Machine {
    pub(super) fn equality(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let right = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let left = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
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
        let Some(Operation::Equality { checked, .. }) =
            program.expression(expression).map(|node| &node.operation)
        else {
            return Err(broken(site, Broken::Address));
        };
        let outcome = match checked.evaluate(program.types(), left, right, &mut resources.scalar) {
            Outcome::Completed(value) => Outcome::Completed(Value::Boolean(value)),
            Outcome::Incomplete(cause) => Outcome::Incomplete(cause),
            Outcome::Refused(cause) => Outcome::Refused(cause),
            Outcome::Undefined(cause) => Outcome::Undefined(cause),
        };
        self.computed_result(outcome, site, resources)
    }

    pub(super) fn unary(
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
        if matches!(
            node.operation,
            Operation::SequenceSize { .. } | Operation::SequenceFlatten { .. }
        ) {
            return self.sequence_query(program, supplied, expression, resources);
        }
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Union(owner) = resolve(
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
        match &node.operation {
            Operation::UnionIs { variant, .. } => {
                // Selecting a union arm has no native scalar retention charge (FR-378).
                let value = owner.member().variant() == variant;
                self.value(Operand::BooleanConstant(value), resources)
            }
            Operation::UnionPayload {
                variant, position, ..
            } => {
                if owner.member().variant() != variant || owner.payload().get(*position).is_none() {
                    return Err(broken(site, Broken::Projection));
                }
                let owner = Arc::clone(owner);
                self.slot(resources)?;
                let index = self.projections.len();
                self.projections.push(Projection::Union {
                    owner,
                    position: *position,
                });
                self.value(Operand::Projection(index), resources)
            }
            _ => Err(broken(site, Broken::Address)),
        }
    }
}
