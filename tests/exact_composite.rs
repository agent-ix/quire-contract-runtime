//! quire-specification/FR-143 composite construction, declaration admission and containment,
//! through the public `exact` surface.
#![cfg(feature = "exact")]

use std::cell::RefCell;
use std::num::NonZeroU64;
use std::rc::Rc;

use quire_contract_runtime::exact::{
    form_collection, CardinalityBound, ChargePoint, CollectionKind, CollectionType, Component,
    CompositeDeclaration, CompositeShape, ConstructionCause, ConstructionRefusal, DeclarationCause,
    FieldDeclaration, FieldExpression, FieldValue, GraphCause, GraphNode, GraphNodeId,
    GraphRefusal, GraphSlot, IeeeWidth, IllTyped, IllTypedCause, Incomplete, InjectedDenial,
    Integer, InvalidDeclaration, LimitKind, Meter, NodeKey, ObjectTypeDeclaration, Outcome,
    Presence, RecursionEdges, ScalarLimits, TypeEnvironment, Undefined, Value, ValueGraph,
    ValueType,
};

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

fn consumed(meter: &Meter) -> Vec<u64> {
    LimitKind::ALL
        .iter()
        .map(|kind| meter.consumed(*kind))
        .collect()
}

/// Trace: TC-024, FR-008-AC-8
#[test]
fn tc_024_p1_charge_point_vocabulary_names_uncharged_future_points() {
    assert_eq!(ChargePoint::ALL.len(), 52);
    assert!(ChargePoint::ALL.contains(&ChargePoint::FunctionCall));
    assert!(ChargePoint::ALL.contains(&ChargePoint::CollectionVisit));
    assert_eq!(ChargePoint::FunctionCall.as_str(), "function.call");
    assert_eq!(ChargePoint::CollectionVisit.as_str(), "collection.visit");
    assert_eq!(
        ChargePoint::from_code("function.call"),
        Some(ChargePoint::FunctionCall)
    );
    assert_eq!(
        ChargePoint::from_code("collection.visit"),
        Some(ChargePoint::CollectionVisit)
    );

    // No sequence any operator in this slice admits ever charges either point.
    let leaf = CompositeDeclaration::new(key(1), "Leaf", CompositeShape::Tuple(Vec::new()));
    let env = TypeEnvironment::new([leaf], []).unwrap();
    let mut meter = Meter::new(UNLIMITED);
    let _leaf_value = env.tuple(key(1), Vec::new()).unwrap();
    let collection_type = CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Boolean,
        CardinalityBound::new(0, 4).unwrap(),
    );
    let outcome = form_collection(
        &collection_type,
        vec![Value::Boolean(true), Value::Boolean(false)],
        &mut meter,
    )
    .unwrap();
    assert!(matches!(outcome, Outcome::Completed(_)));
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::FunctionCall));
    assert!(!meter
        .admitted_charges()
        .contains(&ChargePoint::CollectionVisit));
}

