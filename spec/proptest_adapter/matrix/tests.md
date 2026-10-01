---
id: TM-004
title: "Proptest adapter subsystem test matrix"
type: TestMatrix
---

# Proptest adapter subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-003 | FR-003-AC-1 | TC-004 | ✅ Complete |
| FR-003 | FR-003-AC-2 | — | ✅ Complete (compile_fail doctest, `src/lib.rs`) |
| FR-003 | FR-003-AC-3 | TC-004 | ✅ Complete |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-004 | Preserve proptest tri-state mapping | Unit | P0 | FR-003-AC-1, FR-003-AC-3 | ✅ Complete |

FR-003-AC-2 is verified by a `compile_fail` doctest and has no test case.

## Evidence Locations

- TC-004: `tests/proptest_adapter.rs`.
