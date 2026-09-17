// SPDX-License-Identifier: AGPL-3.0-or-later
//! Terminal `Reference<T>` values (FR-143).
//!
//! A reference is terminal: its identity is the snapshot-supplied FR-009/
//! FR-204 triple (universe, object-type declaration identity, object
//! identity), and equality never inspects the referenced state. No source
//! form creates one.
//!
//! The closed `ObjectEnvironment` machinery that resolves a reference against
//! a bound model snapshot is out of scope for this crate: it is business
//! logic that belongs to a consumer holding that snapshot, not to the exact
//! value/collection/equality core ported here.

use alloc::boxed::Box;
use core::fmt;

use super::node::NodeKey;

/// A universe identity in its canonical identity bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UniverseIdentity(Box<[u8]>);

/// A declared object identity in its canonical identity bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectIdentity(Box<[u8]>);

/// An identity component with no canonical identity bytes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InvalidObjectIdentity;

impl fmt::Display for InvalidObjectIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an identity component has at least one canonical identity byte")
    }
}

fn identity_bytes(bytes: &[u8]) -> Result<Box<[u8]>, InvalidObjectIdentity> {
    if bytes.is_empty() {
        return Err(InvalidObjectIdentity);
    }
    Ok(bytes.into())
}

impl UniverseIdentity {
    /// A universe from its canonical identity bytes.
    pub fn new(bytes: &[u8]) -> Result<Self, InvalidObjectIdentity> {
        identity_bytes(bytes).map(Self)
    }

    /// The canonical identity bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl ObjectIdentity {
    /// An object identity from its canonical identity bytes.
    pub fn new(bytes: &[u8]) -> Result<Self, InvalidObjectIdentity> {
        identity_bytes(bytes).map(Self)
    }

    /// The canonical identity bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// A `Reference<T>` value: its identity triple.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectReference {
    universe: UniverseIdentity,
    object_type: NodeKey,
    identity: ObjectIdentity,
}

impl ObjectReference {
    /// The reference `(universe, object_type, identity)` supplied by a bound
    /// model snapshot.
    pub fn new(universe: UniverseIdentity, object_type: NodeKey, identity: ObjectIdentity) -> Self {
        Self {
            universe,
            object_type,
            identity,
        }
    }

    /// The universe identity.
    pub fn universe(&self) -> &UniverseIdentity {
        &self.universe
    }

    /// The object-type declaration identity.
    pub fn object_type(&self) -> NodeKey {
        self.object_type
    }

    /// The declared object identity.
    pub fn identity(&self) -> &ObjectIdentity {
        &self.identity
    }
}
