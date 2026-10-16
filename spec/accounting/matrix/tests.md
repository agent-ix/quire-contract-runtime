---
id: TM-003
title: "Accounting subsystem test matrix"
type: TestMatrix
---

# Accounting subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-004 | FR-004-AC-1, FR-004-AC-2 | TC-006 | ✅ Complete |
| FR-004 | FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7 | TC-015 | ✅ implemented |
| FR-004 | FR-004-AC-8 | TC-006 | ✅ Complete |
| FR-004 | FR-004-AC-9 | TC-015 | 🚧 planned: encoded-order test exists but lacks this criterion's trace tag |
| FR-004 | FR-004-AC-10 | — (Inspection) | 🚧 planned: key-first unknown-member refusal inspection has no bound evidence symbol |
| FR-004 | FR-004-AC-11 | — (Inspection) | 🚧 planned: generic JSON tree inspection has no bound evidence symbol |
| FR-004 | FR-004-AC-12 | — (Inspection) | 🚧 planned: capped parser-scratch inspection has no bound evidence symbol |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-006 | Retain complete campaign accounting | Unit | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-8 | ✅ Complete |
| TC-015 | Bound immutable campaign snapshot transport | Unit | P0 | FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7, FR-004-AC-9 | ✅ AC-4–AC-7 implemented; 🚧 AC-9 trace tag pending |

Completed rows are backed by `tc_NNN` Rust tests; executable semantic claims retain direct
acceptance-criterion trace tags. The planned rows above are unbacked.

## Evidence Locations

- TC-006: `tests/integration.rs`.
- TC-015: `tests/snapshot.rs`, private near-limit accounting tests, compile-fail API docs,
  and an isolated native memory-ceiling control. REV-009 records independent acceptance of
  the bounded transport; existing Kani evidence does not cover the parser, and exact shared
  stack/full release qualification remains separate.
