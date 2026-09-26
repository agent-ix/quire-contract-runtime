//! Function application under exact semantics (quire-specification/FR-146,
//! FR-273), through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use quire_contract_runtime::exact::{
    negotiate_ieee, negotiate_integer_division, plan_call, plan_evaluation, ChargePoint,
    CheckCause, CheckMode, CheckRefusal, CheckedPackage, CheckingLimits, DepthAboveMaximum,
    EvaluationRefusal, FunctionDeclaration, IeeeBackendCapabilities, IeeeDisposition,
    IeeeItemRequirement, IeeeOperationKind, IeeeUnsupportedCause, IeeeWidth, InputRefusal, Integer,
    IntegerDivisionConsumer, IntegerDivisionDisposition, LimitKind, Location, Meter, NodeKey,
    ObjectEnvironment, ObjectIdentity, ObjectReference, ObjectTypeDeclaration, Origin, Outcome,
    PackageDeclarations, Refusal, RoundingMode, ScalarLimits, TypeEnvironment, UniverseIdentity,
    Value, ValueType, MAX_CALL_DEPTH,
};

/// A `TypeEnvironment` declaring one model object type at `key(9)`, with no
/// attributes: enough for `ValueType::Reference(key(9))` to check.
fn object_type_environment() -> TypeEnvironment {
    TypeEnvironment::new(
        Vec::new(),
        vec![ObjectTypeDeclaration::new(key(9), "Widget", Vec::new())],
    )
    .unwrap()
}

/// `CheckedPackage` (the `Ok` side) holds host closures and so has no
/// `Debug`, which rules out `Result::unwrap_err`. This is the substitute:
/// panics with a fixed message on the `Ok` case rather than trying to format
/// it.
fn expect_check_refusals(result: Result<CheckedPackage, Vec<CheckRefusal>>) -> Vec<CheckRefusal> {
    match result {
        Err(refusals) => refusals,
        Ok(_) => panic!("expected `check` to refuse this package"),
    }
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

fn key(byte: u8) -> NodeKey {
    NodeKey::from_bytes([byte; 32])
}

fn a_reference() -> ObjectReference {
    ObjectReference::new(
        UniverseIdentity::new(b"u").unwrap(),
        key(9),
        ObjectIdentity::new(b"o").unwrap(),
    )
}

/// A one-function package: `identity(Boolean) -> Boolean`, echoing its
/// argument. Suitable for every arity/kind/dangling-reference refusal test,
/// since none of them ever reach the body.
fn identity_package() -> PackageDeclarations {
    PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "identity".to_string(),
            parameters: vec![("x".to_string(), ValueType::Boolean)],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
}

/// Trace: TC-194, FR-273-AC-1
#[test]
fn tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable() {
    let refusals = expect_check_refusals(
        identity_package().check(CheckMode::Kernel, CheckingLimits::default()),
    );
    assert_eq!(refusals.len(), 1);
    assert_eq!(refusals[0].cause, CheckCause::UnsupportedCheckMode);
}

/// Trace: TC-194, FR-273-AC-1, FR-273-AC-2
#[test]
fn tc_194_linked_package_applies_every_declared_function() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("identity", vec![Value::Boolean(true)], &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    assert!(evaluation.location.is_none());
    assert!(evaluation.losses.is_empty());
    assert!(meter
        .admitted_charges()
        .contains(&ChargePoint::FunctionCall));
}

/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_arity_mismatch_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call("identity", Vec::new(), &objects, &mut meter)
        .unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 1,
            supplied: 0,
        }
    );
    assert_eq!(refusal.code(), "invalid_runtime_input");
    assert_eq!(refusal.cause(), "wrong-value-kind");
    assert!(meter.admitted_charges().is_empty());
    assert_eq!(
        meter.consumed(quire_contract_runtime::exact::LimitKind::WorkUnits),
        0
    );
}

