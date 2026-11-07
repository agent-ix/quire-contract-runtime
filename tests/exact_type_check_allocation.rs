// SPDX-License-Identifier: AGPL-3.0-or-later
//! Real allocator denials at RT's declared-type consumers, using controlled
//! native executable parts. These identities do not claim QSL plan admission.
#![cfg(feature = "exact")]

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use qsv_failing_allocator::{arm, disarm, FailingAllocator};
use quire_contract_runtime::exact::{
    Body, BodyStep, CheckCause, CheckMode, CheckedPackage, CheckingLimits, Frame,
    FunctionDeclaration, Origin, PackageDeclarations,
};
use quire_contract_runtime::monitor::{
    Entries, Expression, ExpressionId, Function, FunctionId, Layout, Operation, Program,
    ProgramParts, ProgramRefusalCode, TypeBinding,
};
use quire_exact::{
    CollectionKind, CollectionType, FloatType, IeeeWidth, IllTypedCause, Integer, MemberId,
    MemberOwner, NodeKey, OptionValue, Outcome, PathSegment, Presence, Rational, Value, ValueType,
};
use quire_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, EnvironmentFailure, FieldDeclaration, StorageRequest,
    StorageUnit, TypeCheckError, TypeEnvironment,
};
use quire_semantic_value::quantity::UnitTable;
use quire_semantic_value::unit::{DimensionNode, NominalDeclaration, UnitGraph, UnitNode};

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

fn key(ordinal: u64) -> NodeKey {
    NodeKey::new(
        "rt-native-control",
        vec![PathSegment::Text("fixture".into())],
        ordinal,
    )
    .unwrap()
}

fn sequence(value_type: ValueType) -> ValueType {
    ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        value_type,
        None,
    ))
}

fn record(ordinal: u64, fields: Vec<(&str, ValueType)>) -> CompositeDeclaration {
    CompositeDeclaration::new(
        key(ordinal),
        format!("type{ordinal}"),
        CompositeShape::Record(
            fields
                .into_iter()
                .map(|(name, ty)| FieldDeclaration::new(name, ty, Presence::Required))
                .collect(),
        ),
    )
}

