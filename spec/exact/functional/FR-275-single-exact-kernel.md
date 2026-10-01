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
`quire-exact` crate (the QSL repository's kernel crate, QSL ADR-011 X-1) and shall hold no copy,
port or re-implementation of any item `quire-exact` exports. The end state is that the runtime holds
no ported QSL code at all: its `exact` module is a re-export of QSL-owned crates at the
`quire_contract_runtime::exact` path generated oracles already import, and its `src/exact` source
copy (23 files) is deleted.

The end state is delivered in two steps:

1. **Step 1, the `quire-exact` exports now (IR-349 part 1).** Everything `quire-exact` exports today
   is deleted from the runtime and consumed from `quire-exact`.
2. **Step 2, the residue after QSL-358 phase 2.** QSL says (as relayed by the team leader) that
   QSL-358 moves the whole residue (function application, checking environments, containment and
   unit graphs, and the backend negotiators) into QSL-owned `no_std` plus `alloc` leaf crate(s).
   When QSL-358 phase 2 is merged the runtime deletes the rest of `src/exact` and consumes those
   crate(s). The residue is not runtime-owned.

Between the two steps the residue stays in the runtime. That is a **temporary exception to the
no-vendoring rule**, not a settlement: its expiry condition is "QSL-358 phase 2 merged", it is owned
by QSL-358 and IR-349, and the owner's approval of the exception is **pending**. An exception with
no approval is not in force; step 2 is the only thing that ends it.

This requirement states the end state of IR-342 (AD-016 owner decision 2) and the rule the code
steps IR-349 implement. It follows AD-003 decision F. It adds no behaviour: every behaviour the
kernel owns stays specified by FR-006 to FR-008 and FR-010 to FR-012, now as behaviour of QSL's
crates that the runtime consumes.

## Inputs

- The `exact` feature of `quire-contract-runtime`, and the `quire-exact` crate it enables.
- The runtime's `Cargo.toml`, `Cargo.lock` and `deny.toml`.

## Outputs

- A build graph in which `quire-exact` resolves once, and (after step 2) a runtime `exact` module
  that defines nothing and only re-exports QSL-owned crates.
- A build that fails when a QSL crate other than `quire-exact` enters the runtime's dependency graph.

## Behavior

- **One kernel.** `quire-exact` is the one owner of values and value types, the kernel `Outcome` and
  its `Undefined`, `Refusal` and `Incomplete` reasons, `Meter` charge-before-work accounting,
  `Origin`/`Location` provenance, `NodeKey`, and the scalar and collection operations over them.
  The runtime defines none of them (interface-001-AC-2, AC-3 and AC-5 state the same rule per item).
  Deleting the runtime's definitions is deletion, not relocation: no module, re-export alias,
  feature or wrapper keeps the old definitions reachable.
- **The interim residue (temporary exception).** Until QSL-358 phase 2 is merged, the `exact` items
  `quire-exact` does not export stay in the runtime's source: function application (FR-273, AD-002),
  the static checking environments, containment and unit graphs, backend negotiation (FR-009) and the
  carried compiler vocabulary (FR-012). They are a port of QSL code and are kept only under the
  temporary exception above (expiry: QSL-358 phase 2 merged; owner approval pending). They consume
  `quire-exact`'s `Value`, `Meter`, `Outcome` and `ScalarLimits`; they define no second one. They
  keep their `exact` path when they move to QSL's crate(s) (interface-001-AC-4).
- **The end state.** After QSL-358 phase 2 the runtime holds no definition of any `exact` item: the
  rest of `src/exact` is deleted, `exact` re-exports QSL-owned crates, and no compatibility layer
  keeps the old source reachable. The runtime then depends on each QSL crate it consumes once, by
  the same spelling rule as `quire-exact`; the names of the QSL crate(s) are QSL-358's to state.
- **Dependency spelling.** The dependency is a git dependency on the QSL repository for the crate
  `quire-exact`, optional, enabled only by the `exact` feature (interface-001-AC-11), and spelled
  `branch = "main"` as every first-party dependency is (IR-434). It carries no `rev`, `tag` or
  `path`, and no committed `[patch]`; local development against a sibling checkout uses the
  untracked `make use-local` patch. The runtime records no version, commit or digest of
  `quire-exact` in its sources or specification.
- **One copy.** `Cargo.lock` holds exactly one `quire-exact` entry (`scripts/check_one_copy.awk`,
  run by `make deny`). `deny.toml` admits that one git source (`allow-git`) and a licence exception
  for the `quire-exact` crate; `unknown-git = "deny"` stays for every other git source.
- **Guarded edges.** The runtime depends on no crate of the QSL repository other than `quire-exact`
  (and, after step 2, the QSL-owned crate(s) QSL-358 names for the residue). `deny.toml` carries a
  `[bans]` `deny` entry for each QSL workspace crate other than those, enumerated from QSL's
  workspace members at QSL main 8b0c1ffe: the root crate `quire-spec-language`, `qsl-attrs`,
  `qsl-bench`, `qsl-cst`, `qsl-eval`, `qsl-foundation`, `qsl-forms`, `qsl-package`, `qsl-replay`,
  `qsl-route`, `qsl-semantics`, `qsl-source`, `xtask` and `arch-lint`. A normal, build or dev
  dependency on any of them fails `make deny` with cargo-deny's `banned` diagnostic. This
  enumeration is what the guard covers and no more: a crate QSL adds to its workspace later is
  not banned until RT's list is extended, because `allow-git` admits the whole repository. The
  complete guard is this list plus QSL's own lint (QSL-356, QSL #554, in review), and keeping the
  list current is an open item owned by the runtime maintainers (see Open questions). Each
  repository guards its own edges, so the guard does not rely on the lint. A dev dependency on
  `qsl-eval` for conformance is not permitted: it is the same edge. If QSL-358 gives a residue
  crate a name on the list, that entry is removed then. The sources policy alone does not catch this, because `allow-git` names a repository, not a
  crate.
- **No substitute for the copy.** The runtime keeps no test that compares its output with a second
  implementation of the kernel, no shared-corpus agreement test, no vendored vector or fixture from
  another repository, and no compatibility layer for the removed copy. With one kernel there is
  nothing to agree with. Tests of kernel behaviour belong to the QSL repository. In step 1 the
  runtime keeps only the tests of the interim residue; in step 2 it keeps none of them, because the
  residue's tests move to QSL with the code (the dispositions are in the test matrix, "Evidence at
  the kernel move").
- **`no_std` plus `alloc`.** `quire-exact` is `#![no_std]` with `alloc`, and builds for
  `thumbv7em-none-eabi` in the QSL repository's `make ci` (QSL-357). With `exact` enabled and `std`
  disabled the runtime builds for that target. `quire-exact` shares values through `alloc::sync::Arc`,
  which needs the target's atomic compare-and-swap; the runtime's only `no_std` target,
  `thumbv7em-none-eabi`, has it. The runtime's own `exact` source uses no `std`.
- **Footprint.** The default profile resolves no `quire-exact`: the `quire-contract-runtime-footprint`
  measurement crate depends on the runtime with `default-features = false`, so NFR-001's 500 byte
  floor, 4 KiB ceiling and panic-path check measure the same code as before and `make size` is the
  check. The kernel's size is outside that budget, as the `exact` feature always was.
- **Toolchain.** `quire-exact` declares `rust-version = "1.98"`, above the runtime's MSRV (1.75). QSL
  says (as relayed) that 1.98 is a workspace default and that QSL-358 phase 1 lowers `quire-exact`'s
  declared version to what it needs (target 1.75). The MSRV row stays open until that lands (see
  Open questions); this requirement fixes no second floor.
- **Nothing is lost.** No requirement, acceptance criterion or test case is deleted by this move.
  A row whose evidence leaves the runtime stays in the matrix as planned, with the reason stated.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-275-AC-1 | With `exact` enabled, the runtime's `exact` module defines no public item that `quire-exact` exports. | Inspection (TC-197) |
| FR-275-AC-2 | No module, alias or feature of the runtime keeps a removed kernel definition reachable. | Inspection (TC-197) |
| FR-275-AC-3 | `quire-exact` is an optional dependency of the runtime, enabled only by the `exact` feature. | Inspection (TC-197) |
| FR-275-AC-4 | The `quire-exact` dependency is a git source on the QSL repository with `branch = "main"` and with no `rev`, `tag`, `path` or committed `[patch]`. | Inspection (TC-197) |
| FR-275-AC-5 | `Cargo.lock` holds exactly one `quire-exact` entry. | Test (TC-197, `make deny`) |
| FR-275-AC-6 | `make deny` exits non-zero when `Cargo.lock` holds a second `quire-exact` entry. | Test (TC-197) |
| FR-275-AC-7 | `deny.toml` carries a `[bans]` `deny` entry for each QSL workspace crate listed under Guarded edges, which includes `qsl-eval`, `qsl-replay` and `quire-spec-language`. | Inspection (TC-198) |
| FR-275-AC-8 | `make deny` reports cargo-deny's `banned` error for the named crate, and exits non-zero, when the runtime's dependency graph, dev dependencies included, contains a crate listed under Guarded edges; a licence failure alone does not satisfy this. | Test (TC-198) |
| FR-275-AC-9 | With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`. Planned and unbacked until QSL-358 phase 1 lowers `quire-exact`'s declared Rust version: the MSRV row stays open (IR-18). | Test (TC-199, `make test-features` row `build-exact-no-std-msrv`) |
| FR-275-AC-10 | The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi` contains no `quire-exact`. | Test (TC-199) |
| FR-275-AC-11 | `make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3's 500 byte to 4 KiB band. | Test (TC-199, `make size`) |
| FR-275-AC-12 | The runtime has no test that compares its output with a second implementation of the kernel or that depends on a QSL crate other than `quire-exact`. | Inspection (TC-197) |
| FR-275-AC-13 | The runtime contains no file copied from the QSL repository and no port of QSL code, where a port is code that keeps the QSL authority's item names and order, except the items on the interim residue list. | Inspection (TC-197) |
| FR-275-AC-14 | The kernel move deletes no requirement, acceptance criterion or test case. | Inspection (TC-197) |
| FR-275-AC-15 | Every matrix row whose evidence leaves the runtime in the kernel move carries a planned status with a stated reason, because the status vocabulary has no "verified upstream" status (a planner question, no status is invented). | Inspection (TC-197) |
| FR-275-AC-16 | After QSL-358 phase 2 is merged, the runtime's `src/exact` directory does not exist and the runtime's `exact` module defines no item. | Inspection (TC-197) |
| FR-275-AC-17 | Until QSL-358 phase 2 is merged, every `exact` item the runtime still defines is named in the interim residue list, which records the expiry condition "QSL-358 phase 2 merged" and the owner's approval status. | Inspection (TC-197) |
| FR-275-AC-18 | The interim residue list is empty when QSL-358 phase 2 is merged. | Inspection (TC-197) |

## Interim residue list (temporary exception)

Expiry condition: QSL-358 phase 2 merged. Owner approval: pending. Owned by QSL-358 and IR-349.
Classified at QSL main against what `quire-exact` exports; IR-349 part 1 re-measures it.

| Residue | Requirement | Source today |
|---|---|---|
| Function application: `PackageDeclarations`, `CheckedPackage`, `Frame`, `Body`, `Evaluation`, `plan_call` | FR-273 | `src/exact/expression.rs` |
| Checking environments: `TypeEnvironment`, composite declarations, `ObjectEnvironment`, `CheckedEquality` | FR-008 | `src/exact/composite.rs`, `reference.rs`, `equality.rs` |
| Containment and unit graphs, enumeration declarations | FR-008, FR-007 | `src/exact/containment.rs`, `unit.rs`, `enumeration.rs` |
| Backend negotiators and their types | FR-009 | `src/exact/division.rs`, `ieee.rs` |
| Carried compiler vocabulary | FR-012 | `src/exact/definition.rs`, `node.rs` |

The list is a classification aid, not a frozen record; an item `quire-exact` exports is not residue
and is deleted in step 1.

## Open questions

- **MSRV of the `exact` profile (open).** `quire-exact` declares `rust-version = "1.98"`; the
  runtime's `compatibility.msrv` is 1.75, and `make msrv` runs `cargo check --all-features` on 1.75,
  which cargo refuses for a dependency that declares more. QSL says (as relayed) that QSL-358 phase 1
  lowers `quire-exact`'s declared version to what it needs, target 1.75. Not measured here (no 1.75
  toolchain installed). This row stays open until QSL-358 phase 1 lands; IR-18 tracks it.
- **Owner approval of the temporary exception (pending).** Step 1 leaves the residue in the runtime
  until QSL-358 phase 2. Peter's approval of that exception, with its expiry "QSL-358 phase 2
  merged", has not been given; QSL-358 and IR-349 own it. Without approval the residue cannot stand
  in the runtime, and step 1 would then wait for phase 2.
- **Upkeep of the ban list (open).** The list under Guarded edges is a snapshot of QSL's workspace at
  8b0c1ffe. The runtime maintainers keep it current until QSL's own lint (QSL-356, QSL #554) is in
  force; whether RT's gate should instead assert on the dependency source is not decided here.
- **Status for rows whose evidence lives upstream (planner question).** The matrix has no "verified
  upstream" status; leaving rows stay "planned" with a reason (FR-275-AC-15).
- **Names of the QSL crate(s) for the residue** are QSL-358's to state; the dependency spelling and
  one-copy rules above apply to each.
- **Kernel shape change.** `quire-exact`'s `Value` is not the runtime's today (it shares through
  `Arc`, carries no object graph, and lacks cross-unit quantity arithmetic and the equality
  conversion table). Adapting the interim residue to it, or moving it to QSL, is code-step work
  (IR-349, QSL-358) and is not decided here.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  [interface-001](../../core/functional/interface-001-runtime-api.md);
  [AD-003](../../assurance/AD-003-codegen-runtime-seam.md) decision F; QSL-357 (merged);
  [NFR-001](../../core/non-functional/NFR-001-no-std-footprint.md).
- **Downstream**: the code steps IR-349: part 1 deletes the `quire-exact` exports and adds the
  `deny.toml` entries; part 2 deletes the residue after QSL-358 phase 2 is merged (as relayed from
  QSL).
- **Routed**: the codegen repository's lock resolves two `quire-exact` copies and three
  `quire-contract-model` revisions; that is codegen's one-copy work (its layout AD, step 1d, the
  codegen owner) and is not a runtime requirement. This requirement covers the runtime only, so
  IR-342 stays open after it merges for the codegen part; the team leader coordinates it. Checked
  read-only at the IR repository's main: it declares no dependency on `quire-exact` and holds no
  copy of the exact kernel (no `Meter`, `ScalarLimits` or kernel `Outcome` definition), so IR needs
  no requirement from this ticket.
