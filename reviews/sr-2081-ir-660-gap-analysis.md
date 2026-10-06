---
id: SR-2081
title: "Gap analysis of FR-008-AC-13 evidence"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@d9ff5e83425403b428f8f91e0d2d3c927b0710fb; spec/exact/functional/FR-008-composite-collection-and-equality.md, spec/exact/matrix/TC-024-composite-construction.md, spec/exact/matrix/tests.md, tests/exact_composite.rs, src/exact/composite.rs"
review_set: subset
---
 
## Summary

The computed matrix tags FR-008-AC-13 to `tc_024_p9_value_type_operations_are_iterative`. The test covers deep Option and mixed Option/Collection work on small and default stacks, but omits a literal oracle for the new Collection debug renderer.

## Verdict

**CONDITIONAL** — one medium evidence gap in a criterion expressly requiring exact derived debug output.

## Scope

- FR-008-AC-13 — role: examined. Excerpt: For an `Option`/`Collection`-nested `ValueType` whose depth exceeds a recursive host-stack walk under the admitted source-size cap, `Clone`, `PartialEq`, `Debug` and `Drop` complete on a small-stack thread and a default-stack child process without host-stack overflow; unequal leaf, collection kind and bound remain unequal, and the shallow compact and alternate `Debug` output matches the derived representation exactly.
- FR-008 Behavior — role: examined. Excerpt: `ValueType`'s `Clone`, `PartialEq`, `Debug` and `Drop` traverse `Option` and `Collection` nesting with heap worklists, never a recursive host-stack walk. The same rule applies when an enclosing declaration or collection type invokes those operations on its `ValueType` member. The declared type keeps its exact equality and debug representation at every depth.
- TC-024 step 7 — role: examined. Excerpt: On a bounded small-stack thread and a default-stack child process, construct an under-source-cap deeply nested `ValueType` through `Option` and `Collection`, then clone, compare, format and drop it. Check a shallow value's compact and alternate `Debug` output against exact literals, and distinguish unequal leaves, collection kinds and bounds.
- FR-008-AC-9 — role: context_only. Excerpt: `Value`'s `Debug` and `Drop` are hand-written and iterative.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Collection Debug has no exact compact or alternate oracle; the new renderer could change a field, bound, separator or indentation while the tagged test stays green | tests/exact_composite.rs:712 |

## Coverage

Plan completion: not assessed. `quire matrix --scope . --format json` reports FR-008-AC-13 tagged by `tests/exact_composite.rs:725`. Its exact string assertions at lines 763–767 exercise only nested Option; the Collection path at lines 712–718 checks only a prefix and leaf substring. The inequality assertions do cover leaf, kind and bound. Reverse code ownership is FR-008-AC-13 for the changed operations. Existing unrelated strict-matrix gaps are outside this PR diff.
