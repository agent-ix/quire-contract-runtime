---
id: SR-35017
title: "code-review and rust-review of IR-349 RT scalar move (quire-contract-runtime PR 160)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@3f6ef45d; src/lib.rs; src/operators.rs; src/exact/mod.rs; src/exact/expression.rs; src/scalar/mod.rs; src/scalar/boolean.rs; src/scalar/division.rs; src/scalar/ieee.rs; tests/exact_function_application.rs; tests/exact_meter_state.rs; tests/exact_negotiation.rs; tests/exact_outcomes.rs; README.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: references
---

# Code review and Rust review of the IR-349 RT scalar move (PR 160)

## Summary

Ticket: IR-349. Frozen head 3f6ef45d, one commit on origin/main fe673bef. The Rust-review lane is folded in here.

The move is complete and exact. `src/exact/{boolean,division,ieee}.rs` are git renames (similarity 93-97%) to `src/scalar/`. `src/scalar/mod.rs` re-exports exactly 11 items: `evaluate_boolean_short_circuit`, `ShortCircuitConnective`, `negotiate_integer_division`, `IntegerDivisionBounds`, `IntegerDivisionConsumer`, `IntegerDivisionDisposition`, `negotiate_ieee`, `IeeeBackendCapabilities`, `IeeeDisposition`, `IeeeItemRequirement` and `IeeeUnsupportedCause`. `src/exact/mod.rs` drops all three `mod` lines and their `pub use` blocks. No alias, re-export or wrapper of any moved item stays at the `exact` path, so there is no compatibility layer. `expression.rs` imports the two types it needs from `crate::scalar`. A repo-wide grep finds no `exact::` path, and no `src/exact/{boolean,division,ieee}.rs` reference, outside historical `reviews/` files and the AD-004 measurement tables.

The only body changes are dropping `#[non_exhaustive]` from the five kept enums, which AD-004 Q-4 and step 3 direct, plus doc and `expect` reason wording. Behaviour is otherwise unchanged. The one test-body change (`tc_030_every_disposition_is_a_known_variant`) replaces `matches!` plus `assert!` with exhaustive `match`es. That makes it stronger, because a new variant is now a compile error even when no fixture reaches it. No test was removed and no `#[trace]` tag changed.

The `IeeeDisposition` `compile_fail` doctest got better. On main it imported `quire_contract_runtime::exact::Value`, which `exact` does not export, so it failed on an unresolved import and proved nothing. At head it imports `Value` from `quire_exact`. A reviewer probe compiled the doctest body without the conversion line cleanly, and with it got only `E0277: Outcome<Value>: From<IeeeDisposition> is not satisfied`. The doctest is now non-vacuous.

Panic, unsafe and footprint: no new arithmetic, indexing, `unwrap`, `expect` or panic site. `#![forbid(unsafe_code)]` still holds. `scalar` uses `alloc::{vec::Vec, collections::BTreeSet}` only, under `#[cfg(feature = "exact")]`, and `exact` implies `alloc`.

Measured at head with one target dir and `RUST_TEST_THREADS=4`:
- `cargo build --locked --no-default-features --features exact --lib --target thumbv7em-none-eabi`: ok.
- `cargo clippy --locked --features exact --all-targets -D warnings`: clean. This covers every test file, not only the four touched, so no stale wildcard arm over the now-exhaustive enums remains anywhere in the crate.
- `cargo test --features exact` over the four touched test files: 45 passed (31+1+12+1).
- `cargo test --features exact --doc`: 13 passed.
- `make spec` coverage: 93 unbacked and 0 contradicted at both head and main. The only output difference is the moved path of the pre-existing `FR-009-AC-5` container-tag diagnostic.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Approve the code. No correctness, idiom, panic-surface, unsafe, footprint or test-strength defect in the Rust diff. Exactly 11 items moved, none duplicated or aliased, and no test was weakened. Residual cross-repo risk, by design in AD-004 ("The RT PR therefore merges first"): CG main still names `quire_contract_runtime::exact::evaluate_boolean_short_circuit` and `ShortCircuitConnective` (`src/core/profile.rs:77-78`; the emitted `use quire_contract_runtime::exact as residue;` in `src/oracle/scalar/mod.rs:3813`). Those break when CG's RT lock moves, and no paired CG PR is open, so IR-349 step 3 is not complete when this PR merges.
