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
| FR-276 | FR-276-AC-2 | TC-200 | ✅ Implemented: capacity 22 and the 23rd-event Incomplete in quire-integration tests/monitor_tick_producer.rs (IR-722 follow-up, branch code/ir722-tc200-gaps; Refs IR-722), against the QSL TickMonitorReference oracle; the sibling-live case by step_2_a_shorter_window_buffer_overflows_while_its_sibling_stays_live (a 2 ms and a 10 ms once get capacities 6 and 22; the short buffer reports Incomplete(6) while the sibling keeps its identity and own outcomes; limit/setting texts asserted); the activation limit by step_2_an_instance_beyond_the_activation_capacity_names_the_activation_limit (the 9th `on each` instance against max_activations 8 yields Incomplete(8) naming activation / monitor.max_activations) |
| FR-276 | FR-276-AC-3 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-4 | TC-200 | 🚧 Partial: counter identity/unit/width/profile/admitted-order changes are covered in quire-integration tests/monitor_tick_producer.rs; the equal-timestamp-without-admitted-order clause is Planned: the producer always supplies the admitted order and the runtime refuses any other, so a plan without one cannot be built (missing: a producer/plan option that omits the admitted order, accepted by the runtime). The test step_3_equal_tick_events_in_both_arrival_orders_under_the_admitted_order_agree_with_the_reference runs under the admitted order and is not a cell for this clause |
| FR-276 | FR-276-AC-5 | TC-200 | ✅ Implemented: a limit on one obligation leaves its sibling's identity and state intact (step_2_a_shorter_window_buffer_overflows_while_its_sibling_stays_live) and the five outcome kinds (definite, Indeterminate, Pending, Incomplete, Failed), each with a known obligation identity, are asserted in one place against the oracle (step_5_the_step_api_carries_five_distinct_outcome_kinds_with_obligation_identities); the reading-gap fault test (step_7_..., tagged AC-9) is not cited for AC-5. In quire-integration tests/monitor_tick_producer.rs (IR-722 follow-up, branch code/ir722-tc200-gaps; Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-6 | TC-200 | 🚧 Partial: independence of two monitors, no verdict from build, one shared plan, and retention after the producer root is dropped of the layout, the four entries, the State/Inputs/Scope types, one capture per atom and the buffer capacities (step_4_the_admitted_program_retains_its_layout_entries_and_declared_types) are covered in tests/monitor_tick_producer.rs; Planned: rebinding (RT exposes no rebind operation) and atom bodies as an object (the Program carries one capture field per atom; the observation owner evaluates the bodies, no accessor exists) |
| FR-276 | FR-276-AC-7 | TC-200 | 🚧 Partial: foreign source selection and starved build are covered in tests/monitor_tick_producer.rs; target-premise refusals in quire-driver monitor_refusal.rs and monitor_closure.rs; Unsupported in quire-driver monitor_lowering.rs. Planned: altered atom/transition/initializer/interval/buffer/activation bindings and equal subformula ids under two subjects, because prepare_monitor takes only the producer's own PlanRoot (missing: a driver entry taking a re-read package together with the original selection to compare; IR strict admission alone accepts a changed counter_width_bits, max_reading_gap, max_event_rate, counter or activation_capacity literal and refuses a changed uncertainty) |
| FR-276 | FR-276-AC-8 | TC-200 | 🚧 Partial: covered in tests/monitor_tick_producer.rs including false activation (step_6_a_false_activation_guard_starts_no_root_instance, an `on each ... when` clause); Planned: a producer-plan executor-invariant failure (no public injection point; the controlled-arena equivalent is AC-12) |
| FR-276 | FR-276-AC-9 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-10 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-11 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-12 | TC-200 | ✅ Implemented: quire-integration tests/monitor_program.rs state_bound_violations_fail_at_the_entry_site_commit_nothing_and_terminate over a controlled arena (the real producer never emits an over-bound entry) |
| FR-276 | FR-276-AC-13 | TC-200 | 🚧 Partial: each StepBudget field one below its measured use, for build and one event, returns the stated StepResource with exact limit and spent+requested and commits nothing (step_11_each_step_budget_field_one_below_its_use_reports_the_exact_stop in tests/monitor_tick_producer.rs; the exact set of fields drawn on is asserted per phase; decimal_digits, scale_expansion, text_input_bytes, text_scalars, normalized_scalars and unit_edges are never drawn on by this clause; the event phase samples only the first event); allocator denials over a controlled arena are in quire-integration tests/monitor_program.rs tests tagged TC-098 / IT-018-SC-16 / interface-002-AC-3. Planned: allocator denials with producer-admitted ProgramParts (missing: a driver entry returning ProgramParts, since prepare_monitor returns only Rc<Program>) and PathArenaCapacity surfaced in path_capacity (no public input exhausts the path arena) |
| FR-276 | FR-276-AC-14 | TC-200 | 🚧 Partial: executor_operating_stops_and_budget_replacement_are_typed_and_atomic and the other TC-098 / IT-018-SC-16 / interface-002-AC-3 tests in quire-integration tests/monitor_program.rs cover denials over a controlled arena. Planned: allocator denials inside Layout, function parameter/result and expression type checks with producer-admitted identities, and their allocation-enabled retries (missing: a driver entry returning ProgramParts) |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-200 | Tick monitor boundaries, limits and faults | Integration | P0 | FR-276-AC-1, FR-276-AC-2, FR-276-AC-3, FR-276-AC-4, FR-276-AC-5, FR-276-AC-6, FR-276-AC-7, FR-276-AC-8, FR-276-AC-9, FR-276-AC-10, FR-276-AC-11, FR-276-AC-12, FR-276-AC-13, FR-276-AC-14 | 🚧 Partial: steps 1, 2, 3, 7-10 covered (AC-2 including the sibling-live case and the activation limit; AC-5 five outcome kinds); steps 4-6 and 11-12 partial; AC-4 equal-timestamp clause uncovered; see Evidence Status |

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

