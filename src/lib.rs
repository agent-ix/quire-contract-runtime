//! Small `no_std` runtime support for generated contract oracles and harness verdicts.
//!
//! # Contracts
//!
//! - **Features:** the default feature set is empty. `alloc` and `std` reserve opt-in convenience
//!   surfaces, `std` implies `alloc`, and `proptest` adds only the optional test adapter.
//! - **Allocation:** the core modules neither allocate nor require a global allocator.
//! - **Panic:** public evaluation and accounting operations contain no intentional panic path. An
//!   undefined partial operation returns `None`, and counters saturate.
//! - **Size:** core values contain only fixed-size enums, integers, and borrowed slices/strings.
//!   Their exact byte size is target-dependent and is measured for each release candidate.
//! - **Compatibility:** public data enums are non-exhaustive. Consumers must retain unknown future
//!   states rather than converting them to success.
//! - **Safety:** the crate forbids unsafe code.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
#![cfg_attr(
    not(feature = "proptest"),
    doc = r#"
## Default feature surface

The proptest adapter is not available without its opt-in feature:

```compile_fail
use quire_contract_runtime::proptest_adapter;
```
"#
)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Implements: FR-004
pub mod accounting;
// Implements: FR-001
pub mod identity;
// Implements: FR-001
pub mod observation;
// Implements: FR-002
pub mod operators;
#[cfg(feature = "proptest")]
// Implements: FR-003
pub mod proptest_adapter;
// Implements: FR-001
pub mod verdict;

#[cfg(kani)]
#[path = "../verification/kani.rs"]
mod kani_proofs;

pub use accounting::{CampaignCounts, CampaignReport, IdentityMismatch};
pub use identity::{ClauseId, ContractIdentity, ExecutionPoint, RequirementId, RevisionId};
pub use observation::{ClauseKind, ClauseOutcome, FailureDetail, FailureKind, Observation};
pub use verdict::{Verdict, VerdictContext, VerdictKind};

/// Version of the documented public layout and semantic contract.
///
// Implements: FR-001
pub const RUNTIME_CONTRACT_VERSION: &str = "quire-contract-runtime-v1";
