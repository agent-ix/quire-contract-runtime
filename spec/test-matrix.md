---
id: TM-001
title: "Contract runtime v0.1 test matrix"
type: TestMatrix
---

# Contract runtime v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-2 | TC-001 | ✅ Complete |
| FR-001 | FR-001-AC-3 | TC-008 | ✅ Complete |
| FR-002 | FR-002-AC-1, FR-002-AC-2 | TC-002 | ✅ Complete |
| FR-002 | FR-002-AC-3 | TC-003 | ✅ Complete |
| FR-003 | FR-003-AC-1 | TC-004 | ✅ Complete |
| FR-003 | FR-003-AC-2 | TC-005 | ✅ Complete |
| FR-004 | FR-004-AC-1, FR-004-AC-2 | TC-006 | ✅ Complete |
| FR-004 | FR-004-AC-3 | TC-008 | ✅ Complete |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Preserve verdict and observation identity | Unit | P0 | FR-001-AC-1, FR-001-AC-2 | ✅ Complete |
| TC-002 | Exercise Boolean evaluation contracts | Unit | P0 | FR-002-AC-1, FR-002-AC-2 | ✅ Complete |
| TC-003 | Check definedness boundaries | Property | P0 | FR-002-AC-3, NFR-002-AC-1 | ✅ Complete |
| TC-004 | Preserve proptest tri-state mapping | Unit | P0 | FR-003-AC-1 | ✅ Complete |
| TC-005 | Build every supported feature profile | Inspection | P0 | FR-003-AC-2, NFR-001-AC-1 | ✅ Complete |
| TC-006 | Retain complete campaign accounting | Unit | P0 | FR-004-AC-1, FR-004-AC-2 | ✅ Complete |
| TC-007 | Audit runtime footprint and packaging policy | Inspection | P0 | NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2 | ✅ Complete |
| TC-008 | Inspect provenance-bearing public model | Inspection | P0 | FR-001-AC-3, FR-004-AC-3, NFR-002-AC-3 | ✅ Complete |

Inspection-class TC-005, TC-007, and TC-008 are deliberately backed by retained build or audit
outputs rather than Rust test symbols. TC-001 through TC-004 and TC-006 name self-identifying
`tc_NNN` Rust test functions.

## Evidence Locations

- TC-001 and TC-006: `tests/integration.rs`.
- TC-002 and TC-003: `tests/operators.rs`; TC-003 also has five Kani harnesses.
- TC-004: `tests/proptest_adapter.rs`.
- TC-005: `make test-features` and the retained default dependency record.
- TC-007: retained dependency, size, unsafe, license, and manifest audit records.
- TC-008: public API documentation, source inspection, and retained schema evidence.