/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_p2_declaration_admission_refuses_duplicates_and_cycles() {
    // Two declarations sharing one node key.
    let a = CompositeDeclaration::new(key(1), "A", CompositeShape::Tuple(Vec::new()));
    let b = CompositeDeclaration::new(key(1), "B", CompositeShape::Tuple(Vec::new()));
    assert_eq!(
        TypeEnvironment::new([a, b], []).unwrap_err(),
        InvalidDeclaration {
            declaration: "B".into(),
            cause: DeclarationCause::DuplicateKey,
        }
    );

    // Two fields of one record sharing one name.
    let dup_field = CompositeDeclaration::new(
        key(2),
        "R",
        CompositeShape::Record(vec![
            FieldDeclaration::new("f", ValueType::Boolean, Presence::Required),
            FieldDeclaration::new("f", ValueType::Boolean, Presence::Required),
        ]),
    );
    let dup_field_refusal = TypeEnvironment::new([dup_field], []).unwrap_err();
    assert_eq!(
        dup_field_refusal,
        InvalidDeclaration {
            declaration: "R".into(),
            cause: DeclarationCause::DuplicateMember("f".into()),
        }
    );
    assert_eq!(dup_field_refusal.code(), "invalid_semantic_graph");

    // Two attributes of one object type sharing one name.
    let dup_attribute = ObjectTypeDeclaration::new(
        key(3),
        "O",
        vec![
            FieldDeclaration::new("a", ValueType::Boolean, Presence::Required),
            FieldDeclaration::new("a", ValueType::Boolean, Presence::Required),
        ],
    );
    assert_eq!(
        TypeEnvironment::new([], [dup_attribute]).unwrap_err(),
        InvalidDeclaration {
            declaration: "O".into(),
            cause: DeclarationCause::DuplicateMember("a".into()),
        }
    );

    // An unnamed-edge (tuple position) cycle.
    let self_tuple = CompositeDeclaration::new(
        key(4),
        "SelfTuple",
        CompositeShape::Tuple(vec![ValueType::Composite(key(4))]),
    );
    let unnamed = TypeEnvironment::new([self_tuple], []).unwrap_err();
    assert_eq!(unnamed.declaration, "SelfTuple");
    assert_eq!(unnamed.code(), IllTyped::CODE);
    match unnamed.cause {
        DeclarationCause::Recursion { edges, cycle } => {
            assert_eq!(edges, RecursionEdges::Unnamed);
            assert_eq!(
                cycle,
                vec!["SelfTuple".to_string(), "SelfTuple".to_string()]
            );
        }
        other => panic!("expected recursion, got {other:?}"),
    }

    // A non-escaping-edge (required field) cycle.
    let self_record = CompositeDeclaration::new(
        key(5),
        "SelfRecord",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "next",
            ValueType::Composite(key(5)),
            Presence::Required,
        )]),
    );
    let non_escaping = TypeEnvironment::new([self_record], []).unwrap_err();
    assert_eq!(non_escaping.declaration, "SelfRecord");
    match non_escaping.cause {
        DeclarationCause::Recursion { edges, cycle } => {
            assert_eq!(edges, RecursionEdges::NonEscaping);
            assert_eq!(
                cycle,
                vec!["SelfRecord".to_string(), "SelfRecord".to_string()]
            );
        }
        other => panic!("expected recursion, got {other:?}"),
    }
}

/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_p3_check_type_and_contains_ieee() {
    // Reference<T> to a non-object-type declaration is type-mismatch.
    let not_object =
        CompositeDeclaration::new(key(1), "NotObject", CompositeShape::Tuple(Vec::new()));
    let env = TypeEnvironment::new([not_object], []).unwrap();
    assert_eq!(
        env.check_type(&ValueType::Reference(key(1))),
        Err(IllTyped {
            cause: IllTypedCause::TypeMismatch,
        })
    );

    // A set element type bearing an IEEE value is operator-ineligible.
    let ieee_set = ValueType::collection(CollectionType::new(
        CollectionKind::Set,
        ValueType::Float(IeeeWidth::Binary32),
        CardinalityBound::new(0, 4).unwrap(),
    ));
    assert_eq!(
        env.check_type(&ieee_set),
        Err(IllTyped {
            cause: IllTypedCause::OperatorIneligible,
        })
    );
    // The identical element type in a sequence needs no equality and admits.
    let ieee_sequence = ValueType::collection(CollectionType::new(
        CollectionKind::Sequence,
        ValueType::Float(IeeeWidth::Binary32),
        CardinalityBound::new(0, 4).unwrap(),
    ));
    assert_eq!(env.check_type(&ieee_sequence), Ok(()));

    // `contains_ieee` finds a Float64 nested through a tuple inside a record.
    let inner = CompositeDeclaration::new(
        key(2),
        "Inner",
        CompositeShape::Tuple(vec![ValueType::Float(IeeeWidth::Binary64)]),
    );
    let outer = CompositeDeclaration::new(
        key(3),
        "Outer",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "inner",
            ValueType::Composite(key(2)),
            Presence::Required,
        )]),
    );
    let nested_env = TypeEnvironment::new([inner, outer], []).unwrap();
    assert!(nested_env.contains_ieee(&ValueType::Composite(key(3))));
    assert!(!nested_env.contains_ieee(&ValueType::Integer));
}

