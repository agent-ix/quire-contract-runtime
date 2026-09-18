// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-189 collection kind algebra: runtime versus authority.
//!
//! Every vector below is evaluated through both boundaries: the runtime port
//! of FR-144 (`quire_contract_runtime::exact::collection`) and the semantic
//! authority `quire_spec_language::value`. Vectors are named after the QSL
//! C-numbered rows of TC-189 they port. C01 (parameter-pair equality) belongs
//! to FR-149, not FR-144, and is folded into `tc_194_equality_matrix.rs`
//! instead. C05 (`exists`, enum ordering iteration) and C11/C12 (collection
//! `convert`, literal-typing ambiguity) name the FR-145 expression machine
//! and the source grammar, both out of scope for this crate (see
//! `src/exact/mod.rs`), and are not ported. C06's reference-keyed portion and
//! C10 are ported using this crate's `Reference<M::Obj>` helpers.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 8] = ["C02", "C03", "C04", "C06", "C07", "C08", "C09", "C10"];

/// Trace: TC-025, FR-008-AC-1
#[test]
fn tc_025_every_tc189_vector_is_evaluated() {
    assert_eq!(EVALUATED.len(), 8);
    println!("TC-189 agreement: {} vectors evaluated", EVALUATED.len());
}

/// `Set<Integer>[0,3]`, `Bag<Integer>[0,3]`, `Sequence<Integer>[0,3]`,
/// `OrderedSet<Integer>[0,3]`, each with a bound large enough that no charge
/// is denied.
macro_rules! int_type {
    ($kind:expr, $max:expr) => {
        CollectionType::new(
            $kind,
            ValueType::Integer,
            CardinalityBound::new(0, $max).unwrap(),
        )
    };
}