/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_wrong_value_kind_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "identity",
            vec![Value::Integer(Integer::from(1i64))],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::WrongValueKind { parameter: 0 });
    assert_eq!(refusal.code(), "invalid_runtime_input");
    assert_eq!(refusal.cause(), "wrong-value-kind");
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_dangling_reference_refuses_before_any_charge() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "take".to_string(),
            parameters: vec![("r".to_string(), ValueType::Reference(key(9)))],
            result: ValueType::Reference(key(9)),
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "take",
            vec![Value::Reference(a_reference())],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::DanglingReference { parameter: 0 });
    assert_eq!(refusal.code(), "dangling_reference");
    assert_eq!(refusal.cause(), "absent-target-in-complete-population");
    assert!(meter.admitted_charges().is_empty());
}

/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_unknown_function_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call("missing", Vec::new(), &objects, &mut meter)
        .unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::UnknownFunction("missing".to_string())
    );
    assert_eq!(refusal.code(), "missing_declaration");
    assert_eq!(refusal.cause(), "missing-name");
    assert!(meter.admitted_charges().is_empty());
}

/// `plan_call` alone reproduces every one of the four `InputRefusal`
/// variants with no `Meter` in scope at all: the refusal path never needs
/// one, for any of them.
///
/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_plan_call_refuses_identically_with_no_meter_in_scope() {
    let identity = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let two_param = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "two".to_string(),
            parameters: vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(key(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();

    // UnknownFunction
    let refusal = plan_call(&identity, "missing", &[], &objects).unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::UnknownFunction("missing".to_string())
    );

    // Arity
    let refusal = plan_call(&identity, "identity", &[], &objects).unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 1,
            supplied: 0,
        }
    );

    // WrongValueKind
    let refusal = plan_call(
        &identity,
        "identity",
        &[Value::Integer(Integer::from(1i64))],
        &objects,
    )
    .unwrap_err();
    assert_eq!(refusal, InputRefusal::WrongValueKind { parameter: 0 });

    // DanglingReference
    let refusal = plan_call(
        &two_param,
        "two",
        &[Value::Boolean(true), Value::Reference(a_reference())],
        &objects,
    )
    .unwrap_err();
    assert_eq!(refusal, InputRefusal::DanglingReference { parameter: 1 });
}

/// Argument 0 is the wrong kind AND argument 1 (an unrelated later
/// parameter) carries a dangling reference: parameter order wins, so the
/// refusal names parameter 0's kind, never the later dangling reference. A
/// naive "collect every violation" or "check references first"
/// implementation would instead report `DanglingReference { parameter: 1 }`.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_earlier_parameter_refusal_wins_over_a_later_dangling_reference() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "two".to_string(),
            parameters: vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(key(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "two",
            vec![
                Value::Integer(Integer::from(1i64)), // wrong kind at parameter 0
                Value::Reference(a_reference()),     // dangling, at parameter 1
            ],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::WrongValueKind { parameter: 0 });
    assert!(meter.admitted_charges().is_empty());
    assert!(LimitKind::ALL.iter().all(|&kind| meter.consumed(kind) == 0));
}

/// A one-argument-too-few call whose sole supplied argument would, at its
/// declared position, be *both* the wrong kind and a dangling reference:
/// arity is decided before any per-argument check runs, so the refusal is
/// `Arity`, never `WrongValueKind` or `DanglingReference`. Declaring two
/// parameters (`a: Boolean`, `b: Reference`) but supplying one dangling
/// `Reference` is discriminating: an implementation that validated the
/// supplied argument against parameter 0 (`Boolean`) before comparing counts
/// would report `WrongValueKind { parameter: 0 }`; one that validated it
/// against parameter 1 (`Reference`) would report `DanglingReference
/// { parameter: 1 }`. Only comparing counts first reports `Arity`.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_arity_is_decided_before_any_per_argument_check() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "two".to_string(),
            parameters: vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(key(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "two",
            vec![Value::Reference(a_reference())],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 2,
            supplied: 1,
        }
    );
    assert!(meter.admitted_charges().is_empty());
    assert!(LimitKind::ALL.iter().all(|&kind| meter.consumed(kind) == 0));
}

