---
id: TM-005
title: "Contract runtime test matrix index"
type: TestMatrixIndex
---

# Contract runtime test matrix index

Each subsystem's matrix is the one coverage authority for the requirements in its directory and
declares the test cases beside it. This index declares none.

## Requirements Traceability

| Subsystem | Requirements | Local Matrix | Status |
| --- | --- | --- | --- |
| Core | StR-001, FR-001, FR-002, NFR-001, NFR-002, interface-001 | `core/matrix/tests.md` | 🚧 interface-001 pending adoption |
| Accounting | FR-004 | `accounting/matrix/tests.md` | ✅ Complete |
| Proptest adapter | FR-003 | `proptest_adapter/matrix/tests.md` | ✅ Complete |
| Exact | FR-006, FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-273, FR-275, FR-277, FR-278 | `exact/matrix/tests.md` | 🚧 QSL agreement evidence removed (one kernel, FR-275); IR-882 value migration and FR-277 kernel route planned; FR-008-AC-10 and AC-11 have RT consumer evidence, and AC-12 has RT consumer evidence through planned equality |
| Tick monitor | FR-276 | `monitor/matrix/tests.md` | 🚧 Planned; QSL-685 contract specified, QSL-587 producer and RT implementation not yet available |
