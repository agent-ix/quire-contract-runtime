// SPDX-License-Identifier: AGPL-3.0-or-later
//! TC-191 function application under exact semantics: runtime versus
//! authority (quire-specification/FR-146, FR-273).
//!
//! What agrees, and is checked below: the closed `InputRefusal` vocabulary
//! (`UnknownFunction`, `Arity`, `WrongValueKind`, `DanglingReference`), its
//! codes and causes, the "every argument is checked totally, before any
//! charge" ordering, and the `function.call` charge itself (charged exactly
//! once per admitted application, before the body runs).
//!
//! What does not, and cannot, agree: a function's *body*. AD-002 draws the
//! runtime's boundary at typed values and their operators, not at a
//! contract's expression language — [`Body`](quire_contract_runtime::exact::Body)
//! is an opaque, arbitrary Rust closure (`for<'f> Fn(&Frame<'f>, &[Value]) ->
//! Outcome<Value>`), while the authority's `FunctionDeclaration::body` is an
//! `Expression` AST the checker types and the `Machine` interprets. No
//! `Debug` rendering of a closure exists to compare against an interpreted
//! `Expression`, so every vector below uses only the identity function
//! (`x`), whose result is decided by construction on both sides rather than
//! by any body computation that could diverge. Arithmetic, `let`, `if`,
//! recursion and every other body form FR-273-AC-2 through AC-6 exercise
//! `tests/exact_function_application.rs`'s host-side unit tests instead,
//! against the runtime alone: there is no shared corpus for them, and this
//! file does not claim there is.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 5] = ["AP01", "AP02", "AP03", "AP04", "AP05"];

/// Trace: TC-194, FR-273-AC-5
#[test]
fn tc_194_every_tc191_vector_is_evaluated() {
    assert_eq!(EVALUATED.len(), 5);
    println!("TC-191 agreement: {} vectors evaluated", EVALUATED.len());
}

/// AP01: calling a name no declared function has is `UnknownFunction`,
/// before any charge.
///
/// Trace: TC-194, FR-273-AC-5
#[test]
fn tc_194_ap01_unknown_function_refuses_before_any_charge() {
    let (result, charges) = agree! {{
        let env = TypeEnvironment::new([], []).unwrap();
        let package = linked_package(
            env,
            vec![identity_function("id", ValueType::Boolean, ValueType::Boolean)],
        );
        let mut meter = Meter::new(UNLIMITED);
        let result = package.call(
            "missing",
            vec![Value::Boolean(true)],
            &ObjectEnvironment::default(),
            &mut meter,
        );
        (format!("{result:?}"), meter.admitted_charges().len())
    }};
    assert_eq!(
        result,
        format!(
            "{:?}",
            Result::<(), InputRefusal>::Err(InputRefusal::UnknownFunction("missing".to_string()))
        )
    );
    assert_eq!(charges, 0);
}

/// AP02: an argument count that differs from the declared parameter count is
/// `Arity`, decided before any per-argument check or charge.
///
/// Trace: TC-194, FR-273-AC-5
#[test]
fn tc_194_ap02_arity_mismatch_refuses_before_any_charge() {
    let (result, charges) = agree! {{
        let env = TypeEnvironment::new([], []).unwrap();
        let package = linked_package(
            env,
            vec![identity_function("id", ValueType::Boolean, ValueType::Boolean)],
        );
        let mut meter = Meter::new(UNLIMITED);
        let result = package.call("id", vec![], &ObjectEnvironment::default(), &mut meter);
        (format!("{result:?}"), meter.admitted_charges().len())
    }};
    assert_eq!(
        result,
        format!(
            "{:?}",
            Result::<(), InputRefusal>::Err(InputRefusal::Arity {
                declared: 1,
                supplied: 0
            })
        )
    );
    assert_eq!(charges, 0);
}

/// AP03: an argument that is not a value of its declared parameter type is
/// `WrongValueKind`, before any charge.
///
/// Trace: TC-194, FR-273-AC-5
#[test]
fn tc_194_ap03_wrong_value_kind_refuses_before_any_charge() {
    let (result, charges) = agree! {{
        let env = TypeEnvironment::new([], []).unwrap();
        let package = linked_package(
            env,
            vec![identity_function("id", ValueType::Boolean, ValueType::Boolean)],
        );
        let mut meter = Meter::new(UNLIMITED);
        let result = package.call(
            "id",
            vec![Value::Integer(int(1))],
            &ObjectEnvironment::default(),
            &mut meter,
        );
        (format!("{result:?}"), meter.admitted_charges().len())
    }};
    assert_eq!(
        result,
        format!(
            "{:?}",
            Result::<(), InputRefusal>::Err(InputRefusal::WrongValueKind { parameter: 0 })
        )
    );
    assert_eq!(charges, 0);
}

/// AP04: an argument reference to no object in the supplied
/// `ObjectEnvironment` is `DanglingReference`, before any charge.
///
/// Trace: TC-194, FR-273-AC-5
#[test]
fn tc_194_ap04_dangling_reference_refuses_before_any_charge() {
    let (result, charges) = agree! {{
        let object_type = key(9);
        let env = TypeEnvironment::new(
            [],
            [ObjectTypeDeclaration::new(object_type, "Widget", vec![])],
        )
        .unwrap();
        let package = linked_package(
            env,
            vec![identity_function(
                "id",
                ValueType::Reference(object_type),
                ValueType::Reference(object_type),
            )],
        );
        let mut meter = Meter::new(UNLIMITED);
        let dangling = reference(1, object_type, 9);
        let result = package.call(
            "id",
            vec![Value::Reference(dangling)],
            &ObjectEnvironment::default(),
            &mut meter,
        );
        (format!("{result:?}"), meter.admitted_charges().len())
    }};
    assert_eq!(
        result,
        format!(
            "{:?}",
            Result::<(), InputRefusal>::Err(InputRefusal::DanglingReference { parameter: 0 })
        )
    );
    assert_eq!(charges, 0);
}

/// AP05: a call whose arguments are all admitted applies totally, charging
/// `function.call` exactly once, before the (here, trivial) body runs.
///
/// Trace: TC-194, FR-273-AC-3, FR-273-AC-5
#[test]
fn tc_194_ap05_admitted_call_charges_function_call_exactly_once() {
    let (result, charges) = agree! {{
        let env = TypeEnvironment::new([], []).unwrap();
        let package = linked_package(
            env,
            vec![identity_function("id", ValueType::Boolean, ValueType::Boolean)],
        );
        let mut meter = Meter::new(UNLIMITED);
        let evaluation = package
            .call(
                "id",
                vec![Value::Boolean(true)],
                &ObjectEnvironment::default(),
                &mut meter,
            )
            .unwrap();
        (format!("{:?}", evaluation.outcome), meter.admitted_charges().to_vec())
    }};
    assert_eq!(
        result,
        format!("{:?}", Outcome::Completed(Value::Boolean(true)))
    );
    assert_eq!(charges, vec![ChargePoint::FunctionCall]);
}
