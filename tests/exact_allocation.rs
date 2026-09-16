//! Sizes are derived from operands before values are materialized: an
//! arithmetic, division or decimal-retention charge denied at its limit
//! allocates nothing the size of the result, and `digits(x)` never renders `x`.
//!
//! A counting global allocator records, per thread, the largest single request
//! made while a measurement window is open. The runtime itself stays
//! `forbid(unsafe_code)`; the allocator lives only in this test binary.
#![cfg(feature = "exact")]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use quire_contract_runtime::exact::{
    divide, evaluate_decimal, evaluate_integer, evaluate_rational, modulo, ChargePoint, Decimal,
    DecimalOperation, DecimalType, DivisionProfile, Incomplete, InjectedDenial, Integer,
    IntegerDomain, IntegerOperation, LimitKind, Meter, Outcome, Rational, RationalOperation,
    RoundingMode, ScalarLimits,
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

fn integer(operation: IntegerOperation<'_>) -> Integer {
    evaluate_integer(
        operation,
        &IntegerDomain::Mathematical,
        &mut Meter::new(UNLIMITED),
    )
    .completed()
    .unwrap()
}

/// `2^(2^16)`: exactly 65,537 bits.
fn power_of_two() -> Integer {
    let mut value = Integer::from(2_u64);
    for _ in 0..16 {
        value = integer(IntegerOperation::Multiply(&value, &value));
    }
    value
}

fn bits_denied(limit: u64, consumed: u64, next: u64, point: ChargePoint) -> Outcome<()> {
    Outcome::Incomplete(Incomplete {
        limit_kind: LimitKind::IntegerBits,
        limit,
        consumed,
        next_charge: Integer::from(next),
        charge_point: point,
    })
}

/// `outcome` with its completed value dropped, for comparing stops.
fn stop<T>(outcome: Outcome<T>) -> Outcome<()> {
    match outcome {
        Outcome::Completed(_) => Outcome::Completed(()),
        Outcome::Undefined(reason) => Outcome::Undefined(reason),
        Outcome::Refused(reason) => Outcome::Refused(reason),
        Outcome::Incomplete(record) => Outcome::Incomplete(record),
    }
}

/// A request below an eighth of the operand's bytes: no result-sized buffer.
fn assert_no_result_allocation(peak: usize, operand: &Integer) {
    assert!(
        peak < byte_len(operand) / 8,
        "peak request {peak} bytes for a {} byte operand",
        byte_len(operand)
    );
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
    // `bits(a) + bits(b)` bounds the square, which is never measured.
    assert!(square.magnitude_bits() <= 2 * operand_bits);
    assert_eq!(unlimited.consumed(LimitKind::IntegerBits), 2 * operand_bits);

    // Denied at the operand bits and one under the amount: nothing result-sized.
    for limit in [operand_bits, 2 * operand_bits - 1] {
        let mut meter = Meter::new(bits_limit(limit));
        let (outcome, peak) = peak_request(|| {
            evaluate_integer(
                IntegerOperation::Multiply(&x, &x),
                &IntegerDomain::Mathematical,
                &mut meter,
            )
        });
        assert_eq!(
            stop(outcome),
            bits_denied(
                limit,
                operand_bits,
                2 * operand_bits,
                ChargePoint::IntegerArithmeticArithmetic
            )
        );
        assert_no_result_allocation(peak, &x);
    }

    // Rational `x/1 + x/2`: `N = max(b + 2, b + 1) + 1`, `D = 1 + 2`.
    let (left, right) = (
        Rational::from_integer(x.clone()),
        Rational::new(x.clone(), Integer::from(2_u64)).unwrap(),
    );
    let mut meter = Meter::new(bits_limit(operand_bits + 2));
    let (outcome, peak) =
        peak_request(|| evaluate_rational(RationalOperation::Add(&left, &right), None, &mut meter));
    assert_eq!(
        stop(outcome),
        bits_denied(
            operand_bits + 2,
            operand_bits,
            operand_bits + 3,
            ChargePoint::RationalArithmeticArithmetic
        )
    );
    assert_no_result_allocation(peak, &x);
}

