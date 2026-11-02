// SPDX-License-Identifier: AGPL-3.0-or-later
//! Native canonical payload formation and sequence-only occurrence traversal.

use super::{
    broken, resolve, Broken, ExpressionId, Failure, Frame, Integer, Machine, Operation, Outcome,
    Program, Resources, Scope, Site, StepResourceStop, StorageRequest, StorageUnit, UnionValue,
    Value, ValueType,
};
use alloc::{boxed::Box, vec::Vec};
use quire_exact::{Charge, ChargePoint, CollectionKind};

impl Machine {
    fn native_charge(
        &self,
        resources: &mut Resources,
        site: Site,
        charge: Charge,
    ) -> Result<(), Failure> {
        resources
            .scalar
            .charge(charge)
            .map_err(|cause| Failure::Scalar {
                site,
                outcome: Box::new(Outcome::Incomplete(cause)),
            })
    }

    pub(super) fn sequence_element(
        &mut self,
        program: &Program,
        expression: ExpressionId,
        position: usize,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let child = super::super::construction::operands(program, expression)
            .and_then(|values| values.get(position))
            .copied()
            .ok_or_else(|| broken(site, Broken::Address))?;
        self.native_charge(resources, site, Charge::new(ChargePoint::CollectionElement))?;
        self.frame(
            Frame::ConstructCheck {
                expression,
                position,
            },
            resources,
        )?;
        self.frame(
            Frame::Evaluate {
                expression: child,
                scope,
            },
            resources,
        )
    }

    fn value_storage(&self, count: usize, resources: &Resources) -> Result<Vec<Value>, Failure> {
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
            .checked_mul(core::mem::size_of::<Value>())
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
        let mut result = Vec::new();
        result
            .try_reserve_exact(count)
            .map_err(|_| StepResourceStop::storage(count))?;
        Ok(result)
    }

    pub(super) fn construct_values(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        base: usize,
        count: usize,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let mut values = self.value_storage(count, resources)?;
        for operand in self
            .values
            .get(base..)
            .ok_or_else(|| broken(site, Broken::ValueStack))?
        {
            resources.charge()?;
            values.push(
                resolve(
                    program,
                    supplied,
                    &self.projections,
                    &self.computed,
                    *operand,
                    site,
                )?
                .clone(),
            );
        }
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let outcome = match (&node.operation, &node.value_type) {
            (Operation::Sequence { .. }, ValueType::Collection(value_type)) => {
                quire_exact::form(value_type, values, &mut resources.scalar)
            }
            (Operation::Union { variant, .. }, ValueType::Composite(key)) => {
                let binding = program
                    .types()
                    .union_member(key.clone(), variant.clone())
                    .and_then(|(_, member)| member.member())
                    .ok_or_else(|| broken(site, Broken::Projection))?;
                quire_exact::retain_composite(
                    UnionValue::from_admitted(binding.clone(), values),
                    &mut resources.scalar,
                )
            }
            _ => return Err(broken(site, Broken::Address)),
        };
        self.values.truncate(base);
        self.computed_result(outcome, site, resources)
    }

    pub(super) fn sequence_query(
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
        let Value::Collection(owner) = resolve(
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
        if owner.collection_type().kind() != CollectionKind::Sequence {
            return Err(broken(site, Broken::Projection));
        }
        let node = program
            .expression(expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        if matches!(node.operation, Operation::SequenceSize { .. }) {
            let count = Integer::from(owner.elements().len());
            self.native_charge(
                resources,
                site,
                Charge::new(ChargePoint::CollectionResultRetain).results(1),
            )?;
            return self.computed_result(
                Outcome::Completed(Value::Integer(count)),
                site,
                resources,
            );
        }
        let ValueType::Collection(result_type) = &node.value_type else {
            return Err(broken(site, Broken::Projection));
        };
        let mut count = 0_usize;
        for value in owner.elements() {
            resources.charge()?;
            let Value::Collection(inner) = value else {
                return Err(broken(site, Broken::Projection));
            };
            if inner.collection_type().kind() != CollectionKind::Sequence {
                return Err(broken(site, Broken::Projection));
            }
            count = count
                .checked_add(inner.elements().len())
                .ok_or_else(|| StepResourceStop::storage(usize::MAX))?;
        }
        let mut values = self.value_storage(count, resources)?;
        for value in owner.elements() {
            self.native_charge(resources, site, Charge::new(ChargePoint::CollectionVisit))?;
            let Value::Collection(inner) = value else {
                return Err(broken(site, Broken::Projection));
            };
            for value in inner.elements() {
                self.native_charge(resources, site, Charge::new(ChargePoint::CollectionVisit))?;
                values.push(value.clone());
            }
        }
        let outcome = quire_exact::form(result_type, values, &mut resources.scalar);
        self.computed_result(outcome, site, resources)
    }
}
