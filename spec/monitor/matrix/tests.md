---
id: TM-006
title: "Tick monitor subsystem test matrix"
type: TestMatrix
---

# Tick monitor subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-276 | FR-276-AC-1 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-2 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-3 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-4 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-5 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-6 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-7 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-8 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-9 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-10 | TC-200 | 🚧 Planned: pending coherent integration evidence |
| FR-276 | FR-276-AC-11 | TC-200 | 🚧 Planned: pending coherent integration evidence |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-200 | Tick monitor boundaries, limits and faults | Integration | P0 | FR-276-AC-1, FR-276-AC-2, FR-276-AC-3, FR-276-AC-4, FR-276-AC-5, FR-276-AC-6, FR-276-AC-7, FR-276-AC-8, FR-276-AC-9, FR-276-AC-10, FR-276-AC-11 | 🚧 Planned |

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

**Planned: complete temporal acceptance:** FR-276-AC-1 through AC-11 and TC-200
remain Planned. These require real QSL-emitted plan admission/lowering and the
independent tick-reference comparison, beyond controlled executable arenas.
IT-018-SC-15's source-body provenance finding remains open: a manually constructed
event-position arena is not execution of the admitted source bodies. Passing the
count sequence alone does not grant FR-276-AC-11 qualification.

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
