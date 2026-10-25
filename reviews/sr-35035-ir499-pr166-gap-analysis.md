---
id: SR-35035
title: "gap-analysis of IR-499 FR-006-AC-1 consumer evidence (quire-contract-runtime PR 166)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@ce96ab304819f3bf37abe6ce4c31cc1fc69393d0; tests/exact_outcome_consumer.rs; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md; spec/exact/matrix/TC-016-exact-outcome-envelope.md; spec/exact/matrix/tests.md; base agent-ix/quire-contract-runtime@4e639dcb7f576a029d06a8ec5b5b94e55f34635c"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-016
    type: references
---

# Gap analysis of the IR-499 FR-006-AC-1 consumer evidence (PR 166)

## Summary

Ticket: IR-499. Planless. Plan completion: not assessed. Frozen head ce96ab30 over base 4e639dc.

`quire coverage --scope . --json` (quire 0.36.2, engine 0.50.2) was run at base and at head.
Totals move from 101/215 to 103/215 backed rows; `status_lies` is empty at both. Per-criterion
status: FR-006-AC-1 moves from `untagged` to `tagged` (binder
`consumed_exact_outcomes_keep_false_distinct_from_all_stops`); FR-006-AC-3 stays `untagged`.

Five rows leave the unbacked set at head:

1. FR-006 `verification` row FR-006-AC-1 (targets FR-006-AC-1, TC-016) — correct.
2. tests.md `functional-coverage` row line 13 (FR-006 / FR-006-AC-1, targets FR-006, TC-016) —
   correct.
3. tests.md `traces-to` row TC-016 line 90 (targets FR-006-AC-1, FR-006-AC-3, TC-016) —
   acceptable; its status stays 🚧 and names AC-3 as open.
4. FR-006 `verification` row FR-006-AC-3 (targets FR-006-AC-3, TC-016, TC-017) — false.
5. tests.md `functional-coverage` row line 16 (FR-006 / FR-006-AC-3, targets FR-006, TC-016,
   TC-017) — false.

Rows 4 and 5 are backed only because the new test carries the `TC-016` id and TC-016 also
verifies AC-3. No test exercises any AC-3 clause (spelling round trips, charge-before-work,
high-water/cumulative counters, first short counter).

Counter-experiment (tag reverted after the run; worktree clean): with
`#[trace("FR-006-AC-1")]` alone, rows 4 and 5 stay unbacked (102/215), rows 1 and 3 are backed,
and row 2 becomes unbacked and is reported as a status lie against its new ✅. So dropping
`TC-016` from the tag alone is not enough; the line 13 status or the TC split must change too.

Examined and clean: the test genuinely exercises AC-1's consumer behavior through
`evaluate_boolean_short_circuit` (see SR-35034); no AC text is changed or narrowed (the FR-006
`## Acceptance Criteria` table is untouched); TC-016 stays 🚧 and its AC-3 procedure is
explicitly open; the fixture uses `ForeignCheckedExpression` and no `CallDepthExceeded`; the tag
uses `#[trace]`, no `/// Trace:` line. No stub or unowned production code (no production code
changed).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `TC-016` id in the new test's trace tag makes `quire coverage` report the FR-006-AC-3 verification row and the FR-006-AC-3 matrix row as backed, although no test exercises AC-3. Base listed both as unbacked; head drops them, so the strict unbacked-row count moves toward a pass for a criterion with no evidence. Fix by splitting TC-016 so AC-1 and AC-3 have separate test cases (then tag the AC-1 one), or tag `FR-006-AC-1` only and set the line 13 matrix status so it is not a status lie. | tests/exact_outcome_consumer.rs:123; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:69; spec/exact/matrix/tests.md:16 |

## Verdict

FR-006-AC-1 is genuinely backed. Not mergeable as is: FND-001 (medium) is a coverage
overstatement for FR-006-AC-3 introduced by this diff, which the brief's "strict coverage must
not move to a false pass" rules out. Mergeable once FND-001 is resolved and re-measured.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Repair head 86275e32 (one commit over ce96ab30): the tag is `#[trace("FR-006-AC-1")]` and the tests.md line 13 status is 🚧. Re-measured `quire coverage` at base 4e639dc (101/215 backed, 96 unbacked, 0 status lies) and head (102/215, 94 unbacked, 0 status lies; `--strict` reports 0 contradicted statuses). Only the FR-006-AC-1 verification row and the TC-016 traces-to row become backed; the FR-006-AC-3 verification row (FR-006 line 69) and the tests.md AC-3 rows stay unbacked; FR-006-AC-3 stays untagged. |