These are 29 component criteria, and the five criteria listed below under
"Backed by tests" are also backed. TC-209's procedures 1 through 4 are backed,
but interface-002-AC-5 and AC-6 remain Planned, so the complete TC-209 case
remains Planned. The observed 86/0 harness result does not
establish every stated test intention or accept the whole IT-018/TC-098 contract.

**Backed by tests in `quire-integration` `tests/monitor_program.rs`:** each criterion
below is exercised by the named test, tagged `TC-098` or `IT-018-SC-14`, in the
owner repository. A reviewer read each test against its criterion text.

- interface-002-AC-1: `retained_program_outlives_preparation_and_keeps_four_entry_keys`
  checks root, entries, literal bodies, package lock, occurrence and the original
  clock, admitted-order and progress selection bytes against literals written in
  the test, not values derived from the returned Layout.
- interface-004-AC-1: `lexical_missing_duplicate_sparse_and_foreign_declarations_refuse`
  covers the four refusals; the retained Let/Local source key and body path are
  asserted by `lexical_value_is_evaluated_once_within_its_scoped_body`.
- interface-006-AC-3: `tuple_projection_retained_addresses_respect_scratch_budget`
  (scratch charge, retained source key and body path) and
  `tuple_deep_projections_use_explicit_continuations_without_subtree_copies`
  (2000-deep projection on a 512 KiB stack; the temporary owner count returns to 1);
  `tuple_projected_state_outlives_the_temporary_owner_in_declared_position_order`
  checks the original owner counts (2 at the monitor State).
- interface-011-AC-3: `equality_and_union_scratch_denials_leave_no_partial_initial_state`,
  `checked_equality_and_union_retention_keep_native_operating_shortages` and
  `union_payloads_stop_in_declaration_order_before_a_later_native_denial`.
