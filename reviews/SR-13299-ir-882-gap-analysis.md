---
id: SR-13299
title: "IR-882 PR 135 gap analysis: RT migration onto quire_exact::Value"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@1f744539a5842f92bd378f2fc2fe85c6c3b37567; spec/exact/matrix/tests.md, spec/tests.md, FR-008, FR-275, FR-277 criteria; tests/exact_composite.rs, tests/exact_collection.rs, tests/exact_equality.rs, src/exact/{composite,collection,equality}.rs"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#135. This is a planless gap analysis.
Plan completion: not assessed. The PR changes no code and no tags. `quoin matrix --json`
(quoin 0.28.3) gives:

- FR-277-AC-1: untagged, no binder. This is correct: the real evaluator still calls
  RT-local `planned_equality` (`src/exact/equality.rs:178`), so no tag may exist.
- FR-008-AC-9: five binders.
- FR-008-AC-10 and AC-11: tagged in `tests/exact_collection.rs:149,203,244`.
- FR-008-AC-12: unbacked, as the matrix states.

No production code is introduced, so there is no orphan code.

The gap: the PR rewrites FR-008-AC-9's subject from RT's `Value` to the kernel's, while the
existing binders still assert RT's `Value`.

## Examined criteria

- FR-277-AC-1 (examined): "... the actual `CheckedEquality::evaluate` path calls `quire_exact::planned_equality_with_plan` and completes `true` ..." — untagged, correct.
- FR-008-AC-9 (examined): "The consumed `quire_exact::Value` has iterative `Debug` and nested-node `Drop` ... RT defines no second `Value`."
- FR-008-AC-10 (examined): tagged by `tc_025` membership-charge tests; status text agrees.
- FR-008-AC-11 (examined): tagged; status text agrees.
- FR-008-AC-12 (examined): unbacked; status text agrees.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | After the AC-9 rewrite, its five binders (`tc_024_p5_value_graph_sharing_and_refusals`, `tc_024_p6_debug_at_depth_has_no_stack_overflow`, `tc_024_p8_debug_exact_string_both_forms`, `tc_025_p6_canonical_key_comparison_at_depth_has_no_stack_overflow`, `tc_025_p8_debug_exact_string_both_forms`) assert RT `quire_contract_runtime::exact::Value`, not the "consumed `quire_exact::Value`" the AC now names. The tags therefore credit a criterion their tests do not assert. Fix: keep AC-9's current subject until the code step rewrites these tests, or treat the bindings as stale and record that in the matrix row (see SR-13298 FND-001). | tests/exact_composite.rs:416,596,701; tests/exact_collection.rs:416,574; spec/exact/functional/FR-008-composite-collection-and-equality.md:170 |

## Verdict

**PASS with one finding.** FR-277-AC-1 is correctly uncredited, and the FR-008-AC-10 to AC-12
statuses match their tags. Within this PR's scope the only gap is that FR-008-AC-9's existing
bindings no longer assert the criterion as reworded.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-008-AC-9 names RT's current `Value` again, so its five binders assert the criterion as worded. The kernel rewrite of those tests is planned in slice 1. |
