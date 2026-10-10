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

Exercise the future RT monitor against QSL-produced plans and independent, test-authored expected
outcomes. This case is planned; no monitor implementation or test binder exists in RT today.

## Test Procedure

1. Obtain a QSL-produced plan for `always (holds(req) implies eventually[0 ms, 3 ms]
   holds(ack))` on a 20-bit, 1 MHz counter. Observe `req` at tick 0 and first `ack` at ticks
   2998, 3000 and 3001 in separate runs. Repeat 2998 with 2 µs uncertainty. Compare each
   obligation's complete typed result with the independent expectations below; do not compare
   one runtime output with another runtime output as the oracle.
2. Obtain the QSL plan for `once[0 ms, 10 ms] holds(p)` at 2 events/ms. Assert its supplied
   capacity is 21, feed 21 in-window events, then offer a 22nd while another independent
   obligation remains live. Check the event is neither silently dropped nor silently accepted
   beyond the fixed capacity, and check both obligation identities and states.
3. Under a 16-bit counter with maximum reading gap 100, offer readings 65530 then 4, then in a
   separate run 65530 then 65500. Request a target with maximum gap 65536 from the QSL plan
   producer and check its refusal happens before RT construction. Repeat a valid pair with a
   counter identity, unit, width, profile or admitted order changed one component at a time;
   equal-tick events with no admitted order must not gain causality from ingestion order.

## Expected Results

- Step 1 yields definite true at 2998, indeterminate at 3000, definite false at 3001, and
  indeterminate at 2998 with 2 µs uncertainty. Pending remains distinct from indeterminate
  when a future observation could still resolve the window.
- Step 2 uses the producer's 21-slot capacity and settles only affected obligations
  `Incomplete(LimitReached{limit, value, setting})`, naming the event-rate setting 2 events/ms
  on the 22nd event. It never drops an event or changes an unrelated sibling's result.
- Step 3 computes the wrap difference 10, reports
  `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` for the backward-looking pair,
  and emits no temporal verdict for that fault. The 65536-gap target yields QSL
  `GapExceedsWidth` with no RT monitor constructed. Every foreign or incomplete binding is
  refused before it can settle an obligation; no order is inferred from equal ticks.
