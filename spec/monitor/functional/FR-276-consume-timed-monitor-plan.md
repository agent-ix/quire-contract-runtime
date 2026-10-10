---
id: FR-276
title: "Consume a QSL tick monitor plan"
type: FR
relationships:
  - target: ix://agent-ix/quire-spec-language/FR-251
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/ADR-026
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-160
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-252
    type: depends_on
---
# FR-276: Consume a QSL tick monitor plan

## Description

When the runtime receives one driver-admitted immutable `TimedMonitorPlan` with QSL's complete
executable program for a timestamped-event claim, `build(plan)` SHALL allocate fresh state, run
the prescribed initialization entry, and yield a bounded monitor without consuming an event or
emitting a verdict. The runtime SHALL execute the plan's event and admitted-watermark transition
bodies and own state, counter arithmetic, storage, input admission, and typed outcomes;
QSL owns the checked claim, complete program's temporal meaning, interval conversion and capacities
(ix://agent-ix/quire-spec-language/FR-251). The runtime SHALL NOT reconstruct a temporal formula
or operator update from graph tags, names, intervals or buffers, or copy QSL's reference evaluator.
This is a planned consumer contract, not an implemented monitor.

## Inputs

- One driver-admitted immutable executable plan in RT representation, bound to the independently
  selected checked package/clause and target. It carries the exact subject, clock binding with
  counter/unit/width, uncertainty, maximum reading gap, rate limit, intervals, buffers, atoms,
  ordered topology, typed state and complete initialization/event/watermark instruction bodies.
  Its `PlanId` selects the complete plan in driver ownership, not merely its resource table.
- An observed event with the plan's admitted counter reading, snapshot/invocation values and
  binding identity; separately, an admitted progress assertion for a watermark transition.
  Hardware acquisition and conversion into admitted inputs belong to the caller, which supplies
  no atom callback, replacement clause, transition function, or new temporal meaning.

## Outputs

- QSL derivation refusal or driver/IR admission refusal yields no RT plan to build. An unsupported
  instruction/predicate receives `Unsupported` before build. RT allocation or initialization
  failure yields typed `MonitorBuildFailure` and no usable partial monitor. A target with
  `max_reading_gap >= 2^width_bits` has QSL
  `MonitorPlanRefusal::GapExceedsWidth{max_reading_gap, width_bits}` and cannot yield a monitor.
- Each affected obligation has a typed step result: definite true, definite false, pending, or
  indeterminate; `Incomplete(LimitReached{limit, value, setting})` for an event-rate buffer stop;
  or `Failed(ReadingGapExceeded{gap, max_reading_gap})` for a counter fault. Incomplete and failed
  are not temporal verdicts. An invalid event/progress input yields a typed refusal without state
  change; executor invariant failure yields `Failed` and terminates the monitor. Unaffected
  obligations retain their prior state.

## Behavior

- Driver admission SHALL bind the complete executable program to its independently selected
  checked package/clause and target before RT build. Missing or foreign atom/operand references,
  instruction bodies, state initializers, interval/buffer owners, activation bindings, or changed
  target premises SHALL refuse admission with no initialization; structural validity alone grants
  no execution authority. RT SHALL own or retain the whole immutable plan for the monitor's
  lifetime, allocate independent mutable state on each build, and prohibit live rebinds.
- The runtime SHALL compare attached counter identity, unit, width, profile, admitted order and
  required entry values with the plan before consuming an event; watermark input SHALL carry an
  admitted progress assertion under QSpec FR-094/FR-160. Invalid input SHALL leave state unchanged
  and settle no obligation. Polling a counter alone creates neither an event position nor a
  watermark. The runtime SHALL NOT infer admitted order from ingestion or numeric tick order.
- The runtime SHALL compute consecutive counter differences modulo `2^width_bits`, using the
  declared width without truncating a reading or overflowing host integer arithmetic. A
  difference greater than `max_reading_gap` SHALL settle each affected obligation `Failed` with
  `ReadingGapExceeded{gap, max_reading_gap}` and SHALL NOT emit a temporal verdict for it.
- The runtime SHALL use the plan's capacities as fixed upper bounds. If an event would exceed a
  buffered subformula's capacity, the runtime SHALL retain the event or stop before consuming it,
  and SHALL settle the affected obligations `Incomplete(LimitReached{limit, value, setting})`
  naming the plan's event-rate limit and exact setting. It SHALL NOT silently drop the event,
  expand storage, or convert the stop into false.
- The runtime SHALL preserve the plan's sound tick-boundary uncertainty. A result SHALL be
  definite only when every instant represented by the observed tick and declared uncertainty
  agrees. Otherwise the result SHALL remain pending or indeterminate under QSpec FR-160; the
  runtime SHALL NOT use a midpoint, host clock, or rounded point estimate to decide it.
- The runtime SHALL preserve the plan's subformula and obligation identities through each
  result. A fault or limit stop on one obligation SHALL NOT overwrite an unrelated sibling's
  decision. Subject-scoped IDs from another plan SHALL NOT alias despite equal numeric indices.
  An executor invariant failure SHALL terminate the monitor as `Failed`, never as a Boolean
  property verdict; the runtime SHALL reject subsequent input. The runtime SHALL NOT treat this
  monitor's result as proof of elapsed real time,
  worst-case execution time, preemption, or interrupt timing (QSL ADR-026 KG-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-276-AC-1 | With a 20-bit 1 MHz counter and zero uncertainty, request at tick 0 and first acknowledgement at 2998, 3000, or 3001 yield respectively definite true, indeterminate, and definite false for the 3 ms obligation; with 2 µs uncertainty the 2998 case remains indeterminate. No boundary case is decided from a point estimate. | Test |
| FR-276-AC-2 | The QSL plan's 21-slot buffer for `once[0 ms, 10 ms] holds(p)` at 2 events/ms admits 21 in-window events; the 22nd yields `Incomplete(LimitReached)` naming 2 events/ms, without dropping that event or changing an unrelated obligation. The runtime uses the capacity supplied by QSL rather than deriving a second formula. | Test |
| FR-276-AC-3 | Under a 16-bit counter with maximum gap 100, readings 65530 then 4 advance by 10; readings 65530 then 65500 settle `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` with no temporal verdict. A plan whose maximum gap is 65536 is refused before construction. | Test |
| FR-276-AC-4 | Changing the counter identity, unit, width, profile or admitted order never reuses the first plan's monitor state or settles its obligation; equal timestamps without admitted order never acquire causality from arrival order. | Test |
| FR-276-AC-5 | The step API carries distinct definite, pending, indeterminate, incomplete and failed outcomes with the affected obligation identities; a limit or fault on one obligation does not turn a sibling's state into a verdict or erase its identity. | Test |
| FR-276-AC-6 | The complete admitted plan alone builds a monitor after driver preparation objects are dropped. Its checked subject, activation/captures, atom bodies, ordered topology, typed state initializers and executable event/watermark entries remain available. Build consumes no event and emits no verdict; a second build has independent state, and neither monitor can be rebound to another plan. No source parse, second clause, callback or RT operator translation completes it. | Test |
| FR-276-AC-7 | For each of a foreign atom/operand, missing transition body, changed initializer, interval/buffer owner, activation binding, uncertainty and maximum gap under the original selection, driver/IR admission refuses without RT initialization; equal numeric subformula IDs under different subjects do not alias. An unsupported executable operation yields `Unsupported`, and RT allocation or initialization failure returns typed build failure with no monitor. | Test |
| FR-276-AC-8 | Invalid event/progress inputs, including a missing required admitted value, leave monitor state unchanged with no verdict. An admitted progress assertion can settle a deadline, while a mere counter poll cannot. Executing the admitted program agrees with QSL's reference evaluator on boundary, reversed-operand, false-activation and open/closed-endpoint cases; an RT executor invariant failure returns `Failed`, terminates the monitor and rejects later input. | Test |

## Dependencies

- QSL ADR-026 MN-1 to MN-7 and FR-251 own the complete executable plan, temporal meaning,
  rounding and capacities; QSpec
  FR-160 owns uncertainty and QSpec FR-252 owns clock binding identity and admitted order.
- QSL-685 fixed the closed executable plan and one-argument build boundary in QSL FR-251/ADR-026;
  QSL-587 must produce the plan, driver/IR must admit and lower its complete program, and IR-349
  must complete the applicable RT architecture migration before runtime implementation acceptance.