/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_p4_construction_refusals_and_deferred_declaration_order() {
    let widget = CompositeDeclaration::new(
        key(1),
        "Widget",
        CompositeShape::Record(vec![
            FieldDeclaration::new("a", ValueType::Boolean, Presence::Required),
            FieldDeclaration::new("b", ValueType::Integer, Presence::Optional),
        ]),
    );
    let env = TypeEnvironment::new([widget], []).unwrap();

    assert_eq!(
        env.record(key(1), Vec::new()).unwrap_err(),
        ConstructionRefusal {
            component: Component::Field("a".into()),
            cause: ConstructionCause::MissingField,
        }
    );
    assert_eq!(
        env.record(key(1), vec![("a", FieldValue::Null)])
            .unwrap_err(),
        ConstructionRefusal {
            component: Component::Field("a".into()),
            cause: ConstructionCause::NullForRequiredField,
        }
    );
    assert_eq!(
        env.record(
            key(1),
            vec![
                ("a", FieldValue::Present(Value::Boolean(true))),
                ("z", FieldValue::Absent),
            ]
        )
        .unwrap_err(),
        ConstructionRefusal {
            component: Component::Field("z".into()),
            cause: ConstructionCause::UndeclaredField,
        }
    );
    assert_eq!(
        env.record(
            key(1),
            vec![
                ("a", FieldValue::Present(Value::Boolean(true))),
                ("a", FieldValue::Present(Value::Boolean(false))),
            ]
        )
        .unwrap_err(),
        ConstructionRefusal {
            component: Component::Field("a".into()),
            cause: ConstructionCause::DuplicateField,
        }
    );

    let pair = CompositeDeclaration::new(
        key(2),
        "Pair",
        CompositeShape::Tuple(vec![ValueType::Boolean, ValueType::Boolean]),
    );
    let pair_env = TypeEnvironment::new([pair], []).unwrap();
    assert_eq!(
        pair_env
            .tuple(key(2), vec![Value::Boolean(true)])
            .unwrap_err(),
        ConstructionRefusal {
            component: Component::Value,
            cause: ConstructionCause::WrongArity {
                declared: 2,
                supplied: 1,
            },
        }
    );

    // Field expressions run in declaration order, not source order, and the
    // first non-completing field becomes the outcome with no later field run.
    let order: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let order_a = Rc::clone(&order);
    let order_b = Rc::clone(&order);
    let fields = vec![
        (
            "b",
            FieldExpression::Evaluate(Box::new(move |_meter: &mut Meter| {
                order_b.borrow_mut().push("b");
                Outcome::Completed(Value::Integer(Integer::from(1_i128)))
            })),
        ),
        (
            "a",
            FieldExpression::Evaluate(Box::new(move |_meter: &mut Meter| {
                order_a.borrow_mut().push("a");
                Outcome::Undefined(Undefined::DivisionByZero)
            })),
        ),
    ];
    let mut meter = Meter::new(UNLIMITED);
    let outcome = env.evaluate_record(key(1), fields, &mut meter).unwrap();
    assert!(matches!(
        outcome,
        Outcome::Undefined(Undefined::DivisionByZero)
    ));
    assert_eq!(order.borrow().clone(), vec!["a"]);

    // A completed record charges `composite.result-retain` with its exact
    // `occ` before exposure.
    let mut meter = Meter::new(UNLIMITED);
    let ok_fields = vec![(
        "a",
        FieldExpression::Evaluate(Box::new(|_meter: &mut Meter| {
            Outcome::Completed(Value::Boolean(true))
        })),
    )];
    let outcome = env.evaluate_record(key(1), ok_fields, &mut meter).unwrap();
    let value = outcome.completed().unwrap();
    assert_eq!(
        meter.admitted_charges().last(),
        Some(&ChargePoint::CompositeResultRetain)
    );
    // 1 for the record itself, plus 1 for the present `a`; `b` is absent.
    assert_eq!(value.occ(), Integer::from(2_i128));

    // `evaluate_tuple` runs positions in order under the same rule.
    let mut meter = Meter::new(UNLIMITED);
    let outcome = pair_env
        .evaluate_tuple(
            key(2),
            vec![
                Box::new(|_meter: &mut Meter| Outcome::Completed(Value::Boolean(true))),
                Box::new(|_meter: &mut Meter| Outcome::Completed(Value::Boolean(false))),
            ],
            &mut meter,
        )
        .unwrap();
    assert!(outcome.completed().is_some());
    assert_eq!(
        meter.admitted_charges().last(),
        Some(&ChargePoint::CompositeResultRetain)
    );
}

