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

When the runtime receives a QSL-produced `TimedMonitorPlan` with the producer-defined executable
topology and event projection for a timestamped-event claim, the runtime SHALL construct and
advance a bounded tick monitor under the plan's clock binding and event-rate limit. The runtime
owns the monitor's state, counter arithmetic, storage, and typed step outcomes;
QSL owns the checked claim, interval conversion, buffer capacities, and temporal meaning
(ix://agent-ix/quire-spec-language/FR-251). The runtime SHALL NOT reconstruct a temporal formula
or copy QSL's reference evaluator from plan metadata.

The exact construction signature is pending a QSL-owned plan topology and event-projection
contract. The four fields currently listed in QSL FR-251 (`binding`, `intervals`, `buffers`,
`rate_limit`) do not identify the operator graph, obligation identities, or event predicates a
monitor must advance. The independent obligations below describe the consumer once that input is
defined; they do not claim an implementation is available.

## Inputs

- A QSL-produced `TimedMonitorPlan` with a QSpec FR-252 `ClockBindingKey`, plan-selected tick
  intervals, `(SubformulaId, capacity)` buffers, and an exact event-rate limit.
- A counter source identified by the plan's binding, and one observed event with its counter
  reading per monitor step. The event projection and executable topology remain QSL-685's
  unresolved producer contract; the runtime neither infers them from `SubformulaId` nor reads
  host wall-clock time.
- The plan's declared counter width and caller-stated `max_reading_gap` premise from its QSL
  `MonitorTarget`; the producer must make these available to the consumer without an inferred
  default.

## Outputs

- Construction either yields a monitor bound to the plan's exact clock identity or a typed
  refusal before an event is consumed. A target with `max_reading_gap >= 2^width_bits` has QSL
  `MonitorPlanRefusal::GapExceedsWidth{max_reading_gap, width_bits}` and cannot yield a monitor.
- Each affected obligation has a typed step result: definite true, definite false, pending, or
  indeterminate; `Incomplete(LimitReached{limit, value, setting})` for an event-rate buffer stop;
  or `Failed(ReadingGapExceeded{gap, max_reading_gap})` for a counter fault. Incomplete and failed
  are not temporal verdicts. Unaffected obligations retain their prior state.

## Behavior

- The runtime SHALL compare the attached counter identity, unit, width, profile, and admitted
  order with the plan's QSpec FR-252 clock binding before consuming an event. A foreign or
  incomplete binding SHALL NOT settle an obligation. The runtime SHALL NOT infer admitted order
  from ingestion order or numeric tick order.
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
  decision. The runtime SHALL NOT treat this monitor's result as proof of elapsed real time,
  worst-case execution time, preemption, or interrupt timing (QSL ADR-026 KG-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-276-AC-1 | With a 20-bit 1 MHz counter and zero uncertainty, request at tick 0 and first acknowledgement at 2998, 3000, or 3001 yield respectively definite true, indeterminate, and definite false for the 3 ms obligation; with 2 µs uncertainty the 2998 case remains indeterminate. No boundary case is decided from a point estimate. | Test |
| FR-276-AC-2 | The QSL plan's 21-slot buffer for `once[0 ms, 10 ms] holds(p)` at 2 events/ms admits 21 in-window events; the 22nd yields `Incomplete(LimitReached)` naming 2 events/ms, without dropping that event or changing an unrelated obligation. The runtime uses the capacity supplied by QSL rather than deriving a second formula. | Test |
| FR-276-AC-3 | Under a 16-bit counter with maximum gap 100, readings 65530 then 4 advance by 10; readings 65530 then 65500 settle `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` with no temporal verdict. A plan whose maximum gap is 65536 is refused before construction. | Test |
| FR-276-AC-4 | Changing the counter identity, unit, width, profile or admitted order never reuses the first plan's monitor state or settles its obligation; equal timestamps without admitted order never acquire causality from arrival order. | Test |
| FR-276-AC-5 | The step API carries distinct definite, pending, indeterminate, incomplete and failed outcomes with the affected obligation identities; a limit or fault on one obligation does not turn a sibling's state into a verdict or erase its identity. | Test |

## Dependencies

- QSL ADR-026 MN-1 to MN-4 and FR-251 own plan production, rounding and capacities; QSpec
  FR-160 owns uncertainty and QSpec FR-252 owns clock binding identity and admitted order.
- QSL-685 must define the executable topology, event projection, obligation identities and
  counter-target fields before RT can freeze its constructor or accept runtime implementation.
  QSL-587 must produce the plan; IR-349 must complete the applicable RT architecture migration.
