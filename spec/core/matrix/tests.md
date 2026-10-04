---
id: TM-002
title: "Core subsystem test matrix"
type: TestMatrix
---

# Core subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-2 | TC-001 | ✅ Complete |
| FR-001 | FR-001-AC-4 | TC-001 | ✅ Complete |
| FR-002 | FR-002-AC-1, FR-002-AC-2 | TC-002 | ✅ Complete |
| FR-002 | FR-002-AC-3 | TC-003 | ✅ Complete |
| FR-002 | FR-002-AC-4 | TC-003 | ✅ Complete |

## Interface Requirement Coverage

| Interface | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| interface-001 | interface-001-AC-1, interface-001-AC-3, interface-001-AC-4, interface-001-AC-5, interface-001-AC-7, interface-001-AC-9 | — | 🚧 pending adoption (unmet today: the runtime still defines an `exact` module and the kernel and residue items; planned, IR-349) |
| interface-001 | interface-001-AC-2, interface-001-AC-6, interface-001-AC-8, interface-001-AC-10, interface-001-AC-11, interface-001-AC-14 | — | 🚧 pending adoption (hold today by inspection: no file under `src/` or `verification/` names `quire_exact` or `quire_semantic_value`, so nothing is re-exported; no `NodeKey` constructor call outside tests, the negotiators and core items are defined in the runtime, `quire-exact` is optional behind `exact`, no other QSL crate is a normal dependency) |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Preserve verdict and observation identity | Unit | P0 | FR-001-AC-1, FR-001-AC-2, FR-001-AC-4 | ✅ Complete |
| TC-002 | Exercise Boolean evaluation contracts | Unit | P0 | FR-002-AC-1, FR-002-AC-2 | ✅ Complete |
| TC-003 | Check definedness boundaries | Property | P0 | FR-002-AC-3, FR-002-AC-4, NFR-002-AC-1 | ✅ Complete |
| TC-007 | Audit runtime footprint | Inspection | P0 | NFR-001-AC-3 | ✅ Complete |

NFR-001-AC-1 is verified by `make size` and NFR-001-AC-2 by `make lint` under
`#![forbid(unsafe_code)]`; NFR-002-AC-3 by a `compile_fail` doctest; and interface-001-AC-13 by the `make test-features`
row `build-exact-no-std-msrv`; none of them has a test case. Every other row is backed by a `tc_NNN`
Rust test, a Kani harness or a `compile_fail` doctest; executable semantic claims retain direct
acceptance-criterion trace tags.

The seven Kani harnesses tagged to TC-001 through TC-003 (`verification/kani.rs`) also prove
accounting and exact behaviour; retagging them to the owning subsystem's test cases is a follow-up.

## Evidence Locations

- TC-001: `tests/integration.rs`.
- TC-001 through TC-003: `tests/integration.rs`, `tests/operators.rs`, and seven Kani harnesses. The
  proof scope is bounded: it checks public identity/observation/verdict provenance,
  dispatch/truth-table wiring, independent widened i8 arithmetic oracles,
  symbolic invalid division/remainder, full-width `usize` index definedness, option definedness, and
  the public campaign record/discard paths' five saturating increments plus saturating totals from
  symbolic near-overflow states. It does not prove unlisted module behavior.
- TC-007: the footprint crate's fixed-result test and the linked-footprint and panic-relocation
  measurement run by `make size`.
