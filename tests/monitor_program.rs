// SPDX-License-Identifier: AGPL-3.0-or-later
//! Construction controls; these do not qualify monitor execution or the producer handoff.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::monitor::{
    Entries, Expression, ExpressionId, Function, FunctionId, Layout, Operation, Program,
    ProgramParts, ProgramRefusalCode, TypeBinding,
};
use quire_exact::{
    CollectionKind, CollectionType, FieldValue, Identifier, Integer, NodeKey, PathSegment,
    Presence, Rational, Value, ValueType,
};
use quire_semantic_value::{
    declaration::{
        CompositeDeclaration, CompositeShape, FieldDeclaration, TypeEnvironment,
        UnionMemberDeclaration,
    },
    quantity::UnitTable,
    unit::{DimensionNode, NominalDeclaration, UnitGraph, UnitNode},
};
use std::{collections::BTreeMap, rc::Rc};

fn key(name: &str) -> NodeKey {
    NodeKey::new("monitor-test", vec![PathSegment::Text(name.into())], 0).unwrap()
}
fn record(name: &str, fields: Vec<(&str, ValueType)>) -> CompositeDeclaration {
    CompositeDeclaration::new(
        key(name),
        name,
        CompositeShape::Record(
            fields
                .into_iter()
                .map(|(name, ty)| FieldDeclaration::new(name, ty, Presence::Required))
                .collect(),
        ),
    )
}
fn parts() -> ProgramParts {
    let state = ValueType::Composite(key("state"));
    let inputs = ValueType::Composite(key("inputs"));
    let scope = ValueType::Composite(key("scope"));
    let results_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Composite(key("result")),
        None,
    );
    let results = ValueType::collection(results_type.clone());
    let step_result = ValueType::Composite(key("step"));
    let types = TypeEnvironment::new(
        [
            record("state", vec![("now", ValueType::Integer)]),
            record("inputs", vec![]),
            record("scope", vec![]),
            record(
                "gap_payload",
                vec![
                    ("gap", ValueType::Integer),
                    ("max_reading_gap", ValueType::Integer),
                ],
            ),
            record(
                "limit_payload",
                vec![
                    ("limit", ValueType::String),
                    ("value", ValueType::Integer),
                    ("setting", ValueType::String),
                ],
            ),
            record(
                "instance",
                vec![
                    ("subject", ValueType::String),
                    ("subformula", ValueType::Integer),
                    ("origin", ValueType::Integer),
                ],
            ),
            CompositeDeclaration::new(
                key("outcome"),
                "outcome",
                CompositeShape::Union(
                    [
                        ("Failed", vec![ValueType::Composite(key("gap_payload"))]),
                        ("False", vec![]),
                        (
                            "Incomplete",
                            vec![ValueType::Composite(key("limit_payload"))],
                        ),
                        ("Indeterminate", vec![]),
                        ("Pending", vec![]),
                        ("True", vec![]),
                    ]
                    .into_iter()
                    .map(|(name, positions)| {
                        UnionMemberDeclaration::resolved(Identifier::new(name).unwrap(), positions)
                    })
                    .collect(),
                ),
            ),
            record(
                "result",
                vec![
                    ("instance", ValueType::Composite(key("instance"))),
                    ("outcome", ValueType::Composite(key("outcome"))),
                ],
            ),
            CompositeDeclaration::new(
                key("step"),
                "step",
                CompositeShape::Tuple(vec![state.clone(), results.clone()]),
            ),
        ],
        [],
    )
    .expect("admit diagnostic nominal declarations");
    let zero = Rational::from_integer(Integer::zero());
    let one = Rational::from_integer(Integer::one());
    let dimensions = BTreeMap::from([(
        key("Time"),
        DimensionNode::checked(vec![], NominalDeclaration::default()).unwrap(),
    )]);
    let units = BTreeMap::from([(
        key("tick"),
        UnitNode::checked(
            key("Time"),
            None,
            one.clone(),
            zero.clone(),
            NominalDeclaration::default(),
        )
        .unwrap(),
    )]);
    let graph = UnitGraph::from_checked_nodes(&dimensions, &units).unwrap();
    let types = types.with_units(UnitTable::declared(&graph));
    let state_value = types
        .record(
            key("state"),
            vec![("now", FieldValue::Present(Value::Integer(Integer::zero())))],
        )
        .unwrap();
    let result_value = quire_exact::from_admitted(results_type, vec![]);
    let step_value = types
        .tuple(key("step"), vec![state_value.clone(), result_value])
        .unwrap();
    let function_keys = [key("init"), key("event"), key("watermark"), key("gap")];
    let signatures = [
        vec![],
        vec![state.clone(), ValueType::Integer, inputs.clone()],
        vec![state.clone(), ValueType::Integer, scope.clone()],
        vec![state.clone(), ValueType::Integer],
    ];
    let mut expressions = Vec::new();
    let mut functions = Vec::new();
    for (index, (source, parameters)) in function_keys.into_iter().zip(signatures).enumerate() {
        let (value_type, value) = if index == 0 {
            (state.clone(), state_value.clone())
        } else {
            (step_result.clone(), step_value.clone())
        };
        expressions.push(Expression {
            key: source.clone(),
            body_path: vec![0],
            value_type: value_type.clone(),
            operation: Operation::Literal(value),
        });
        functions.push(Function {
            key: source,
            parameters,
            result: value_type,
            body: ExpressionId(index),
        });
    }
    let binding = |name, value_type| TypeBinding {
        key: key(name),
        value_type,
    };
    ProgramParts {
        layout: Layout {
            root: NodeKey::new(
                "monitor-test",
                vec![
                    PathSegment::Text("monitor_plan".into()),
                    PathSegment::Node(key("gap")),
                ],
                0,
            )
            .unwrap(),
            clause: key("clause"),
            package_lock: Box::from(&b"selected-lock"[..]),
            subject: "selected-subject".into(),
            occurrence: Box::from(&b"occurrence"[..]),
            profile: "timestamped-event".into(),
            state: binding("state", state),
            inputs: binding("inputs", inputs),
            scope: binding("scope", scope),
            results: binding("results", results),
            step_result: binding("step", step_result),
            entries: Entries {
                init: FunctionId(0),
                on_event: FunctionId(1),
                on_watermark: FunctionId(2),
                on_gap: FunctionId(3),
            },
            counter: "counter-a".into(),
            tick_unit: key("tick"),
            time_dimension: key("Time"),
            counter_width_bits: 16,
            max_reading_gap: 100,
            max_event_rate: one,
            uncertainty: zero,
            buffers: vec![],
        },
        types,
        functions,
        expressions,
    }
}
fn refused(parts: ProgramParts, expected: ProgramRefusalCode) {
    match Program::try_new(parts) {
        Err(error) => assert_eq!(error.code, expected),
        Ok(_) => panic!("invalid construction must refuse"),
    }
}

