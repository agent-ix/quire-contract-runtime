---
id: SR-13298
title: "IR-882 PR 135 integrity analysis: RT migration onto quire_exact::Value"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@1f744539a5842f92bd378f2fc2fe85c6c3b37567; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-008-composite-collection-and-equality.md, FR-275-single-exact-kernel.md, FR-277-checked-equality-evaluation.md, spec/exact/matrix/TC-024, TC-025, TC-026, TC-203, spec/exact/matrix/tests.md, spec/tests.md"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#135.

Checks run:

- `quire validate` on the ten touched files: 10/10 grammar-clean.
- `make spec`: 222/222 grammar-clean; strict coverage reports 123 unbacked rows and 0
  contradicted statuses. The unbacked rows are expected and pre-existing; FR-277 is deliberately
  PLANNED.

Clean:

- No ids are added or reused.
- No hashes or pins are added.
- FR-277-AC-1 has no binder (`quoin matrix`: untagged) and stays PLANNED in FR-277, TC-203
  and both matrix rows.
- The `spec/tests.md` index now lists FR-277/FR-278, matching `exact/matrix/tests.md`.
- The FR-008-AC-10/11/12 status text agrees with the tags in `tests/exact_collection.rs`.
- No conversion layer, bridge, compatibility layer or re-export of the old `Value` is
  introduced. Every touched FR forbids one.

Defects: several edits restate current-state criteria in their post-migration form while
their statuses still read implemented, and one untouched table row in a touched AD still
names an adapter.

## Examined criteria

- FR-008-AC-9 (examined): "The consumed `quire_exact::Value` has iterative `Debug` and nested-node `Drop` ... RT defines no second `Value`."
- exact/matrix/tests.md FR-008-AC-9 row (examined): "✅ implemented"
- TC-024 description (examined): "`ValueGraph`/`build` containment using `quire_exact::Value` directly" with status "✅ RT construction tests remain"
- TC-026 description and step 4 (examined): "`CheckedEquality::evaluate` over `quire_exact::Value`"; "Check a kernel `ObjectReference` carries exactly its supplied `(UniverseId, EffectiveId, ObjectId)` triple"
- FR-008 Behavior reference bullet (examined): "its identity is the supplied `(UniverseId, EffectiveId, ObjectId)` triple"
- AD-004 classification table row (examined): "Same name, different declaration | 22 | delete after the adapter in the step | ... `Value`, `ValueType`, `Quantity` ..."
- AD-004 IR-882 paragraph (examined): "No conversion layer, alias, old-`Value` re-export or second pair traversal is a step-4 output."
- FR-277-AC-1 (examined): "... the actual `CheckedEquality::evaluate` path calls `quire_exact::planned_equality_with_plan` and completes `true`; that kernel call reports 17 planned pair events ... A separate kernel call does not establish this criterion." Verification: Test
- TC-203 step 2 and Status (examined): "Inspect the production evaluator's direct call ..."; "PLANNED after FR-275 kernel routing ..."
- FR-277 / TC-203 matrix rows and spec/tests.md Exact row (examined): consistent.
- TC-025 description (examined): clean.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-008-AC-9 is rewritten to the post-migration state ("The consumed `quire_exact::Value` ... RT defines no second `Value`"), but its matrix row still reads "✅ implemented". Five tests tagged FR-008-AC-9 (`tc_024_p5`, `tc_024_p6`, `tc_024_p8`, `tc_025_p6`, `tc_025_p8`) exercise RT's own `Value` (`src/exact/composite.rs:262`), and that `Value` is exactly the "second `Value`" the AC now forbids. Strict coverage reports no contradiction only because the tags exist. Fix: keep AC-9 describing current behaviour until the code step lands, or mark its row 🚧 pending IR-882 and say the current tags exercise RT's `Value`. Move "no second `Value`" to an FR-275 inspection AC. | spec/exact/functional/FR-008-composite-collection-and-equality.md:170; spec/exact/matrix/tests.md:33 |
| FND-002 | medium | TC-024's description, TC-026's description and step 4, and the FR-008 reference bullet now describe kernel-typed values and the kernel `(UniverseId, EffectiveId, ObjectId)` triple, while TC-024 and TC-026 (FR-008-AC-5/6) still read ✅ implemented. The existing tests use RT's `ObjectReference(UniverseId, NodeKey, ObjectIdentity)` and RT `Value`. Fix: mark these procedure changes as planned for IR-882, or keep the current-tense text and record the rewrite in the plan. | spec/exact/matrix/TC-026-equality-matrix.md:13-14,34; spec/exact/matrix/TC-024-composite-construction.md:14; spec/exact/functional/FR-008-composite-collection-and-equality.md:94 |
| FND-003 | medium | AD-004's classification table still says the 22 same-name/different-declaration items, `Value` and `ValueType` among them, are deleted "after the adapter in the step". The paragraph this PR adds to the same file says no conversion layer is a step-4 output. Fix: change the row to "delete in the step; consumers repoint to the owner type, no adapter". | spec/assurance/AD-004-runtime-crate-layout.md:116 |
| FND-004 | medium | FR-277-AC-1 is verified by "Test", but two of its clauses cannot be observed by a test: that the evaluate path "calls `quire_exact::planned_equality_with_plan`", and that "that kernel call reports 17 planned pair events". `CheckedEquality::evaluate` returns `Outcome<bool>` and exposes no `EqualityEvaluation`. TC-203 step 2 falls back to inspecting the diff. Fix: either split AC-1 into a Test criterion (17 admitted `EqualityPair` charges in schedule order on the real evaluate path) and an Inspection criterion (a direct call, with its relation and plan used), or set the verification to "Test, Inspection (TC-203)". | spec/exact/functional/FR-277-checked-equality-evaluation.md:32; spec/exact/matrix/TC-203-checked-equality-evaluation.md:18 |

## Verdict

**FAIL.** Grammar is clean, no ids are reused, and FR-277 status is correctly kept PLANNED.
FND-001 is a real status overclaim: the matrix marks an AC implemented that the current code
contradicts. FND-002 to FND-004 are consistency and verification-method defects.
