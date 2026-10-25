// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Direct public-kernel admission control for the resolved exact dependency.

#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_exact::{Integer, NonZeroInteger, ZeroDivisor};

/// A zero divisor is refused, while one admitted token works for all three helpers.
#[trace("TC-206", "NFR-002-AC-10")]
#[test]
fn public_integer_division_requires_nonzero_token() {
    let zero = Integer::zero();
    assert_eq!(NonZeroInteger::try_from(&zero), Err(ZeroDivisor));

    let divisor = Integer::from(2_i64);
    let token = NonZeroInteger::try_from(&divisor).expect("two is nonzero");
    assert_eq!(Integer::from(6_i64).exact_div(token), Integer::from(3_i64));
    assert_eq!(
        Integer::from(7_i64).div_rem_truncating(token),
        (Integer::from(3_i64), Integer::from(1_i64))
    );
    assert_eq!(
        Integer::from(-7_i64).div_mod_floor(token),
        (Integer::from(-4_i64), Integer::from(1_i64))
    );
}