/// The body records whether `function.call` was already admitted at the
/// instant it started running: `call` must charge before invoking the body,
/// not after.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_function_call_precedes_the_body() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "observe".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|frame, _arguments| {
                // `Frame::meter` returns `Result<R, Refusal>`, not
                // `Option<R>`: a refusal here (the shared `Meter` already
                // mutably borrowed on this re-entrant chain) must propagate
                // as a refused outcome, not silently collapse to
                // `charged: false` and complete anyway.
                match frame.meter(|meter| {
                    meter
                        .admitted_charges()
                        .contains(&ChargePoint::FunctionCall)
                }) {
                    Ok(charged) => Outcome::Completed(Value::Boolean(charged)),
                    Err(r) => Outcome::Refused(Box::new(r)),
                }
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("observe", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// `evaluate` charges nothing for its own root, but each re-entrant
/// application the root reaches through `Frame::call` charges exactly one
/// `function.call`. A root with zero applications leaves the meter
/// untouched; a root with two charges exactly two.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_evaluate_charges_one_function_call_per_reached_application() {
    let checked = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "leaf".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();

    // Zero applications reached: the root itself is not an application.
    let expression_zero = checked
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        )
        .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = checked
        .evaluate(&expression_zero, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::FunctionCall));

    // Two applications reached, both of `leaf`.
    let expression_two = checked
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|frame, _arguments| {
                let first = frame.call("leaf", &[]).completed();
                let second = frame.call("leaf", &[]).completed();
                match (first, second) {
                    (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
                        Outcome::Completed(Value::Boolean(a && b))
                    }
                    _ => Outcome::Refused(Box::new(
                        quire_contract_runtime::exact::Refusal::CheckedInvariant,
                    )),
                }
            }),
        )
        .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = checked
        .evaluate(&expression_two, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    let calls = meter
        .admitted_charges()
        .iter()
        .filter(|point| **point == ChargePoint::FunctionCall)
        .count();
    assert_eq!(calls, 2);
}

/// An `InjectedDenial` at the `function.call` charge stops before the body
/// runs: the body's side effect (setting a flag) never happens.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_incomplete_function_call_charge_stops_before_the_body() {
    let ran = Rc::new(Cell::new(false));
    let ran_in_body = Rc::clone(&ran);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(move |_frame, _arguments| {
                ran_in_body.set(true);
                Outcome::Completed(Value::Boolean(true))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter =
        Meter::new(UNLIMITED).with_injected_denial(quire_contract_runtime::exact::InjectedDenial {
            point: ChargePoint::FunctionCall,
            occurrence: std::num::NonZeroU64::new(1).unwrap(),
        });
    let evaluation = package.call("f", Vec::new(), &objects, &mut meter).unwrap();
    assert!(matches!(evaluation.outcome, Outcome::Incomplete(_)));
    assert!(!ran.get());
}

/// `CallPlan::call_events` predicts, before any charge, exactly the number
/// of `function.call` charges the planned call or evaluation actually gets
/// admitted once run: one for a named `call`, zero for `evaluate`'s own
/// root.
///
/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_call_plan_predicts_the_admitted_function_call_charge_count() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();

    let plan = plan_call(&package, "identity", &[Value::Boolean(true)], &objects).unwrap();
    assert_eq!(*plan.call_events(), Integer::one());
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("identity", vec![Value::Boolean(true)], &objects, &mut meter)
        .unwrap();
    assert!(matches!(evaluation.outcome, Outcome::Completed(_)));
    let admitted_calls = meter
        .admitted_charges()
        .iter()
        .filter(|&&point| point == ChargePoint::FunctionCall)
        .count();
    assert_eq!(Integer::from(admitted_calls as i64), *plan.call_events());

    let expression = package
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        )
        .unwrap();
    let plan = plan_evaluation(&package, &expression, &[], &objects).unwrap();
    assert_eq!(*plan.call_events(), Integer::zero());
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(evaluation.outcome, Outcome::Completed(_)));
    let admitted_calls = meter
        .admitted_charges()
        .iter()
        .filter(|&&point| point == ChargePoint::FunctionCall)
        .count();
    assert_eq!(Integer::from(admitted_calls as i64), *plan.call_events());
}