fn parts() -> ProgramParts {
    let state = ValueType::Composite(key(1));
    let inputs = ValueType::Composite(key(2));
    let scope = ValueType::Composite(key(3));
    let results = sequence(ValueType::Boolean);
    let step = ValueType::Composite(key(4));
    let dimensions = BTreeMap::from([(
        key(10),
        DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
    )]);
    let units = BTreeMap::from([(
        key(11),
        UnitNode::checked(
            key(10),
            None,
            Rational::from_integer(Integer::from(1_u64)),
            Rational::from_integer(Integer::zero()),
            NominalDeclaration::default(),
        )
        .unwrap(),
    )]);
    let types = TypeEnvironment::new(
        [
            record(
                1,
                vec![
                    ("now", ValueType::Integer),
                    ("history_start", ValueType::Integer),
                    ("activations", sequence(ValueType::Integer)),
                    ("event_position", ValueType::Integer),
                ],
            ),
            record(2, vec![]),
            record(3, vec![]),
            CompositeDeclaration::new(
                key(4),
                "step",
                CompositeShape::Tuple(vec![state.clone(), results.clone()]),
            ),
        ],
        [],
    )
    .unwrap()
    .with_units(UnitTable::declared(
        &UnitGraph::from_checked_nodes(&dimensions, &units).unwrap(),
    ));
    let binding = |id, ty| TypeBinding {
        key: key(id),
        value_type: ty,
    };
    let functions = vec![
        Function {
            key: key(20),
            parameters: vec![],
            result: state.clone(),
            body: ExpressionId(2),
        },
        Function {
            key: key(21),
            parameters: vec![state.clone(), ValueType::Integer, inputs.clone()],
            result: step.clone(),
            body: ExpressionId(4),
        },
        Function {
            key: key(22),
            parameters: vec![state.clone(), ValueType::Integer, scope.clone()],
            result: step.clone(),
            body: ExpressionId(4),
        },
        Function {
            key: key(23),
            parameters: vec![state.clone(), ValueType::Integer],
            result: step.clone(),
            body: ExpressionId(4),
        },
    ];
    ProgramParts {
        layout: Layout {
            root: key(30),
            clause: key(31),
            package_lock: vec![1].into(),
            occurrence: vec![1].into(),
            subject: "subject".into(),
            profile: "profile".into(),
            clock_binding: vec![1].into(),
            admitted_order: vec![1].into(),
            progress_context: vec![1].into(),
            state: binding(1, state),
            inputs: binding(2, inputs),
            scope: binding(3, scope),
            results: binding(5, results),
            step_result: binding(4, step),
            entries: Entries {
                init: FunctionId(0),
                on_event: FunctionId(1),
                on_watermark: FunctionId(2),
                on_gap: FunctionId(3),
            },
            counter: "counter".into(),
            tick_unit: key(11),
            time_dimension: key(10),
            counter_width_bits: 32,
            max_reading_gap: 100,
            max_event_rate: Rational::from_integer(Integer::from(1_u64)),
            uncertainty: Rational::from_integer(Integer::zero()),
            buffers: vec![],
            activation_capacity: Integer::zero(),
        },
        types,
        functions,
        expressions: vec![
            Expression {
                key: key(40),
                body_path: vec![],
                value_type: ValueType::option(ValueType::Integer),
                operation: Operation::Literal(
                    OptionValue::present(ValueType::Integer, Value::Integer(Integer::zero()))
                        .unwrap(),
                ),
            },
            Expression {
                key: key(41),
                body_path: vec![],
                value_type: sequence(ValueType::Integer),
                operation: Operation::Sequence { elements: vec![] },
            },
            Expression {
                key: key(42),
                body_path: vec![],
                value_type: ValueType::Composite(key(1)),
                operation: Operation::Record {
                    fields: [
                        ("now", 5),
                        ("history_start", 5),
                        ("activations", 1),
                        ("event_position", 5),
                    ]
                    .into_iter()
                    .map(|(name, id)| {
                        (
                            MemberId::new(MemberOwner::Node(key(1)), name).unwrap(),
                            ExpressionId(id),
                        )
                    })
                    .collect(),
                },
            },
            Expression {
                key: key(43),
                body_path: vec![],
                value_type: sequence(ValueType::Boolean),
                operation: Operation::Sequence { elements: vec![] },
            },
            Expression {
                key: key(44),
                body_path: vec![],
                value_type: ValueType::Composite(key(4)),
                operation: Operation::Tuple {
                    positions: vec![ExpressionId(2), ExpressionId(3)],
                },
            },
            Expression {
                key: key(45),
                body_path: vec![],
                value_type: ValueType::Integer,
                operation: Operation::Literal(Value::Integer(Integer::zero())),
            },
        ],
    }
}

#[trace("TC-200", "FR-276-AC-13")]
#[trace("TC-200", "FR-276-AC-14")]
#[test]
fn monitor_type_walk_denials_preserve_membership_and_native_request_then_retry() {
    // Five Layout walks precede RT's function-key scratch. Twelve function
    // signature walks then precede the first expression type check.
    for (successful, source) in [(0, 1), (6, 20), (7, 21), (18, 40)] {
        let input = parts();
        arm(successful);
        let result = Program::try_new(input);
        let denied = disarm();
        let refusal = match result {
            Err(refusal) => refusal,
            Ok(_) => panic!("denied type walk published a Program"),
        };
        assert_eq!(
            refusal.code,
            ProgramRefusalCode::Membership,
            "allocation index {successful}: {refusal:?}"
        );
        assert_eq!(refusal.node, Some(key(source)));
        let Some(EnvironmentFailure::Allocation(request)) = refusal.membership_failure else {
            panic!("native request lost: {refusal:?}")
        };
        assert_request(request, denied);
        assert!(Program::try_new(parts()).is_ok());
    }
}

#[trace("TC-200", "FR-276-AC-13")]
#[trace("TC-200", "FR-276-AC-14")]
#[test]
fn monitor_rt_owned_scratch_denial_stays_storage() {
    let input = parts();
    arm(5);
    let result = Program::try_new(input);
    let denied = disarm();
    let refusal = match result {
        Err(refusal) => refusal,
        Ok(_) => panic!("denied scratch published a Program"),
    };
    assert!(denied.is_some());
    assert_eq!(refusal.code, ProgramRefusalCode::Storage);
    assert_eq!(refusal.membership_failure, None);
    assert!(Program::try_new(parts()).is_ok());
}

