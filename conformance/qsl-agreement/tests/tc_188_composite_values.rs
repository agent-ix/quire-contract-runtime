// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-188 record, tuple and recursive values: runtime versus authority.
//!
//! Every vector below is evaluated through both boundaries: the runtime port
//! of FR-143 (`quire_contract_runtime::exact::composite`,
//! `quire_contract_runtime::exact::containment`) and the semantic authority
//! `quire_spec_language::value`. Vectors are named after the QSL R-numbered
//! rows of TC-188 they port; rows that name packages, imports, `function`
//! declarations, the source grammar or a bound model snapshot are out of
//! scope for this crate (see `src/exact/mod.rs`) and are not ported.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 9] = [
    "R01", "R05a", "R05b", "R06", "R07", "R08", "R09", "R10", "graph",
];

/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_every_tc188_vector_is_evaluated() {
    assert_eq!(EVALUATED.len(), 9);
    println!("TC-188 agreement: {} vectors evaluated", EVALUATED.len());
}

/// `record P { a: Integer; b: Integer?; }`. A macro, not a function: it must
/// expand inside an [`agree!`] block so its bare type names resolve against
/// whichever side (`qsl_side` or `rt_side`) is active at the call site,
/// exactly like the shared `support` helpers.
macro_rules! p_env {
    () => {{
        let p = key(1);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                p,
                "P",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new("b", ValueType::Integer, Presence::Optional),
                ]),
            )],
            [],
        )
        .unwrap();
        (env, p)
    }};
}

/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_r01_field_order_is_not_identity() {
    let (both, orders) = agree! {{
        let (env, p) = p_env!();
        let a1 = env.record(p, vec![("a", FieldValue::Present(Value::Integer(int(1))))]);
        let a2 = env.record(
            p,
            vec![
                ("a", FieldValue::Present(Value::Integer(int(1)))),
                ("b", FieldValue::Present(Value::Integer(int(2)))),
            ],
        );
        let b2 = env.record(
            p,
            vec![
                ("b", FieldValue::Present(Value::Integer(int(2)))),
                ("a", FieldValue::Present(Value::Integer(int(1)))),
            ],
        );
        (format!("{a1:?}"), (format!("{a2:?}"), format!("{b2:?}")))
    }};
    assert!(both.contains("Present(Integer"));
    assert_eq!(
        orders.0, orders.1,
        "declaration order, not source order, is identity"
    );
}

/// Every slot-presence combination (Absent/Null/Present) of two optional
/// fields on `record Q { a: Integer?; b: Integer?; }`.
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_every_slot_presence_combination() {
    let rendered = agree! {{
        let q = key(2);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                q,
                "Q",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("a", ValueType::Integer, Presence::Optional),
                    FieldDeclaration::new("b", ValueType::Integer, Presence::Optional),
                ]),
            )],
            [],
        )
        .unwrap();
        let slot = |n: u8| match n {
            0 => FieldValue::Absent,
            1 => FieldValue::Null,
            _ => FieldValue::Present(Value::Integer(int(7))),
        };
        let mut out = Vec::new();
        for a in 0..3_u8 {
            for b in 0..3_u8 {
                let result = env.record(q, vec![("a", slot(a)), ("b", slot(b))]);
                out.push(format!("{result:?}"));
            }
        }
        out
    }};
    assert_eq!(rendered.len(), 9);
    // Every combination is a well-typed record: `Integer?` admits all three
    // slot states, so no combination refuses.
    assert!(rendered.iter().all(|r| r.starts_with("Ok(")));
}

