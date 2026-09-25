// SPDX-License-Identifier: AGPL-3.0-or-later
//! quire-specification/FR-141 declaration-qualified enumerations over compiler-admitted node keys.
//!
//! The compiler computes the `quire.enum-declaration-node/v1` and
//! `quire.enum-member-node/v1` keys, joins owners and refuses stale keys. The
//! runtime receives the admitted declaration key, its ordering and its member
//! cases, checks the structural rules it compares under, and never recomputes
//! a key.

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use super::accounting::{Charge, ChargePoint, LimitKind, Meter};
use super::comparison::{ComparisonOperator, IllTyped, IllTypedCause};
use super::node::{is_identifier, refuse, InvalidSemanticGraph, NodeKey, SemanticGraphCause};
use super::outcome::{Outcome, Stop};

/// An admitted enum declaration node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumDeclaration {
    key: NodeKey,
    ordered: bool,
    members: Vec<String>,
}

impl EnumDeclaration {
    /// Admit the declaration node `key` with its member cases, in declaration
    /// order (or sorted by case name, if unordered).
    ///
    /// An empty, repeated or non-identifier member list is
    /// `NonCanonicalPreimage`; an unordered declaration whose cases are not
    /// sorted is `UnsortedUnorderedMembers`.
    pub fn new(
        key: NodeKey,
        ordered: bool,
        members: &[&str],
    ) -> Result<Self, InvalidSemanticGraph> {
        let distinct: BTreeSet<_> = members.iter().collect();
        let well_formed = !members.is_empty()
            && distinct.len() == members.len()
            && members.iter().all(|case| is_identifier(case));
        if !well_formed {
            return Err(refuse(SemanticGraphCause::NonCanonicalPreimage));
        }
        let sorted = members
            .iter()
            .zip(members.iter().skip(1))
            .all(|(left, right)| left <= right);
        if !ordered && !sorted {
            return Err(refuse(SemanticGraphCause::UnsortedUnorderedMembers));
        }
        Ok(Self {
            key,
            ordered,
            members: members.iter().map(|case| String::from(*case)).collect(),
        })
    }

    /// The declaration node key.
    pub fn key(&self) -> NodeKey {
        self.key
    }

    /// Whether the declaration selects ordered semantics.
    pub fn is_ordered(&self) -> bool {
        self.ordered
    }

    /// Declaration-ordered (or case-sorted, if unordered) member cases.
    pub fn members(&self) -> &[String] {
        &self.members
    }

    /// The value of member `case`, whose admitted member node key is `member`.
    /// A case outside the declaration is `UndeclaredCase`.
    pub fn member(&self, case: &str, member: NodeKey) -> Result<EnumValue, InvalidSemanticGraph> {
        let position = self
            .members
            .iter()
            .position(|declared| declared == case)
            .ok_or(refuse(SemanticGraphCause::UndeclaredCase))?;
        Ok(EnumValue(Box::new(EnumValueFields {
            declaration: self.key,
            member,
            ordered: self.ordered,
            position,
            case: Box::from(case),
        })))
    }
}

/// An enumeration value: (declaration identity, member identity).
// Fields behind one `Box` (IR-286): `Value` carries this type inline, and
// CBMC's cost for moving a `Value` through any enum grows with the combined
// size of every inline variant payload.
#[derive(Clone)]
pub struct EnumValue(Box<EnumValueFields>);

#[derive(Clone)]
struct EnumValueFields {
    declaration: NodeKey,
    member: NodeKey,
    ordered: bool,
    position: usize,
    case: Box<str>,
}

impl core::fmt::Debug for EnumValue {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("EnumValue")
            .field("declaration", &self.0.declaration)
            .field("member", &self.0.member)
            .field("ordered", &self.0.ordered)
            .field("position", &self.0.position)
            .field("case", &self.0.case)
            .finish()
    }
}

impl EnumValue {
    /// Declaration node key.
    pub fn declaration(&self) -> NodeKey {
        self.0.declaration
    }

    /// Member node key.
    pub fn member(&self) -> NodeKey {
        self.0.member
    }

    /// Zero-based declaration position of the member.
    pub(crate) fn position(&self) -> usize {
        self.0.position
    }

    /// Whether the declaration is an `ordered enum`.
    pub fn is_ordered(&self) -> bool {
        self.0.ordered
    }

    /// The case identifier.
    pub fn case(&self) -> &str {
        &self.0.case
    }
}

/// Compare two enum values. Members of different declarations, and ordering
/// requests over an unordered declaration, are ill-typed and consume nothing.
pub fn compare_enum(
    operator: ComparisonOperator,
    left: &EnumValue,
    right: &EnumValue,
    meter: &mut Meter,
) -> Result<Outcome<bool>, IllTyped> {
    if left.0.declaration != right.0.declaration {
        return Err(IllTyped {
            cause: IllTypedCause::DistinctEnumDeclarations,
        });
    }
    if operator.is_ordering() && !left.0.ordered {
        return Err(IllTyped {
            cause: IllTypedCause::UnorderedEnumOrdering,
        });
    }
    Ok(Outcome::from_stop(compare(operator, left, right, meter)))
}

fn compare(
    operator: ComparisonOperator,
    left: &EnumValue,
    right: &EnumValue,
    meter: &mut Meter,
) -> Result<bool, Stop> {
    for read in 1..=2 {
        meter.charge(
            Charge::new(ChargePoint::EnumIdentityRead).size(LimitKind::ValueOccurrences, read),
        )?;
    }
    let ordering = if operator.is_ordering() {
        left.0.position.cmp(&right.0.position)
    } else {
        left.0.member.cmp(&right.0.member)
    };
    meter.charge(Charge::new(ChargePoint::EnumResultRetain).results(1))?;
    Ok(operator.holds(ordering))
}
