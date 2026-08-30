//! Small `no_std` runtime support for generated contract oracles and harness verdicts.
//!
//! # Contracts
//!
//! - **Features:** the default feature set is empty. `alloc` exposes owned convenience types, `std`
//!   implies `alloc`, and `proptest` adds only the optional test adapter.
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

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod accounting;
pub mod identity;
pub mod observation;
pub mod operators;
#[cfg(feature = "proptest")]
pub mod proptest_adapter;
pub mod verdict;

pub use accounting::{CampaignCounts, CampaignReport};
pub use identity::{ClauseId, ContractIdentity, ExecutionPoint, RequirementId, RevisionId};
pub use observation::{ClauseKind, ClauseOutcome, FailureDetail, FailureKind, Observation};
pub use verdict::{Verdict, VerdictContext, VerdictKind};

/// Version of the documented public layout and semantic contract.
pub const RUNTIME_CONTRACT_VERSION: &str = "quire-contract-runtime-v1";
