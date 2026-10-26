---
id: SR-35038
title: 'IR-497 code review of RT #161'
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-runtime; src/exact/expression.rs, measurement/exact-panic/src/lib.rs,
  tests/exact_function_application.rs
review_set: subset
---

## Summary

Ticket: IR-497. PR #161 at 48457f5be2963da589a60ecdada758546feb6bb8. Examined FR-273 acceptance criteria and the changed runtime and tests.

## Verdict

**FAIL** — 2 actionable finding(s).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The exact-panic probe still constructs the removed two-argument Body closure, so its crate cannot compile | measurement/exact-panic/src/lib.rs:92 |
| FND-002 | medium | Nested package entry reports GeneratedUnexpectedOutcome, which describes a different adapter fault | src/exact/expression.rs:704 |

## Coverage

Scope examined: FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-4, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7, FR-273-AC-8, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11, FR-273-AC-12, FR-273-AC-13.
Rust gates were not run by this reviewer because another process was running focused tests in the same worktree. Static inspection traced the exact-panic probe compile error.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The Kani census comment still names the removed Frame::call API | verification/kani.rs:20 |

## Dispositions

| Round | ID | Outcome | Evidence |
| --- | --- | --- | --- |
| 1 | FND-001 | fixed | measurement/exact-panic/src/lib.rs:92-96 now builds a Body factory returning a state whose transition returns BodyStep::Return(Outcome::Completed(Value::Boolean(true))); the exact-panic probe builds with panic_references=0. |
| 1 | FND-002 | fixed | src/exact/expression.rs:712,745 now return CheckedInvariantCause::PackageReentry on failed begin(); tests/exact_function_application.rs:1657-1663 asserts this cause and zero nested work units. |
| 2 | FND-003 | fixed | verification/kani.rs:19-21 now describes checked package application through CheckedPackage::call; no live Kani comment names removed Frame::call. |
