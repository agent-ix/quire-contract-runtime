---
id: SR-35041
title: 'IR-497 spec integrity review of RT #161'
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-runtime; spec/exact/functional/FR-273-exact-function-application.md,
  spec/exact/matrix/TC-194-function-application.md
review_set: subset
---

## Summary

Ticket: IR-497. PR #161 at 48457f5be2963da589a60ecdada758546feb6bb8. Examined the changed FR-273 statement and TC-194 procedure.

## Verdict

**CONDITIONAL** — 2 actionable finding(s).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-194 requires nested-entry refusal without an FR-273 obligation or a defined cause | spec/exact/matrix/TC-194-function-application.md:73 |
| FND-002 | low | Function-application residue inventories still name removed items and omit BodyState and BodyStep | spec/assurance/AD-004-runtime-crate-layout.md:145 |

## Coverage

Scope examined: FR-273-Behavior, TC-194-step-14.
