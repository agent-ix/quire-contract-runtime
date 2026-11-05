---
id: SR-206004
title: IR-499 PR206 collection denials spec-review-evidence
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-runtime; PR206; tests/exact_collection_consumer.rs;
  spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
review_set: subset
---

## Summary

PR206, head branch titan/ir499-collection-denials; ticket IR-499. Scope is the five added tests and TC-025 Status. No applicable AssuranceProfile exists in the repository spec tree. Production, dependencies, proofs, CI, gate scripts and the disjoint kernel lane are untouched.

## Verdict

PASS within changed scope, zero defects. No whole-repository assurance or IR-499 closure is asserted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Manual scoped evidence qualification as requested. The deterministic advisor was excluded because it computes fresh coverage/properties; no catalog-wide audit is claimed. Test is the unchanged authored verification method for the examined clauses. The fixtures execute real public kernel formation, key comparison and Meter injection. Independently authored numeric schedules, payloads, call counts and projected literal retained members provide the primary oracle; generous charge records add richer prefix discrimination without replacing those expectations.

Inspection of supplied qualification-logs/ir499-collection-denials shows consumer.log: thirteen tests passed; clippy.log: successful focused warnings-denied check; fmt.log: clean; quire-validate.log: changed-TC structural validation without errors. These logs are supplied focused run evidence, not reviewer executions or the final lead gate. Natural result-unit denial specifically checks an earlier scanned size request was not committed. No Inspection/compile/SCA, vocabulary census, corpus agreement, composite-slot Null or depth qualification is inferred. No Cargo, Kani, memory-pressure or global scans ran here. Root owns the final scoped gate following artifact custody.

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
- id: FR-008-AC-4
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: The type-owned canonical key totally orders every keyed type, ranks `Absent
    < Null < Present`, fixes set and bag canonical and visiting order, and is evaluated
    iteratively at a depth that would overflow a recursive host-stack walk.
- id: FR-008-AC-8
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: Every one of the twelve added charge points round-trips its QSpec spelling,
    `ChargePoint::ALL` has exactly 63 members including `temporal.visit`, and an injected
    denial at each of the twelve yields `Incomplete` on `work_units` naming that point
    with every counter left unchanged.
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
  excerpt: These are partial FR-008-AC-3/8 contributions, not twelve-point vocabulary
    or shared-corpus acceptance. Native option controls check None-before-Present
    and integer payload order for all ordered pairs, directly and inside singleton
    sequences; outer set/bag formation checks literal retained order and multiplicities.
```
