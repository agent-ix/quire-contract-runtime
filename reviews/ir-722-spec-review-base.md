---
id: SR-2026110705000805
title: "Base spec review of RT #209 bounded monitor state and budget boundary"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/monitor/functional/FR-276-consume-timed-monitor-plan.md, spec/monitor/matrix/TC-200-tick-monitor-consumer.md, spec/monitor/matrix/tests.md"
review_set: subset
---
# Base spec review of RT #209

## Summary

Ticket: IR-722. PR quire-contract-runtime#209 at d82a03d3e1332ab8af9a8724040d002fbc7e4c3a.
Examined the new FR-276 Bounded state and Preparation and operating budgets bullets,
FR-276-AC-12, FR-276-AC-13, TC-200 steps 10 and 11 with their expected results, and the
two new matrix rows, against RT `Program::try_new`, `Layout`, `StepBudget`, `build` and
entry dispatch on origin/main, QSL FR-251 (summary cell and activation capacity) and the
QSL producer's `startAct` capacity guard. The capacity-plus-one bound agrees with QSL
FR-251 ("at most the capacity of entries and the cell"), and every `activations` push in
the producer is guarded by `activation_capacity`, so the bound check cannot terminate a
correct monitor. No pins, digests, vendoring or ticket narration were added. Three
acceptance criteria leave the coder an unknown or can pass vacuously.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The bound violation names no `ExecutorInvariantCode` and no build-failure code. The enum is closed (Address..Membership) and `build_failure` maps by cause, so the coder must invent a variant and AC-12 cannot assert which one | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:132-135, src/monitor/executor.rs:41-61 |
| FND-002 | medium | AC-13 "a 20,000-expression arena" does not require depth: a recursive validator passes on a flat 20,000-node arena. TC-209 already says "20,000-level" | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:181, spec/monitor/matrix/TC-200-tick-monitor-consumer.md:85-87 |
| FND-003 | medium | AC-13 "Build and steps ... stop only through `StepBudget` resources" is a universal negative with no stated control or oracle. Allocator `StepResource::Storage` and path-arena capacity stops are not bounded by a `StepBudget` field | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:181, src/monitor/budget.rs:34-35 |
| FND-004 | low | TC-200 step 10 runs the cases in sequence. After the 24-element failure the monitor is terminated, so the 3-activation case only tests `Terminated`. An `init` failure leaves no monitor to "offer one further input" to | spec/monitor/matrix/TC-200-tick-monitor-consumer.md:82-85, spec/monitor/matrix/TC-200-tick-monitor-consumer.md:131-132 |

## Verdict

Not mergeable as written: FND-001 to FND-003 must be fixed in this PR. The bounded state
text still meets IR-722 acceptance 1 ("fresh bounded state"): it bounds element counts at
every commit, including `init`, without narrowing any existing AC. It bounds counts, not
per-element bytes. Entry size follows the admitted input types.
