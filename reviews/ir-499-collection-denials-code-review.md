---
id: SR-206001
title: IR-499 PR206 collection denials code-review
type: SpecReview
analysis: code-review
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

Rust lane included. Existing charge prefix, input, counter assertion and stopped-outcome helpers are reused. Focused helper searches in collection tests and tests/support found no existing direct/nested option projection helper or equivalent construct_decimal_input. Added helpers construct bounded public API fixtures and inspect output; they implement no collection semantics. No identity, QSL, or copied kernel implementation is introduced.

Late natural membership denials independently specify two completed comparisons at work 6, then refuse member-walk requesting 2; work 8 admits that walk and refuses member-test requesting 1. Literal alternating prefixes, exact Incomplete fields, admission count and every counter are checked for all three coalescing kinds.

Retention work denial occurs after work 1 for Sequence or 19 otherwise; result-unit denial occurs at zero results with ample work. Both preserve prior bound count 5 or 2 rather than raising size high-water to retained occurrence count 6 or 3. Exact Incomplete variant excludes any partial completed value, and all counters are checked. Thus a partial size/work commit on later result failure is discriminated.

Injected controls exercise actual public Meter test support on the third element, third walk, third test, bound and result-retain. The independently enumerated generous schedule has five element charges and six alternating membership pairs, then bound/retain; total work 7 or 25. Expected injected prefix lengths 2/9/10/17/18 and work 2/11/13/23/24 for coalescing kinds are arithmetic constants rather than computed from production output. Sequence skips impossible membership points and uses bound prefix 5, work 5. Each denied result requires exact payload, every counter, admission count and callback count. Comparison with the successful full charge-record prefix is supplementary, so shared drift in generous and denied schedules cannot alone pass.

The nine-entry literal key-order matrix for None, Present(-1), Present(2) includes reverse and equal pairs, directly and nested in singleton sequences. It does not use compare_keys to derive expected output. Outer Set and Bag compare literal retained option payloads and multiplicities, checking outer length, nested kind/length and Integer payload type. The test uses public production formation for nested inputs; independent direct-key and literal projection assertions prevent formation serving as its own ordering oracle. No composite Null/Absent or deep-stack claim is made. Tiny fixed loops and values add no memory-pressure, async or unsafe behavior. Existing AC10/11 trace attributes remain unchanged; new partial tests carry no whole-criterion tags.

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
