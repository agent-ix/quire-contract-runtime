---
id: SR-13121
title: "IR-885 quire-contract-runtime PR 134 gap analysis: FR-008-AC-10/AC-11 backing"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime PR #134 head ee0985f vs main 1a7e12b; FR-008-AC-10, FR-008-AC-11, TC-025, tests/exact_collection.rs, src/exact/collection.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
---

## Summary

Ticket: IR-885 (parent IR-878). Plan completion: not assessed (planless, diff-scoped). `quire coverage --scope . --strict` (quire 0.36.2, engine 0.50.2) reports 90/197 rows backed at head and 89/197 on main. FR-008 goes from 11/13 to 12/13. The only newly backed row is FR-008-AC-11, and there are 0 contradicted statuses at both. FR-008-AC-12 stays unbacked, and the matrix says so.

Clause by clause, against the tagged tests:
- AC-10: `tc_025_p3` asserts member-walk and member-test counts of 4 for set, bag and ordered set, and 0 for sequence. The whole statement is asserted for all four kinds.
- AC-11 sequence: `tc_025_p3` asserts `[2, 1, 2, 3]`, the supplied order. Asserted.
- AC-11 ordered set: `tc_025_p3` asserts `[2, 1, 3]`, and the decimal test asserts the first representation is kept (scales `[1, 0]`). Asserted; M12 dies.
- AC-11 set: `tc_025_p3` asserts `[1, 2, 3]` on integers only. The statement's "one occurrence of each distinct member" is asserted. Which occurrence is kept is not pinned, and the AC does not say.
- AC-11 bag: `tc_025_p3` asserts `[1, 2, 2, 3]`, and the new decimal test asserts per-occurrence representations in authored order among equal keys. That second assertion is the reading FND-001 disputes.

## Reviewed units

- FR-008-AC-10 (examined)
- FR-008-AC-11 (examined)
- FR-008-AC-12 (context_only; unbacked, unchanged)
- FR-008 Behavior lines 67-71 (context_only)
- tests/exact_collection.rs `tc_025_p3_...`, `ordered_set_retains_first_equal_decimal_representation`, `bag_retains_each_decimal_occurrence_in_stable_key_order` (examined)
- src/exact/collection.rs `coalesce` (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-11's bag clause "every occurrence in ascending canonical-key order" does not say whether FR-149-equal occurrences keep their own representations. The authority (FR-144-AC-21: one member of multiplicity 2) and the kernel (quire-exact `coalesce`) read it one way, and this PR reads it the other. AC-11 is now counted as backed by a test that asserts the contested reading, so the backed row records a claim the spec does not settle | spec/exact/functional/FR-008-composite-collection-and-equality.md:161 |

## Verdict

AC-10 is fully backed for all four kinds. AC-11's sequence, set and ordered-set clauses are backed. The bag clause is backed only under a reading of AC-11 that conflicts with FR-144 and the kernel; see SR-13120 FND-001 for the code consequence. The +1 coverage delta (89 to 90) is measured, not claimed. It should stand only after AC-11 is clarified and the bag test asserts the clarified behaviour. No production code lacks an owning requirement, and no stub or coverage inflation was found beyond FND-001.
