---
id: SR-001
title: "Runtime v0.1 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "runtime requirements, implementation, tests, evidence, and release gates"
review_set: subset
---

# Runtime v0.1 gap analysis

## Summary

The runtime requirements and implementation have no unresolved semantic gap after two executed
source-review rounds. The candidate remains gated by upstream governance reconciliation, current
protected checks and review, and the human source-release decision.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-012 | high | PGM-01 review and the final merged identity remain open. | PGM-01, REV-002 |
| FND-013 | medium | The rebased manual-CI head has no deliberately dispatched protected run. | MP-001, PR #5, PR #6 |
| FND-014 | medium | CODEOWNER approval and the human source-release decision remain pending. | AA-001, REV-003 |

## Source-review disposition

| Review finding | Disposition |
|---|---|
| FND-001 | Removed Rust 1.83-only mutating `const fn`s and made both local and remote MSRV gates invoke `cargo +1.75.0` explicitly. |
| FND-002 | Replaced the conditional `rg` gate with a status-aware `grep` audit that exits 2 when its scanner is unavailable or errors. |
| FND-003 | Made report and counter state private; exposed read-only accessors and a typed identity-mismatch result. |
| FND-004 | Added five downstream compile-fail doctests plus TC-008 source-policy coverage for every non-exhaustive enum. |
| FND-005 | Replaced the compiler-sensitive rlib ceiling with a fail-closed linked-section footprint gate fixed to Rust 1.75 and `thumbv7em-none-eabi`; rlib bytes remain an observation. |
| FND-006 | Added a default-profile compile-fail doctest proving `proptest_adapter` is absent without the feature. |
| FND-007 | Added `adapt_recording`, which records the full campaign census before returning the proptest result. |
| FND-008 | Replaced the Boolean mismatch signal with `IdentityMismatch`, retaining expected and actual identities. |
| FND-009 | Removed ignored nightly-only rustfmt options and aligned contributor guidance with stable rustfmt. |
| FND-010 | Classified reserved `alloc`/`std` rows explicitly as resolver/build compatibility checks. |
| FND-011 | Added a crate-wide `missing_docs` denial. |
| GAP-001 | Authored TC-007 and TC-008 with exact procedures and evidence locations. |
| GAP-002 | Added requirement implementation bindings and now reports Quire's measures separately: 66/103 production symbols are owned, while zero test symbols carry an unbound trace ID. The 37 deliberately unowned symbols are generated enum variants, macro-expanded trait methods, private helpers, and measurement plumbing. |
| GAP-003 | Upstream schema/selector contradiction is being corrected by `agent-ix/spec-artifacts-process#77`; this repository retains the structurally valid column until that draft lands. |
| GAP-004 | Added exact stakeholder and inspection bindings; coverage is now 27/27 with 13/13 Rust candidates tagged and bound. |
| GAP-005 | Ran the complete local gate on the current source; immutable records intentionally describe their clean parent source revision, and final merge evidence remains a release task. |
| GAP-006 | Expanded `make ci` to check all targets/features at the explicit MSRV, build a fixed bare-metal consumer, and enforce its linked-section footprint. |
| GAP-007 | Added `plan/PLAN-001-runtime-v01/` with five completed typed tasks and one explicitly human-owned open task. |

## Re-review disposition

| Review finding | Disposition |
|---|---|
| NEW-001 | Replaced the unsatisfiable rlib ceiling with a representative `no_std` static-library consumer built by Rust 1.75 for `thumbv7em-none-eabi`. The fixed, non-overridable gate measures only runtime/harness `.text` plus `.rodata`: 590 bytes against 4,096. |
| NEW-002 | Removed the Makefile-text assertion. TC-007's procedure consumes the executed `make ci` output instead of claiming target behavior from a target-name string. |
| NEW-003 | Annotated the mutable historical README: the retained `msrv_1_75=SUCCESS` is invalidated by the toolchain-precedence defect, while the Kani result applies only to its historical revision. |
| NEW-004 | Replaced the overstated traceability row with the engine's 66/103 production-symbol measure and an explicit classification of deliberately unowned symbol classes. |
| NEW-005 | Changed `adapt_recording` to return `TestCaseResult`; identity mismatch is now a proptest failure containing expected and observed identities, with a direct regression test. |
| NEW-006 | Denied Clippy indexing and arithmetic-side-effect lints crate-wide, made indexing use `slice::get`, and extended the fail-closed text audit to verification source while permitting Kani proof assertions. |
| NEW-007 | Anchored all five enum declarations exactly and asserted distinct `VerdictKind`/`Verdict` offsets. |
| NEW-008 | Added a public `&mut self` method allowlist for accounting types plus built-in ordinary/`const` setter mutation probes. |
| NEW-009 | Removed the attribute-order string assertion; the compile-fail doctest remains the behavioral feature-absence proof. |
| NEW-010 | Implemented allocation-free `Display` for `IdentityMismatch`. |
| NEW-011 | Removed the environment-overridable threshold; the linked footprint script contains the fixed target and 4,096-byte ceiling. |
| NEW-012 | Converted every `Implements:` annotation from a doc comment to an ordinary source comment; Quire retains the bindings without publishing them in rustdoc. |
| NEW-013 | Moved the private saturation test into an explicitly named test-only source file excluded from the shipped-source panic scan, restoring normal assertive diagnostics. |

