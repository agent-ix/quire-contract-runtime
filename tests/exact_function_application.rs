//! Function application under exact semantics (quire-specification/FR-146,
//! FR-273), through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::Cell;
use std::rc::Rc;

use quire_contract_runtime::exact::{
    negotiate_ieee, negotiate_integer_division, ChargePoint, CheckCause, CheckMode, CheckRefusal,
    CheckedPackage, CheckingLimits, DepthAboveMaximum, FunctionDeclaration,
    IeeeBackendCapabilities, IeeeDisposition, IeeeItemRequirement, IeeeOperationKind,
    IeeeUnsupportedCause, IeeeWidth, InputRefusal, Integer, IntegerDivisionConsumer,
    IntegerDivisionDisposition, Meter, NodeKey, ObjectEnvironment, ObjectIdentity, ObjectReference,
    ObjectTypeDeclaration, Outcome, PackageDeclarations, RoundingMode, ScalarLimits,
    TypeEnvironment, UniverseIdentity, Value, ValueType, MAX_CALL_DEPTH,
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

/// `plan_call` alone reproduces every `InputRefusal` with no `Meter` in
/// scope at all: the refusal path never needs one.
///
/// Trace: TC-194, FR-273-AC-2, FR-273-AC-3
#[test]
fn tc_194_plan_call_refuses_identically_with_no_meter_in_scope() {
    let package = identity_package()
        .check(CheckMode::Linked, CheckingLimits::default())
        .unwrap();
    let objects = ObjectEnvironment::default();
    let refusal =
        quire_contract_runtime::exact::plan_call(&package, "identity", &[], &objects).unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 1,
            supplied: 0,
        }
    );
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
}

/// A one-argument-too-many call whose sole argument also carries a dangling
/// reference: arity is decided before any per-argument check runs, so the
/// refusal is `Arity`, never `DanglingReference`. A naive implementation
/// that validated arguments before comparing counts would report the
/// dangling reference instead.
///
/// Trace: TC-194, FR-273-AC-2
#[test]
fn tc_194_arity_is_decided_before_any_per_argument_check() {
    let package = PackageDeclarations {
        types: Default::default(),
        functions: vec![FunctionDeclaration {
            name: "none".to_string(),
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
    let mut meter = Meter::new(UNLIMITED);
    let refusal = package
        .call(
            "none",
            vec![Value::Reference(a_reference())],
            &objects,
            &mut meter,
        )
        .unwrap_err();
    assert_eq!(
        refusal,
        InputRefusal::Arity {
            declared: 0,
            supplied: 1,
        }
    );
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
                let charged = frame.meter(|meter| {
                    meter
                        .admitted_charges()
                        .contains(&ChargePoint::FunctionCall)
                });
                Outcome::Completed(Value::Boolean(charged))
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
                    _ => Outcome::Refused(quire_contract_runtime::exact::Refusal::CheckedInvariant),
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

/// `CheckingLimits::new` refuses a depth above `MAX_CALL_DEPTH`.
///
/// Trace: TC-194, FR-273-AC-1
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
/// Trace: TC-194, FR-273-AC-1
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
        Outcome::Refused(quire_contract_runtime::exact::Refusal::CheckedInvariant)
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

/// Negotiation is a pure function of its inputs: it takes no `Meter` at all
/// (so it cannot change any counter), and its result type has no path into
/// `Outcome`/`InputRefusal` — proven at compile time below.
///
/// Trace: TC-195, FR-273-AC-4
#[test]
fn tc_195_negotiation_takes_no_meter_and_changes_no_counter() {
    let backend = IeeeBackendCapabilities::default();
    let requirement = IeeeItemRequirement {
        width: IeeeWidth::Binary64,
        operation: IeeeOperationKind::Add,
        rounding: RoundingMode::NearestEven,
        requires_finite_proof: false,
    };
    // No Meter is constructed anywhere in this test, and `negotiate_ieee`'s
    // signature (`&[IeeeItemRequirement], &IeeeBackendCapabilities`) takes
    // none: the type system, not a runtime assertion, is FR-273-AC-4's
    // "before any application, without changing any Meter counter."
    let _ = negotiate_ieee(&[requirement], &backend);
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
    for refusal in &refusals {
        assert!(matches!(refusal.cause, CheckCause::AmbiguousName { .. }));
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
