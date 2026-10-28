---
id: SR-35044
title: "Gap analysis of IR-497 RT consumer control"
type: SpecReview
analysis: gap-analysis
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
| FND-001 | medium | FR-008-AC-13 is tagged, but the default-stack child evidence is hollow when the exact test filter runs zero tests. | tests/exact_deep_value_type.rs:93-108 |

## Coverage

The computed Quoin matrix tags both new test functions to FR-008-AC-13 and TC-024. No run evidence is bound yet; the code lead owns the final gate. The child-process oracle can pass with zero tests, so static tags alone do not establish the criterion. Plan completion: not assessed. The repository-wide matrix contains unrelated gaps outside this PR diff; they were not reclassified here.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The traced child control falsely fails on a one-thread libtest runner because its exact result-line assertion spans uncaptured test output. | tests/exact_deep_value_type.rs:110-113 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at b3e71ed: child test count and completion marker are asserted |
| FND-002 | still-open | With RUST_TEST_THREADS=1, libtest writes the test name before execution and the completion marker before ok; the exact substring is absent although the child exits successfully. |
| FND-002 | fixed | Verified at 9490bfe3: the fragile exact result-line assertion is removed; child success, one-test count and post-exercise marker remain. |
| FND-003 | fixed | At 630866fb, the traced TC-024 exercise adds a pure 100,000-consecutive-Option ValueType and runs its Clone, Eq, Debug and Drop on both required stack sizes; the mixed Option/Collection case remains. |

## Disposition verdict

**PASS** — every finding has a fixed latest disposition at 9490bfe3; no new finding was introduced in disposition pass 2.

## Revalidation (guard-fix pass 3)

Frozen head: 664438839c5476e8d9526ef87f7b182e2212bafa. Previous approved code head: 9490bfe3d9412687d068ee7ea304c1e8b5853b58.

**PASS** — no open or new finding. FR-008-AC-13 and TC-024 remain bound to two active integration tests. The removed ignored child test contributed no independent acceptance evidence; its exercised body moved to an exact-feature Cargo helper. The parent checks child success and exact marker output after the shared deep exercise. No new trace or acceptance gap was found. Final aggregate gates remain with the code lead.

## New findings (revalidation pass 4)

Frozen head: 930fdf8aa6aa3393f7ccd0fda09e02b8c9748581, compared with prior reviewed head 57897a8a5859ccb95780899e282df88dc2d5f86d and base 03d66fc2d24db0da87d4addb0423fb066b0bb343.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | TC-024 now traces FR-008-AC-13 at 100,000 total mixed levels, but the IR-497 ticket's 100,000-deep `Option` target still has no test: this fixture has only 50,000 `Option` wrappers. | tests/support/deep_value_type.rs:6-25 |

## Revalidation verdict (pass 4)

**CONDITIONAL** — FR-008-AC-13's mixed `Option`/`Collection` criterion has active trace tags and non-hollow assertions on both stack sizes, but those tags do not prove the stricter numeric `Option` target in IR-497. The snapshot test is tagged to FR-004-AC-7 and covers root and `counts` malformed nesting on a 64 KiB thread. Plan completion: not assessed.

## Disposition verdict (pass 5)

**PASS** — FND-003 is fixed at 630866fbe3dfe98322582c200cf162f2e69d7ab0. The same traced exercise now includes the ticket's pure 100,000-deep Option target; the existing mixed case and snapshot refusal control remain. Plan completion: not assessed.
