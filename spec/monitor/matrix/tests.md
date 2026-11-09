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
| FR-276 | FR-276-AC-2 | TC-200 | 🚧 Partial: capacity 22 and the 23rd-event Incomplete are covered in quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle; the sibling-live-obligation case is not covered |
| FR-276 | FR-276-AC-3 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-4 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-5 | TC-200 | 🚧 Planned: not covered; the producer gives both buffers of an and-ed clause the same capacity, so no live sibling survives an overflow to observe |
| FR-276 | FR-276-AC-6 | TC-200 | 🚧 Partial: independence of two monitors, no verdict from build and one shared plan are covered in tests/monitor_tick_producer.rs; subject/atom-body/topology retention and rebinding are not covered |
| FR-276 | FR-276-AC-7 | TC-200 | 🚧 Partial: foreign source selection and starved build are covered in tests/monitor_tick_producer.rs; target-premise refusals in quire-driver monitor_refusal.rs and monitor_closure.rs; Unsupported in quire-driver monitor_lowering.rs; altered atom/transition/initializer/interval/buffer/activation bindings and equal subformula ids under two subjects are not covered |
| FR-276 | FR-276-AC-8 | TC-200 | 🚧 Partial: covered in tests/monitor_tick_producer.rs except false activation and a producer-plan executor-invariant failure, which are not covered |
| FR-276 | FR-276-AC-9 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-10 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-11 | TC-200 | ✅ Implemented: quire-integration tests/monitor_tick_producer.rs (PR agent-ix/quire-integration#96, Refs IR-722), against the QSL TickMonitorReference oracle |
| FR-276 | FR-276-AC-12 | TC-200 | ✅ Implemented: quire-integration tests/monitor_program.rs state_bound_violations_fail_at_the_entry_site_commit_nothing_and_terminate over a controlled arena (the real producer never emits an over-bound entry) |
| FR-276 | FR-276-AC-13 | TC-200 | ✅ Implemented: quire-integration tests/monitor_program.rs (first_layout_type_allocation_returns_native_membership_then_retries, later_declared_type_walk_denials_keep_native_membership_and_source, rt_validation_scratch_denial_stays_storage_after_five_layout_walks, native_type_projection_keeps_allocation_and_capacity_requests_distinct, deeply_shared_inline_arena_validates_on_a_small_native_stack) |
| FR-276 | FR-276-AC-14 | TC-200 | ✅ Implemented: quire-integration tests/monitor_program.rs executor_operating_stops_and_budget_replacement_are_typed_and_atomic |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-200 | Tick monitor boundaries, limits and faults | Integration | P0 | FR-276-AC-1, FR-276-AC-2, FR-276-AC-3, FR-276-AC-4, FR-276-AC-5, FR-276-AC-6, FR-276-AC-7, FR-276-AC-8, FR-276-AC-9, FR-276-AC-10, FR-276-AC-11, FR-276-AC-12, FR-276-AC-13, FR-276-AC-14 | 🚧 Partial: steps 1, 2, 3, 7-12 covered (AC-2 sibling-live case not covered); steps 4-6 partial; FR-276-AC-5 not covered; see Evidence Status |

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
quire-integration `tests/monitor_tick_producer.rs` (PR agent-ix/quire-integration#96,
Refs IR-722; head 755a41d0) and `tests/monitor_program.rs`, against the QSL
`TickMonitorReference` oracle. Steps 1, 2 (capacity 22; the sibling-live case of
FR-276-AC-2 is not covered), 3, 7, 8, 9 and 10 are covered; steps 4, 5 and 6 are
partial; steps 11 and 12 are covered by existing `tests/monitor_program.rs` tests.
Not covered: FR-276-AC-5; AC-6 retention of subject/atoms/topology and rebinding;
AC-7 altered bindings and equal subformula ids (target-premise refusals and
Unsupported are covered by quire-driver `monitor_refusal.rs`, `monitor_closure.rs`
and `monitor_lowering.rs`); AC-8 false activation and executor-invariant failure
of a producer plan.
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
