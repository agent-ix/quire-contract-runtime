---
id: SR-5266
title: spec-review/failure-domain review of IR-519 PR 114
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-runtime@399ea71d7e6b677bc55633f1dee578d674f7e486; spec/monitor/functional/FR-276-consume-timed-monitor-plan.md,
  spec/monitor/matrix/TC-200-tick-monitor-consumer.md, spec/monitor/matrix/tests.md,
  spec/spec.md, spec/tests.md
review_set: subset
---

## Summary

Ticket: IR-519. Reviewed foreign references, missing bodies, state ownership, changed target premises, invalid inputs, counter gaps, buffer overflow, and executor failure. Each has an explicit refusal, incomplete, failed, or build-failure boundary.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-276 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | When the runtime receives one driver-admitted immutable `TimedMonitorPlan` with QSL's complete |
| TC-200 | examined | spec/monitor/matrix/TC-200-tick-monitor-consumer.md | Exercise the future RT monitor against QSL-produced plans and independent, test-authored expected |
| TM-006 | examined | spec/monitor/matrix/tests.md | \| FR-276 \| FR-276-AC-1 \| TC-200 \| 🚧 Planned: QSL plan producer and RT monitor absent \| |
| Spec | examined | spec/spec.md | - The planned consumer of QSL-produced tick monitor plans: bounded counter steps, fixed buffers, |
| TestIndex | examined | spec/tests.md | \| Tick monitor \| FR-276 \| `monitor/matrix/tests.md` \| 🚧 Planned; QSL-685 contract specified, QSL-587 producer and RT implementation not yet available \| |
| FR-276-AC-1 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | With a 20-bit 1 MHz counter and zero uncertainty, request at tick 0 and first acknowledgement at 2998, 3000, or 3001 yield respectively definite true, indeterminate, and definite false for the 3 ms obligation; with 2 µs uncertainty the 2998 case remains indeterminate. No boundary case is decided from a point estimate. |
| FR-276-AC-2 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | The QSL plan's 21-slot buffer for `once[0 ms, 10 ms] holds(p)` at 2 events/ms admits 21 in-window events; the 22nd yields `Incomplete(LimitReached)` naming 2 events/ms, without dropping that event or changing an unrelated obligation. The runtime uses the capacity supplied by QSL rather than deriving a second formula. |
| FR-276-AC-3 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | Under a 16-bit counter with maximum gap 100, readings 65530 then 4 advance by 10; readings 65530 then 65500 settle `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` with no temporal verdict. A plan whose maximum gap is 65536 is refused before construction. |
| FR-276-AC-4 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | Changing the counter identity, unit, width, profile or admitted order never reuses the first plan's monitor state or settles its obligation; equal timestamps without admitted order never acquire causality from arrival order. |
| FR-276-AC-5 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | The step API carries distinct definite, pending, indeterminate, incomplete and failed outcomes with the affected obligation identities; a limit or fault on one obligation does not turn a sibling's state into a verdict or erase its identity. |
| FR-276-AC-6 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | The complete admitted plan alone builds a monitor after driver preparation objects are dropped. Its checked subject, activation/captures, atom bodies, ordered topology, typed state initializers and executable event/watermark entries remain available. Build consumes no event and emits no verdict; a second build has independent state, and neither monitor can be rebound to another plan. No source parse, second clause, callback or RT operator translation completes it. |
| FR-276-AC-7 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | For each of a foreign atom/operand, missing transition body, changed initializer, interval/buffer owner, activation binding, uncertainty and maximum gap under the original selection, driver/IR admission refuses without RT initialization; equal numeric subformula IDs under different subjects do not alias. An unsupported executable operation yields `Unsupported`, and RT allocation or initialization failure returns typed build failure with no monitor. |
| FR-276-AC-8 | examined | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md | Invalid event/progress inputs, including a missing required admitted value, leave monitor state unchanged with no verdict. An admitted progress assertion can settle a deadline, while a mere counter poll cannot. Executing the admitted program agrees with QSL's reference evaluator on boundary, reversed-operand, false-activation and open/closed-endpoint cases; an RT executor invariant failure returns `Failed`, terminates the monitor and rejects later input. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean for this analysis at the reviewed head. The RT monitor and test binder remain planned.
