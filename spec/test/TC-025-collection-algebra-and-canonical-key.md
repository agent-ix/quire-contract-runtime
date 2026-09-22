---
id: TC-025
title: "Construct collections and order them by the canonical key"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: verifies
---
# TC-025: Construct collections and order them by the canonical key

## Description

Check `construct_collection`/`form_collection` charge and refusal ordering, membership-comparison
and bound charges, each of the four kinds' retained and visiting order, and the total canonical key
that fixes set and bag order. Evidence:
`tests/exact_collection.rs` (`--features exact`).

## Test Procedure

1. Construct a sequence, set, bag and ordered set from deferred elements; check `collection.element`
   is charged before each deferred element runs, and that the first non-completing element becomes
   the outcome with no later element run.
2. Call `form_collection` with an occurrence outside the declared element type; check the refusal
   names the offending `Element(index)` before any charge is admitted.
3. Form a set, a bag and an ordered set from occurrences with repeats; check one
   `collection.member-walk` and one `collection.member-test` are charged per comparison against the
   members retained so far, in retention order, stopping at the first equal member. Form a sequence
   from the same occurrences; check neither point is charged at all and every occurrence is
   retained. For all four kinds check `collection.bound` is charged before the uncharged
   bound-violation check — over the member count for the set and ordered set and over the occurrence
   count for the sequence and bag — and that `collection.result-retain` charges the exact retained
   `occ`.
4. Form collections at one below the declared minimum and one above the declared maximum; check both
   `Refusal::CardinalityOutOfBound` directions report `code()` `cardinality_out_of_bound` and
   `cause()` `below-minimum` / `above-maximum` respectively, with the violating kind, bound and
   count.
5. Check each of the four kinds stores its elements in its own canonical order, from occurrences
   whose supplied order differs from ascending canonical-key order: a sequence in supplied
   occurrence order, an ordered set as the first occurrence of each distinct member in
   first-occurrence order, a set as one occurrence of each distinct member in ascending
   canonical-key order, and a bag as every occurrence in ascending canonical-key order with equal
   occurrences adjacent. Check for each kind that this stored order is also the visiting order used
   by equality and further formation, and that the canonical key ranks `Absent < Null < Present`
   across option and composite slots nested inside collection elements.
6. Compare canonical keys of two values nested past typical host recursion limits and check the
   comparison completes with no stack overflow, proving the task-stack walk is iterative.
7. Inject a denial at each of `collection.element`, `collection.member-walk`,
   `collection.member-test`, `collection.bound` and `collection.result-retain`; check each
   `Incomplete` record and that no counter changed.

## Expected Results

Charges precede the work they pay for in the declared order; a type-mismatched occurrence refuses
before any charge; canonical order is total, stable and iterative at depth; both bound-violation
directions are typed and distinct; every injected denial fires with no partial collection, and
`CheckedInvariant` is unreachable from every vector in this test's corpus.
