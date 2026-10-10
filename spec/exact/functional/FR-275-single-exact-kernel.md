---
id: FR-275
title: "Consume QSL's exact kernel and hold no ported QSL code"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---
# FR-275: Consume QSL's exact kernel and hold no ported QSL code

## Description

Where the `exact` feature is enabled, the runtime shall take the exact value kernel from the one
`quire-exact` crate (the kernel crate, QSL ADR-011 X-1, now in the `agent-ix/quire-exact` repository) and shall hold no copy,
port or re-implementation of any item `quire-exact` exports. The end state is that the runtime holds
no ported QSL code at all: it is reached via `quire-exact`, `quire-semantic-value` (QSL-358),
and the QSL evaluation leaf after IR-590 target acceptance.
The runtime has no `quire_contract_runtime::exact` module: it re-exports
no item of those QSL-owned crates (no `pub use`, alias, wrapper, feature or compatibility layer), and
generated oracles and the code generator name the owning crates' own paths. Under the `exact` feature
the runtime's own code is only the `scalar` module: the backend negotiators and the lazy Boolean
connective, which are runtime-owned (below; AD-004). The rest of its `src/exact` source copy (23 files,
about 12,000 lines) is deleted. This is the end state; the criteria below that assert it are planned
until the IR-349 code steps land.

The end state is delivered in two steps:

1. **Step 1, the `quire-exact` exports now (IR-349 part 1).** Everything `quire-exact` exports today
   is deleted from the runtime and consumed from `quire-exact`.
2. **Step 2, the residue.** QSL says (as relayed by the team leader, from the IR planner) that
   QSL-358 slices place checking environments, containment and unit graphs, enumeration declarations
   and carried compiler vocabulary in `quire-semantic-value` (QSL ADR-011 layer SV); `quire-exact`
   stays the kernel. Function application belongs to QSL's separate evaluation leaf: IR-583 defines
   its shared `core` plus `alloc` interface and std reference seam; IR-590 supplies the usable
   `no_std` plus `alloc` evaluator (QSL FR-262). The owner ruled FB-05 YES (relayed via the planner):
   shared QSL leaves the runtime may depend on. The residue is about 3,365 lines of the runtime's source (as
   relayed); about 7,670 further lines already exist in `quire-exact` in a different shape (`Rc`
   against `Arc`, `EffectiveId` against `NodeKey`, `Quantity`), which IR-349 adapts to. The residue
   is not runtime-owned: the runtime deletes it and consumes the QSL-owned crates. The order and
   mechanism are tracked by QSL-358, IR-583, IR-590 and IR-349. IR-583 alone does not authorize
   deletion of the function-application copy; IR-590's checked-input and generated-body implementation
   must pass RT's governed `no_std` target acceptance first.
3. **The negotiators are runtime-owned, not residue.** QSL deleted its negotiators on purpose
   (QSL FR-078, as relayed), so `negotiate_integer_division`, `negotiate_ieee` and their types are the
   runtime's own code, not a port, and stay in the runtime in the end state (FR-009,
   interface-001-AC-8). QSL-358 does not move them.

The residue in the runtime's `src/exact` is ported QSL code. It is **not authorized**: it is vendored
code that violates the no-vendoring rule. There is no exception, no expiry and no approval for it. The
owner (Peter) ruled directly: "no vendoring. i didnt realize you had tried to approve vendoring 3000
lines of corpus. Absolutely not allowed." The end state is that the runtime holds no ported QSL code,
and the residue is to be deleted. How the runtime handles its function-application copy until the
usable IR-590 `no_std` evaluator passes RT target acceptance is an open decision with the owner
(see Open questions); this requirement states no interim policy.

This requirement states the end state of IR-342 (AD-016 owner decision 2) and the rule the code
steps IR-349 implement. It follows AD-003 decision F. It adds no behaviour: every behaviour the
kernel owns stays specified by FR-006 to FR-008 and FR-010 to FR-012, now as behaviour of QSL's
crates that the runtime consumes.

## Inputs

- The `exact` feature of `quire-contract-runtime`, and the `quire-exact` and `quire-semantic-value` crates it enables.
- The runtime's `Cargo.toml`, `Cargo.lock` and `deny.toml`.

## Outputs

- A build graph in which `quire-exact` and `quire-semantic-value` each resolve once, and in the end state a runtime with no `exact`
  module whose `exact`-feature code defines only the runtime-owned `scalar` items and re-exports no
  QSL-owned crate.
- A build that rejects the QSL repository as a Git dependency source.

## Behavior

