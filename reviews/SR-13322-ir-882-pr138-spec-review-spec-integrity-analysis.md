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