#[trace("TC-200", "interface-002-AC-1")]
#[test]
fn retained_program_outlives_preparation_and_keeps_four_entry_keys() {
    let program = {
        let input = parts();
        Program::try_new(input).unwrap()
    };
    let second = Rc::clone(&program);
    drop(program);
    assert_eq!(
        second.layout().root,
        NodeKey::new(
            "monitor-test",
            vec![
                PathSegment::Text("monitor_plan".into()),
                PathSegment::Node(key("gap"))
            ],
            0
        )
        .unwrap()
    );
    assert!(second.function(FunctionId(100)).is_none());
    assert!(second.expression(ExpressionId(100)).is_none());
    assert_eq!(second.layout().package_lock.as_ref(), b"selected-lock");
    assert_eq!(second.layout().occurrence.as_ref(), b"occurrence");
    let gap = second.function(second.layout().entries.on_gap).unwrap();
    assert_eq!(gap.key, key("gap"));
    assert_eq!(gap.parameters.len(), 2);
    assert!(matches!(
        second.expression(gap.body).unwrap().operation,
        Operation::Literal(_)
    ));
    assert!(second.types().composite(key("state")).is_some());
}

#[trace("TC-200", "interface-002-AC-2")]
#[test]
fn missing_and_mistyped_fourth_entry_refuse_before_any_execution() {
    let mut missing = parts();
    missing.layout.entries.on_gap = FunctionId(100);
    refused(missing, ProgramRefusalCode::InvalidLayout);
    let mut wrong = parts();
    wrong.functions[3].parameters.pop();
    refused(wrong, ProgramRefusalCode::TypeMismatch);
}

#[trace("TC-200", "interface-002-AC-3")]
#[test]
fn duplicate_source_keys_invalid_edges_and_cycles_refuse() {
    let mut duplicate = parts();
    duplicate.functions[1].key = duplicate.functions[0].key.clone();
    refused(duplicate, ProgramRefusalCode::DuplicateKey);
    let mut invalid = parts();
    invalid.functions[0].body = ExpressionId(100);
    refused(invalid, ProgramRefusalCode::InvalidReference);
    let mut cycle = parts();
    cycle.expressions[0].operation = Operation::If {
        condition: ExpressionId(0),
        when_true: ExpressionId(0),
        when_false: ExpressionId(0),
    };
    // Type mismatch at the condition has precedence over graph traversal.
    refused(cycle, ProgramRefusalCode::TypeMismatch);
    let mut cycle = parts();
    cycle.expressions.push(Expression {
        key: key("unreached-cycle"),
        body_path: vec![],
        value_type: ValueType::Boolean,
        operation: Operation::If {
            condition: ExpressionId(4),
            when_true: ExpressionId(4),
            when_false: ExpressionId(4),
        },
    });
    refused(cycle, ProgramRefusalCode::ExpressionCycle);
}