- **One kernel.** `quire-exact` is the one owner of values and value types, the kernel `Outcome` and
  its `Undefined`, `Refusal` and `Incomplete` reasons, `Meter` charge-before-work accounting,
  `Origin`/`Location` provenance, `NodeKey`, and the scalar and collection operations over them.
  The runtime defines none of them (interface-001-AC-3 and AC-5 state the same rule per item; AC-2 and AC-4 state that no path re-exports them).
  Deleting the runtime's definitions is deletion, not relocation: no module, re-export alias,
  feature or wrapper keeps the old definitions reachable.
- **The residue (not authorized).** The `exact` items `quire-exact` does not export, other than the
  runtime-owned `scalar` items (the negotiators and their types, `evaluate_boolean_short_circuit` and
  `ShortCircuitConnective`, which are not residue), are still in the runtime's source: function application (FR-273, AD-002), the static
  checking environments, containment and unit graphs, enumeration declarations and the carried
  compiler vocabulary (FR-012). They are a port of QSL code: vendored code that violates the
  no-vendoring rule, with no exception, no expiry and no approval. Each is a defect against
  FR-275-AC-1's end state, recorded in the residue list below, and is to be deleted. Where they
  consume `quire-exact`, they use its `Value`, `Meter`, `Outcome` and `ScalarLimits` and define no
  second one. Nothing replaces them under an `exact` path: consumers name the owning crates' own
  paths (interface-001-AC-4).
- **The end state.** There is no `exact` module. The runtime defines under the `exact` feature only the
  runtime-owned `scalar` items (the negotiators and their types, and `evaluate_boolean_short_circuit`
  with `ShortCircuitConnective`): the rest of `src/exact` is deleted, no `pub use`, alias or wrapper
  re-exports `quire-exact`, `quire-semantic-value` or the evaluation leaf crate (IR-583 interface,
  IR-590 implementation), and no compatibility layer keeps the old source reachable. The runtime
  then depends on each of those crates
  once, by the same spelling rule as `quire-exact`.
- **Runtime-owned negotiators.** `negotiate_integer_division`, `negotiate_ieee` and their requirement,
  capability and disposition types are defined in the runtime's own source (interface-001-AC-8). They
  are not a port and not vendoring, and they are outside QSL-358 and the residue list. They
  consume the QSL `Value`, `Meter` and `Outcome` types where they need one.
- **Dependency spelling.** `quire-exact` and `quire-semantic-value` are optional Git dependencies on
  their own repositories, enabled only by the `exact` feature (interface-001-AC-11), and spelled
  `branch = "main"` as every first-party dependency is (IR-434). It carries no `rev`, `tag` or
  `path`, and no committed `[patch]`; local development against a sibling checkout uses the
  untracked `make use-local` patch. The runtime records no version, commit or digest of
  either shared crate in its sources or specification.
- **One copy.** `Cargo.lock` holds one entry each for `quire-exact` and `quire-semantic-value`;
  `quire-canonical` and `quire-canonical-derive` enter transitively. `scripts/check_one_copy.awk`,
  run by `make deny`, checks every first-party Git crate. `deny.toml` admits only their three
  own-repository Git sources and carries scoped licence exceptions for the four crates.
- **Guarded edges.** No normal, build or dev dependency may resolve from the QSL repository.
  `unknown-git = "deny"` rejects that repository because `allow-git` admits only the three
  own-repository sources. `make deny-mutations` adds QSL Git dependencies in scratch copies and
  requires cargo-deny's source-rejection diagnostic for normal, build and dev dependencies. A licence
  error alone does not prove the source guard.
- **No substitute for the copy.** The runtime keeps no test that compares its output with a second
  implementation of the kernel, no shared-corpus agreement test, no vendored vector or fixture from
  another repository, and no compatibility layer for the removed copy. With one kernel there is
  nothing to agree with. Tests of kernel behaviour belong to the QSL repository. In step 1 the
  runtime keeps only the tests of the residue; when the residue is deleted it keeps none of them,
  because the residue's tests move to QSL with the code (the dispositions are in the test matrix, "Evidence at
  the kernel move").
- **`no_std` plus `alloc`.** `quire-exact` and `quire-semantic-value` are `#![no_std]` with `alloc`.
  `quire-exact` builds for
  `thumbv7em-none-eabi` in the QSL repository's `make ci` (QSL-357). With `exact` enabled and `std`
  disabled the runtime builds for that target. `quire-exact` shares values through `alloc::sync::Arc`,
  which needs the target's atomic compare-and-swap; the runtime's only `no_std` target,
  `thumbv7em-none-eabi`, has it. The runtime's own `exact` source uses no `std`.