/// `plan_evaluation` alone reproduces every `EvaluationRefusal` with no
/// `Meter` in scope at all: the same three `InputRefusal` variants
/// `plan_call` shares (there is no `UnknownFunction` here — `evaluate` takes
/// no function name to look up), plus the foreign-expression identity check
/// `plan_call` does not need.
///
/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5
#[test]
fn tc_194_plan_evaluation_refuses_every_variant_with_no_meter_in_scope() {
    let two_param = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let live_objects =
        ObjectEnvironment::new(&object_type_environment(), [(a_reference(), Vec::new())]).unwrap();

    let expression = two_param
        .check_expression(
            vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(key(9))),
            ],
            ValueType::Boolean,
            Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        )
        .unwrap();

    // Success: no refusal at all.
    assert!(plan_evaluation(
        &two_param,
        &expression,
        &[Value::Boolean(true), Value::Reference(a_reference())],
        &live_objects,
    )
    .is_ok());

    // Arity
    let refusal = plan_evaluation(&two_param, &expression, &[], &objects).unwrap_err();
    assert_eq!(
        refusal,
        EvaluationRefusal::Input(InputRefusal::Arity {
            declared: 2,
            supplied: 0,
        })
    );

    // WrongValueKind
    let refusal = plan_evaluation(
        &two_param,
        &expression,
        &[
            Value::Integer(Integer::from(1i64)),
            Value::Reference(a_reference()),
        ],
        &objects,
    )
    .unwrap_err();
    assert_eq!(
        refusal,
        EvaluationRefusal::Input(InputRefusal::WrongValueKind { parameter: 0 })
    );

    // DanglingReference
    let refusal = plan_evaluation(
        &two_param,
        &expression,
        &[Value::Boolean(true), Value::Reference(a_reference())],
        &objects,
    )
    .unwrap_err();
    assert_eq!(
        refusal,
        EvaluationRefusal::Input(InputRefusal::DanglingReference { parameter: 1 })
    );

    // ForeignExpression: `expression` was checked against `two_param`, not
    // `other`. Arguments are otherwise fully admitted, isolating the
    // identity check.
    let other = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let refusal = plan_evaluation(
        &other,
        &expression,
        &[Value::Boolean(true), Value::Reference(a_reference())],
        &live_objects,
    )
    .unwrap_err();
    assert_eq!(refusal, EvaluationRefusal::ForeignExpression);
}

/// `CheckedPackage::evaluate` shares `plan_call`'s argument validation and
/// `InputRefusal` set: every refusal `plan_evaluation` can produce surfaces
/// correctly through `evaluate` itself, not only through `plan_evaluation`
/// in isolation. A foreign expression (checked against a different package)
/// is not an `InputRefusal` at all — `evaluate`'s signature cannot carry a
/// fifth variant — so it surfaces as `Ok(Evaluation { outcome:
/// Outcome::Refused(Refusal::CheckedInvariant), .. })` instead, decided at
/// the plan boundary before any charge.
///
/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5
#[test]
fn tc_194_evaluate_shares_plan_call_argument_validation() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let live_objects =
        ObjectEnvironment::new(&object_type_environment(), [(a_reference(), Vec::new())]).unwrap();
    let expression = package
        .check_expression(
            vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(key(9))),
            ],
            ValueType::Boolean,
            Box::new(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        )
        .unwrap();

    // Arity
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 2,
            supplied: 0,
        }
    );
    assert!(meter.admitted_charges().is_empty());

    // WrongValueKind
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .evaluate(
            &expression,
            vec![
                Value::Integer(Integer::from(1i64)),
                Value::Reference(a_reference()),
            ],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::WrongValueKind { parameter: 0 });
    assert!(meter.admitted_charges().is_empty());

    // DanglingReference
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .evaluate(
            &expression,
            vec![Value::Boolean(true), Value::Reference(a_reference())],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::DanglingReference { parameter: 1 });
    assert!(meter.admitted_charges().is_empty());

    // ForeignExpression: checked against `package`, evaluated against
    // `other`. Arguments are otherwise fully admitted, isolating the
    // identity check.
    let other = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = other
        .evaluate(
            &expression,
            vec![Value::Boolean(true), Value::Reference(a_reference())],
            &live_objects,
            &mut meter,
        )
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Refused(ref refusal) if matches!(**refusal, Refusal::CheckedInvariant)
    ));
    assert!(meter.admitted_charges().is_empty());

    // The admitted case still applies totally, with no `function.call`
    // charge for `evaluate`'s own root.
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .evaluate(
            &expression,
            vec![Value::Boolean(true), Value::Reference(a_reference())],
            &live_objects,
            &mut meter,
        )
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::FunctionCall));
}

