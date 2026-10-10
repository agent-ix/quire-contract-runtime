---
id: TM-004
title: "Proptest adapter subsystem test matrix"
type: TestMatrix
---

# Proptest adapter subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-003 | FR-003-AC-1 | TC-004 | ✅ Complete |
| FR-003 | FR-003-AC-2 | TC-004 | ✅ Complete (compile_fail doctest, `src/lib.rs`) |
| FR-003 | FR-003-AC-3 | TC-004 | ✅ Complete |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-004 | Verify the optional proptest adapter | Compile | P0 | FR-003-AC-1, FR-003-AC-2, FR-003-AC-3 | ✅ Complete |

TC-004's feature-off step is the `compile_fail` doctest in `src/lib.rs`.

## Evidence Locations

- TC-004: `src/lib.rs` doctest and `tests/proptest_adapter.rs`.
