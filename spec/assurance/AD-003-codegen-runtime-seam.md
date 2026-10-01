---
id: AD-003
title: "Codegen to runtime seam: what generated crates and the generator use from this crate"
type: ArchitectureDescription
status: proposed
owner: runtime-maintainers
system: quire-contract-runtime v0.1 public items that quire-contract-codegen links at generation time and emits into generated crates, the manifest text codegen writes for them, and the outcome vocabularies that cross back
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: references
---
# Codegen to runtime seam

AD-001 describes the crate's core and AD-002 the function-application boundary. Neither says who
consumes the crate or how. This AD states the seam to its one consumer, `quire-contract-codegen`,
and is one of the seam descriptions of IR-323. It lives here because this crate defines the
runtime contract, so it is the one repository whose specification can describe the surface without
naming a downstream. Codegen's own side (the generated text, its manifest and its mirrors) is
stated as routed gaps. File paths of the form `codegen src/...` are in the codegen repository.

## System Boundary

The runtime is a leaf library. Codegen uses it in two ways. At generation time codegen links the
`exact` feature and calls its checking and construction items. In generated output codegen emits
source that names runtime items, and for three emitters a `Cargo.toml` that depends on this crate.
Generated code runs later, outside both repositories (a customer build, a codegen test, a Kani
run).

