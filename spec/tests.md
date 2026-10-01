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
| Exact | FR-006, FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-273, FR-275 | `exact/matrix/tests.md` | 🚧 QSL agreement evidence removed (recreation tracked by IR-355 and IR-20); FR-008-AC-10 through AC-12 sequence and ordered-set cases untested |