#[trace("TC-200", "interface-002-AC-3")]
#[test]
fn argument_ownership_is_checked_through_the_expression_closure() {
    let mut wrong = parts();
    wrong.expressions[0] = Expression {
        key: key("foreign-argument"),
        body_path: vec![],
        value_type: wrong.layout.state.value_type.clone(),
        operation: Operation::Argument {
            function: FunctionId(1),
            position: 0,
        },
    };
    refused(wrong, ProgramRefusalCode::ForeignArgument);
}

#[trace("TC-200", "interface-002-AC-2")]
#[test]
fn counter_premises_keep_width_64_and_reject_out_of_width_gap() {
    let mut full_width = parts();
    full_width.layout.counter_width_bits = 64;
    full_width.layout.max_reading_gap = u64::MAX;
    assert!(Program::try_new(full_width).is_ok());
    for width in [0, 65] {
        let mut wrong = parts();
        wrong.layout.counter_width_bits = width;
        refused(wrong, ProgramRefusalCode::InvalidLayout);
    }
    let mut wrong = parts();
    wrong.layout.max_reading_gap = 65536;
    refused(wrong, ProgramRefusalCode::InvalidLayout);
    let mut wrong = parts();
    wrong.layout.uncertainty = Rational::from_integer(Integer::from(-1_i64));
    refused(wrong, ProgramRefusalCode::InvalidLayout);
    let mut wrong = parts();
    wrong.layout.time_dimension = key("Length");
    refused(wrong, ProgramRefusalCode::InvalidLayout);
}

#[trace("TC-200", "interface-002-AC-4")]
#[test]
fn compatible_entry_roles_may_share_one_function() {
    let mut input = parts();
    input.layout.scope = input.layout.inputs.clone();
    input.layout.entries.on_watermark = input.layout.entries.on_event;
    let program = Program::try_new(input).unwrap();
    assert_eq!(
        program.layout().entries.on_event,
        program.layout().entries.on_watermark
    );
}

#[trace("TC-200", "interface-002-AC-3")]
#[test]
fn shared_expression_nodes_and_distinct_inline_paths_are_not_cycles_or_duplicate_keys() {
    let mut input = parts();
    let original = input.expressions[0].key.clone();
    input.expressions.push(Expression {
        key: original.clone(),
        body_path: vec![1],
        value_type: ValueType::Boolean,
        operation: Operation::Literal(Value::Boolean(true)),
    });
    input.expressions.push(Expression {
        key: original.clone(),
        body_path: vec![2],
        value_type: input.layout.state.value_type.clone(),
        operation: Operation::If {
            condition: ExpressionId(4),
            when_true: ExpressionId(0),
            when_false: ExpressionId(0),
        },
    });
    input.functions[0].body = ExpressionId(5);
    let program = Program::try_new(input).unwrap();
    assert_eq!(program.expression(ExpressionId(0)).unwrap().key, original);
    assert_eq!(program.expression(ExpressionId(5)).unwrap().body_path, [2]);
}

#[trace("TC-200", "interface-002-AC-3")]
#[test]
fn deeply_shared_inline_arena_validates_on_a_small_native_stack() {
    std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(|| {
            let mut input = parts();
            input.expressions.push(Expression {
                key: key("condition"),
                body_path: vec![],
                value_type: ValueType::Boolean,
                operation: Operation::Literal(Value::Boolean(true)),
            });
            let mut previous = ExpressionId(0);
            for position in 0..20_000 {
                let next = ExpressionId(input.expressions.len());
                input.expressions.push(Expression {
                    key: key("init"),
                    body_path: vec![position + 1],
                    value_type: input.layout.state.value_type.clone(),
                    operation: Operation::If {
                        condition: ExpressionId(4),
                        when_true: previous,
                        when_false: previous,
                    },
                });
                previous = next;
            }
            input.functions[0].body = previous;
            let program = Program::try_new(input).unwrap();
            assert_eq!(program.function(FunctionId(0)).unwrap().body, previous);
        })
        .unwrap()
        .join()
        .unwrap();
}
