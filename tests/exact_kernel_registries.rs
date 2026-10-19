//! Caller-owned identities used by the eventual kernel value cutover.
#![cfg(feature = "exact")]

use ix_trace_rs::trace;
use quire_contract_runtime::exact::{
    CompoundUnit, EnumDeclaration, KernelIdentityRegistry, NodeKey, ObjectTypeDeclaration,
    RegistryCause, TypeEnvironment, UnitDeclaration, UnitGraph,
};
use quire_exact::{EffectiveId, Integer, Rational, UnitId, VariantId};

fn key(byte: u8) -> NodeKey {
    NodeKey::from_bytes([byte; 32])
}

#[trace("TC-026", "FR-008-AC-5")]
#[test]
fn declared_unit_id_resolves_only_an_admitted_unit() {
    let dimension = key(1);
    let unit = key(2);
    let graph = UnitGraph::admit(
        [(dimension, vec![])],
        [(
            unit,
            UnitDeclaration {
                dimension,
                target: None,
                scale: Rational::from_integer(Integer::one()),
                offset: Rational::from_integer(Integer::zero()),
            },
        )],
    )
    .unwrap();
    let types = TypeEnvironment::new([], []).unwrap();
    let mut registry = KernelIdentityRegistry::new(&graph, &types);
    let id = UnitId::declared(quire_exact::NodeKey::from_digest(*unit.as_bytes()));
    assert_eq!(registry.resolve_unit(id).unwrap().key(), unit);
    assert!(registry
        .resolve_unit(UnitId::compound(*unit.as_bytes()))
        .is_none());
    assert!(registry
        .resolve_unit(UnitId::declared(quire_exact::NodeKey::from_digest([3; 32])))
        .is_none());
    let compound = UnitId::compound([4; 32]);
    registry
        .register_compound(compound, CompoundUnit::dimensionless())
        .unwrap();
    assert_eq!(
        registry.resolve_compound(compound),
        Some(&CompoundUnit::dimensionless())
    );
    assert_eq!(
        registry.register_compound(compound, CompoundUnit::dimensionless()),
        Err(RegistryCause::DuplicateIdentity)
    );
    assert_eq!(
        registry.register_compound(id, CompoundUnit::dimensionless()),
        Err(RegistryCause::WrongUnitDomain)
    );
}

#[trace("TC-024", "FR-008-AC-1")]
#[test]
fn enum_shapes_and_effective_object_types_are_admitted_once() {
    let graph = UnitGraph::default();
    let object_key = key(8);
    let types = TypeEnvironment::new(
        [],
        [ObjectTypeDeclaration::new(object_key, "Object", vec![])],
    )
    .unwrap();
    let mut registry = KernelIdentityRegistry::new(&graph, &types);
    let declaration = EnumDeclaration::new(key(4), true, &["first", "second"]).unwrap();
    let first = VariantId::from_digest([5; 32]);
    let second = VariantId::from_digest([6; 32]);
    registry
        .register_enum(&declaration, [first, second])
        .unwrap();
    let shape = registry.enum_shape(declaration.key()).unwrap();
    assert_eq!(shape.rank(first), Some(0));
    assert_eq!(shape.rank(second), Some(1));
    assert!(shape.is_ordered());
    assert_eq!(
        registry.register_enum(&declaration, [first, second]),
        Err(RegistryCause::DuplicateIdentity)
    );
    let effective = EffectiveId::from_digest([7; 32]);
    registry
        .register_object_type(effective, object_key)
        .unwrap();
    assert_eq!(registry.object_type(effective).unwrap().key(), object_key);
    assert_eq!(
        registry.register_object_type(effective, object_key),
        Err(RegistryCause::DuplicateIdentity)
    );
    assert_eq!(
        registry.register_object_type(EffectiveId::from_digest([9; 32]), key(10)),
        Err(RegistryCause::UnknownObjectType)
    );
}
