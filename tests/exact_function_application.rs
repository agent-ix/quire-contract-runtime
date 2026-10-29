//! Function application under exact semantics (quire-specification/FR-146,
//! FR-273), through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[path = "support/charge_points.rs"]
mod charge_points;
use charge_points::ChargePoints;

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    plan_call, plan_evaluation, Body, BodyStep, CheckCause, CheckMode, CheckRefusal,
    CheckedPackage, CheckingLimits, EvaluationRefusal, Frame, FunctionDeclaration, InputRefusal,
    Location, Origin, PackageDeclarations,
};
use quire_contract_runtime::scalar::{
    negotiate_ieee, negotiate_integer_division, IeeeBackendCapabilities, IeeeDisposition,
    IeeeItemRequirement, IeeeUnsupportedCause, IntegerDivisionConsumer, IntegerDivisionDisposition,
};
use quire_exact::{
    ChargePoint, CheckedInvariantCause, EffectiveId, Identifier, IeeeOperationKind, IeeeWidth,
    Integer, LimitKind, Meter, NodeKey, ObjectId, ObjectReference, Outcome, Refusal, RoundingMode,
    ScalarLimits, Undefined, UnionMember, UnionValue, UniverseId, Value, ValueType,
};
use quire_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, ObjectTypeDeclaration, TypeEnvironment,
    UnionMemberDeclaration,
};
use quire_semantic_value::object_closure::ObjectClosure;

