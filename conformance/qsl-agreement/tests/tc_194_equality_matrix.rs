// SPDX-License-Identifier: AGPL-3.0-or-later
//! QSpec TC-194 complete equality matrix: runtime versus authority.
//!
//! Every vector below is evaluated through both boundaries: the runtime port
//! of FR-149 (`quire_contract_runtime::exact::equality`) and the semantic
//! authority `quire_spec_language::value`. Vectors are named after the QSL
//! E-numbered rows of TC-194 they port. E17-E19 (IEEE numeric-equality,
//! total-order equivalence and bit identity) name a different operator
//! (`compare_ieee`/FR-148), not `=`/FR-149's `CheckedEquality`: `=` on any
//! IEEE-bearing type refuses `operator-ineligible` before any charge, proven
//! by E28 instead. E20 (undefined/refused/incomplete propagation through
//! nested construction) and E22 (reference-keyed Set/Bag rank-matched pair
//! counts) name the FR-145 expression machine or add no new mechanism beyond
//! what E10-E16/E21 already exercise, and are not ported. E26's final row
//! (`let x = convert<T>(r) in x = d`) composes FR-140's general decimal
//! rounding operator with an already-matched-type equality and is a
//! different mechanism than `admits_equality_conversion`'s closed table; it
//! is not ported here.

#[macro_use]
mod support;

use support::rt_side::*;

/// Vectors evaluated through both boundaries.
const EVALUATED: [&str; 12] = [
    "E02", "E03", "E04", "E05", "E06", "E07", "E16", "E21", "E23", "E24", "E26", "E28",
];

/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_every_tc194_vector_is_evaluated() {
    assert_eq!(EVALUATED.len(), 12);
    println!("TC-194 agreement: {} vectors evaluated", EVALUATED.len());
}

/// An empty type environment, sufficient for every equality check whose
/// operand types are not composites.
macro_rules! empty_env {
    () => {
        TypeEnvironment::new([], []).unwrap()
    };
}

/// E02: the Plan schedule over `Integer`, and `Int[0,2]` `convert<Rational[0,2;1,1]>`
/// admitted on the left operand but refused without it.
///
/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_e02_plan_schedule_and_admitted_conversion() {
    let (equal, unequal, converted, unconverted) = agree! {{
        let env = empty_env!();
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Integer),
                EqualityOperand::typed(ValueType::Integer),
            )
            .unwrap();
        assert_eq!(checked.schedule(), EqualitySchedule::Plan);
        let equal = checked.evaluate(&Value::Integer(int(1)), &Value::Integer(int(1)), &mut Meter::new(UNLIMITED));
        let unequal = checked.evaluate(&Value::Integer(int(1)), &Value::Integer(int(2)), &mut Meter::new(UNLIMITED));

        let bound = ValueType::Int(IntegerInterval::new(int(0), int(2)).unwrap());
        let target = ValueType::Rational(rational_type("0", "2", "1", "1"));
        let checked_conv = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::converted(bound, target.clone()),
                EqualityOperand::typed(target),
            )
            .unwrap();
        let converted = checked_conv.evaluate(&Value::Integer(int(1)), &Value::Rational(ratio(1, 1)), &mut Meter::new(UNLIMITED));

        let unbound = ValueType::Int(IntegerInterval::new(int(0), int(2)).unwrap());
        let target2 = ValueType::Rational(rational_type("0", "2", "1", "1"));
        let unconverted = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(unbound),
            EqualityOperand::typed(target2),
        );
        (
            format!("{equal:?}"),
            format!("{unequal:?}"),
            format!("{converted:?}"),
            format!("{unconverted:?}"),
        )
    }};
    assert_eq!(equal, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert_eq!(unequal, format!("{:?}", Outcome::<bool>::Completed(false)));
    assert_eq!(converted, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert!(unconverted.contains("TypeMismatch"), "{unconverted}");
}

/// E03: differing decimal scales compare numerically equal
/// (`(10,1)` = `(100,2)`, both spelling `1.0`), and `Decimal[0,100;0,2]`
/// admits `convert<Rational[0,100;1,100]>`.
///
/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_e03_differing_decimal_scales() {
    let (equal_scales, unequal, converted) = agree! {{
        let env = empty_env!();
        let decimal_ty = ValueType::Decimal(decimal_type(0, 100, 0, 2, RoundingMode::Exact));
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(decimal_ty.clone()),
                EqualityOperand::typed(decimal_ty.clone()),
            )
            .unwrap();
        // `(10, 1)` (1.0 at scale 1) and `(100, 2)` (1.00 at scale 2): a
        // different decimal scale on each side, numerically equal.
        let equal_scales = checked.evaluate(&Value::Decimal(dec(10, 1)), &Value::Decimal(dec(100, 2)), &mut Meter::new(UNLIMITED));
        let unequal = checked.evaluate(&Value::Decimal(dec(10, 1)), &Value::Decimal(dec(11, 1)), &mut Meter::new(UNLIMITED));

        let source = ValueType::Decimal(decimal_type(0, 100, 0, 2, RoundingMode::Exact));
        let target = ValueType::Rational(rational_type("0", "100", "1", "100"));
        let checked_conv = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::converted(source, target.clone()),
                EqualityOperand::typed(target),
            )
            .unwrap();
        let before = format!("{:?}", Value::Decimal(dec(10, 1)));
        let converted = checked_conv.evaluate(&Value::Decimal(dec(10, 1)), &Value::Rational(ratio(1, 1)), &mut Meter::new(UNLIMITED));
        let after = format!("{:?}", Value::Decimal(dec(10, 1)));
        assert_eq!(before, after, "the source decimal is not mutated by conversion");
        (format!("{equal_scales:?}"), format!("{unequal:?}"), format!("{converted:?}"))
    }};
    assert_eq!(
        equal_scales,
        format!("{:?}", Outcome::<bool>::Completed(true))
    );
    assert_eq!(unequal, format!("{:?}", Outcome::<bool>::Completed(false)));
    assert_eq!(converted, format!("{:?}", Outcome::<bool>::Completed(true)));
}