/// Every `ConstructionRefusal` cause with its originating `Component`.
///
/// Trace: TC-024, FR-008-AC-1, FR-008-AC-2
#[test]
fn tc_024_r07_construction_refusals_name_their_component() {
    let (record_causes, tuple_causes, option_cause) = agree! {{
        let (env, p) = p_env!();
        let missing = env.record(p, vec![]);
        let null_required = env.record(p, vec![("a", FieldValue::Null)]);
        let undeclared = env.record(
            p,
            vec![
                ("a", FieldValue::Present(Value::Integer(int(1)))),
                ("c", FieldValue::Present(Value::Integer(int(2)))),
            ],
        );
        let duplicate = env.record(
            p,
            vec![
                ("a", FieldValue::Present(Value::Integer(int(1)))),
                ("a", FieldValue::Present(Value::Integer(int(2)))),
            ],
        );
        let type_mismatch = env.record(p, vec![("a", FieldValue::Present(Value::Boolean(true)))]);
        let unknown_decl = env.record(key(99), vec![]);

        let t = key(3);
        let tuple_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                t,
                "T",
                CompositeShape::Tuple(vec![ValueType::Integer, ValueType::Integer]),
            )],
            [],
        )
        .unwrap();
        let wrong_arity = tuple_env.tuple(t, vec![Value::Integer(int(1))]);
        let tuple_type_mismatch =
            tuple_env.tuple(t, vec![Value::Boolean(true), Value::Integer(int(1))]);

        let bad_payload = OptionValue::present(ValueType::Integer, Value::Boolean(true));

        (
            [
                format!("{missing:?}"),
                format!("{null_required:?}"),
                format!("{undeclared:?}"),
                format!("{duplicate:?}"),
                format!("{type_mismatch:?}"),
                format!("{unknown_decl:?}"),
            ],
            [format!("{wrong_arity:?}"), format!("{tuple_type_mismatch:?}")],
            format!("{bad_payload:?}"),
        )
    }};
    assert!(record_causes[0].contains("Field(\"a\")") && record_causes[0].contains("MissingField"));
    assert!(
        record_causes[1].contains("Field(\"a\")")
            && record_causes[1].contains("NullForRequiredField")
    );
    assert!(
        record_causes[2].contains("Field(\"c\")") && record_causes[2].contains("UndeclaredField")
    );
    assert!(
        record_causes[3].contains("Field(\"a\")") && record_causes[3].contains("DuplicateField")
    );
    assert!(record_causes[4].contains("Field(\"a\")") && record_causes[4].contains("TypeMismatch"));
    assert!(record_causes[5].contains("Value") && record_causes[5].contains("UnknownDeclaration"));
    assert!(tuple_causes[0].contains("WrongArity"));
    assert!(tuple_causes[1].contains("Position(0)") && tuple_causes[1].contains("TypeMismatch"));
    assert!(option_cause.contains("Payload") && option_cause.contains("TypeMismatch"));
}

/// The recursion rule admits finite recursion and a model reference, and
/// refuses a reference to a non-object-type declaration.
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_r05_recursion_rule_admits() {
    let outcomes = agree! {{
        let list = key(10);
        let list_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                list,
                "List",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("head", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new("tail", ValueType::Composite(list), Presence::Optional),
                ]),
            )],
            [],
        );

        let n = key(11);
        let bound = CardinalityBound::new(0, 2).unwrap();
        let n_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                n,
                "N",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "kids",
                    ValueType::collection(CollectionType::new(
                        CollectionKind::Sequence,
                        ValueType::Composite(n),
                        bound,
                    )),
                    Presence::Required,
                )]),
            )],
            [],
        );

        let (a, b) = (key(12), key(13));
        let ab_env = TypeEnvironment::new(
            [
                CompositeDeclaration::new(
                    a,
                    "A",
                    CompositeShape::Record(vec![FieldDeclaration::new(
                        "b",
                        ValueType::Composite(b),
                        Presence::Required,
                    )]),
                ),
                CompositeDeclaration::new(
                    b,
                    "B",
                    CompositeShape::Record(vec![FieldDeclaration::new(
                        "a",
                        ValueType::Composite(a),
                        Presence::Optional,
                    )]),
                ),
            ],
            [],
        );

        let m_obj = key(14);
        let o = key(15);
        let o_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                o,
                "O",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "r",
                    ValueType::Reference(m_obj),
                    Presence::Required,
                )]),
            )],
            [ObjectTypeDeclaration::new(m_obj, "M::Obj", vec![])],
        );

        let p = key(16);
        let bad = key(17);
        let bad_env = TypeEnvironment::new(
            [
                CompositeDeclaration::new(p, "P", CompositeShape::Record(vec![])),
                CompositeDeclaration::new(
                    bad,
                    "Bad",
                    CompositeShape::Record(vec![FieldDeclaration::new(
                        "r",
                        ValueType::Reference(p),
                        Presence::Required,
                    )]),
                ),
            ],
            [],
        );

        [
            format!("{:?}", list_env.is_ok()),
            format!("{:?}", n_env.is_ok()),
            format!("{:?}", ab_env.is_ok()),
            format!("{:?}", o_env.is_ok()),
            format!("{:?}", bad_env.err().map(|e| e.cause)),
        ]
    }};
    assert_eq!(outcomes[0], "true");
    assert_eq!(outcomes[1], "true");
    assert_eq!(outcomes[2], "true");
    assert_eq!(outcomes[3], "true");
    assert!(
        outcomes[4].contains("Type(TypeMismatch)"),
        "{}",
        outcomes[4]
    );
}

