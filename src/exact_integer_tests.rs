// SPDX-License-Identifier: AGPL-3.0-or-later
//! In-crate `Integer` representation tests for behaviour no public operator
//! exercises directly: `abs` and the private big-form constructor are
//! `pub(crate)`, reachable only from inside the crate.

use core::str::FromStr as _;

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
    let expected = Integer::from_str("9223372036854775808").unwrap();
    assert_eq!(magnitude, expected);
    assert_ne!(magnitude, value);
    assert!(!magnitude.is_negative());
}
