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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | AD-004 leaves a dangling CheckingLimits::depth clause in the still-missing section | spec/assurance/AD-004-runtime-crate-layout.md:291 |

## Dispositions

| Round | ID | Outcome | Evidence |
| --- | --- | --- | --- |
| 1 | FND-001 | fixed | FR-273 Behavior at spec/exact/functional/FR-273-exact-function-application.md:91-92 and AC-8 at :154 require PackageReentry before nested charge; TC-194 step 14 names that cause and its test is tagged AC-8. |
| 1 | FND-002 | still-open | AD-004 now lists BodyState and BodyStep, but spec/core/functional/interface-001-runtime-api.md:194 still lists Body without those two public transition items. |
| 2 | FND-002 | fixed | spec/core/functional/interface-001-runtime-api.md:194 now lists Body, BodyState and BodyStep in function_application, matching src/exact/expression.rs:97-113 and AD-004:145. |
| 2 | FND-003 | fixed | spec/assurance/AD-004-runtime-crate-layout.md:291-295 ends the still-missing QSL leaf sentence, then separately states the removal of CheckingLimits::depth, DepthAboveMaximum and MAX_CALL_DEPTH. |
