---
id: SR-204001
title: IR-499 PR204 collection refusals code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-runtime; PR204; tests/exact_collection_consumer.rs;
  spec/exact/matrix/TC-025-collection-algebra-and-canonical-key.md
review_set: subset
---

## Summary

PR204, head branch titan/ir499-collection-refusals; ticket IR-499. Only the native collection test and TC-025 Status changed. No applicable AssuranceProfile exists in the spec tree. No production, dependency, proof, CI or gate-script file changed. No repository edits were made by the reviewer.

## Verdict

PASS within changed scope, zero defects. No whole-repository assurance or IR-499 closure is asserted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust lane included. Focused existing-helper searches in exact tests/support and exact test files found no equivalent refusal schedule/terminal assertion helper; existing ChargePoints support is reused. The changes exercise actual public form_collection/construct_collection with owned Decimal values, without duplicate implementation, identities or QSL agreement claims. The new helpers only author expected schedules and compare public effects.

The late type mismatch has two invalid values but demands first Element(3), empty charges/admission count and every consumed counter zero at both zero and generous work budgets. The impossible [0,0] bound prevents a later cardinality result from masking validation precedence. The [3,3] cardinality control independently distinguishes two numeric members from five occurrences, requires each typed payload and distinct code/cause, and demands the full membership-then-bound prefix with no retain. Exact [count,count] controls show inclusive endpoints and the retained declared type.

Six scalar comparisons independently consume 6*(2+1)=18 work units. Bound refusal consumes one additional work unit; bound-work denial at 18 (or zero for Sequence) leaves the exact prior prefix/counters and reports the precise Incomplete fields. ValueOccurrences is high-water: scalar comparisons retain 1; failed bound charges leave it unchanged. ResultUnits remains zero until retention. Completed construction adds five element work units, then bound and retain, yielding Sequence work 7 and coalescing work 25, with occurrence/result amounts 6 or 3.

The real Deferred callbacks assert the element charge precedes each invocation and that no membership charge occurs before all elements complete. Distinct supplied Undefined, Refused cardinality and Incomplete payloads pass through exact assert_stopped variant/payload comparisons; these fixtures prove propagation rather than the producing operations. The late Boolean result instead requires CollectionElementNotAdmitted. A fourth callback panics if reached; calls and exact admitted element prefix are asserted after each stop. Natural element-work denial at two units prevents the third callback; the generous control reaches all five callbacks then formation. These deterministic fixtures use tiny bounded inputs, no unsafe/async/concurrent behavior, and retain the existing AC10/11 attributes. Partial new controls deliberately have no whole-AC trace attributes, per the authorized scope.

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
- id: FR-008-AC-7
  path: spec/exact/functional/FR-008-composite-collection-and-equality.md
  role: examined
  excerpt: The extended `Undefined` and `Refusal` vocabularies, `BoundViolation` and
    `Refusal::cause()` are closed, typed and distinct from every FR-006 variant; both
    `CardinalityOutOfBound` directions are reachable and report their `code()` and
    `cause()`; `CheckedInvariant` is unreachable from any admitted vector in the shared
    corpus.
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
  excerpt: 'These are partial FR-008-AC-3/7 contributions, not claims for their shared-corpus

    or closed-vocabulary clauses. The supplied callback terminal fixtures establish

    propagation, not the operations producing those terminal payloads.'
```