Out of scope: IR's seam to codegen (`quire-contract-ir` AD-006), the evidence chain and replay
(codegen's AD-002 and AD-003), and the runtime's relation to QSL beyond what the codegen seam needs
(decision F and Current state).

## Views

The seam is described as what crosses it, how identity and versions are asserted, which way
dependencies point and who reports each failure.

### What crosses the seam

Measured at the `origin/main` of the runtime and codegen repositories on 2026-10-01 (the commits,
runtime 215e443 and codegen 2fad745, are informational and asserted nowhere).

| Group | Runtime items | Owner | Where codegen uses them (`codegen` paths) |
| --- | --- | --- | --- |
| Verdict and identity (default features) | `ContractIdentity`, `RequirementId`, `RevisionId`, `ExecutionPoint`, `ClauseId`, `Observation`, `ClauseKind`, `ClauseOutcome`, `FailureDetail`, `FailureKind`, `Verdict`, `VerdictContext`, `VerdictKind` | runtime (FR-001) | emitted by `codegen src/harness.rs:417-972` and `codegen src/bound_strategy/generation.rs:661-747` |
| Campaign accounting | `CampaignReport`, `CampaignCounts`, `CampaignSnapshot`, `IdentityMismatch` | runtime (FR-004) | the same emitters; harness runners take `&mut CampaignReport` and derive a summary from a snapshot |
| Exact operators and values | `Integer`, `IntegerInterval`, `IeeeWidth`, `RoundingMode`, `TextProfile`, `QuantityTarget`, `Meter`, `ScalarLimits`, `Outcome`, the operator and comparison enums | runtime (FR-006 to FR-008) | linked by the generator for parameter checks (`codegen src/exact_scalar.rs:58`); emitted as `rt::` calls (`:2344`) |
| Composite and equality checking | `TypeEnvironment::check_equality`, `CheckedEquality`, `CompositeDeclaration`, `ValueType`, `NodeKey`, `IllTypedCause`, `DeclarationCause` | runtime (FR-008, FR-012) | linked at `codegen src/composite_equality.rs:802`; emitted at `:1396` |
| Function application | `PackageDeclarations::check`, `CheckedPackage::call`, `Value`, `ObjectEnvironment`, `InputRefusal`, `Origin`, `Location` | runtime (FR-273, AD-002) | linked at `codegen src/exact_function.rs:952`; emitted at `:1277` and `:1353` |
| Backend negotiation | `negotiate_ieee`, `negotiate_integer_division` and their disposition types | runtime (FR-009) | no codegen source calls either (grep of `codegen src/`) |
| Snapshot wire | `runtime.campaign-snapshot/v1`, feature `snapshot-json` | runtime (FR-004) | not consumed: codegen's interface document says so (`codegen spec/core/functional/interface-001-codegen-api.md:304`) |
| Generated manifest text | `quire-contract-runtime = { git, rev, features }` and the `[package.metadata.kani]` flags | codegen writes it | three emitters: `codegen src/exact_scalar.rs:2817`, `codegen src/composite_equality.rs:1573`, `codegen src/exact_function.rs:1378`; the revision constant is `codegen src/oracle.rs:13`, and the other users of it are codegen tests |
| Spelling of runtime paths in generated text | `quire_contract_runtime::ContractIdentity`, `::ClauseId` | codegen's schema asserts the text | `codegen schemas/generated-rust-oracle-v1.schema.json:9-10` |
| Serializable copies of runtime enums | `IllTypedCause`, `RecursionEdges`, `DeclarationCause`, `Origin`, `Location` | codegen | `codegen src/composite_equality.rs:201-330`, `codegen src/exact_function.rs:374-400`; each has a `From` impl over a runtime enum |
| A trusted flag | `FunctionDeclaration::measure_discharged` | runtime reads it, codegen sets it | codegen sets `true` unconditionally (`codegen src/exact_function.rs:935`, and in emitted source at `:1303`) |

Features: only the three exact emitters write a manifest into generator output, and it requests
`exact`. The harness crates (`proptest`) and the Kani crates (`exact`) are built by codegen's tests
with manifests the tests write (for example `codegen tests/it/harness_generation.rs:223` and
`codegen tests/it/kani_generation.rs:304`), not by the generator. RT's own `#[cfg(kani)]`
verification module imports `crate::exact` unconditionally (`verification/kani.rs:1`), so a crate
checked with `cargo kani` must enable `exact` even when it uses nothing from it (codegen's test
records the same, and it is runtime's coupling).

### Identity and versions on this seam

- This crate is `0.1.0` with `publish = false`. Its features (`default`, `alloc`, `std`,
  `proptest`, `snapshot-json`, `exact`) are the opt-in surface. There is no semantic version to
  negotiate, and none is proposed.
- The assertion on the Rust surface is that generated source compiles against this crate, and
  codegen's tests build every generator's output against it (for example
  `codegen tests/it/exact_scalar_generation.rs:1424`).
- Codegen's own dependency is a git `branch = "main"` with `version = "=0.1.0"`
  (`codegen Cargo.toml:18`), resolved in codegen's lock at one commit (informational: ccc722b). The
  emitted manifests name a different one: `RUNTIME_REVISION` is a commit before RT #83 to #88
  (informational: ed0a04b; SR-623 records that the runtime changed a lot after it). Between the
  two the runtime's `src/` differs by 8 files and 28 changed lines. Two runtime revisions are
  therefore live for one codegen build, and the compile tests build against the old one while the
  generator links the newer. CG's layout AD (IR-344, CG PR #215, open) deletes `RUNTIME_REVISION`
  at its step 1c and has the emitted manifest name the runtime the way CG's own `Cargo.toml` does;
  that ends the two-revision condition, and until it lands the condition stands (T-2).
- Wire versions this crate owns: `runtime.campaign-snapshot/v1` (decoder refuses another
  version, `UnsupportedVersion`), the node-key domain `quire.checked-semantic-node/v1` and the
  accounting version `quire.value.accounting/v1`, carried verbatim; IEEE definition
  `quire.value.ieee754-2019-default/v1`. None is a digest. The runtime computes no node key and no
  digest (FR-012); a `NodeKey` is an opaque value the compiler supplies.
- The enumerations are `#[non_exhaustive]` (72 sites) and the crate's documentation tells
  consumers to retain unknown future states rather than convert them to success (`src/lib.rs`,
  Compatibility). Codegen's side of that rule is in Current state.

### Dependency direction and what enforces it

| Edge | Allowed | Held by | Gap |
| --- | --- | --- | --- |
| codegen to runtime | yes (normal, `exact`; dev adds `proptest`) | `codegen Cargo.toml:18`, `:31` | none |
| runtime to codegen or IR | no | `deny.toml` has `unknown-git = "deny"` with no `allow-git`, and every first-party dependency is a git source, so `make deny` fails on any such edge. `scripts/check_one_copy.awk` checks duplicate revisions only and blocks no edge | the rule is implicit in the sources policy; no test or comment says it is the guard; a path dependency would not be caught; `make ci` stops at `spec` (failing on `main`) before it reaches `deny`, so run `make deny` directly |
| runtime to any other QSL crate (`qsl-eval`, `qsl-replay`, the QSL root) | no | the same sources policy once `allow-git` admits the one `quire-exact` source: `allow-git` names a repository, so the exception must be narrowed to that crate or the lint of QSL-356 must carry it | `deny.toml` cannot tell crates of one repository apart; the guard is QSL's lint (QSL-356, as relayed) or a `bans` entry per crate |
| runtime to `quire-exact` | yes (accepted, decision F) | QSL ADR-011 lists "RT to `quire-exact`" as a new edge. QSL's `arch-lint` classifies a dependency whose source names the QSL repository as QSL and exempts every normal CG edge into QSL, which is not the RT edge | QSL fixes FB-05 and its lint (QSL-356, as relayed); RT's `deny.toml` then needs `allow-git` for that one source |
| any cycle among the four repositories | no | QSL `arch-lint direction` (FB-05, FB-11), run on request with the clones and outside QSL's `make ci` | manual; QSL gives no CI option (as relayed) |
| generated crate to runtime | yes, by manifest | codegen's compile tests | covers the pinned revision only |

### Failure outcomes and who reports them

| Condition | Reported by | Outcome |
| --- | --- | --- |
| A clause passes, fails or its precondition excludes the case | the oracle, through `Verdict` | `Passed`, `FailedPostcondition`, `RejectedPrecondition`; no Boolean conversion, rejection is not failure |
| A verdict names another campaign | `CampaignReport::record` | `IdentityMismatch` carrying both identities, no counter moves |
| Counters reach `u64::MAX` | the report | saturate, `at_limit` is true; never a panic |
| An exact operation has no value, is refused, or a charge is unavailable | the operator | `Outcome::Undefined`, `Refused` or `Incomplete`; the oracle returns it unchanged to its caller |
| A function call has a bad argument | `CheckedPackage::call` | `InputRefusal` (`Arity`, `WrongValueKind`, `DanglingReference`, `UnknownFunction`), before any charge |
| A package is not admissible | `PackageDeclarations::check` | `CheckRefusal` with a typed cause |
| A backend lacks a capability | the negotiators | one disposition per item (`Supported`, `RequiresBound`, `Unsupported`); not an `Outcome` |
| A runtime enum gains a variant | the generator for an arm in generator code, the generated crate for the two arms emitted into it | a typed `UnknownRuntimeVariant` in one path, and a panic at every other `unreachable!` arm (Current state) |
| A limit is reached inside a Kani proof | not reported | proof harnesses meter at `u64::MAX` for every limit (`codegen src/kani_obligations.rs:2060-2071`), so exhaustion paths are not exercised by them |

## Decisions

- A. The runtime is the one authority for the spelling and meaning of its items. Generated text
  names them by path and codegen defines no second spelling.
- B. Codegen reports a runtime variant it does not know as a typed refusal of the whole
  generation, never as a panic at generation time and never as a panic inside generated source.
- C. Emitted manifests name this crate the way codegen's own manifest does, so one runtime
  revision is live per codegen build. CG's layout AD (IR-344 step 1c, PR #215) settles the form:
  `RUNTIME_REVISION` is deleted and the manifest names the runtime by branch as `Cargo.toml` does.
- D. The runtime computes no digest and keeps no version record about codegen, and codegen keeps
  none about the runtime beyond its lock.
- E. Provider dispositions stay outside the evaluation path (AD-002, FR-009).
- F. `exact` is a copy of QSL's kernel, which is vendoring and not allowed to stand. Its own
  header says it is "a `no_std + alloc` port" that "keeps the authority's name and order"
  (`src/exact/mod.rs:5-11`), and QSL's kernel now lives in `quire-exact`. The decision is to
  delete `exact` and depend on `quire-exact`, once QSL-357 makes `quire-exact` `no_std` plus
  `alloc` (its manifest today declares `rust-version = "1.98"` and default-feature dependencies,
  against this crate's 1.75 and `no_std`). No shared-corpus agreement test is kept between two
  copies, because there will be one. Pointing this crate's conformance at `qsl-eval` (the idea
  recorded under IR-355) is rejected: `qsl-eval` is QSL layer 5 and an FB-05 violation even as a
  dev edge. Until the deletion lands, the exact rows of the seam above describe this crate's own
  `exact`; after it they describe `quire-exact`, and the items SR-623 FND-002 lists as absent from
  `quire-exact` (cross-unit quantity, the equality-conversion table) are for the code move to
  resolve, not re-measured here. No compatibility layer carries the old copy.

No compatibility layer is proposed. A runtime item that changes is changed in codegen in the
same step.

### Invariants a test can check

Local labels; the repository assigns requirement ids when one is authored.

- T-1. This crate declares no dependency on codegen or IR, and its only first-party edge is
  `quire-exact` once decision F lands (enforced by the sources policy today; add a direct
  assertion).
- T-2. Every generated crate compiles against the runtime codegen itself links (codegen's
  compile tests, with one runtime revision; not true today: two revisions).
- T-3. No `unreachable!` arm names a runtime enum in generator code or in emitted source (a count
  a test can take; not true today).
- T-4. `Verdict` has exactly three kinds and none converts to `bool` (existing: FR-001).
- T-5. A rejected precondition increments rejected and not failed; a mismatched identity moves no
  counter (existing: FR-004).
- T-6. A negotiator returns one disposition per item in input order, and permuting the input
  permutes the result (existing: FR-009-AC-1); an `InputRefusal` leaves the meter unchanged
  (existing: FR-273).
- T-7. For the same input the oracle's `Outcome` equals QSL's replay outcome. As relayed, this
  belongs in codegen's parity comparator (ADR-011 names `quire-contract-codegen#50`), not here;
  it is trivial once both sides call one kernel (decision F).

Stated but not testable today: `measure_discharged` is set only for a function read from an
admitted package that carries the authority's discharge (as relayed, admission carries it; no
carrier is measured on the wire yet).

## Risks

What is measured today, what is open and with whom, and what is routed.

### Current state and gaps

- The audit SR-623 (`reviews/ir-319-code-review.md`) covers the runtime's own design. This AD
  adds only what it does not: the consumer's side. Its findings that touch this seam are not
  restated: FND-001 and FND-002 (the kernel is a copy of QSL's, the agreement evidence was deleted
  and the shared kernel is not adopted; tracked there as IR-342, IR-345 and IR-355). IR-355 is
  superseded by decision F, which keeps no agreement test and rejects its `qsl-eval` idea. The
  interim gap is that RT has no agreement evidence at all until QSL-357 lands and the copy is
  deleted. Also FND-003 (`CheckedInvariant` merges at least ten conditions; IR-356) and FND-007
  (one Kani harness reaches `exact`; IR-340).
  least ten conditions; IR-356) and FND-007 (one Kani harness reaches `exact`; IR-340).
