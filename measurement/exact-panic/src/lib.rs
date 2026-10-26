// SPDX-License-Identifier: AGPL-3.0-or-later
//! Linked no-std exact-profile population for the runtime-owned panic audit.

#![no_std]
extern crate alloc;

use alloc::{boxed::Box, vec};
use core::hint::black_box;
#[cfg(target_os = "none")]
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
    ptr,
};

use quire_contract_runtime::{
    exact::{BodyStep, CheckMode, CheckingLimits, Frame, PackageDeclarations},
    scalar::{
        evaluate_boolean_short_circuit, negotiate_ieee, negotiate_integer_division,
        IeeeBackendCapabilities, IntegerDivisionConsumer, ShortCircuitConnective,
    },
};
use quire_exact::{Meter, Outcome, ScalarLimits, Value};
use quire_semantic_value::object_closure::ObjectClosure;

// The archive is inspected, never executed. A refusing allocator satisfies
// alloc's staticlib link requirement without supplying a target heap.
#[cfg(target_os = "none")]
struct NoAllocation;

// SAFETY: both functions honor GlobalAlloc's contract: allocation returns null
// and deallocation has no allocation to release.
#[cfg(target_os = "none")]
unsafe impl GlobalAlloc for NoAllocation {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[cfg(target_os = "none")]
#[global_allocator]
static ALLOCATOR: NoAllocation = NoAllocation;

const LIMITS: ScalarLimits = ScalarLimits {
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
};

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

/// Retains the exact-profile call graph in the archive.
#[used]
pub static EXACT_PANIC_ENTRY: extern "C" fn(u32) -> u32 = quire_runtime_exact_panic;

/// Calls the runtime-owned scalar families and remaining expression surface.
pub extern "C" fn quire_runtime_exact_panic(input: u32) -> u32 {
    let division = black_box(negotiate_integer_division(black_box(&[
        IntegerDivisionConsumer::Mathematical,
    ])));
    let ieee = black_box(negotiate_ieee(
        black_box(&[]),
        black_box(&IeeeBackendCapabilities::default()),
    ));
    let mut meter = Meter::new(LIMITS);
    let boolean = black_box(evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        input & 1 != 0,
        || Outcome::Completed(input & 2 != 0),
        &mut meter,
    ));
    let package = black_box(PackageDeclarations::default())
        .check(CheckMode::Linked, CheckingLimits::default());
    let evaluation = package.ok().and_then(|package| {
        let expression = package
            .check_expression(
                vec![],
                quire_exact::ValueType::Boolean,
                Box::new(|_| {
                    Box::new(|_: &Frame<'_, '_>, _: Option<Value>| {
                        BodyStep::Return(Outcome::Completed(Value::Boolean(true)))
                    })
                }),
            )
            .ok()?;
        package
            .evaluate(&expression, vec![], &ObjectClosure::default(), &mut meter)
            .ok()
    });
    let boolean_score = u32::from(matches!(boolean, Outcome::Completed(true)));
    let evaluation_score = u32::from(evaluation.is_some());
    input
        .saturating_add(u32::try_from(division.len()).unwrap_or(u32::MAX))
        .saturating_add(u32::try_from(ieee.len()).unwrap_or(u32::MAX))
        .saturating_add(boolean_score)
        .saturating_add(evaluation_score)
}
