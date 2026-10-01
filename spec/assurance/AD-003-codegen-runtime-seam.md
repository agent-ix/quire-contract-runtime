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
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-006
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
runtime contract and depends on nothing first-party, so it is the one repository whose
specification can describe the surface without naming a downstream. Codegen's own side (the
generated text, its manifest and its mirrors) is stated as routed gaps.

## System Boundary

The runtime is a leaf library. Codegen uses it in two ways. At generation time codegen links the
`exact` feature and calls its checking and construction items. In generated output codegen emits
source that names runtime items, plus a `Cargo.toml` that depends on this crate. Generated code
runs later, outside both repositories (a customer build, a codegen test, a Kani run).

Out of scope: IR's seam to codegen (`quire-contract-ir` AD-006), the evidence chain and replay
(codegen's AD-002 and AD-003), and the runtime's relation to QSL's kernel beyond what the
codegen seam needs (Current state).

## Views

The seam is described as what crosses it, how identity and versions are asserted, which way
dependencies point and who reports each failure.

### What crosses the seam

Measured at runtime `origin/main` 215e443 and codegen `origin/main` 2fad745.

| Group | Runtime items | Owner | Where codegen uses them |
| --- | --- | --- | --- |
| Verdict and identity (default features) | `ContractIdentity`, `RequirementId`, `RevisionId`, `ExecutionPoint`, `ClauseId`, `Observation`, `ClauseKind`, `ClauseOutcome`, `FailureDetail`, `FailureKind`, `Verdict`, `VerdictContext`, `VerdictKind` | runtime (FR-001) | emitted by `src/harness.rs:417-972` and `src/bound_strategy/generation.rs:661-747` |
| Campaign accounting | `CampaignReport`, `CampaignCounts`, `CampaignSnapshot`, `IdentityMismatch` | runtime (FR-004) | the same emitters; harness runners take `&mut CampaignReport` and derive a summary from a snapshot |
| Exact operators and values | `Integer`, `IntegerInterval`, `IeeeWidth`, `RoundingMode`, `TextProfile`, `QuantityTarget`, `Meter`, `ScalarLimits`, `Outcome`, the operator and comparison enums | runtime (FR-006 to FR-008) | linked by the generator for parameter checks (`src/exact_scalar.rs:58`); emitted as `rt::` calls (`:2344`) |
| Composite and equality checking | `TypeEnvironment::check_equality`, `CheckedEquality`, `CompositeDeclaration`, `ValueType`, `NodeKey`, `IllTypedCause`, `DeclarationCause` | runtime (FR-008, FR-012) | linked at `src/composite_equality.rs:802`; emitted at `:1396` |
| Function application | `PackageDeclarations::check`, `CheckedPackage::call`, `Value`, `ObjectEnvironment`, `InputRefusal`, `Origin`, `Location` | runtime (FR-273, AD-002) | linked at `src/exact_function.rs:952`; emitted at `:1277` and `:1353` |
| Backend negotiation | `negotiate_ieee`, `negotiate_integer_division` and their disposition types | runtime (FR-009) | no codegen source calls either (grep of `src/`) |
| Snapshot wire | `runtime.campaign-snapshot/v1`, feature `snapshot-json` | runtime (FR-004) | not consumed: codegen's interface document says so (`spec/core/functional/interface-001-codegen-api.md:304`) |
| Generated manifest text | `quire-contract-runtime = { git, rev, features }` and the `[package.metadata.kani]` flags | codegen writes it, runtime's features and `Cargo.toml` decide what it must say | six generator sites, for example `src/exact_scalar.rs:2817`; the revision constant is `src/oracle.rs:13` |
| Spelling of runtime paths in generated text | `quire_contract_runtime::ContractIdentity`, `::ClauseId` | codegen's schema asserts the text | `schemas/generated-rust-oracle-v1.schema.json:9-10` |
| Serializable copies of runtime enums | `IllTypedCause`, `RecursionEdges`, `DeclarationCause`, `Origin`, `Location` | codegen | `src/composite_equality.rs:201-330`, `src/exact_function.rs:374-400`; each has a `From` impl over a runtime enum |
| A trusted flag | `FunctionDeclaration::measure_discharged` | runtime reads it, codegen sets it | codegen sets `true` unconditionally (`src/exact_function.rs:935`, and in emitted source at `:1303`) |

Features codegen requests: `exact` for generated exact, function and Kani crates, `proptest` for
generated harness crates. RT's own `#[cfg(kani)]` verification module imports `crate::exact`
unconditionally (`verification/kani.rs:1`), so a generated crate checked with `cargo kani` must
enable `exact` even when it uses nothing from it (codegen's test records the same, and it is
runtime's coupling).

### Identity and versions on this seam

- This crate is `0.1.0` with `publish = false`. Its features (`default`, `alloc`, `std`,
  `proptest`, `snapshot-json`, `exact`) are the opt-in surface. There is no semantic version to
  negotiate, and none is proposed.
- The assertion on the Rust surface is that generated source compiles against this crate, and
  codegen's tests build every generator's output against it (for example
  `tests/it/exact_scalar_generation.rs:1424`).
- Codegen's own dependency is a git `branch = "main"` with `version = "=0.1.0"`
  (`Cargo.toml:18`), resolved in codegen's lock at commit ccc722b. The generated crates name a
  different revision: `RUNTIME_REVISION` is `ed0a04b` (`src/oracle.rs:13`), a commit before
  RT #83 to #88 (SR-623 records that the runtime changed a lot after it). Between the two the runtime's `src/` differs by 8
  files and 28 changed lines. Two runtime revisions are therefore live for one codegen build, and
  the compile tests build against the old one while the generator links the newer. This is a pin
  with no property it protects (D-4 and the open question below).
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
| codegen to runtime | yes (normal, `exact`; dev adds `proptest`) | codegen `Cargo.toml:18`, `:31` | none |
| runtime to codegen, IR or QSL | no | `deny.toml` has `unknown-git = "deny"` with no `allow-git`, and every first-party dependency is a git source, so `make deny` (part of `make ci`) fails on any such edge; `scripts/check_one_copy.awk` over every tracked lock | the rule is implicit in the sources policy; no test or comment says it is the guard, and a path dependency would not be caught |
| any cycle among the four repositories | no | QSL `arch-lint direction` (FB-05, FB-11), run on request with the clones and outside QSL's `make ci` | manual |
| generated crate to runtime | yes, by manifest | codegen's compile tests | covers the pinned revision only |
| runtime to `quire-exact` (planned) | to decide | QSL ADR-011 lists "RT to `quire-exact`" as a new edge; `quire-exact` is a crate in the QSL repository; QSL's `arch-lint` classifies a dependency whose source names that repository as QSL and reports a backend edge into QSL as an FB-05 violation unless it is CG's `qsl-replay` edge | the planned edge and the rule disagree (open question) |

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
| A runtime enum gains a variant | the generator | a typed `UnknownRuntimeVariant` in one path and a panic in 37 others (Current state) |
| A limit is reached inside a Kani proof | not reported | proof harnesses meter at `u64::MAX` for every limit (`src/kani_obligations.rs:2060-2071`), so exhaustion paths are not exercised by them |

## Decisions

- A. The runtime is the one authority for the spelling and meaning of its items. Generated text
  names them by path and codegen defines no second spelling.
- B. Codegen reports a runtime variant it does not know as a typed refusal of the whole
  generation, never as a panic at generation time and never as a panic inside generated source.
- C. Generated manifests name this crate by the same source spelling codegen's own manifest
  uses, so one runtime revision is live per codegen build (open question for the form).
- D. The runtime computes no digest and keeps no version record about codegen, and codegen keeps
  none about the runtime beyond its lock.
- E. Provider dispositions stay outside the evaluation path (AD-002, FR-009).

No compatibility layer is proposed. A runtime item that changes is changed in codegen in the
same step.

### Invariants a test can check

Local labels; the repository assigns requirement ids when one is authored.

- T-1. This crate declares no dependency on codegen, IR or QSL (enforced by the sources policy
  today; add a direct assertion).
- T-2. Every generated crate compiles against the runtime codegen itself links (codegen's
  compile tests, with one runtime revision; not true today: two revisions).
- T-3. Every runtime enum codegen matches is handled by a typed refusal when a new variant
  appears (a test that adds none can still count the arms: no `unreachable!` names a runtime
  enum; not true today).
- T-4. `Verdict` has exactly three kinds and none converts to `bool` (existing: FR-001).
- T-5. A rejected precondition increments rejected and not failed; a mismatched identity moves no
  counter (existing: FR-004).
- T-6. Each exact operator returns exactly one of the four `Outcome` variants and a negotiator
  returns one disposition per item in input order (existing: FR-006, FR-009-AC-1).
- T-7. A generated oracle's `Outcome` for an input equals QSL's replay outcome for the same
  input (cross-repository agreement; no such test exists, see Current state).
- T-8. `measure_discharged` is set only for a function whose admitted source carries the
  authority's discharge (not testable today: no carrier).

## Risks

What is measured today, what is open and with whom, and what is routed.

### Current state and gaps

- The audit SR-623 (`reviews/ir-319-code-review.md`) covers the runtime's own design. This AD
  adds only what it does not: the consumer's side. Its findings that touch this seam are not
  restated: FND-001 and FND-002 (the exact kernel is a port of QSL's, the agreement evidence was
  deleted and the shared kernel is not adopted; tracked there as IR-342, IR-345, IR-355),
  FND-003 (`CheckedInvariant` merges at least ten conditions; IR-356) and FND-007 (one Kani
  harness reaches `exact`; IR-340).
- The runtime documents `exact` as "a port" of QSL's value module, keeping every name and order
  (`src/exact/mod.rs:5-11`). A port is a copy, and a copy of a semantic authority is what
  FND-002 measures drifting. The consequence on this seam is T-7: the oracle codegen emits
  (runtime kernel) and the replay QSL runs (`quire-exact`) can disagree, and codegen's build
  graph holds both (`rt::ScalarLimits` and `qsl_replay::ScalarLimits`, `src/spine_replay.rs:16`).
- Pin: `RUNTIME_REVISION` (`src/oracle.rs:13`) is a stale SHA written into six generators'
  manifests. It protects nothing a compile test against the linked runtime would not, and it
  makes the tests build against a runtime codegen does not link.
- Panics at the seam: codegen's conversions from runtime enums to serializable mirrors end in
  `unreachable!` at 37 sites (`src/exact_scalar.rs` 26, `src/composite_equality.rs` 9,
  `src/exact_function.rs` 2), and two of those are emitted into generated oracle source
  (`src/exact_function.rs:1331`, `:1363`), so a generated crate can panic on a future
  `Outcome` variant. One path already returns the typed `UnknownRuntimeVariant`
  (`src/generation.rs`, `check_parameters` in `exact_scalar.rs`).
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
| What does a generated manifest say about this crate? Options: a branch spelling identical to codegen's own; or no manifest line (the consumer declares the dependency) | codegen | the branch spelling, with the compile tests building against the lock-resolved runtime; the output text stays deterministic and no SHA is minted | a stale SHA keeps two runtimes live |
| Replace codegen's `unreachable!` conversions with `UnknownRuntimeVariant` | codegen | yes, and drop the emitted `unreachable!` for a typed stop | a runtime variant crashes generation or generated code |
| Where does the discharge of a function's `decreases` measure travel to codegen? | QSpec, QSL | carry it on the admitted package so codegen sets the flag from it | the flag stays a constant codegen asserts |
| Does the runtime adopt `quire-exact`, and if so how does that edge satisfy FB-05? | runtime and QSL (IR-342, IR-345) | decide the edge before the code move; if `quire-exact` stays in the QSL repository, FB-05 needs a named exception | the move lands and QSL's own lint reports it |
| Should RT's Kani module stop importing `exact` unconditionally? | runtime | gate the import so a core-only generated crate verifies without `exact` | every Kani crate pulls in the 12k-line kernel |

### Routed gaps

Needs stated to owners, not decisions. Ids are routing ids of IR-323; they are not requirement
ids. Runtime-owned items are the invariants above and SR-623's findings and need no routing.

To codegen:

| Id | Stated need |
| --- | --- |
| R3-C5 | Stop minting a runtime SHA: replace `RUNTIME_REVISION` (`src/oracle.rs:13`) and its use in the six manifest sites and the tests with one spelling that matches codegen's own dependency. |
| R3-C6 | Replace the 37 `unreachable!` conversions over runtime enums, and the two emitted into generated source, with a typed refusal (`UnknownRuntimeVariant` already exists). |
| R3-C7 | Set `measure_discharged` from a carried discharge, not as the constant `true`, once QSL and QSpec say where it travels. |
| R3-C8 | State whether the serializable mirrors of runtime enums stay, or the runtime exposes a serialization the generator reuses (a stated need, not a request to copy). |

To QSL:

| Id | Stated need |
| --- | --- |
| R3-Q6 | Decide whether the planned runtime edge to `quire-exact` is allowed under FB-05, and where the oracle-to-replay agreement test (T-7) lives; SR-623 records the recreation under IR-355. |
| R3-Q7 | Say where the authority's discharge of a function's `decreases` measure is carried on an admitted package (see R3-C7). |

To QSpec: none added by this AD.