- The copy of QSL's kernel (decision F) puts two kernels in codegen's build graph
  (`rt::ScalarLimits` and `qsl_replay::ScalarLimits`, `codegen src/spine_replay.rs:16`), so the
  oracle codegen emits and the replay QSL runs can disagree (T-7).
- Pin: `RUNTIME_REVISION` (`codegen src/oracle.rs:13`) is a stale SHA written into the three
  emitters' manifests, and it makes the tests build against a runtime codegen does not link. It is
  deleted by CG's layout AD at step 1c (IR-344, PR #215, open), so nothing is routed for it here.
- Panics at the seam: codegen has 37 `unreachable!` arms over runtime `#[non_exhaustive]` enums
  (`codegen src/exact_scalar.rs` 26, `codegen src/composite_equality.rs` 9,
  `codegen src/exact_function.rs` 2). Most are lowering matches that choose the `rt::` path to
  emit; a few in `composite_equality.rs` are the `From` conversions to serializable mirrors. The
  two in `exact_function.rs` (`:1331`, `:1363`) are text inside the emitted source and run in the
  generated crate on a future `Outcome` variant, so the generator is not what reports them. The 37
  include those two. One path already returns the typed `UnknownRuntimeVariant`
  (`codegen src/generation.rs`; `check_parameters` in `exact_scalar.rs`). Tracked as IR-352.
