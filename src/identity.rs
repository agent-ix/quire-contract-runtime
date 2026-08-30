//! Borrowed identities retained by every observation, verdict, and report.

macro_rules! borrowed_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        ///
        // Implements: FR-001
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name<'a>(&'a str);

        impl<'a> $name<'a> {
            /// Creates an identity without allocation or normalization.
            ///
            // Implements: FR-001
            #[must_use]
            pub const fn new(value: &'a str) -> Self {
                Self(value)
            }

            /// Returns the exact source identity.
            ///
            // Implements: FR-001
            #[must_use]
            pub const fn as_str(self) -> &'a str {
                self.0
            }
        }

        impl core::fmt::Display for $name<'_> {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(self.0)
            }
        }
    };
}

borrowed_id!(
    /// Stable requirement identifier, for example `FR-001`.
    RequirementId
);
borrowed_id!(
    /// Source revision identifier supplied by the generated artifact.
    RevisionId
);
borrowed_id!(
    /// Named program point at which the contract is evaluated.
    ExecutionPoint
);
borrowed_id!(
    /// Stable identity of one clause within a requirement revision.
    ClauseId
);

/// Requirement and revision identity shared by all runtime evidence.
///
// Implements: FR-001
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ContractIdentity<'a> {
    /// Stable requirement identifier.
    ///
    // Implements: FR-001
    pub requirement: RequirementId<'a>,
    /// Exact requirement/source revision.
    ///
    // Implements: FR-001
    pub revision: RevisionId<'a>,
}

impl<'a> ContractIdentity<'a> {
    /// Creates a requirement/revision pair.
    ///
    // Implements: FR-001
    #[must_use]
    pub const fn new(requirement: RequirementId<'a>, revision: RevisionId<'a>) -> Self {
        Self {
            requirement,
            revision,
        }
    }
}
