---
id: TM-001
title: "Contract runtime v0.1 test matrix"
type: TestMatrix
---

# Contract runtime v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-2 | TC-001 | ✅ Complete |
| FR-001 | FR-001-AC-3 | TC-008 | ✅ Complete |
| FR-001 | FR-001-AC-4 | TC-001 | ✅ Complete |
| FR-001 | FR-001-AC-5 | TC-008 | ✅ Complete |
| FR-002 | FR-002-AC-1, FR-002-AC-2 | TC-002 | ✅ Complete |
| FR-002 | FR-002-AC-3 | TC-003 | ✅ Complete |
| FR-002 | FR-002-AC-4 | TC-003 | ✅ Complete |
| FR-003 | FR-003-AC-1 | TC-004 | ✅ Complete |
| FR-003 | FR-003-AC-2 | TC-005 | ✅ Complete |
| FR-003 | FR-003-AC-3 | TC-004 | ✅ Complete |
| FR-004 | FR-004-AC-1, FR-004-AC-2 | TC-006 | ✅ Complete |
| FR-004 | FR-004-AC-3 | TC-008 | ✅ Complete |
| FR-004 | FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7 | TC-015 | ✅ implemented |
| FR-004 | FR-004-AC-8 | TC-006 | ✅ Complete |
| FR-005 | FR-005-AC-1 | TC-009 | ✅ Complete |
| FR-005 | FR-005-AC-2 | TC-010 | ✅ Complete |
| FR-005 | FR-005-AC-3 | TC-011 | ✅ Complete |
| FR-005 | FR-005-AC-5 | TC-013 | ✅ Complete |
| FR-005 | FR-005-AC-6 | TC-014 | ✅ Complete |
| FR-006 | FR-006-AC-1, FR-006-AC-5, FR-006-AC-6 | TC-016 | ✅ implemented |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | ✅ implemented |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | ✅ implemented |
| FR-006 | FR-006-AC-4 | TC-017 | ✅ implemented |
| FR-007 | FR-007-AC-1 | TC-018 | ✅ implemented |
| FR-007 | FR-007-AC-2 | TC-019 | ✅ implemented |
| FR-007 | FR-007-AC-3 | TC-020 | ✅ implemented |
| FR-007 | FR-007-AC-4 | TC-021 | ✅ implemented |
| FR-007 | FR-007-AC-5 | TC-022 | ✅ implemented |
| FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | ✅ implemented |
| FR-007 | FR-007-AC-7 | TC-023 | ✅ implemented |
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented |
| FR-007 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | TC-034 | ✅ implemented |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5 | TC-030 | ✅ implemented |
| FR-010 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | TC-031 | ✅ implemented |
| FR-011 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | TC-032 | ✅ implemented |
| FR-012 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5, FR-012-AC-6 | TC-033 | ✅ implemented |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented: AC-1's linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable` (a package `check` rejects under `CheckMode::Kernel` is never applicable — that is AC-6's inspection too) together with `tc_194_linked_package_applies_every_declared_function` and the rest of this corpus's `Linked`-application tests, which call only a package `check` admitted under `CheckMode::Linked`. AC-2/AC-3's "arity before any per-argument check, all before the `function.call` charge, before the body" ordering is covered for both `call` and `Frame::call` |
| FR-273 | FR-273-AC-7 | TC-194 | ✅ implemented: re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by `CheckingLimits::depth` on a shared counter — not only `Frame::call` — including the direct-re-entry attack a body holding its own `Rc<CheckedPackage>` could otherwise use to bypass it, proved by `tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal`, `tc_194_direct_reentrant_package_call_is_bounded_like_frame_call` and `tc_194_checking_limits_refuses_a_depth_above_the_maximum`. The bound is per-`CheckedPackage`, not universal: a host body that builds a *fresh* `CheckedPackage` at each hop gets a fresh budget and can still overflow the host stack — but so does a body that recurses without touching this crate's runtime at all, since under AD-002 a body is arbitrary host Rust and its own stack usage is the host's concern, not this crate's |
| FR-273 | FR-273-AC-5 | TC-194 | ✅ implemented: AC-5 quantifies over shared-corpus function-application vectors only, and the shared corpus agrees on all five of them — the closed `InputRefusal` vocabulary (with codes and causes), the charge count of one admitted call, and — via AP01–AP04's `charges == 0` assertions on each refusal path (`conformance/qsl-agreement/tests/tc_191_function_application.rs:98` and the matching lines in the other three vectors) — that every refusal precedes the `function.call` charge, agreed on both sides. Relative order *among* the four checks themselves (arity, value kind, dangling reference, unknown function) is not something any vector needs to discriminate for AC-5 to be met, since each corpus vector isolates exactly one violation by design; that ordering is instead verified by the runtime-only tests in `tests/exact_function_application.rs` (see Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared corpus either, for the same reason: AC-5 does not claim them. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Preserve verdict and observation identity | Unit | P0 | FR-001-AC-1, FR-001-AC-2, FR-001-AC-4 | ✅ Complete |
| TC-002 | Exercise Boolean evaluation contracts | Unit | P0 | FR-002-AC-1, FR-002-AC-2 | ✅ Complete |
| TC-003 | Check definedness boundaries | Property | P0 | FR-002-AC-3, FR-002-AC-4, NFR-002-AC-1 | ✅ Complete |
| TC-004 | Preserve proptest tri-state mapping | Unit | P0 | FR-003-AC-1, FR-003-AC-3 | ✅ Complete |
| TC-005 | Resolve and build every supported feature profile | Inspection | P0 | FR-003-AC-2, NFR-001-AC-1 | ✅ Complete |
| TC-006 | Retain complete campaign accounting | Unit | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-8 | ✅ Complete |
| TC-007 | Audit runtime footprint and packaging policy | Inspection | P0 | NFR-001-AC-2, NFR-001-AC-3, NFR-002-AC-2 | ✅ Complete |
| TC-008 | Inspect provenance-bearing public model | Inspection | P0 | FR-001-AC-3, FR-001-AC-5, FR-004-AC-3, NFR-002-AC-3 | ✅ Complete |
| TC-009 | Classify every shared pin through the packaged compatibility matrix | Integration | P0 | FR-005-AC-1 | ✅ Complete |
| TC-010 | Reach Quoin through the declared adapter with no producer executed | Integration | P0 | FR-005-AC-2 | ✅ Complete |
| TC-011 | Bind the sealed record's impact snapshot to the Quire static export | Integration | P0 | FR-005-AC-3 | ✅ Complete |
| TC-013 | Demonstrate all twelve outcomes and pair every negative with a positive control | Integration | P0 | FR-005-AC-5, NFR-002-AC-3 | ✅ Complete |
| TC-014 | Prove no generic evidence machinery remains | Integration | P0 | FR-005-AC-6 | ✅ Complete |
| TC-015 | Bound immutable campaign snapshot transport | Unit | P0 | FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7 | ✅ implemented |
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3, FR-006-AC-5, FR-006-AC-6 | ✅ implemented |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | ✅ implemented |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | ✅ implemented |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | ✅ implemented |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | ✅ implemented |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | ✅ implemented |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | ✅ implemented |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | ✅ implemented |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9 | ✅ implemented |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9 | ✅ implemented |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | ✅ implemented |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | ✅ implemented |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | ✅ implemented |
| TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5, FR-012-AC-6 | ✅ implemented |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | ✅ implemented |
| TC-194 | Apply checked functions totally, before any charge | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7 | ✅ implemented |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4 | ✅ implemented |

