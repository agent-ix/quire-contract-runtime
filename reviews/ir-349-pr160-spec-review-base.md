---
id: SR-35019
title: "spec-review of IR-349 RT scalar move (quire-contract-runtime PR 160)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@3f6ef45d; spec/core/functional/interface-001-runtime-api.md; spec/exact/functional/FR-009-i13-backend-negotiation.md; spec/exact/matrix/tests.md; spec/spec.md; spec/core/non-functional/NFR-002-panic-compatibility-license.md (context); spec/assurance/AD-004-runtime-crate-layout.md (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# Spec review of the IR-349 RT scalar move (PR 160)

## Summary

Ticket: IR-349. Frozen head 3f6ef45d. The spec diff makes path updates in FR-009-AC-5, in three spec/exact/matrix/tests.md rows and in Evidence Locations. It also changes the interface-001 `runtime_owned.module` comment, rewrites the spec.md Exact row, and amends interface-001 `compatibility.enums` to "the five runtime-owned scalar enums are exhaustive". AD-004 Q-4 and step 3 call for that amendment. `quire validate` passes, and `make spec` coverage is unchanged from main.

Examined: interface-001 `compatibility.enums`, interface-001-AC-8, FR-009-AC-5, the NFR-002 statement and NFR-002-AC-3, the AD-004 Q-4 row and the AD-004 step 3 row.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR makes the five scalar enums exhaustive, and interface-001 now says so. NFR-002 was not amended. Its statement requires "non-exhaustive public data enums where downstream exhaustive matching would impede evolution", and NFR-002-AC-3 says "Public data enums whose evolution affects downstream matching remain non-exhaustive". The dispositions are matched downstream: the removed test comment cited exactly this NFR-002 rule as its reason. NFR-002 and interface-001 now disagree on these five enums. Scope NFR-002 to the core and residue enums, or name the AD-004 Q-4 exception, in this PR. | spec/core/non-functional/NFR-002-panic-compatibility-license.md:16,45 |

## Verdict

Changes requested: one medium consistency amendment. No AC is narrowed or deleted. The FR-009-AC-5 edit changes only the evidence path.

## Dispositions

Round 1, checked at head e3f2a07889fd3dbe0a536ac69fec989e7d66c387.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The NFR-002 statement adds "The five runtime-owned scalar enums are closed under AD-004 Q-4 and exhaustive; core and remaining expression-residue data enums retain their non-exhaustive compatibility contract." NFR-002-AC-3 is now scoped to "Public core and remaining expression-residue data enums" and states the scalar exception. The scope reduction is exactly the AD-004 Q-4 owner exception, and the observation.rs/verdict.rs doctest evidence is unchanged, so no AC is narrowed beyond what Q-4 directs. |
