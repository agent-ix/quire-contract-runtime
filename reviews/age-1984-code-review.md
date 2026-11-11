---
id: SR-2026110705000813
title: "Code review of AGE-1984 FR-008-AC-12 visiting-order test"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; tests/exact_collection_consumer.rs; spec/tests.md; spec/exact/matrix/tests.md; FR-008-AC-12"
review_set: subset
---

## Summary

Ticket AGE-1984, PR #221, head b3fb6224b5c6d9b4d0045b256bd928fad1c82ca1 against base 6214fdb650db834f52581c98ec470f3162e206db. Code review with the rust-review lane folded in, over the new test `visiting_a_retained_collection_follows_its_stored_order` and two matrix prose cells. The focused test binary `exact_collection_consumer` passes (14 of 14) with `--features exact`.

## Verdict

**CHANGES REQUESTED** — the new test cannot fail independently of FR-008-AC-11. Its "further formation" step is the test itself copying `collection.elements()` into a `Vec` and forming a sequence, which by AC-11 preserves supplied order whatever any runtime visitor does. The equality-planning half of AC-12 is never exercised, although `quire_exact::planned_equality` is public and pairs elements by stored rank. Rust idiom is clean: let-else destructuring, reuse of `form` and `assert_representations`, no unwrap outside test fixtures that matters.

## Examined scope

- FR-008-AC-12: When a retained collection is visited by equality planning or by a further formation, the runtime visits its elements in the order it stored them, for every one of the four kinds.
- FR-008-AC-11 (context only): stored order per collection kind.
- tests/exact_collection_consumer.rs lines 159-210 (new test), and the existing untagged `further_formation_consumes_retained_order_and_repeated_occurrences`.
- quire-exact 5db2d5e `src/equality.rs` `planned_equality` / `plan_pairs_with_relation` collection arm (context only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-008-AC-12 test has no oracle for visiting order: it re-asserts AC-11 stored order and a caller-side copy into a sequence, so it stays green if equality planning or formation visited in canonical-key order. | tests/exact_collection_consumer.rs:159-210 |
| FND-002 | low | The trailing ordered-set block re-forms the same collection to re-prove a key inequality already established by the opening assertion and the ordered-set expectation. | tests/exact_collection_consumer.rs:196-210 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 29e351bc: `equality_planning_pairs_elements_in_stored_order` forms each kind from descending and ascending inputs and asserts `planned_equality` gives `Completed(false)` for sequence and ordered set and `Completed(true)` for set and bag. Scratch mutation (left formed from canonically sorted input) went red on Sequence, then on OrderedSet. |
| FND-002 | fixed | Verified at 29e351bc: the redundant trailing ordered-set re-formation block is removed. |

## Disposition verdict (round 1)

**PASS** — both findings fixed at 29e351bc; no new finding. Focused binary `exact_collection_consumer` passes 14 of 14. A same-order positive control for the ordered kinds would also rule out a planner that always answers false for them; this is optional hardening, not a defect.