- **Footprint.** The default profile resolves no `quire-exact`, `quire-semantic-value`,
  `quire-canonical` or `quire-canonical-derive`: the `quire-contract-runtime-footprint`
  measurement crate depends on the runtime with `default-features = false`, so NFR-001's 500 byte
  floor, 4 KiB ceiling and panic-path check measure the same code as before and `make size` is the
  check. The kernel's size is outside that budget, as the `exact` feature always was.
- **Toolchain.** The runtime takes one floor, Rust 1.98.1, for all features and for the footprint
  measurement: the owner's decision (Peter, 2026-10-01: "we are on rust 1.98.1"), the toolchain in
  use everywhere the runtime spells one. It replaces the 1.75 floor in interface-001 and
  NFR-001-AC-3, and is above the Rust 1.82 that `quire-exact` needs (as relayed), so `quire-exact`
  builds on it. The `make msrv` and `make size` toolchain, `rust-version`, `clippy.toml` and
  `MSRV` settings and the CI `msrv` job follow this floor.
- **Nothing is lost.** No requirement, acceptance criterion or test case is deleted by this move.
  A row whose evidence leaves the runtime stays in the matrix as planned, with the reason stated.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-275-AC-1 | With `exact` enabled, no public item the runtime defines (in `scalar` or any other module) has the name of an item that `quire-exact` or `quire-semantic-value` exports. | Inspection (TC-197) |
