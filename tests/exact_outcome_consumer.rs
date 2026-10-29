// SPDX-License-Identifier: AGPL-3.0-or-later
//! RT consumer evidence for the kernel's closed exact outcome envelope.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::scalar::{evaluate_boolean_short_circuit, ShortCircuitConnective};
use quire_exact::{
    ChargePoint, CheckedInvariantCause, Incomplete, Integer, LimitKind, Meter, Outcome, Refusal,
    ScalarLimits, Undefined,
};
use quire_specification::VALUE_ACCOUNTING_CHARGE_POINT_VECTORS;
use std::collections::BTreeSet;

fn closed_refusal(reason: &Refusal) {
    match reason {
        Refusal::InexactDecimal { .. }
        | Refusal::DecimalOutOfDomain { .. }
        | Refusal::DivisionOutOfDomain { .. }
        | Refusal::ModuloOutOfDomain { .. }
        | Refusal::TextLengthOutOfDomain { .. }
        | Refusal::IntegerOutOfDomain { .. }
        | Refusal::RationalOutOfDomain { .. }
        | Refusal::IeeeNotExact { .. }
        | Refusal::IeeeNanPayloadNotRepresentable { .. }
        | Refusal::IeeeRationalOutOfDomain { .. }
        | Refusal::ForeignReference { .. }
        | Refusal::CardinalityOutOfBound { .. }
        | Refusal::CheckedInvariant { .. } => {}
    }
}

fn closed_undefined(reason: &Undefined) {
    match reason {
        Undefined::DivisionByZero
        | Undefined::IeeeNotFinite
        | Undefined::EmptyReduction
        | Undefined::NoneValue
        | Undefined::SumOutOfDomain => {}
    }
}

fn closed_limit_kind(kind: LimitKind) {
    match kind {
        LimitKind::IntegerBits
        | LimitKind::DecimalDigits
        | LimitKind::ScaleExpansion
        | LimitKind::TextInputBytes
        | LimitKind::TextScalars
        | LimitKind::NormalizedScalars
        | LimitKind::UnitEdges
        | LimitKind::ValueOccurrences
        | LimitKind::WorkUnits
        | LimitKind::ResultUnits => {}
    }
}

fn closed_charge_point(point: ChargePoint) {
    match point {
        ChargePoint::DecimalOperands
        | ChargePoint::DecimalScaleExpansion
        | ChargePoint::DecimalArithmetic
        | ChargePoint::DecimalRounding
        | ChargePoint::DecimalResultRetain
        | ChargePoint::TextInputBytes
        | ChargePoint::TextDecodeScalars
        | ChargePoint::TextNormalizeInput
        | ChargePoint::TextNormalizeOutput
        | ChargePoint::TextResultRetain
        | ChargePoint::EnumIdentityRead
        | ChargePoint::EnumResultRetain
        | ChargePoint::UnitIdentityRead
        | ChargePoint::UnitEdge
        | ChargePoint::UnitRationalArithmetic
        | ChargePoint::UnitTargetDomain
        | ChargePoint::UnitResultRetain
        | ChargePoint::IntegerDivisionOperands
        | ChargePoint::IntegerDivisionArithmetic
        | ChargePoint::IntegerDivisionDomain
        | ChargePoint::IntegerDivisionResultRetain
        | ChargePoint::IntegerModulusOperands
        | ChargePoint::IntegerModulusArithmetic
        | ChargePoint::IntegerModulusDomain
        | ChargePoint::IntegerModulusResultRetain
        | ChargePoint::IeeeOperands
        | ChargePoint::IeeeExactIntermediate
        | ChargePoint::IeeeRound
        | ChargePoint::IeeeResultRetain
        | ChargePoint::EqualityPlanForm
        | ChargePoint::EqualityPlan
        | ChargePoint::EqualityPair
        | ChargePoint::EqualityResultRetain
        | ChargePoint::FunctionCall
        | ChargePoint::CollectionElement
        | ChargePoint::CollectionVisit
        | ChargePoint::CollectionMemberWalk
        | ChargePoint::CollectionMemberTest
        | ChargePoint::CollectionBound
        | ChargePoint::CollectionResultRetain
        | ChargePoint::CompositeResultRetain
        | ChargePoint::IntegerArithmeticOperands
        | ChargePoint::IntegerArithmeticArithmetic
        | ChargePoint::IntegerArithmeticResultRetain
        | ChargePoint::RationalArithmeticOperands
        | ChargePoint::RationalArithmeticArithmetic
        | ChargePoint::RationalArithmeticNormalize
        | ChargePoint::RationalArithmeticResultRetain
        | ChargePoint::OrderingOperands
        | ChargePoint::OrderingArithmetic
        | ChargePoint::OrderingResultRetain
        | ChargePoint::BooleanResultRetain
        | ChargePoint::LookupKey
        | ChargePoint::LookupResultRetain
        | ChargePoint::PopulationVisit
        | ChargePoint::DispatchSelect
        | ChargePoint::DeclarationCheck
        | ChargePoint::GraphExpand
        | ChargePoint::GraphEdge
        | ChargePoint::GraphResultRetain
        | ChargePoint::ModelDeref
        | ChargePoint::ModelNavigate
        | ChargePoint::TemporalVisit
        | ChargePoint::FrameWrite
        | ChargePoint::FrameRedefinesLink => {}
    }
}