// ---- TC-194: a `CheckedPackage`'s identity survives being moved ----------

/// `PackageDeclarations::check` returns `CheckedPackage` by value, so any
/// move afterward (into an `Rc`, a `Box`, or simply up the stack) must not
/// change whether the package's own, already-checked `CheckedExpression`
/// still evaluates against it. An address-derived identity would fail this:
/// `self as *const Self as usize` changes on every move, so the identity
/// captured at `check_expression` time (before the move) would mismatch the
/// moved package's new address at `evaluate` time, refusing the package's
/// own expression as `ForeignExpression` — silently indistinguishable from a
/// genuine invariant violation inside a body.
///
/// Trace: TC-194
#[test]
fn tc_194_a_moved_package_still_evaluates_its_own_expression() {
    let objects = ObjectEnvironment::default();

    // Moved into an `Rc` after `check_expression`.
    let package = PackageDeclarations {
        types: Default::default(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let expression = package
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        )
        .unwrap();
    let package = Rc::new(package);
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(
        matches!(evaluation.outcome, Outcome::Completed(Value::Boolean(true))),
        "an `Rc`-moved package must still evaluate its own expression, got {:?}",
        evaluation.outcome
    );

    // Moved into a `Box` after `check_expression`.
    let package = PackageDeclarations {
        types: Default::default(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let expression = package
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        )
        .unwrap();
    let package = Box::new(package);
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(
        matches!(evaluation.outcome, Outcome::Completed(Value::Boolean(true))),
        "a `Box`-moved package must still evaluate its own expression, got {:?}",
        evaluation.outcome
    );
}

/// The converse of the move case above: an expression checked against a
/// package that has since been dropped must be refused by a *different*,
/// later-built package, even when the allocator hands the later package the
/// exact freed address the first one held. An address-derived identity would
/// admit this: once the first package is dropped and its allocation reused,
/// `self as *const Self as usize` for the second package equals the stored
/// identity on the first package's expression, and a genuinely foreign
/// expression would be accepted. Both packages are boxed identically (same
/// fields, so the same size class) to make address reuse likely on this
/// allocator; the monotonic id must refuse regardless of whether reuse
/// actually happens.
///
/// Trace: TC-194
#[test]
fn tc_194_an_expression_from_a_dropped_package_is_refused_by_a_new_one() {
    let objects = ObjectEnvironment::default();
    let freed_address: usize;
    let expression = {
        let package = Box::new(
            PackageDeclarations {
                types: Default::default(),
                functions: Vec::new(),
            }
            .check(CheckMode::Linked, CheckingLimits::default())
            .unwrap(),
        );
        freed_address = &*package as *const CheckedPackage as usize;
        package
            .check_expression(
                Vec::new(),
                ValueType::Boolean,
                Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
            )
            .unwrap()
        // `package` (the `Box`) is dropped here, freeing its allocation.
    };
    let second = Box::new(
        PackageDeclarations {
            types: Default::default(),
            functions: Vec::new(),
        }
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap(),
    );
    let second_address = &*second as *const CheckedPackage as usize;
    // This test only discriminates the false negative it exists to catch
    // (a freed address handed back to a *different* package, which
    // address-derived identity would wrongly admit) when the allocator
    // actually reuses `freed_address` here. On a hardened or governed
    // allocator, or under Miri, `second` can land at a fresh address, and
    // the test would then pass whether or not the monotonic-id fix is in
    // place -- silently. Pin the premise instead of hoping for it: fail
    // loudly, naming both addresses, the moment reuse does not happen, so a
    // green run of this test is never mistaken for a green run of the
    // defect check below it.
    assert_eq!(
        freed_address, second_address,
        "expected the allocator to hand `second` the freed address of the dropped \
         package (freed = {freed_address:#x}, second = {second_address:#x}); without \
         that reuse this test does not exercise the false negative it exists to catch, \
         so its pass below is not evidence of anything"
    );
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = second
        .evaluate(&expression, Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Refused(ref refusal) if matches!(**refusal, Refusal::CheckedInvariant)
    ));
    assert!(meter.admitted_charges().is_empty());
}

/// A body reached through `Frame::call` (not `CheckedPackage::call` itself)
/// counts how many `function.call` charges were already admitted at the
/// instant it started running, proving `Frame::call` also charges before
/// invoking the callee's body.
///
/// The count, not a `contains` check, is what discriminates here: `observe`
/// is reached from `root`, and `root`'s own `function.call` charge is already
/// admitted by the time `root`'s body runs and calls `observe` — so a mere
/// `contains(&ChargePoint::FunctionCall)` would read `true` at `observe`'s
/// first instruction regardless of whether `Frame::call` charges before or
/// after invoking `observe`'s body, since one prior charge (`root`'s) is
/// already on the meter either way. Requiring the count to be exactly `2`
/// (root's charge plus `Frame::call`'s own, for `observe`) is a witness of
/// *this* charge's ordering relative to *this* body: an implementation that
/// ran `observe`'s body before charging for it would leave the count at `1`
/// when the body observed it.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_frame_call_charges_function_call_before_the_body_it_invokes() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![
            FunctionDeclaration {
                name: "root".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(|frame, _arguments| frame.call("observe", &[])),
            },
            FunctionDeclaration {
                name: "observe".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(|frame, _arguments| {
                    // `Frame::meter` returns `Result<R, Refusal>`; propagate
                    // a refusal instead of collapsing it into a bogus count.
                    let attempt: Result<Value, Refusal> = frame
                        .meter(|meter| {
                            meter
                                .admitted_charges()
                                .iter()
                                .filter(|&&charge| charge == ChargePoint::FunctionCall)
                                .count()
                        })
                        .map(|count| {
                            let count = i64::try_from(count)
                                .expect("admitted-charge count fits in i64 for this test");
                            Value::Integer(Integer::from(count))
                        });
                    match attempt {
                        Ok(value) => Outcome::Completed(value),
                        Err(r) => Outcome::Refused(Box::new(r)),
                    }
                }),
            },
        ],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("root", Vec::new(), &objects, &mut meter)
        .unwrap();
    match &evaluation.outcome {
        Outcome::Completed(Value::Integer(count)) => {
            assert_eq!(*count, Integer::from(2i64));
        }
        other => panic!("expected Completed(Integer(2)), got {other:?}"),
    }
}