/// The recursion rule refuses an unnamed-edge cycle (a self-referential
/// required field and a tuple) and a non-escaping cycle (a minimum-one
/// sequence of itself).
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_r06_recursion_rule_refuses() {
    let outcomes = agree! {{
        let loop_key = key(20);
        let loop_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                loop_key,
                "Loop",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "next",
                    ValueType::Composite(loop_key),
                    Presence::Required,
                )]),
            )],
            [],
        );

        let m = key(21);
        let bound = CardinalityBound::new(1, 2).unwrap();
        let m_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                m,
                "M",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "kids",
                    ValueType::collection(CollectionType::new(
                        CollectionKind::Sequence,
                        ValueType::Composite(m),
                        bound,
                    )),
                    Presence::Required,
                )]),
            )],
            [],
        );

        let pair = key(22);
        let pair_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                pair,
                "Pair",
                CompositeShape::Tuple(vec![
                    ValueType::Integer,
                    ValueType::option(ValueType::Composite(pair)),
                ]),
            )],
            [],
        );

        [
            format!("{:?}", loop_env.err().map(|e| e.cause)),
            format!("{:?}", m_env.err().map(|e| e.cause)),
            format!("{:?}", pair_env.err().map(|e| e.cause)),
        ]
    }};
    // `Loop{next: Loop}` is a required *named* field, so it is excluded from
    // the unnamed-edge subgraph and caught only by the non-escaping one.
    assert!(
        outcomes[0].contains("Recursion") && outcomes[0].contains("NonEscaping"),
        "{}",
        outcomes[0]
    );
    assert!(
        outcomes[1].contains("Recursion") && outcomes[1].contains("NonEscaping"),
        "{}",
        outcomes[1]
    );
    assert!(
        outcomes[2].contains("Recursion") && outcomes[2].contains("Unnamed"),
        "{}",
        outcomes[2]
    );
}

/// `TypeEnvironment::new` refuses `DuplicateKey` and `DuplicateMember`.
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_r08_declaration_admission_refusals() {
    let outcomes = agree! {{
        let shared = key(30);
        let dup_key = TypeEnvironment::new(
            [
                CompositeDeclaration::new(shared, "X", CompositeShape::Record(vec![])),
                CompositeDeclaration::new(shared, "Y", CompositeShape::Record(vec![])),
            ],
            [],
        );
        let dup_member = TypeEnvironment::new(
            [CompositeDeclaration::new(
                key(31),
                "Z",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
                ]),
            )],
            [],
        );
        [
            format!("{:?}", dup_key.err().map(|e| e.cause)),
            format!("{:?}", dup_member.err().map(|e| e.cause)),
        ]
    }};
    assert_eq!(outcomes[0], "Some(DuplicateKey)");
    assert!(outcomes[1].contains("DuplicateMember"));
}

/// `TypeEnvironment::check_type`/`contains_ieee`: an unknown declaration, a
/// `Reference<T>` to a non-object-type declaration, and an IEEE-bearing set
/// element type refuse; the same element type in a sequence is admitted.
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_r09_check_type_refusals() {
    let outcomes = agree! {{
        let (env, p) = p_env!();
        let unknown = env.check_type(&ValueType::Composite(key(200)));
        let ref_to_record = env.check_type(&ValueType::Reference(p));
        let ieee_set = env.check_type(&ValueType::collection(CollectionType::new(
            CollectionKind::Set,
            ValueType::Float(IeeeWidth::Binary32),
            CardinalityBound::new(0, 2).unwrap(),
        )));
        let ieee_sequence = env.check_type(&ValueType::collection(CollectionType::new(
            CollectionKind::Sequence,
            ValueType::Float(IeeeWidth::Binary32),
            CardinalityBound::new(0, 2).unwrap(),
        )));
        [
            format!("{unknown:?}"),
            format!("{ref_to_record:?}"),
            format!("{ieee_set:?}"),
            format!("{ieee_sequence:?}"),
        ]
    }};
    assert!(outcomes[0].contains("Err"));
    assert!(outcomes[1].contains("TypeMismatch"));
    assert!(outcomes[2].contains("OperatorIneligible"));
    assert_eq!(outcomes[3], "Ok(())");
}

