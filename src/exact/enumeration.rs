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
        Ok(EnumValue {
            declaration: self.key,
            member,
            ordered: self.ordered,
            position,
            case: Box::from(case),
        })
    }
}

/// An enumeration value: (declaration identity, member identity).
#[derive(Clone, Debug)]
pub struct EnumValue {
    declaration: NodeKey,
    member: NodeKey,
    ordered: bool,
    position: usize,
    case: Box<str>,
}

impl EnumValue {
    /// Declaration node key.
    pub fn declaration(&self) -> NodeKey {
        self.declaration
    }

    /// Member node key.
    pub fn member(&self) -> NodeKey {
        self.member
    }

    /// Zero-based declaration position of the member.
    pub(crate) fn position(&self) -> usize {
        self.position
    }

    /// Whether the declaration is an `ordered enum`.
    pub fn is_ordered(&self) -> bool {
        self.ordered
    }

    /// The case identifier.
    pub fn case(&self) -> &str {
        &self.case
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
    if left.declaration != right.declaration {
        return Err(IllTyped {
            cause: IllTypedCause::DistinctEnumDeclarations,
        });
    }
    if operator.is_ordering() && !left.ordered {
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
        left.position.cmp(&right.position)
    } else {
        left.member.cmp(&right.member)
    };
    meter.charge(Charge::new(ChargePoint::EnumResultRetain).results(1))?;
    Ok(operator.holds(ordering))
}
