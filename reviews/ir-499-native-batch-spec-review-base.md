---
id: SR-20261104024419
title: IR-499 PR202 native batch spec-review-base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime; PR202; tests/exact_collection_consumer.rs;
  tests/snapshot.rs; spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
review_set: subset
---

## Summary

PR202, head branch titan/rt-coverage-classification. Ticket: IR-499. No applicable AssuranceProfile was found in the repository spec tree. No production, dependency, lockfile, CI or proof changes occur in this diff.

## Verdict

PASS within the stated changed scope; no defects found. This is not whole-repository assurance or IR-499 closure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Only TC-025 Status prose changed; requirements, IDs, relationships and test procedures are unchanged. Its new assertions match the new public consumer tests: retained representations, per-comparison schedules, Sequence exemption, bound-before-retain ordering and exact retain occurrences. It explicitly qualifies caller enumeration and denies whole equality-planner/TC acceptance. Existing PARTIAL status and deferred procedures remain truthful. The ownership references preserve IR-669 identity boundaries and kernel/integration responsibilities. No hash, pin, registry, gate weakening or replacement requirement is introduced. Structural/ID/link review is confined to this changed artifact and its existing FR-008 relationship; no whole-repository census was run.

## Examined units

```yaml
scope:
- id: TC-025-status
  path: spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
  role: examined
  excerpt: 'PARTIAL: retained under FR-275-AC-14 after removal of RT''s duplicate
    implementation.

    The producer-origin foreign-reference control supplies one consumer case;

    `tests/exact_collection_consumer.rs` supplies native Decimal controls for all
    four

    kinds'' retained coefficient/scale representations, first-supplied coalescence,

    per-comparison membership charges, sequence exemption, bound-before-retain order,

    and exact retained occurrence charges (including empty input). It also re-forms

    an ordered set from each public retained slice, checking order and repeated

    occurrences; the caller explici'
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
```
