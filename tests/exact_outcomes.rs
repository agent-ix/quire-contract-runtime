//! FR-006 typed outcomes and metering of the function call and lazy connective RT still defines.
//!
//! The kernel `Outcome`, `Refusal`, `Meter` and injected-denial tests left with the kernel to the
//! `quire-exact` crate (FR-275; the matrix section "Evidence at the kernel move"). What stays
//! here is `CheckedPackage` and the lazy Boolean connective.
#![cfg(feature = "exact")]

use std::collections::BTreeSet;
use std::num::NonZeroU64;

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    BodyStep, CheckMode, CheckingLimits, Frame, FunctionDeclaration, PackageDeclarations,
};
use quire_contract_runtime::scalar::{evaluate_boolean_short_circuit, ShortCircuitConnective};
use quire_exact::{
    ChargePoint, Incomplete, InjectedDenial, Integer, LimitKind, Meter, Outcome, ScalarLimits,
    Value, ValueType,
};
use quire_semantic_value::declaration::TypeEnvironment as SharedTypeEnvironment;
use quire_semantic_value::object_closure::ObjectClosure;

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

/// Unwrap an [`Outcome`] known to be [`Outcome::Incomplete`]; panics with the
/// disposition otherwise. `T` need not be `Debug`: only the non-`Incomplete`
/// dispositions, which are `Debug` regardless of `T`, are ever formatted.
fn expect_incomplete<T>(outcome: Outcome<T>) -> Incomplete {
    match outcome {
        Outcome::Incomplete(record) => record,
        Outcome::Completed(_) => panic!("expected Outcome::Incomplete, got Completed"),
        Outcome::Undefined(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
        Outcome::Refused(reason) => panic!("expected Outcome::Incomplete, got {reason:?}"),
    }
}

// One driver per admitted charge point family the runtime itself charges, run under `UNLIMITED`
// so every charge before the point under test is genuinely admitted. Each returns the
// `Incomplete` an injected denial at that call's first matching point produces.

/// A checked package of one nullary function, called through
/// `CheckedPackage::call`: the injected denial fires on the `function.call`
/// charge itself, strictly before the body (which would complete instead of
/// denying) ever runs.
fn drive_function_call(meter: &mut Meter) -> Incomplete {
    let package = PackageDeclarations {
        types: SharedTypeEnvironment::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_| {
                Box::new(|_, _| BodyStep::Return(Outcome::Completed(Value::Boolean(true))))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let evaluation = package
        .call("f", Vec::new(), &ObjectClosure::default(), meter)
        .unwrap();
    expect_incomplete(evaluation.outcome)
}

fn drive_boolean_result(meter: &mut Meter) -> Incomplete {
    expect_incomplete(evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        false,
        || Outcome::Completed(true),
        meter,
    ))
}

/// One `(point, driver)` entry of the AC-1 sweep below.
type Driver = fn(&mut Meter) -> Incomplete;

/// An injected denial at occurrence one of the charge point the runtime still owns.
#[trace("TC-031", "FR-010-AC-1")]
#[test]
fn tc_031_injected_denial_at_occurrence_one_names_every_residue_charge_point() {
    let drivers: Vec<(ChargePoint, Driver)> = vec![
        (ChargePoint::FunctionCall, drive_function_call),
        (ChargePoint::BooleanResultRetain, drive_boolean_result),
    ];

    let covered: BTreeSet<ChargePoint> = drivers.iter().map(|(point, _)| *point).collect();
    assert_eq!(
        covered.len(),
        drivers.len(),
        "a charge point is driven twice"
    );

    for (point, drive) in drivers {
        let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
            point,
            occurrence: NonZeroU64::MIN,
        });
        let record = drive(&mut meter);
        assert_eq!(record.limit_kind, LimitKind::WorkUnits, "{point:?}");
        assert_eq!(record.charge_point, point);
        assert_eq!(record.limit, record.consumed, "{point:?}");
        assert_eq!(record.next_charge, int(1), "{point:?}");
        assert_eq!(
            meter.consumed(LimitKind::WorkUnits),
            record.consumed,
            "{point:?}"
        );
        assert_eq!(meter.consumed(LimitKind::ResultUnits), 0, "{point:?}");
        assert_eq!(
            meter.admitted_charges().len(),
            usize::try_from(record.consumed).unwrap(),
            "{point:?}"
        );
        assert!(
            !meter.admitted_charge_points().contains(&point),
            "{point:?}"
        );
    }
}
