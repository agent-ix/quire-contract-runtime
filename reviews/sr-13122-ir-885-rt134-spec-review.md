---
id: SR-13122
title: "IR-885 quire-contract-runtime PR 134 spec review: TC-025 step 5 and matrix rows"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime PR #134 head ee0985f vs main 1a7e12b; spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md, spec/exact/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/TC-025
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
---

## Summary

Ticket: IR-885 (parent IR-878). The PR edits spec text in two places. It adds a sentence to TC-025 step 5, and it rewrites the status cells of the FR-008 AC-10/11/12 row and the TC-025 row in `spec/exact/matrix/tests.md`. This review covers those edits and their consistency with FR-008, with the integrity sub-analysis folded in. The edits are structural only, with no new requirement statement, object or relationship. EARS, object, dependency and the other sub-analyses do not apply, and were not run.

## Reviewed units

- TC-025 step 5, new sentence: "Use numerically equal decimals with different scales to check that the ordered set keeps the first representation and the bag keeps each representation in authored order among equal keys." (examined)
- tests.md FR-008 AC-10/11/12 row status (examined)
- tests.md TC-025 row status (examined)
- FR-008-AC-11 and FR-008 Behavior lines 67-71 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-025 step 5 now requires the bag to keep "each representation in authored order among equal keys". That obligation is in neither FR-008-AC-11 nor FR-008's Behavior, which say only that equal occurrences are adjacent. It also conflicts with FR-144, where bag order among equal keys "is not semantic" and equal occurrences are one member with a multiplicity. The test case exceeds its requirement | spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md:44-46 |

## Verdict

The matrix wording is honest about what the tests do: both rows stay 🚧, AC-12 is still named unbacked, and IR-885 is no longer cited as pending. It inherits FND-001, though. "Stable, every-occurrence bag retention" is stated as settled behaviour, and it would need to change with any clarification of AC-11. The ordered-set half of the new TC-025 sentence matches AC-11 ("the first occurrence of each distinct member") and is sound.
