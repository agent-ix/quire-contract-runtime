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

The runtime requirements and implementation have no unresolved semantic gap after ten executed
source-review rounds. PR #5 is merged at the exact reviewed tree, and every local merged-main gate,
including pinned Kani 0.67.0, passes. Hosted checks and the human source-release decision remain open.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-013 | medium | Merged `main` has no deliberately dispatched hosted run after manual-CI PR #6 merged; hosted CI is explicitly deferred while ticket work continues locally. | MP-001, PR #5, PR #6 |
| FND-014 | medium | PR #5 was operator-authorized for admin merge after Round 10 cleared every finding; GitHub records no approval, and the human source-release decision remains pending. | AA-001, REV-003 |
| FND-015 | medium | The manual-only hosted workflow does not yet execute the local rustdoc and evidence-tool gates; its externally owned reconciliation remains pending. | MP-001, PR #5, PR #6 |

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
| GAP-002 | Added requirement implementation bindings and now reports Quire's measures separately: 70/137 production symbols are owned, while zero test symbols carry an unbound trace ID. The 67 deliberately unowned symbols are generated enum variants, macro-expanded trait methods, private helpers, test support, and measurement plumbing. |
| GAP-003 | Upstream schema/selector contradiction is being corrected by `agent-ix/spec-artifacts-process#77`; this repository retains the structurally valid column until that draft lands. |
| GAP-004 | Added exact stakeholder and inspection bindings; coverage is now 28/28 with 15/15 Rust candidates tagged and bound. |
| GAP-005 | Ran the complete local gate on the current source; immutable records intentionally describe their clean parent source revision, and final merge evidence remains a release task. |
| GAP-006 | Expanded `make ci` to check all targets/features at the explicit MSRV, build a fixed bare-metal consumer, and enforce its linked-section footprint. |
| GAP-007 | Added `plan/PLAN-001-runtime-v01/` with five completed typed tasks and one explicitly human-owned open task. |

## Re-review disposition

| Review finding | Disposition |
|---|---|
| NEW-001 | Replaced the unsatisfiable rlib ceiling with a representative `no_std` static-library consumer built by Rust 1.75 for `thumbv7em-none-eabi`. The fixed, non-overridable gate measures only runtime/harness `.text` plus `.rodata`: 907 bytes against 4,096. |
| NEW-002 | Removed the Makefile-text assertion. TC-007's procedure consumes the executed `make ci` output instead of claiming target behavior from a target-name string. |
| NEW-003 | Annotated the mutable historical README: the retained `msrv_1_75=SUCCESS` is invalidated by the toolchain-precedence defect, while the Kani result applies only to its historical revision. |
| NEW-004 | Replaced the overstated traceability row with the engine's current 70/137 production-symbol measure and an explicit classification of deliberately unowned symbol classes. |
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

## Third re-review disposition

| Review finding | Disposition |
|---|---|
| NEW-014 | MP-001 now versions the footprint population as every public constructor, both accounting mutations, and every operator family. TC-007 parses actual call expressions with `syn`, ignores comments, and fails if the harness population shrinks. The harness inherits the root release profile. |
| NEW-015 | Replaced brace/string source parsing with a `syn` AST census of all top-level public accounting items, every inherent implementation block, and every public inherent method. The test constrains the complete public surface and self-probes free-function and second-impl bypasses. |
| NEW-016 | Made the footprint crate a workspace member; workspace formatting, target-specific Clippy, host test compilation, Rust 1.75 bare-metal compilation, rustdoc, and panic scanning now cover it. |
| NEW-017 | Replaced the broad `*_tests.rs` exclusion with the exact `accounting_tests.rs` path, scans that file separately for panic/unfinished operations, and asserts its `cfg(test)` module attachment. Arbitrary shipped `*_tests.rs` files are scanned normally. |
| NEW-018 | Extended the fail-closed runtime pattern to the demonstrated `split_at` and `chunks` families plus other slice operations with documented precondition panics; the compiler lints continue to cover indexing and arithmetic. |
| NEW-019 | Documented in the public interface that safe checked index lookup is runtime-only because the non-panicking slice API is not const-stable at Rust 1.75. |

The harmless trailing empty rustdoc paragraphs noted by the reviewer were also removed while keeping
ordinary-comment `Implements:` bindings intact.

Date: 2026-08-30