/// C02: `sequence[1,1,2]`, `set[1,1,2]`, `bag[1,1,2]` and `orderedSet[2,1,2]`
/// each show their canonical occurrence/member representation.
///
/// Trace: TC-025, FR-008-AC-1
#[test]
fn tc_025_c02_occurrence_and_member_representation() {
    let (sequence, set, bag, ordered_set) = agree! {{
        let ints = |v: &[i128]| v.iter().map(|n| Value::Integer(int(*n))).collect::<Vec<_>>();
        let sequence = form_collection(&int_type!(CollectionKind::Sequence, 3), ints(&[1, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        let set = form_collection(&int_type!(CollectionKind::Set, 3), ints(&[1, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        let bag = form_collection(&int_type!(CollectionKind::Bag, 3), ints(&[1, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        let ordered_set = form_collection(&int_type!(CollectionKind::OrderedSet, 3), ints(&[2, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        (
            format!("{sequence:?}"),
            format!("{set:?}"),
            format!("{bag:?}"),
            format!("{ordered_set:?}"),
        )
    }};
    assert!(sequence.contains("Integer(1)") && sequence.contains("Integer(2)"));
    // Set: members `{1, 2}`, one occurrence each.
    assert_eq!(set.matches("Integer(1)").count(), 1);
    assert_eq!(set.matches("Integer(2)").count(), 1);
    // Bag: `1` with multiplicity 2, `2` with multiplicity 1.
    assert_eq!(bag.matches("Integer(1)").count(), 2);
    assert_eq!(bag.matches("Integer(2)").count(), 1);
    // Ordered set: members `[2, 1]`, first-occurrence order.
    let pos2 = ordered_set.find("Integer(2)").unwrap();
    let pos1 = ordered_set.find("Integer(1)").unwrap();
    assert!(pos2 < pos1, "{ordered_set}");
}

/// C03: cardinality violations both directions, asserting the full
/// `CardinalityOutOfBound` payload, not just the code.
///
/// Trace: TC-025, FR-008-AC-1, FR-008-AC-2
#[test]
fn tc_025_c03_cardinality_out_of_bound_both_directions() {
    let (admitted, set_overflow, bag_overflow, sequence_underflow) = agree! {{
        let ints = |v: &[i128]| v.iter().map(|n| Value::Integer(int(*n))).collect::<Vec<_>>();
        let admitted = form_collection(&int_type!(CollectionKind::Set, 2), ints(&[1, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();

        let set_overflow = form_collection(&int_type!(CollectionKind::Set, 2), ints(&[1, 2, 3]), &mut Meter::new(UNLIMITED)).unwrap();
        let bag_overflow = form_collection(&int_type!(CollectionKind::Bag, 2), ints(&[1, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        let sequence_underflow = form_collection(
            &CollectionType::new(CollectionKind::Sequence, ValueType::Integer, CardinalityBound::new(1, 3).unwrap()),
            Vec::new(),
            &mut Meter::new(UNLIMITED),
        ).unwrap();
        (
            format!("{admitted:?}"),
            format!("{set_overflow:?}"),
            format!("{bag_overflow:?}"),
            format!("{sequence_underflow:?}"),
        )
    }};
    assert!(admitted.contains("Integer(1)") && admitted.contains("Integer(2)"));

    assert_eq!(
        set_overflow,
        format!(
            "{:?}",
            Outcome::<Value>::Refused(Refusal::CardinalityOutOfBound {
                violation: BoundViolation::AboveMaximum,
                kind: CollectionKind::Set,
                bound: CardinalityBound::new(0, 2).unwrap(),
                count: 3,
            })
        )
    );
    assert_eq!(
        bag_overflow,
        format!(
            "{:?}",
            Outcome::<Value>::Refused(Refusal::CardinalityOutOfBound {
                violation: BoundViolation::AboveMaximum,
                kind: CollectionKind::Bag,
                bound: CardinalityBound::new(0, 2).unwrap(),
                count: 3,
            })
        )
    );
    assert_eq!(
        sequence_underflow,
        format!(
            "{:?}",
            Outcome::<Value>::Refused(Refusal::CardinalityOutOfBound {
                violation: BoundViolation::BelowMinimum,
                kind: CollectionKind::Sequence,
                bound: CardinalityBound::new(1, 3).unwrap(),
                count: 0,
            })
        )
    );

    // No retain charge and no collection materializes on a bound refusal.
    let (outcome, charges) = agree! {{
        let ints = |v: &[i128]| v.iter().map(|n| Value::Integer(int(*n))).collect::<Vec<_>>();
        scheduled(UNLIMITED, |m: &mut Meter| {
            form_collection(&int_type!(CollectionKind::Set, 2), ints(&[1, 2, 3]), m).unwrap()
        })
    }};
    assert!(format!("{outcome:?}").contains("CardinalityOutOfBound"));
    assert!(!charges.contains(&ChargePoint::CollectionResultRetain));
}

/// C04: canonical order of a formed set proves the canonical key. Two
/// differently ordered `set[..]` inputs converge on the same ascending key
/// order, for integers, `Option<Integer>` (absent/null-ranked slots before
/// present) and text.
///
/// Trace: TC-025, FR-008-AC-1
#[test]
fn tc_025_c04_canonical_order_proves_the_key() {
    let (order_a, order_b, option_order, text_order) = agree! {{
        let ints = |v: &[i128]| v.iter().map(|n| Value::Integer(int(*n))).collect::<Vec<_>>();
        let a = form_collection(&int_type!(CollectionKind::Set, 3), ints(&[3, 1, 2]), &mut Meter::new(UNLIMITED)).unwrap();
        let b = form_collection(&int_type!(CollectionKind::Set, 3), ints(&[2, 3, 1]), &mut Meter::new(UNLIMITED)).unwrap();

        let option_type = ValueType::option(ValueType::Integer);
        let present = OptionValue::present(ValueType::Integer, Value::Integer(int(1))).unwrap();
        let none = OptionValue::none(ValueType::Integer);
        let option_set = form_collection(
            &CollectionType::new(CollectionKind::Set, option_type, CardinalityBound::new(0, 2).unwrap()),
            vec![present, none],
            &mut Meter::new(UNLIMITED),
        ).unwrap();

        let text_type = TextType::new(0, 4, TextProfile::BinaryUtf8).unwrap();
        let admit = |s: &str| admit_text(&payload(s), &text_type, &mut Meter::new(UNLIMITED)).completed().unwrap();
        let text_set = form_collection(
            &CollectionType::new(CollectionKind::Set, ValueType::Text(text_type), CardinalityBound::new(0, 2).unwrap()),
            vec![Value::Text(admit("b")), Value::Text(admit("a"))],
            &mut Meter::new(UNLIMITED),
        ).unwrap();

        (format!("{a:?}"), format!("{b:?}"), format!("{option_set:?}"), format!("{text_set:?}"))
    }};
    assert_eq!(
        order_a, order_b,
        "both source orders converge on the same canonical key order"
    );
    let p1 = order_a.find("Integer(1)").unwrap();
    let p2 = order_a.find("Integer(2)").unwrap();
    let p3 = order_a.find("Integer(3)").unwrap();
    assert!(p1 < p2 && p2 < p3, "{order_a}");

    // `none` before the present value 1: absent/null slots rank before
    // present in the canonical key.
    let none_pos = option_order.find("None").unwrap();
    let present_pos = option_order.find("Some").unwrap();
    assert!(none_pos < present_pos, "{option_order}");

    let a_pos = text_order.find("\"a\"").unwrap();
    let b_pos = text_order.find("\"b\"").unwrap();
    assert!(a_pos < b_pos, "{text_order}");
}

/// A `record Holder { r: Reference<M::Obj>; }` type environment plus two
/// distinct-universe-local objects `h1`, `h2` whose identity bytes order
/// `h1 < h2`. A macro, not a function, so it expands per-side inside
/// [`agree!`].
macro_rules! holder_env {
    () => {{
        let m_obj = key(60);
        let holder = key(61);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                holder,
                "Holder",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "r",
                    ValueType::Reference(m_obj),
                    Presence::Required,
                )]),
            )],
            [ObjectTypeDeclaration::new(m_obj, "M::Obj", vec![])],
        )
        .unwrap();
        let h1 = env
            .record(
                holder,
                vec![(
                    "r",
                    FieldValue::Present(Value::Reference(reference(1, m_obj, 1))),
                )],
            )
            .unwrap();
        let h2 = env
            .record(
                holder,
                vec![(
                    "r",
                    FieldValue::Present(Value::Reference(reference(1, m_obj, 2))),
                )],
            )
            .unwrap();
        (env, holder, m_obj, h1, h2)
    }};
}

/// C06: a reference-keyed `Holder` set orders and coalesces by the reference
/// identity triple, and an IEEE-bearing element type refuses `Set`/`Bag`
/// formation as `OperatorIneligible` before any charge.
///
/// Trace: TC-025, FR-008-AC-1, FR-008-AC-2
#[test]
fn tc_025_c06_reference_keyed_set_and_ieee_element_refusal() {
    let (members, set_check, bag_check) = agree! {{
        let (env, holder, _m_obj, h1, h2) = holder_env!();
        let holder_type = CollectionType::new(CollectionKind::Set, ValueType::Composite(holder), CardinalityBound::new(0, 2).unwrap());
        let members = form_collection(&holder_type, vec![h2.clone(), h1.clone()], &mut Meter::new(UNLIMITED)).unwrap();

        let float_env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                key(62),
                "F",
                CompositeShape::Record(vec![FieldDeclaration::new(
                    "x",
                    ValueType::Float(IeeeWidth::Binary32),
                    Presence::Required,
                )]),
            )],
            [],
        )
        .unwrap();
        let set_check = float_env.check_type(&ValueType::collection(CollectionType::new(
            CollectionKind::Set,
            ValueType::Float(IeeeWidth::Binary64),
            CardinalityBound::new(0, 2).unwrap(),
        )));
        let bag_check = float_env.check_type(&ValueType::collection(CollectionType::new(
            CollectionKind::Bag,
            ValueType::Composite(key(62)),
            CardinalityBound::new(0, 2).unwrap(),
        )));
        let _ = env;
        (format!("{members:?}"), format!("{set_check:?}"), format!("{bag_check:?}"))
    }};
    // h1's identity bytes (1) order before h2's (2), so the canonical set
    // representation is `[h1, h2]` regardless of source order `h2, h1`.
    let h1_pos = members.find("identity: ObjectIdentity").unwrap();
    let h2_pos = members.rfind("identity: ObjectIdentity").unwrap();
    assert!(h1_pos < h2_pos, "{members}");
    assert!(set_check.contains("OperatorIneligible"), "{set_check}");
    assert!(bag_check.contains("OperatorIneligible"), "{bag_check}");
}

/// C07: the full charge-sequence trace for `Set<Integer>[0,3]` from
/// `set[1, 2, 1]`: three `collection.element`, then, per QSL's own retention
/// order, `collection.member-walk`/`collection.member-test` per candidate
/// against retained members, `collection.bound`, `collection.result-retain`.
/// Eleven work units, three result units, `value_occurrences` high-water 3.
///
/// Trace: TC-025, FR-008-AC-3, FR-008-AC-4
#[test]
fn tc_025_c07_full_construction_charge_trace() {
    let (outcome, charges, consumed) = agree! {{
        let deferred: Vec<Deferred<'_>> = vec![
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(2)))),
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
        ];
        metered(UNLIMITED, |m: &mut Meter| {
            construct_collection(&int_type!(CollectionKind::Set, 3), deferred, m)
        })
    }};
    assert!(format!("{outcome:?}").contains("Integer(1)"));
    use ChargePoint::*;
    assert_eq!(
        charges,
        [
            CollectionElement,
            CollectionElement,
            CollectionElement,
            CollectionMemberWalk,
            CollectionMemberTest,
            CollectionMemberWalk,
            CollectionMemberTest,
            CollectionBound,
            CollectionResultRetain,
        ]
    );
    let work_units = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::WorkUnits)
        .unwrap()];
    let result_units = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::ResultUnits)
        .unwrap()];
    let value_occurrences = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::ValueOccurrences)
        .unwrap()];
    assert_eq!(work_units, 11, "eleven work units");
    assert_eq!(result_units, 3, "three result units");
    assert_eq!(value_occurrences, 3, "value_occurrences high-water of 3");
}