| FR-275-AC-2 | No module, alias or feature of the runtime keeps a removed kernel definition reachable. | Inspection (TC-197) |
| FR-275-AC-3 | `quire-exact` is an optional dependency of the runtime, enabled only by the `exact` feature. | Inspection (TC-197) |
| FR-275-AC-4 | The optional `quire-exact` and `quire-semantic-value` dependencies use their own `agent-ix` Git repositories with `branch = "main"` and no `rev`, `tag`, `path` or committed `[patch]`. | Inspection (TC-197) |
| FR-275-AC-5 | `Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`. | Test (TC-197, `make deny`) |
| FR-275-AC-6 | `make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate. | Test (TC-197) |
| FR-275-AC-7 | `deny.toml` admits only the `quire-exact`, `quire-semantic-value` and `quire-canonical` own-repository Git sources and keeps `unknown-git = "deny"`; it does not admit the QSL Git repository. | Inspection (TC-198) |
| FR-275-AC-8 | `make deny` reports cargo-deny's `source-not-allowed` error for the QSL Git repository, and exits non-zero, when a normal, build or dev dependency resolves from it; a licence failure alone does not satisfy this. | Test (TC-198) |
| FR-275-AC-9 | With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`. The row `build-exact-no-std-msrv` builds it on the 1.98.1 floor. | Test (TC-199, `make test-features` row `build-exact-no-std-msrv`) |
| FR-275-AC-10 | The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi` contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`. | Test (TC-199) |
| FR-275-AC-11 | `make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3's 500 byte to 4 KiB band. | Test (TC-199, `make size`) |
| FR-275-AC-12 | The runtime has no test that compares its output with a second implementation of the kernel or that depends on a crate from the QSL Git repository. | Inspection (TC-197) |
| FR-275-AC-13 | The runtime contains no file copied from the QSL repository and no port of QSL code, where a port is code that keeps the QSL authority's item names and order. The residue list is not an allowance: every item on it is a violation of this criterion, which stays unmet while the list is non-empty. The runtime-owned negotiators are not ports. | Inspection (TC-197) |
| FR-275-AC-14 | The kernel move deletes no requirement, acceptance criterion or test case. | Inspection (TC-197) |
| FR-275-AC-15 | Every matrix row whose evidence leaves the runtime in the kernel move carries a planned status with a stated reason, because the status vocabulary has no "verified upstream" status (a planner question, no status is invented). | Inspection (TC-197) |
| FR-275-AC-16 | The runtime has no `quire_contract_runtime::exact` module and no `pub use` of `quire-exact`, `quire-semantic-value` or the evaluation leaf crate; the only items it defines under the `exact` feature are the `scalar` items (the backend negotiators and their types, `evaluate_boolean_short_circuit` and `ShortCircuitConnective`), and no file under `src/exact` exists. | Inspection (TC-197) |
| FR-275-AC-17 | Every item the runtime defines under the `exact` feature, other than the `scalar` items, is named in the residue list, and the list records no exception, expiry or approval. | Inspection (TC-197) |
| FR-275-AC-18 | The residue list is empty. | Inspection (TC-197) |
| FR-275-AC-19 | The runtime defines `negotiate_integer_division`, `negotiate_ieee` and their types in its own source and depends on no QSL crate for them. | Inspection (TC-197) |
| FR-275-AC-20 | The runtime's declared `rust-version` is 1.98.1. | Inspection (TC-199) |
| FR-275-AC-21 | `make msrv` and `make size` build with Rust 1.98.1. | Test (TC-199) |

## Residue list (not authorized; to be deleted)

Every item below is vendored QSL code that violates the no-vendoring rule. There is no exception,
no expiry and no approval. The order and mechanism of deletion are tracked by QSL-358 and IR-349;
function-application deletion also needs IR-583, IR-590 and RT target acceptance.
Classified against what `quire-exact` exports; IR-349 part 1 re-measures it. Where a source file holds both an exported item and a residue item, only the
residue item is listed: the exported item is deleted in step 1. The negotiators are not on this list: they are runtime-owned.

| Residue | Requirement | Source today |
|---|---|---|
| Function application: `PackageDeclarations`, `CheckedPackage`, `Frame`, `Body`, `Evaluation`, `plan_call` | FR-273 | `src/exact/expression.rs` |
| Checking environments: `TypeEnvironment`, composite declarations, `ObjectEnvironment`, `CheckedEquality` | FR-008 | `src/exact/composite.rs` (declarations and `TypeEnvironment`, not `Value`/`ValueType`), `reference.rs` (`ObjectEnvironment` and its refusal types only; the identities and `ObjectReference` are exported by `quire-exact` and are deleted in step 1), `equality.rs` (`CheckedEquality`, not the equality plan) |
| Containment and unit graphs, enumeration declarations | FR-008, FR-007 | `src/exact/containment.rs`, `unit.rs`, `enumeration.rs` |
| Carried compiler vocabulary | FR-012 | `src/exact/definition.rs`, `node.rs` (`SemanticGraphCause` and `InvalidSemanticGraph` only; `NodeKey` is exported by `quire-exact` and is deleted in step 1) |

The list is a classification aid, not a frozen record; an item `quire-exact` exports is not residue
and is deleted in step 1.

## Open questions

- **MSRV is the owner's decision.** One floor, Rust 1.98.1, for all features, decided by Peter on
  2026-10-01 (it replaces the 1.82 the IR planner proposed under IR-18, as relayed). The footprint
  measurement at 1.98.1 was taken by the IR-349 foundation slice (NFR-001-AC-3).
- **Interim function-application residue (open, owner decision).** The owner ruled there is no
  exception, no expiry and no approval for the copy. What the runtime does before the IR-590
  implementation passes RT target acceptance is not decided here; no interim policy is stated.
- **Status for rows whose evidence lives upstream (planner question).** The matrix has no "verified
  upstream" status; leaving rows stay "planned" with a reason (FR-275-AC-15).
- **`quire-semantic-value`.** The owner ruled FB-05 YES (relayed via the planner): a shared `no_std`
  plus `alloc` leaf the runtime may depend on. The dependency spelling and one-copy rules above
  apply to it once it exists.
- **Kernel shape change.** `quire-exact`'s `Value` is not the runtime's today (it shares through
  `Arc`, carries no object graph, and lacks cross-unit quantity arithmetic and the equality
  conversion table). QSL-358 slice 0 gives it an iterative `Value` (as relayed). Adapting the
  runtime to the new shape is code-step work (IR-349, QSL-358) and is not decided here.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  [interface-001](../../core/functional/interface-001-runtime-api.md);
  [AD-003](../../assurance/AD-003-codegen-runtime-seam.md) decision F; QSL-357 (merged);
  [NFR-001](../../core/non-functional/NFR-001-no-std-footprint.md).
- **Downstream**: the code steps IR-349: the foundation slice (floor, dependency, bans, one copy;
  no deletion; it adds the `deny.toml` entries) comes first; part 1 deletes the `quire-exact`
  exports; part 2 deletes the residue, with function-application deletion gated by the IR-583 interface, IR-590 implementation and RT target acceptance (QSL FR-262).
- **Routed**: the codegen repository's lock resolves two `quire-exact` copies and three
  `quire-contract-model` revisions; that is codegen's one-copy work (its layout AD, step 1d, the
  codegen owner) and is not a runtime requirement. This requirement covers the runtime only, so
  IR-342 stays open after it merges for the codegen part; the team leader coordinates it. Checked
  read-only at the IR repository's main: it declares no dependency on `quire-exact` and holds no
  copy of the exact kernel (no `Meter`, `ScalarLimits` or kernel `Outcome` definition), so IR needs
  no requirement from this ticket.