/// `CheckingLimits::new` refuses a depth above `MAX_CALL_DEPTH`.
///
/// Trace: TC-194, FR-273-AC-7
#[test]
fn tc_194_checking_limits_refuses_a_depth_above_the_maximum() {
    assert_eq!(
        CheckingLimits::new(0, MAX_CALL_DEPTH + 1),
        Err(DepthAboveMaximum {
            depth: MAX_CALL_DEPTH + 1
        })
    );
    assert!(CheckingLimits::new(0, MAX_CALL_DEPTH).is_ok());
}

/// Recursion through `Frame::call` beyond `CheckingLimits::depth` refuses
/// with `Refusal::CheckedInvariant` rather than recursing without bound.
///
/// Trace: TC-194, FR-273-AC-7
#[test]
fn tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal() {
    let limits = CheckingLimits::new(u64::MAX, 3).unwrap();
    fn recurse(
        frame: &quire_contract_runtime::exact::Frame<'_>,
        _arguments: &[Value],
    ) -> Outcome<Value> {
        frame.call("loop", &[])
    }
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "loop".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(recurse),
        }],
    }
    .check(CheckMode::Linked, limits)
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("loop", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Refused(ref refusal) if matches!(**refusal, quire_contract_runtime::exact::Refusal::CheckedInvariant)
    ));
}