Source identity is recorded once in each immutable evidence bundle's `source-revision.txt` and
manifest. This mutable review document deliberately does not duplicate a source hash that would
become stale on its own next commit.

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
| Acceptance-criterion traceability | 28/28 rows backed; 15/15 Rust test symbols bound; 70/137 production symbols owned; 67 deliberately unowned generated/private/test-support/measurement symbols; zero unbound test trace IDs | pass with explicit implementation-ownership boundary |
| #3 Kani harness coverage | seven checked-in, trace-censed proofs; pinned Kani 0.67.0 local result | pass; refreshed record pending |
| Epic local gates and measurements | individual local source/tooling gates pass; exact PGM-01 schema and custom-validator gates; revision-bound MP-001 record | pending refreshed evidence |
| Protected remote gates | successful checks for pre-reconciliation revision retained under `evidence/historical/`; main-based candidate run | pending deliberate dispatch |

## Gap disposition

No unresolved implementation or specification gap was found. The source candidate's fixed-population
bare-metal linked footprint is 907 bytes inside the fixed 500-to-4,096-byte interval with zero linked
runtime/harness panic-path references, and every declared target/feature compiles under the explicit Rust 1.75 lane. It has
no default normal dependency, contains no unsafe or intentional runtime panic surface, and passes
every locally available gate.

PGM-01 PR #12 is merged at `7dac9d8c19952412b56a0347387666e2ca81e01d`. Its tree is
byte-identical to reviewed head `d8d376d887c40255e87ef9656bc0faf79216b321`; the complete merged-main
release check passes, and this candidate reconciles that exact merged revision and envelope schema
digest `0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256`.

The following are release/workflow gates, not silently accepted gaps:

1. Pinned Kani 0.67.0 executes all seven current proofs successfully; its exact transcript,
   zero exit status, and `passed` outcome must be retained in the refreshed post-merge evidence record. A hosted
   run is deferred rather than represented as complete.
2. Manual-CI PR #6 is merged. The externally owned manual-only workflow must eventually run on
   merged `main` and reconcile the local rustdoc and evidence-tool lanes without restoring automatic
   triggers. Round 10 cleared every source finding before the operator-authorized admin merge.
3. The human release owner must record the v0.1 decision in `planning/release-decision.md` after merge
   evidence is collected. No agent or automated gate may substitute for that decision.

Implementation gap-analysis result: **source remediation and local Kani pass, with refreshed evidence,
hosted checks, and the human release decision still open**.

## Fourth re-review disposition

The fourth review was queued from source `2a6aa82628c34b52a958ede822ed57285f73b75e`
and attached after the branch advanced, so its repeated NEW-014 through NEW-019 observations do not
describe the current AST-based, workspace-gated source. Its additional macro-generated accounting
variation is nevertheless closed by rejecting module- and impl-level macros in the accounting
surface census. NEW-020 is closed by a vendored PGM-01 schema whose computed digest must equal the
executable pin plus tests that require both planning copies to agree. NEW-021 is closed by fixture
tests for envelope assembly, digests, roles, extensions, pin mismatch, and accepted/rejected local
schema validation. Kani remains truthfully `skipped-unavailable` and is not converted into a pass.

## Fifth re-review disposition

The fifth review was executed from source `cc2d2188ea897a9570039f05b7f9401a770fe5fe` and attached to a
later PR head. Its source probes still applied because the intervening changes only reconciled merged
PGM-01 and refreshed evidence. They are dispositioned as follows:

| Review finding | Disposition |
|---|---|
| NEW-022 | TC-008 now recursively parses every shipped runtime source file. It resolves private aliases, counts inherent blocks across files, constrains trait implementations on both accounting types, rejects unexpected accounting-typed public functions, constrains macros, and self-probes the four demonstrated trait/alias/extra-file/cross-file bypasses. |
| NEW-023 | The linked-footprint gate now fails below a fixed 500-byte population floor as well as above 4,096 bytes. A footprint-crate test executes inputs 0 and 1 and requires exact results 6 and 14, so lexically present but unreachable population code also fails. |
| NEW-024 | The retained `test-footprint` outcome now executes the fixed-result TC-007 unit test rather than reporting a zero-test host build. |
| NEW-025 | The source audit adds `windows` and `copy_within`, while the linked static-library gate now independently rejects runtime/harness object references to `rust_begin_unwind`, bounds-check, core-panicking, or slice-index-failure symbols. This turns the representative population into a durable compiled panic-path check rather than relying only on a name denylist. |
| NEW-026 | Local `make ci` now runs both rustdoc and the evidence-tool tests. The manual-only hosted workflow is externally owned and intentionally untouched here; adding those two lanes there remains explicit FND-015 rather than risking a collision with the CI-trigger work. |
| NEW-027 | Every Python evidence-tool test now carries an explicit `Trace: TC-007, NFR-002-AC-4` comment, and the ownership test constructs its marker from separate strings so it is no longer misread as an implementation binding. |
| NEW-028 | Removed the inherently stale mutable source-revision declaration. Every immutable record already binds and checksums its exact clean source revision. |
| NEW-029 | Closed before this review arrived: both executable and planning pins name merged PGM-01 revision `7dac9d8c19952412b56a0347387666e2ca81e01d`, validated against its byte-identical merged schema and complete merged-main release check. |

