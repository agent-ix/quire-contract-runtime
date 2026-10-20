---
id: SR-13328
title: "IR-882 PR 140 code review (with Rust lane): RT consumes QSV semantic-node refusals directly"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@2a5119f8dab783682df44a8e235f86cfaceda11c vs origin/main f5c3570; src/exact/{enumeration,mod,node,unit}.rs, tests/{exact_meter_state,exact_vocabulary,ir882_qsv_vocabulary}.rs"
review_set: subset
---

## Summary

Ticket: IR-882 (semantic-node slice of AD-004 step 4). PR agent-ix/quire-contract-runtime#140. The Rust lane is folded into this file. The diff deletes `src/exact/node.rs` (114 lines: `InvalidSemanticGraph`, `SemanticGraphCause`, `refuse`, `is_identifier`, `check_terms`), drops `mod node` and `pub use node::{InvalidSemanticGraph, SemanticGraphCause}`, and repoints `unit.rs` and `enumeration.rs` to `quire_semantic_value::semantic_node` and `quire_exact::is_identifier`. Two test files repoint their imports, and one new test is added.

Measured by the reviewer at the head, in a fresh worktree with one target dir:

- Owner equivalence, diffed against the locked sources (quire-semantic-value `9fd13752` `src/semantic_node.rs`, quire-exact `1c6cbcb6` `src/node.rs`):
  - `InvalidSemanticGraph`: same single `pub cause` field, same derives (`Clone, Copy, Debug, Eq, Hash, PartialEq`), same `CODE = "invalid_semantic_graph"`. `Display` is `#[error("invalid_semantic_graph: {cause:?}")]`, byte-identical to RT's `write!`. The owner also implements `Error` (additive).
  - `SemanticGraphCause`: the same 20 variants in the same declaration order, with the same names (so derived `Debug` is identical) and the same derives. Neither type derives `Ord`. The owner drops `#[non_exhaustive]`, which matches AD-004 Q-4 ("follow the shared crate, none").
  - `check_terms`: same three checks in the same order (zero, then duplicate, then not strictly ascending). The owner is generic over `K: Ord`, and `is_sorted_by(|l, r| l < r)` is equivalent to RT's zip-all `<`.
  - `is_identifier`: quire-exact's body is byte-identical to the deleted one.
  - `refuse` was a one-line private constructor. It is replaced by struct literals, not by a new helper.
