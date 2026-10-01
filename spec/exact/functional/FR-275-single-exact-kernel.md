---
id: FR-275
title: "Consume quire-exact as the single exact kernel and hold no copy"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---
# FR-275: Consume quire-exact as the single exact kernel and hold no copy

## Description

Where the `exact` feature is enabled, the runtime shall take the exact value kernel from the one
`quire-exact` crate (the QSL repository's kernel crate, QSL ADR-011 X-1) and shall hold no copy,
port or re-implementation of any item `quire-exact` exports. The runtime keeps only the `exact`
items `quire-exact` does not export (interface-001, "Exact kernel surface"), and it keeps them at
the `quire_contract_runtime::exact` path generated oracles already import.

This requirement states the end state of IR-342 (AD-016 owner decision 2) and the rule the code
step IR-349 implements. It follows AD-003 decision F. It adds no behaviour: every behaviour the
kernel owns stays specified by FR-006 to FR-008 and FR-010 to FR-012, now as behaviour of
`quire-exact` that the runtime consumes.

## Inputs

- The `exact` feature of `quire-contract-runtime`, and the `quire-exact` crate it enables.
- The runtime's `Cargo.toml`, `Cargo.lock` and `deny.toml`.

## Outputs

- A build graph in which `quire-exact` resolves once, and the runtime's `exact` module holds only
  runtime-owned items.
- A build that fails when a QSL crate other than `quire-exact` enters the runtime's dependency graph.

## Behavior

- **One kernel.** `quire-exact` is the one owner of values and value types, the kernel `Outcome` and
  its `Undefined`, `Refusal` and `Incomplete` reasons, `Meter` charge-before-work accounting,
  `Origin`/`Location` provenance, `NodeKey`, and the scalar and collection operations over them.
  The runtime defines none of them (interface-001-AC-2, AC-3 and AC-5 state the same rule per item).
  Deleting the runtime's definitions is deletion, not relocation: no module, re-export alias,
  feature or wrapper keeps the old definitions reachable.
- **What the runtime keeps.** Every `exact` item `quire-exact` does not export stays runtime-owned
  and is defined in the runtime's own source, as interface-001 sets out: function application
  (FR-273, AD-002), the static checking environments, backend negotiation (FR-009) and the carried
  compiler vocabulary (FR-012). Those items consume `quire-exact`'s `Value`, `Meter`, `Outcome` and
  `ScalarLimits`; they do not define a second one. An item moves out of the runtime when `quire-exact`
  starts exporting it, and keeps its `exact` path (interface-001-AC-4).
- **Dependency spelling.** The dependency is a git dependency on the QSL repository for the crate
  `quire-exact`, optional, enabled only by the `exact` feature (interface-001-AC-11), and spelled
  `branch = "main"` as every first-party dependency is (IR-434). It carries no `rev`, `tag` or
  `path`, and no committed `[patch]`; local development against a sibling checkout uses the
  untracked `make use-local` patch. The runtime records no version, commit or digest of
  `quire-exact` in its sources or specification.
- **One copy.** `Cargo.lock` holds exactly one `quire-exact` entry (`scripts/check_one_copy.awk`,
  run by `make deny`). `deny.toml` admits that one git source (`allow-git`) and a licence exception
  for the `quire-exact` crate; `unknown-git = "deny"` stays for every other git source.
- **Guarded edges.** The runtime depends on no other crate of the QSL repository. `deny.toml` carries
  a `[bans]` `deny` entry for each of `qsl-eval`, `qsl-replay` and the QSL root crate
  (`quire-spec-language`), so a normal, build or dev dependency on any of them fails `make deny`.
  Each repository guards its own edges, so the guard does not wait for, or rely on, any lint in the
  QSL repository. A dev dependency on `qsl-eval` for conformance is not permitted: it is the same
  edge. The sources policy alone does not catch this, because `allow-git` names a repository, not a
  crate.
- **No substitute for the copy.** The runtime keeps no test that compares its output with a second
  implementation of the kernel, no shared-corpus agreement test, no vendored vector or fixture from
  another repository, and no compatibility layer for the removed copy. With one kernel there is
  nothing to agree with. Tests of kernel behaviour belong to the QSL repository; the runtime keeps
  only tests of behaviour it owns (the dispositions are in the test matrix, "Evidence at the kernel
  move").
- **`no_std` plus `alloc`.** `quire-exact` is `#![no_std]` with `alloc`, and builds for
  `thumbv7em-none-eabi` in the QSL repository's `make ci` (QSL-357). With `exact` enabled and `std`
  disabled the runtime builds for that target. `quire-exact` shares values through `alloc::sync::Arc`,
  which needs the target's atomic compare-and-swap; the runtime's only target, `thumbv7em-none-eabi`,
  has it. The runtime's own `exact` source uses no `std`.
- **Footprint.** The default profile resolves no `quire-exact`: the `quire-contract-runtime-footprint`
  measurement crate depends on the runtime with `default-features = false`, so NFR-001's 500 byte
  floor, 4 KiB ceiling and panic-path check measure the same code as before and `make size` is the
  check. The kernel's size is outside that budget, as the `exact` feature always was.
- **Toolchain.** `quire-exact` declares a Rust version higher than the runtime's MSRV (1.75). The
  default profile and the footprint measurement stay at the runtime's MSRV. Which toolchain the
  `exact` profile builds with is an open owner decision (see Open questions); until it is decided
  the `exact` profile builds on the toolchain `quire-exact` requires.
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
| FR-275-AC-7 | `deny.toml` carries a `[bans]` `deny` entry for each of `qsl-eval`, `qsl-replay` and `quire-spec-language`. | Inspection (TC-198) |
| FR-275-AC-8 | `make deny` exits non-zero, naming the crate, when the runtime's dependency graph, dev dependencies included, contains `qsl-eval`, `qsl-replay` or `quire-spec-language`. | Test (TC-198) |
| FR-275-AC-9 | With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`. | Test (TC-199, `make test-features` row `build-exact-no-std-msrv`) |
| FR-275-AC-10 | The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi` contains no `quire-exact`. | Test (TC-199) |
| FR-275-AC-11 | `make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3's 500 byte to 4 KiB band. | Test (TC-199, `make size`) |
| FR-275-AC-12 | The runtime has no test that compares its output with a second implementation of the kernel or that depends on a QSL crate other than `quire-exact`. | Inspection (TC-197) |
| FR-275-AC-13 | The runtime contains no file copied from the QSL repository. | Inspection (TC-197) |
| FR-275-AC-14 | The kernel move deletes no requirement, acceptance criterion or test case. | Inspection (TC-197) |
| FR-275-AC-15 | Every matrix row whose evidence leaves the runtime in the kernel move carries a planned status with a stated reason. | Inspection (TC-197) |

## Open questions

- **MSRV of the `exact` profile.** `quire-exact` declares `rust-version = "1.98"` and the QSL
  repository builds on 1.98.1. The runtime's `compatibility.msrv` is 1.75, and `make msrv` runs
  `cargo check --all-features` on 1.75, which cargo refuses for a dependency that declares more.
  Either QSL lowers `quire-exact`'s declared version (a QSL decision, not measured here: this
  checkout has no 1.75 toolchain to try) or the runtime states two floors: 1.75 for the default
  profile and the footprint, and `quire-exact`'s for `exact`. The second is the recommendation, and
  needs the owner's decision (IR-18 tracks it); it narrows `make msrv` to the default profile.
- **The runtime-owned residue is a port of QSL code.** `quire-exact` exports no function
  application, checking environment, containment graph, unit graph or backend negotiation; those
  items exist in QSL's `qsl-eval` and `qsl-semantics`, which the runtime may not depend on. The
  runtime therefore keeps them under interface-001's rule, and that residue is still a copy of QSL
  behaviour. Whether QSL moves them into `quire-exact` or the owner accepts them as runtime-owned
  (AD-002 and interface-001 say runtime-owned) is a QSL and owner question; the rule above makes the
  residue shrink as `quire-exact` grows, and needs no change either way.
- **Kernel shape change.** `quire-exact`'s `Value` is not the runtime's today (it shares through
  `Arc`, carries no object graph, and lacks cross-unit quantity arithmetic and the equality
  conversion table). Adapting the runtime-owned residue to it is the code step's work (IR-349) and
  may need QSL additions; it is not decided here.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md);
  [interface-001](../../core/functional/interface-001-runtime-api.md);
  [AD-003](../../assurance/AD-003-codegen-runtime-seam.md) decision F; QSL-357 (merged);
  [NFR-001](../../core/non-functional/NFR-001-no-std-footprint.md).
- **Downstream**: the code step IR-349 (delete the kernel copy and add the `deny.toml` entries).
- **Routed**: the codegen repository's lock resolves two `quire-exact` copies and three
  `quire-contract-model` revisions; that is codegen's one-copy work (its layout AD, step 1d) and is
  not a runtime requirement.
