---
id: SR-204003
title: IR-499 PR204 collection refusals spec-review-base
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime; PR204; tests/exact_collection_consumer.rs;
  spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
review_set: subset
---

## Summary

PR204, head branch titan/ir499-collection-refusals; ticket IR-499. Only the native collection test and TC-025 Status changed. No applicable AssuranceProfile exists in the spec tree. No production, dependency, proof, CI or gate-script file changed. No repository edits were made by the reviewer.

## Verdict

PASS within changed scope, zero defects. No whole-repository assurance or IR-499 closure is asserted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reviewed only the changed TC-025 Status against unchanged procedures, FR-008 context and final test source. Existing ID and FR-008 relationship are unchanged. The added status accurately distinguishes completed-input refusal, numeric coalescence cardinality, inclusive endpoints, natural work denial and real deferred callback propagation. Its explicit propagation-only qualification prevents supplied terminal fixtures being credited as proof of the operations producing them. Partial AC3/7 wording excludes shared-corpus and closed-vocabulary obligations. The status retains PARTIAL and names remaining denial, deep/key and equality visiting work. No requirement, registry, hash, pin, relationship or procedure change is introduced. Base structural and linkage review is scoped to the changed artifact; global census was excluded.

## Examined units

```yaml
scope:
- id: FR-008-AC-3
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: '`construct_collection` charges `collection.element` before running each
    deferred element and stops at the first non-completing element with no later element
    run; a completed element outside its declared type returns `CheckedInvariant {
    cause: CollectionElementNotAdmitted }`; `form_collection` refuses a type-mismatched
    occurrence before any charge; formation charges membership comparisons, `collection.bound`
    and `collection.result-retain` in that order, and a set or bag is stored in ascending
    canonical-key order.'
- id: FR-008-AC-7
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and
    `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both
    `CardinalityOutOfBound` directions are reachable and report their `code()` and
    `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared
    corpus.
- id: FR-008-AC-10
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: context_only
  excerpt: When formation coalesces occurrences, the runtime charges one `collection.member-walk`
    and one `collection.member-test` per membership comparison for a set, bag or ordered
    set, and charges neither point for a sequence.
- id: FR-008-AC-11
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: context_only
  excerpt: 'When a collection is retained, the runtime stores a sequence in supplied
    occurrence order, an ordered set as the first occurrence of each distinct member
    in first-occurrence order, a set as one occurrence of each distinct member in
    ascending canonical-key order, and a bag as every occurrence in ascending canonical-key
    order by multiplicity: equal occurrences are one member whose multiplicity is
    their number, and each is represented by that member''s first-supplied value,
    as in ix://agent-ix/quire-specification/FR-144 and FR-144-AC-21.'
- id: FR-008-AC-12
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: context_only
  excerpt: When a retained collection is visited by equality planning or by a further
    formation, the runtime visits its elements in the order it stored them, for every
    one of the four kinds.
- id: TC-025-status
  path: spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
  role: examined
  excerpt: 'These are partial FR-008-AC-3/7 contributions, not claims for their shared-corpus

    or closed-vocabulary clauses. The supplied callback terminal fixtures establish

    propagation, not the operations producing those terminal payloads.'
```
