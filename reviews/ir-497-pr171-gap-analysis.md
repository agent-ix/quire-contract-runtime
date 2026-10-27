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

## Disposition verdict

**PASS** — every finding has a fixed latest disposition at 9490bfe3; no new finding was introduced in disposition pass 2.

## Revalidation (guard-fix pass 3)

Frozen head: 664438839c5476e8d9526ef87f7b182e2212bafa. Previous approved code head: 9490bfe3d9412687d068ee7ea304c1e8b5853b58.

**PASS** — no open or new finding. FR-008-AC-13 and TC-024 remain bound to two active integration tests. The removed ignored child test contributed no independent acceptance evidence; its exercised body moved to an exact-feature Cargo helper. The parent checks child success and exact marker output after the shared deep exercise. No new trace or acceptance gap was found. Final aggregate gates remain with the code lead.
