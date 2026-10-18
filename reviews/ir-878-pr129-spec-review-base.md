---
id: SR-12891
title: "IR-878 RT PR 129 spec-review-base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; PR 129; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md; spec/exact/matrix/TC-016-exact-outcome-envelope.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket IR-878. Reviewed RT PR #129 at df10d63337c5c13ee6f858472a81f4665203b39e against base ecfdfba8049d3804c881f0b8dbb459f44e9dad69. FR-006-AC-1 retirement, owner mapping, TC-016 and matrix consistency.

## Verdict

FAIL

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-016 still instructs a four-disposition test and promises four distinct outcomes although its only remaining mapped criterion FR-006-AC-3 concerns charge vocabulary and accounting. Remove or clearly mark the retired procedure and expected result as owner context so the RT test case is executable for its actual remaining obligation. | spec/exact/matrix/TC-016-exact-outcome-envelope.md:26-38 |

## Checks and limitations

The changed files are spec-only. `quire validate` passed (200/200). `quire coverage --strict` measured 86/184 backed, 109 unbacked, 0 contradicted; `make ci` stops on that existing debt. Owner quire-exact main was inspected read-only. Other Cargo and Kani gates were requested separately with one isolated target and shared lock; see reviewer report for final results.