#[trace("TC-200", "FR-276-AC-13")]
#[trace("TC-200", "FR-276-AC-14")]
#[test]
fn monitor_literal_membership_denial_retains_native_environment_failure() {
    let input = parts();
    arm(19);
    let result = Program::try_new(input);
    let denied = disarm();
    let refusal = match result {
        Err(refusal) => refusal,
        Ok(_) => panic!("denied literal membership published a Program"),
    };
    assert_eq!(refusal.code, ProgramRefusalCode::Membership);
    assert_eq!(refusal.node, Some(key(40)));
    let Some(EnvironmentFailure::Allocation(request)) = refusal.membership_failure else {
        panic!("literal membership cause lost: {refusal:?}")
    };
    assert_request(request, denied);
    assert!(Program::try_new(parts()).is_ok());
}

#[trace("TC-194", "FR-273-AC-14")]
#[test]
fn exact_semantic_refusals_keep_cause_location_and_declaration_order() {
    let mut input = declarations(true);
    input.functions[0].parameters[0].1 = ValueType::Composite(key(90));
    input.functions[0].result = ValueType::Population(None);
    input.functions[0].measure_discharged = false;
    let refusals = match input.check(CheckMode::Linked, CheckingLimits::default()) {
        Err(refusals) => refusals,
        Ok(_) => panic!("semantically invalid package admitted"),
    };
    assert_eq!(
        refusals.iter().map(|r| r.cause.clone()).collect::<Vec<_>>(),
        vec![
            CheckCause::IllTyped(IllTypedCause::TypeMismatch),
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
            vec![("x".into(), ValueType::Composite(key(90)))],
            ValueType::Population(None),
            IllTypedCause::TypeMismatch,
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

#[trace("TC-200", "FR-276-AC-14")]
#[test]
fn monitor_semantic_type_refusals_keep_locus_and_layout_precedence() {
    for (which, value_type, source) in [
        (0, ValueType::Composite(key(90)), 1),
        (0, ValueType::Population(None), 1),
        (0, ieee_collection(CollectionKind::Set), 1),
        (1, ieee_collection(CollectionKind::Bag), 40),
        (1, ValueType::Population(None), 40),
        (2, ValueType::Composite(key(90)), 50),
    ] {
        let mut input = parts();
        match which {
            0 => input.layout.state.value_type = value_type,
            1 => input.expressions[0].value_type = value_type,
            2 => input.functions.push(Function {
                key: key(50),
                parameters: vec![value_type],
                result: ValueType::Boolean,
                body: ExpressionId(0),
            }),
            _ => unreachable!(),
        }
        let refusal = match Program::try_new(input) {
            Err(refusal) => refusal,
            Ok(_) => panic!("invalid declared type admitted"),
        };
        assert_eq!(refusal.code, ProgramRefusalCode::TypeMismatch);
        assert_eq!(refusal.node, Some(key(source)));
        assert_eq!(refusal.membership_failure, None);
    }
    let mut input = parts();
    input.layout.package_lock = Box::new([]);
    input.layout.state.value_type = ValueType::Population(None);
    let refusal = match Program::try_new(input) {
        Err(refusal) => refusal,
        Ok(_) => panic!("invalid layout admitted"),
    };
    assert_eq!(refusal.code, ProgramRefusalCode::InvalidLayout);
    assert_eq!(refusal.node, Some(key(30)));
    assert_eq!(refusal.membership_failure, None);
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
            ValueType::option(sequence(ValueType::Composite(key(90)))),
            IllTypedCause::TypeMismatch,
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
    let unknown = ValueType::option(sequence(ValueType::Composite(key(90))));
    let set = ieee_collection(CollectionKind::Set);
    let bag = ieee_collection(CollectionKind::Bag);
    let mut input = declarations(true);
    input.functions[0].parameters = vec![
        ("unknown".into(), unknown.clone()),
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
            CheckCause::IllTyped(IllTypedCause::TypeMismatch),
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
            vec![("unknown".into(), unknown), ("ieee".into(), set.clone())],
            bag.clone(),
            IllTypedCause::TypeMismatch,
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