/// C08: C07 under three tight ceilings, each producing the exact `Incomplete`
/// payload with no collection exposed.
///
/// Trace: TC-025, FR-008-AC-2
#[test]
fn tc_025_c08_incomplete_payloads_at_three_ceilings() {
    let (work4, work10, occ2) = agree! {{
        let run = |m: &mut Meter| {
            let deferred: Vec<Deferred<'_>> = vec![
                Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
                Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(2)))),
                Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
            ];
            construct_collection(&int_type!(CollectionKind::Set, 3), deferred, m)
        };
        let work4 = run(&mut Meter::new(ScalarLimits { work_units: 4, ..UNLIMITED }));
        let work10 = run(&mut Meter::new(ScalarLimits { work_units: 10, ..UNLIMITED }));
        let occ2 = run(&mut Meter::new(ScalarLimits { value_occurrences: 2, ..UNLIMITED }));
        (format!("{work4:?}"), format!("{work10:?}"), format!("{occ2:?}"))
    }};
    assert_eq!(
        work4,
        format!(
            "{:?}",
            Outcome::<Value>::Incomplete(incomplete(
                LimitKind::WorkUnits,
                4,
                3,
                int(2),
                ChargePoint::CollectionMemberWalk,
            ))
        )
    );
    assert_eq!(
        work10,
        format!(
            "{:?}",
            Outcome::<Value>::Incomplete(incomplete(
                LimitKind::WorkUnits,
                10,
                10,
                int(1),
                ChargePoint::CollectionResultRetain,
            ))
        )
    );
    assert_eq!(
        occ2,
        format!(
            "{:?}",
            Outcome::<Value>::Incomplete(incomplete(
                LimitKind::ValueOccurrences,
                2,
                2,
                int(3),
                ChargePoint::CollectionResultRetain,
            ))
        )
    );
    assert!(!work4.contains("Collection("));
    assert!(!work10.contains("Collection("));
    assert!(!occ2.contains("Collection("));
}