The style note is also closed by replacing the target-specific `as usize` conversion with explicit
`usize::try_from` handling. Kani remains truthfully `skipped-unavailable` and is not converted into a
pass.

## Sixth and seventh re-review disposition

The sixth review was executed against the pointer-only predecessor, so its source findings were
superseded by the fifth-review fixes. The seventh review executed the current substantive source and
confirmed that all six previously demonstrated accounting bypasses and both footprint-reachability
probes now fail. Its remaining findings are dispositioned as follows:

| Review finding | Disposition |
|---|---|
| HIGH-1 / FND-101 / FND-103 / FND-104 | The TC-008 census now resolves aliases in function signatures, recognizes reference and compound trait-implementation self types, follows `#[path]` modules outside `src/`, rejects inline modules, and baselines one crate-wide public-item scope. Direct regression probes cover the aliased-function and reference-trait seams. |
| HIGH-2 / NEW-030 / FND-102 | Every collected command now retains a numeric exit status. The builder derives passed/failed/inconclusive outcomes from those records, makes failed results inconclusive at envelope level, and has a negative regression test proving a failed command cannot produce an all-passed summary. NFR-002-AC-4 and MP-001 explicitly own outcome truthfulness. |
| Panic denylist residual | The fail-closed source pattern now covers `rchunks`, `swap`, and `step_by` in addition to the linked-artifact panic relocation audit. |
| PGM-01 revision pin | The signed raw merge commit is vendored and its canonical Git object SHA-1 must equal the executable revision pin; replacing the planning/executable copies with zeros now fails closed. |
| NEW-031 | Every new envelope discloses that merged PGM-01 used a bounded admin exception without protected checks and that the exception excludes runtime release qualification. |
| FND-105 | The installed TestMatrix schema requires `Coverage Status` while its coverage selector expects `Status`, so one document cannot satisfy both. The schema-valid header is retained pending upstream `agent-ix/spec-artifacts-process#77`; the unavailable status-classification arm is disclosed, while independent backing reports 28/28 with zero unbacked rows or status lies. |
| FND-106 | The footprint regression test documents the contribution arithmetic that derives its exact results 6 and 14. |

FND-015 remains externally owned: this branch does not edit or dispatch the manual-only hosted
workflow. Kani remains truthfully `skipped-unavailable` and is not converted into a pass.

## Eighth re-review disposition

The eighth review closed both standing high-severity accounting and evidence-truthfulness findings,
the panic-audit residual, and the ungated PGM pin. Its new findings are dispositioned as follows:

| Review finding | Disposition |
|---|---|
| FND-201 | Moved the retained failed `runtime-v01-aca8fe85025b-20260831T014740Z` collection beneath `evidence/historical/` and documented why its obsolete producer's all-pass manifest contradicts the custom PGM validator's exit status 2. It is explicitly not current evidence. |
| FND-202 | Added a collector self-test that exercises the production transcript/status recorder, proves nonzero commands set the collection failure flag, checks status-word mapping, and detects a changed envelope at the checksum fixed point. The evidence-tool suite executes it on every local gate. |
| FND-203 | Replaced positional validator-transcript exclusions with an explicit immutable name set covered by a regression test. |
| FND-204 | The TC-008 public-surface census now records every `pub use` leaf with its full use-tree path and rename, with a rename self-probe. |
| FND-205 | A zero status is rejected if the retained command transcript contains a command-specific failure marker; missing stdout or stderr is inconclusive rather than passed. A negative regression test proves the contradiction fails closed. |
| FND-206 | The census exclusion now matches only the exact `src/accounting_tests.rs` relative path for both directory-discovered and path-attached sources. |

FND-015 remains externally owned and this branch neither edits nor dispatches the manual-only hosted
workflow. This statement is historical: Kani was unavailable during that round; post-merge evidence
later executed the checked-in harnesses.

## Ninth re-review disposition

