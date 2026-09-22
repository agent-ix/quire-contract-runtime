// SPDX-License-Identifier: AGPL-3.0-or-later
//! The closed static-refusal vocabularies of definition selection and package
//! admission.
//!
//! Parsing the definition lock, admitting a selection and checking a
//! definition closure are compiler work. The runtime never repeats that
//! decision; it carries these exact spellings so a generated oracle can report
//! a compiler refusal it was handed without inventing a code.

use core::fmt;

/// The lock's closed `selection_refusal_codes`, in check order.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SelectionRefusalCode {
    /// `selection_unknown_trigger`.
    SelectionUnknownTrigger,
    /// `selection_duplicate_trigger`.
    SelectionDuplicateTrigger,
    /// `selection_unknown_role`.
    SelectionUnknownRole,
    /// `selection_duplicate_role`.
    SelectionDuplicateRole,
    /// `selection_required_missing`.
    SelectionRequiredMissing,
    /// `selection_alternative_conflict`.
    SelectionAlternativeConflict,
    /// `selection_trigger_unsatisfied`.
    SelectionTriggerUnsatisfied,
    /// `selection_untriggered_profile`.
    SelectionUntriggeredProfile,
}

impl SelectionRefusalCode {
    /// Every code in normative check order.
    pub const ALL: [Self; 8] = [
        Self::SelectionUnknownTrigger,
        Self::SelectionDuplicateTrigger,
        Self::SelectionUnknownRole,
        Self::SelectionDuplicateRole,
        Self::SelectionRequiredMissing,
        Self::SelectionAlternativeConflict,
        Self::SelectionTriggerUnsatisfied,
        Self::SelectionUntriggeredProfile,
    ];

    /// Normative spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectionUnknownTrigger => "selection_unknown_trigger",
            Self::SelectionDuplicateTrigger => "selection_duplicate_trigger",
            Self::SelectionUnknownRole => "selection_unknown_role",
            Self::SelectionDuplicateRole => "selection_duplicate_role",
            Self::SelectionRequiredMissing => "selection_required_missing",
            Self::SelectionAlternativeConflict => "selection_alternative_conflict",
            Self::SelectionTriggerUnsatisfied => "selection_trigger_unsatisfied",
            Self::SelectionUntriggeredProfile => "selection_untriggered_profile",
        }
    }

    /// Resolve a spelling.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|refusal| refusal.as_str() == code)
    }
}

/// The I04 diagnostic code of a definition-closure refusal.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PackageRefusalCode {
    /// `invalid_package`.
    InvalidPackage,
}

impl PackageRefusalCode {
    /// Normative spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPackage => "invalid_package",
        }
    }
}

/// The subset of the closed I04 `cause_tag` vocabulary that definition-closure
/// admission reports.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PackageCause {
    /// `missing-member`: a required definition is not retained.
    MissingMember,
    /// `conflicting-definition`: more than one alternative is retained, or a
    /// user declaration binds a reserved intrinsic identity.
    ConflictingDefinition,
    /// `incompatible-definition`: the retained definition is not the required
    /// kind, or `mod` claims a non-Euclidean law.
    IncompatibleDefinition,
    /// `revision-mismatch`.
    RevisionMismatch,
    /// `digest-domain-mismatch`.
    DigestDomainMismatch,
    /// `byte-digest-mismatch`.
    ByteDigestMismatch,
}

impl PackageCause {
    /// Every cause in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::MissingMember,
        Self::ConflictingDefinition,
        Self::IncompatibleDefinition,
        Self::RevisionMismatch,
        Self::DigestDomainMismatch,
        Self::ByteDigestMismatch,
    ];

    /// Normative spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingMember => "missing-member",
            Self::ConflictingDefinition => "conflicting-definition",
            Self::IncompatibleDefinition => "incompatible-definition",
            Self::RevisionMismatch => "revision-mismatch",
            Self::DigestDomainMismatch => "digest-domain-mismatch",
            Self::ByteDigestMismatch => "byte-digest-mismatch",
        }
    }
}

/// `refused { code, cause }` at semantic admission.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PackageRefusal {
    /// Diagnostic code.
    pub code: PackageRefusalCode,
    /// Typed cause.
    pub cause: PackageCause,
}

impl fmt::Display for PackageRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code.as_str(), self.cause.as_str())
    }
}
