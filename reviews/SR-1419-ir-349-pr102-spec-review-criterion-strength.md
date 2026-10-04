---
id: "SR-1419"
title: "IR-349 spec review (criterion strength): rewritten ACs of FR-275 and interface-001 (PR 102)"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-runtime@3524ef89238e11143afd57f6c9eabc6433561f05; FR-275-AC-1, AC-16, AC-17; interface-001-AC-1 to AC-6, AC-10 (rewritten by git diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1419: IR-349 criterion-strength review of PR 102

## Summary

Ticket: IR-349. **Method caveat:** no Jev client is installed (`quoin` exposes none, and the skill
documents the client as not yet built). Every verdict here is the reviewer's own judgment over spec
text, using the skill's `weakness_kind` vocabulary and severity mapping. Treat each one as
unconfirmed by the calibrated lens.

Judged sound, so no finding: FR-275-AC-1, AC-16 and AC-17, and interface-001-AC-2, AC-3, AC-5 and
AC-6. Each is a direct inspection that fails on a concrete artefact.

- FR-275-AC-1 fails on any `pub` item name shared with either crate.
- FR-275-AC-16 fails on an `exact` module, a `pub use` of a shared crate, a non-`scalar` item or
  any file under `src/exact`.
- FR-275-AC-17 fails on an unlisted item, or on a listed exception, expiry or approval.
- interface-001-AC-2 fails on any re-export of the three crates.
- interface-001-AC-3 fails on any `pub` definition of a `quire-exact` name.
- interface-001-AC-5 names its thirteen types exactly.
- interface-001-AC-6 fails on any `NodeKey` constructor call in library source.

The `scalar` items are named exactly in FR-275-AC-16 and in the yaml. The `core_items` rename is
consistent everywhere it is referenced.

FR-275 adverse-case coverage over the three rewritten ACs is good. The ACs state prohibitions, and
each prohibition is its own adverse case: a re-export, an alias, a stray item and a leftover file.
Three findings follow.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface-001-AC-1 is not falsifiable by inspecting this repository; its weakness kind is out of scope. Unconfirmed. Its second half, "generated oracle source and `quire-contract-codegen` shall name the owning crates' own paths ... for every kernel item", is an obligation on CG's emitters, so no RT inspection can fail it. "Kernel item" is defined by the `quire-exact` names, yet the AC also cites `quire_semantic_value::<Name>`, so the domain is unclear. Only the first half, no `exact` module, is a checkable runtime criterion. | interface-001-AC-1 |
| FND-002 | low | interface-001-AC-4 restates another requirement. Unconfirmed. "When an item's owner changes ..., the runtime shall keep no path for that item" adds no failing case beyond interface-001-AC-2 (no re-export by `pub use`, alias, wrapper or feature) and FR-275-AC-2. The "owner changes" event cannot be observed by inspecting a single tree. | interface-001-AC-4 |
| FND-003 | low | interface-001-AC-10 uses an undefined term, a measurability weakness. Unconfirmed. "The runtime shall define every item the `core_items` list names in its core modules": "core modules" is defined nowhere in interface-001, so an inspector cannot decide whether an item defined in, say, `scalar` violates it. Name the modules (`verdict`, `observation`, `accounting`), or say "outside the `exact` feature". | interface-001-AC-10 |

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@8a236d727d89aa32d632bb3d033865501b534db0 (fix commit 8a236d7). These are still reviewer judgment, with no Jev.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8a236d7. interface-001-AC-1 is reduced to "The runtime shall define no `quire_contract_runtime::exact` module", which an RT inspection can fail. The CG obligation sits with AD-004 step 3's paired CG PR, and CG's compile against the RT head fails if an emitter still names the removed path. |
| FND-002 | fixed | 8a236d7. interface-001-AC-4 now reads "The runtime shall expose no item named in the `kernel.consumed` lists ... at any `quire_contract_runtime` path, whether defined or re-exported there". It is a concrete named-list check, covering both definition and re-export at any path, and it fails on a case neither AC-2 (crate-level re-export) nor AC-3 (`quire-exact` names, definitions only) states per item. |
| FND-003 | fixed | 8a236d7. interface-001-AC-10 now names "its `verdict`, `observation` and `accounting` modules". This matches `src/verdict.rs`, `src/observation.rs` and `src/accounting.rs`, which define all six `core_items`. |
