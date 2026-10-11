---
id: SR-7140
title: "Code and Rust review of IEEE finite-proof precedence tests"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@cd74fb750805fc4faafc715061674a587ded8614; tests/exact_negotiation.rs; src/exact/ieee.rs; spec/exact/functional/FR-009-i13-backend-negotiation.md; spec/exact/matrix/TC-030-i13-negotiation.md"
review_set: subset
---

## Summary

Ticket: IR-749. Reviewed the one-file PR diff and its public production negotiation path against FR-009-AC-4 and TC-030. The new test checks each exact typed unsupported cause when finite proof is unavailable, while the existing proof-only case checks `RequiresBound`.

## Verdict

**PASS** — The changed test exercises production `negotiate_ieee` and would fail if proof refusal moved before operation, rounding, or exceptional-policy refusal. Width remains covered. The test uses concrete causes, a rounding operation, and no test-only production seam. Focused test, formatting, and Clippy checks passed. The separately reported inherited full-gate failures were not attributed to this diff.

## Coverage

- Examined: FR-009-AC-4, the finite-proof precedence criterion; TC-030 step 6 and expected result; `tests/exact_negotiation.rs:321-403`; `src/exact/ieee.rs:84-110`.
- Context only: FR-009-AC-3, the earlier width/operation/rounding/policy cause order; the surrounding TC-030 tests.
- `cargo test --features exact --test exact_negotiation`: 12 passed.
- `cargo fmt --check`: passed.
- `cargo clippy --features exact --test exact_negotiation -- -D warnings`: passed.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-009-AC-4 | examined | `requires_finite_proof` yields `RequiresBound` only when no `unsupported` cause applies; an item that both lacks a capability and needs an undischargeable proof reports the capability cause. |
| TC-030 step 6 | examined | Negotiate an item that needs a finite proof the backend cannot discharge, alone and combined with a missing capability. |
| FR-009-AC-3 | context_only | IEEE causes are decided in the order width, operation, rounding, exceptional policy, first failure wins: an item failing two checks reports the earlier cause, and a non-rounding operation with an unsupported rounding direction is not `Unsupported(Rounding(..))`. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