/// A `TypeEnvironment` declaring one model object type at `key(9)`, with no
/// attributes: enough for `ValueType::Reference(effective(9))` to check.
fn object_type_environment() -> TypeEnvironment {
    TypeEnvironment::new(
        Vec::new(),
        vec![ObjectTypeDeclaration::new(
            effective(9),
            "Widget",
            Vec::new(),
        )],
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

/// Adapt a single-transition test body to the generated body contract.
fn one_step<F>(run: F) -> Body
where
    F: for<'a, 'm> Fn(&Frame<'a, 'm>, &[Value]) -> Outcome<Value> + 'static,
{
    let run = Rc::new(run);
    Box::new(move |arguments| {
        let run = Rc::clone(&run);
        Box::new(move |frame: &Frame<'_, '_>, _completed: Option<Value>| {
            BodyStep::Return(run(frame, &arguments))
        })
    })
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

fn effective(byte: u8) -> EffectiveId {
    EffectiveId::from_digest([byte; 32])
}

fn a_reference() -> ObjectReference {
    ObjectReference::new(
        UniverseId::from_digest([0x75; 32]),
        effective(9),
        ObjectId::new("o").unwrap(),
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
            body: one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
}

#[trace("TC-194", "FR-273-AC-1")]
#[test]
fn tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable() {
    let refusals = expect_check_refusals(
        identity_package().check(CheckMode::Kernel, CheckingLimits::default()),
    );
    assert_eq!(refusals.len(), 1);
    assert_eq!(refusals[0].cause, CheckCause::UnsupportedCheckMode);
}

#[trace("TC-194", "FR-273-AC-1", "FR-273-AC-2")]
#[test]
fn tc_194_linked_package_applies_every_declared_function() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectClosure::default();
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
        .admitted_charge_points()
        .contains(&ChargePoint::FunctionCall));
}

#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_arity_mismatch_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectClosure::default();
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
    assert_eq!(meter.consumed(quire_exact::LimitKind::WorkUnits), 0);
}

#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_wrong_value_kind_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectClosure::default();
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

#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_dangling_reference_refuses_before_any_charge() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "take".to_string(),
            parameters: vec![("r".to_string(), ValueType::Reference(effective(9)))],
            result: ValueType::Reference(effective(9)),
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
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

#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_dangling_reference_inside_union_refuses_before_any_charge() {
    let declaration_key = NodeKey::from_digest([10; 32]);
    let member = UnionMember::from_admitted(
        declaration_key,
        NodeKey::from_digest([11; 32]),
        Identifier::new("with_ref").unwrap(),
    );
    let types = TypeEnvironment::new(
        [CompositeDeclaration::new(
            declaration_key,
            "ReferenceUnion",
            CompositeShape::Union(vec![UnionMemberDeclaration::from_verified(
                member.clone(),
                vec![ValueType::Reference(effective(9))],
            )]),
        )],
        [ObjectTypeDeclaration::new(
            effective(9),
            "Widget",
            Vec::new(),
        )],
    )
    .unwrap();
    let package = PackageDeclarations {
        types,
        functions: vec![FunctionDeclaration {
            name: "take_union".to_string(),
            parameters: vec![("u".to_string(), ValueType::Composite(declaration_key))],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let argument = UnionValue::from_admitted(member, vec![Value::Reference(a_reference())]);
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "take_union",
            vec![argument],
            &ObjectClosure::default(),
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(refusal, InputRefusal::DanglingReference { parameter: 0 });
    assert!(meter.admitted_charges().is_empty());
}

#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_unknown_function_refuses_before_any_charge() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectClosure::default();
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
#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
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
                ("b".to_string(), ValueType::Reference(effective(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();

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
#[trace("TC-194", "FR-273-AC-2")]
#[test]
fn tc_194_earlier_parameter_refusal_wins_over_a_later_dangling_reference() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "two".to_string(),
            parameters: vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(effective(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
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
#[trace("TC-194", "FR-273-AC-2")]
#[test]
fn tc_194_arity_is_decided_before_any_per_argument_check() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: vec![FunctionDeclaration {
            name: "two".to_string(),
            parameters: vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(effective(9))),
            ],
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
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
#[trace("TC-194", "FR-273-AC-2")]
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
            body: one_step(|frame, _arguments| {
                // `Frame::meter` returns `Result<R, Refusal>`, not
                // `Option<R>`: a refusal here (the shared `Meter` already
                // mutably borrowed on this re-entrant chain) must propagate
                // as a refused outcome, not silently collapse to
                // `charged: false` and complete anyway.
                match frame.meter(|meter| {
                    meter
                        .admitted_charge_points()
                        .contains(&ChargePoint::FunctionCall)
                }) {
                    Ok(charged) => Outcome::Completed(Value::Boolean(charged)),
                    Err(r) => Outcome::Refused(r),
                }
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("observe", Vec::new(), &objects, &mut meter)
        .unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

/// `evaluate` charges nothing for its own root, while each requested child
/// application charges exactly one `function.call`. A root with zero
/// applications leaves the meter untouched; two children charge twice.
#[trace("TC-194", "FR-273-AC-2")]
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
            body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();

    // Zero applications reached: the root itself is not an application.
    let expression_zero = checked
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
        .admitted_charge_points()
        .contains(&ChargePoint::FunctionCall));

    // Two applications reached, both of `leaf`.
    let expression_two = checked
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(|_| {
                let mut phase = 0;
                let mut first_value = false;
                Box::new(
                    move |_: &Frame<'_, '_>, completed: Option<Value>| match phase {
                        0 => {
                            phase = 1;
                            BodyStep::Call {
                                function: "leaf".to_string(),
                                arguments: Vec::new(),
                            }
                        }
                        1 => {
                            first_value = matches!(completed, Some(Value::Boolean(true)));
                            phase = 2;
                            BodyStep::Call {
                                function: "leaf".to_string(),
                                arguments: Vec::new(),
                            }
                        }
                        _ => BodyStep::Return(Outcome::Completed(Value::Boolean(
                            first_value && matches!(completed, Some(Value::Boolean(true))),
                        ))),
                    },
                )
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
        .admitted_charge_points()
        .iter()
        .filter(|point| **point == ChargePoint::FunctionCall)
        .count();
    assert_eq!(calls, 2);
}

/// An injected or ordinary work-limit denial at `function.call` stops before
/// the body runs: its side effect (setting a flag) never happens.
#[trace("TC-194", "FR-273-AC-2")]
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
            body: one_step(move |_frame, _arguments| {
                ran_in_body.set(true);
                Outcome::Completed(Value::Boolean(true))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(quire_exact::InjectedDenial {
        point: ChargePoint::FunctionCall,
        occurrence: std::num::NonZeroU64::MIN,
    });
    let evaluation = package.call("f", Vec::new(), &objects, &mut meter).unwrap();
    assert!(matches!(evaluation.outcome, Outcome::Incomplete(_)));
    assert!(!ran.get());

    let mut ordinary_limit = Meter::new(ScalarLimits {
        work_units: 0,
        ..UNLIMITED
    });
    let denied = package
        .call("f", Vec::new(), &objects, &mut ordinary_limit)
        .unwrap();
    let Outcome::Incomplete(record) = denied.outcome else {
        panic!("a denied function call did not return Incomplete");
    };
    assert_eq!(
        record,
        Incomplete {
            limit_kind: LimitKind::WorkUnits,
            limit: 0,
            consumed: 0,
            next_charge: Integer::from(1_i64),
            charge_point: ChargePoint::FunctionCall,
        }
    );
    assert!(!ran.get(), "a denied charge ran the function body");
    assert_eq!(ordinary_limit.admission_count(), 0);
    assert_eq!(ordinary_limit.consumed(LimitKind::ResultUnits), 0);
}

/// `CallPlan::call_events` predicts, before any charge, exactly the number
/// of `function.call` charges the planned call or evaluation actually gets
/// admitted once run: one for a named `call`, zero for `evaluate`'s own
/// root.
#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3")]
#[test]
fn tc_194_call_plan_predicts_the_admitted_function_call_charge_count() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectClosure::default();

    let plan = plan_call(&package, "identity", &[Value::Boolean(true)], &objects).unwrap();
    assert_eq!(*plan.call_events(), Integer::one());
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package
        .call("identity", vec![Value::Boolean(true)], &objects, &mut meter)
        .unwrap();
    assert!(matches!(evaluation.outcome, Outcome::Completed(_)));
    let admitted_calls = meter
        .admitted_charge_points()
        .iter()
        .filter(|&&point| point == ChargePoint::FunctionCall)
        .count();
    assert_eq!(Integer::from(admitted_calls as i64), *plan.call_events());

    let expression = package
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
        .admitted_charge_points()
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
#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3", "FR-273-AC-5")]
#[test]
fn tc_194_plan_evaluation_refuses_every_variant_with_no_meter_in_scope() {
    let two_param = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
    let live_objects = ObjectClosure::new(
        &object_type_environment(),
        [(a_reference(), Vec::new())],
        &[],
    )
    .unwrap();

    let expression = two_param
        .check_expression(
            vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(effective(9))),
            ],
            ValueType::Boolean,
            one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
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
/// Outcome::Refused(Refusal::CheckedInvariant {
///     cause: CheckedInvariantCause::ForeignCheckedExpression,
/// }), .. })` instead, decided at
/// the plan boundary before any charge.
#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-3", "FR-273-AC-5", "FR-273-AC-8")]
#[test]
fn tc_194_evaluate_shares_plan_call_argument_validation() {
    let package = PackageDeclarations {
        types: object_type_environment(),
        functions: Vec::new(),
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
    let live_objects = ObjectClosure::new(
        &object_type_environment(),
        [(a_reference(), Vec::new())],
        &[],
    )
    .unwrap();
    let expression = package
        .check_expression(
            vec![
                ("a".to_string(), ValueType::Boolean),
                ("b".to_string(), ValueType::Reference(effective(9))),
            ],
            ValueType::Boolean,
            one_step(|_frame, arguments| Outcome::Completed(arguments[0].clone())),
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
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::ForeignCheckedExpression
        })
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
        .admitted_charge_points()
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
#[trace("TC-194")]
#[test]
fn tc_194_a_moved_package_still_evaluates_its_own_expression() {
    let objects = ObjectClosure::default();

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
            one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
            one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
#[trace("TC-194")]
#[test]
fn tc_194_an_expression_from_a_dropped_package_is_refused_by_a_new_one() {
    let objects = ObjectClosure::default();
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
                one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::ForeignCheckedExpression
        })
    ));
    assert!(meter.admitted_charges().is_empty());
}

/// A child body requested by a generated transition
/// counts how many `function.call` charges were admitted before it starts.
///
/// The count, not a `contains` check, is what discriminates here: `observe`
/// is reached from `root`, whose charge is already admitted. Requiring the
/// count to be exactly two witnesses that the child charge preceded the
/// child's body.
#[trace("TC-194", "FR-273-AC-2")]
#[test]
fn tc_194_child_call_charges_before_callee_body() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![
            FunctionDeclaration {
                name: "root".to_string(),
                parameters: Vec::new(),
                result: ValueType::Integer,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(|_| {
                    let mut requested = false;
                    Box::new(move |_: &Frame<'_, '_>, completed: Option<Value>| {
                        if requested {
                            BodyStep::Return(Outcome::Completed(
                                completed.expect("observe returned a value"),
                            ))
                        } else {
                            requested = true;
                            BodyStep::Call {
                                function: "observe".to_string(),
                                arguments: Vec::new(),
                            }
                        }
                    })
                }),
            },
            FunctionDeclaration {
                name: "observe".to_string(),
                parameters: Vec::new(),
                result: ValueType::Integer,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(|frame, _arguments| {
                    // `Frame::meter` returns `Result<R, Refusal>`; propagate
                    // a refusal instead of collapsing it into a bogus count.
                    let attempt: Result<Value, Refusal> = frame
                        .meter(|meter| {
                            meter
                                .admitted_charge_points()
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
                        Err(r) => Outcome::Refused(r),
                    }
                }),
            },
        ],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let objects = ObjectClosure::default();
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

/// The generated body returns a request, then receives the child's value in
/// a later transition. The runtime owns every suspended state on its heap.
#[trace("TC-194", "FR-273-AC-7", "FR-273-AC-9", "FR-273-AC-11")]
#[test]
fn tc_194_explicit_frames_reach_4096_calls_on_64_kib_stack_with_exact_fuel() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            for depth in [1usize, 64, 128, 4096] {
                let active = Rc::new(Cell::new(0usize));
                let active_body = Rc::clone(&active);
                let package = PackageDeclarations {
                    types: Default::default(),
                    functions: vec![FunctionDeclaration {
                        name: "count".to_string(),
                        parameters: vec![("remaining".to_string(), ValueType::Integer)],
                        result: ValueType::Integer,
                        ieee_requirements: Vec::new(),
                        integer_division_consumers: Vec::new(),
                        measure_discharged: true,
                        body: Box::new(move |arguments| {
                            let remaining = match &arguments[0] {
                                Value::Integer(value) => value.to_u64().unwrap(),
                                other => panic!("expected checked integer argument: {other:?}"),
                            };
                            let active = Rc::clone(&active_body);
                            let mut waiting = false;
                            Box::new(move |_: &Frame<'_, '_>, completed: Option<Value>| {
                                assert_eq!(active.get(), 0, "only one body transition is active");
                                active.set(1);
                                let next = if waiting {
                                    assert!(matches!(
                                        completed,
                                        Some(Value::Integer(value))
                                            if value == Integer::from((remaining - 1) as i64)
                                    ));
                                    BodyStep::Return(Outcome::Completed(Value::Integer(
                                        Integer::from(remaining as i64),
                                    )))
                                } else if remaining > 1 {
                                    waiting = true;
                                    BodyStep::Call {
                                        function: "count".to_string(),
                                        arguments: vec![Value::Integer(Integer::from(
                                            (remaining - 1) as i64,
                                        ))],
                                    }
                                } else {
                                    BodyStep::Return(Outcome::Completed(Value::Integer(
                                        Integer::from(1i64),
                                    )))
                                };
                                active.set(0);
                                next
                            })
                        }),
                    }],
                }
                .check(CheckMode::Linked, CheckingLimits::default())
                .unwrap();
                let input = vec![Value::Integer(Integer::from(depth as i64))];
                let objects = ObjectClosure::default();
                let mut exact_meter = Meter::new(ScalarLimits {
                    work_units: depth as u64,
                    ..UNLIMITED
                });
                let exact = package
                    .call("count", input.clone(), &objects, &mut exact_meter)
                    .unwrap();
                assert!(matches!(
                    exact.outcome,
                    Outcome::Completed(Value::Integer(ref value))
                        if *value == Integer::from(depth as i64)
                ));
                assert_eq!(exact_meter.consumed(LimitKind::WorkUnits), depth as u64);
                assert_eq!(
                    exact_meter.admitted_charge_points(),
                    vec![ChargePoint::FunctionCall; depth]
                );

                let mut short_meter = Meter::new(ScalarLimits {
                    work_units: (depth - 1) as u64,
                    ..UNLIMITED
                });
                let short = package
                    .call("count", input.clone(), &objects, &mut short_meter)
                    .unwrap();
                match short.outcome {
                    Outcome::Incomplete(record) => {
                        assert_eq!(record.limit_kind, LimitKind::WorkUnits);
                        assert_eq!(record.charge_point, ChargePoint::FunctionCall);
                        assert_eq!(record.consumed, (depth - 1) as u64);
                        assert_eq!(record.next_charge, Integer::one());
                    }
                    other => panic!("expected work fuel stop: {other:?}"),
                }
                assert_eq!(
                    short_meter.consumed(LimitKind::WorkUnits),
                    (depth - 1) as u64
                );
                assert_eq!(
                    short_meter.admitted_charge_points(),
                    vec![ChargePoint::FunctionCall; depth - 1]
                );
                if depth == 64 {
                    let mut retry_meter = Meter::new(ScalarLimits {
                        work_units: 64,
                        ..UNLIMITED
                    });
                    let retry = package
                        .call("count", input, &objects, &mut retry_meter)
                        .unwrap();
                    assert!(matches!(
                        retry.outcome,
                        Outcome::Completed(Value::Integer(ref value))
                            if *value == Integer::from(64i64)
                    ));
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Argument applications are requested left to right. The enclosing charge
/// occurs after both have returned and before its body is entered.
#[trace("TC-194", "FR-273-AC-10", "FR-273-AC-11")]
#[test]
fn tc_194_nested_arguments_charge_before_enclosing_body_and_stop_at_one_less() {
    let entered_join = Rc::new(Cell::new(false));
    let entered_join_body = Rc::clone(&entered_join);
    let resumed_root = Rc::new(Cell::new(false));
    let resumed_root_body = Rc::clone(&resumed_root);
    let body_order = Rc::new(RefCell::new(Vec::new()));
    let left_order = Rc::clone(&body_order);
    let right_order = Rc::clone(&body_order);
    let join_order = Rc::clone(&body_order);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![
            FunctionDeclaration {
                name: "left".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(move |_, _| {
                    left_order.borrow_mut().push("left");
                    Outcome::Completed(Value::Boolean(true))
                }),
            },
            FunctionDeclaration {
                name: "right".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(move |_, _| {
                    right_order.borrow_mut().push("right");
                    Outcome::Completed(Value::Boolean(true))
                }),
            },
            FunctionDeclaration {
                name: "join".to_string(),
                parameters: vec![("input".to_string(), ValueType::Boolean)],
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(move |frame, arguments| {
                    entered_join_body.set(true);
                    join_order.borrow_mut().push("join");
                    assert_eq!(
                        frame.meter(|meter| meter.admitted_charge_points().len()),
                        Ok(3)
                    );
                    Outcome::Completed(arguments[0].clone())
                }),
            },
        ],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let expression = package
        .check_expression(
            Vec::new(),
            ValueType::Boolean,
            Box::new(move |_| {
                let resumed_root = Rc::clone(&resumed_root_body);
                let mut phase = 0;
                let mut left = false;
                Box::new(move |_: &Frame<'_, '_>, child: Option<Value>| {
                    let next = match phase {
                        0 => BodyStep::Call {
                            function: "left".to_string(),
                            arguments: Vec::new(),
                        },
                        1 => {
                            left = matches!(child, Some(Value::Boolean(true)));
                            BodyStep::Call {
                                function: "right".to_string(),
                                arguments: Vec::new(),
                            }
                        }
                        2 => BodyStep::Call {
                            function: "join".to_string(),
                            arguments: vec![Value::Boolean(
                                left && matches!(child, Some(Value::Boolean(true))),
                            )],
                        },
                        _ => {
                            resumed_root.set(true);
                            BodyStep::Return(Outcome::Completed(
                                child.expect("join returned a value"),
                            ))
                        }
                    };
                    phase += 1;
                    next
                })
            }),
        )
        .unwrap();
    let objects = ObjectClosure::default();
    let mut exact_meter = Meter::new(UNLIMITED);
    let exact = package
        .evaluate(&expression, Vec::new(), &objects, &mut exact_meter)
        .unwrap();
    assert!(matches!(
        exact.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    assert!(entered_join.get());
    assert!(resumed_root.get());
    assert_eq!(*body_order.borrow(), ["left", "right", "join"]);
    assert_eq!(exact_meter.consumed(LimitKind::WorkUnits), 3);
    assert_eq!(
        exact_meter.admitted_charge_points(),
        &[ChargePoint::FunctionCall; 3]
    );

    entered_join.set(false);
    resumed_root.set(false);
    body_order.borrow_mut().clear();
    let mut short_meter = Meter::new(ScalarLimits {
        work_units: 2,
        ..UNLIMITED
    });
    let short = package
        .evaluate(&expression, Vec::new(), &objects, &mut short_meter)
        .unwrap();
    assert!(matches!(
        short.outcome,
        Outcome::Incomplete(ref record)
            if record.limit_kind == LimitKind::WorkUnits
                && record.charge_point == ChargePoint::FunctionCall
                && record.consumed == 2
    ));
    assert!(!entered_join.get());
    assert!(!resumed_root.get());
    assert_eq!(*body_order.borrow(), ["left", "right"]);
    assert_eq!(
        short_meter.admitted_charge_points(),
        &[ChargePoint::FunctionCall; 2]
    );
}

/// Re-entering the same mutable meter borrow is a typed checked invariant.
#[trace("TC-194", "FR-273-AC-8")]
#[test]
fn tc_194_reentrant_meter_borrow_has_typed_cause() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "borrow".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: one_step(|frame, _| {
                let inner = frame.meter(|_| frame.meter(|_| true));
                Outcome::Completed(Value::Boolean(matches!(
                    inner,
                    Ok(Err(Refusal::CheckedInvariant {
                        cause: CheckedInvariantCause::MeterBorrowConflict
                    }))
                )))
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let result = package
        .call("borrow", Vec::new(), &ObjectClosure::default(), &mut meter)
        .unwrap();
    assert!(matches!(
        result.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
}

/// Child arity and kind faults stop before the child body or charge.
#[trace("TC-194", "FR-273-AC-2", "FR-273-AC-11")]
#[test]
fn tc_194_invalid_child_arguments_refuse_before_child_charge_or_body() {
    for arguments in [Vec::new(), vec![Value::Integer(Integer::one())]] {
        let entered_child = Rc::new(Cell::new(false));
        let child_flag = Rc::clone(&entered_child);
        let resumed_parent = Rc::new(Cell::new(false));
        let parent_flag = Rc::clone(&resumed_parent);
        let package = PackageDeclarations {
            types: Default::default(),
            functions: vec![
                FunctionDeclaration {
                    name: "parent".to_string(),
                    parameters: Vec::new(),
                    result: ValueType::Boolean,
                    ieee_requirements: Vec::new(),
                    integer_division_consumers: Vec::new(),
                    measure_discharged: true,
                    body: Box::new(move |_| {
                        let arguments = arguments.clone();
                        let resumed = Rc::clone(&parent_flag);
                        let mut first = true;
                        Box::new(move |_: &Frame<'_, '_>, _: Option<Value>| {
                            if first {
                                first = false;
                                BodyStep::Call {
                                    function: "child".to_string(),
                                    arguments: arguments.clone(),
                                }
                            } else {
                                resumed.set(true);
                                BodyStep::Return(Outcome::Completed(Value::Boolean(true)))
                            }
                        })
                    }),
                },
                FunctionDeclaration {
                    name: "child".to_string(),
                    parameters: vec![("value".to_string(), ValueType::Boolean)],
                    result: ValueType::Boolean,
                    ieee_requirements: Vec::new(),
                    integer_division_consumers: Vec::new(),
                    measure_discharged: true,
                    body: one_step(move |_, _| {
                        child_flag.set(true);
                        Outcome::Completed(Value::Boolean(true))
                    }),
                },
            ],
        }
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
        let mut meter = Meter::new(UNLIMITED);
        let result = package
            .call("parent", Vec::new(), &ObjectClosure::default(), &mut meter)
            .unwrap();
        assert!(matches!(
            result.outcome,
            Outcome::Refused(Refusal::CheckedInvariant {
                cause: CheckedInvariantCause::GeneratedArgumentShapeMismatch
            })
        ));
        assert!(!entered_child.get());
        assert!(!resumed_parent.get());
        assert_eq!(meter.admitted_charge_points(), &[ChargePoint::FunctionCall]);
    }
}

/// A stopped child terminates the scheduler without delivering a value to its parent.
#[trace("TC-194", "FR-273-AC-11")]
#[test]
fn tc_194_stopped_child_does_not_resume_parent() {
    let resumed = Rc::new(Cell::new(false));
    let resumed_body = Rc::clone(&resumed);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![
            FunctionDeclaration {
                name: "parent".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: Box::new(move |_| {
                    let resumed = Rc::clone(&resumed_body);
                    let mut first = true;
                    Box::new(move |_: &Frame<'_, '_>, _: Option<Value>| {
                        if first {
                            first = false;
                            BodyStep::Call {
                                function: "child".to_string(),
                                arguments: Vec::new(),
                            }
                        } else {
                            resumed.set(true);
                            BodyStep::Return(Outcome::Completed(Value::Boolean(true)))
                        }
                    })
                }),
            },
            FunctionDeclaration {
                name: "child".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(|_, _| Outcome::Undefined(Undefined::DivisionByZero)),
            },
        ],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let result = package
        .call("parent", Vec::new(), &ObjectClosure::default(), &mut meter)
        .unwrap();
    assert!(matches!(
        result.outcome,
        Outcome::Undefined(Undefined::DivisionByZero)
    ));
    assert!(!resumed.get());
    assert_eq!(
        meter.admitted_charge_points(),
        &[ChargePoint::FunctionCall; 2]
    );
}

/// A generated child request for a missing checked function preserves its
/// distinct invariant cause and the already admitted parent's charge.
#[trace("TC-194", "FR-273-AC-8", "FR-273-AC-11")]
#[test]
fn tc_194_missing_child_has_typed_cause_and_does_not_resume_parent() {
    let resumed = Rc::new(Cell::new(false));
    let resumed_body = Rc::clone(&resumed);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "parent".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(move |_| {
                let resumed = Rc::clone(&resumed_body);
                let mut first = true;
                Box::new(move |_: &Frame<'_, '_>, _: Option<Value>| {
                    if first {
                        first = false;
                        BodyStep::Call {
                            function: "absent".to_string(),
                            arguments: Vec::new(),
                        }
                    } else {
                        resumed.set(true);
                        BodyStep::Return(Outcome::Completed(Value::Boolean(true)))
                    }
                })
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let result = package
        .call("parent", Vec::new(), &ObjectClosure::default(), &mut meter)
        .unwrap();
    assert!(matches!(
        result.outcome,
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::UnknownCheckedFunction
        })
    ));
    assert!(!resumed.get());
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
}

/// A generated transition cannot open another scheduler on the same checked
/// package while it is active, even if host code captures the package.
#[trace("TC-194", "FR-273-AC-7", "FR-273-AC-8")]
#[test]
fn tc_194_direct_host_reentry_is_refused_before_nested_charge() {
    let holder: Rc<RefCell<Option<Rc<CheckedPackage>>>> = Rc::new(RefCell::new(None));
    let body_holder = Rc::clone(&holder);
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "reenter".to_string(),
            parameters: Vec::new(),
            result: ValueType::Boolean,
            ieee_requirements: Vec::new(),
            integer_division_consumers: Vec::new(),
            measure_discharged: true,
            body: Box::new(move |_| {
                let holder = Rc::clone(&body_holder);
                Box::new(move |_: &Frame<'_, '_>, _: Option<Value>| {
                    let package = holder.borrow().as_ref().unwrap().clone();
                    let mut nested_meter = Meter::new(UNLIMITED);
                    let nested = package
                        .call(
                            "reenter",
                            Vec::new(),
                            &ObjectClosure::default(),
                            &mut nested_meter,
                        )
                        .unwrap();
                    assert_eq!(nested_meter.consumed(LimitKind::WorkUnits), 0);
                    BodyStep::Return(nested.outcome)
                })
            }),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();
    let package = Rc::new(package);
    *holder.borrow_mut() = Some(Rc::clone(&package));
    let mut meter = Meter::new(UNLIMITED);
    let result = package
        .call("reenter", Vec::new(), &ObjectClosure::default(), &mut meter)
        .unwrap();
    assert!(matches!(
        result.outcome,
        Outcome::Refused(Refusal::CheckedInvariant {
            cause: CheckedInvariantCause::PackageReentry
        })
    ));
    assert_eq!(meter.consumed(LimitKind::WorkUnits), 1);
}

// ---- TC-195: FR-009 negotiation is not an evaluator outcome ---------------

#[trace("TC-195", "FR-273-AC-4")]
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
            body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
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

#[trace("TC-195", "FR-273-AC-4")]
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
/// doctest on `IeeeDisposition` (`src/scalar/ieee.rs`).
#[trace("TC-195", "FR-273-AC-4")]
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
// (`src/scalar/ieee.rs`, run under `cargo test --doc`) rather than by a
// `#[test]` here — a doc comment on a `#[test]` fn in an integration test
// file is never scanned as a doctest, so that placement would have been
// silently vacuous.

#[trace("TC-195", "FR-273-AC-4")]
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
            body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    }
    .check(CheckMode::Linked, CheckingLimits::default())
    .unwrap();

    assert_eq!(
        negotiate_ieee(package.ieee_requirements("f").unwrap(), &backend),
        vec![IeeeDisposition::Supported]
    );

    let objects = ObjectClosure::default();
    let mut meter = Meter::new(UNLIMITED);
    let evaluation = package.call("f", Vec::new(), &objects, &mut meter).unwrap();
    assert!(matches!(
        evaluation.outcome,
        Outcome::Completed(Value::Boolean(true))
    ));
}

// ---- PackageDeclarations::check: what a generated package must supply -----

#[trace("TC-194", "FR-273-AC-1")]
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
                body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
            },
            FunctionDeclaration {
                name: "dup".to_string(),
                parameters: Vec::new(),
                result: ValueType::Boolean,
                ieee_requirements: Vec::new(),
                integer_division_consumers: Vec::new(),
                measure_discharged: true,
                body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(false))),
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

#[trace("TC-194", "FR-273-AC-1")]
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
            body: one_step(|_frame, _arguments| Outcome::Completed(Value::Boolean(true))),
        }],
    };
    let refusals =
        expect_check_refusals(package.check(CheckMode::Linked, CheckingLimits::default()));
    assert_eq!(refusals.len(), 1);
    assert_eq!(refusals[0].cause, CheckCause::UndischargedMeasure);
}