/// C09: `Sequence<Integer>[0,3]` from `sequence[1,1]` (four work units, three
/// result units, no membership comparison); `Set<Integer>[0,1]` from
/// `set[1,2]` overflows after six work units with no result unit.
///
/// Trace: TC-025, FR-008-AC-1, FR-008-AC-2
#[test]
fn tc_025_c09_sequence_versus_set_overflow() {
    let (
        sequence_result,
        sequence_charges,
        sequence_consumed,
        set_result,
        set_charges,
        set_consumed,
    ) = agree! {{
        let seq_deferred: Vec<Deferred<'_>> = vec![
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
        ];
        let (sequence_result, sequence_charges, sequence_consumed) = metered(UNLIMITED, |m: &mut Meter| {
            construct_collection(&int_type!(CollectionKind::Sequence, 3), seq_deferred, m)
        });

        let set_deferred: Vec<Deferred<'_>> = vec![
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(1)))),
            Box::new(|_m: &mut Meter| Outcome::Completed(Value::Integer(int(2)))),
        ];
        let (set_result, set_charges, set_consumed) = metered(UNLIMITED, |m: &mut Meter| {
            construct_collection(&int_type!(CollectionKind::Set, 1), set_deferred, m)
        });
        (
            format!("{sequence_result:?}"),
            sequence_charges,
            sequence_consumed,
            format!("{set_result:?}"),
            set_charges,
            set_consumed,
        )
    }};
    use ChargePoint::*;
    assert!(sequence_result.contains("Integer(1)"));
    assert_eq!(
        sequence_charges,
        [
            CollectionElement,
            CollectionElement,
            CollectionBound,
            CollectionResultRetain
        ]
    );
    let work = |consumed: &[u64]| {
        consumed[LimitKind::ALL
            .iter()
            .position(|k| *k == LimitKind::WorkUnits)
            .unwrap()]
    };
    let results = |consumed: &[u64]| {
        consumed[LimitKind::ALL
            .iter()
            .position(|k| *k == LimitKind::ResultUnits)
            .unwrap()]
    };
    assert_eq!(work(&sequence_consumed), 4);
    assert_eq!(results(&sequence_consumed), 3);

    assert_eq!(
        set_result,
        format!(
            "{:?}",
            Outcome::<Value>::Refused(Refusal::CardinalityOutOfBound {
                violation: BoundViolation::AboveMaximum,
                kind: CollectionKind::Set,
                bound: CardinalityBound::new(0, 1).unwrap(),
                count: 2,
            })
        )
    );
    assert_eq!(
        set_charges,
        [
            CollectionElement,
            CollectionElement,
            CollectionMemberWalk,
            CollectionMemberTest,
            CollectionBound
        ]
    );
    assert_eq!(work(&set_consumed), 6);
    assert_eq!(results(&set_consumed), 0);
}