/// E04: `Rational[0,1;1,3]` does not admit `convert<Decimal[..]>` at the
/// equality operand table, because the source denominator bound (`1,3`)
/// does not collapse to exactly one; this is a static refusal, not `false`.
///
/// Trace: TC-026, FR-008-AC-2
#[test]
fn tc_026_e04_ill_typed_conversion_denominator_bound() {
    let checked = agree! {{
        let env = empty_env!();
        let source = ValueType::Rational(rational_type("0", "1", "1", "3"));
        let target = ValueType::Decimal(decimal_type(0, 100, 2, 2, RoundingMode::NearestEven));
        format!(
            "{:?}",
            env.check_equality(
                EqualityOperator::Equal,
                EqualityOperand::converted(source, target.clone()),
                EqualityOperand::typed(target),
            )
        )
    }};
    assert!(checked.contains("TypeMismatch"), "{checked}");
}

/// E05: the Quantity schedule over one declared unit, and incompatible
/// dimensions refuse.
///
/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_e05_quantity_schedule_and_incompatible_dimensions() {
    let (equal, incompatible) = agree! {{
        let env = empty_env!();
        let fx = fixture();
        let m = fx.unit("m");
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Quantity(m.clone())),
                EqualityOperand::typed(ValueType::Quantity(m.clone())),
            )
            .unwrap();
        assert_eq!(checked.schedule(), EqualitySchedule::Quantity);
        let equal = checked.evaluate(&Value::Quantity(fx.qi(1, "m")), &Value::Quantity(fx.qi(1, "m")), &mut Meter::new(UNLIMITED));

        let s = fx.unit("s");
        let incompatible = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Quantity(m)),
            EqualityOperand::typed(ValueType::Quantity(s)),
        );
        (format!("{equal:?}"), format!("{incompatible:?}"))
    }};
    assert_eq!(equal, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert!(
        incompatible.contains("IncompatibleDimensions"),
        "{incompatible}"
    );
}

/// E06: the Text schedule over NFC-equal composed/decomposed spellings, and
/// distinct text profiles refuse at `check_equality`.
///
/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_e06_text_schedule_and_distinct_profiles() {
    let (equal, distinct) = agree! {{
        let env = empty_env!();
        let nfc = ValueType::Text(text_type(0, 64, TextProfile::Nfc));
        let checked = env
            .check_equality(EqualityOperator::Equal, EqualityOperand::typed(nfc.clone()), EqualityOperand::typed(nfc))
            .unwrap();
        assert_eq!(checked.schedule(), EqualitySchedule::Text);
        let equal = checked.evaluate(
            &Value::Text(text("\u{e9}", TextProfile::Nfc)),
            &Value::Text(text("e\u{301}", TextProfile::Nfc)),
            &mut Meter::new(UNLIMITED),
        );

        let distinct = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Text(text_type(0, 64, TextProfile::Nfc))),
            EqualityOperand::typed(ValueType::Text(text_type(0, 64, TextProfile::BinaryUtf8))),
        );
        (format!("{equal:?}"), format!("{distinct:?}"))
    }};
    assert_eq!(equal, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert!(distinct.contains("DistinctTextProfiles"), "{distinct}");
}

