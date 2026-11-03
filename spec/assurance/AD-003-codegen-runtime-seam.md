---
id: AD-003
title: "Codegen to runtime seam: what generated crates and the generator use from this crate"
type: ArchitectureDescription
status: accepted
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

## System Boundary

This description covers codegen as a consumer of runtime's public surface. Runtime is
a leaf relative to codegen, IR and QSL. It owns default-core identity, observations,
verdicts, campaign accounting and checked operators, plus opt-in scalar negotiation and
lazy Boolean evaluation. Shared semantic values and operations have their own owners.
Other consumer seams and the pending monitor executor are outside this description.

## Views

These views separate runtime-owned behavior, shared semantic ownership, consumer use and
the dependency-direction check.

### Current ownership and use

The implemented RT surface is established by `src/lib.rs`, `src/scalar/mod.rs`,
`src/exact/mod.rs` and `Cargo.toml`; the codegen view is established by its current
`Cargo.toml` and source, rather than the earlier pre-cutover path and line-count snapshot.

| Surface | Owner | Current consumer seam |
| --- | --- | --- |
| Contract identity, observations, verdicts and campaign accounting | RT default core | CG `src/strategy/harness.rs`, `src/strategy/v2_numeric.rs` and `src/strategy/bound/generation.rs` emit the RT paths |
| Lazy Boolean and backend negotiation | RT `scalar`, feature `exact` | CG `src/core/profile.rs` tests the lazy connective; scalar emitters use the owner module |
| Exact scalars, values, outcomes and metering | `quire-exact` | CG imports the owner directly; RT imports the same owner for its kept scalar and expression code |
| Type environment, equality and object closure | `quire-semantic-value` | CG `src/oracle/equality/` and `src/oracle/function/owner_path.rs` name the shared owner directly |
| Function application remaining in RT | RT's temporary expression port | AD-002 and the IR-1009 exception govern this existing public surface; shared-owner CG paths do not make RT's port a second semantic authority |
| Generated manifest and emitted source compilation | CG | CG owns manifest generation and compilation against its selected Cargo dependencies; runtime keeps no consumer-version record |
| Snapshot wire | RT `snapshot-json` | `runtime.campaign-snapshot/v1`; decoder failures are RT's typed outcomes |

The RT `exact` feature enables `alloc` and optional `quire-exact` and
`quire-semantic-value`; it is not a module reexport of their values. Its remaining
`exact` module exposes expression items only. CG's normal runtime dependency does
not enable that feature; its development dependency enables `exact` and `proptest`.
Generated crates select the features their emitted source uses. CG also depends
on the shared owners, IR root/model, `qsl-replay` and the evaluation leaf; those
are CG's edges, never reverse dependencies from RT.

### Identity and outcomes

Authored contract identity and revision values identify the governed contract. Shared
node keys remain opaque owner values; RT does not reconstruct them or keep a file,
consumer-version or digest ledger. Cargo manifests and lockfiles carry dependency
selection. Default-core enum compatibility and shared-owner exhaustive matches remain
separate contracts.

| Scenario | Owner and result |
| --- | --- |
| Oracle pass, failed postcondition or excluded precondition | RT verdict vocabulary; rejection is not failure |
| Verdict from another campaign | `IdentityMismatch`, with no counter change |
| Counter at its bound | saturation and the existing at-limit observation |
| Shared semantic undefinedness, refusal or resource stop | the shared `Outcome`, retained by the caller |
| Invalid function argument or foreign checked expression | AD-002 planning/refusal path before charging or running that body |
| Missing backend capability | RT scalar provider disposition, separate from semantic evaluation |
| Unknown future RT enum state at a CG boundary | CG's typed refusal/unknown handling; it may not turn the state into success or an unreachable panic |

### Dependency direction and reviewed owner

| Edge | Current policy | Enforcement or review owner |
| --- | --- | --- |
| CG to RT | allowed | CG's manifest and generated-source compile controls |
| RT to shared exact/semantic-value leaves | allowed, optional under `exact` | RT manifest, lock and cargo-deny; canonical support is transitive |
| RT to CG, IR or the QSL monorepo | forbidden | Git-source policy plus runtime-maintainers' manifest/resolved-graph review; a path edge must not bypass that review |
| A cycle returning to RT, IR, CG or QSL | forbidden | reviewed all-feature normal/build dependency closure described below |
| Generated crate to RT or a shared owner | selected by the emitted manifest | CG's generated-crate compile controls |

Runtime-maintainers own the manual cross-repository direction check. It is required
when an RT manifest, lock, optional-feature graph or first-party dependency boundary
changes, and at architecture acceptance. The reviewer obtains locked, all-feature
`cargo metadata --format-version 1` for RT and selected main checkouts of IR, CG and
QSL. Starting from each root, the review follows normal and build edges, separates dev
edges, identifies workspace/package owners, and checks for forbidden RT edges and a
path returning to any of the four roots. It also inspects dev/path declarations for
an owner-dependency bypass. Unknown source or missing metadata is an unavailable check,
not a clean result.

The IR-1293 check covers RT/IR/CG application version 0.1.0 and QSL 0.2.0. It found
RT's direct production first-party edges to `quire-exact` and `quire-semantic-value`,
with canonical support transitively, no RT edge to IR/CG/QSL, and no cycle involving
those four roots. Package identities, not a collapsed repository name alone, decide
actual Cargo cycles: the wider metadata has separate packages that produce a
`filament-core-data`/`quire-rs` repository-name loop. This is not asserted to be a
Cargo package cycle, an RT edge or an all-repository DAG. The independent architecture
review records this bounded manual check and its owner; this is not a CI automation claim.

## Decisions

- Each public semantic surface has one owner. CG names RT's kept items and the shared
  owners' items directly, rather than defining another spelling or mirrored evaluator.
- Provider dispositions remain outside semantic evaluation. Unknown states must retain
  their distinct failure meaning at the consuming boundary.
- Generated manifests and compile controls belong to CG. This crate adds no consumer
  version record, compatibility alias, fallback or second copy of a shared owner.
- The expression port is retained only as the temporary staged exception tracked in
  [IR-1009](https://linear.app/agent-ix/issue/IR-1009). It expires at the usable shared
  `no_std` evaluator cutover described by AD-002/AD-004. The no_std rewrite remains
  deferred and is not staffed by this acceptance.
- The current verification configuration still imports `exact` under `cfg(kani)`.
  This acceptance changes no proof configuration and grants no core-only or backend
  proof claim. Architecture acceptance and implementation/proof qualification differ.

## Risks

The shared-evaluator cutover and deferred FR-273/FR-275 end-state acceptance stay with
IR-1009 and its predecessor IR-590. The temporary exception is not a fulfilled
one-owner end state. Changes in shared payloads or CG emission need coordinated source
and compile review; accepting this AD does not claim every CG obligation is delivered.

Manual direction checks need a maintainer review on changes; existing source policy
alone does not classify a path dependency or prove the cross-repository boundary.
A missing peer checkout or unresolved dependency graph must be reported explicitly.