/// C10: reference-keyed `Holder` set membership charges (seventeen work
/// units, five result units for three occurrences of `h1, h2, h1`), and a
/// cross-universe reference refuses `ForeignReference` after
/// `collection.element` twice and one `collection.member-walk`, with no
/// `collection.member-test` charge.
///
/// Trace: TC-025, FR-008-AC-2, FR-008-AC-3
#[test]
fn tc_025_c10_reference_keyed_charges_and_foreign_reference() {
    let (outcome, charges, consumed, foreign_outcome, foreign_charges) = agree! {{
        let (env, holder, m_obj, h1, h2) = holder_env!();
        let holder_type = CollectionType::new(CollectionKind::Set, ValueType::Composite(holder), CardinalityBound::new(0, 3).unwrap());
        let (h1a, h2a, h1b) = (h1.clone(), h2.clone(), h1.clone());
        let deferred: Vec<Deferred<'_>> = vec![
            Box::new(move |_m: &mut Meter| Outcome::Completed(h1a)),
            Box::new(move |_m: &mut Meter| Outcome::Completed(h2a)),
            Box::new(move |_m: &mut Meter| Outcome::Completed(h1b)),
        ];
        let (outcome, charges, consumed) = metered(UNLIMITED, |m: &mut Meter| {
            construct_collection(&holder_type, deferred, m)
        });

        let hx = env
            .record(
                holder,
                vec![(
                    "r",
                    FieldValue::Present(Value::Reference(reference(9, m_obj, 1))),
                )],
            )
            .unwrap();
        let cross_type = CollectionType::new(CollectionKind::Set, ValueType::Composite(holder), CardinalityBound::new(0, 3).unwrap());
        let foreign_deferred: Vec<Deferred<'_>> = vec![
            Box::new(move |_m: &mut Meter| Outcome::Completed(h1)),
            Box::new(move |_m: &mut Meter| Outcome::Completed(hx)),
        ];
        let (foreign_outcome, foreign_charges) = scheduled(UNLIMITED, |m: &mut Meter| {
            construct_collection(&cross_type, foreign_deferred, m)
        });
        (
            format!("{outcome:?}"),
            charges,
            consumed,
            format!("{foreign_outcome:?}"),
            foreign_charges,
        )
    }};
    assert!(outcome.contains("identity: ObjectIdentity"));
    use ChargePoint::*;
    assert_eq!(
        charges,
        [
            CollectionElement,
            CollectionElement,
            CollectionElement,
            CollectionMemberWalk,
            CollectionMemberTest,
            CollectionMemberWalk,
            CollectionMemberTest,
            CollectionBound,
            CollectionResultRetain,
        ]
    );
    let work = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::WorkUnits)
        .unwrap()];
    let results = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::ResultUnits)
        .unwrap()];
    assert_eq!(work, 17, "seventeen work units");
    assert_eq!(results, 5, "five result units");

    assert!(
        foreign_outcome.contains("ForeignReference"),
        "{foreign_outcome}"
    );
    assert_eq!(
        foreign_charges,
        [CollectionElement, CollectionElement, CollectionMemberWalk]
    );
}
