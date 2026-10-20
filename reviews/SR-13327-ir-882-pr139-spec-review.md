---
id: SR-13327
title: "IR-882 PR 139 spec review: FR-012 key-owner cutover wording and matrix rows"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@22e885d76b7c1bca523ed79f0129a102da6557cc vs origin/main 9567ff0; spec/exact/functional/FR-012-carried-compiler-vocabulary.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: reviews
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#139. The spec diff is 11 added lines across two files:

- FR-012 Description and Behavior now say `NodeKey` and `NODE_KEY_DOMAIN` come from `quire_exact`.
- FR-012 Behavior and Kernel ownership say the constructors "were deleted ... in the IR-882 key-owner slice of step 4".
- The FR-012-AC-1 matrix row is marked retired.
- The FR-012-AC-6 row is backed by TC-033.
- The TC-033 row drops AC-1 and adds AC-6.

`quire validate` is grammar-clean on both files. No AC row text changed: AC-1's original text and ID are kept verbatim, and AC-6 is unchanged.

The reviewer checked the new wording against AD-004 step 4 and Q-5 option A, and against QSL ADR-012 §13.4 item 2 and §13.5 as merged by QSL PR #1070. They agree. RT holds shared keys and constructs none in library code, and test-only fixtures may use `from_digest`. AD-004's split rule permits a key-owner slice. FR-012's "in the current runtime values" correctly avoids claiming the `quire_exact::Value` hold that AD-004 Q-5 describes as the end state.

Sub-analyses were not run. The edits are wording and matrix-status changes to one FR, with no new requirement statement, structure or relationship edge, so `spec-ears-analysis`, `spec-integrity-analysis` and `spec-dependency-analysis` have nothing in scope.

## Examined criteria

- FR-012 Description (examined): "`src/exact/mod.rs` exports the remaining runtime vocabulary; `NodeKey` and `NODE_KEY_DOMAIN` are consumed directly from `quire_exact`."
- FR-012 Behavior, first bullet (examined): "The runtime never computes or constructs a node key in library code. It carries admitted `quire_exact::NodeKey` values in the current runtime values. The local `NodeKey::from_bytes` and `NodeKey::from_hex` constructors were deleted ..."
- FR-012 Outputs (examined): "`NodeKey` values and the `NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` digest/evaluator domain strings."
- FR-012 Kernel ownership (examined): "... except for the local-constructor contract in FR-012-AC-1, retired in the IR-882 key-owner slice of step 4."
- FR-012-AC-1 (examined): "... Retired at IR-882 step 4; this ID remains reserved with its original criterion text."
- FR-012-AC-6 (examined): "At the IR-882 step-4 cutover, inspection finds no RT-local `NodeKey` ... RT library code has no `NodeKey` construction or hex decoder."
- spec/exact/matrix/tests.md rows FR-012-AC-1, FR-012-AC-6, TC-033 (examined).
- AD-004 step 4 and Q-5 (context_only); QSL ADR-012 §13.4 item 2 and §13.5 "how RT obtains `NodeKey`s" (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-012 Outputs still lists "`NodeKey` values and the `NODE_KEY_DOMAIN` ... domain strings" as runtime outputs. After this PR RT neither defines nor exports either one, and the Description was amended to say both are consumed from `quire_exact`. Outputs and Description now disagree | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:36-37 |

## Verdict

**PASS with one low finding.** The amended wording is accurate, consistent with AD-004 and the merged QSL ADR-012 rows, and keeps the retired AC-1 record intact. The stale TC-033 procedure is reported as SR-13326 FND-001, because it is a document this PR did not touch.

## Dispositions

Review pass only; no disposition round has run yet.
