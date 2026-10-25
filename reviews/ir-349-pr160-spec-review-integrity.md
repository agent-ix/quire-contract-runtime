---
id: SR-35020
title: "Integrity review of IR-349 RT scalar move (quire-contract-runtime PR 160)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@3f6ef45d; spec/core/functional/interface-001-runtime-api.md; spec/spec.md; spec/exact/matrix/tests.md; spec/exact/functional/FR-009-i13-backend-negotiation.md; README.md; src/lib.rs (docs)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# Integrity review of the IR-349 RT scalar move (PR 160)

## Summary

Ticket: IR-349. This checks whether the move's prose is applied consistently across the repository at head 3f6ef45d. Consistent inside the diff: the interface-001 yaml comment, FR-009-AC-5, the three spec/exact/matrix/tests.md path references, the README module table, the `src/lib.rs` Compatibility paragraph and the `src/operators.rs` doc all name `scalar`. The PR leaves one contradicting sentence in the same file it edits, and one weakened summary.

Examined: interface-001 "Exact kernel surface" prose (lines 113-125), interface-001 `runtime_owned` and `not_authorized` yaml (lines 189-211), the spec.md Exact row (line 73), and spec/exact/matrix/tests.md lines 54, 76 and 183.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface-001 prose still says the runtime's "own kept items are in a `scalar` module, which does not exist yet". The PR creates that module, and in the same file the yaml now reads "implemented: the 11 runtime-owned items live here". The prose contradicts the yaml and the code. | spec/core/functional/interface-001-runtime-api.md:118 |
| FND-002 | low | The spec.md Exact row rewrite drops "the ported function-application code is not authorized and is replaced or deleted per AD-004 Q-1". The new text calls it "function-application residue for IR-1009". The status is still stated in interface-001 (`status: not authorized`), but the architecture overview no longer says the residue is unauthorized, so it reads as a sanctioned interim. | spec/spec.md:73 |

## Verdict

Changes requested: one sentence in interface-001 to fix, and optionally restore the "not authorized" qualifier in spec.md. No AC or matrix AC/TC list changed.
