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

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-200 | Tick monitor boundaries, limits and faults | Integration | P0 | FR-276-AC-1, FR-276-AC-2, FR-276-AC-3, FR-276-AC-4, FR-276-AC-5, FR-276-AC-6, FR-276-AC-7, FR-276-AC-8, FR-276-AC-9, FR-276-AC-10 | 🚧 Planned |

## Pending Integration Evidence

FR-276-AC-1 through FR-276-AC-11 and every acceptance criterion in
interface-002 through interface-012 remain **Planned: pending integration evidence**.
This includes the retained-program, initializer, lexical, Boolean, tuple, record,
scalar, composite, sequence, union/equality and dispatch controls in the complete
86-test `monitor_program` target of `quire-integration`.

Native runtime checks and source trace tags are supporting evidence only. They do
not upgrade these criteria or grant completion credit. The controls must execute
successfully on a coherent published producer/reader/runtime cohort before their
integration evidence is accepted. The real event-position sequence control for
FR-276-AC-11 remains pending with that target.

## Evidence Locations

- `quire-integration` owns the authoritative `tests/monitor_program.rs` controls.
  The runtime references the owner's source on main; it does not copy the tests.
- TC-209 describes retained-program construction controls. TC-200 independently
  owns complete producer/build/event/watermark/gap acceptance; construction and
  native operating checks do not replace it.
- After the runtime and integration changes land, rerun the main-only reference
  coverage scan. Keep IR-722 open until the producer refusal QSL-1570 is resolved
  and all 86 controls pass. No status is upgraded by the landing sequence alone.
