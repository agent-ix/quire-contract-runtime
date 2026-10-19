---
id: SR-13081
title: "IR-878 quire-contract-runtime PR 133 gap analysis: FR-008 AC-10/AC-11 backing"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime PR #133 vs main; tests/exact_collection.rs, spec/exact/matrix/tests.md, src/exact/collection.rs (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: reviews
---

## Summary

Ticket: IR-878. This is a planless, diff-scoped audit. Plan completion: not assessed. The spec-review sub-analyses were skipped because no requirement or acceptance-criterion text changed. The only spec edit is one matrix status cell.

`quire coverage --scope . --strict` (quire 0.36.2, engine 0.50.2) gives:
- origin/main: 88/197 rows backed.
- PR head: 90/197 rows backed.
- 0 contradicted statuses at both.

Diffing the per-criterion status shows exactly two changes: FR-008-AC-10 and FR-008-AC-11 move from untagged to tagged. FR-008-AC-12 stays untagged, which is deliberate.

Twelve hand mutants were applied to `src/exact/collection.rs`, then `cargo test --all-features --test exact_collection` was run. Each mutant was reverted after its run.

| Mutant | Result | Killed by |
| --- | --- | --- |
| M1: drop the member-test charge | killed | p3, p7 |
| M2: drop the member-walk charge | killed | p3, p7 |
| M3: charge comparisons for a sequence while keeping its order | killed | p3 only; new coverage |
| M4: store a set in insertion order | killed | p3, p5, p6 |
| M5: leave a bag unsorted | killed | p3 only |
| M6: sort an ordered set | killed | p3 only |
| M7: sort a sequence | killed | p3 only |
| M8: ordered set keeps duplicates | killed | p3 only |
| M9: bag dedupes | killed | p3 only |
| M10: ordered set in last-occurrence order | killed | p3 only |
| M11: no early stop at the first equal member, giving 5 comparisons | killed | p3 |
| M12: ordered set keeps a later equal occurrence's value in the first position | SURVIVED | none |

M12 is not an equivalent mutant. Membership equality for `Decimal` is `numerically_equal`, while `Decimal` retains its pre-normalized representation. A reviewer throwaway probe with ordered-set occurrences `[1.0 (10,1), 2, 1.00 (100,2)]` shows the difference:
- The probe passed on PR head, with retained scales `[1, 0]`.
- It failed under M12, with scales `[2, 0]`.
- `tc_025_p3` stayed green under M12.

The same probe on a bag returned scales `[1, 1, 0]`. The second occurrence, `1.00`, is not retained; the first occurrence is repeated instead. See FND-002.

## Reviewed units

- FR-008-AC-10 (examined): When formation coalesces occurrences, the runtime charges one `collection.member-walk` and one `collection.member-test` per membership comparison for a set, bag or ordered set, and charges neither point for a sequence.
- FR-008-AC-11 (examined): When a collection is retained, the runtime stores a sequence in supplied occurrence order, an ordered set as the first occurrence of each distinct member in first-occurrence order, a set as one occurrence of each distinct member in ascending canonical-key order, and a bag as every occurrence in ascending canonical-key order.
- FR-008-AC-12 (examined): When a retained collection is visited by equality planning or by a further formation, the runtime visits its elements in the order it stored them, for every one of the four kinds.
- FR-008-AC-3 (context_only): formation charges membership comparisons, `collection.bound` and `collection.result-retain` in that order, and a set or bag is stored in ascending canonical-key order.
- spec/exact/matrix/tests.md FR-008 AC-10/11/12 row (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-008-AC-11's ordered-set clause, "the first occurrence of each distinct member", is not asserted. The fixture uses only `Integer` values, and equal integers cannot be told apart. A mutant that keeps a later equal occurrence's value in the first position (M12) survives `tc_025_p3`. The mutant is observable: with `Decimal` 1.0 (10,1) and 1.00 (100,2), membership-equal but retained with distinct scales, a probe kills it. The matrix row's "AC-11 retained order now tested for all four kinds" claims more than the test asserts. Add a representation-distinct decimal case asserting that the ordered set retains the first occurrence's representation. | tests/exact_collection.rs:149-197; src/exact/collection.rs:247-275 |
| FND-002 | medium | FR-008-AC-11's bag clause, "a bag as every occurrence", is tagged as backed, but the implementation does not retain every occurrence. `coalesce` keeps `(member, multiplicity)` and emits `repeat_n(member, multiplicity)`, the first occurrence repeated. A probe on bag occurrences `[1.0 (10,1), 2, 1.00 (100,2)]` stores scales `[1, 1, 0]`, so the `1.00` occurrence is lost. The code predates this PR. The new AC-11 tag still claims backing for a clause the code violates under the provenance-retaining reading of spec lines 68-71. The fix is either to retain each occurrence in the code, or to clarify that equal occurrences are interchangeable, then assert that clause in the test. A fixture of only equal integers cannot detect the gap. | src/exact/collection.rs:247-275; spec/exact/functional/FR-008-composite-collection-and-equality.md:68-71,161 |

## Verdict

AC-10 is fully backed. All four kinds are asserted, including "neither point for a sequence" (M3 killed, new). The per-comparison counting is exact (M1, M2 and M11 killed).

AC-11 is backed for the order clause of all four kinds (M4-M10 killed). It is not backed for which occurrence is retained when members are equal (FND-001). The bag "every occurrence" clause is violated by the implementation for representation-bearing decimals (FND-002).

The matrix row's AC-12 wording, "AC-12 visiting order remains unbacked", is honest, and the coverage output agrees: AC-12 is untagged and there is no status contradiction.

The coverage delta is exactly +2 (AC-10 and AC-11), with 0 contradicted statuses.

Not mergeable as-is. FND-001 needs a test case in this PR. FND-002 needs a fix, or a spec clarification plus test, before the AC-11 bag tag is justified; it could instead be deferred to a ticket, with the AC-11 tag withheld until then.

## Round 1 verification

The updated `tests/exact_collection.rs` includes a decimal ordered-set case with membership-equal values of different scales. The focused suite passes 10/10. Reapplying M12 makes only the new targeted case fail: observed scales `[2, 0]` versus expected `[1, 0]`. The source was restored after the mutant run. No production code changed. The FR-008-AC-11 trace tag was removed from the existing test; both the existing ordering test and new decimal test remain untagged for AC-11 pending IR-885. The matrix now states that AC-10 is tested for all four kinds, AC-11 ordering and ordered-set first occurrence are tested, the bag every-occurrence clause is pending IR-885, and AC-12 is unbacked. `quire coverage --scope . --strict` reports 89/197 backed, exactly +1 over main's 88/197 (AC-10 only), with 0 contradicted statuses. Existing unrelated strict-coverage findings remain.

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | The decimal ordered-set assertion retains scales `[1, 0]` and kills M12. |
| FND-002 | deferred: IR-885 | The bag every-occurrence clause is a pre-existing spec/code issue. AC-11 stays untagged, and the matrix states the gap. |
