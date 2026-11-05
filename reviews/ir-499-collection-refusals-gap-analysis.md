---
id: SR-204002
title: IR-499 PR204 collection refusals gap-analysis
type: SpecReview
analysis: gap-analysis
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

Manual changed-scope acceptance-criteria-to-tests check only. User excluded global matrix/coverage/trace/properties recomputation, whole-repository census and Cargo reruns. This is not a completed end-to-end Quoin repository gap audit. Plan completion: not assessed.

The five new tests supply real partial FR-008-AC-3/7 evidence: first-stopped callback propagation, charge-before-run, non-admitted result cause, input validation precedence, cardinality result/code/cause, exact admitted prefixes, atomic denial and completed controls. Callback Refused/Incomplete fixtures establish unchanged propagation only; actual bound and element denials independently exercise production terminal generation. No fake whole-row closure is inferred from either category.

Existing AC10/11 tags and representation-sensitive tests remain intact. TC-025 stays PARTIAL. Closed vocabulary, shared-corpus invariant unreachability, remaining charge-denial points, nested canonical keys, deep iteration and equality-planner visitation are not established. IR-669 identity-dependent source evidence is unchanged and IR-499 remains open. No metric gain or imported whole-file acceptance is claimed.

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