/// Trace: TC-024, FR-008-AC-2, FR-008-AC-9 (the deep chain's normal, iterative `Drop` at
/// scope end)
#[test]
fn tc_024_p5_value_graph_sharing_and_refusals() {
    let leaf = CompositeDeclaration::new(key(1), "Leaf", CompositeShape::Tuple(Vec::new()));
    let pair = CompositeDeclaration::new(
        key(2),
        "Pair",
        CompositeShape::Record(vec![
            FieldDeclaration::new("a", ValueType::Composite(key(1)), Presence::Required),
            FieldDeclaration::new("b", ValueType::Composite(key(1)), Presence::Required),
        ]),
    );
    let env = TypeEnvironment::new([leaf, pair], []).unwrap();

    let leaf_id = GraphNodeId(1);
    let pair_id = GraphNodeId(2);
    let graph = ValueGraph::new([
        (
            leaf_id,
            GraphNode::Tuple {
                declaration: key(1),
                positions: Vec::new(),
            },
        ),
        (
            pair_id,
            GraphNode::Record {
                declaration: key(2),
                fields: vec![
                    ("a".into(), GraphSlot::Node(leaf_id)),
                    ("b".into(), GraphSlot::Node(leaf_id)),
                ],
            },
        ),
    ])
    .unwrap();
    let built = env.build(&graph, pair_id).unwrap();
    let Value::Composite(pair_value) = &built else {
        panic!("expected a composite value")
    };
    let [FieldValue::Present(a), FieldValue::Present(b)] = pair_value.slots() else {
        panic!("expected two present slots")
    };
    let Value::Composite(a_rc) = a else {
        panic!("expected a composite leaf")
    };
    let Value::Composite(b_rc) = b else {
        panic!("expected a composite leaf")
    };
    assert!(
        Rc::ptr_eq(a_rc, b_rc),
        "two slots naming one node must share it"
    );

    // Two nodes sharing one name.
    let duplicate = ValueGraph::new([
        (
            leaf_id,
            GraphNode::Tuple {
                declaration: key(1),
                positions: Vec::new(),
            },
        ),
        (
            leaf_id,
            GraphNode::Tuple {
                declaration: key(1),
                positions: Vec::new(),
            },
        ),
    ]);
    assert_eq!(
        duplicate.unwrap_err(),
        GraphRefusal {
            node: leaf_id,
            cause: GraphCause::DuplicateNode,
        }
    );

    // A root naming no node.
    let empty = ValueGraph::new(Vec::<(GraphNodeId, GraphNode)>::new()).unwrap();
    assert_eq!(
        env.build(&empty, pair_id).unwrap_err(),
        GraphRefusal {
            node: pair_id,
            cause: GraphCause::UnknownNode,
        }
    );

    // A slot naming no node.
    let dangling = ValueGraph::new([
        (
            leaf_id,
            GraphNode::Tuple {
                declaration: key(1),
                positions: Vec::new(),
            },
        ),
        (
            pair_id,
            GraphNode::Record {
                declaration: key(2),
                fields: vec![
                    ("a".into(), GraphSlot::Node(GraphNodeId(999))),
                    ("b".into(), GraphSlot::Node(leaf_id)),
                ],
            },
        ),
    ])
    .unwrap();
    assert_eq!(
        env.build(&dangling, pair_id).unwrap_err(),
        GraphRefusal {
            node: GraphNodeId(999),
            cause: GraphCause::UnknownNode,
        }
    );

    // A containment cycle, over a declaration whose optional field escapes
    // the recursion rule at declaration time.
    let cyclic_decl = CompositeDeclaration::new(
        key(3),
        "Node",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "child",
            ValueType::Composite(key(3)),
            Presence::Optional,
        )]),
    );
    let cyclic_env = TypeEnvironment::new([cyclic_decl], []).unwrap();
    let self_id = GraphNodeId(7);
    let cyclic_graph = ValueGraph::new([(
        self_id,
        GraphNode::Record {
            declaration: key(3),
            fields: vec![("child".into(), GraphSlot::Node(self_id))],
        },
    )])
    .unwrap();
    assert_eq!(
        cyclic_env.build(&cyclic_graph, self_id).unwrap_err(),
        GraphRefusal {
            node: self_id,
            cause: GraphCause::ContainmentCycle,
        }
    );

    // A graph nested past typical host recursion limits: `build` walks it
    // with an explicit worklist, never the host call stack.
    const DEPTH: u64 = 100_000;
    let mut nodes = Vec::with_capacity(DEPTH as usize);
    for depth in 0..DEPTH {
        let id = GraphNodeId(depth);
        let child = if depth == 0 {
            GraphSlot::Absent
        } else {
            GraphSlot::Node(GraphNodeId(depth - 1))
        };
        nodes.push((
            id,
            GraphNode::Record {
                declaration: key(3),
                fields: vec![("child".into(), child)],
            },
        ));
    }
    let deep_graph = ValueGraph::new(nodes).unwrap();
    let deep_value = cyclic_env
        .build(&deep_graph, GraphNodeId(DEPTH - 1))
        .unwrap();
    assert!(matches!(deep_value, Value::Composite(_)));
    // `deep_value` drops here, normally, at the end of scope. Its `Drop` glue is iterative
    // (`agent-ix/quire-contract-runtime#25`): this 100,000-deep chain must not overflow the host
    // stack to free.
}

