---
id: SR-7141
title: "Gap analysis of IEEE finite-proof precedence evidence"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@cd74fb750805fc4faafc715061674a587ded8614; spec/exact/functional/FR-009-i13-backend-negotiation.md; spec/exact/matrix/TC-030-i13-negotiation.md; tests/exact_negotiation.rs; src/exact/ieee.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: references
---

## Summary

Ticket: IR-749. The computed matrix binds FR-009-AC-4 to the proof-only and combined-capability TC-030 tests, and the changed test supplies the missing operation, rounding, and policy combinations. Source inspection found no reverse gap or hollow assertion in the changed behavior.

## Verdict

**PASS for the reviewed PR scope** — The AC-4 acceptance chain from requirement through TC-030 trace to public production code is covered. This is a subset review of the one-file change, not a repository-wide assurance verdict.

## Coverage

- Examined: FR-009-AC-4; TC-030 step 6 and expected result; the two AC-4-tagged test functions; public `negotiate_ieee`.
- Context only: FR-009-AC-3 and its cause-order tests; FR-009-AC-1, AC-2, and AC-5 matrix entries.
- `quoin matrix --repo . --json`: FR-009-AC-4 is tagged to both AC-4 tests in `tests/exact_negotiation.rs`. AC-1 through AC-4 are tagged. AC-5 is untagged, an inherited compile-time-check matrix state outside this PR's modified evidence.
- No plan was supplied. Plan completion: not assessed.
- Optional repository-wide semantic review: not requested. The changed AC-4 binding was inspected as part of the required scoped PR review.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-009-AC-4 | examined | `requires_finite_proof` yields `RequiresBound` only when no `unsupported` cause applies; an item that both lacks a capability and needs an undischargeable proof reports the capability cause. |
| TC-030 step 6 | examined | Negotiate an item that needs a finite proof the backend cannot discharge, alone and combined with a missing capability. |
| FR-009-AC-3 | context_only | IEEE causes are decided in the order width, operation, rounding, exceptional policy, first failure wins: an item failing two checks reports the earlier cause, and a non-rounding operation with an unsupported rounding direction is not `Unsupported(Rounding(..))`. |
| FR-009-AC-5 | context_only | No `IeeeDisposition` is convertible into an `Outcome` variant, as the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) shows. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
