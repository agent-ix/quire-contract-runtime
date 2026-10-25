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
  - target: ix://agent-ix/quire-cli/FR-027
    type: depends_on
  - target: ix://agent-ix/quire-semantic-value/FR-124
    type: depends_on
---
# FR-275: Consume QSL's exact kernel and hold no ported QSL code

## Description

Where the `exact` feature is enabled, the runtime shall take the exact value kernel from the one
`quire-exact` crate (the kernel crate, QSL ADR-011 X-1, now in the `agent-ix/quire-exact` repository) and shall hold no copy,
port or re-implementation of any item `quire-exact` exports. The runtime shall remove
its distinct `Value` enum, `CheckedEquality` evaluator, `planned_equality`,
`plan_pairs` and collection membership walker; it shall consume QSV's
`CheckedEquality::evaluate` over `quire_exact::Value` directly. The QSV evaluator shall call
`quire_exact::planned_equality_with_plan` for its planned pair schedule. The runtime shall define
no conversion layer or old-`Value` re-export. The end state also holds
no ported QSL code at all: it is reached via `quire-exact`, `quire-semantic-value` (QSL-358),
and the QSL evaluation leaf after IR-590 target acceptance.
The runtime has no `quire_contract_runtime::exact` module: it re-exports
no item of those QSL-owned crates (no `pub use`, alias, wrapper, feature or compatibility layer), and
generated oracles and the code generator name the owning crates' own paths. Under the `exact` feature
the runtime's own code is only the `scalar` module: the backend negotiators and the lazy Boolean
connective, which are runtime-owned (below; AD-004). The rest of its `src/exact` source copy (23 files,
about 12,000 lines) is deleted. This is the end state; the criteria below that assert it are planned
until the owning IR-349 and IR-1009 code steps land.

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
   mechanism are tracked by QSL-358, IR-583, IR-590, IR-349 (independent AD-004 steps 1 through 4)
   and IR-1009 (AD-004 step 5 and dependent end-state closure). IR-583 alone does not authorize
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
steps IR-349 and IR-1009 implement. It follows AD-003 decision F. It adds no behaviour: every behaviour the
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
- **Shared identity and declaration owners.** The kernel has no cross-unit quantity
  arithmetic or equality conversion table. RT consumes `quire-semantic-value`'s
  `UnitTable::get(UnitId)` for admitted units, `TypeEnvironment::object_type(EffectiveId)`
  for object declarations, and `EnumMemberIndex::resolve(VariantId)` with
  `EnumDeclaration` for enum members and shapes. The IR-882 migration repoints
  quantity, enum and reference payloads to those owner types directly, with no
  RT-owned `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, second
  value representation or adapter.
- **The residue (not authorized).** The `exact` items retained in the runtime's source beyond the
  runtime-owned `scalar` items (the negotiators and their types, `evaluate_boolean_short_circuit` and
  `ShortCircuitConnective`) are function application (FR-273, AD-002), quantity and unit graphs,
  and the carried definition vocabulary (FR-012). RT's checking environments, containment and
  enumeration declarations have been removed by IR-882 slices. The retained items are ports of QSL
  code: vendored code that violates the
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
  `quire-canonical` and `quire-canonical-derive` enter transitively. `make deny` invokes the distributed `quire check-lock` command once on all
  tracked Cargo.lock files; the copied AWK checker is removed. [CLI FR-027](ix://agent-ix/quire-cli/FR-027) owns that algorithm;
  AC-24 through AC-28 govern this repository's adoption. `deny.toml` admits their three
  own-repository Git sources plus the dev-only `ix-trace-rs` tooling source, and carries scoped
  licence exceptions for the four shared crates.
- **Guarded edges.** No normal, build or dev dependency may resolve from the QSL workspace repository.
  `unknown-git = "deny"` rejects that repository because `allow-git` admits only those three
  own-repository sources and the dev-only `ix-trace-rs` tooling source before AD-004 step 5;
  the evaluation leaf's own repository may be admitted after that step. `make deny-mutations`
  adds QSL Git dependencies in scratch copies and
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
  After AD-004 step 5 the default graph also resolves no evaluation leaf. The
  governed default linked `.text` plus `.rodata` band remains 500 bytes to
  4 KiB on `thumbv7em-none-eabi` at Rust 1.98.1; the deep exact-profile
  acceptance instead caps the native thread stack at 512 KiB. No linked-size
  or peak-heap ceiling for the `exact` profile has been approved. Its linked
  bytes and native peak memory are reported by the acceptance run, separately from
  the default footprint, without claiming the 4 KiB ceiling for them.
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
| FR-275-AC-2 | No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, `CheckedEquality` evaluator, pair walker or membership walker, `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, copy of `UnitTable`, `TypeEnvironment`, `EnumMemberIndex` or `EnumDeclaration`, conversion layer, adapter or old-`Value` re-export. Quantity, enum, reference and checked-equality paths use the QSV owners directly. | Inspection (TC-197) |
| FR-275-AC-3 | `quire-exact` is an optional dependency of the runtime, enabled only by the `exact` feature. | Inspection (TC-197) |
| FR-275-AC-4 | The optional `quire-exact` and `quire-semantic-value` dependencies use their own `agent-ix` Git repositories with `branch = "main"` and no `rev`, `tag`, `path` or committed `[patch]`. | Inspection (TC-197) |
| FR-275-AC-5 | `Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`, and, after AD-004 step 5, the QSL-owned evaluation leaf. | Test (TC-197, `make deny`) |
| FR-275-AC-6 | `make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate. | Test (TC-202) |
| FR-275-AC-7 | `deny.toml` admits only the `quire-exact`, `quire-semantic-value` and `quire-canonical` own-repository Git sources and the dev-only `ix-trace-rs` first-party tooling source before AD-004 step 5, then also the QSL-owned evaluation leaf's own-repository Git source; it keeps `unknown-git = "deny"` and never admits the QSL Git repository. | Inspection (TC-198) |
| FR-275-AC-8 | `make deny` reports cargo-deny's `source-not-allowed` error for the QSL Git repository, and exits non-zero, when a normal, build or dev dependency resolves from it; a licence failure alone does not satisfy this. | Test (TC-198) |
| FR-275-AC-9 | With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`. The row `build-exact-no-std-msrv` builds it on the 1.98.1 floor. | Test (TC-199, `make test-features` row `build-exact-no-std-msrv`) |
| FR-275-AC-10 | The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi` contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`, and after AD-004 step 5 contains no evaluation leaf. | Test (TC-199) |
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
| FR-275-AC-22 | Inspection of runtime source finds no local definition or alias of QSV's `SemanticGraphCause`, `InvalidSemanticGraph`, or semantic-node term checker, including definitions emitted by macros. | Inspection (TC-205) |
| FR-275-AC-23 | After the step-4 value-bearing cutover, QSV's `Conversion::identified_quantity` returns a `quire_exact::Quantity` with the exact magnitude and the target's compound-arm `UnitId`, paired with QSV's `IdentifiedUnit` for that same target; a non-exact conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)` to supply the same normalized `QuantityUnit::Compound` to the next conversion. Declared and compound ID arms remain distinct even for equal bytes. QSpec TC-187 U25's first compound-target conversion feeds its result into the conversion to `J` with exact value `1` and FR-007-AC-23's charges, without a new charge for the handoff. RT defines no `CompoundUnit` identity, `UnitId` resolver, parallel unit table or conversion adapter. | Test |
| FR-275-AC-24 | The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock` command on this repository's committed lock. On a coherent scratch lock the real command succeeds and the recipe completes when its other policy checks pass; an invocation record identifies the executable and argument without substituting checker logic. | Test |
| FR-275-AC-25 | Through the same normal `make deny` recipe, a scratch lock with duplicate first-party Git package names and a scratch lock with distinct crate names at split resolved revisions of one repository each cause a non-zero Make exit and the real command's coherence diagnostic. The aligned multi-crate control passes the command. Every agent-ix Git source in these coherence controls has a valid full resolved fragment admitted by CLI FR-027; a malformed-input diagnostic cannot satisfy a duplicate or split-revision control. These controls exercise CLI FR-027; they do not define a second normalization algorithm. | Test |
| FR-275-AC-26 | A missing checker executable, malformed scratch lock, or checker refusal cannot produce successful `make deny`. The recipe propagates failure without an ignored exit, success fallback, or skipped check. | Test |
| FR-275-AC-27 | The committed `scripts/check_one_copy.awk` file is absent, the normal recipe contains no reference to it, and no replacement local implementation, executable copy, compatibility shim, or revision allowance bypasses the owned command. | Inspection |
| FR-275-AC-28 | Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged and adds no runtime dependency on developer tooling. Qualification runs the real command on the committed lock and reports any existing incoherence as a failing gate with an owning blocker; a scratch passing control or an unrelated gate failure cannot qualify that lock. | Test |
| FR-275-AC-29 | After AD-004 step 5, with `exact` enabled and `std` disabled on Rust 1.98.1, the RT consumer and a generated-body consumer of the QSL-owned evaluation leaf compile for `thumbv7em-none-eabi` using one lock entry for that leaf; neither consumer resolves the QSL workspace Git source, an RT-local evaluator copy, or a compatibility alias. A host build of the same no_std profile executes the checked-input and generated-body cases of FR-273-AC-12 and AC-13. | Test (TC-199, TC-194) |
| FR-275-AC-30 | After AD-004 step 5, `make size` still measures the default fixed population at 500 bytes through 4 KiB linked `.text` plus `.rodata` with no evaluation leaf in its graph and no panic relocation. A separate `measurement/evaluator` no_std executable that links RT, the QSL evaluation leaf and one generated-body consumer for `thumbv7em-none-eabi` has its linked ELF path and `.text` plus `.rodata` byte sum reported from `llvm-size -A`. A solitary native FR-273-AC-13 deep-test executable has its peak resident set size reported in bytes from `/usr/bin/time -v`'s `Maximum resident set size` field, with the reported KiB multiplied by 1024. The receipt names both artifact paths and measurements separately from the default footprint; neither is compared with the default 4 KiB ceiling, and no exact-profile memory limit is inferred from this default measurement. | Test (TC-199) |

## Lock Gate Adoption Status

The copied checker is removed. The normal Make recipe invokes the owned command;
consumer qualification uses the installed executable selected through `QUIRE`.
A local candidate qualification does not establish release publication or other
hosts' installation. AC-5 and AC-6 remain
required; AC-24 through AC-28 extend their consumer acceptance without redefining
CLI FR-027 or replacing existing runtime dependency-policy checks.

## Residue list (not authorized; to be deleted)

Every item below is vendored QSL code that violates the no-vendoring rule. There is no exception,
no expiry and no approval. The order and mechanism of deletion are tracked by QSL-358 and IR-349;
function-application deletion is assigned to IR-1009 and needs IR-583, IR-590 and RT target acceptance.
Classified against what `quire-exact` exports; IR-349 part 1 re-measures it. Where a source file holds both an exported item and a residue item, only the
residue item is listed: the exported item is deleted in step 1. The negotiators are not on this list: they are runtime-owned.

| Residue or disposition | Requirement | Source status |
|---|---|---|
| Function application: `PackageDeclarations`, `CheckedPackage`, `Frame`, `Body`, `Evaluation`, `plan_call` | FR-273 | `src/exact/expression.rs` |
| Checking environments, composite declarations, object closure and `CheckedEquality` | FR-008 | RT's copies in `src/exact/composite.rs`, `reference.rs` and `equality.rs` were removed in the IR-882 value-bearing cutover; QSV owns these interfaces. |
| Containment and unit graphs, enumeration declarations | FR-008, FR-007 | RT's `containment.rs`, `enumeration.rs` and `unit.rs` were removed in IR-882; QSV owns their declarations and graph behavior. |
| Carried compiler vocabulary | FR-012 | RT's `node.rs` and `definition.rs` were deleted in IR-882. R-3 retired `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch`; RT consumes QSV's vocabulary directly, with no alias or adapter (FR-012-AC-7). |

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
- **Kernel shape change.** The locked `quire_exact::Value` has `Arc`-backed
  option, composite and collection nodes, iterative formatting and nested-node
  drop, and direct constructors and collection operations. It is distinct from
  RT's current `Rc`-backed `Value`. IR-882 migrates RT to the kernel value
  with no conversion layer; the RT `repr(u64)` tag is a layout choice, not a
  semantic requirement. The code step measures Kani and footprint effects.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  [interface-001](../../core/functional/interface-001-runtime-api.md);
  [AD-003](../../assurance/AD-003-codegen-runtime-seam.md) decision F; QSL-357 (merged);
  [NFR-001](../../core/non-functional/NFR-001-no-std-footprint.md).
- **Downstream**: IR-349 keeps the independent AD-004 steps 1 through 4, including the
  runtime-owned `scalar` items and paired CG consumers. The foundation slice (floor,
  dependency, bans, one copy; no deletion) preceded those steps. IR-1009 receives AD-004
  step 5, the ported function-application deletion and paired CG consumption, gated by
  the IR-583 interface, IR-590 implementation and RT target acceptance (QSL FR-262),
  together with the dependent empty-`exact` end-state closure in step 6. FR-275-AC-1,
  AC-13, AC-16, AC-18, AC-29 and AC-30 remain unchanged and planned until their full
  end-state evidence exists; this delivery split waives none of them.
- **Routed**: the codegen repository's lock resolves two `quire-exact` copies and three
  `quire-contract-model` revisions; that is codegen's one-copy work (its layout AD, step 1d, the
  codegen owner) and is not a runtime requirement. This requirement covers the runtime only, so
  IR-342 stays open after it merges for the codegen part; the team leader coordinates it. Checked
  read-only at the IR repository's main: it declares no dependency on `quire-exact` and holds no
  copy of the exact kernel (no `Meter`, `ScalarLimits` or kernel `Outcome` definition), so IR needs
  no requirement from this ticket.

## Consumer Verification Plan

Repository-owned integration tests bind each new criterion on actual
`#[test]` functions. The tests run the normal `make deny` recipe in a scratch
consumer tree with the actual released Quire executable. They may isolate the
unrelated Cargo policy stage to make the checker observation decisive, but may
not substitute the checker, copy its algorithm, or treat a malformed-input refusal
as coherence evidence. Final qualification separately runs the repository's
normal unchanged gates on its committed lock.

The runtime consumer controls extend `tests/deny_one_copy.rs` and the dependency
policy qualification of TC-197 and TC-202. They execute the owned checker directly through the Make recipe, including
aligned, duplicate-name, split-revision, malformed-input and missing-command controls.