- The `measure_discharged` flag is trusted: FR-273 states that nothing links a runtime package to
  the authority's proof and that a body is arbitrary host Rust. Codegen sets the flag to `true`
  for every function it assembles.
- No codegen source calls the FR-009 negotiators, so the `unsupported` and `requires-bound`
  dispositions are decided elsewhere or not at all on this seam. Not measured: whether QSL or
  another consumer calls them.
- `Evaluation.location` and `.losses` are public but no producer fills them (SR-623 FND-011), and
  codegen mirrors `Location` (`RecordedLocation`) with a path that is always empty.

### Open questions

| Question | Owner | Recommendation | Cost of the alternative |
| --- | --- | --- | --- |
| Confirm CG's step 1c lands (the emitted manifest) | codegen | settled in CG's layout AD (IR-344 step 1c, PR #215, open) | a stale SHA keeps two runtimes live |
| Replace codegen's `unreachable!` arms with `UnknownRuntimeVariant`, and the two emitted ones with a typed stop | codegen (IR-352) | yes | a runtime variant crashes generation or generated code |
| Where does the discharge of a function's `decreases` measure travel to codegen? | QSL and codegen | admission carries it (QSL's answer, as relayed); codegen sets the flag only for functions read from an admitted package | the flag stays a constant codegen asserts |
| Should RT's Kani module stop importing `exact` unconditionally? | runtime | gate the import so a core-only crate verifies without `exact`; moot for the kernel once decision F lands | every Kani crate pulls in the large kernel |

### Routed gaps

Needs stated to owners, not decisions. Ids are routing ids of IR-323; they are not requirement
ids. Runtime-owned items are the decisions and invariants above and SR-623's findings and need no
routing.

To codegen:

| Id | Stated need |
| --- | --- |
| R3-C6 | Replace the 37 `unreachable!` arms over runtime enums (35 in generator code, 2 emitted into generated source) with a typed refusal; `UnknownRuntimeVariant` already exists. Tracked as IR-352. |
| R3-C7 | Set `measure_discharged` only for functions read from an admitted package that carries the discharge, not as the constant `true`. |
| R3-C8 | State whether the serializable mirrors of runtime enums stay, or the kernel exposes a serialization the generator reuses (a stated need, not a request to copy). |

To QSL, answered as relayed and no longer open: the runtime edge to `quire-exact` is accepted and
QSL fixes FB-05 and its lint (QSL-356); T-7 belongs in codegen's parity comparator; admission
carries the discharge (R3-Q7); `quire-exact` becomes `no_std` plus `alloc` under QSL-357 (decision
F).

To QSpec: none added by this AD.
