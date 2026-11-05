---
id: SR-20261104024418
title: IR-499 PR202 native batch gap-analysis review
type: SpecReview
analysis: gap-analysis
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

Manual scoped acceptance-criteria-to-tests inspection only; full Quoin gap-analysis and fresh matrix/coverage/properties/trace computation were excluded by the user. No repository-wide traceability, reverse-gap or baseline closure is asserted.

FR-008-AC-10 is narrowly and directly discriminated for all four collection kinds by exact alternating charge vectors including six comparisons and zero for Sequence. FR-008-AC-11 is discriminated by representation-sensitive literal tuples, supplied-versus-key-order disagreement, repeated numeric equals and first-supplied representation. These two complete narrow bindings are appropriate. Decimal finite examples are regression evidence, not a universal proof.

The untagged further-formation test observes a partial FR-008-AC-12 clause: caller enumeration cannot demonstrate equality-planner visiting order. The new untagged snapshot tests cover partial FR-004-AC-4/5/9 clauses; they do not claim complete immutable-capture, refusal, or wire-order acceptance alone. Existing wire-order control remains. TC-025 remains PARTIAL and retains deferred elements, refusal, bounded violations, denial, nested keys and deep iteration work. FR-008-AC-3/4, whole AC-12 and whole TC-025 are not credited. IR-669 borrowed identity/source evidence and deferred IR-590 removals remain excluded. No numerical coverage gain is asserted.

Plan completion: not assessed

## Examined units

```yaml
scope:
- id: FR-008-AC-10
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: When formation coalesces occurrences, the runtime charges one `collection.member-walk`
    and one `collection.member-test` per membership comparison for a set, bag or ordered
    set, and charges neither point for a sequence.
- id: FR-008-AC-11
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: 'When a collection is retained, the runtime stores a sequence in supplied
    occurrence order, an ordered set as the first occurrence of each distinct member
    in first-occurrence order, a set as one occurrence of each distinct member in
    ascending canonical-key order, and a bag as every occurrence in ascending canonical-key
    order by multiplicity: equal occurrences are one member whose multiplicity is
    their number, and each is represented by that member''s first-supplied value,
    as in ix://agent-ix/quire-specification/FR-144 and FR-144-AC-21.'
- id: FR-008-AC-12
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: When a retained collection is visited by equality planning or by a further
    formation, the runtime visits its elements in the order it stored them, for every
    one of the four kinds.
- id: FR-004-AC-4
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Allocation-free immutable snapshots retain exact identity and all counters
    after subsequent report recording; imported snapshots cannot resume a mutable
    report.
- id: FR-004-AC-5
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: JSON transport round-trips real mixed reports and refuses independently
    authored malformed, incomplete, duplicate, unsupported or inconsistent data.
- id: FR-004-AC-9
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: examined
  excerpt: Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`,
    `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in
    that order inside `counts`; byte-for-byte comparison of encoded output observes
    this order.
- id: FR-004-AC-10
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: context_only
  excerpt: An unknown top-level or `counts` member is refused at its key before its
    value is read. In `src/snapshot_json.rs`, the unknown-key arm of `FieldVisitor::visit_str`
    must return an error before `SnapshotSeed` or `CountsSeed` can call `next_value`
    or `next_value_seed` for that member; neither visitor may consume an unknown value
    with `IgnoredAny` or an equivalent skip/decode path.
- id: FR-004-AC-11
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: context_only
  excerpt: The decoder validates each member without materializing an intermediate
    generic JSON value tree.
- id: FR-004-AC-12
  path: spec/accounting/functional/FR-004-campaign-accounting.md
  role: context_only
  excerpt: Parser scratch on the `decode_campaign_snapshot` path remains bounded proportionally
    to the input cap of `MAX_BYTES` (65536 bytes), including escaped-string scratch
    and owned decoded identities; no input can cause scratch allocation to grow without
    a bound derived from that cap.
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
```