/// E07: the Enum schedule over one declaration's members, and distinct
/// declarations refuse at `check_equality`.
///
/// Trace: TC-026, FR-008-AC-1
#[test]
fn tc_026_e07_enum_schedule_and_distinct_declarations() {
    let (equal, unequal, distinct) = agree! {{
        let env = empty_env!();
        let color = enum_declaration("Color194", false, &["blue", "green", "red"]).unwrap();
        let level = enum_declaration("Level194", false, &["high", "low"]).unwrap();
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Enum(color.key())),
                EqualityOperand::typed(ValueType::Enum(color.key())),
            )
            .unwrap();
        assert_eq!(checked.schedule(), EqualitySchedule::Enum);
        let equal = checked.evaluate(
            &Value::Enum(color.value("red").unwrap()),
            &Value::Enum(color.value("red").unwrap()),
            &mut Meter::new(UNLIMITED),
        );
        let unequal = checked.evaluate(
            &Value::Enum(color.value("red").unwrap()),
            &Value::Enum(color.value("blue").unwrap()),
            &mut Meter::new(UNLIMITED),
        );
        let distinct = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Enum(color.key())),
            EqualityOperand::typed(ValueType::Enum(level.key())),
        );
        (format!("{equal:?}"), format!("{unequal:?}"), format!("{distinct:?}"))
    }};
    assert_eq!(equal, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert_eq!(unequal, format!("{:?}", Outcome::<bool>::Completed(false)));
    assert!(distinct.contains("DistinctEnumDeclarations"), "{distinct}");
}

/// E16: reference identity equality within one universe, distinct identity
/// inequality, and cross-universe refusal as `ForeignReference` after
/// exactly `equality.plan-form` (two work units) and before
/// `equality.plan`, with no further charge.
///
/// Trace: TC-026, FR-008-AC-2, FR-008-AC-3
#[test]
fn tc_026_e16_reference_identity_and_foreign_reference() {
    let (equal, unequal, foreign_outcome, foreign_charges, foreign_work) = agree! {{
        let m_obj = key(80);
        let env = TypeEnvironment::new([], [ObjectTypeDeclaration::new(m_obj, "M::Obj", vec![])]).unwrap();
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Reference(m_obj)),
                EqualityOperand::typed(ValueType::Reference(m_obj)),
            )
            .unwrap();
        let r1 = Value::Reference(reference(1, m_obj, 1));
        let r1_again = Value::Reference(reference(1, m_obj, 1));
        let r2 = Value::Reference(reference(1, m_obj, 2));
        let foreign = Value::Reference(reference(9, m_obj, 1));

        let equal = checked.evaluate(&r1, &r1_again, &mut Meter::new(UNLIMITED));
        let unequal = checked.evaluate(&r1, &r2, &mut Meter::new(UNLIMITED));
        let (foreign_outcome, foreign_charges, foreign_consumed) =
            metered(UNLIMITED, |m: &mut Meter| checked.evaluate(&r1, &foreign, m));
        let work = foreign_consumed[LimitKind::ALL.iter().position(|k| *k == LimitKind::WorkUnits).unwrap()];
        (format!("{equal:?}"), format!("{unequal:?}"), format!("{foreign_outcome:?}"), foreign_charges, work)
    }};
    assert_eq!(equal, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert_eq!(unequal, format!("{:?}", Outcome::<bool>::Completed(false)));
    assert!(
        foreign_outcome.contains("ForeignReference"),
        "{foreign_outcome}"
    );
    assert_eq!(foreign_charges, [ChargePoint::EqualityPlanForm]);
    assert_eq!(foreign_work, 2, "two work units");
}

/// `record List { head: Integer; tail: List?; }`, keyed at `list_key`. A
/// macro, not a function, so it expands per-side inside [`agree!`].
macro_rules! list_env {
    ($list_key:expr) => {{
        TypeEnvironment::new(
            [CompositeDeclaration::new(
                $list_key,
                "List",
                CompositeShape::Record(vec![
                    FieldDeclaration::new("head", ValueType::Integer, Presence::Required),
                    FieldDeclaration::new(
                        "tail",
                        ValueType::Composite($list_key),
                        Presence::Optional,
                    ),
                ]),
            )],
            [],
        )
        .unwrap()
    }};
}

