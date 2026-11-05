---
id: SR-20261104024417
title: IR-499 PR202 native batch code-review review
type: SpecReview
analysis: code-review
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

The tests call the actual public quire_exact::form_collection on owned Decimal values, consistent with FR-008 and IR-882 consumer ownership. They add no value implementation, walker, QSL dependency or minted identity. Existing ChargePoints support is reused. Focused searches of exact tests found no existing representation assertion helper or equivalent Decimal collection control; generic limit fixtures are ordinary bounded test setup.

Input A=2.0, B=1.10, B=1.1, A=2, B=1.100 makes canonical sorting disagree with supplied order and numeric equality disagree with representation equality. Literal coefficient/scale assertions discriminate Sequence original, OrderedSet [A,B], Set [B,A], Bag [B,B,B,A,A], including first-supplied representation. Counts 0,1,2,1,2 independently yield six alternating member-walk/member-test pairs; Sequence has zero. Retain amounts 6/3/3/6 and empty amount 1 are explicit. Re-formation has comparisons 6/1/1/5 and orders AB/AB/BA/BA. It tests public slice consumption, not equality-planner visitation.

Snapshot import uses complete independently authored wires, surrogate-pair decoding, literal expected UTF-8, preserved revision 007, exact counters and NFC/NFD inequality. Encoder escaping uses serde_json as the independent semantic oracle, not RT decode or a copied codec, and permits valid escape spellings. The existing byte-exact member-order test is retained. All loops and strings are small deterministic fixtures. No unsafe, async, network or memory-pressure work is added. Partial tests deliberately carry explanatory comments rather than whole-criterion tags; the caller expressly required that scope distinction.

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