#[test]
fn consumed_kernel_charge_points_match_authoritative_vocabulary() {
    let vectors: serde_json::Value = serde_json::from_str(VALUE_ACCOUNTING_CHARGE_POINT_VECTORS)
        .expect("QSpec vectors are JSON");
    let declared = vectors["charge_points"]
        .as_array()
        .expect("QSpec charge points are an array");
    let declared: Vec<&str> = declared
        .iter()
        .map(|entry| entry.as_str().expect("QSpec charge point is a string"))
        .collect();
    let kernel: Vec<&str> = ChargePoint::ALL
        .iter()
        .map(|point| point.as_str())
        .collect();
    let declared_set: BTreeSet<&str> = declared.iter().copied().collect();
    let kernel_set: BTreeSet<&str> = kernel.iter().copied().collect();

    assert_eq!(
        declared_set.len(),
        declared.len(),
        "QSpec spellings are unique"
    );
    assert_eq!(
        kernel_set.len(),
        kernel.len(),
        "kernel spellings are unique"
    );
    assert_eq!(
        kernel_set, declared_set,
        "kernel and QSpec spellings differ"
    );

    for point in ChargePoint::ALL {
        closed_charge_point(point);
        assert_eq!(ChargePoint::from_code(point.as_str()), Some(point));
    }
    assert_ne!(ChargePoint::EqualityPlanForm, ChargePoint::EqualityPlan);
    assert_eq!(
        ChargePoint::from_code("equality.plan-form"),
        Some(ChargePoint::EqualityPlanForm)
    );

    let undeclared = vectors["undeclared"]
        .as_array()
        .expect("QSpec undeclared spellings are an array");
    assert!(!undeclared.is_empty(), "QSpec has negative spellings");
    for entry in undeclared {
        let spelling = entry.as_str().expect("QSpec negative spelling is a string");
        assert!(!declared_set.contains(spelling), "{spelling:?} is declared");
        assert_eq!(ChargePoint::from_code(spelling), None);
    }
}

#[trace("FR-006-AC-1")]
#[test]
fn consumed_exact_outcomes_keep_false_distinct_from_all_stops() {
    let limits = ScalarLimits {
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

    let completed_false = evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        false,
        || panic!("short-circuited right operand was evaluated"),
        &mut Meter::new(limits),
    );
    let completed_true = evaluate_boolean_short_circuit(
        ShortCircuitConnective::Or,
        true,
        || panic!("short-circuited right operand was evaluated"),
        &mut Meter::new(limits),
    );
    let undefined_reason = Undefined::DivisionByZero;
    let undefined = evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        true,
        || Outcome::Undefined(undefined_reason.clone()),
        &mut Meter::new(limits),
    );
    let refusal_reason = Refusal::CheckedInvariant {
        cause: CheckedInvariantCause::ForeignCheckedExpression,
    };
    let refused = evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        true,
        || Outcome::Refused(refusal_reason.clone()),
        &mut Meter::new(limits),
    );
    let incomplete_record = Incomplete {
        limit_kind: LimitKind::WorkUnits,
        limit: 0,
        consumed: 0,
        next_charge: Integer::from(1_i64),
        charge_point: ChargePoint::BooleanResultRetain,
    };
    let incomplete = evaluate_boolean_short_circuit(
        ShortCircuitConnective::And,
        false,
        || panic!("short-circuited right operand was evaluated"),
        &mut Meter::new(ScalarLimits {
            work_units: 0,
            ..limits
        }),
    );

    assert_eq!(completed_false, Outcome::Completed(false));
    assert_eq!(completed_true, Outcome::Completed(true));
    assert_eq!(undefined, Outcome::Undefined(undefined_reason.clone()));
    assert_eq!(refused, Outcome::Refused(refusal_reason.clone()));
    assert_eq!(incomplete, Outcome::Incomplete(incomplete_record.clone()));

    let outcomes = [
        completed_false,
        completed_true,
        undefined,
        refused,
        incomplete,
    ];
    for (index, left) in outcomes.iter().enumerate() {
        for right in &outcomes[index + 1..] {
            assert_ne!(left, right, "distinct exact dispositions were conflated");
        }
    }
    assert_eq!(outcomes[0].clone().completed(), Some(false));
    assert_eq!(outcomes[1].clone().completed(), Some(true));
    for stopped in &outcomes[2..] {
        assert_eq!(stopped.clone().completed(), None);
    }

    closed_undefined(&undefined_reason);
    closed_refusal(&refusal_reason);
    closed_limit_kind(incomplete_record.limit_kind);
    closed_charge_point(incomplete_record.charge_point);
}
