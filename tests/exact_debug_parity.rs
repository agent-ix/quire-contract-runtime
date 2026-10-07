//! Debug-rendering regression pins for the value/type structs the runtime still defines whose
//! `Debug` impl is hand-written because their fields sit behind one `Box`.
//! FR-007-AC-6's own oracle -- equal Debug renderings against the quire-spec-language
//! authority -- is removed from this repository; recreating it in
//! agent-ix/quire-integration is planned under Linear IR-430. This is
//! not that oracle (FR-007-AC-13, TC-035): it only pins each type's own
//! rendering against a fixed string, so a field added to a `*Fields` struct
//! but not to its `Debug` impl is caught here even while that oracle
//! is red. The kernel scalars' pins left with the kernel to `quire-exact` (FR-275).
#![cfg(feature = "exact")]

use quire_contract_runtime::exact::{
    CompoundUnit, EnumDeclaration, NodeKey, ObjectIdentity, ObjectReference,
};
use quire_exact::UniverseId;

/// Trace: TC-035, FR-007-AC-13
///
/// Only the runtime-held `CompoundUnit` and `Dimension` rendering is pinned here; the kernel
/// scalars' own renderings are `quire-exact`'s to pin.
#[test]
fn tc_035_debug_parity_compound_unit() {
    let compound = CompoundUnit::dimensionless();
    assert_eq!(
        format!("{compound:?}"),
        "CompoundUnit { terms: {}, dimension: Dimension({}) }"
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
        UniverseId::from_digest([1_u8; 32]),
        NodeKey::from_bytes([3_u8; 32]),
        ObjectIdentity::new(&[2_u8]).unwrap(),
    );
    assert_eq!(
        format!("{reference:?}"),
        "ObjectReference { universe: UniverseId(0101010101010101010101010101010101010101010101010101010101010101), object_type: \
         NodeKey(0303030303030303030303030303030303030303030303030303030303030303), identity: \
         ObjectIdentity([2]) }"
    );
}