Inspection-class TC-005, TC-007, and TC-008 combine self-identifying Rust source-policy tests with
retained build, compile-fail, or audit outputs. Every test-matrix row now has a `tc_NNN` Rust test
binding; executable semantic claims retain direct acceptance-criterion trace tags.

`FR-010-AC-5` is verified at both `check_injected` call sites: `Meter::charge`
(`tc_031_further_charges_after_the_injected_denial_meter_normally`,
`tc_031_work_accounting_is_correct_before_and_after_the_injected_denial`) and
`Meter::charge_plan`
(`tc_031_further_charge_plan_calls_after_the_injected_denial_meter_normally`,
`tc_031_charge_plan_reservation_is_unaffected_by_the_injected_denial`).

## Evidence Locations

- TC-194, TC-195: `tests/exact_function_application.rs` (`--features exact`), landed under
  agent-ix/quire-contract-runtime#34. FR-273-AC-4's evidence is a `compile_fail` doctest on
  `IeeeDisposition` (`src/exact/ieee.rs`), mirroring `InjectedDenial`'s.
- FR-273-AC-5 (TC-194's shared-corpus row): `conformance/qsl-agreement/tests/tc_191_function_application.rs`,
  pinned to the quire-spec-language `ea39f91` authority (`conformance/qsl-agreement/Cargo.toml`).
  It agrees on the closed `InputRefusal` vocabulary (`UnknownFunction`, `Arity`, `WrongValueKind`,
  `DanglingReference`) with its codes and causes, and the charge count of one admitted call (AP05).
  It does **not** agree on check *ordering*: each of its five vectors (AP01 through AP05) triggers
  exactly one refusal in isolation — no vector supplies a call violating two checks at once — so the
  corpus cannot distinguish an implementation that checks arity, then per-argument kind and
  reference, then charges `function.call`, from one that checks in some other order and happens to
  agree on each single-violation vector's result. That ordering claim is instead backed only by the
  runtime-only tests in `tests/exact_function_application.rs`
  (`tc_194_arity_is_decided_before_any_per_argument_check`,
  `tc_194_earlier_parameter_refusal_wins_over_a_later_dangling_reference`,
  `tc_194_function_call_precedes_the_body`,
  `tc_194_frame_call_charges_function_call_before_the_body_it_invokes`), which construct vectors
  that do carry two simultaneous violations specifically to discriminate check order. Nor does the
  corpus agree on function-body semantics: AD-002 draws the runtime's boundary at typed values and
  operators, so `Body` is an opaque Rust closure while the authority's function bodies are a typed
  `Expression` AST an interpreter runs — there is no `Debug` rendering that could compare the two,
  so no shared corpus exists for arithmetic, `let`, `if`, recursion or any other body form. Those
  are covered by `tests/exact_function_application.rs` against the runtime alone. AC-5 quantifies
  over shared-corpus function-application vectors only, so neither the check-ordering gap nor the
  absent body-semantics corpus is a gap in AC-5 itself — the corpus agrees on every vector it
  supplies, which is all AC-5 claims — and AC-5 is recorded as fully implemented.
- TC-030: `tests/exact_negotiation.rs`; TC-032: `tests/exact_meter_state.rs` and the in-crate
  `src/exact/accounting_tests.rs` for the cumulative-counter boundary no public operator can
  reach;
  TC-033: `tests/exact_vocabulary.rs`; TC-034: `tests/exact_semantics.rs`.
- TC-016, TC-017, TC-031: `tests/exact_outcomes.rs`; TC-023: `tests/exact_arithmetic.rs`; allocation bounds for
  TC-016, TC-018, TC-019 and TC-023: `tests/exact_allocation.rs`. All run with
  `--features exact`. FR-010-AC-6's evidence is a `compile_fail` doctest on `InjectedDenial`
  (`src/exact/accounting.rs`): `occurrence: 0` does not compile, so the malformed request cannot be
  written.
- TC-018 through TC-022: `conformance/qsl-agreement/tests/`, run by `make conformance` against
  quire-spec-language d01371b9, with charges checked against agent-ix/quire-specification@7d7943a.
- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`; TC-026:
  `tests/exact_equality.rs`. All run with `--features exact` against quire-spec-language d01371b9.

- TC-015: `tests/snapshot.rs`, private near-limit accounting tests, compile-fail API docs,
  and an isolated native memory-ceiling control. REV-009 records independent acceptance of
  the bounded transport; existing Kani evidence does not cover the parser, and exact shared
  stack/full release qualification remains separate.

- TC-001 and TC-006: `tests/integration.rs`.
- TC-001 through TC-003: `tests/integration.rs`, `tests/operators.rs`, and seven Kani harnesses. The
  proof scope is bounded: it checks public identity/observation/verdict provenance,
  dispatch/truth-table wiring, independent widened i8 arithmetic oracles,
  symbolic invalid division/remainder, full-width `usize` index definedness, option definedness, and
  the public campaign record/discard paths' five saturating increments plus saturating totals from
  symbolic near-overflow states. `make kani-census` enforces the seven proof names and trace markers.
  It does not prove unlisted module behavior.
- TC-004: `tests/proptest_adapter.rs`.
- TC-005: `tests/release_contract.rs`, compile-fail crate documentation, `make test-features`, and the
  retained default dependency record.
- TC-007: `tests/release_contract.rs`, the footprint crate's fixed-result test, plus the governed
  linked-footprint and panic-relocation measurement published by
  `scripts/measure_footprint.py`, and the unsafe, panic-surface, and license audits.
- TC-008: `tests/release_contract.rs` recursively scans all shipped runtime source, supplemented by
  five compile-fail enum doctests and public API documentation.
- TC-009 through TC-011, TC-013 and TC-014: `tests/shared_assurance.rs`, which invokes the shared-assurance gates
  rather than reimplementing them. A test that recomputes what a gate computes is a second
  implementation that can agree with itself while both are wrong.
