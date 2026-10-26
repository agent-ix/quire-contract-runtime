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
- TC-024 step 7: On a bounded small-stack thread and a default-stack child process, construct an under-source-cap
- tests/exact_deep_value_type.rs, all 127 lines at the frozen head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-13 is tagged, but the default-stack child evidence is hollow when the exact test filter runs zero tests. | tests/exact_deep_value_type.rs:93-108 |

## Coverage

The computed Quoin matrix tags both new test functions to FR-008-AC-13 and TC-024. No run evidence is bound yet; the code lead owns the final gate. The child-process oracle can pass with zero tests, so static tags alone do not establish the criterion. Plan completion: not assessed. The repository-wide matrix contains unrelated gaps outside this PR diff; they were not reclassified here.