The residual noted under FND-001 is also closed: `make msrv` now checks all targets and features with
Cargo 1.75, covering optional features, examples, and test targets rather than only the
dependency-free library profile.

Date: 2026-08-30

Candidate source revision: `b5ce806a9f4f3315ce89e77bc80e816d696d904c`

Evidence: the immutable record named for this reconciliation commit is added by the immediately
following evidence-only commit.

## Requirement audit

| Requirement | Authoritative evidence | Result |
|---|---|---|
| #2 baseline, dual license, publication lock | `Cargo.toml`, both license files, `deny.stdout`, protected `main` API response | pass |
| #2 stakeholder/functional/non-functional/interface/test requirements | 23 documents under `spec/`; `quire-validate` output | pass |
| #2 composite review and assurance artifacts | `planning/foundation-review.md`; AP/AD/CAC/MP/AA under `spec/assurance/` | pass |
| #2 implementation plan and dependency DAG | `plan/PLAN-001-runtime-v01/`; typed Task-001 through Task-006 | pass; human Task-006 open by design |
| #1 three distinct terminal verdicts | `src/verdict.rs`; TC-001 output | pass |
| #1 identity, observations, structured details | `src/identity.rs`, `src/observation.rs`; TC-001 output | pass |
| #1 default no_std/allocation-free surface | `#![no_std]`, empty default feature set, default dependency tree containing only this crate | pass |
| #1 size, panic, feature, compatibility contracts | crate/README docs, semantic Clippy lints, shipped/Kani panic audit, feature matrix, layout, fixed-target linked footprint, and observational rlib measurement | pass |
| #1 permissive generated-code surface | default dependency tree empty; crate `MIT OR Apache-2.0` | pass |
| #3 short-circuit and total operators | `src/operators.rs`; exhaustive TC-002 truth/evaluation tests | pass |
| #3 safe option/index/arithmetic/division helpers | checked sealed trait; boundary/property TC-003 tests | pass |
| #3 optional proptest mapping | pinned proptest feature; TC-004 maps and records pass/fail/reject distinctly | pass |
| #3 complete per-requirement accounting | opaque `CampaignReport`; typed mismatch; TC-006 mixed and saturation tests | pass |
| Acceptance-criterion traceability | 27/27 rows backed; 14/14 Rust test symbols bound; 66/103 production symbols owned; 37 deliberately unowned generated/private/tooling symbols; zero unbound test trace IDs | pass with explicit implementation-ownership boundary |
| #3 Kani harness coverage | five checked-in proofs; pinned Kani 0.67.0 CI result | pass |
| Epic local gates and measurements | local `make ci`; local input/manifest schema gates; exact PGM-01 schema and custom-validator gates; revision-bound MP-001 record | pass |
| Protected remote gates | successful checks for pre-reconciliation revision retained under `evidence/historical/`; rebased candidate run | pending deliberate dispatch |

## Gap disposition

No unresolved implementation or specification gap was found. The source candidate's representative
bare-metal linked footprint is 590 bytes against the fixed 4,096-byte ceiling, and every declared
target/feature compiles under the explicit Rust 1.75 lane. It has no default normal dependency,
contains no unsafe or intentional runtime panic surface, and passes every locally available gate.

The following are release/workflow gates, not silently accepted gaps:

1. PGM-01 (`agent-ix/quire-contract-ir#3`) is in review at PR #12. This candidate reconciles exact
   revision `942670a0db78be57cfa9bdd6d04302b453781a49` and its envelope schema digest, but must reconcile
   again after the policy merges.
2. Local Kani was unavailable and remains truthfully recorded as `skipped-unavailable`. Pinned Kani
   0.67.0 executed all five proofs successfully for a historical pre-reconciliation revision; the
   rebased candidate requires a fresh manual CI dispatch.
3. Manual-CI PR #6 must merge, protected checks must pass on the rebased candidate, and protected
   code-owner review must complete on runtime PR #5.
4. The human release owner must record the v0.1 decision in `planning/release-decision.md` after merge
   evidence is collected. No agent or automated gate may substitute for that decision.

Implementation gap-analysis result: **pass, with PGM-01 merge/reconciliation, current protected
checks, code-owner review, and the human release decision still open**.
