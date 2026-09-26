// SPDX-License-Identifier: AGPL-3.0-or-later
//! In-crate `Integer` representation tests for behaviour no public operator
//! exercises directly: `abs` and the private big-form constructor are
//! `pub(crate)`, reachable only from inside the crate.

use super::Integer;

/// `abs(i64::MIN)` cannot return `i64::MIN` as its own magnitude: `i64::MIN`
/// has no positive `i64` counterpart, so the small path's `checked_abs` fails
/// and the result must promote to `BigInt`.
///
/// Trace: TC-023, FR-007-AC-7
#[test]
fn tc_023_abs_of_i64_min_promotes_to_its_positive_magnitude() {
    let value = Integer::from(i64::MIN);
    let magnitude = value.abs();
    // `2^63`, `i64::MIN`'s exact magnitude, built through the infallible
    // `From<u64>` path rather than a parse that could fail.
    let expected = Integer::from(9_223_372_036_854_775_808_u64);
    assert_eq!(magnitude, expected);
    assert_ne!(magnitude, value);
    assert!(!magnitude.is_negative());
}