/// A `List` chain over `values`, tail-first, the last value's `tail` absent.
/// A macro, not a function: it must expand inside an [`agree!`] block so its
/// bare type names resolve against whichever side is active at the call
/// site, exactly like [`list_env!`].
macro_rules! build_list {
    ($env:expr, $list_key:expr, $values:expr) => {{
        let mut tail: Option<Value> = None;
        for value in $values.iter().rev() {
            let tail_field = match tail.take() {
                Some(node) => FieldValue::Present(node),
                None => FieldValue::Absent,
            };
            tail = Some(
                $env.record(
                    $list_key,
                    vec![
                        (
                            "head",
                            FieldValue::Present(Value::Integer(int(i128::from(*value)))),
                        ),
                        ("tail", tail_field),
                    ],
                )
                .unwrap(),
            );
        }
        tail.unwrap()
    }};
}

/// E21: two equal depth-8 `List` chains (`occ = 16` each), once as
/// duplicated independently-constructed trees and once with the tail chain
/// Rc-shared between operands: both admit `true` after exactly
/// `equality.plan-form` (32 work units), `equality.plan`, 17 `equality.pair`
/// events and `equality.result-retain` (51 work units total), the exact
/// count [`plan_equality`] (which takes no [`Meter`] and so cannot charge)
/// independently reports. Denying the 17th pair or the retain charge each
/// end incomplete with no Boolean.
///
/// Trace: TC-026, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4
#[test]
fn tc_026_e21_deep_nested_pair_count_duplicated_and_shared() {
    let (dup_result, shared_result, plan_pairs, charges, consumed, denied_pair, denied_retain) = agree! {{
        let limits = limits([4, 0, 0, 0, 0, 0, 0, 17, 51, 1]);
        let list_key = key(90);
        let env = list_env!(list_key);
        let left = build_list!(&env, list_key, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let right_duplicated = build_list!(&env, list_key, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let right_shared = left.clone();

        // `plan_equality` takes no `Meter`: it structurally cannot charge.
        let plan = plan_equality(&left, &right_duplicated).unwrap();
        let plan_pairs = format!("{:?}", plan.pair_events());

        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Composite(list_key)),
                EqualityOperand::typed(ValueType::Composite(list_key)),
            )
            .unwrap();
        let dup_result = checked.evaluate(&left, &right_duplicated, &mut Meter::new(limits));
        let shared_result = checked.evaluate(&left, &right_shared, &mut Meter::new(limits));

        let (_, charges, consumed) = metered(limits, |m: &mut Meter| checked.evaluate(&left, &right_duplicated, m));

        let mut denied_pair = Meter::new(limits).with_injected_denial(InjectedDenial {
            point: ChargePoint::EqualityPair,
            occurrence: to_occurrence(17),
        });
        let denied_pair = checked.evaluate(&left, &right_duplicated, &mut denied_pair);
        let mut denied_retain = Meter::new(limits).with_injected_denial(InjectedDenial {
            point: ChargePoint::EqualityResultRetain,
            occurrence: to_occurrence(1),
        });
        let denied_retain = checked.evaluate(&left, &right_duplicated, &mut denied_retain);

        (
            format!("{dup_result:?}"),
            format!("{shared_result:?}"),
            plan_pairs,
            charges,
            consumed,
            format!("{denied_pair:?}"),
            format!("{denied_retain:?}"),
        )
    }};
    assert_eq!(plan_pairs, format!("{:?}", int(17)));
    assert_eq!(
        dup_result,
        format!("{:?}", Outcome::<bool>::Completed(true))
    );
    assert_eq!(
        shared_result, dup_result,
        "DAG sharing does not change the equality/accounting result"
    );
    let mut expected = vec![ChargePoint::EqualityPlanForm, ChargePoint::EqualityPlan];
    expected.extend(core::iter::repeat(ChargePoint::EqualityPair).take(17));
    expected.push(ChargePoint::EqualityResultRetain);
    assert_eq!(charges, expected);
    let work = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::WorkUnits)
        .unwrap()];
    let results = consumed[LimitKind::ALL
        .iter()
        .position(|k| *k == LimitKind::ResultUnits)
        .unwrap()];
    assert_eq!(work, 51, "fifty-one work units");
    assert_eq!(results, 1, "one result unit");
    assert!(!denied_pair.contains("true") && !denied_pair.contains("false"));
    assert!(denied_pair.contains("Incomplete"), "{denied_pair}");
    assert!(!denied_retain.contains("true") && !denied_retain.contains("false"));
    assert!(denied_retain.contains("Incomplete"), "{denied_retain}");
}