- No adapter, alias, conversion or re-export: there is no `pub use quire_`, no `type ... =` and no wrapper. RT signatures now return the owner `InvalidSemanticGraph`.
- Callers: a grep of `src`, `tests`, `verification` and `scripts` finds no remaining `node::` path. Every `SemanticGraphCause`/`InvalidSemanticGraph` use names the QSV path.
- Retired tests: `node.rs` had no test module, and no RT test exercised the deleted private helpers directly, so nothing needed to be retired.
- Commit order: `0cb88a2` adds `tests/ir882_qsv_vocabulary.rs`, and it is red at that commit (reviewer ran it: it panics at `tests/ir882_qsv_vocabulary.rs:16`, on `src/exact/definition.rs`). `2a5119f` makes the deletion and rewrites that test (see FND-003).
- Tags use `#[trace(...)]` only. The diff adds no `Trace:` lines.
- `Cargo.toml` and `Cargo.lock` are unchanged, with one revision per first-party crate: quire-exact `1c6cbcb6`, quire-semantic-value `9fd13752`, quire-canonical `8b69bedc`, ix-trace-rs `9378a677`.
- `make fmt-check lint test-features doc msrv size deny test`: one run, exit 0. `qsv_owns_semantic_node_vocabulary` runs and passes. Footprint: 973 bytes, 0 panic references.
- Reviewer mutant: an RT-local `SemanticGraphCause` and `InvalidSemanticGraph` in a new `src/exact/semantic_copy.rs`, declared in `mod.rs`. It survives both `ir882_qsv_vocabulary` and `exact_vocabulary`.
- Kani, kani-mutations and deny-mutations were not run (lead's lane).

## Examined criteria

- src/exact/node.rs (examined): deleted whole; "pub struct InvalidSemanticGraph { pub cause: SemanticGraphCause }", 20-variant `#[non_exhaustive] pub enum SemanticGraphCause`, `pub(crate) fn refuse`, `pub(crate) fn is_identifier`, `pub(crate) fn check_terms(terms: &[(NodeKey, Integer)])`.
- src/exact/mod.rs (examined): "mod node;" and "pub use node::{InvalidSemanticGraph, SemanticGraphCause};" deleted; module doc reworded.
- src/exact/unit.rs (examined): "use quire_semantic_value::semantic_node::{check_terms, InvalidSemanticGraph, SemanticGraphCause};" and 14 `refuse(..)` sites rewritten as struct literals.
- src/exact/enumeration.rs (examined): "is_identifier, ..." from `quire_exact`; three `refuse(..)` sites rewritten.
- tests/exact_meter_state.rs, tests/exact_vocabulary.rs (examined): import repoint only.
- tests/ir882_qsv_vocabulary.rs (examined): "#[trace(\"FR-012-AC-4\")] fn qsv_owns_semantic_node_vocabulary()".
- FR-012-AC-4 (context_only): "Each of the thirteen graph causes is raised by an admissible input to `UnitGraph::admit`, and `UndeclaredCase` by reading a case against its declaration; ..."
- quire-semantic-value 9fd13752 src/semantic_node.rs (context_only): owner `InvalidSemanticGraph`, `SemanticGraphCause`, `check_terms<K: Ord>`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `qsv_owns_semantic_node_vocabulary` is tagged `FR-012-AC-4`, but it asserts nothing AC-4 states. AC-4 is about raise sites through `UnitGraph::admit`, `EnumDeclaration::new` and `member`, and it is already backed by 17 TC-033 tests. The new test checks only that the path `src/exact/node.rs` does not exist, plus a property of QSV's own `check_terms`. So it is a wrong binding. As an ownership check it is vacuous: a reviewer mutant re-adding RT-local `SemanticGraphCause` and `InvalidSemanticGraph` in another file passes it. Either remove the AC-4 tag, or make it a real inspection (extend the `syn` walk in `tc_033_runtime_has_no_local_node_key_constructor` to flag RT definitions of `SemanticGraphCause`, `InvalidSemanticGraph` and `check_terms`) bound to an AC that states ownership | tests/ir882_qsv_vocabulary.rs:8-17 |
| FND-002 | low | A stale comment says "`check_terms` is `pub(crate)`, so the three term causes are reached through `admit`". The function is now QSV's public `check_terms`, so the stated reason is false | tests/exact_vocabulary.rs:353-356 |
| FND-003 | low | The test-first commit's test is not the shipped test. At `0cb88a2` it is `qsv_owns_definition_and_semantic_node_vocabulary`, tagged `TC-197`, and it also requires `definition.rs` to be absent (it is red on that file, which this PR keeps). `2a5119f` renames it, retags it `FR-012-AC-4` and narrows it. The PR body's claim that `qsv_owns_semantic_node_vocabulary` "was committed red first" does not hold in that form | tests/ir882_qsv_vocabulary.rs |

## Verdict

**PASS with findings.** The deletion is semantically lossless. Every deleted item has a locked owner equivalent with the same variants, payload, `Display`, `Debug`, code string and check order. The one difference is the dropped `#[non_exhaustive]`, which AD-004 Q-4 rules for. No adapter, alias or re-export was added, all callers name the owner path, the lock is untouched and all requested gates pass. Fix FND-001 in this PR before merge, because it is a false trace credit. FND-002 is a one-line comment fix. FND-003 needs a corrected PR-body claim, not a code change.

Pre-existing and outside the diff (not a finding): `UnitGraph::compound_unit` (`src/exact/unit.rs:271-277`) still maps every non-term cause to `UnsortedTerms` through a `_` arm. That arm was needed while RT's enum was `#[non_exhaustive]`; Q-4 now allows it to be exhaustive.