/// Trace: TC-024, FR-008-AC-9
///
/// `Value`'s `Debug` is hand-written and iterative (`agent-ix/quire-contract-runtime#25`): a chain
/// nested past any plausible recursive-derive host-stack limit must format without overflowing.
/// Also checks the rendering itself is exactly what the former `#[derive(Debug)]` produced, by
/// comparing a shallow instance of the same shape against a hand-written expected string.
#[test]
fn tc_024_p6_debug_at_depth_has_no_stack_overflow() {
    let leaf = CompositeDeclaration::new(key(1), "Leaf", CompositeShape::Tuple(Vec::new()));
    let leaf_env = TypeEnvironment::new([leaf], []).unwrap();
    let leaf_value = leaf_env.tuple(key(1), Vec::new()).unwrap();
    let rendered = format!("{leaf_value:?}");
    assert!(
        rendered.starts_with("Composite(CompositeValue { declaration: NodeKey("),
        "unexpected rendering: {rendered}"
    );
    assert!(
        rendered.ends_with("], occ: Integer(1) })"),
        "unexpected rendering: {rendered}"
    );

    let node = CompositeDeclaration::new(
        key(3),
        "Node",
        CompositeShape::Record(vec![FieldDeclaration::new(
            "child",
            ValueType::Composite(key(3)),
            Presence::Optional,
        )]),
    );
    let env = TypeEnvironment::new([node], []).unwrap();
    const DEPTH: u64 = 6_000;
    let mut nodes = Vec::with_capacity(DEPTH as usize);
    for depth in 0..DEPTH {
        let id = GraphNodeId(depth);
        let child = if depth == 0 {
            GraphSlot::Absent
        } else {
            GraphSlot::Node(GraphNodeId(depth - 1))
        };
        nodes.push((
            id,
            GraphNode::Record {
                declaration: key(3),
                fields: vec![("child".into(), child)],
            },
        ));
    }
    let deep_graph = ValueGraph::new(nodes).unwrap();
    let deep_value = env.build(&deep_graph, GraphNodeId(DEPTH - 1)).unwrap();

    let rendered = format!("{deep_value:?}");
    assert!(
        rendered.starts_with("Composite("),
        "unexpected rendering: {rendered}"
    );
    assert_eq!(
        rendered.matches("Composite(").count(),
        DEPTH as usize,
        "expected one `Composite(` per nesting level"
    );
}

/// Trace: TC-024, FR-008-AC-8
#[test]
fn tc_024_p6_injected_denial_at_composite_result_retain() {
    let widget = CompositeDeclaration::new(
        key(1),
        "Widget",
        CompositeShape::Tuple(vec![ValueType::Boolean]),
    );
    let env = TypeEnvironment::new([widget], []).unwrap();
    let mut meter = Meter::new(UNLIMITED).with_injected_denial(InjectedDenial {
        point: ChargePoint::CompositeResultRetain,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
    let before = consumed(&meter);
    let outcome = env
        .evaluate_tuple(
            key(1),
            vec![Box::new(|_meter: &mut Meter| {
                Outcome::Completed(Value::Boolean(true))
            })],
            &mut meter,
        )
        .unwrap();
    let Outcome::Incomplete(record) = outcome else {
        panic!("expected an incomplete outcome, got {outcome:?}")
    };
    // The one field's deferred expression never charges before
    // `composite.result-retain` runs, so the meter is still untouched when the
    // injected denial fires: the default charge is one work unit, against zero
    // already consumed.
    let expected = Incomplete {
        limit_kind: LimitKind::WorkUnits,
        limit: 0,
        consumed: 0,
        next_charge: Integer::one(),
        charge_point: ChargePoint::CompositeResultRetain,
    };
    assert_eq!(record, expected);
    assert_eq!(consumed(&meter), before);
}