/// E23: E21's duplicated-tree comparison under `work_units: 50`.
/// `equality.plan-form` consumes 32 work units; `equality.plan`'s
/// reservation of `17 + 2 = 19` is then unavailable, atomically, before any
/// pair event runs.
///
/// Trace: TC-026, FR-008-AC-2
#[test]
fn tc_026_e23_plan_reservation_incomplete() {
    let outcome = agree! {{
        let tight = limits([4, 0, 0, 0, 0, 0, 0, 17, 50, 1]);
        let list_key = key(91);
        let env = list_env!(list_key);
        let left = build_list!(&env, list_key, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let right = build_list!(&env, list_key, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Composite(list_key)),
                EqualityOperand::typed(ValueType::Composite(list_key)),
            )
            .unwrap();
        format!("{:?}", checked.evaluate(&left, &right, &mut Meter::new(tight)))
    }};
    assert_eq!(
        outcome,
        format!(
            "{:?}",
            Outcome::<bool>::Incomplete(incomplete(
                LimitKind::WorkUnits,
                50,
                32,
                int(19),
                ChargePoint::EqualityPlan
            ))
        )
    );
}

/// E24: a `Quantity` field nested in a record compares equal through the
/// Plan schedule with no `unit.*` charge: the leaf comparison is a direct
/// Rust equality inside `plan_pairs`, not a metered unit conversion.
///
/// Trace: TC-026, FR-008-AC-2, FR-008-AC-3
#[test]
fn tc_026_e24_quantity_in_composite_no_unit_charge() {
    let (outcome, charges, work7, work3) = agree! {{
        let full = limits([0, 0, 0, 0, 0, 0, 0, 2, 8, 1]);
        let tight7 = limits([0, 0, 0, 0, 0, 0, 0, 2, 7, 1]);
        let tight3 = limits([0, 0, 0, 0, 0, 0, 0, 2, 3, 1]);
        let fx = fixture();
        let cm = fx.unit("cm");
        let d_key = key(92);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                d_key,
                "D",
                CompositeShape::Record(vec![FieldDeclaration::new("d", ValueType::Quantity(cm.clone()), Presence::Required)]),
            )],
            [],
        )
        .unwrap();
        let left = env.record(d_key, vec![("d", FieldValue::Present(Value::Quantity(fx.qi(1, "cm"))))]).unwrap();
        let right = env.record(d_key, vec![("d", FieldValue::Present(Value::Quantity(fx.qi(1, "cm"))))]).unwrap();
        let checked = env
            .check_equality(
                EqualityOperator::Equal,
                EqualityOperand::typed(ValueType::Composite(d_key)),
                EqualityOperand::typed(ValueType::Composite(d_key)),
            )
            .unwrap();
        let (outcome, charges) = scheduled(full, |m: &mut Meter| checked.evaluate(&left, &right, m));
        let work7 = format!("{:?}", checked.evaluate(&left, &right, &mut Meter::new(tight7)));
        let work3 = format!("{:?}", checked.evaluate(&left, &right, &mut Meter::new(tight3)));
        (format!("{outcome:?}"), charges, work7, work3)
    }};
    assert_eq!(outcome, format!("{:?}", Outcome::<bool>::Completed(true)));
    assert_eq!(
        charges,
        [
            ChargePoint::EqualityPlanForm,
            ChargePoint::EqualityPlan,
            ChargePoint::EqualityPair,
            ChargePoint::EqualityPair,
            ChargePoint::EqualityResultRetain,
        ]
    );
    assert!(
        charges
            .iter()
            .all(|c| !format!("{c:?}").starts_with("Unit")),
        "no unit.* charge point among {charges:?}"
    );
    assert_eq!(
        work7,
        format!(
            "{:?}",
            Outcome::<bool>::Incomplete(incomplete(
                LimitKind::WorkUnits,
                7,
                4,
                int(4),
                ChargePoint::EqualityPlan
            ))
        )
    );
    assert_eq!(
        work3,
        format!(
            "{:?}",
            Outcome::<bool>::Incomplete(incomplete(
                LimitKind::WorkUnits,
                3,
                0,
                int(4),
                ChargePoint::EqualityPlanForm
            ))
        )
    );
}

