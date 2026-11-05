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

The code step for IR-882 moves collection formation, membership comparisons
and canonical keys to `quire-exact`. RT consumes `quire_exact::Value`
directly; no RT-local membership or pair walker is acceptable evidence.

Check `construct_collection`/`form_collection` charge and refusal ordering, membership-comparison
and bound charges, each of the four kinds' retained and visiting order, and the total canonical key
that fixes set and bag order. IR-882 removed RT's duplicate implementation and its former
`tests/exact_collection.rs` controls. `quire-integration` TC-096's
`admitted_model_roots_set_the_reference_universe_and_distinct_owner_result`
restores the moved foreign-reference owner/result control with admitted identities.
The other owner evidence and RT consumer controls need mapping before this
procedure receives full acceptance credit.

## Test Procedure

1. Construct a sequence, set, bag and ordered set from deferred elements; check `collection.element`
   is charged before each deferred element runs, and that the first non-completing element becomes
   the outcome with no later element run.
2. Call `form_collection` with an occurrence outside the declared element type; check the refusal
   names the offending `Element(index)` before any charge is admitted.
   Separately, have a deferred element complete with a value outside its checked type and assert
   `CheckedInvariant { cause: CollectionElementNotAdmitted }`.
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
   canonical-key order, and a bag as every occurrence in ascending canonical-key order by
   multiplicity. Use numerically equal decimals with different scales to check that the
   ordered set keeps the first representation and the bag repeats each distinct member's
   first-supplied representation by its multiplicity. Check for each kind that this stored order
   is also the visiting order used by equality and further formation, and that the canonical key ranks `Absent < Null < Present`
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
`CheckedInvariant` is unreachable from every admitted vector in this test's corpus; the deliberately
invalid deferred element carries `CollectionElementNotAdmitted`.

## Status

PARTIAL: retained under FR-275-AC-14 after removal of RT's duplicate implementation.
The producer-origin foreign-reference control supplies one consumer case;
`tests/exact_collection_consumer.rs` supplies native Decimal controls for all four
kinds' retained coefficient/scale representations, first-supplied coalescence,
per-comparison membership charges, sequence exemption, bound-before-retain order,
and exact retained occurrence charges (including empty input). It also re-forms
an ordered set from each public retained slice, checking order and repeated
occurrences; the caller explicitly enumerates that slice, so this does not
establish equality-planner visiting order.
Additional native controls exercise a late completed-input type mismatch before
any charge even with zero work budget; numeric Decimal coalescence followed by
both cardinality refusal directions with exact kind/bound/count and code/cause;
inclusive count boundaries; and natural bound-work denial before the otherwise
violated cardinality check, with the complete admitted prefix and counters.
Public `construct_collection` callbacks check element-charge-before-run, exact
propagation of supplied undefined/refused/incomplete terminal payloads, a late
non-admitted result's invariant cause, element-work denial
before the callback, and the generous completed control. Later callbacks do not
run on those stops, and completed prefix elements do not coalesce early.
These are partial FR-008-AC-3/7 contributions, not claims for their shared-corpus
or closed-vocabulary clauses. The supplied callback terminal fixtures establish
propagation, not the operations producing those terminal payloads. Remaining
charge-denial prefixes, option/composite canonical keys, deep iteration
and equality visiting order remain planned under IR-499 and their existing
kernel/integration owners. The
identity-dependent integration evidence boundary remains with IR-669. These
controls do not establish whole TC-025 or FR-008-AC-12 acceptance. No deleted RT
test is counted as evidence.
