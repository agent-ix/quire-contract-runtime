---
id: SR-12892
title: "IR-878 RT PR 129 spec-review-scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; PR 129; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md; spec/exact/matrix/TC-016-exact-outcome-envelope.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket IR-878. Reviewed RT PR #129 at df10d63337c5c13ee6f858472a81f4665203b39e against base ecfdfba8049d3804c881f0b8dbb459f44e9dad69. Allocation of RT exact consumption and quire-exact ownership, checked against owner FR-362, FR-090, FR-368 and public types.

## Verdict

FAIL

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new paragraph assigns all of retired FR-006-AC-1, including closed refusal, undefined, and incomplete reasons, to quire-exact FR-362-AC-11. That criterion only tests scalar outcome stops, Completed(false), and admitted prefixes; it contains no closed-reason census. Refusal and Undefined are enums, but Incomplete is a struct carrying LimitKind and ChargePoint. Identify the actual owner criteria for the closed typed reasons or state precisely which old clause was superseded; do not claim FR-362-AC-11 verifies it. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:106-115 |

## Checks and limitations

The changed files are spec-only. `quire validate` passed (200/200). `quire coverage --strict` measured 86/184 backed, 109 unbacked, 0 contradicted; `make ci` stops on that existing debt. Owner quire-exact main was inspected read-only. Other Cargo and Kani gates were requested separately with one isolated target and shared lock; see reviewer report for final results.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 402699c, FR-006-AC-1 remains open. The text limits owner FR-362-AC-11 to scalar stop dispositions, explicitly says it does not census all closed reasons, and correctly calls Incomplete a typed struct with closed LimitKind and ChargePoint enums. |