/// `admits_equality_conversion`'s closed table, algebraically confirmed from
/// `src/exact/equality.rs`: eight rows spanning `Int`, unbounded `Integer`,
/// `Rational` and `Decimal` source/target pairs, including two
/// differing-decimal-scale rows (both directions, one admitted and one
/// refused).
///
/// Trace: TC-026, FR-008-AC-1, FR-008-AC-2
#[test]
fn tc_026_e26_admits_equality_conversion_table() {
    let rows = agree! {{
        let int_ty = |lo: i64, hi: i64| ValueType::Int(IntegerInterval::new(int(i128::from(lo)), int(i128::from(hi))).unwrap());
        let dec_ty = |lo: i64, hi: i64, min: u64, max: u64| ValueType::Decimal(decimal_type(lo, hi, min, max, RoundingMode::Exact));
        let rat_ty = |lo: &str, hi: &str, dmin: &str, dmax: &str| ValueType::Rational(rational_type(lo, hi, dmin, dmax));

        [
            // Int[0,2] -> Decimal[0,200;2,2]: admitted.
            admits_equality_conversion(&int_ty(0, 2), &dec_ty(0, 200, 2, 2)),
            // Decimal[0,9;0,0] -> Int[0,9]: admitted (max_scale is zero).
            admits_equality_conversion(&dec_ty(0, 9, 0, 0), &int_ty(0, 9)),
            // Rational[0,5;1,1] -> Int[0,5]: admitted (denominator bound is exactly one).
            admits_equality_conversion(&rat_ty("0", "5", "1", "1"), &int_ty(0, 5)),
            // Decimal[0,9;0,1] -> Int[0,9]: refused (max_scale is not zero).
            admits_equality_conversion(&dec_ty(0, 9, 0, 1), &int_ty(0, 9)),
            // Int[0,300] -> Decimal[0,200;2,2]: refused (300 exceeds the target's shifted upper bound).
            admits_equality_conversion(&int_ty(0, 300), &dec_ty(0, 200, 2, 2)),
            // Unbounded Integer -> Rational[0,2;1,1]: refused (no table row for an unbounded source).
            admits_equality_conversion(&ValueType::Integer, &rat_ty("0", "2", "1", "1")),
            // Decimal[0,100;1,2] -> Decimal[0,100;0,2]: admitted, scale 1 down to scale 0..2.
            admits_equality_conversion(&dec_ty(0, 100, 1, 2), &dec_ty(0, 100, 0, 2)),
            // Decimal[0,100;0,2] -> Decimal[0,100;1,2]: refused, target's minimum scale exceeds the source's.
            admits_equality_conversion(&dec_ty(0, 100, 0, 2), &dec_ty(0, 100, 1, 2)),
        ]
    }};
    assert_eq!(rows, [true, true, true, false, false, false, true, false]);
}

/// E28: `=` on any IEEE-bearing type — a bare `Float32`, a record with a
/// `Float32` field, and declaring `Set<Float64>[0,2]` — refuses
/// `operator-ineligible` before any charge. This is the actual mechanism
/// behind `Value` having no structural `PartialEq`: `-0.0` and `NaN` never
/// reach a comparison path, structural or otherwise.
///
/// Trace: TC-026, FR-008-AC-2
#[test]
fn tc_026_e28_ieee_operator_ineligible() {
    let (bare, record_check, set_check) = agree! {{
        let f_key = key(93);
        let env = TypeEnvironment::new(
            [CompositeDeclaration::new(
                f_key,
                "F",
                CompositeShape::Record(vec![FieldDeclaration::new("x", ValueType::Float(IeeeWidth::Binary32), Presence::Required)]),
            )],
            [],
        )
        .unwrap();
        let bare = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Float(IeeeWidth::Binary32)),
            EqualityOperand::typed(ValueType::Float(IeeeWidth::Binary32)),
        );
        let record_check = env.check_equality(
            EqualityOperator::Equal,
            EqualityOperand::typed(ValueType::Composite(f_key)),
            EqualityOperand::typed(ValueType::Composite(f_key)),
        );
        let set_check = env.check_type(&ValueType::collection(CollectionType::new(
            CollectionKind::Set,
            ValueType::Float(IeeeWidth::Binary64),
            CardinalityBound::new(0, 2).unwrap(),
        )));
        (format!("{bare:?}"), format!("{record_check:?}"), format!("{set_check:?}"))
    }};
    assert!(bare.contains("OperatorIneligible"), "{bare}");
    assert!(
        record_check.contains("OperatorIneligible"),
        "{record_check}"
    );
    assert!(set_check.contains("OperatorIneligible"), "{set_check}");
}
