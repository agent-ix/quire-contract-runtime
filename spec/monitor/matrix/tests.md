---
id: TM-006
title: "Tick monitor subsystem test matrix"
type: TestMatrix
---

# Tick monitor subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-276 | FR-276-AC-1 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-2 | TC-200 | ✅ Implemented: capacity 22 and the 23rd-event Incomplete in quire-integration tests/monitor_tick_producer.rs (IR-722 follow-up, branch code/ir722-tc200-gaps; Refs IR-722), against the QSL TickMonitorReference oracle; the sibling-live case by step_2_a_shorter_window_buffer_overflows_while_its_sibling_stays_live (a 2 ms and a 10 ms once get capacities 6 and 22; the short buffer reports Incomplete(6) while the sibling keeps its identity and own outcomes) |
| FR-276 | FR-276-AC-3 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-4 | TC-200 | 🚧 Partial: counter identity/unit/width/profile/admitted-order changes are covered in quire-integration tests/monitor_tick_producer.rs; the equal-timestamp-without-admitted-order clause is Planned: the producer always supplies the admitted order and the runtime refuses any other, so a plan without one cannot be built (missing: a producer/plan option that omits the admitted order, accepted by the runtime). The test step_3_equal_tick_events_in_both_arrival_orders_under_the_admitted_order_agree_with_the_reference runs under the admitted order and is not a cell for this clause |
| FR-276 | FR-276-AC-5 | TC-200 | ✅ Implemented: a limit on one obligation leaves its sibling's identity and state intact (step_2_a_shorter_window_buffer_overflows_while_its_sibling_stays_live) and a reading-gap fault on one open instance leaves the others alone (step_7_a_gap_fault_drops_only_the_open_instance_and_rebases_the_next_delta), in quire-integration tests/monitor_tick_producer.rs (IR-722 follow-up, branch code/ir722-tc200-gaps; Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-6 | TC-200 | 🚧 Partial: independence of two monitors, no verdict from build, one shared plan, and retention after the producer root is dropped of the layout, the four entries, the State/Inputs/Scope types, one capture per atom and the buffer capacities (step_4_the_admitted_program_retains_its_layout_entries_and_declared_types) are covered in tests/monitor_tick_producer.rs; Planned: rebinding (RT exposes no rebind operation) and atom bodies as an object (the Program carries one capture field per atom; the observation owner evaluates the bodies, no accessor exists) |
| FR-276 | FR-276-AC-7 | TC-200 | 🚧 Partial: foreign source selection and starved build are covered in tests/monitor_tick_producer.rs; target-premise refusals in quire-driver monitor_refusal.rs and monitor_closure.rs; Unsupported in quire-driver monitor_lowering.rs. Planned: altered atom/transition/initializer/interval/buffer/activation bindings and equal subformula ids under two subjects, because prepare_monitor takes only the producer's own PlanRoot (missing: a driver entry taking a re-read package together with the original selection to compare; IR strict admission alone accepts a changed counter_width_bits, max_reading_gap, max_event_rate, counter or activation_capacity literal and refuses a changed uncertainty) |
| FR-276 | FR-276-AC-8 | TC-200 | 🚧 Partial: covered in tests/monitor_tick_producer.rs including false activation (step_6_a_false_activation_guard_starts_no_root_instance, an `on each ... when` clause); Planned: a producer-plan executor-invariant failure (no public injection point; the controlled-arena equivalent is AC-12) |
| FR-276 | FR-276-AC-9 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-10 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-11 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-12 | TC-200 | ✅ Implemented: quire-integration tests/monitor_program.rs state_bound_violations_fail_at_the_entry_site_commit_nothing_and_terminate over a controlled arena (the real producer never emits an over-bound entry) |
| FR-276 | FR-276-AC-13 | TC-200 | 🚧 Partial: each StepBudget field one below its measured use, for build and one event, returns the stated StepResource with exact limit and spent+requested and commits nothing (step_11_each_step_budget_field_one_below_its_use_reports_the_exact_stop in tests/monitor_tick_producer.rs; fields the program does not draw on are skipped); allocator denials over a controlled arena are in quire-integration tests/monitor_program.rs tests tagged TC-098 / IT-018-SC-16 / interface-002-AC-3. Planned: allocator denials with producer-admitted ProgramParts (missing: a driver entry returning ProgramParts, since prepare_monitor returns only Rc<Program>) and PathArenaCapacity surfaced in path_capacity (no public input exhausts the path arena) |
| FR-276 | FR-276-AC-14 | TC-200 | 🚧 Partial: executor_operating_stops_and_budget_replacement_are_typed_and_atomic and the other TC-098 / IT-018-SC-16 / interface-002-AC-3 tests in quire-integration tests/monitor_program.rs cover denials over a controlled arena. Planned: allocator denials inside Layout, function parameter/result and expression type checks with producer-admitted identities, and their allocation-enabled retries (missing: a driver entry returning ProgramParts) |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-200 | Tick monitor boundaries, limits and faults | Integration | P0 | FR-276-AC-1, FR-276-AC-2, FR-276-AC-3, FR-276-AC-4, FR-276-AC-5, FR-276-AC-6, FR-276-AC-7, FR-276-AC-8, FR-276-AC-9, FR-276-AC-10, FR-276-AC-11, FR-276-AC-12, FR-276-AC-13, FR-276-AC-14 | 🚧 Partial: steps 1, 2, 3, 7-10 covered (AC-2 sibling-live case not covered); steps 4-6 and 11-12 partial; AC-4 equal-timestamp clause uncovered; FR-276-AC-5 not covered; see Evidence Status |