/// A body that re-enters its own [`Frame::meter`] from inside the closure
/// [`Frame::meter`] already handed it refuses the inner access with
/// `Err(Refusal::CheckedInvariant)` instead of panicking on the double
/// `RefCell` borrow.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_reentrant_frame_meter_refuses_instead_of_panicking() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "reentrant_meter".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|frame, _arguments| {
                let outer = frame.meter(|_outer_meter| frame.meter(|_inner_meter| true));
                let inner_refused = matches!(outer, Ok(Err(Refusal::CheckedInvariant)));
                Outcome::Completed(Value::Boolean(inner_refused))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("reentrant_meter", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// A body that calls [`Frame::call`] from inside a [`Frame::meter`] closure
/// re-enters the same frame's `Meter` `RefCell`: `Frame::run` refuses with
/// [`Refusal::CheckedInvariant`] instead of panicking on the double borrow.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_reentrant_frame_call_during_meter_access_refuses_instead_of_panicking() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "reentrant_call".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|frame, _arguments| {
                frame
                    .meter(|_meter| frame.call("reentrant_call", &[]))
                    .unwrap_or(Outcome::Refused(Box::new(Refusal::CheckedInvariant)))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("reentrant_call", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Refused(ref refusal) if matches!(**refusal, Refusal::CheckedInvariant)
    ));
}

/// The original attack this bounds: a running [`Body`](quire_contract_runtime::exact::Body)
/// that holds its own `Rc<CheckedPackage>` (obtainable in safe Rust) and
/// re-enters [`CheckedPackage::call`] directly, bypassing [`Frame::call`]'s
/// depth check entirely — every fresh root `Frame` it builds looks identical
/// to a program's first call, since depth was once threaded only by value.
/// The shared `Cell<u64>` counter on `CheckedPackage` itself bounds this
/// path exactly as it bounds `Frame::call`, refusing rather than recursing
/// the host stack without limit.
///
/// Trace: TC-194, FR-273-AC-7
#[test]
fn tc_194_direct_reentrant_package_call_is_bounded_like_frame_call() {
    let limits = CheckingLimits::new(u64::MAX, 3).unwrap();
    let holder: Rc<RefCell<Option<Rc<CheckedPackage>>>> = Rc::new(RefCell::new(None));
    let body_holder = Rc::clone(&holder);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "loop".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(move |_frame, _arguments| {
                let package = body_holder
                    .borrow()
                    .as_ref()
                    .expect("holder populated before first call")
                    .clone();
                let objects = ObjectEnvironment::default();
                let mut meter = Meter::new(UNLIMITED);
                match package.call("loop", Vec::new(), &objects, &mut meter) {
                    Ok(evaluation) => evaluation.outcome,
                    Err(refusal) => panic!("unexpected InputRefusal: {refusal:?}"),
                }
            }),
        }],
    }
    .check(CheckMode::Linked, limits)
    .unwrap();
    let package = Rc::new(package);
    *holder.borrow_mut() = Some(Rc::clone(&package));
    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("loop", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Refused(ref refusal) if matches!(**refusal, Refusal::CheckedInvariant)
    ));
}

// ---- TC-195: FR-009 negotiation is not an evaluator outcome ---------------

/// Trace: TC-195, FR-273-AC-4
#[test]
fn tc_195_undischargeable_function_capability_negotiates_unsupported() {
    let requirement = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    let backend = IeeeBackendCapabilities::default();
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: vec![requirement],
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let dispositions = negotiate_ieee(package.ieee_requirements("f").unwrap(), &backend);
    assert_eq!(dispositions.len(), 1);
    match dispositions[0] {
        IeeeDisposition::Unsupported(cause) => {
            assert_eq!(cause, IeeeUnsupportedCause::Width(IeeeWidth::Binary64))
        }
        other => panic!("expected Unsupported, got {other:?}"),
    }
}

/// Trace: TC-195, FR-273-AC-4
#[test]
fn tc_195_dischargeable_requirements_negotiate_supported_or_requires_bound() {
    let mut backend = IeeeBackendCapabilities::default();
    backend.widths.insert(IeeeWidth::Binary64);
    backend.operations.insert(IeeeOperationKind::Add);
    backend.roundings.insert(RoundingMode::NearestEven);
    backend.exceptional_policy = true;
    backend.finite_proof = false;

    let plain = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    let bounded = IeeeItemRequirement {
        requires_finite_proof: true,
        ..plain
    };
    let dispositions = negotiate_ieee(&[plain, bounded], &backend);
    assert_eq!(
        dispositions,
        vec![IeeeDisposition::Supported, IeeeDisposition::RequiresBound]
    );

    let mathematical = IntegerDivisionConsumer::Mathematical;
    assert_eq!(
        negotiate_integer_division(&[mathematical]),
        vec![IntegerDivisionDisposition::Supported]
    );
}

