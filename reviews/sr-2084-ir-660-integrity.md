---
id: SR-2084
title: "Integrity review of the new stack-safety criterion"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@d9ff5e83425403b428f8f91e0d2d3c927b0710fb; spec/exact/functional/FR-008-composite-collection-and-equality.md, spec/exact/matrix/TC-024-composite-construction.md, spec/exact/matrix/tests.md"
review_set: subset
---
 
## Summary

Reviewed the new behavior, criterion, and test step as one change for completeness, consistency, and testability. The obligation covers Option and Collection, the four relevant operations, two stack contexts, equality distinctions and representation preservation without introducing a depth cap.

## Verdict

**PASS** — no contradictory or ambiguous specification requirement was found.

## Scope

- FR-008-AC-13 — role: examined. Excerpt: For an `Option`/`Collection`-nested `ValueType` whose depth exceeds a recursive host-stack walk under the admitted source-size cap, `Clone`, `PartialEq`, `Debug` and `Drop` complete on a small-stack thread and a default-stack child process without host-stack overflow; unequal leaf, collection kind and bound remain unequal, and the shallow compact and alternate `Debug` output matches the derived representation exactly.
- FR-008 Behavior — role: examined. Excerpt: `ValueType`'s `Clone`, `PartialEq`, `Debug` and `Drop` traverse `Option` and `Collection` nesting with heap worklists, never a recursive host-stack walk. The same rule applies when an enclosing declaration or collection type invokes those operations on its `ValueType` member. The declared type keeps its exact equality and debug representation at every depth.
- TC-024 step 7 — role: examined. Excerpt: On a bounded small-stack thread and a default-stack child process, construct an under-source-cap deeply nested `ValueType` through `Option` and `Collection`, then clone, compare, format and drop it. Check a shallow value's compact and alternate `Debug` output against exact literals, and distinguish unequal leaves, collection kinds and bounds.
- FR-008-AC-9 — role: context_only. Excerpt: `Value`'s `Debug` and `Drop` are hand-written and iterative.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
