//! Sizes are derived before values are materialized: an arithmetic charge denied
//! at its limit allocates nothing the size of the result, and `digits(x)` never
//! renders `x`.
//!
//! A counting global allocator records, per thread, the largest single request
//! made while a measurement window is open. The runtime itself stays
//! `forbid(unsafe_code)`; the allocator lives only in this test binary.
#![cfg(feature = "exact")]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use quire_contract_runtime::exact::{
    evaluate_integer, evaluate_rational, ChargePoint, Incomplete, Integer, IntegerDomain,
    IntegerOperation, LimitKind, Meter, Outcome, Rational, RationalOperation, ScalarLimits,
};

thread_local! {
    static MEASURING: Cell<bool> = const { Cell::new(false) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
}

struct PeakRequest;

fn record(size: usize) {
    // `try_with` so a request during thread teardown is simply not recorded.
    let _ = MEASURING.try_with(|measuring| {
        if measuring.get() {
            let _ = PEAK.try_with(|peak| peak.set(peak.get().max(size)));
        }
    });
}

// SAFETY: every method forwards to `System` with the caller's arguments
// unchanged, so `System`'s contract is this allocator's contract; recording
// touches only const-initialized thread-locals, which never allocate.
unsafe impl GlobalAlloc for PeakRequest {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        // SAFETY: forwarded unchanged from our caller, who upholds `alloc`'s contract.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` was returned by `System` through this allocator with `layout`.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record(new_size);
        // SAFETY: forwarded unchanged; `ptr` and `layout` came from `System`.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: PeakRequest = PeakRequest;

/// The largest single allocation request `run` makes on this thread.
fn peak_request<T>(run: impl FnOnce() -> T) -> (T, usize) {
    PEAK.with(|peak| peak.set(0));
    MEASURING.with(|measuring| measuring.set(true));
    let value = run();
    MEASURING.with(|measuring| measuring.set(false));
    (value, PEAK.with(Cell::get))
}

const UNLIMITED: ScalarLimits = ScalarLimits {
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

fn bits_limit(bits: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: bits,
        ..UNLIMITED
    }
}

/// `3^(2^16)`: about 104 thousand bits, far from any power of two.
fn large() -> Integer {
    let mut value = Integer::from(3_u64);
    for _ in 0..16 {
        value = evaluate_integer(
            IntegerOperation::Multiply(&value, &value),
            &IntegerDomain::Mathematical,
            &mut Meter::new(UNLIMITED),
        )
        .completed()
        .unwrap();
    }
    value
}

fn byte_len(value: &Integer) -> usize {
    usize::try_from(value.magnitude_bits().div_ceil(8)).unwrap()
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_arithmetic_denied_at_the_operand_bits_allocates_no_result() {
    let x = large();
    let operand_bits = x.magnitude_bits();
    let mut unlimited = Meter::new(UNLIMITED);
    let square = evaluate_integer(
        IntegerOperation::Multiply(&x, &x),
        &IntegerDomain::Mathematical,
        &mut unlimited,
    )
    .completed()
    .unwrap();
    let result_bits = square.magnitude_bits();
    assert!(result_bits > operand_bits);
    assert_eq!(unlimited.consumed(LimitKind::IntegerBits), result_bits);

    let mut meter = Meter::new(bits_limit(operand_bits));
    let (outcome, peak) = peak_request(|| {
        evaluate_integer(
            IntegerOperation::Multiply(&x, &x),
            &IntegerDomain::Mathematical,
            &mut meter,
        )
    });
    assert_eq!(
        outcome,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            limit: operand_bits,
            consumed: operand_bits,
            next_charge: Integer::from(result_bits),
            charge_point: ChargePoint::IntegerArithmeticArithmetic,
        })
    );
    // A materialized square would need `2 × byte_len(x)` bytes in one request.
    assert!(
        peak < byte_len(&x) / 8,
        "peak request {peak} bytes for a {} byte operand",
        byte_len(&x)
    );

    // Rational `x/1 + x/2`: `N = 3x`, `D = 2`, sized the same way.
    let (left, right) = (
        Rational::from_integer(x.clone()),
        Rational::new(x.clone(), Integer::from(2_u64)).unwrap(),
    );
    let mut meter = Meter::new(bits_limit(operand_bits));
    let (outcome, peak) =
        peak_request(|| evaluate_rational(RationalOperation::Add(&left, &right), None, &mut meter));
    assert!(matches!(
        outcome,
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::IntegerBits,
            charge_point: ChargePoint::RationalArithmeticArithmetic,
            ..
        })
    ));
    assert!(peak < byte_len(&x) / 8, "peak request {peak} bytes");
}

/// Trace: TC-016, FR-006-AC-3
#[test]
fn tc_016_decimal_digits_never_render_the_value() {
    let x = large();
    let (digits, peak) = peak_request(|| x.decimal_digits());
    assert_eq!(digits, u64::try_from(x.to_string().len()).unwrap());
    // The rendered digits alone would be about 2.4 × the operand's bytes.
    assert!(
        peak <= byte_len(&x).next_multiple_of(8),
        "peak request {peak} bytes for a {} byte operand",
        byte_len(&x)
    );
}
