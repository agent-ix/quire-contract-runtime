// SPDX-License-Identifier: AGPL-3.0-or-later
//! Opaque I04 nominal semantic-node keys and the `invalid_semantic_graph`
//! refusal vocabulary.
//!
//! The compiler computes every key as the SHA-256 of the RFC 8785 JCS preimage
//! in the `quire.checked-semantic-node/v1` domain and admits owners and stale
//! keys. The runtime never recomputes a key: it receives admitted keys as
//! opaque 32-byte values and checks only the structural graph rules it
//! evaluates over.

use alloc::collections::BTreeSet;
use core::fmt;

use super::integer::Integer;

/// Digest domain of every checked semantic node key.
pub const NODE_KEY_DOMAIN: &str = "quire.checked-semantic-node/v1";

/// An opaque `quire.checked-semantic-node/v1` node key.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeKey([u8; 32]);

impl NodeKey {
    /// Wrap an admitted raw digest.
    pub const fn from_bytes(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    /// Parse 64 lowercase hexadecimal digits.
    pub fn from_hex(digest: &str) -> Option<Self> {
        let bytes = digest.as_bytes();
        if bytes.len() != 64 {
            return None;
        }
        let mut key = [0_u8; 32];
        let mut digits = bytes.iter();
        for slot in &mut key {
            let high = lower_hex(*digits.next()?)?;
            let low = lower_hex(*digits.next()?)?;
            *slot = high.checked_shl(4)? | low;
        }
        Some(Self(key))
    }

    /// The raw digest.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

fn lower_hex(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => digit.checked_sub(b'0'),
        b'a'..=b'f' => digit.checked_sub(b'a')?.checked_add(10),
        _ => None,
    }
}

impl fmt::Display for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|byte| write!(f, "{byte:02x}"))
    }
}

impl fmt::Debug for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeKey({self})")
    }
}

/// The strict reader's `refused { code: invalid_semantic_graph }`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InvalidSemanticGraph {
    /// The typed reason.
    pub cause: SemanticGraphCause,
}

impl InvalidSemanticGraph {
    /// Stable refusal code.
    pub const CODE: &'static str = "invalid_semantic_graph";
}

impl fmt::Display for InvalidSemanticGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid_semantic_graph: {:?}", self.cause)
    }
}

/// Why a nominal semantic node or node graph was refused.
///
/// The vocabulary is closed and shared with the compiler. Preimage, owner and
/// stale-key causes are raised only at compiler admission; the runtime raises
/// the structural causes of the graphs it is handed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticGraphCause {
    /// The preimage does not satisfy `node-identity-preimage.schema.json`.
    NonCanonicalPreimage,
    /// An unordered declaration's members are not sorted by case name.
    UnsortedUnorderedMembers,
    /// The owner does not join the lock selection.
    OwnerNotSelected,
    /// The retained key is not the digest of the admitted content.
    StaleKey,
    /// A member node references a different declaration node.
    ForeignDeclaration,
    /// A member node's case is not a member of its declaration.
    UndeclaredCase,
    /// A dimension term has exponent zero.
    ZeroExponent,
    /// A dimension term names the same base dimension twice.
    DuplicateTerm,
    /// Dimension terms are not strictly ascending by node key.
    UnsortedTerms,
    /// A unit scale or offset is not a reduced rational.
    UnreducedRational,
    /// A unit scale is zero.
    ZeroScale,
    /// A targetless unit does not have scale one and offset zero.
    NonIdentityRoot,
    /// Two admitted nodes have the same key.
    DuplicateNode,
    /// A dimension term or unit names a dimension node that is not admitted.
    UnknownDimension,
    /// A dimension term names a derived (non-base) dimension.
    NonBaseDimensionTerm,
    /// A unit target is not an admitted unit.
    UnknownTarget,
    /// A unit target belongs to a different dimension node.
    CrossDimensionTarget,
    /// A dimension's unit graph has no targetless canonical root.
    MissingRoot,
    /// A dimension's unit graph has more than one targetless root.
    DuplicateRoot,
    /// Unit targets form a cycle.
    TargetCycle,
}

pub(crate) fn refuse(cause: SemanticGraphCause) -> InvalidSemanticGraph {
    InvalidSemanticGraph { cause }
}

/// Whether `text` is a schema identifier.
pub(crate) fn is_identifier(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Refuse terms that are zero, repeated or not strictly ascending, in that
/// order.
pub(crate) fn check_terms(terms: &[(NodeKey, Integer)]) -> Result<(), SemanticGraphCause> {
    if terms.iter().any(|(_, exponent)| exponent.is_zero()) {
        return Err(SemanticGraphCause::ZeroExponent);
    }
    let distinct: BTreeSet<_> = terms.iter().map(|(key, _)| key).collect();
    if distinct.len() != terms.len() {
        return Err(SemanticGraphCause::DuplicateTerm);
    }
    let ascending = terms
        .iter()
        .zip(terms.iter().skip(1))
        .all(|((left, _), (right, _))| left < right);
    if !ascending {
        return Err(SemanticGraphCause::UnsortedTerms);
    }
    Ok(())
}
