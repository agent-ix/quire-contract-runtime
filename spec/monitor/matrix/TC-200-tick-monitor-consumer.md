---
id: TC-200
title: "Tick monitor boundaries, limits and faults"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-276
    type: verifies
---
# TC-200: Tick monitor boundaries, limits and faults

## Description

Qualify the RT monitor against complete QSL-produced plans and independent, test-authored
expected outcomes. This complete temporal acceptance case remains Planned. The implemented
construction and ordinary-executor controls do not establish this procedure.

## Evidence Status

**Planned.** The 86/0 `monitor_program` result recorded in
[quire-integration PR #41](https://github.com/agent-ix/quire-integration/pull/41)
backs the component criteria identified in the monitor evidence matrix. The evaluated
source is now published on integration main; the guarded reference scan completed
with 161 unbacked rows and zero contradicted statuses (strict exit 1), not a passing
global coverage result. Neither fact establishes the remaining behavior. The run does not
execute a QSL-emitted FR-487 plan through the complete lowerer and independent
QSL tick-reference comparison required here. The source-body provenance finding
also prevents treating the diagnostic event-position sequence as full AC-11 evidence.
No complete temporal criterion is upgraded by the retained run, publication or reference scan.

## Test Procedure

1. Obtain a QSL-produced plan for `always (holds(req) implies eventually[0 ms, 3 ms]
   holds(ack))` on a 20-bit, 1 MHz counter. Observe `req` at tick 0 and first `ack` at ticks
   2998, 3000 and 3001 in separate runs; reading `k` covers the half-open `[k, k+1)`. Repeat 2998
   with 2 µs uncertainty. Compare each
   obligation's complete typed result with the independent expectations below; do not compare
   one runtime output with another runtime output as the oracle.
2. Obtain the QSL plan for `once[0 ms, 10 ms] holds(p)` at 2 events/ms. Assert its supplied
   capacity is 22 (1 MHz counter), feed 22 in-window events, then offer a 23rd while another independent
   obligation remains live. Check the event is neither silently dropped nor silently accepted
   beyond the fixed capacity, and check both obligation identities and states.
3. Under a 16-bit counter with maximum reading gap 100, offer readings 65530 then 4, then in a
   separate run 65530 then 65500. Request a target with maximum gap 65536 from the QSL plan
   producer and check its refusal happens before RT construction. Repeat a valid pair with a
   counter identity, unit, width, profile or admitted order changed one component at a time;
   equal-tick events with no admitted order must not gain causality from ingestion order.
4. Admit one QSL-derived closed executable plan against an independently selected checked
   package/clause and target. Drop the driver's preparation objects, then build two RT monitors
   from that plan. Verify the plan retains subject, activation/captures, atom bodies, ordered
   implication/eventually/always topology, the `State` type and the `init`, `on_event` and
   `on_watermark` function entries (the `monitor_plan` plan-root node, QSpec FR-487); feed a request only to the first monitor. Try rebinding either live monitor to a
   changed plan. No source parse, additional clause, atom callback or RT operator translation may
   complete the program.
5. Before RT initialization, independently replace an atom/operand reference, entry body,
   `init` body, interval association, buffer owner, activation binding, uncertainty, counter,
   counter width, tick unit, maximum event rate, buffer capacity and maximum gap while keeping the original selected subject/target. Also give equal numeric
   subformula IDs from two different subjects. Exercise unsupported executable content and RT
   allocation and initialization failures separately. Record the boundary and typed result for
   every rejection rather than treating them as Boolean property decisions.
6. Offer an event missing a required admitted value and a progress input lacking QSpec-admitted
   assertion; compare monitor state before and after each. Separately provide an admitted
   watermark, whose unwrapped history-boundary tick and scope identities RT passes to
   `on_watermark`, past a deadline and an ordinary counter poll with no watermark. Run the same admitted
   plan against QSL's reference evaluator on the boundary cases above, reversed implication
   operands, false activation, and open/closed interval endpoints. Inject an executor invariant
   failure and offer one further input. Use test-authored expected subject/obligation origins and
   outcomes rather than comparing two RT outputs as the oracle.
7. On the 16-bit target of Step 3, keep one obligation instance open across the fault and one
   already decided before it, and start a third after it. Offer the first reading, then readings
   `65530`, `65500` (the fault), `65510`, recording each `on_event` and `on_gap` call and the
   delta passed.
8. Build with a per-step budget too small for one `on_event`; offer an event, then raise the budget
   and offer the same event; separately run an injected executor invariant failure and offer
   another input. Compare state before and after each call.


9. Observe the shared State's event_position throughout the AC-9 counter sequence.
   Build starts at 0; the baseline on_event(0) changes it to 1. Insert an admitted
   watermark and a clock poll: both preserve 1. The gap-faulting reading calls on_gap
   and preserves 1; the next admitted event changes it to 2. Inspect the final required
   Integer field after activations; no host-width mirror supplies the observation.
10. With the plan of Step 2 (capacity 22) and activation capacity 2, run the admitted program
    with entries whose returned State holds 23 buffer elements and 2 activations, then 24
    elements, then 3 activations, and an `init` returning 3 activations. Offer one further input
    after each failure.
11. Construct a Program from complete parts, then with a forced validation scratch reservation
    failure, and a 20,000-expression arena on a 512 KiB stack. Inspect the `try_new` signature.
    Build and step under `StepBudget` limits only.

Steps 1 to 9 run in `quire-integration` on the genuine QSL plan root prepared by the driver from
the same emission; steps 10 and 11 are RT-local controls over the same representation.

## Expected Results

- Step 1 yields definite true at 2998 (difference in `(2997, 2999)`), indeterminate at 3000
  (`(2999, 3001)`), definite false at 3001 (`(3000, 3002)`), and indeterminate at 2998 with 2 µs
  uncertainty (`(2993, 3003)`). Pending remains distinct from indeterminate
  when a future observation could still resolve the window.
- Step 2 uses the producer's 22-slot capacity and settles only affected obligations
  `Incomplete(LimitReached{limit, value, setting})`, naming the event-rate setting 2 events/ms
  on the 23rd event. It never drops an event or changes an unrelated sibling's result.
- Step 3 computes the wrap difference 10 in RT and passes `on_event` that delta (the entry never
  sees a raw wrapping reading), reports
  `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` for the backward-looking pair,
  and emits no temporal verdict for that fault; RT calls `on_gap` with 65506 (not `on_event`) and each instance still open across the fault settles `Failed`. The 65536-gap target yields QSL
  `GapExceedsWidth` with no RT monitor constructed. Every foreign or incomplete binding is
  refused before it can settle an obligation; no order is inferred from equal ticks.
- Step 4 builds two independent monitors without a verdict from initialization; a request in the
  first changes no state in the second. Dropped preparation objects do not invalidate either
  monitor, rebinding fails unchanged, and outputs retain the exact selected subject, target
  premises, obligation instance and origin.
- Step 5 refuses altered or incomplete programs at driver/IR admission with no RT initialization,
  even when numeric IDs match. Unsupported content yields `Unsupported` before build. Allocation
  and initialization failures yield typed `MonitorBuildFailure` with no usable monitor, not a
  false property verdict.
- Step 6 refuses invalid event/progress input without a state change or verdict. Admitted progress
  can settle the deadline, while a counter poll cannot. RT execution and the independent QSL
  reference evaluator agree per obligation in all named cases. Executor invariant failure yields
  `Failed` with no verdict; the monitor refuses subsequent input.
- Step 7 passes delta 0 for the first reading with no gap check and leaves the unwrapped time at
  `init`'s value; at the fault RT calls `on_gap` with 65506, the open instance settles
  `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` and is dropped, the decided and the
  not-yet-started instances are unaffected, and `65510` is differenced from `65500` (delta 10).
- Step 8 returns `StepResourceStop` with state unchanged and the event unconsumed, the same event
  then executes under the larger budget, the stop is distinct from a verdict, `Incomplete`,
  `Failed(ReadingGapExceeded)` and `ExecutorInvariantFailure`, and only the invariant failure
  terminates the monitor and rejects later input.
- Step 9 observes event positions 0, 1, 1, 1 and 2 at build, baseline event,
  watermark/poll, gap fault and following admitted event respectively. The baseline
  event increments the count despite delta 0; the faulting reading does not increment it.
- Step 10 commits the first State; the 24-element, 3-activation and `init` cases return
  `ExecutorInvariantFailure` or a typed build failure, commit nothing and refuse later input.
- Step 11 returns a Program, then `ProgramRefusal` `Storage` with no Program; the deep arena
  completes; `try_new` takes only `ProgramParts`, and only `StepBudget` stops build and steps.
