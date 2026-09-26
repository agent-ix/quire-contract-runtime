//! Debug-rendering regression pins for the nine value/type structs whose
//! `Debug` impl is hand-written because their fields sit behind one `Box`.
//! FR-007-AC-6's own oracle -- equal Debug renderings against the pinned
//! authority -- lives in `conformance/qsl-agreement`, which does not compile
//! on this tree or on `origin/main` (E0004, unrelated to this PR). This is
//! not that oracle (FR-007-AC-13, TC-035): it only pins each type's own
//! rendering against a fixed string, so a field added to a `*Fields` struct
//! but not to its `Debug` impl is caught here even while the conformance
//! crate is red.
#![cfg(feature = "exact")]

use quire_contract_runtime::exact::{
    admit_text, CompoundUnit, Decimal, DecimalType, EnumDeclaration, Integer, IntegerInterval,
    Meter, NodeKey, ObjectIdentity, ObjectReference, Outcome, Quantity, QuantityUnit, Rational,
    RationalDomain, RoundingMode, ScalarLimits, TextPayload, TextProfile, TextType,
    UniverseIdentity,
};

fn unlimited() -> ScalarLimits {
    ScalarLimits {
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
    }
}

/// Trace: TC-035, FR-007-AC-13
#[test]
fn tc_035_debug_parity_rational_and_rational_domain() {
    let rational = Rational::new(Integer::from(1_i64), Integer::from(2_i64)).unwrap();
    assert_eq!(
        format!("{rational:?}"),
        "Rational { numerator: Integer(1), denominator: Integer(2) }"
    );

    let domain = RationalDomain::new(
        IntegerInterval::new(Integer::from(0_i64), Integer::from(1_i64)).unwrap(),
        IntegerInterval::new(Integer::from(1_i64), Integer::from(2_i64)).unwrap(),
    )
    .unwrap();
    assert_eq!(
        format!("{domain:?}"),
        "RationalDomain { numerator: IntegerInterval { lower: Integer(0), upper: Integer(1) }, \
         denominator: IntegerInterval { lower: Integer(1), upper: Integer(2) } }"
    );
}

/// Trace: TC-035, FR-007-AC-13
#[test]
fn tc_035_debug_parity_decimal_and_decimal_type() {
    let decimal = Decimal::new(Integer::from(123_i64), 2);
    assert_eq!(
        format!("{decimal:?}"),
        "Decimal { representation: DecimalRepresentation { coefficient: Integer(123), scale: 2 \
         }, normalized: DecimalRepresentation { coefficient: Integer(123), scale: 2 } }"
    );

    let decimal_type = DecimalType::new(
        Integer::from(0_i64),
        Integer::from(100_i64),
        0,
        2,
        RoundingMode::Exact,
    )
    .unwrap();
    assert_eq!(
        format!("{decimal_type:?}"),
        "DecimalType { lower: Integer(0), upper: Integer(100), min_scale: 0, max_scale: 2, \
         rounding: Exact }"
    );
}

/// Trace: TC-035, FR-007-AC-13
#[test]
fn tc_035_debug_parity_quantity_and_compound_unit() {
    let quantity = Quantity::new(
        Rational::new(Integer::from(3_i64), Integer::from(1_i64)).unwrap(),
        QuantityUnit::Compound(CompoundUnit::dimensionless()),
    );
    assert_eq!(
        format!("{quantity:?}"),
        "Quantity { value: Rational { numerator: Integer(3), denominator: Integer(1) }, unit: \
         Compound(CompoundUnit { terms: {}, dimension: Dimension({}) }) }"
    );

    let compound = CompoundUnit::dimensionless();
    assert_eq!(
        format!("{compound:?}"),
        "CompoundUnit { terms: {}, dimension: Dimension({}) }"
    );
}

/// Trace: TC-035, FR-007-AC-13
#[test]
fn tc_035_debug_parity_text() {
    let text_type = TextType::new(0, 10, TextProfile::UnicodeScalars).unwrap();
    let payload = TextPayload::from_utf8(b"hi").unwrap();
    let mut meter = Meter::new(unlimited());
    let Outcome::Completed(text) = admit_text(&payload, &text_type, &mut meter) else {
        panic!("admit_text refused a well-formed payload");
    };
    assert_eq!(
        format!("{text:?}"),
        "Text { text_type: TextType { min: 0, max: 10, profile: UnicodeScalars }, payload: \
         TextPayload { text: \"hi\", provenance: Runtime }, retained: \"hi\" }"
    );
}

/// Trace: TC-035, FR-007-AC-13
#[test]
fn tc_035_debug_parity_enum_value_and_object_reference() {
    let declaration =
        EnumDeclaration::new(NodeKey::from_bytes([1_u8; 32]), true, &["A", "B"]).unwrap();
    let member = declaration
        .member("A", NodeKey::from_bytes([2_u8; 32]))
        .unwrap();
    assert_eq!(
        format!("{member:?}"),
        "EnumValue { declaration: NodeKey(0101010101010101010101010101010101010101010101010101010101010101), \
         member: NodeKey(0202020202020202020202020202020202020202020202020202020202020202), ordered: true, \
         position: 0, case: \"A\" }"
    );

    let reference = ObjectReference::new(
        UniverseIdentity::new(&[1_u8]).unwrap(),
        NodeKey::from_bytes([3_u8; 32]),
        ObjectIdentity::new(&[2_u8]).unwrap(),
    );
    assert_eq!(
        format!("{reference:?}"),
        "ObjectReference { universe: UniverseIdentity([1]), object_type: \
         NodeKey(0303030303030303030303030303030303030303030303030303030303030303), identity: \
         ObjectIdentity([2]) }"
    );
}
