// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Exact, no-std static-library consumer for the TC-206 linked panic check.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    hint::black_box,
    panic::PanicInfo,
};
use quire_contract_runtime::{
    exact::{CheckMode, CheckingLimits, PackageDeclarations},
    scalar::{
        evaluate_boolean_short_circuit, negotiate_ieee, negotiate_integer_division,
        IeeeBackendCapabilities, IeeeItemRequirement, IntegerDivisionConsumer,
        ShortCircuitConnective,
    },
};
use quire_exact::{
    IeeeOperationKind, IeeeWidth, Meter, Outcome, RoundingMode, ScalarLimits, Value, ValueType,
};
use quire_semantic_value::object_closure::ObjectClosure;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(pointer: *mut c_void);
}

struct ProbeAllocator;

// The archive is inspected, not executed; these unresolved C allocator symbols
// supply the allocation boundary required to monomorphize the exact calls.
unsafe impl GlobalAlloc for ProbeAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the C allocator accepts a byte count and returns a suitably
        // aligned pointer for the target's ordinary Rust allocations.
        unsafe { malloc(layout.size()).cast() }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _layout: Layout) {
        // SAFETY: every pointer here originated in malloc above.
        unsafe { free(pointer.cast()) }
    }
}

#[global_allocator]
static ALLOCATOR: ProbeAllocator = ProbeAllocator;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

/// Retain the probe's public entry in the archive.
#[used]
pub static EXACT_PANIC_ENTRY: extern "C" fn(u32) -> u32 = quire_runtime_exact_panic;

/// Exercise each runtime-owned exact call with input-dependent, observable results.
pub extern "C" fn quire_runtime_exact_panic(input: u32) -> u32 {
    let division = black_box(negotiate_integer_division)(black_box(&[
        IntegerDivisionConsumer::Mathematical,
        IntegerDivisionConsumer::Finite(Default::default()),
    ]));
    let ieee = black_box(negotiate_ieee)(
        black_box(&[IeeeItemRequirement {
            width: IeeeWidth::Binary32,
            operation: IeeeOperationKind::Add,
            rounding: RoundingMode::Exact,
            requires_finite_proof: input & 1 == 0,
        }]),
        black_box(&IeeeBackendCapabilities::default()),
    );
    let mut meter = Meter::new(ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    });
    let boolean = black_box(evaluate_boolean_short_circuit)(
        ShortCircuitConnective::And,
        input & 1 == 0,
        || Outcome::Completed(input & 2 == 0),
        &mut meter,
    );
    let package =
        match PackageDeclarations::default().check(CheckMode::Linked, CheckingLimits::default()) {
            Ok(package) => package,
            Err(_) => return 0,
        };
    let expression = match package.check_expression(
        Vec::new(),
        ValueType::Boolean,
        Box::new(move |_, _| Outcome::Completed(Value::Boolean(input & 4 == 0))),
    ) {
        Ok(expression) => expression,
        Err(_) => return 0,
    };
    let expression_result = black_box(&package).evaluate(
        black_box(&expression),
        Vec::new(),
        &ObjectClosure::default(),
        &mut meter,
    );
    black_box(u32::try_from(division.len()).unwrap_or(0))
        .saturating_add(u32::try_from(ieee.len()).unwrap_or(0))
        .saturating_add(u32::from(matches!(boolean, Outcome::Completed(true))))
        .saturating_add(u32::from(expression_result.is_ok()))
}
