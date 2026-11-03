// SPDX-License-Identifier: AGPL-3.0-or-later
//! Scoped sequence iteration over canonical owners and compact lexical addresses.

use super::{
    broken, resolve, ActiveBinding, Broken, ExpressionId, Failure, Frame, Machine, Operand,
    Operation, Outcome, Program, Projection, Resources, Scope, Site, StepResourceStop, Value,
};
use alloc::{boxed::Box, sync::Arc};
use quire_exact::{Charge, ChargePoint, CollectionKind, CollectionValue, LimitKind};

pub(super) struct FoldState {
    expression: ExpressionId,
    scope: Scope,
    source: Arc<CollectionValue>,
    next: usize,
    accumulator: Operand,
    base: usize,
    previous_accumulator: Option<ActiveBinding>,
    previous_item: Option<ActiveBinding>,
    bound: bool,
}

impl Machine {
    pub(super) fn fold_start(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        expression: ExpressionId,
        scope: Scope,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(expression);
        let accumulator = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
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
        let equal = self
            .checks
            .expressions
            .get(expression.0)
            .and_then(|edges| edges.first())
            .copied()
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !equal {
            let expected = &program
                .expression(expression)
                .ok_or_else(|| broken(site, Broken::Address))?
                .value_type;
            self.admit(program, supplied, accumulator, expected, site, resources)?;
        }
        // The suspended fold retains one source owner and one accumulator address.
        for _ in 0..2 {
            self.slot(resources)?;
            self.active_folds = self
                .active_folds
                .checked_add(1)
                .ok_or_else(|| StepResourceStop::storage(1))?;
        }
        self.frame(
            Frame::FoldNext(FoldState {
                expression,
                scope,
                source,
                next: 0,
                accumulator,
                base: self.values.len(),
                previous_accumulator: None,
                previous_item: None,
                bound: false,
            }),
            resources,
        )
    }

    pub(super) fn fold_next(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        mut state: FoldState,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(state.expression);
        let Some(Operation::SequenceFold {
            accumulator,
            item,
            body,
            ..
        }) = program
            .expression(state.expression)
            .map(|node| &node.operation)
        else {
            return Err(broken(site, Broken::Address));
        };
        if accumulator.function != state.scope.function || item.function != state.scope.function {
            return Err(broken(site, Broken::Binding));
        }
        let (accumulator_slot, _) = program
            .bindings
            .resolve(*accumulator)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        let (item_slot, _) = program
            .bindings
            .resolve(*item)
            .ok_or_else(|| broken(site, Broken::Binding))?;
        if state.source.elements().get(state.next).is_none() {
            let value = resolve(
                program,
                supplied,
                &self.projections,
                &self.computed,
                state.accumulator,
                site,
            )?;
            let occurrence = value.occ();
            resources
                .scalar
                .charge(
                    Charge::new(ChargePoint::CollectionResultRetain)
                        .exact_size(LimitKind::ValueOccurrences, occurrence.clone())
                        .exact_results(occurrence),
                )
                .map_err(|cause| Failure::Scalar {
                    site,
                    outcome: Box::new(Outcome::Incomplete(cause)),
                })?;
            if state.bound {
                self.restore(
                    state.expression,
                    item_slot,
                    state.previous_item,
                    state.scope.invocation,
                )?;
                self.restore(
                    state.expression,
                    accumulator_slot,
                    state.previous_accumulator,
                    state.scope.invocation,
                )?;
            }
            self.active_folds = self
                .active_folds
                .checked_sub(2)
                .ok_or_else(|| broken(site, Broken::ValueStack))?;
            return self.value(state.accumulator, resources);
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
        let item = Operand::Projection(projection);
        if state.bound {
            for (slot, operand) in [(accumulator_slot, state.accumulator), (item_slot, item)] {
                let target = self
                    .bindings
                    .get_mut(slot)
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
            }
        } else {
            self.slot(resources)?;
            state.previous_accumulator = self.activate(
                accumulator_slot,
                state.accumulator,
                state.scope.invocation,
                site,
            )?;
            self.slot(resources)?;
            state.previous_item = self.activate(item_slot, item, state.scope.invocation, site)?;
            state.bound = true;
        }
        state.next = state
            .next
            .checked_add(1)
            .ok_or_else(|| StepResourceStop::storage(1))?;
        let scope = state.scope;
        self.frame(Frame::FoldBody(state), resources)?;
        self.frame(
            Frame::Evaluate {
                expression: *body,
                scope,
            },
            resources,
        )
    }

    pub(super) fn fold_body(
        &mut self,
        program: &Program,
        supplied: &[&Value],
        mut state: FoldState,
        resources: &mut Resources,
    ) -> Result<(), Failure> {
        let site = Site::Expression(state.expression);
        let value = self
            .values
            .pop()
            .ok_or_else(|| broken(site, Broken::ValueStack))?;
        if self.values.len() != state.base {
            return Err(broken(site, Broken::ValueStack));
        }
        let equal = self
            .checks
            .expressions
            .get(state.expression.0)
            .and_then(|edges| edges.get(1))
            .copied()
            .ok_or_else(|| broken(site, Broken::Address))?;
        if !equal {
            let expected = &program
                .expression(state.expression)
                .ok_or_else(|| broken(site, Broken::Address))?
                .value_type;
            self.admit(program, supplied, value, expected, site, resources)?;
        }
        state.accumulator = value;
        self.frame(Frame::FoldNext(state), resources)
    }
}
