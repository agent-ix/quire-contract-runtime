---
id: SR-13325
title: "IR-882 PR 139 code review (with Rust lane): RT uses quire_exact::NodeKey directly"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@22e885d76b7c1bca523ed79f0129a102da6557cc vs origin/main 9567ff0; src/exact/{composite,containment,enumeration,equality,mod,node,reference,unit}.rs, tests/exact_*.rs (10 files)"
review_set: subset
---

## Summary

Ticket: IR-882 (key-owner slice of AD-004 step 4). PR agent-ix/quire-contract-runtime#139. The Rust lane is folded into this file. The diff deletes RT's local `NodeKey`, `from_bytes`, `from_hex`, the hex parser, `NODE_KEY_DOMAIN` and the `Display`/`Debug` impls from `src/exact/node.rs`. It drops `NodeKey` and `NODE_KEY_DOMAIN` from the `exact` re-export and repoints the eight library modules to `quire_exact::NodeKey`. It converts the fixture sites to `NodeKey::from_digest`, deletes the two FR-012-AC-1 tests and adds `tc_033_runtime_has_no_local_node_key_constructor`.

Measured by the reviewer at the head, in a fresh worktree with one target dir:

- Commit order is test first: `1cb1c54` adds the AC-6 inspection test, then `22e885d` makes the deletion and conversion. The local constructors and their two FR-012-AC-1 tests are deleted in the same commit.
- `make fmt-check lint test-features doc msrv size deny test`: one run, exit 0. The new test runs and passes.
- Library code constructs no `NodeKey`. A grep of `src/`, `verification/kani.rs`, `examples/`, `measurement/` and `scripts/` finds no `from_digest`, `decode_admitted`, `from_hex` or `from_bytes` call on a key. The only `src/` call is `src/exact/equality.rs:689`, inside `#[cfg(test)] mod tests`. RT has no build script and no generated code. `verification/kani.rs` names no `NodeKey`.
- No conversion layer or alias is added, and no kernel name is re-exported: no `pub use quire_exact`, no `type NodeKey`, no `as NodeKey`.
- 27 removed `NodeKey::from_bytes` lines: 26 converted fixture sites, plus the call inside the deleted round-trip test. `tests/exact_quantity_conversion.rs` and `tests/exact_semantics.rs` turn `const` keys into `fn`s because `from_digest` is not `const fn`. The reviewer checked all 28 name-to-byte mappings, and each is unchanged (`IN` became `inch_key()`).
- Tags use `#[trace(...)]` only; the diff adds no `Trace:` lines.
- `Cargo.toml` and `Cargo.lock` are unchanged, with one revision per first-party crate: quire-exact `1c6cbcb6`, quire-semantic-value `9fd13752`, quire-canonical `8b69bedc`.
- Kani, kani-mutations and deny-mutations were not run (lead's lane).

Reviewer mutation probes against `tc_033_runtime_has_no_local_node_key_constructor`. Each mutant was applied, that test run, and `src` restored:

| Mutant | Result |
| --- | --- |
| local `pub struct NodeKey` with `from_bytes`/`from_hex` in `node.rs` | killed |
| same local key in `key.rs` | survived |
| `NodeKey::from_digest(..)` in `unit.rs` production code | killed |
| `let mint = NodeKey::from_digest; mint(..)` in `unit.rs` | survived |
| `<NodeKey>::from_digest(..)` in `unit.rs` | survived |
| `quire_exact::NodeKey::from_digest(..)` in a new `src/exact/probe.rs` module | survived |
| a hex-to-`[u8; 32]` decoder named `decode_hex_key` in `unit.rs` | survived |
| `pub use quire_exact::NodeKey` in `src/lib.rs` | survived |
| `pub use quire_exact::NodeKey` in `src/exact/mod.rs` | killed |
| `NodeKey::decode_admitted(..)` in `reference.rs` | killed |
| `from_digest` call in `src/identity.rs` (outside `exact`) | survived |

Result: 4 killed, 7 survived.

## Examined criteria

- src/exact/node.rs (examined): "use quire_exact::{Integer, NodeKey};" with the local key, both constructors, the hex parser, `NODE_KEY_DOMAIN` and the formatting impls deleted.
- src/exact/mod.rs (examined): "pub use node::{InvalidSemanticGraph, SemanticGraphCause};"
- src/exact/{composite,containment,enumeration,reference,unit}.rs (examined): import repoint to `quire_exact::NodeKey`.
- src/exact/equality.rs:603-689 (examined): test-module fixture `NodeKey::from_digest([byte; 32])`.
- tests/exact_vocabulary.rs:13-48 (examined): `tc_033_runtime_has_no_local_node_key_constructor`.
- tests/exact_{collection,composite,debug_parity,equality,function_application,meter_state,outcomes,quantity_conversion,semantics}.rs (examined): fixture conversion.
- quire-exact 1c6cbcb6 src/node.rs:75-121 (context_only): "pub fn from_digest(digest: [u8; 32]) -> Self", "pub fn decode_admitted(digest: [u8; 32]) -> Self", with `Display`/`Debug` identical to the deleted RT impls.
- AD-004 Q-5 option A and step 4 (context_only): "RT holds shared `NodeKey`s ... but constructs none in library code ... Test-only RT fixtures may use `NodeKey::from_digest([u8; 32])`".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-012-AC-6 test is a substring scan over a hand-listed set of 16 `src/exact` files, and it checks for a local `NodeKey` type only in `node.rs`. Seven of eleven reviewer mutants survive: a local `NodeKey` in another file, a constructor reached through a fn pointer or `<NodeKey>::`, a call in a new `src/exact` module or anywhere outside `src/exact`, a hex decoder not named `from_hex`, and a re-export from `lib.rs`. Mutants that survive break AC-6's "no RT-local `NodeKey`" and "RT library code has no `NodeKey` construction or hex decoder" clauses. The test also cuts each file at its first `#[cfg(test)]`, so production code after a test module, or a comment containing that text, goes unchecked | tests/exact_vocabulary.rs:13-48 |
| FND-002 | low | The new `quire_exact::NodeKey` import is added as a separate `use` after the `super::` imports rather than merged into the module's existing `use quire_exact::{..}` group. This affects five library modules and nine test files, which now carry two `use quire_exact` lines; `tests/exact_quantity_conversion.rs` puts it above the `#[path]` module declaration. `node.rs` merges it correctly | src/exact/composite.rs:25,36; src/exact/unit.rs:18,21; src/exact/enumeration.rs:15,21; src/exact/reference.rs:19,24; tests/exact_quantity_conversion.rs:8 |

## Verdict

**PASS with findings.** The code change is correct and minimal: there is one key owner, and RT library code constructs no key. Nothing restores the deleted key, whether a conversion, an alias or a re-export. The retired AC-1 tests were deleted with their code, the fixtures were converted faithfully, the lock was untouched and all requested gates pass. FND-001 should be fixed in this PR before merge, because the matrix credits this test with AC-6 (see SR-13326). A fix would walk `src/` recursively with `std::fs`, flag any `struct NodeKey` or `enum NodeKey`, and flag any `from_digest`, `decode_admitted`, `from_hex` or `from_bytes` token after stripping `#[cfg(test)]` modules properly. Keep the hand list only as a cross-check. FND-002 is a style nit.

## Dispositions

Round 1 checked the fix commit `abe9f4a` at head `abe9f4ac6c95b5d7d483682975ea7c582473a59f`. The reviewer re-ran the gates on a fresh worktree: `make fmt-check lint test-features doc msrv size deny test` exited 0. `make spec` reports 235/235 grammar-clean, and strict coverage has 125 unbacked rows and 0 contradicted, unchanged from the first pass.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The test now walks every `.rs` file under `src/`, `verification/`, `examples/`, `measurement/` and `scripts/` with `syn`, skipping only items marked `#[cfg(test)]`. All 11 first-pass mutants now fail it, each with the expected message, including the 7 that survived before |
| FND-002 | fixed | `NodeKey` was merged into the existing `use quire_exact::{..}` groups in composite, enumeration, reference and unit and in all test files. Each file now has one `use quire_exact` line |
| FND-003 | still-open | New in round 1; see below |
| FND-003 | fixed | Round 2 (fix commit `f933788`, head `f9337880da592df5ceeec75eeb6adb4327b7ff99`), measured by the reviewer. `visit_macro` now scans each macro's token stream for the constructor and hex-decoding names and for `struct`/`enum`/`type`/`union NodeKey`. The digest-signature check now also accepts `&[u8]` input. TC-033 step 1 now describes only the checks the test performs. All 15 reviewer mutants (11 first-pass and 4 round-1) fail the test, each with the expected message. The gates were re-run and exited 0. The dev-dependency change is `proc-macro2 = "1"` plus `syn = "2"` as a caret requirement; the lock adds no package and no new first-party revision (quire-exact is locked once), and `make deny` passed |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The `syn` visitor never looks inside macro invocations, because `syn` keeps macro bodies as raw tokens. Three round-1 reviewer mutants survive: `alloc::vec![NodeKey::from_digest([0; 32])]` in `unit.rs` production code; a `macro_rules!` that expands to a local `struct NodeKey` in `key.rs`; and a `&[u8]`-to-`Option<[u8; 32]>` nibble decoder named `decode_digest`. TC-033 step 1 says the inspection covers "any constructor reference or call" and "a hex decoder under any name", which claims more than the test checks. At this head, AC-6 still holds: a grep finds no constructor outside `#[cfg(test)]` (`src/exact/equality.rs:689`). A fix could scan each `syn::Macro` token stream for the constructor idents and `NodeKey` definitions, or the TC-033 wording could be narrowed | tests/exact_vocabulary.rs:84-196; spec/exact/matrix/TC-033-carried-compiler-vocabulary.md:19-23 |
