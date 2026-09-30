---
id: "SR-614"
title: "IR-430 gap analysis: evidence lost when conformance/qsl-agreement left RT"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@2eb0b5299cbf61e1c1cc421580384c82fdb9dfc5; spec/test-matrix.md, spec/test/TC-018..TC-022, spec/test/TC-035, plan/PLAN-003-exact-scalar-oracles, tests/exact_debug_parity.rs, src/exact/mod.rs, conformance/qsl-agreement (removed)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
---

# SR-614: IR-430 gap analysis

## Summary

Ticket: IR-430. PR agent-ix/quire-contract-runtime#88 deletes `conformance/qsl-agreement`, which
held 9 test files. It repoints every spec and plan evidence line at "`tests/…` in
agent-ix/quire-integration". Plan completion: not assessed (planless).

Those files do not exist in agent-ix/quire-integration. Its `main` `tests/` holds only
`integration.rs`. No branch has any `tc_18x`/`tc_19x` or agreement file (the only other branch
with tests, `task/74-c10-c14-spec`, has the `it002`/consumer-view suites). No quire-integration PR
moves them. The tests have been deleted, not moved. The only copy left is RT's git history.

## Method

- I ran `quire coverage --scope . --strict` on the head and on an `origin/main` export, and
  diffed the two.
- I extracted every `TC-`/`FR-…-AC-` tag from each deleted file at `origin/main`, and grepped
  the head's `tests/`, `src/` and `verification/` for each id.
- I read FR-006-AC-2, FR-007-AC-1..6 and FR-273-AC-5, and TC-018..022, TC-024..026 and TC-035.
- I listed quire-integration's tree on every branch with `gh api`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Spec, plan and test docs name evidence files in agent-ix/quire-integration that do not exist there. The agreement tests were deleted, not moved. | spec/test-matrix.md:50, spec/test-matrix.md:107, spec/test-matrix.md:143, spec/test/TC-018-integer-division-agreement.md:14, spec/test/TC-019-decimal-agreement.md:14, spec/test/TC-020-ieee-agreement.md:14, spec/test/TC-021-text-enum-agreement.md:14, spec/test/TC-022-quantity-agreement.md:14, plan/PLAN-003-exact-scalar-oracles/plan.md:19, plan/PLAN-003-exact-scalar-oracles/tasks/Task-003-shared-corpus-agreement.md:16 |
| FND-002 | high | The matrix keeps `✅ implemented` on 7 units that now have no evidence at all: TC-020, TC-021, TC-022, FR-006-AC-2, FR-007-AC-3, FR-007-AC-4 and FR-007-AC-5. `quire coverage` reports "claims ✅ implemented but is not backed" for the three TCs, and backed rows fall from 100/124 to 92/124. | spec/test-matrix.md:25, spec/test-matrix.md:30-32, spec/test-matrix.md:73-75 |
| FND-003 | medium | Three authority-agreement units lost their oracle but still read as backed. FR-007-AC-6 is backed only by a doc comment that disclaims being its oracle. FR-273-AC-5 is backed only by runtime-only tests. TC-018 and TC-019 keep only their allocation-bound step. | spec/test-matrix.md:33, spec/test-matrix.md:50, spec/test-matrix.md:71-72, tests/exact_debug_parity.rs:3, tests/exact_function_application.rs:594, tests/exact_function_application.rs:694 |
| FND-004 | low | The `src/exact` module doc still calls the port "conformance-gated", but after this PR no conformance gate exists in or for this repo. | src/exact/mod.rs:7 |

### FND-001 detail

Fix, either:

- (a) land the move first: a quire-integration PR that adds the 9 files and a Makefile target,
  merged before or with this one, so the pointers are true; or
- (b) keep the deletion and change every pointer to state that the oracle is removed pending a
  named IR ticket. Do not name a path that does not exist.

The plan (step 4) says the tests move to quire-integration, so (a) is the plan-conformant fix.

Context, not a finding against this PR: on `origin/main` the deleted crate already failed to
compile (E0004 at 4 sites; see this repo's `reviews/2026-09-25-ir-286-gap-analysis.md` FND-003).
So the ✅ rows were already backed by tests that could not run. The PR makes that worse. It
removes the tagged symbols, so `quire coverage` now sees the gap. It also adds text claiming the
evidence lives somewhere it does not.

### FND-002 detail

`quire coverage` diff, origin/main → head:

- rows backed: 100/124 → 92/124
- evidence symbols: 304 → 189
- FR-006 file: 5/5 → 4/5
- FR-007 file: 13/13 → 9/13
- test-matrix: 26/26 → 23/26

New "no backing symbol" findings: FR-006-AC-2, FR-007-AC-3, FR-007-AC-4, FR-007-AC-5, TC-020,
TC-021 and TC-022. `git grep` finds none of these 7 ids anywhere in `tests/`, `src/` or
`verification/` at the head.

Fix: set those matrix rows (and the FR-006-AC-2 / FR-007-AC-3..5 rows) to 🚧, citing the IR
ticket that restores the agreement suite in quire-integration.

### FND-003 detail

- **FR-007-AC-6** ("every evaluated shared-corpus vector is executed on both the runtime and the
  quire-spec-language authority with equal Debug renderings"). Its only remaining match is
  `tests/exact_debug_parity.rs:3`, the doc-comment sentence "FR-007-AC-6's own oracle … lives in
  agent-ix/quire-integration". The coverage engine counts it as a `rust-doc-comment-id` binding.
  TC-035 itself says it "is not FR-007-AC-6's oracle".
- **FR-273-AC-5** ("agree with the quire-spec-language authority on every shared-corpus
  function-application vector"). Its remaining tags are on runtime-only tests. The matrix row's
  own prose says the agreement half came from `tc_191_function_application.rs`, which is now
  deleted.
- **TC-018 and TC-019.** Steps 1-3 (census, two-sided evaluation, `Incomplete` agreement) are
  unbacked. Only the allocation-bound step in `tests/exact_allocation.rs` remains.

Fix: mark these rows partial (🚧: agreement half unbacked, with the ticket). Give TC-035's
evidence line a form the coverage engine does not bind to AC-6, for example by dropping the AC id
from the doc comment.

## Verdict

Request changes. The removal is sound as dependency hygiene: nothing else in RT depends on the
crate (see SR-613). But the spec now makes false evidence claims.

Exactly which units lost evidence:

- **Lost evidence entirely:** TC-020, TC-021, TC-022, FR-006-AC-2, FR-007-AC-3, FR-007-AC-4,
  FR-007-AC-5.
- **Lost their authority-agreement half but still read as backed:** FR-007-AC-6, FR-273-AC-5,
  TC-018 (steps 1-3), TC-019 (steps 1-3).

Examined and clean:

- TC-024, TC-025 and TC-026 lost supplementary QSpec-vector files (`tc_188`, `tc_189`,
  `tc_194_equality_matrix`). But their TC docs cite only `tests/exact_composite.rs`,
  `tests/exact_collection.rs` and `tests/exact_equality.rs`, and no FR-008 AC requires authority
  agreement. So FR-008 lost no required evidence.
- `scripts/run_feature_matrix.py` and TC-035's own evidence (`tests/exact_debug_parity.rs`) are
  unaffected.

`quire validate` still reports the same 2 structural failures as origin/main (interface-001 and
test-matrix columns). This PR adds none.
