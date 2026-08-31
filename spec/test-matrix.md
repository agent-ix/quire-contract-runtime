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
| TC-005 | Resolve and build every supported feature profile | Inspection | P0 | FR-003-AC-2, NFR-001-AC-1 | ✅ Complete |
| TC-006 | Retain complete campaign accounting | Unit | P0 | FR-004-AC-1, FR-004-AC-2 | ✅ Complete |
| TC-007 | Audit runtime footprint and packaging policy | Inspection | P0 | NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2, NFR-002-AC-4 | ✅ Complete |
| TC-008 | Inspect provenance-bearing public model | Inspection | P0 | FR-001-AC-3, FR-004-AC-3, NFR-002-AC-3 | ✅ Complete |

Inspection-class TC-005, TC-007, and TC-008 combine self-identifying Rust source-policy tests with
retained build, compile-fail, or audit outputs. Every test-matrix row now has a `tc_NNN` Rust test
binding; executable semantic claims retain direct acceptance-criterion trace tags.

## Evidence Locations

- TC-001 and TC-006: `tests/integration.rs`.
- TC-002 and TC-003: `tests/operators.rs`; TC-003 also has five Kani harnesses.
- TC-004: `tests/proptest_adapter.rs`.
- TC-005: `tests/release_contract.rs`, compile-fail crate documentation, `make test-features`, and the
  retained default dependency record.
- TC-007: `tests/release_contract.rs`, the footprint crate's fixed-result test,
  `tests/test_evidence_tooling.py`, plus retained dependency, enforced linked-footprint and panic-reference,
  observational rlib-size, unsafe, license, manifest, and pinned PGM schema audit records.
- TC-008: `tests/release_contract.rs` recursively scans all shipped runtime source, supplemented by
  five compile-fail enum doctests, public API documentation, and retained schema evidence.
