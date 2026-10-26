---
id: SR-35043
title: "Code review of IR-497 RT consumer control"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; tests/exact_deep_value_type.rs; FR-008-AC-13; TC-024"
review_set: subset
---

## Summary

Ticket IR-497, PR #171, frozen code head b08897598693c085a737c7d43788b2ad17b52d1b. Reviewed the new RT consumer test against FR-008-AC-13 and TC-024.

## Verdict

**CONDITIONAL** — the default-stack child can report success without executing the test. The small-stack thread, deep positive and negative comparisons, and shallow Debug literals have real assertions.

## Examined scope

- FR-008-AC-13: For an `Option`/`Collection`-nested `ValueType` whose depth exceeds a recursive host-stack walk under the admitted source-size cap, `Clone`, `PartialEq`, `Debug` and `Drop` complete on a small-stack thread and a default-stack child process without host-stack overflow; unequal leaf, collection kind and bound remain unequal, and the shallow compact and alternate `Debug` output matches the derived representation exactly.
- TC-024 step 7: On a bounded small-stack thread and a default-stack child process, construct an under-source-cap
- tests/exact_deep_value_type.rs, all 127 lines at the frozen head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Child exit success can report zero tests, so the default-stack arm can pass without exercising deep type traits. | tests/exact_deep_value_type.rs:93-108 |
| FND-002 | low | An inherited child-marker variable makes the parent skip both the 64 KiB thread and child process. | tests/exact_deep_value_type.rs:80-83 |
| FND-003 | low | The child inherits RUST_MIN_STACK, so its libtest thread may use a configured stack rather than the default stack. | tests/exact_deep_value_type.rs:93-101 |

## Review evidence

Independent semantic review used claude-opus-5-5 on the frozen test, FR-008-AC-13, TC-024 and the pinned quire-exact ValueType trait implementation. The pinned source shows iterative Clone, PartialEq, Debug and Drop. No Cargo gate was run by this reviewer; focused checks in the PR body are claims pending the code lead's final gate. No applicable AssuranceProfile was found under RT spec/.
