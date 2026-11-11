---
id: SR-2026110705000814
title: "Gap analysis of AGE-1984 FR-008-AC-12 evidence"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; tests/exact_collection_consumer.rs; spec/tests.md; spec/exact/matrix/tests.md; FR-008-AC-10; FR-008-AC-11; FR-008-AC-12"
review_set: subset
---

## Summary

Ticket AGE-1984, PR #221, head b3fb6224b5c6d9b4d0045b256bd928fad1c82ca1. Planless gap analysis of the FR-008-AC-12 binding and the matrix prose that now claims RT consumer evidence for AC-10, AC-11 and AC-12.

## Verdict

**CHANGES REQUESTED** — AC-10 (`coalescence_charges_each_comparison_before_bound_and_retention`) and AC-11 (`retained_decimal_representations_follow_each_collection_kind`) are genuinely backed. The new AC-12 tag binds a test that asserts AC-11 behaviour, so AC-12 remains effectively unbacked, and the edited prose now says otherwise. The prose correctly keeps the 🚧 status and says kernel evidence is open. No local TC row is expected for FR-008 per the lead's decision; not raised. Plan completion: not assessed.

## Examined scope

- FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 (texts as in spec/exact/functional/FR-008-composite-collection-and-equality.md:173-175).
- spec/exact/matrix/tests.md:32 and spec/tests.md:19 status cells.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Trace: `visiting_a_retained_collection_follows_its_stored_order` is tagged FR-008-AC-12 but asserts FR-008-AC-11 stored order; neither equality-planning nor runtime formation visitation is observed. | tests/exact_collection_consumer.rs:159 |
| FND-002 | medium | Matrix prose now claims RT consumer evidence covers AC-12 for all four kinds and drops "visiting order remains unbacked"; with FND-001 that overclaims. | spec/exact/matrix/tests.md:32; spec/tests.md:19 |

## Coverage

FR-008-AC-10 and FR-008-AC-11 have real RT-local consumer tests. FR-008-AC-12 has a tag but no discriminating assertion. A direct observable exists through the public kernel API: form two ordered sets (and two sequences) from the same members in different supplied orders, e.g. [2.0, 1.10] and [1.10, 2.0]; `planned_equality` must return `Completed(false)` because pairs are formed by stored rank, while two sets and two bags from the same inputs return `Completed(true)`. That goes red if planning visits canonical-key order while storage keeps supplied order.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified at 29e351bc: the FR-008-AC-12 tag now binds `equality_planning_pairs_elements_in_stored_order`, which observes equality-planning visitation through public `planned_equality` and goes red under canonical-key visiting (scratch mutation red on Sequence and OrderedSet). |
| FND-002 | fixed | Verified at 29e351bc: prose now scopes AC-12 evidence to `planned_equality`, states set and bag pair by canonical stored order, keeps 🚧 and "remaining kernel evidence is open"; no overclaim found. |

## Disposition verdict (round 1)

**PASS** — both findings fixed at 29e351bc; no new finding. The further-formation clause of FR-008-AC-12 is evidenced only by caller enumeration in the untagged `further_formation_consumes_retained_order_and_repeated_occurrences`, since the kernel exposes no formation over a retained collection; the new prose does not claim it. Plan completion: not assessed.
