// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scoped order-preserving map and filter over canonical sequence owners.

use super::{
    broken, resolve, ActiveBinding, Broken, ExpressionId, Failure, Frame, Machine, Operand,
    Operation, Outcome, Program, Projection, Resources, Scope, Site, StepResourceStop, Value,
    ValueType,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use quire_exact::{Charge, ChargePoint, CollectionKind, CollectionValue};

pub(super) struct MapState {
    expression: ExpressionId,
    scope: Scope,
    source: Arc<CollectionValue>,
    next: usize,
    /// Reserved once for at most one value per source occurrence.
    results: Vec<Value>,
    base: usize,
    previous_item: Option<ActiveBinding>,
    bound: bool,
}

impl Machine {
    pub(super) fn map_start(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let source = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        let Value::Collection(source) = resolve(
            program,
            supplied,
            &self.projections,
            &self.computed,
            source,
            site,
        )?
        else {
            return Err(broken(site, Broken::Projection));
        };
        if source.collection_type().kind() != CollectionKind::Sequence {
            return Err(broken(site, Broken::Projection));
        }
        let source = Arc::clone(source);
        // The suspended operation retains one source owner and one result owner.
        for _ in 0..2 {
            self.slot(resources)?;
            self.active_folds = self
                .active_folds
                .checked_add(1)
                .ok_or_else(|| StepResourceStop::storage(1))?;
        }
        let results = self.value_storage(source.elements().len(), resources)?;
        self.frame(
            Frame::MapNext(MapState {
                expression,
                scope,
                source,
                next: 0,
                results,
                base: self.values.len(),
                previous_item: None,
                bound: false,
            }),
            resources,
        )
    }

    pub(super) fn map_next(
        &mut self,
        program: &Program,
        mut state: MapState,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(state.expression);
        let node = program
            .expression(state.expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        let (item, scoped) = match &node.operation {
            Operation::SequenceMap { item, body, .. } => (*item, *body),
            Operation::SequenceFilter {
                item, predicate, ..
            } => (*item, *predicate),
            _ => return Err(broken(site, Broken::Address)),
        };
        if item.function != state.scope.function {
            return Err(broken(site, Broken::Binding));
        }
        let (item_slot, _) = program
            .bindings
            .resolve(item)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        if state.source.elements().get(state.next).is_none() {
            let ValueType::Collection(result_type) = &node.value_type else {
                return Err(broken(site, Broken::Address));
            };
            if state.bound {
                self.restore(
                    state.expression,
                    item_slot,
                    state.previous_item,
                    state.scope.invocation,
                )?;
            }
            self.active_folds = self
                .active_folds
                .checked_sub(2)
                .ok_or_else(|| broken(site, Broken::ValueStack))?;
            let outcome = quire_exact::form(result_type, state.results, &mut resources.scalar);
            return self.computed_result(outcome, site, resources);
        }
        resources
            .scalar
            .charge(Charge::new(ChargePoint::CollectionVisit))
            .map_err(|cause| Failure::Scalar {
                site,
                outcome: Box::new(Outcome::Incomplete(cause)),
            })?;
        self.slot(resources)?;
        let projection = self.projections.len();
        self.projections.push(Projection::Sequence {
            owner: Arc::clone(&state.source),
            position: state.next,
        });
        let operand = Operand::Projection(projection);
        if state.bound {
            let target = self
                .bindings
                .get_mut(item_slot)
                .ok_or_else(|| broken(site, Broken::Address))?;
            if target
                .as_ref()
                .is_none_or(|binding| binding.invocation != state.scope.invocation)
            {
                return Err(broken(site, Broken::Binding));
            }
            *target = Some(ActiveBinding {
                invocation: state.scope.invocation,
                operand,
            });
        } else {
            self.slot(resources)?;
            state.previous_item =
                self.activate(item_slot, operand, state.scope.invocation, site)?;
            state.bound = true;
        }
        state.next = state
            .next
            .checked_add(1)
            .ok_or_else(|| StepResourceStop::storage(1))?;
        let scope = state.scope;
        self.frame(Frame::MapBody(state), resources)?;
        self.frame(
            Frame::Evaluate {
                expression: scoped,
                scope,
            },
            resources,
        )
    }

    pub(super) fn map_body(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        mut state: MapState,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(state.expression);
        let operand = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        if self.values.len() != state.base {
            return Err(broken(site, Broken::ValueStack));
        }
        let node = program
            .expression(state.expression)
            .ok_or_else(|| broken(site, Broken::Address))?;
        if matches!(node.operation, Operation::SequenceMap { .. }) {
            let equal = self
                .checks
                .expressions
                .get(state.expression.0)
                .and_then(|edges| edges.first())
                .copied()
                .ok_or_else(|| broken(site, Broken::Address))?;
            if !equal {
                let ValueType::Collection(result) = &node.value_type else {
                    return Err(broken(site, Broken::Address));
                };
                self.admit(
                    program,
                    supplied,
                    operand,
                    result.element(),
                    site,
                    resources,
                )?;
            }
            let value = resolve(
                program,
                supplied,
                &self.projections,
                &self.computed,
                operand,
                site,
            )?
            .clone();
            state.results.push(value);
        } else {
            let Value::Boolean(keep) = resolve(
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
            if *keep {
                let position = state
                    .next
                    .checked_sub(1)
                    .ok_or_else(|| broken(site, Broken::Address))?;
                let value = state
                    .source
                    .elements()
                    .get(position)
                    .ok_or_else(|| broken(site, Broken::Address))?
                    .clone();
                state.results.push(value);
            }
        }
        self.frame(Frame::MapNext(state), resources)
    }
}
