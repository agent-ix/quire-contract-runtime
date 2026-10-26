---
id: SR-35039
title: 'IR-497 gap analysis of RT #161'
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-runtime; src/exact/expression.rs, measurement/exact-panic/src/lib.rs,
  tests/exact_function_application.rs
review_set: subset
---

## Summary

Ticket: IR-497. PR #161 at 48457f5be2963da589a60ecdada758546feb6bb8. Examined FR-273 acceptance criteria and the changed runtime and tests.

## Verdict

**CONDITIONAL** — 3 actionable finding(s).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No test exercises child-call argument validation or its precharge failure | src/exact/expression.rs:779 |
| FND-002 | low | AC-11 tests do not assert the exact child result delivered to the parent | tests/exact_function_application.rs:1154 |
| FND-003 | low | No tagged test has an active child body return a stopped Outcome | src/exact/expression.rs:803 |

## Coverage

Scope examined: FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-4, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7, FR-273-AC-8, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11, FR-273-AC-12, FR-273-AC-13.
Plan completion: not assessed. The computed Quire matrix tags AC-1 through AC-5 and AC-7 through AC-11; AC-6 is an inspection criterion without a symbol; AC-12 and AC-13 remain untagged future leaf and 100,000-call obligations.
