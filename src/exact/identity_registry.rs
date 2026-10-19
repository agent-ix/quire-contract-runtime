// SPDX-License-Identifier: AGPL-3.0-or-later
//! Caller-owned identities for admitted unit, enum and object declarations.
//!
//! The exact kernel carries these identities without the declaration context.
//! This registry retains that context without constructing a `NodeKey` or
//! converting between value representations.

use alloc::collections::{BTreeMap, BTreeSet};

use quire_exact::{EffectiveId, EnumShape, UnitDomain, UnitId, VariantId};

use super::composite::{ObjectTypeDeclaration, TypeEnvironment};
use super::enumeration::EnumDeclaration;
use super::node::NodeKey;
use super::unit::{CompoundUnit, Unit, UnitGraph};

/// Why a caller-supplied identity cannot be registered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryCause {
    /// An identity has already been bound.
    DuplicateIdentity,
    /// The number of supplied variant identities differs from the declaration.
    MemberCountMismatch,
    /// One variant identity was supplied twice.
    DuplicateVariant,
    /// The effective identity names no admitted object type.
    UnknownObjectType,
    /// A declared-unit identity was supplied for a compound unit.
    WrongUnitDomain,
}

/// Admitted declaration lookups for kernel identity payloads.
///
/// The caller supplies precomputed variant, compound and effective identities.
/// This type checks their relationship to admitted RT declarations, without
/// minting any digest or node key.
pub struct KernelIdentityRegistry<'a> {
    units: &'a UnitGraph,
    objects: &'a TypeEnvironment,
    compounds: BTreeMap<UnitId, CompoundUnit>,
    enums: BTreeMap<NodeKey, EnumShape>,
    effective_objects: BTreeMap<EffectiveId, NodeKey>,
}

impl<'a> KernelIdentityRegistry<'a> {
    /// Borrow the admitted unit graph and object-type environment.
    pub fn new(units: &'a UnitGraph, objects: &'a TypeEnvironment) -> Self {
        Self {
            units,
            objects,
            compounds: BTreeMap::new(),
            enums: BTreeMap::new(),
            effective_objects: BTreeMap::new(),
        }
    }

    /// Resolve a declared unit by its domain-labelled kernel identity.
    ///
    /// Byte comparison against already admitted keys needs no `NodeKey`
    /// constructor and cannot mistake a compound digest for a declared unit.
    pub fn resolve_unit(&self, identity: UnitId) -> Option<&Unit> {
        if identity.domain() != UnitDomain::Declared {
            return None;
        }
        self.units
            .admitted_units()
            .find(|unit| unit.key().as_bytes() == identity.as_bytes())
    }

    /// Bind an already-computed compound digest to its normalized unit.
    pub fn register_compound(
        &mut self,
        identity: UnitId,
        unit: CompoundUnit,
    ) -> Result<(), RegistryCause> {
        if identity.domain() != UnitDomain::Compound {
            return Err(RegistryCause::WrongUnitDomain);
        }
        if self.compounds.contains_key(&identity) {
            return Err(RegistryCause::DuplicateIdentity);
        }
        self.compounds.insert(identity, unit);
        Ok(())
    }

    /// Resolve a previously bound compound-unit identity.
    pub fn resolve_compound(&self, identity: UnitId) -> Option<&CompoundUnit> {
        self.compounds.get(&identity)
    }

    /// Bind the admitted variants of an enum in canonical declaration order.
    pub fn register_enum(
        &mut self,
        declaration: &EnumDeclaration,
        variants: impl IntoIterator<Item = VariantId>,
    ) -> Result<(), RegistryCause> {
        let variants: alloc::vec::Vec<_> = variants.into_iter().collect();
        if variants.len() != declaration.members().len() {
            return Err(RegistryCause::MemberCountMismatch);
        }
        if variants.iter().copied().collect::<BTreeSet<_>>().len() != variants.len() {
            return Err(RegistryCause::DuplicateVariant);
        }
        if self.enums.contains_key(&declaration.key()) {
            return Err(RegistryCause::DuplicateIdentity);
        }
        self.enums.insert(
            declaration.key(),
            EnumShape::new(declaration.is_ordered(), variants),
        );
        Ok(())
    }

    /// The kernel shape of a registered declaration.
    pub fn enum_shape(&self, declaration: NodeKey) -> Option<&EnumShape> {
        self.enums.get(&declaration)
    }

    /// Bind a model-supplied effective identity to an admitted object type.
    pub fn register_object_type(
        &mut self,
        identity: EffectiveId,
        declaration: NodeKey,
    ) -> Result<(), RegistryCause> {
        if self.objects.object_type(declaration).is_none() {
            return Err(RegistryCause::UnknownObjectType);
        }
        if self.effective_objects.contains_key(&identity) {
            return Err(RegistryCause::DuplicateIdentity);
        }
        self.effective_objects.insert(identity, declaration);
        Ok(())
    }

    /// Resolve an effective identity to the admitted object-type declaration.
    pub fn object_type(&self, identity: EffectiveId) -> Option<&ObjectTypeDeclaration> {
        self.effective_objects
            .get(&identity)
            .and_then(|key| self.objects.object_type(*key))
    }
}
