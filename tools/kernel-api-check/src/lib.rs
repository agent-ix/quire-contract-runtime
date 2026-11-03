// SPDX-License-Identifier: AGPL-3.0-or-later
//! Direct type boundary controls for the resolved exact kernel.
//!
//! ```compile_fail,E0308
//! use quire_exact::Integer;
//! let one = Integer::from(1_i64);
//! one.exact_div(&one);
//! ```
//!
//! ```compile_fail,E0308
//! use quire_exact::Integer;
//! let one = Integer::from(1_i64);
//! one.div_rem_truncating(&one);
//! ```
//!
//! ```compile_fail,E0308
//! use quire_exact::Integer;
//! let one = Integer::from(1_i64);
//! one.div_mod_floor(&one);
//! ```

//!
//! A real public fault cause is importable from the same resolved carrier:
//!
//! ```
//! use quire_exact::CheckedInvariantCause::UnknownCheckedFunction;
//! let _ = UnknownCheckedFunction;
//! ```
//!
//! The final carrier has no depth-exhaustion fault. Importing the name rather
//! than constructing it makes this independent of any historical payload:
//!
//! ```compile_fail,E0432
//! #[allow(unused_imports)]
//! use quire_exact::CheckedInvariantCause::CallDepthExceeded;
//! ```

#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use quire_exact::{Integer, NonZeroInteger, ZeroDivisor};

    // The positive carrier control and paired negative doctest are executed
    // by the existing MSRV kernel API gate, alongside the runtime cause/fuel tests.
    #[trace("FR-278-AC-1")]
    #[test]
    fn the_resolved_fault_carrier_has_the_public_function_cause() {
        let cause = quire_exact::CheckedInvariantCause::UnknownCheckedFunction;
        assert!(matches!(
            cause,
            quire_exact::CheckedInvariantCause::UnknownCheckedFunction
        ));
    }

    /// Trace: NFR-002-AC-10
    #[test]
    fn admitted_divisor_is_required_by_all_three_helpers() {
        let zero = Integer::zero();
        assert_eq!(NonZeroInteger::try_from(&zero), Err(ZeroDivisor));

        let ten = Integer::from(10_i64);
        let three = Integer::from(3_i64);
        let admitted = NonZeroInteger::try_from(&three).expect("three is nonzero");
        assert_eq!(ten.exact_div(admitted), Integer::from(3_i64));
        assert_eq!(
            ten.div_rem_truncating(admitted),
            (Integer::from(3_i64), Integer::from(1_i64))
        );
        assert_eq!(
            ten.div_mod_floor(admitted),
            (Integer::from(3_i64), Integer::from(1_i64))
        );
    }
}