/// Negotiation is a pure function of its inputs: `negotiate_ieee`'s signature
/// (`&[IeeeItemRequirement], &IeeeBackendCapabilities`) takes no `Meter`
/// parameter at all, so no application-time charge is reachable from it by
/// construction — that is FR-273-AC-4's "before any application, with no
/// `Meter` participation," and it is a compile-time property the signature
/// itself proves, not one a runtime assertion on an unreachable local
/// `Meter` could discriminate. Its result type having no path into
/// `Outcome`/`InputRefusal` is proven separately, by the `compile_fail`
/// doctest on `IeeeDisposition` (`src/exact/ieee.rs`).
///
/// Trace: TC-195, FR-273-AC-4
#[test]
fn tc_195_negotiate_ieee_takes_no_meter_by_signature() {
    let backend = IeeeBackendCapabilities::default();
    let requirement = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    // The call itself is not the evidence for "no `Meter` participation" —
    // the signature above is. This just confirms negotiation still runs and
    // reports exactly one disposition for the one requirement supplied.
    let dispositions = negotiate_ieee(&[requirement], &backend);
    assert_eq!(dispositions.len(), 1);
}

// TC-195, FR-273-AC-4: `IeeeDisposition` has no conversion to `Outcome<Value>`
// or `InputRefusal`. That is a static property, not a runtime one, so it is
// enforced by a real `compile_fail` doctest on `IeeeDisposition` itself
// (`src/exact/ieee.rs`, run under `cargo test --doc`) rather than by a
// `#[test]` here — a doc comment on a `#[test]` fn in an integration test
// file is never scanned as a doctest, so that placement would have been
// silently vacuous.

/// Trace: TC-195, FR-273-AC-4
#[test]
fn tc_195_supported_function_applies_normally_and_produces_no_disposition() {
    let mut backend = IeeeBackendCapabilities::default();
    backend.widths.insert(IeeeWidth::Binary64);
    backend.operations.insert(IeeeOperationKind::Add);
    backend.roundings.insert(RoundingMode::NearestEven);
    backend.exceptional_policy = true;

    let requirement = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: vec![requirement],
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();

    assert_eq!(
        negotiate_ieee(package.ieee_requirements("f").unwrap(), &backend),
        vec![IeeeDisposition::Supported]
    );

    let objects = ObjectEnvironment::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package.call("f", Vec::new(), &objects, &mut meter).unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

// ---- PackageDeclarations::check: what a generated package must supply -----

/// Trace: TC-194, FR-273-AC-1
#[test]
fn tc_194_ambiguous_function_names_refuse_admission() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![
            FunctionDeclaration {
                name: "dup".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
            },
            FunctionDeclaration {
                name: "dup".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(false))),
            },
        ],
    };
    let refusals =
        expect_check_refusals(package.check(CheckMode::Linked, CheckingLimits::default()));
    assert_eq!(refusals.len(), 2);
    let expected_loci = vec![
        Location {
            origin: Origin::Body {
                function: "dup".to_string(),
                index: 0,
            },
            path: Vec::new(),
        },
        Location {
            origin: Origin::Body {
                function: "dup".to_string(),
                index: 1,
            },
            path: Vec::new(),
        },
    ];
    for refusal in &refusals {
        match &refusal.cause {
            CheckCause::AmbiguousName { name, loci } => {
                assert_eq!(name, "dup");
                assert_eq!(loci, &expected_loci);
            }
            other => panic!("expected CheckCause::AmbiguousName, got {other:?}"),
        }
    }
}

/// Trace: TC-194, FR-273-AC-1
#[test]
fn tc_194_undischarged_measure_refuses_admission() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "f".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: false,
            body: Box::new(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    };
    let refusals =
        expect_check_refusals(package.check(CheckMode::Linked, CheckingLimits::default()));
    assert_eq!(refusals.len(), 1);
    assert_eq!(refusals[0].cause, CheckCause::UndischargedMeasure);
}
