---
id: "SR-629"
title: "IR-342 spec review: single exact kernel FR-275 (PR 93)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@4985ce341b742e8c736be535c48493b76f258e16; spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/matrix/TC-198-qsl-crate-edge-guard.md, spec/exact/matrix/TC-199-exact-no-std-and-footprint.md, spec/exact/matrix/tests.md, spec/exact/functional/FR-006..FR-012 and FR-273 (Kernel ownership notes), spec/core/functional/interface-001-runtime-api.md, spec/assurance/AD-003-codegen-runtime-seam.md, spec/spec.md, spec/tests.md; base agent-ix/quire-contract-runtime@9e07f7a21def0970f7f2b1040f4830ea47de7f54; cross-checked read-only against quire-spec-language origin/main 8b0c1ffe"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---

# SR-629: IR-342 spec review of FR-275

## Summary

Ticket: IR-342. PR agent-ix/quire-contract-runtime#93 is spec-only: it adds FR-275 (15 ACs),
TC-197 to TC-199, matrix rows and an "Evidence at the kernel move" table, and adds Kernel
ownership notes to FR-006 to FR-012 and FR-273. Reviewed at head 4985ce3 (9bb9369 plus the
AD-003 doc-fix commit). Checks: integrity, EARS, id allocation, matrix, traceability, and
consistency with interface-001, AD-003 decision F, the code at RT main and `quire-exact` at QSL
main 8b0c1ffe.

## Method

- Diffed the id set (FR, NFR, StR, TC, interface, AC) under `spec/` at base and head: nothing
  removed; added only FR-275, FR-275-AC-1..AC-15, TC-197..TC-199. The diff deletes three table
  rows, each re-added with additions only (interface-001-AC-13, spec.md Exact row, tests.md Exact
  row). The exact matrix diff is additions only (the awk/sed edits removed no row).
- Read `quire-exact/src/lib.rs` and `quire-exact/Cargo.toml` at QSL 8b0c1ffe; grepped QSL for
  `UnitGraph`, `negotiate_`, `CheckedPackage`, `Frame`, containment, `Dimension`, `CompoundUnit`,
  `EnumDeclaration`.
- Mapped every `fn tc_NNN` in `tests/exact_*.rs` and `src/exact_*_tests.rs` to its file and grepped
  those files for runtime-owned types.
- Read `Makefile` (`msrv`, `deny`, `size`), `scripts/run_feature_matrix.py`,
  `scripts/check_one_copy.awk`, `deny.toml`, `measurement/footprint/Cargo.toml`.
