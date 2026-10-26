---
id: SR-35042
title: 'IR-497 EARS review of RT #161'
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-runtime; spec/exact/functional/FR-273-exact-function-application.md,
  spec/exact/matrix/TC-194-function-application.md
review_set: subset
---

## Summary

Ticket: IR-497. PR #161 at 48457f5be2963da589a60ecdada758546feb6bb8. Examined the changed FR-273 statement and TC-194 procedure.

## Verdict

**CONDITIONAL** — 1 actionable finding(s).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The edited statement says Body is replaced although Body remains public as a state factory | spec/exact/functional/FR-273-exact-function-application.md:102 |

## Coverage

Scope examined: FR-273-Behavior, TC-194-step-14.