The ninth review closed all six eighth-review findings and reconfirmed the two prior high-severity
gates under fresh mutations. Its new findings are dispositioned as follows:

| Review finding | Disposition |
|---|---|
| FND-301 | Closed in repository settings by the external CI/settings owner: live strict branch protection now requires `Rust 1.75 surface and footprint` and no longer names obsolete context `Rust 1.75 core`. This branch did not rename, edit, or dispatch the manual-only workflow. |
| FND-302 | The TC-008 use-tree self-probe now includes `pub use accounting::*;` and requires the exact `src/lib.rs::use accounting::*` census label, making the `UseTree::Glob` arm load-bearing. |
| FND-303 | NFR-002-AC-4 now explicitly owns collector transcript/status capture, command and fixed-point failure behavior, and the builder's transcript consistency. The collector carries an implementation marker enforced by the ownership test, and TC-007 describes the collector self-test, named exclusions, and contradiction checks. |

This statement is also round-scoped history: Kani was then `skipped-unavailable`. Task-006 remains an
explicitly human-authored release decision.

## Post-merge local evidence disposition

PR #5 was admin-merged at `e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3`, whose Git tree is identical
to the Round-10-reviewed head. The immutable record
`evidence/runtime-v01-e360dad8a3e0-20260831T160256Z` binds that exact clean merged revision and
retains 25/25 passing local outcomes: the full local gate, pinned Kani 0.67.0 with 5/5 historical harnesses and
zero failures, and both merged-PGM validators. All 91 retained checksums verify. This closes the
current-source Kani evidence gap without making a hosted-CI or human-release claim.

## Post-merge evidence review disposition

The post-merge evidence review demonstrated that the result was accurate but its recorder could not
distinguish it from a forgery. This branch now derives Kani from numeric status, exact successful
harness names/count, version, and transcript markers; treats every skipped outcome as pending with a
named limitation; censes unknown status files; verifies anchored records independently; binds the
external merged-PGM schema; adds strict coverage and local Kani targets; and runs all Python test
filenames. Proof scope is narrowed and stated: i8 addition uses independent widening arithmetic,
invalid division is symbolic, index proof covers full `usize`, and accounting saturation has its own
harness. The authoritative `runtime-v01-f3f1c28d1703-20260831T174552Z` record binds the clean
remediation source, retains 26/26 passing outcomes including 6/6 Kani harnesses, and verifies 101
checksums plus 81 manifest artifacts. Hosted workflow changes remain outside this branch by operator
direction.

## Post-merge evidence review Round 2 disposition

Round 2 reconfirmed every first-round remediation and identified two new fail-open local gates plus
twelve hardening findings. They are dispositioned as follows:

| Review finding | Disposition |
|---|---|
| FND-101 | `make kani` now fails nonzero when `cargo-kani` is absent. Its prerequisite first checks the exact declared harness census, so a green `make ci` means Kani was available and executed. |
| FND-102 | `scripts/check_coverage_status.py` owns the installed module's `Status` versus schema-valid `Coverage Status` compatibility seam. It consumes strict JSON, requires every functional row and report row to be fully backed/complete, and rejects ignored trace-bearing tests. The retained transcript reports this local classification instead of recording the upstream skipped classifier as passed. |
| FND-103 / FND-112 | `scripts/check_kani_harnesses.py` requires the exact seven-function census and a TC-001/TC-002/TC-003 marker on every proof before Kani runs. Deletion, rename, untraced proof, and unexpected proof all fail the local gate. |
| FND-104 | `scripts/update_evidence_anchors.py` deterministically regenerates the complete anchor census; the human operation is review of its diff, not digest transcription. A regression test requires the committed file to equal generated output. |
| FND-105 | Verifier tests now execute checksum and symlink rejection, anchor generation, outcome census, revision binding, and distinct unavailable/failed channels instead of checking only for an ownership marker. |
| FND-106 | The verifier requires the recorded revision to be an existing commit whose complete non-`evidence/` tree equals current `HEAD`, and requires the non-evidence worktree to equal `HEAD`. Evidence-only seal commits remain possible without allowing stale source/spec claims. |
| FND-107 | Missing schema packages are mapped to `VerificationUnavailable`. The verifier writes `target/evidence-verification-status.json`, preserving passed/failed/unavailable and the original exit code even though GNU Make collapses recipe failures to exit 2. |
| FND-108 | The weak Makefile substring assertions were removed. Tests interrogate Make's actual `ci` dependency graph and execute `make kani` under a PATH with Python but no `cargo-kani`, proving the gate is wired and fails closed. |
| FND-109 | The accounting proof now drives the public `CampaignReport::record_verdict` and `record_discard` paths from symbolic near-overflow counts and independently asserts all five saturating increments plus the saturating total. |
| FND-110 | Every checksummed record member is rejected if it is a symlink, matching the recursive tree-anchor rule. |
| FND-111 | Current proof-count statements now say six; older 5/5 statements are explicitly historical records. |
| FND-113 | The validator recursively inventories every `format` used by the supplied schema and refuses validation when any checker is absent; an unknown-format regression test holds the rule. |
| FND-114 | The verifier independently reconciles retained numeric/availability status names, the declared command census, and manifest outcome names before re-deriving values. Exact Kani 0.67.0 identity is required. The remaining absence of an externally signed runner attestation is stated as a limitation rather than inferred from self-authored transcripts. |