- Ran `make spec` at base and head; checked PR 80 and other open PRs and branches for id use.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-275's end state keeps a port of QSL code in the runtime, which AD-003 decision F rules is vendoring. FR-275 forbids only a copy of items `quire-exact` exports; the residue (function application, checking environments, containment and unit graphs, negotiators, compiler vocabulary, ported from QSL `qsl-eval` and `qsl-semantics`) stays, and FR-275's own open question says it "is still a copy of QSL behaviour". AD-003 F says "`exact` is a copy of QSL's kernel, which is vendoring and not allowed to stand ... delete `exact`". FR-273's new note says the port is "not a second kernel", which contradicts FR-275. AC-13 ("no file copied") does not catch a port, so a runtime keeping the whole residue passes every FR-275 AC. The residue has no ticket and no expiry ("needs no change either way"). A planner and owner decision is needed: either (a) the residue is vendoring, so FR-275 names it as a temporary exception with a ticket and an expiry condition (for example, until QSL exports these items from `quire-exact`) and AC-13 covers ports as well as file copies; or (b) the owner rules the residue genuinely runtime-owned (AD-002, interface-001), so AD-003 F is amended to "delete the items `quire-exact` exports" and FR-275 drops "still a copy". | spec/exact/functional/FR-275-single-exact-kernel.md:17-21, spec/exact/functional/FR-275-single-exact-kernel.md:47-52, spec/exact/functional/FR-275-single-exact-kernel.md:107, spec/exact/functional/FR-275-single-exact-kernel.md:120-126, spec/assurance/AD-003-codegen-runtime-seam.md:146-149, spec/exact/functional/FR-273-exact-function-application.md:119-124 |
| FND-002 | high | FR-275-AC-9 cannot pass as verified, and the Toolchain bullet decides an open owner question. AC-9 names the `build-exact-no-std-msrv` row, which builds with `+1.75.0` (`scripts/run_feature_matrix.py:81,93`). Once the runtime depends on `quire-exact` (`rust-version = "1.98"`), cargo refuses that build. The Behavior bullet's "until it is decided the `exact` profile builds on the toolchain `quire-exact` requires" settles the IR-18 question for the interim. It also contradicts merged interface-001-AC-13, which still requires the build at `compatibility.msrv`. Fix: state AC-9 as pending IR-18 the way interface-001-AC-13 does (or drop the row name), and remove the interim toolchain sentence or move it into Open questions. | spec/exact/functional/FR-275-single-exact-kernel.md:84-87, spec/exact/functional/FR-275-single-exact-kernel.md:103, spec/core/functional/interface-001-runtime-api.md:208 |
| FND-003 | high | The "Evidence at the kernel move" table marks tests of runtime-owned items as "leaves", which breaks its own rule ("a test of a runtime-owned item stays"). `tests/exact_meter_state.rs` tests `UnitGraph::admit` and `check_terms` (FR-011-AC-7, lines 561-693). `tests/exact_semantics.rs` builds a `UnitGraph` and runs quantity operations (lines 17, 481-557). `tests/exact_debug_parity.rs` pins `CompoundUnit`, `Dimension` and `EnumDeclaration` Debug output (lines 13, 85-120). `quire-exact` at QSL 8b0c1ffe exports none of `UnitGraph`, `CompoundUnit`, `Dimension`, `EnumDeclaration`, `QuantityOperation` or `evaluate_quantity`. Interface-001 lists the last two as runtime-owned. If IR-349 follows this table, it deletes the evidence for code the runtime keeps. These three files (TC-032, TC-034, TC-035) should split, not leave. | spec/exact/matrix/tests.md:100 |
| FND-004 | medium | The edge guard bans 3 of the QSL repository's 12 non-kernel library crates (the root and 11 `qsl-*`), but the rule it states is "the runtime depends on no other crate of the QSL repository". `qsl-semantics` is where most of the residue comes from. It and `qsl-foundation`, `qsl-package`, `qsl-cst`, `qsl-source`, `qsl-forms`, `qsl-route`, `qsl-attrs` and `qsl-bench` can be added as a dev dependency without failing `make deny`, because `allow-git` admits the whole repository. interface-001-AC-14 covers normal dependencies only, by inspection. Ban every QSL workspace crate other than `quire-exact`, or state why these three are enough. | spec/exact/functional/FR-275-single-exact-kernel.md:62-68, spec/exact/functional/FR-275-single-exact-kernel.md:101-102 |
| FND-005 | medium | FR-275-AC-8 passes for the wrong reason. In TC-198's scratch copy, adding `qsl-eval` (AGPL-3.0-or-later) also fails the `[licenses]` check, which names the crate, because `deny.toml` excepts AGPL only for this workspace's two crates. So "exits non-zero, naming the crate" holds with no `[bans]` entry at all. TC-198 says "names `qsl-eval` as banned" but AC-8 does not. Make AC-8 require the bans diagnostic (cargo-deny's `banned` error for that crate), not any non-zero exit. | spec/exact/functional/FR-275-single-exact-kernel.md:102, spec/exact/matrix/TC-198-qsl-crate-edge-guard.md:21-24 |
| FND-006 | medium | "Planned" is the wrong status for rows whose evidence leaves for QSL. FR-275-AC-15 and the evidence table mark every leaving row (TC-016 to TC-023, TC-025, TC-031, TC-032, TC-034, TC-035) planned, yet the same table says that evidence "belongs to the QSL repository, which this repository does not track". No future RT work will implement them, so ACs of FR-006, FR-007, FR-008, FR-010 and FR-011 become RT requirements with no evidence for good, labelled as pending. Give these rows a status that says they are verified upstream and outside RT's evidence, or re-scope the ACs to the consuming surface. This is a planner question, not a reason to delete rows. | spec/exact/functional/FR-275-single-exact-kernel.md:88-89, spec/exact/functional/FR-275-single-exact-kernel.md:109, spec/exact/matrix/tests.md:85-91 |
| FND-007 | low | The PR adds FR-275 to the Exact row of `spec/tests.md` but keeps "recreation tracked by IR-355 and IR-20". AD-003 F supersedes IR-355, and FR-275-AC-12 forbids an agreement test in the runtime. | spec/tests.md:19 |
| FND-008 | low | "The runtime's only target, `thumbv7em-none-eabi`" is not accurate: the runtime also builds and tests on host targets (`std` feature, `make test`). It means the only `no_std` target. | spec/exact/functional/FR-275-single-exact-kernel.md:78 |
| FND-009 | low | The "leaves" row puts TC-020, TC-021 and TC-022 (in "TC-018 to TC-023") under test files. None of those files has a `tc_020` to `tc_022` test, because those agreement cases were removed earlier and the rows are already planned. | spec/exact/matrix/tests.md:100 |
| FND-010 | low | TC-197 is typed Inspection in the summary, but its procedure runs `make deny` and a negative scratch-copy gate, and FR-275-AC-5 and AC-6 say Test. Split the gate steps into a test case, or type TC-197 to match. | spec/exact/matrix/tests.md:68, spec/exact/matrix/TC-197-single-kernel-ownership.md:24-26 |

## Verdict

**Not mergeable: three high findings.** FND-002 and FND-003 are wording and classification
fixes. FND-001 needs a planner or owner ruling before FR-275 can state the residue's status.

Verified correct (coder claims):

- `quire-exact` at QSL 8b0c1ffe is `#![no_std]` with `extern crate alloc`, uses
  `alloc::sync::Arc`, and QSL `Makefile:193` builds it for `thumbv7em-none-eabi`. That target has
  32-bit atomic compare-and-swap, so `Arc` is available.
- `quire-exact` exports no function application, checking environment, containment or unit graph,
  or negotiator (`negotiate_*` appears only in doc comments). These live in `qsl-eval` and
  `qsl-semantics`.
- QSL's root package name is `quire-spec-language`.
- RT `src/exact` has 23 files and 12,080 lines.
- The footprint crate uses `default-features = false`.
- `make msrv` runs `--all-targets --all-features` on 1.75.
- `make deny` runs `check_one_copy.awk`, which counts `quire-exact` from an agent-ix git source.
- The NFR-001-AC-3 band is 500 B to 4 KiB.
- PR 80 is CLOSED and unmerged, and its branch `rt-spec-id-check-fr` holds FR-274 and TC-196.
  Skipping those ids was not needed but does no harm. No other open PR (78) or branch uses
  FR-275 or TC-197..199.
- The id set lost nothing. interface-001-AC-13 was edited only to correct the `no_std` fact.
  No pins, SHAs or version records were added. FR-275 forbids a compatibility layer.
- The two-floor MSRV recommendation is in Open questions as needing the owner's decision. The
  Behavior bullet in FND-002 is the one place that decides it.
- The AD-003 trailing commit's cites resolve. codegen#50 is the parity-comparator issue, cited by
  QSL ADR-011's §7.1 OBS-040 row. QSL #554 is QSL-356 and is open.
- EARS: FR-275's statement is a well-formed optional-feature ("Where") requirement. `make spec`
  is the same at base and head: the same 5 documents fail structural validation (four matrix
  `Coverage Status` headers, interface-001 `id`/`features`, IR-365), with no new failure.
  Grammar is clean (80/80 at base, 84/84 at head).

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-contract-runtime@1e7f279c8af4473da052466a1834002ca65bb785.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | medium | FR-275 records a QSL commit id (`8b0c1ffe`) twice, as the snapshot the ban list was taken from. The repository's CLAUDE.md forbids introducing SHAs or records that track versions. FR-275's own Dependency spelling bullet says "the runtime records no version, commit or digest of `quire-exact` in its sources or specification", and `8b0c1ffe` is a commit of the repository that holds `quire-exact`. TC-198 step 5 already compares the list with QSL's current workspace, so the commit id guards nothing. Drop it ("QSL's workspace members when this was written"). | spec/exact/functional/FR-275-single-exact-kernel.md:78-79, spec/exact/functional/FR-275-single-exact-kernel.md:86, spec/exact/functional/FR-275-single-exact-kernel.md:171-172 |
| FND-012 | low | The evidence table says `tests/exact_arithmetic.rs` and `tests/exact_allocation.rs` "use only items `quire-exact` exports". `exact_arithmetic.rs` also imports `CHARGE_LOG_CAPACITY` (line 19, used at 1075-1083), which `quire-exact` does not export: its charge log is behind its `test-support` feature, through `Meter::admitted_charges`. The "leaves" disposition still holds, because it is the runtime `Meter`'s internal constant, but the stated reason is not exact. | spec/exact/matrix/tests.md:105 |
| FND-013 | low | AD-002's System Boundary still extends `exact` with "a port of the quire-spec-language authority's own type, carrying its name and order" (`CheckedPackage`). The PR leaves it unchanged, with no pointer to FR-275's temporary exception, so a reader of AD-002 alone sees a standing port. FR-275 cites AD-002 for the residue. | spec/assurance/AD-002-function-application-boundary.md:16-21 |
| FND-014 | low | The expiry condition "QSL-358 phase 2 merged" has no single event to check. QSL-358 is a phase-1 scoping ticket whose description plans phase 2 "split into slices", and phase 2 has no ticket of its own yet. FR-275-AC-16, AC-17 and AC-18 all key on that event. Name the event that ends the exception (for example, the phase-2 ticket, once filed, closed Done), or say the condition is restated when phase 2 is ticketed. | spec/exact/functional/FR-275-single-exact-kernel.md:34-37, spec/exact/functional/FR-275-single-exact-kernel.md:140-142, spec/exact/functional/FR-275-single-exact-kernel.md:146 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1e7f279: the end state is no ported QSL code, delivered in two steps. The residue is a temporary exception with expiry "QSL-358 phase 2 merged", owned by QSL-358 (exists, In Coding) and IR-349, with owner approval stated as pending. AC-13 covers ports. AC-16..AC-18, the interim residue list and TC-197 steps 6 and 7 are added. FR-273, FR-009, interface-001 and AD-003 F no longer call the residue runtime-owned (grep finds "runtime-owned" only in "not runtime-owned"). |
| FND-002 | fixed | 1e7f279: the Toolchain bullet fixes no floor and the MSRV row is open (QSL-358 phase 1, IR-18). AC-9 and TC-199 step 0 say the row is unbacked until QSL-358 phase 1. This is consistent with interface-001-AC-13. |
| FND-003 | fixed | 1e7f279: verified by imports. `exact_outcomes.rs` (TypeEnvironment, CheckedPackage, UnitGraph, EnumDeclaration, evaluate_quantity) and `exact_collection.rs` (TypeEnvironment, CompositeDeclaration) use residue items, and they, `exact_meter_state.rs`, `exact_semantics.rs` and `exact_debug_parity.rs` now split. `exact_allocation.rs` uses only `quire-exact` exports. On `exact_arithmetic.rs`, see FND-012. |
| FND-004 | fixed | 1e7f279: the list holds all 14 non-kernel QSL workspace crates, which match QSL main's members and package names (`quire-spec-language`, 11 `qsl-*`, `xtask`, `arch-lint`). The gap for crates QSL adds later is stated, and TC-198 step 5 checks for drift. |
| FND-005 | fixed | 1e7f279: AC-8 and TC-198 steps 3 and 4 require cargo-deny's `banned` error and say that a licence failure alone does not satisfy them. |
| FND-006 | deferred | Planner question, raised in the PR body and in FR-275's Open questions. The status vocabulary has exactly four markers, ✅ ❌ 🚧 ⛔ (spec-artifacts-process StatusMarker.json; ⛔ = retired), none of which means "verified upstream". Rows stay 🚧 with a note saying RT will never back them. |
| FND-007 | fixed | 1e7f279: spec/tests.md:19 now reads "no agreement test is kept: one kernel, FR-275". |
| FND-008 | fixed | 1e7f279: "the runtime's only `no_std` target". |
| FND-009 | fixed | 1e7f279: the leaving row lists TC-016, TC-017, TC-018, TC-019 and TC-023. TC-020..TC-022 are gone from the file table. |
| FND-010 | fixed | 1e7f279: TC-197 is typed Integration in the summary. |
