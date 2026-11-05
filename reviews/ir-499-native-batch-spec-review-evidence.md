---
id: SR-20261104024420
title: IR-499 PR202 native batch spec-review-evidence review
type: SpecReview
analysis: evidence
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

Manual evidence qualification confined to TC-025 Status and the changed tests, as expressly requested. The deterministic advisor was not run because it derives fresh coverage/properties measurements forbidden for this invocation; this is not a completed catalog-wide advisor audit.

The inspected FR-008 AC10/11 obligations are authored Test and have concrete public-API tests with independently derived representation/schedule expectations. Their assertions can fail on changed coalescence, sort order, first representative, comparison counts or Sequence exemption. Further formation is partial AC12 evidence only. Snapshot identity and escaping controls are useful partial executable transport evidence; they neither discharge FR-004 AC10/11/12 Inspection obligations nor qualify compile/SCA channels. No imported whole partial file receives acceptance credit. PLAT-1160 delivery does not by itself qualify these remaining channels.

Existing local logs show rustfmt, scoped warnings-denied Clippy, three collection tests, three identity-filtered snapshot tests, retained exact-order test and TC structure validation succeeded. These are supplied focused execution evidence, independently inspected here rather than re-run; they are not this reviewer's aggregate gate. Root owns the one final scoped gate after artifact custody. Kani, high-memory families, feature sweeps and additional measurements were excluded and were not executed.

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
