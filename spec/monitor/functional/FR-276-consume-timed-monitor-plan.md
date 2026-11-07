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
the plan's `init` entry, and yield a bounded monitor without consuming an event or
emitting a verdict. The plan's entries are the state-passing checked-expression `function` nodes
`init`, `on_event` and `on_watermark` of its plan-root node on the checked-package v2 wire (QSL
FR-251; the plan-root node is the `temporal`/`monitor_plan` node of QSpec FR-487, FR-322-AC-308). The runtime SHALL execute `on_event` and
`on_watermark` and own the modular counter arithmetic, state storage, input admission, and typed outcomes;
QSL owns the checked claim, complete program's temporal meaning, interval conversion and capacities
(ix://agent-ix/quire-spec-language/FR-251). The runtime SHALL NOT reconstruct a temporal formula
or operator update from graph tags, names, intervals or buffers, or copy QSL's reference evaluator.
This is a planned consumer contract, not an implemented monitor.

## Inputs

- One driver-admitted immutable executable plan in RT representation, bound to the independently
  selected checked package/clause and target. It carries the exact subject, clock binding with
  counter/unit/width, uncertainty, maximum reading gap, rate limit, intervals, buffer capacities
  and activation capacity (premises of the plan root, seventeen ordered bindings in QSpec FR-487;
  `activation_capacity` is member 16), atoms, ordered topology, the `State` type (exactly `now`,
  `history_start`, one `buffer_<n>` per buffer, then `activations` and `event_position`;
  QSpec FR-487) and the four function
  entries `init`, `on_event`, `on_watermark` and `on_gap` (QSL FR-251's fourth entry,
  member 15 of QSpec FR-487's root, FR-487-AC-7).
  The unwrapped event_position is 0 from init and increases by 1 per admitted event;
  an "on each" activation reads its position when created, without a separate
  activation field (QSpec FR-487, QSL FR-251).
  Its `PlanId` selects the complete plan in driver ownership, not merely its resource table.
- An observed event with the plan's admitted raw counter reading, snapshot/invocation values and
  binding identity; the runtime, not the plan, computes the modular delta from the previous reading
  and passes `on_event` the gap-checked delta (an entry never sees a raw wrapping reading and keeps
  unwrapped time in its `State`). Separately, an admitted progress assertion for a watermark
  transition, from which the runtime passes `on_watermark` the unwrapped history-boundary tick (an
  integer) and the admitted scope identities of QSpec FR-094/FR-160.
  Hardware acquisition and conversion into admitted inputs belong to the caller, which supplies
  no atom callback, replacement clause, transition function, or new temporal meaning.

## Outputs

- QSL derivation refusal or driver/IR admission refusal yields no RT plan to build. An unsupported
  entry operation or predicate receives `Unsupported` before build. RT allocation or initialization
  failure yields typed `MonitorBuildFailure` and no usable partial monitor. A target with
  `max_reading_gap >= 2^width_bits` has QSL
  `MonitorPlanRefusal::GapExceedsWidth{max_reading_gap, width_bits}` and cannot yield a monitor.
- Each affected obligation has a typed step result: definite true, definite false, pending, or
  indeterminate; `Incomplete(LimitReached{limit, value, setting})` for an event-rate buffer stop (`limit` `event_buffer`, setting `monitor.max_event_rate`) or an
  activation stop (`limit` `activation`, setting `monitor.max_activations`);
  or `Incomplete(LimitReached{limit: atom_projection, value: <AtomId integer>, setting: monitor.atom_projection})`
  when an atom has no admitted projection (QSpec #597, FR-487-AC-12);
  or `Failed(ReadingGapExceeded{gap, max_reading_gap})` for a counter fault. Incomplete and failed
  are not temporal verdicts. An invalid event/progress input yields a typed refusal without state
  change. A per-step budget exhaustion yields the typed `StepResourceStop` (below), distinct from every
  verdict, from `Incomplete`, from `Failed(ReadingGapExceeded)` and from the terminal
  `ExecutorInvariantFailure`, which yields `Failed` for the monitor and terminates it. Unaffected
  obligations retain their prior state.

## Behavior

- Driver admission SHALL bind the complete executable program to its independently selected
  checked package/clause and target before RT build. Missing or foreign atom/operand references,
  entry bodies (including `init`, the state initializer), interval/buffer owners, activation bindings, or changed
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
  difference greater than `max_reading_gap` SHALL settle each live instance whose verdict is still open `Failed` with
  `ReadingGapExceeded{gap, max_reading_gap}` (through `on_gap`, below) and SHALL NOT emit a temporal verdict for it.
- **Reporting.** The runtime SHALL return the plan's results as produced: each interval-operator instance once on settlement, each pending instance at every watermark step and no event step, and one folded outcome per activation for an unbounded root (QSpec FR-487-AC-10, QSL FR-251); it SHALL NOT synthesise per-position results.
- **Atom projection.** The runtime SHALL preserve the admitted program's
  `atom_projection` limit kind, exact AtomId integer and `monitor.atom_projection`
  setting in canonical Outcome values. This is not an executor resource stop.
- RT SHALL NOT substitute an atom callback or infer an unavailable projection.
- **Event position.** State SHALL end with the required integer `event_position`
  field after `activations`, with no extra field.
- `init` SHALL set `event_position` to 0.
- `on_event` SHALL add 1 to `event_position` per admitted event, including the baseline reading's
  `on_event(0)`. `on_watermark`, `on_gap` and a clock poll SHALL leave it unchanged.
  The count is unwrapped and distinct from tick time; a faulting reading handled by
  `on_gap` is not an admitted event (QSpec #595, FR-487-AC-11).
- **Baseline.** The first admitted reading of a monitor SHALL establish the baseline: the runtime
  makes no gap check for it and passes `on_event` the delta 0, so the entry's unwrapped time stays
  at the value `init` set.
- When a gap fault occurs, the runtime SHALL make the faulting reading the new baseline, so the
  next reading's difference is taken from it, and the monitor SHALL continue. The baseline moves only
  on admitted readings (an event reading or a gap-faulting reading); a counter poll or a watermark
  never moves it.
- When a gap fault occurs, the runtime SHALL call the plan's fourth entry
  `on_gap(State, gap) -> (State, Results)` with the checked gap (QSL FR-251; the entry's wire
  binding is member 15 of QSpec FR-487's root, FR-487-AC-7; its returned state has `now` and `history_start` at the faulting reading's tick, every `buffer_<n>`
  empty; a past-time window reaching before `history_start` is `indeterminate` until refilled, QSL
  FR-251-AC-15, QSpec FR-487-AC-9). Every live instance whose verdict is still
  open (pending, its window or deadline spanning the gap) yields
  `Failed(ReadingGapExceeded{gap, max_reading_gap})` and is dropped from `State`; instances already
  decided, results already emitted and instances not yet started are unaffected. The runtime SHALL
  NOT infer instance identities from buffer contents or reconstruct formula meaning.
- **Step resource stop.** The runtime SHALL charge each step against the work, stack and scratch
  limits of the monitor's current `StepBudget` (supplied at build, replaceable through
  `Monitor::set_step_budget`). When a step exhausts it, the runtime SHALL return the typed
  `StepResourceStop`, leave the state unchanged and the event or watermark unconsumed, and remain
  usable; the unconsumed input may be re-presented and executes the transition body once (QSL
  FR-251's carve-out, FR-251-AC-12). `StepResourceStop` is not a temporal verdict, not `Incomplete(LimitReached)` (the
  event-buffer stop), not `Failed(ReadingGapExceeded)` and not the terminal
  `ExecutorInvariantFailure`; allocation or resource failure is not an invariant defect and not a
  property result. The runtime SHALL NOT promise recovery from an arbitrary allocator
  out-of-memory.
- **Bounded state.** Before committing the State returned by `init`, `on_event`, `on_watermark`
  or `on_gap`, the runtime SHALL check that each `buffer_<n>` sequence holds at most its plan
  capacity plus one element (the single summary cell an unbounded past operator keeps first,
  QSL FR-251) and that `activations` holds at most `activation_capacity` elements.
- When `on_event`, `on_watermark` or `on_gap` returns a State outside these bounds, the runtime SHALL
  return `MonitorFailure` with code `Invariant` and an `ExecutorInvariantFailure` whose code
  is `ExecutorInvariantCode::StateBound` and whose site is `ExecutionSite::Function` of that entry,
  commit nothing and terminate the monitor; later input SHALL return code `Terminated`.
- When `init` returns a State outside these bounds, `monitor::build` SHALL return `MonitorFailure`
  with code `Invariant` and an `ExecutorInvariantFailure` with code `StateBound` at
  `ExecutionSite::Function` of `init`, and no monitor.
- `ExecutorInvariantCode::StateBound` SHALL be the variant of the closed executor invariant enum
  for this condition; no other code SHALL report it.
- The bound check SHALL read sequence lengths only, without interpreting entries, inferring which
  subformula is unbounded or settling an obligation; RT keeps no State store besides the
  canonical State value.
- **Preparation and operating budgets.** The driver's preparation budget ends when it hands the
  complete `ProgramParts` to `Program::try_new`, which SHALL take no budget parameter.
- `Program::try_new` SHALL perform only these walks, each iteratively and with the stated bound
  over the supplied parts (F functions, E expressions, D operand edges, B lexical bindings, S State
  fields, U buffers, T `ValueType` nodes across all checked types, C shared declarations, L total
  nodes of all literal values):

  | Walk | Bound | Reservation failure |
  | --- | --- | --- |
  | Layout premises, State fields and buffers, excluding the type checks below | O(S + U) | none allocated |
  | Function-key uniqueness scratch | O(F log F) work, F elements | `ProgramRefusal` `Storage` |
  | Fallible `TypeEnvironment::try_check_type` of the five Layout type bindings, then of each parameter, result and expression type | O((5 + F + E) x (T + C)) | `ProgramRefusal` `Membership` with the native reservation cause retained in `membership_failure` |
  | Expression graph, ownership and cycle walk | O(E + D) work, E elements per scratch vector | `ProgramRefusal` `Storage` |
  | `TypeEnvironment::admits` of each literal | O(L + T + C) | `ProgramRefusal` `Membership` with the native `EnvironmentFailure` retained |
  | Bindings, fields and construction tables | O(F + E + B + S) | `ProgramRefusal` `Storage` |

- `Program::try_new` SHALL reserve its own validation scratch fallibly, sized from these counts,
  before use.
- `Program::try_new` SHALL make every type check, including the Layout bindings checked first,
  through quire-semantic-value's fallible check-type entry, and SHALL report its reservation failure
  as `ProgramRefusal` `Membership` with the native cause retained in `membership_failure`. A
  failure inside a walk owned by semantic-value (type check or literal membership) is `Membership`;
  a failure of RT-owned scratch is `Storage`.
  RT SHALL retain the native type-check error class and original request: semantic
  `Refused(IllTyped)` keeps the existing `TypeMismatch` behavior, whereas `Allocation` and
  `Capacity` remain distinct native failures with their original `StorageRequest` unit and
  amount. RT MAY carry an allocation or capacity failure in the corresponding existing
  `EnvironmentFailure` variant only by moving the original `StorageRequest` unchanged.
  Literal membership retains its own `EnvironmentFailure`. Projecting native failure SHALL
  allocate no new storage and perform no allocating clone, including of a source `NodeKey`.
  RT SHALL return no partially validated Program on failure.
  With allocation enabled, RT SHALL admit the same valid parts on retry after an allocator-denied attempt.
- The runtime SHALL bound build and every event, watermark, gap and poll step by the monitor's
  current `StepBudget` (supplied at build, replaceable through `Monitor::set_step_budget`) and by
  the native stops below; neither budget changes a plan capacity:

  | Stop | Governing bound | How a test observes it |
  | --- | --- | --- |
  | `StepResource::Work` | `StepBudget.work` | Measured work minus one returns the stop with `limit`, `spent`, `requested`. |
  | `StepResource::Stack` | `StepBudget.stack_frames` | Measured depth minus one. |
  | `StepResource::ScratchElements`, `ScratchBytes` | `StepBudget.scratch_elements`, `scratch_bytes` | Measured use minus one. |
  | `StepResource::Scalar(kind)` | `StepBudget.scalar` counter of that kind | That counter's measured use minus one; the original `ScalarFailure` is retained. |
  | Native membership resource stop | The same `StepBudget` work and scratch limits | `MonitorFailure.admission` holds the `SuppliedAdmissionError` and its arena beside the `StepResourceStop`. |
  | `StepResource::Storage` | The allocator; `limit` is absent | A failing global allocator in a dedicated test binary refuses the next allocation. |
  | `path_capacity` | semantic-value's path-arena identifier space | Inspection that a native `PathArenaCapacity` reaches `MonitorFailure.path_capacity` with code `Resource`; the owner exercises exhaustion. |

- A forced allocation failure SHALL be produced only by a failing global allocator installed in a
  dedicated integration test binary, with a documented `SAFETY` contract as in
  `measurement/exact-panic`; production code SHALL gain no test-only reservation seam.
- The runtime SHALL use the plan's capacities as fixed upper bounds. If an event would exceed a
  buffered subformula's capacity, the runtime SHALL retain the event or stop before consuming it,
  and SHALL settle the affected obligations `Incomplete(LimitReached{limit, value, setting})`
  naming the plan's event-rate limit and exact setting. It SHALL NOT silently drop the event,
  expand storage, or convert the stop into false.
- The runtime SHALL preserve the plan's sound tick-boundary uncertainty. A reading `k` covers the
  half-open interval `[k, k+1)` of the counter unit, widened by the declared uncertainty `u` to
  `[k-u, k+1+u)` (QSL FR-251, ADR-026 MN-2). A result SHALL be definite only when every instant
  represented by the observed tick and declared uncertainty agrees. Otherwise the result SHALL remain pending or indeterminate under QSpec FR-160; the
  runtime SHALL NOT use a midpoint, host clock, or rounded point estimate to decide it.
- The runtime SHALL preserve the plan's subformula and obligation identities through each
  result. A fault or limit stop on one obligation SHALL NOT overwrite an unrelated sibling's
  decision. Subject-scoped IDs from another plan SHALL NOT alias despite equal numeric indices.
  An executor invariant failure (`ExecutorInvariantFailure`) SHALL terminate the monitor as `Failed`, never as a Boolean
  property verdict; the runtime SHALL reject subsequent input. The runtime SHALL NOT treat this
  monitor's result as proof of elapsed real time,
  worst-case execution time, preemption, or interrupt timing (QSL ADR-026 KG-4).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-276-AC-1 | With a 20-bit 1 MHz counter and zero uncertainty, request at tick 0 (covering `[0, 1)`) and first acknowledgement at 2998, 3000, or 3001 yield respectively definite true (difference in `(2997, 2999)`), indeterminate (`(2999, 3001)`, which straddles the bound 3000) and definite false (`(3000, 3002)`) for the 3 ms obligation; with 2 µs uncertainty the 2998 case (`(2993, 3003)`) remains indeterminate. No boundary case is decided from a point estimate. | Test |
| FR-276-AC-2 | The QSL plan's 22-slot buffer for `once[0 ms, 10 ms] holds(p)` at 2 events/ms on a 1 MHz counter (upper bound plus tick cover, QSL FR-251-AC-2) admits 22 in-window events; the 23rd yields `Incomplete(LimitReached)` naming 2 events/ms (an instance beyond `activation_capacity` is likewise not started and yields `limit` `activation`, setting `monitor.max_activations`), without dropping that event or changing an unrelated obligation. The runtime uses the capacity supplied by QSL rather than deriving a second formula. | Test |
| FR-276-AC-3 | Under a 16-bit counter with maximum gap 100, readings 65530 then 4 advance by 10 (RT passes `on_event` the delta 10); readings 65530 then 65500 make RT call `on_gap` with 65506 and not `on_event`; each instance still open across the fault settles `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` with no temporal verdict. A plan whose maximum gap is 65536 is refused before construction. | Test |
| FR-276-AC-4 | Changing the counter identity, unit, width, profile or admitted order never reuses the first plan's monitor state or settles its obligation; equal timestamps without admitted order never acquire causality from arrival order. | Test |
| FR-276-AC-5 | The step API carries distinct definite, pending, indeterminate, incomplete and failed outcomes with the affected obligation identities; a limit or fault on one obligation does not turn a sibling's state into a verdict or erase its identity. | Test |
| FR-276-AC-6 | The complete admitted plan alone builds a monitor after driver preparation objects are dropped. Its checked subject, activation/captures, atom bodies, ordered topology, `State` type and the `init`, `on_event` and `on_watermark` function entries (the plan-root node's dependencies, QSpec FR-487) remain available. Build runs `init` and the monitor's state is its result. Build consumes no event and emits no verdict; a second build has independent state, and neither monitor can be rebound to another plan. No source parse, second clause, callback or RT operator translation completes it. | Test |
| FR-276-AC-7 | For each of a foreign atom/operand, missing entry body, changed `init` body, interval/buffer owner, activation binding, uncertainty, counter, counter width, tick unit, maximum event rate, buffer capacity, activation capacity and maximum gap under the original selection (each of uncertainty, counter, tick unit, counter width, gap, rate and capacities is part of the plan-root's identity, QSpec FR-487-AC-6), driver/IR admission refuses without RT initialization; equal numeric subformula IDs under different subjects do not alias. An unsupported entry operation yields `Unsupported`, and RT allocation or initialization failure returns typed build failure with no monitor. | Test |
| FR-276-AC-8 | Invalid event/progress inputs, including a missing required admitted value, leave monitor state unchanged with no verdict. An admitted progress assertion can settle a deadline, while a mere counter poll cannot. Executing the admitted program agrees with QSL's reference evaluator on boundary, reversed-operand, false-activation and open/closed-endpoint cases; an RT executor invariant failure returns `Failed`, terminates the monitor and rejects later input. | Test |
| FR-276-AC-9 | On the 16-bit target of AC-3 the first admitted reading, whatever its value, passes `on_event` the delta 0 with no gap check and leaves the unwrapped time at `init`'s value; after readings `65530` then `65500` fault, a following reading `65510` is differenced from `65500` (delta 10), not from `65530`. RT calls `on_gap` with the gap 65506: the instance still open across the gap settles `Failed(ReadingGapExceeded{gap: 65506, max_reading_gap: 100})` and is dropped from the state, while an instance already decided before the fault and an instance not yet started are unaffected and settle normally afterwards. | Test |
| FR-276-AC-10 | With a per-step budget too small for one `on_event`, the call returns `StepResourceStop`, the monitor state is unchanged and the event unconsumed, and the same event under a sufficient budget then executes and gives the outcome it would have given; the result is distinct from a verdict, `Incomplete(LimitReached)`, `Failed(ReadingGapExceeded)` and `ExecutorInvariantFailure` (after which the monitor rejects input), and the monitor accepts the next input after a stop. No test relies on recovery from an allocator out-of-memory. | Test |
| FR-276-AC-11 | Build observes event_position 0 from init. With width 16 and maximum gap 100, the baseline event reading 65530 calls on_event(0) and leaves event_position 1; a watermark and poll leave it 1; reading 65500 faults through on_gap and leaves it 1; admitted reading 65510 calls on_event(10) and leaves it 2. The unwrapped exact Integer count is independent of tick time, and State keeps it after activations with no extra field. | Test |
| FR-276-AC-12 | For a plan with one bounded buffer of capacity 22 and activation capacity 2, each case on a fresh monitor: an `on_event` returning 23 buffer elements and 2 activations commits; one returning 24 elements, and one returning 3 activations, each return `MonitorFailure` code `Invariant` with `ExecutorInvariantCode::StateBound` at that entry's function site, commit nothing, and the next input returns code `Terminated`. An `init` returning 3 activations makes `build` return `MonitorFailure` code `Invariant` with `StateBound` at the `init` function site and no monitor. | Test |
| FR-276-AC-13 | `Program::try_new` takes only `ProgramParts`. In a dedicated `quire-integration` test binary with producer-admitted `ProgramParts`, whose failing global allocator refuses the first allocation inside `try_new` (the first Layout type check's worklist), construction returns `ProgramRefusal` code `Membership` with the native reservation cause and no Program; refusing the first RT-owned scratch reservation returns code `Storage`; refusing an allocation inside a literal's membership walk returns `Membership` with the native `EnvironmentFailure`. An expression arena nested 20,000 levels deep constructs on a 512 KiB native stack. For build and a step, each `StepBudget` field set one below its measured use returns the stated `StepResource` with exact `limit`, `spent` and `requested`; a refused allocation returns `StepResource::Storage` with no `limit`; source inspection shows `PathArenaCapacity` surfaced in `path_capacity`. | Test, Inspection |
| FR-276-AC-14 | In the linked `quire-integration` consumer PR, using producer-admitted identities rather than RT-owned fixtures, actual allocator denials inside Layout, function parameter/result and expression type checks return `Membership` with the original native allocation class and `StorageRequest`, without allocating to construct the refusal or returning a Program; allocation-enabled retries admit the same valid parts. Semantic unknown-type and operator-ineligible controls retain the existing `TypeMismatch` behavior and validation precedence. RT-local identity-free private native-error projection controls preserve `Allocation` versus `Capacity` and the original request unit and amount, with no synthetic configured limit; in the integration binary, a separately denied RT-owned scratch reservation remains `Storage` and literal membership retains its `EnvironmentFailure`. The RT PR alone does not discharge the producer-admitted construction cells. | Test, Inspection |

## Dependencies

- quire-semantic-value provides `TypeEnvironment::try_check_type`, whose `TypeCheckError`
  distinguishes `Refused(IllTyped)`, `Allocation(StorageRequest)` and
  `Capacity(StorageRequest)` (IR-1400). Its fallible worklist reservation is the owner boundary
  consumed by RT FR-276-AC-13 and FR-276-AC-14; it is distinct from `EnvironmentFailure`
  returned by literal membership.
- QSL ADR-026 MN-1 to MN-7 and FR-251 own the complete executable plan, temporal meaning,
  rounding and capacities; QSpec
  FR-160 owns uncertainty and QSpec FR-252 owns clock binding identity and admitted order.
- QSL-685 fixed the closed executable plan and one-argument build boundary in QSL FR-251/ADR-026;
  QSL-587 must produce the plan, driver/IR must admit and lower its complete program, and IR-349
  must complete the applicable RT architecture migration before runtime implementation acceptance.
- QSL FR-251 (QSL #2096) fixes the function-entry form, the plan-root node, the RT-owned modular
  delta and the half-open tick cover; QSpec FR-487 (with FR-322-AC-308) fixes the plan-root node,
  and FR-487-AC-6 makes uncertainty, counter, tick unit, counter width, gap, rate and capacities
  inputs of the plan's identity.
