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
- TC-024 step 7: On a bounded small-stack thread and a default-stack child process, construct an under-source-cap deeply nested `ValueType` through `Option` and `Collection`, then clone, compare, format and drop it. Check a shallow value's compact and alternate `Debug` output against exact literals, and distinguish unequal leaves, collection kinds and bounds.
- tests/exact_deep_value_type.rs, all 127 lines at the frozen head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Child exit success can report zero tests, so the default-stack arm can pass without exercising deep type traits. | tests/exact_deep_value_type.rs:93-108 |
| FND-002 | low | An inherited child-marker variable makes the parent skip both the 64 KiB thread and child process. | tests/exact_deep_value_type.rs:80-83 |
| FND-003 | low | The child inherits RUST_MIN_STACK, so its libtest thread may use a configured stack rather than the default stack. | tests/exact_deep_value_type.rs:93-101 |

## Review evidence

Independent semantic review used claude-opus-5-5 on the frozen test, FR-008-AC-13, TC-024 and the pinned quire-exact ValueType trait implementation. The pinned source shows iterative Clone, PartialEq, Debug and Drop. No Cargo gate was run by this reviewer; focused checks in the PR body are claims pending the code lead's final gate. No applicable AssuranceProfile was found under RT spec/.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Single-thread libtest interleaves the completion marker into the exact result-line substring, so a passing child is reported as failed. | tests/exact_deep_value_type.rs:110-113 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at b3e71ed: child test count and completion marker are asserted |
| FND-002 | fixed | Verified at b3e71ed: ambient marker branch removed |
| FND-003 | fixed | Verified at b3e71ed: child removes inherited RUST_MIN_STACK |
| FND-004 | still-open | With RUST_TEST_THREADS=1, libtest writes the test name before execution and the completion marker before ok; the exact substring is absent although the child exits successfully. |
| FND-004 | fixed | Verified at 9490bfe3: the fragile exact result-line assertion is removed; child success, one-test count and post-exercise marker remain. |
| FND-005 | fixed | At 630866fb, `nested_option` wraps Boolean in 100,000 consecutive Option nodes; the shared exercise clones, compares, formats and drops it before the existing mixed fixture on both the 64 KiB thread and child process. The Debug assertion counts all 100,000 Option tokens. |

## Disposition verdict

**PASS** — every finding has a fixed latest disposition at 9490bfe3; no new finding was introduced in disposition pass 2.

## Revalidation (guard-fix pass 3)

Frozen head: 664438839c5476e8d9526ef87f7b182e2212bafa. Previous approved code head: 9490bfe3d9412687d068ee7ea304c1e8b5853b58.

**PASS** — no open or new finding. Cargo declares an exact-only helper binary with test = false; both the integration test and child compile the same tests/support/deep_value_type.rs source. The child executable runs that exercise on its process main thread, exits unsuccessfully on panic or stack failure, and prints only the completion marker after all deep drops. The 64 KiB thread remains the recursion-sensitive control. No code duplication or gate narrowing was found. Final aggregate gates remain with the code lead.

## New findings (revalidation pass 4)

Frozen head: 930fdf8aa6aa3393f7ccd0fda09e02b8c9748581, compared with prior reviewed head 57897a8a5859ccb95780899e282df88dc2d5f86d and base 03d66fc2d24db0da87d4addb0423fb066b0bb343.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The 100,000-level alternating fixture contains only 50,000 `Option` wrappers, so it does not exercise the ticket's explicit 100,000-deep `Option` type target. | tests/support/deep_value_type.rs:6-25 |

## Revalidation verdict (pass 4)

**CONDITIONAL** — the mixed-depth test has a real small-stack and child-process oracle for Clone, Eq, Debug and Drop, and the new snapshot test exercises bounded refusal for malformed nesting at both the root and `counts` value. A pure 100,000-deep `Option` case remains untested. The current snapshot visitor rejects the first `[` before descending, which is the intended closed-shape behavior; a replacement generic recursive JSON parse would fail the 64 KiB test. No gate narrowing was found. The code lead owns the aggregate gate.

## Disposition verdict (pass 5)

**PASS** — FND-005 is fixed at 630866fbe3dfe98322582c200cf162f2e69d7ab0. The pure-Option exercise runs on the small-stack thread and default-stack child; the prior mixed fixture is retained. No new finding. The code lead owns the final aggregate gate.