/// `evaluate_record` runs field expressions in declaration order under the
/// first-stopped rule: the field declared first that does not complete stops
/// construction before any later field expression runs, and a fully
/// completed record charges exactly `composite.result-retain` with `occ`.
///
/// Trace: TC-024, FR-008-AC-3, FR-008-AC-4
#[test]
fn tc_024_r10_evaluate_record_first_stop_and_retain() {
    use std::cell::Cell;
    use std::rc::Rc;

    let (stopped, never_ran, schedule, denied) = agree! {{
        let two = key(40);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                two,
                "Two",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new("b", ValueType::Integer, Presence::Required),
                ]),
            )],
            [],
        )
        .unwrap();

        let ran = Rc::new(Cell::new(false));
        let ran_b = ran.clone();
        let fields: Vec<(&str, FieldExpression<'_>)> = vec![
            (
                "b",
                FieldExpression::Evaluate(Box::new(move |_m: &mut Meter| {
                    ran_b.set(true);
                    Outcome::Completed(Value::Integer(int(2)))
                })),
            ),
            (
                "a",
                FieldExpression::Evaluate(Box::new(|_m: &mut Meter| {
                    Outcome::Refused(Refusal::CheckedInvariant)
                })),
            ),
        ];
        let stopped = env.evaluate_record(two, fields, &mut Meter::new(UNLIMITED));

        let run = |m: &mut Meter| {
            let fields: Vec<(&str, FieldExpression<'_>)> = vec![
                (
                    "a",
                    FieldExpression::Evaluate(Box::new(|_m: &mut Meter| {
                        Outcome::Completed(Value::Integer(int(1)))
                    })),
                ),
                (
                    "b",
                    FieldExpression::Evaluate(Box::new(|_m: &mut Meter| {
                        Outcome::Completed(Value::Integer(int(2)))
                    })),
                ),
            ];
            env.evaluate_record(two, fields, m).unwrap()
        };
        (
            format!("{stopped:?}"),
            !ran.get(),
            scheduled(UNLIMITED, run),
            denials(UNLIMITED, run),
        )
    }};
    assert!(stopped.contains("Refused"));
    assert!(
        never_ran,
        "field b must never run once field a stops construction"
    );
    assert_eq!(schedule.1, [ChargePoint::CompositeResultRetain]);
    assert_eq!(denied.len(), 1);
    let (point, _, outcome, results) = &denied[0];
    assert_eq!(*point, ChargePoint::CompositeResultRetain);
    assert_eq!(
        format!("{outcome:?}"),
        format!(
            "{:?}",
            Outcome::<Value>::Incomplete(work_denied(0, ChargePoint::CompositeResultRetain))
        )
    );
    assert_eq!(*results, 0);

    // The exact incomplete payload under a one-result-unit ceiling.
    let limited = metered(
        ScalarLimits {
            result_units: 1,
            ..UNLIMITED
        },
        |m: &mut Meter| {
            let two = key(41);
            let env = TypeEnvironment::new(
                [CompositeDeclaration::new(
                    two,
                    "Two",
                    CompositeShape::Record(vec![
                        FieldDeclaration::new("a", ValueType::Integer, Presence::Required),
                        FieldDeclaration::new("b", ValueType::Integer, Presence::Optional),
                    ]),
                )],
                [],
            )
            .unwrap();
            let fields: Vec<(&str, FieldExpression<'_>)> = vec![(
                "a",
                FieldExpression::Evaluate(Box::new(|_m: &mut Meter| {
                    Outcome::Completed(Value::Integer(int(1)))
                })),
            )];
            env.evaluate_record(two, fields, m).unwrap()
        },
    );
    assert_eq!(
        format!("{:?}", limited.0),
        format!(
            "{:?}",
            Outcome::<Value>::Incomplete(incomplete(
                LimitKind::ResultUnits,
                1,
                0,
                int(2),
                ChargePoint::CompositeResultRetain
            ))
        )
    );
}

