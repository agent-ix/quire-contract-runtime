// SPDX-License-Identifier: AGPL-3.0-or-later
//! Atomic entry execution over the retained canonical state.

use alloc::rc::Rc;
use quire_exact::{FieldValue, Value};

use super::{
    executor::{build_failure, state_within_bounds},
    input, vm, Event, FunctionId, InputRefusal, Monitor, MonitorFailure, MonitorFailureCode,
    MonitorStep, Reading, Watermark,
};

impl Monitor {
    /// Validate an associated reading without executing an entry or moving the baseline.
    pub fn poll(&self, reading: Reading<'_>) -> Result<(), MonitorFailure> {
        self.ready()?;
        input::reading(&self.program, reading)
    }

    /// Execute an independently admitted inclusive progress assertion atomically.
    ///
    /// A resource stop leaves state and progress unchanged and the assertion unconsumed.
    /// Polling and watermark execution never move the event-reading baseline.
    pub fn watermark(&mut self, watermark: Watermark<'_>) -> Result<MonitorStep, MonitorFailure> {
        self.ready()?;
        input::context(&self.program, watermark.context)?;
        if watermark.progress_context != self.program.layout().progress_context.as_ref() {
            return Err(input::refused(InputRefusal::ProgressContext));
        }
        if watermark.assertion.is_empty() {
            return Err(input::refused(InputRefusal::MissingProgress));
        }
        if self
            .progress
            .as_ref()
            .is_some_and(|previous| watermark.inclusive_boundary < previous)
        {
            return Err(input::refused(InputRefusal::ProgressRegression));
        }
        let boundary = Value::Integer(watermark.inclusive_boundary.clone());
        let function = self.program.layout().entries.on_watermark;
        let value = self
            .machine
            .invoke_with_input(
                &self.program,
                function,
                &[&self.state, &boundary, watermark.scope],
                Some((watermark.scope, &self.program.layout().scope.value_type)),
                self.budget,
            )
            .map_err(|failure| self.failed(failure))?;
        let (state, results) = self.step_values(value, function)?;
        self.state = state;
        self.progress = Some(watermark.inclusive_boundary.clone());
        Ok(MonitorStep {
            program: Rc::clone(&self.program),
            results,
        })
    }

    /// Consume one admitted event atomically, using a width-aware modular delta.
    ///
    /// The first event executes on_event with zero delta. A later gap executes
    /// on_gap instead; only a completed entry replaces the reading baseline.
    pub fn event(&mut self, event: Event<'_>) -> Result<MonitorStep, MonitorFailure> {
        self.ready()?;
        input::reading(&self.program, event.reading)?;
        let layout = self.program.layout();
        let delta = match self.baseline {
            None => 0,
            Some(previous) => {
                let difference = event.reading.value.wrapping_sub(previous);
                if layout.counter_width_bits == 64 {
                    difference
                } else {
                    let mask = 1_u64
                        .checked_shl(u32::from(layout.counter_width_bits))
                        .and_then(|width| width.checked_sub(1))
                        .ok_or_else(|| input::refused(InputRefusal::Width))?;
                    difference & mask
                }
            }
        };
        let delta_value = Value::Integer(quire_exact::Integer::from(delta));
        let gap = self.baseline.is_some() && delta > layout.max_reading_gap;
        let function = if gap {
            layout.entries.on_gap
        } else {
            layout.entries.on_event
        };
        // Both routes admit the event captures before any source body executes.
        // The gap entry deliberately receives no event and owns all affected-instance semantics.
        let value = if gap {
            self.machine.invoke_with_input(
                &self.program,
                function,
                &[&self.state, &delta_value],
                Some((event.inputs, &layout.inputs.value_type)),
                self.budget,
            )
        } else {
            self.machine.invoke_with_input(
                &self.program,
                function,
                &[&self.state, &delta_value, event.inputs],
                Some((event.inputs, &layout.inputs.value_type)),
                self.budget,
            )
        }
        .map_err(|failure| self.failed(failure))?;
        let (state, results) = self.step_values(value, function)?;
        self.state = state;
        self.baseline = Some(event.reading.value);
        Ok(MonitorStep {
            program: Rc::clone(&self.program),
            results,
        })
    }

    fn ready(&self) -> Result<(), MonitorFailure> {
        if self.terminated {
            return Err(input::refused(InputRefusal::Terminated));
        }
        Ok(())
    }

    fn failed(&mut self, failure: vm::Failure) -> MonitorFailure {
        let failure = build_failure(&self.program, failure);
        if failure.code == MonitorFailureCode::Invariant {
            self.terminated = true;
        }
        failure
    }

    fn step_values(
        &mut self,
        value: Value,
        function: FunctionId,
    ) -> Result<(Value, Value), MonitorFailure> {
        if let Value::Composite(tuple) = value {
            if let (
                Some(FieldValue::Present(state @ Value::Composite(_))),
                Some(FieldValue::Present(results @ Value::Collection(_))),
            ) = (tuple.slots().first(), tuple.slots().get(1))
            {
                let layout = self.program.layout();
                if !state_within_bounds(&layout.buffers, &layout.activation_capacity, state) {
                    return Err(self.failed(vm::Failure::Invariant {
                        site: vm::Site::Function(function),
                        cause: vm::Broken::StateBound,
                    }));
                }
                return Ok((state.clone(), results.clone()));
            }
        }
        Err(self.failed(vm::Failure::Invariant {
            site: vm::Site::Function(function),
            cause: vm::Broken::ValueStack,
        }))
    }

    /// Last successfully consumed event reading, absent before the first event.
    pub const fn baseline(&self) -> Option<u64> {
        self.baseline
    }
}
