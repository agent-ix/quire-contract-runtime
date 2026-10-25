//! FR-011 the determinate `Meter` state at an `Undefined`, `Refused` or
//! `Incomplete` stop for the runtime's stop-carrying short-circuit connective.
//!
//! The cases whose subject is the kernel `Meter` or a kernel scalar operation (the charge log
//! cap, the injected-denial occurrence counter, the scan order over integer, decimal and
//! division counters, IEEE flag order) left with the kernel to `quire-exact` (FR-275).
#![cfg(feature = "exact")]

use std::cell::Cell;

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;

use ix_trace_rs::trace;
use quire_contract_runtime::scalar::{evaluate_boolean_short_circuit, ShortCircuitConnective};
use quire_exact::{
    ChargePoint, Incomplete, Integer, LimitKind, Meter, Outcome, Refusal, ScalarLimits, Undefined,
};

const UNLIMITED: ScalarLimits = limits([u64::MAX; 10]);

const fn limits(v: [u64; 10]) -> ScalarLimits {
    ScalarLimits {
        integer_bits: v[0],
        decimal_digits: v[1],
        scale_expansion: v[2],
        text_input_bytes: v[3],
        text_scalars: v[4],
        normalized_scalars: v[5],
        unit_edges: v[6],
        value_occurrences: v[7],
        work_units: v[8],
        result_units: v[9],
    }
}

fn int(value: i128) -> Integer {
    Integer::from(value)
}

/// `evaluate_boolean_short_circuit` is the exact/metered subsystem's stop-carrying connective
/// (`agent-ix/quire-contract-runtime#27`). Two halves:
///
/// - When `left` alone decides the result (`And` with `left = false`, `Or` with `left = true`,
///   `Implies` with `left = false`), `right` is never called — so a stop it could have produced
///   can never arise — and the decided result charges `boolean.result-retain` exactly once
///   (FR-011-AC-3's half).
/// - Otherwise `right()` runs. If it stops (`Undefined`, `Refused` or `Incomplete`), that stop
///   returns unchanged, with no `boolean.result-retain` charge and no result unit consumed
///   (FR-011-AC-8). If it completes, the combined result charges `boolean.result-retain` exactly
///   once.
#[trace("TC-032", "FR-011-AC-3", "FR-011-AC-8")]
#[test]
fn tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once() {
    let stops = [
        Outcome::Undefined(Undefined::DivisionByZero),
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: quire_exact::CheckedInvariantCause::CallDepthExceeded,
        }),
        Outcome::Incomplete(Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: int(1),
            charge_point: ChargePoint::BooleanResultRetain,
        }),
    ];

    // The right operand decides nothing: it is skipped entirely, and no stop it could have
    // produced can arise.
    let short_circuiting = [
        (ShortCircuitConnective::And, false, false),
        (ShortCircuitConnective::Or, true, true),
        (ShortCircuitConnective::Implies, false, true),
    ];
    for (connective, left, expected) in short_circuiting {
        for stop in &stops {
            let stop = stop.clone();
            let mut meter = Meter::new(UNLIMITED);
            let called = Cell::new(false);
            let outcome = evaluate_boolean_short_circuit(
                connective,
                left,
                || {
                    called.set(true);
                    stop
                },
                &mut meter,
            );
            assert!(!called.get());
            assert_eq!(outcome, Outcome::Completed(expected));
            assert_eq!(
                meter.admitted_charge_points(),
                [ChargePoint::BooleanResultRetain]
            );
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
        }
    }

    // The right operand decides the result: a stop it produces returns unchanged, with no
    // boolean.result-retain charge and no result unit consumed.
    let evaluated = [
        (ShortCircuitConnective::And, true),
        (ShortCircuitConnective::Or, false),
        (ShortCircuitConnective::Implies, true),
    ];
    for (connective, left) in evaluated {
        for stop in &stops {
            let stop = stop.clone();
            let mut meter = Meter::new(UNLIMITED);
            let outcome =
                evaluate_boolean_short_circuit(connective, left, || stop.clone(), &mut meter);
            assert_eq!(outcome, stop);
            assert_eq!(meter.admitted_charge_points(), []);
            assert_eq!(meter.consumed(LimitKind::WorkUnits), 0);
            assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
        }
    }

    // The right operand decides the result and completes: the combined result charges
    // boolean.result-retain exactly once.
    let decided = [
        (ShortCircuitConnective::And, true, true, true),
        (ShortCircuitConnective::And, true, false, false),
        (ShortCircuitConnective::Or, false, true, true),
        (ShortCircuitConnective::Or, false, false, false),
        (ShortCircuitConnective::Implies, true, true, true),
        (ShortCircuitConnective::Implies, true, false, false),
    ];
    for (connective, left, right, expected) in decided {
        let mut meter = Meter::new(UNLIMITED);
        let outcome = evaluate_boolean_short_circuit(
            connective,
            left,
            || Outcome::Completed(right),
            &mut meter,
        );
        assert_eq!(outcome, Outcome::Completed(expected));
        assert_eq!(
            meter.admitted_charge_points(),
            [ChargePoint::BooleanResultRetain]
        );
        assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 1);
    }

    // The runtime-owned lazy connective also retains no result when its own charge is denied,
    // whether the left operand short-circuits or the right operand completes.
    let mut limits = UNLIMITED;
    limits.work_units = 0;
    for (connective, left) in [
        (ShortCircuitConnective::And, false),
        (ShortCircuitConnective::And, true),
    ] {
        let mut meter = Meter::new(limits);
        let outcome = evaluate_boolean_short_circuit(
            connective,
            left,
            || Outcome::Completed(true),
            &mut meter,
        );
        assert_eq!(
            outcome,
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: 0,
                consumed: 0,
                next_charge: int(1),
                charge_point: ChargePoint::BooleanResultRetain,
            })
        );
        assert!(meter.admitted_charges().is_empty());
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0);
    }
}
