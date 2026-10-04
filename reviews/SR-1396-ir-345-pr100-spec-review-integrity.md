---
id: "SR-1396"
title: "IR-345 spec review (integrity): AD-004 against merged specs (PR 100)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-runtime@5a09652802d2e0e13a6f233e9a2ad723d9bba16f; spec/assurance/AD-004-runtime-crate-layout.md, spec/spec.md; read against FR-275, interface-001, AD-002, AD-003"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1396: IR-345 integrity review of PR 100

## Summary

This review checks whether the AD's statement "FR-275 and interface-001 say the opposite" is
complete and honest. A proposed AD may contradict merged text, provided it names every statement
it contradicts and schedules the amendment for step 1. The org rule forbids shims and
compatibility layers, so dropping the `exact` re-export is the right direction. The merged FR-275
already contradicts itself: it requires a re-export, yet also says "no module, re-export alias,
feature or wrapper keeps the old definitions reachable". The AD is honest that it diverges. Its
list of contradicted statements is incomplete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AD lists only some of the merged statements it contradicts. It names FR-275's Description and interface-001-AC-1, AC-2, AC-4 and AC-5. Step 1 adds FR-275 step 2, AC-16 to AC-18, interface-001-AC-7 and AD-002. Still unlisted: (a) FR-275 Outputs ("an `exact` module that ... otherwise only re-exports QSL-owned crates"). (b) FR-275 Behavior, "The end state" ("`exact` re-exports `quire-exact` and `quire-semantic-value`"). (c) FR-275 Behavior, "The residue" ("keeps the `exact` path (interface-001-AC-4)"). (d) interface-001 "Exact kernel surface" prose ("That path is the path the runtime guarantees stays available"; "When an item moves ... it keeps its `exact` path"). (e) The interface-001 contract yaml (`module: quire_contract_runtime::exact`, `runtime_role: re-export unchanged at the same exact path`). (f) interface-001-AC-3 and AC-6, which say "the `exact` module"; that module will not exist, so they become vacuous. (g) If Q-1 is answered as recommended: FR-275-AC-13 and AD-003 decision F (see SR-1398 FND-001). Step 1 is the only spec step, so an incomplete list leaves contradictions merged after step 1. | AD-004 "The end state of the shared kernel", Migration step 1 |
| FND-002 | low | This PR adds AD-004 to the `Exact` row of `spec/spec.md`. That row still describes the end state as "`exact` (to shrink to the runtime-owned backend negotiators ... the residue ... is to be deleted)". AD-004 says there is no `exact` module, puts the kept items in crate-root `scalar` and `expression`, and recommends keeping `expression`. Readers of the registry get two end states. Either qualify the row ("AD-004 proposes ...") or leave the AD out of the row until step 1 amends it. | spec/spec.md Exact row |

## Verdict

**Integrity: one medium, one low.** The AD discloses its conflict and does not shim. But its
step-1 amendment list must name every merged statement it contradicts before IR-349 step 1 can
amend them in one pass.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@c7087f953a24cc49774ed81c5914d5eef0f3fbd0 (fix commits 6a5d672, c7087f9).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6a5d672. Every item (a) to (g) is now listed under "Merged statements this AD contradicts": FR-275 Outputs, "The residue", "The end state", AC-16 and AC-17; the interface-001 prose and yaml (`module`, `runtime_role`, `runtime_owned`, `outside_exact`); AC-1 to AC-6; AC-10. AC-13 and AD-003 F appear under "relies on and must not weaken". Step 1 now amends FR-275 (Description, Outputs, both Behavior bullets, step 2, AC-16 to AC-18) and interface-001 (prose, yaml, AC-1 to AC-7, AC-10), and AD-002. One omission remains: FND-003 below. |
| FND-002 | fixed | 6a5d672. The `spec.md` Exact row now reads "`exact` today; the end state is AD-004 (proposed): no `exact` module, ... keeps the backend negotiators and the lazy connective in `scalar`, and the ported function-application code ... is replaced or deleted per AD-004 Q-1". |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The contradiction list still misses FR-275-AC-1: "With `exact` enabled, the runtime's `exact` module defines no public item that `quire-exact` exports". Once there is no `exact` module, AC-1 is vacuous, just as interface-001-AC-3 is, and AC-1 is the criterion that keeps a kept module (`scalar`, `expression`) from redefining kernel items. Step 1 lists FR-275 "AC-16 to AC-18" but not AC-1. The yaml `residue` block and interface-001-AC-9 ("while a residue `exact` operation exists") also name `exact`, and neither is listed. There is a small internal tension too: AC-18 is listed both under "relies on and must not weaken" and among the ACs step 1 amends ("AC-16 to AC-18"). Add AC-1, the yaml `residue` block and AC-9 to the list and to step 1, and say that AC-18 is kept unchanged. This is a wording fix. | AD-004 "The end state of the shared kernel", Migration step 1; FR-275-AC-1 |

### Dispositions, round 2

Disposition pass 2, reviewed at agent-ix/quire-contract-runtime@5777d53d8d6ea60cc609421e2545a1634f93dc34 (fix commit 5777d53).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 5777d53. On RT origin/main I re-read FR-275 and interface-001 in full against the new lists. The contradicted list now includes FR-275-AC-1, which is restated over the kept modules. It also includes the yaml `residue` and `dependencies` lines, interface-001-AC-9, AC-10, AC-11 and AC-14. AC-14 ("no crate ... other than `quire-exact`") is genuinely contradicted once `quire-semantic-value` is added. AC-11 needs extending, not reversing, and the AD words it as "becomes `quire-exact` and `quire-semantic-value`". AC-18 now "stays" in both places ("step 1 leaves it as written"). AC-13 stays. interface-001-AC-7 is relied on unless Q-1 yields (b), and step 1 says "AC-7 per Q-1". Step 1 amends exactly that set: FR-275 Description, Outputs, both Behavior bullets, step 2, the interim Open question, AC-1, AC-16 and AC-17; interface-001 prose, the yaml blocks, AC-1 to AC-6, AC-9, AC-10, AC-11 and AC-14; AD-002. The other FR-275 ACs (AC-3 to AC-5, AC-10, AC-12, AC-14 and AC-15) are about `quire-exact` or the move. This AD extends them but does not contradict them. |
