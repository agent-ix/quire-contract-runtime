---
id: SR-2026110705000807
title: "Integrity review of RT #209 bounded monitor state and budget boundary"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; spec/monitor/functional/FR-276-consume-timed-monitor-plan.md, spec/monitor/matrix/TC-200-tick-monitor-consumer.md, spec/monitor/matrix/tests.md"
review_set: subset
---
# Integrity review of RT #209

## Summary

Ticket: IR-722. Checked consistency of the new text with the rest of FR-276,
interface-002/003, the matrix, and the sibling quire-contract-ir#945 (AD-010) and
quire-driver#129 (FR-019). The siblings agree on the boundary: the preparation budget
ends at `Program::try_new`, RT construction is uncharged, and build and steps use
`StepBudget`. AC-12 and AC-13 rows trace to TC-200, and the TC summary row lists them.
The `make spec` coverage refusal is environmental: the local quire-integration checkout
is on stale local main 74eb0a8 with staged deletions of tracked files, while origin/main
carries tests/monitor_program.rs as a regular blob. It is not a #209 trace defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The Step resource stop bullet still says the budget is "supplied at build". The new bullet says build and every step are bounded by "the caller's `StepBudget`", and `Monitor::set_step_budget` replaces it. The two phrasings should name one source | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:119, spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:144-145 |

## Verdict

No structural or cross-repo contradiction. FND-001 is wording only.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both bullets now say the monitor's current `StepBudget`, supplied at build and replaceable through `Monitor::set_step_budget` |