/// `evaluate_tuple` checks arity before running any position, then runs
/// positions in order under the first-stopped rule.
///
/// Trace: TC-024, FR-008-AC-3, FR-008-AC-4
#[test]
fn tc_024_r10b_evaluate_tuple_arity_and_order() {
    let (wrong_arity, ran_second) = agree! {{
        let t = key(42);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                t,
                "T",
                CompositeShape::Tuple(vec![ValueType::Integer, ValueType::Integer]),
            )],
            [],
        )
        .unwrap();
        let positions: Vec<Deferred<'_>> =
            vec![Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1))))];
        let wrong_arity = env.evaluate_tuple(t, positions, &mut Meter::new(UNLIMITED));

        let positions: Vec<Deferred<'_>> = vec![
            Box::new(|_m: &mut Meter| Outcome::Refused(Refusal::CheckedInvariant)),
            Box::new(|_m: &mut Meter| {
                // Never reached: `evaluate_tuple` never allocates the second
                // closure once the first position stops construction, so the
                // captured flag stays observable only through the harness
                // itself calling this deferred body, which does not happen.
                Outcome::Completed(Value::Integer(int(9)))
            }),
        ];
        let stopped = env.evaluate_tuple(t, positions, &mut Meter::new(UNLIMITED));
        (format!("{wrong_arity:?}"), format!("{stopped:?}"))
    }};
    assert!(wrong_arity.contains("WrongArity"));
    assert!(ran_second.contains("Refused"));
}

/// `ValueGraph`: bottom-up sharing, `DuplicateNode`, `UnknownNode` and
/// `ContainmentCycle`.
///
/// Trace: TC-024, FR-008-AC-1
#[test]
fn tc_024_graph_sharing_and_refusals() {
    let outcomes = agree! {{
        let list = key(50);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                list,
                "List",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("head", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new("tail", ValueType::Composite(list), Presence::Optional),
                ]),
            )],
            [],
        )
        .unwrap();

        // Two roots that both point at the same leaf node: sharing renders
        // identically without duplicating the leaf's construction.
        let leaf = GraphNodeId(0);
        let root_a = GraphNodeId(1);
        let root_b = GraphNodeId(2);
        let nodes = [
            (
                leaf,
                GraphNode::Record {
                    declaration: list,
                    fields: vec![("head".into(), GraphSlot::Value(Value::Integer(int(0))))],
                },
            ),
            (
                root_a,
                GraphNode::Record {
                    declaration: list,
                    fields: vec![
                        ("head".into(), GraphSlot::Value(Value::Integer(int(1)))),
                        ("tail".into(), GraphSlot::Node(leaf)),
                    ],
                },
            ),
            (
                root_b,
                GraphNode::Record {
                    declaration: list,
                    fields: vec![
                        ("head".into(), GraphSlot::Value(Value::Integer(int(2)))),
                        ("tail".into(), GraphSlot::Node(leaf)),
                    ],
                },
            ),
        ];
        let graph = ValueGraph::new(nodes.clone()).unwrap();
        let shared = (
            format!("{:?}", env.build(&graph, root_a)),
            format!("{:?}", env.build(&graph, root_b)),
        );

        let duplicate = ValueGraph::new([nodes[0].clone(), nodes[0].clone()]);

        let unknown = env.build(&ValueGraph::new([]).unwrap(), GraphNodeId(99));

        // A self-referential `tail` closes a containment cycle.
        let cyclic = GraphNodeId(3);
        let cycle_graph = ValueGraph::new([(
            cyclic,
            GraphNode::Record {
                declaration: list,
                fields: vec![
                    ("head".into(), GraphSlot::Value(Value::Integer(int(0)))),
                    ("tail".into(), GraphSlot::Node(cyclic)),
                ],
            },
        )])
        .unwrap();
        let cycle = env.build(&cycle_graph, cyclic);

        (
            shared.0,
            shared.1,
            format!("{:?}", duplicate.err().map(|e| e.cause)),
            format!("{:?}", unknown.err().map(|e| e.cause)),
            format!("{:?}", cycle.err().map(|e| e.cause)),
        )
    }};
    assert!(outcomes.0.contains("Integer(1)"));
    assert!(outcomes.1.contains("Integer(2)"));
    assert_eq!(outcomes.2, "Some(DuplicateNode)");
    assert_eq!(outcomes.3, "Some(UnknownNode)");
    assert_eq!(outcomes.4, "Some(ContainmentCycle)");
}