## Component Execution Evidence

**Executed owner controls, now published on integration main:** the retained complete
`monitor_program` run in [quire-integration PR #41](https://github.com/agent-ix/quire-integration/pull/41)
reports 86 passed, zero failed and zero ignored against the coherent
producer/reader/runtime cohort (quire-contract-runtime 0.1.0). Inspection of the
named passing controls backs these complete construction or ordinary-executor criteria:

- interface-002-AC-2 through AC-4;
- interface-003-AC-1, AC-2 and AC-4;
- interface-004-AC-2 through AC-4;
- interface-005-AC-1 through AC-3;
- interface-006-AC-1 and AC-2;
- interface-007-AC-1 and AC-2;
- interface-008-AC-1 through AC-3;
- interface-009-AC-1 through AC-3;
- interface-010-AC-1 and AC-2;
- interface-011-AC-1 and AC-2;
- interface-012-AC-2 through AC-4.

These are 29 component criteria. TC-209's procedures 2 through 4 are backed,
but procedure 1 lacks independent retained-selection-byte assertions, so the
complete TC-209 case remains Planned. The observed 86/0 harness result does not
establish every stated test intention or accept the whole IT-018/TC-098 contract.

**Planned: incomplete whole-criterion evidence (eight criteria):**

- interface-002-AC-1: root, entries, literal contents, package lock and occurrence
  retention are checked, but original clock, admitted-order and progress selection
  bytes have no independent expected-value oracle. Dispatch expectations derived
  from the returned Layout do not supply that missing retention observation.
- interface-003-AC-3: invalid membership and native Work-limit channels are observed;
  native shared-conformance storage/path-capacity failure is not. An outer stack
  reservation failure during budget replacement is a different channel.
- interface-004-AC-1: lexical refusal and functional controls pass, but no positive
  retained Let/Local source-key/body-path assertion establishes the location clause.

- interface-006-AC-3 and interface-007-AC-3: scratch, continuation, original-value
  ownership and declaration-scan controls pass, but no projection-specific assertion
  checks the retained original source key/body path.
- interface-010-AC-3: visit, element formation, work/stack/scratch and retry controls
  pass; a post-formation native collection-retention denial remains unexercised.
- interface-011-AC-3: payload-order and native equality/union shortage controls pass;
  operation-specific equality/union scratch-denial evidence remains absent.
- interface-012-AC-1: context components, invalid values, State and reading-baseline
  checks pass; preservation of the progress frontier after foreign/invalid rejection
  needs a distinguishing subsequent-watermark control.

**Partial: complete temporal acceptance:** TC-200 is partially covered by
quire-integration `tests/monitor_tick_producer.rs` (IR-722 and its follow-up, Refs IR-722)
and `tests/monitor_program.rs`, against the QSL `TickMonitorReference` oracle. Steps 1,
2 (capacity 22 and the sibling-live case), 3, 7, 8, 9 and 10 are covered; step 4 (retention and
independence), 5 and 6 (including false activation) are partial; step 11 is partial (every
`StepBudget` field one below its use is covered for build and an event) and step 12 is partial
(allocator denials only over a controlled arena). Planned, with the missing API: FR-276-AC-4
equal timestamps without an admitted order (a plan option omitting the order); AC-6 rebinding and
atom bodies as an object; AC-7 altered bindings and equal subformula ids (a driver entry taking a
re-read package with the original selection); AC-8 a producer-plan executor-invariant failure;
AC-13/AC-14 allocator denials with producer-admitted `ProgramParts` (a driver entry returning
`ProgramParts`) and `PathArenaCapacity` in `path_capacity`.
IT-018-SC-15's source-body provenance finding remains open: a manually constructed
event-position arena is not execution of the admitted source bodies. Passing the
count sequence alone does not grant FR-276-AC-11 qualification.

## Map and Filter Evidence

interface-010-AC-4 and AC-5 (`SequenceMap`, `SequenceFilter`; TC-098, IT-018-SC-12)
coverage has moved to the `quire-integration` `tests/monitor_program.rs` owner file,
because RT's TC-033 guard (FR-012-AC-8) forbids this repository from minting NodeKeys.

## Published Reference Qualification

The evaluated PR #41 controls are now published on integration main. The owner
monitor source, helpers, manifest and lock are unchanged from the evaluated run;
no native rerun is inferred from publication. The unchanged main-only reference
guard accepted the genuine clean owner main checkout. The completed strict scan
reports 161 unbacked rows and zero contradicted statuses, with exit 1: acquisition
and scanning succeeded, but repository-wide strict coverage is not passing.
No source copy, candidate substitution or guard modification was used. This
reference result does not supply missing behavioral assertions or upgrade any
remaining Planned criterion.

## Evidence Locations

- `quire-integration` owns the authoritative `tests/monitor_program.rs` controls.
  The runtime references the owner's source on main; it does not copy the tests.
- TC-209 describes retained-program construction controls. TC-200 independently
  owns complete producer/build/event/watermark/gap acceptance; construction and
  native operating checks do not replace it.
- The completed main-only reference scan is separate from runtime acceptance.
  Keep IR-722 open until complete admitted-plan execution and the independently
  required TC-200 qualification are backed. The former QSL-1570 producer refusal
  no longer explains the passing owner run; the landing sequence alone grants
  no additional acceptance credit.
