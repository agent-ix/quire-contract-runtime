// SPDX-License-Identifier: AGPL-3.0-or-later
//! Real allocator denials and identity-free semantic controls at RT's exact
//! declared-type consumers. Producer-admitted monitor controls belong to the linked integration leg.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use qsv_failing_allocator::{arm, disarm, FailingAllocator};
use quire_contract_runtime::exact::{
    Body, BodyStep, CheckCause, CheckMode, CheckedPackage, CheckingLimits, Frame,
    FunctionDeclaration, Origin, PackageDeclarations,
};
use quire_exact::{
    CollectionKind, CollectionType, FloatType, IeeeWidth, IllTypedCause, Outcome, Value, ValueType,
};
use quire_semantic_value::declaration::{
    StorageRequest, StorageUnit, TypeCheckError, TypeEnvironment,
};

#[global_allocator]
static ALLOCATOR: FailingAllocator = FailingAllocator;

fn body() -> Body {
    Box::new(|_| {
        Box::new(|_: &Frame<'_, '_>, _: Option<Value>| {
            BodyStep::Return(Outcome::Completed(Value::Boolean(true)))
        })
    })
}

fn declarations(parameter: bool) -> PackageDeclarations {
    PackageDeclarations {
        types: TypeEnvironment::default(),
        functions: vec![FunctionDeclaration {
            name: "identity".into(),
            parameters: if parameter {
                vec![(String::new(), ValueType::Boolean)]
            } else {
                vec![]
            },
            result: ValueType::Boolean,
            body: body(),
            measure_discharged: true,
            ieee_requirements: vec![],
            integer_division_consumers: vec![],
        }],
    }
}

fn checked() -> CheckedPackage {
    match declarations(false).check(CheckMode::Linked, CheckingLimits::default()) {
        Ok(package) => package,
        Err(refusals) => panic!("valid package refused: {refusals:?}"),
    }
}