The branch continues to avoid hosted workflow edits and dispatches. Human release authority remains
outside this evidence remediation.

## Post-merge evidence review Round 3 disposition

Round 3 confirmed the prior high-severity fixes and identified four positive-evidence failures,
eleven control-integrity gaps, and seven lower-severity scope/documentation gaps. The source
remediation is:

| Review finding | Disposition |
|---|---|
| FND-201 | Kani acceptance parses every harness block, requires a positive check count at or above its checked-in floor, and records the per-harness values in the schema-validated manifest. The exact census now includes a seventh public-model provenance proof. |
| FND-202 | The verifier rejects every authoritative record whose internally consistent result is not `conclusive`; an explicit failure-direction test covers an `inconclusive` result. |
| FND-203 | Every zero-exit gate now needs command-specific positive corroboration (or the defined empty-success contract for rustfmt). Empty test/output transcripts become `inconclusive`, and NFR-002-AC-4 requires positive work evidence. |
| FND-204 | Process-level mutation tests delete a Kani harness, delete a matrix row, and remove an anchored evidence entry; each real script must return nonzero. Make-control and PATH-shadow mutations likewise execute the real guard. |
| FND-205 / FND-206 | Repository-owned Python caches are ignored explicitly. Untracked enumeration disables the global excludes file and applies only the committed root `.gitignore`, so self-hiding nested ignore files and `.git/info/exclude` cannot conceal build inputs. |
| FND-207 | Anchor generation is no longer part of collection, refuses silent top-level removals, and enforces two named history directories plus a 29-record floor in both updater and verifier. |
| FND-208 | Exactly one authoritative record is allowed; its directory, envelope `recordId`, README declaration, and strict timestamped name must agree. An incomplete `runtime-v01-*` directory is an error, never a generic tree anchor. |
| FND-209 / FND-210 | The verifier independently re-derives parameters, collector executable, and dependency-lock digests from the recorded Git revision and rejects an `ANCHORS` symlink. |
| FND-211 | The ignored-test detector walks every repository Rust source, binds ignore/cfg-attr attributes to the following function, and recognizes trace-bearing function names without a fixed line window. |
| FND-212 / FND-213 | A parse-time Make guard rejects ambient flags and unsafe directives. The executable guard enforces exact ordered prerequisites, forbids failure suppression/control operators, substitutes `false` at every recipe position, and verifies exact Cargo/Python/Quire/Make paths and version shapes. |
| FND-214 / FND-215 | Repairing the upstream status classifier now emits a notice rather than failing. The matrix gate fixes the eight-row census, requires a nonempty test citation per row, and resolves every cited TC/SUITE identity against the registry. |
| FND-216 / FND-217 | The AA-001 gate consumes the verifier's machine-readable passed/failed/unavailable status. Coverage and Kani-census missing-input/tool paths return the unavailable exit channel. |
| FND-218 / FND-219 | Both proof-only accounting constructors carry ownership markers, and `CLAUDE.md` lists the complete local gate surface. |
| FND-220 / FND-221 | The stable-Clippy `cfg(kani)` boundary is documented and controlled by formatting, census, obligation floors, and Kani. Checked i8 sub/mul/div/rem now use independent widened oracles, while the seventh proof covers identity, observation, and verdict modules. |
| FND-222 | AA-001 declares its authoritative-record count, outcome count, required verdict, and anchor; `check_assurance_anchor.py` executes that binding after evidence verification. |

Kani 0.67.0 locally verifies all seven proofs with positive per-harness obligation counts. The
authoritative evidence refresh remains pending until that source-bound transcript is collected. No
hosted CI was dispatched and no workflow file was changed.