- interface-012-AC-1: `dispatch_checks_each_independent_context_component_before_executing`
  refuses each of package, clause, subject, occurrence, clock, counter, unit, width,
  profile and order with state and reading baseline unchanged, and a lower valid
  watermark afterwards shows a rejected progress was not committed;
  `dispatch_refuses_foreign_associations_ranges_and_regressing_progress_atomically`
  covers invalid values.

**Planned: incomplete whole-criterion evidence (three criteria):**

- interface-003-AC-3: invalid membership and native Work-limit channels are observed;
  native shared-conformance storage/path-capacity failure is not. An outer stack
  reservation failure during budget replacement is a different channel.
- interface-007-AC-3: scratch charging and the retained source key and body path are
  asserted by `record_member_addresses_consume_scratch_before_returning_state`, and
  declaration scans are charged by
  `record_dynamic_declaration_uses_its_own_order_and_charges_every_inspected_field`.
  Original-value ownership is asserted by
  `record_members_retain_declared_identity_and_original_value_owners` (owner count 1,
  State count 2). Record and tuple projection share one `Frame::TupleElement`
  continuation in `src/monitor/vm.rs`, which the deep tuple test reaches, but no deep
  record projection test exists, so the explicit-continuation clause is unproven for
  records.
- interface-010-AC-3: native visit, element and collection-result-retain stops are
  asserted by `sequence_visit_and_formation_stops_retain_native_causes_without_a_partial_value`
  and `collection_result_retention_denials_keep_the_native_charge_and_no_partial_state`.
  Explicit scratch stops are asserted by `sequence_fold_temporary_addresses_are_bounded_by_explicit_scratch`
  and `dispatch_stopped_fold_restores_slots_before_same_machine_retry`.
  `sequence_kind_and_authored_bound_are_not_reinterpreted_or_replaced_by_scratch_limits`
  asserts refusals, not a stop.
  `exhausting_work_stack_or_scratch_stops_map_and_filter_as_it_stops_a_fold`
  (interface-010-AC-5) stops fold, map and filter under Work 40, Stack 4 and
  ScratchElements 6. It asserts only the failure code and the resource kind, not the
  exact limit and spent evidence, and its full-budget run uses freshly built
  programs, not a later invocation on the same machine. The row stays Planned for
  that reason.

**Partial: complete temporal acceptance:** TC-200 is partially covered by
quire-integration `tests/monitor_tick_producer.rs` (IR-722 and its follow-up, Refs IR-722)
and `tests/monitor_program.rs`, against the QSL `TickMonitorReference` oracle. Steps 1,
2 (capacity 22, the sibling-live case and the activation limit), 3, 5's five outcome kinds, 7, 8, 9 and 10 are covered; step 4 (retention and
independence), 5 and 6 (including false activation) are partial; step 11 is partial (every
`StepBudget` field one below its use is covered for build and an event, with the exact set of fields drawn on asserted per phase; six fields are never drawn on by this clause and the event phase samples only the first event) and step 12 is partial
(allocator denials only over a controlled arena). Planned, with the missing API: FR-276-AC-4
equal timestamps without an admitted order (a plan option omitting the order); AC-6 rebinding and
atom bodies as an object; AC-7 altered bindings and equal subformula ids (a driver entry taking a
re-read package with the original selection); AC-8 a producer-plan executor-invariant failure;
AC-13/AC-14 allocator denials with producer-admitted `ProgramParts` (a driver entry returning
`ProgramParts`) and `PathArenaCapacity` in `path_capacity`.
IT-018-SC-15's source-body provenance finding remains open: a manually constructed
event-position arena is not execution of the admitted source bodies. FR-276-AC-11 is
nevertheless backed: `step_9_event_position_counts_ingested_events_through_the_producer_plan`
runs the stated reading sequence through the producer plan, and
`event_position_counts_ingested_events_and_not_watermarks_polls_or_gap_faults` in
`tests/monitor_program.rs` asserts the State has exactly four slots with
`event_position` last. `closed_state_refuses_missing_optional_mistyped_extra_and_misordered_fields`
shows `Program::try_new` refuses an extra State field, so no extra counter field can
exist.

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