/// Trace: TC-023, FR-007-AC-7, FR-006-AC-3
#[test]
fn tc_023_power_of_two_products_and_cancellation_deny_before_any_result() {
    let two_k = power_of_two();
    let k = two_k.magnitude_bits() - 1;
    let one = Integer::one();
    let below = integer(IntegerOperation::Subtract(&two_k, &one));
    let above = integer(IntegerOperation::Add(&two_k, &one));

    // `(2^k - 1)(2^k + 1) = 2^2k - 1` has `2k` bits but charges `k + (k + 1)`:
    // a limit equal to the result's size is still denied, before the product.
    let product = integer(IntegerOperation::Multiply(&below, &above));
    assert_eq!(product.magnitude_bits(), 2 * k);
    let mut meter = Meter::new(bits_limit(2 * k));
    let (outcome, peak) = peak_request(|| {
        evaluate_integer(
            IntegerOperation::Multiply(&below, &above),
            &IntegerDomain::Mathematical,
            &mut meter,
        )
    });
    assert_eq!(
        stop(outcome),
        bits_denied(
            2 * k,
            k + 1,
            2 * k + 1,
            ChargePoint::IntegerArithmeticArithmetic
        )
    );
    assert_no_result_allocation(peak, &two_k);

    // Cancellation: `(2^k + 1) - 2^k = 1` still charges `max(k+1, k+1) + 1`.
    let mut meter = Meter::new(bits_limit(k + 1));
    let (outcome, peak) = peak_request(|| {
        evaluate_integer(
            IntegerOperation::Subtract(&above, &two_k),
            &IntegerDomain::Mathematical,
            &mut meter,
        )
    });
    assert_eq!(
        stop(outcome),
        bits_denied(
            k + 1,
            k + 1,
            k + 2,
            ChargePoint::IntegerArithmeticArithmetic
        )
    );
    assert_no_result_allocation(peak, &two_k);

    // Rational cancellation `(2^k/1) × (1/2^k) = 1`: `N = D = (k + 1) + 1`,
    // denied before the unreduced `2^k/2^k` is formed.
    let (whole, inverse) = (
        Rational::from_integer(two_k.clone()),
        Rational::new(one.clone(), two_k.clone()).unwrap(),
    );
    let mut meter = Meter::new(bits_limit(k + 1));
    let (outcome, peak) = peak_request(|| {
        evaluate_rational(
            RationalOperation::Multiply(&whole, &inverse),
            None,
            &mut meter,
        )
    });
    assert_eq!(
        stop(outcome),
        bits_denied(
            k + 1,
            k + 1,
            k + 2,
            ChargePoint::RationalArithmeticArithmetic
        )
    );
    assert_no_result_allocation(peak, &two_k);
}

/// Trace: TC-018, FR-007-AC-1
#[test]
fn tc_018_division_arithmetic_is_charged_before_the_quotient_exists() {
    let x = large();
    let three = Integer::from(3_u64);
    let bits = x.magnitude_bits();
    // `max(bits(a), bits(b))`, at the operands and again at the arithmetic.
    let mut meter = Meter::new(bits_limit(bits));
    assert!(divide(
        DivisionProfile::Floor,
        &x,
        &three,
        &IntegerDomain::Mathematical,
        &mut meter
    )
    .completed()
    .is_some());
    assert_eq!(meter.consumed(LimitKind::IntegerBits), bits);
    assert_eq!(
        stop(divide(
            DivisionProfile::Floor,
            &x,
            &three,
            &IntegerDomain::Mathematical,
            &mut Meter::new(bits_limit(bits - 1))
        )),
        bits_denied(bits - 1, 0, bits, ChargePoint::IntegerDivisionOperands)
    );

    // A denied arithmetic charge computes no quotient or remainder.
    let deny_arithmetic = |point| {
        Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: 1,
        })
    };
    let mut meter = deny_arithmetic(ChargePoint::IntegerDivisionArithmetic);
    let (outcome, peak) = peak_request(|| {
        divide(
            DivisionProfile::Truncating,
            &x,
            &three,
            &IntegerDomain::Mathematical,
            &mut meter,
        )
    });
    assert!(matches!(outcome, Outcome::Incomplete(_)));
    assert_no_result_allocation(peak, &x);
    let mut meter = deny_arithmetic(ChargePoint::IntegerModulusArithmetic);
    let (outcome, peak) =
        peak_request(|| modulo(&x, &three, &IntegerDomain::Mathematical, &mut meter));
    assert!(matches!(outcome, Outcome::Incomplete(_)));
    assert_no_result_allocation(peak, &x);
}

/// Trace: TC-019, FR-007-AC-2
#[test]
fn tc_019_decimal_upscale_is_denied_before_the_power_of_ten() {
    // `(1,0) × (1,0)` retained at scale 2^20 would need `10^1048576`, about
    // 435 KiB; `decimal_digits = sdigits(1, 2^20)` is denied first.
    let scale = 1_u64 << 20;
    let target = DecimalType::new(
        Integer::zero(),
        Integer::one(),
        0,
        scale,
        RoundingMode::Exact,
    )
    .unwrap();
    let one = Decimal::new(Integer::one(), 0);
    let mut meter = Meter::new(ScalarLimits {
        decimal_digits: scale,
        ..UNLIMITED
    });
    let (outcome, peak) = peak_request(|| {
        evaluate_decimal(DecimalOperation::Multiply(&one, &one), &target, &mut meter)
    });
    assert_eq!(
        stop(outcome),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::DecimalDigits,
            limit: scale,
            consumed: 2,
            next_charge: Integer::from(scale + 1),
            charge_point: ChargePoint::DecimalResultRetain,
        })
    );
    assert!(peak < 4096, "peak request {peak} bytes");
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