fn assert_request(request: StorageRequest, denied: Option<usize>) {
    assert!(
        denied.is_some_and(|bytes| bytes > 0),
        "real owner allocation was denied"
    );
    assert_eq!(request.unit(), StorageUnit::AdditionalElements);
    assert_eq!(request.amount(), 1);
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn package_parameter_and_result_denials_preserve_native_request_then_retry() {
    for parameter in [true, false] {
        let input = declarations(parameter);
        // Duplicate-name diagnostic preparation reserves twice; the refusal
        // vector and its nonempty source name are prepared before the owner walk.
        arm(4);
        let result = input.check(CheckMode::Linked, CheckingLimits::default());
        let denied = disarm();
        let refusals = match result {
            Err(refusals) => refusals,
            Ok(_) => panic!("denied type walk published a package"),
        };
        assert_eq!(refusals.len(), 1);
        assert_eq!(
            refusals[0].location.origin,
            Origin::Body {
                function: "identity".into(),
                index: 0
            }
        );
        assert!(refusals[0].location.path.is_empty());
        let CheckCause::TypeCheck(TypeCheckError::Allocation(request)) = refusals[0].cause else {
            panic!("native reservation became semantic: {:?}", refusals[0]);
        };
        assert_request(request, denied);
        assert!(declarations(parameter)
            .check(CheckMode::Linked, CheckingLimits::default())
            .is_ok());
    }
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn expression_parameter_and_result_denials_preserve_native_request_then_retry() {
    let package = checked();
    for parameter in [true, false] {
        let parameters = if parameter {
            vec![(String::new(), ValueType::Boolean)]
        } else {
            vec![]
        };
        let root = body();
        arm(0);
        let result = package.check_expression(parameters, ValueType::Boolean, root);
        let denied = disarm();
        let refusal = match result {
            Err(refusal) => refusal,
            Ok(_) => panic!("denied type walk published an expression"),
        };
        assert_eq!(refusal.location.origin, Origin::Expression);
        assert!(refusal.location.path.is_empty());
        let CheckCause::TypeCheck(TypeCheckError::Allocation(request)) = refusal.cause else {
            panic!("native failure became semantic: {refusal:?}");
        };
        assert_request(request, denied);
        let parameters = if parameter {
            vec![(String::new(), ValueType::Boolean)]
        } else {
            vec![]
        };
        assert!(package
            .check_expression(parameters, ValueType::Boolean, body())
            .is_ok());
    }
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn exact_native_projection_retains_class_unit_and_amount() {
    for request in [
        StorageRequest::new(7, StorageUnit::Bytes),
        StorageRequest::new(19, StorageUnit::AdditionalElements),
    ] {
        for error in [
            TypeCheckError::Allocation(request),
            TypeCheckError::Capacity(request),
        ] {
            arm(0);
            let projected = CheckCause::from(error.clone());
            let denied = disarm();
            assert_eq!(denied, None, "projection allocates no storage");
            assert_eq!(projected, CheckCause::TypeCheck(error));
        }
    }
}

fn sequence(value_type: ValueType) -> ValueType {
    ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        value_type,
        None,
    ))
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn exact_semantic_refusals_keep_cause_location_and_declaration_order() {
    let mut input = declarations(true);
    input.functions[0].parameters[0].1 = ValueType::option(ValueType::Population(None));
    input.functions[0].result = ValueType::Population(None);
    input.functions[0].measure_discharged = false;
    let mut second = declarations(false).functions.remove(0);
    second.name = "second".into();
    second.result = ValueType::Population(None);
    input.functions.push(second);
    let refusals = match input.check(CheckMode::Linked, CheckingLimits::default()) {
        Err(refusals) => refusals,
        Ok(_) => panic!("semantically invalid package admitted"),
    };
    assert_eq!(
        refusals.iter().map(|r| r.cause.clone()).collect::<Vec<_>>(),
        vec![
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
            CheckCause::UndischargedMeasure,
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
        ]
    );
    for (position, refusal) in refusals.into_iter().enumerate() {
        assert_eq!(
            refusal.location.origin,
            Origin::Body {
                function: if position < 3 {
                    "identity".into()
                } else {
                    "second".into()
                },
                index: usize::from(position >= 3)
            }
        );
        assert!(refusal.location.path.is_empty());
    }
    let package = checked();
    for (parameters, result, expected) in [
        (
            vec![("x".into(), ValueType::option(ValueType::Population(None)))],
            ValueType::Population(None),
            IllTypedCause::OperatorIneligible,
        ),
        (
            vec![],
            ValueType::Population(None),
            IllTypedCause::OperatorIneligible,
        ),
    ] {
        let refusal = match package.check_expression(parameters, result, body()) {
            Err(refusal) => refusal,
            Ok(_) => panic!("semantically invalid expression admitted"),
        };
        assert_eq!(refusal.cause, CheckCause::IllTyped(expected));
        assert_eq!(refusal.location.origin, Origin::Expression);
        assert!(refusal.location.path.is_empty());
    }
}

fn ieee_collection(kind: CollectionKind) -> ValueType {
    ValueType::collection(CollectionType::new(
        kind,
        ValueType::option(ValueType::Float(FloatType::exact(IeeeWidth::Binary64))),
        None,
    ))
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn nested_declared_types_and_ieee_sets_and_bags_keep_semantic_parity() {
    let nested = ValueType::option(sequence(ValueType::option(ValueType::Boolean)));
    let mut input = declarations(true);
    input.functions[0].parameters[0].1 = nested.clone();
    input.functions[0].result = nested.clone();
    assert!(input
        .check(CheckMode::Linked, CheckingLimits::default())
        .is_ok());
    let package = checked();
    assert!(package
        .check_expression(
            vec![("nested".into(), nested.clone())],
            nested.clone(),
            body()
        )
        .is_ok());

    for refused in [
        (
            ValueType::option(sequence(ValueType::Population(None))),
            IllTypedCause::OperatorIneligible,
        ),
        (
            ieee_collection(CollectionKind::Set),
            IllTypedCause::OperatorIneligible,
        ),
        (
            ieee_collection(CollectionKind::Bag),
            IllTypedCause::OperatorIneligible,
        ),
    ] {
        let (value_type, expected) = refused;
        for parameter in [true, false] {
            let mut input = declarations(parameter);
            if parameter {
                input.functions[0].parameters[0].1 = value_type.clone();
            } else {
                input.functions[0].result = value_type.clone();
            }
            let refusals = match input.check(CheckMode::Linked, CheckingLimits::default()) {
                Err(refusals) => refusals,
                Ok(_) => panic!("ineligible nested type admitted"),
            };
            assert_eq!(refusals.len(), 1);
            assert_eq!(refusals[0].cause, CheckCause::IllTyped(expected));
            assert_eq!(
                refusals[0].location.origin,
                Origin::Body {
                    function: "identity".into(),
                    index: 0
                }
            );
            assert!(refusals[0].location.path.is_empty());
            let (parameters, result) = if parameter {
                (vec![("nested".into(), value_type.clone())], nested.clone())
            } else {
                (vec![("nested".into(), nested.clone())], value_type.clone())
            };
            let refusal = match package.check_expression(parameters, result, body()) {
                Err(refusal) => refusal,
                Ok(_) => panic!("ineligible nested expression type admitted"),
            };
            assert_eq!(refusal.cause, CheckCause::IllTyped(expected));
            assert_eq!(refusal.location.origin, Origin::Expression);
            assert!(refusal.location.path.is_empty());
        }
    }
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn nested_and_ieee_refusals_keep_parameter_before_result_order() {
    let population = ValueType::option(sequence(ValueType::Population(None)));
    let set = ieee_collection(CollectionKind::Set);
    let bag = ieee_collection(CollectionKind::Bag);
    let mut input = declarations(true);
    input.functions[0].parameters = vec![
        ("population".into(), population.clone()),
        ("ieee".into(), set.clone()),
    ];
    input.functions[0].result = bag.clone();
    input.functions[0].measure_discharged = false;
    let refusals = match input.check(CheckMode::Linked, CheckingLimits::default()) {
        Err(refusals) => refusals,
        Ok(_) => panic!("invalid declarations admitted"),
    };
    assert_eq!(
        refusals.iter().map(|r| r.cause.clone()).collect::<Vec<_>>(),
        vec![
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
            CheckCause::IllTyped(IllTypedCause::OperatorIneligible),
            CheckCause::UndischargedMeasure
        ]
    );
    for refusal in refusals {
        assert_eq!(
            refusal.location.origin,
            Origin::Body {
                function: "identity".into(),
                index: 0
            }
        );
        assert!(refusal.location.path.is_empty());
    }
    let package = checked();
    for (parameters, result, expected) in [
        (
            vec![
                ("population".into(), population),
                ("ieee".into(), set.clone()),
            ],
            bag.clone(),
            IllTypedCause::OperatorIneligible,
        ),
        (
            vec![("ieee".into(), set)],
            bag,
            IllTypedCause::OperatorIneligible,
        ),
    ] {
        let refusal = match package.check_expression(parameters, result, body()) {
            Err(refusal) => refusal,
            Ok(_) => panic!("invalid expression admitted"),
        };
        assert_eq!(refusal.cause, CheckCause::IllTyped(expected));
        assert_eq!(refusal.location.origin, Origin::Expression);
        assert!(refusal.location.path.is_empty());
    }
}
