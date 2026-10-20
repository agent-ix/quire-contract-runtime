---
id: SR-13322
title: "IR-882 PR 138 integrity analysis: FR-275 QSV ownership directive"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@96c1519ae6a6b149696b168f8d6c2e3b9c1d1a55 vs origin/main de63b6b4; spec/exact/functional/FR-275-single-exact-kernel.md, spec/core/functional/interface-001-runtime-api.md, spec/assurance/AD-004-runtime-crate-layout.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: reviews
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#138. This is a structural and consistency check of the amended text. It covers where each statement sits, whether a criterion can enforce it, and whether neighbouring statements still agree with it. `make spec` is grammar-clean (228/228) with coverage unchanged at 124 unbacked rows.

## Examined criteria

- FR-275 "Shared identity and declaration owners" bullet, lines 217-224 (examined): "RT consumes `quire-semantic-value`'s `UnitTable::get(UnitId)` ... The IR-882 migration repoints quantity, enum and reference payloads to those owner types directly, with no RT-owned `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, second value representation or adapter."
- FR-275 section headings (examined): the bullet sits under `## Open questions` (line 198), not `## Behavior` (line 81).
- FR-275-AC-2 (examined): "No module, alias or feature of the runtime keeps a removed kernel definition reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, pair walker or membership walker and no conversion layer or old-`Value` re-export."
- interface-001 lines 147-150 (examined), under `## Exact kernel surface`; interface-001 AC-1 to AC-14 (context_only).
- AD-004 line 412 (examined): "What stays in step 4 is the rest: `Quantity`, `EnumValue`, `NodeKey`, object identity and the `EffectiveId` registry change."
- AD-004 rows 87, 89, 90, 116 (context_only): map `unit`, `enumeration`, `TypeEnvironment` to quire-semantic-value, "no adapter".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new ownership rule is a settled directive, but it sits under FR-275 `## Open questions`, not `## Behavior`. No acceptance criterion enforces it: FR-275-AC-2 names only a second `Value`, pair walker, membership walker, conversion layer and re-export, and interface-001's paragraph has no AC either. A re-submission of closed PR #137's `KernelIdentityRegistry` would therefore pass every AC. This carries SR-13314 FND-004 ("no AC states the registry") forward. Move the bullet to Behavior and extend FR-275-AC-2, or add an AC, to cover "no RT-owned `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry or adapter" | spec/exact/functional/FR-275-single-exact-kernel.md:159,198,217-224; spec/core/functional/interface-001-runtime-api.md:147-150 |
| FND-002 | low | AD-004 line 412 still names "the `EffectiveId` registry change" as step-4 work. Under the amended FR-275, RT has no `EffectiveId` registry and repoints to `TypeEnvironment::object_type(EffectiveId)`. The phrase can be read as RT building one. Say "repointing object identity to `TypeEnvironment::object_type`" | spec/assurance/AD-004-runtime-crate-layout.md:412 |

## Verdict

**FAIL: one medium finding.** The FR-275, interface-001 and AD-004 rows 87, 89, 90 and 116 now agree on ownership, and the closed PR #137 conflict is gone. What remains is that the rule is in the wrong section and has no AC that would enforce it (FND-001), plus a stale phrase (FND-002).

## Dispositions

Review pass only; no disposition round has run yet.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | FR-012-AC-1 says "its ID stays reserved and is never reused", yet the same ID now carries a different criterion. The old one was a constructor Test (TC-033); the new one is a cutover Inspection. This reuses the ID in substance and contradicts its own sentence. Keep FR-012-AC-1's original text, marked retired at IR-882 step 4, and state the cutover inspection under a new ID (for example FR-012-AC-6) | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:98,108-110 |
| FND-004 | low | interface-001-AC-5 now bundles two obligations: the existing per-type "defines none of" list and a new rule to use the QSV owners with no registry or adapter. The second duplicates FR-275-AC-2's new clause, so a reviewer cannot mark AC-5 met while only one half holds. Put the QSV clause in its own interface-001 AC, or rely on FR-275-AC-2 | spec/core/functional/interface-001-runtime-api.md:246 |

Round 1, verified against fix commit a9b45d77 on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The bullet moved to FR-275 `## Behavior`. FR-275-AC-2 now names the `UnitId` resolver, `EnumShape` and `EffectiveId` registries, QSV owner copies and adapters, and TC-197 step 1 inspects them. The FR-275 and TC-197 matrix rows say the clause is planned and untagged |
| FND-002 | fixed | AD-004 line 412 now reads "repointing object identity to QSV's `TypeEnvironment::object_type(EffectiveId)`" |
| FND-003 | still-open | New this round; awaiting a fix round |
| FND-004 | still-open | New this round; awaiting a fix round |

Round 2, verified against fix commit 36cb2cee on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | FR-012-AC-1 again carries its original criterion text verbatim, plus "Retired at IR-882 step 4; this ID remains reserved with its original criterion text". The cutover inspection is a new FR-012-AC-6, which was unused on main and at the head before this commit. AD-004 step 4, the Q-5 row, FR-012 "Kernel ownership" and both matrix tables name AC-6 consistently |
| FND-004 | fixed | interface-001-AC-5 is restored to its original per-type wording. The QSV-owner, no-registry obligation lives only in FR-275-AC-2 (and interface-001's prose paragraph). The core matrix row says FR-275-AC-2 owns it |
