---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1, FR-006-AC-6 | TC-016 | 🚧 planned (IR-349 part 1, slice 1): the kernel `Outcome`, `Refusal` and `Undefined` are `quire-exact`'s, and the tests of them left with the runtime's copy; the evidence belongs to the QSL repository |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | 🚧 planned (IR-349 part 1, slice 1): the `ChargePoint` vocabulary and `Meter` charge-before-work accounting are `quire-exact`'s, and the tests of them left with the runtime's copy; the evidence belongs to the QSL repository |
| FR-006 | FR-006-AC-4 | TC-017 | 🚧 planned (IR-349 part 1, slice 1): the kernel `Meter` is `quire-exact`'s, and its tests left with the runtime's copy; the evidence belongs to the QSL repository |
| FR-007 | FR-007-AC-1 | TC-018 | 🚧 planned (IR-349 part 1, slice 1): integer division is `quire-exact`'s, and the allocation-bound test `tests/exact_allocation.rs` left with the runtime's copy |
| FR-007 | FR-007-AC-2 | TC-019 | 🚧 planned (IR-349 part 1, slice 1): exact decimals are `quire-exact`'s, and the allocation-bound test `tests/exact_allocation.rs` left with the runtime's copy |
| FR-007 | FR-007-AC-3 | TC-020 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-4 | TC-021 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-5 | TC-022 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | 🚧 partly evidenced: the runtime-side Debug pins of the types the runtime still defines remain (TC-035); the kernel scalars' pins left with the kernel (IR-349 part 1, slice 1) and the QSL Debug-parity half is removed from this repository, recreation in agent-ix/quire-integration is planned under Linear IR-430 |
| FR-007 | FR-007-AC-7 | TC-023 | 🚧 planned (IR-349 part 1, slice 1): integer, rational, ordering and Boolean operations are `quire-exact`'s, and `tests/exact_arithmetic.rs` left with the runtime's copy |
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-025 | 🚧 sequence and ordered-set cases not yet in tests/exact_collection.rs |
| FR-007 | FR-007-AC-12 | TC-034 | ✅ implemented (the quantity type-fault order, over the runtime's `UnitGraph` and `evaluate_quantity`) |
| FR-007 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11 | TC-034 | 🚧 planned (IR-349 part 1, slice 1): decimal rounding, IEEE exceptional semantics, rational membership and decimal normalization are `quire-exact`'s, and their tests left with the runtime's copy |
| FR-007 | FR-007-AC-13 | TC-035 | 🚧 partly evidenced: the Debug pins of the types the runtime still defines (`CompoundUnit`, `EnumValue`, `ObjectReference`) remain; the pins of the kernel scalars left with the kernel (IR-349 part 1, slice 1) |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`, `src/exact/ieee.rs`) |
| FR-010 | FR-010-AC-1 | TC-031 | ✅ implemented for the charge points the runtime's own operations charge (`function.call`, `equality.*`, `collection.*`, `composite.result-retain`, `enum.*`, `unit.*`); the kernel's own scalar points left with the kernel (IR-349 part 1, slice 1) |
| FR-010 | FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | TC-031 | 🚧 planned (IR-349 part 1, slice 1): the injected-denial seam is the kernel `Meter`'s, so its tests left with the runtime's copy. `quire-exact`'s seam differs from these ACs in two ways the runtime no longer evidences: its `InjectedDenial::occurrence` is a plain `u64`, so FR-010-AC-6's "a zero occurrence cannot be written" no longer holds of the type; and its denial is not cleared when it fires, so a later charge at the same point is denied again until a charge is admitted there, which FR-010-AC-5 forbids. The runtime's own check of AC-5 at both `check_injected` call sites (`Meter::charge`, `Meter::charge_plan`) failed against `quire-exact` and was removed with the kernel tests; the divergence is for QSL to settle |
| FR-011 | FR-011-AC-2, FR-011-AC-6, FR-011-AC-8 | TC-032 | ✅ implemented (the quantity and short-circuit cases over the runtime's `evaluate_quantity` and `evaluate_boolean_short_circuit`) |
| FR-011 | FR-011-AC-3, FR-011-AC-4, FR-011-AC-7 | TC-032 | 🚧 partly evidenced: the stop-carrying connective's single `boolean.result-retain`, the `unit.identity-read` scan order and the unit-graph and compound-unit ordering rules remain; the kernel's `evaluate_boolean` retention, its scan order over integer, decimal and division counters and the IEEE flag order left with the kernel (IR-349 part 1, slice 1) |
| FR-011 | FR-011-AC-1, FR-011-AC-5 | TC-032 | 🚧 planned (IR-349 part 1, slice 1): the kernel `Meter`'s retained state at a stop and its charge log are `quire-exact`'s, so their tests left with the runtime's copy. `quire-exact` has no `CHARGE_LOG_CAPACITY` and no `charge_log_truncated`: a production meter keeps a count and the ordered log exists only under its `test-support` feature, unbounded, so FR-011-AC-5's cap of 4096 no longer describes the meter the runtime consumes |
| FR-012 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033 | ✅ implemented |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented: AC-1's linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable` (a package `check` rejects under `CheckMode::Kernel` is never applicable — that is AC-6's inspection too) together with `tc_194_linked_package_applies_every_declared_function` and the rest of this corpus's `Linked`-application tests, which call only a package `check` admitted under `CheckMode::Linked`. AC-2/AC-3's "arity before any per-argument check, all before the `function.call` charge, before the body" ordering is covered for both `call` and `Frame::call` |
| FR-273 | FR-273-AC-7 | TC-194 | ✅ implemented: re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by `CheckingLimits::depth` on a shared counter — not only `Frame::call` — including the direct-re-entry attack a body holding its own `Rc<CheckedPackage>` could otherwise use to bypass it, proved by `tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal`, `tc_194_direct_reentrant_package_call_is_bounded_like_frame_call` and `tc_194_checking_limits_refuses_a_depth_above_the_maximum`. The bound is per-`CheckedPackage`, not universal: a host body that builds a *fresh* `CheckedPackage` at each hop gets a fresh budget and can still overflow the host stack — but so does a body that recurses without touching this crate's runtime at all, since under AD-002 a body is arbitrary host Rust and its own stack usage is the host's concern, not this crate's |
| FR-273 | FR-273-AC-5 | TC-194 | 🚧 partly evidenced: the QSL shared-corpus half is removed from this repository, recreation in agent-ix/quire-integration is planned under Linear IR-430; the runtime-only ordering tests remain. AC-5 quantifies over shared-corpus function-application vectors only, and the shared corpus agrees on all five of them — the closed `InputRefusal` vocabulary (with codes and causes), the charge count of one admitted call, and — via AP01–AP04's `charges == 0` assertions on each refusal path (the removed QSL shared-corpus vectors; see Evidence Locations) — that every refusal precedes the `function.call` charge, agreed on both sides. Relative order *among* the four checks themselves (arity, value kind, dangling reference, unknown function) is not something any vector needs to discriminate for AC-5 to be met, since each corpus vector isolates exactly one violation by design; that ordering is instead verified by the runtime-only tests in `tests/exact_function_application.rs` (see Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared corpus either, for the same reason: AC-5 does not claim them. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 planned (Linear IR-349; AC-16 and AC-18 also need QSL-358 phase 2): the runtime still holds part of the kernel copy (its `Value` and `ValueType`, collections, `NodeKey`, `ObjectReference`, quantity and equality) and the residue. The IR-349 foundation slice (floor, dependency, bans, one copy; it deletes nothing) made AC-3 to AC-6 true (the optional `quire-exact` git dependency at `branch = "main"`, one lock entry, `make deny`'s one-copy check); IR-349 part 1, slice 1 deleted the scalars, `Meter` and `Outcome`; AC-1, AC-2, AC-12, AC-13 and AC-17 need the rest of the copy deleted |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | ✅ implemented (IR-349 foundation slice; evidence is a gate script, not a `tc_NNN` test): `deny.toml` bans every listed QSL crate, and `make deny-mutations` (`scripts/check_deny_bans.sh`) adds `qsl-eval`, `qsl-replay`, `qsl-semantics` and `quire-spec-language` as normal and as dev dependencies in a scratch copy and requires cargo-deny's `banned` error for each |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | ✅ implemented (IR-349 foundation slice; evidence is gate targets, not a `tc_NNN` test): `make test-features` row `build-exact-no-std-msrv` builds `exact` without `std` for `thumbv7em-none-eabi` on 1.98.1 with `quire-exact` in the graph; `make msrv` and `make size` run on 1.98.1; `make size` fails when the footprint graph holds `quire-exact` and when the linked size leaves the band |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3, FR-006-AC-6 | 🚧 planned (IR-349 part 1, slice 1): the kernel `Outcome`, `Refusal`, `Undefined` and `ChargePoint` are `quire-exact`'s, and the tests of them left with the runtime's copy |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | 🚧 planned (IR-349 part 1, slice 1): the kernel `Meter` is `quire-exact`'s, and the tests of it left with the runtime's copy |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 planned (IR-349 part 1, slice 1): `tests/exact_allocation.rs` (steps 4-5) left with the kernel's integer division; steps 1-3, the QSL agreement half, are removed from this repository, recreation in agent-ix/quire-integration is planned under Linear IR-430 |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 planned (IR-349 part 1, slice 1): `tests/exact_allocation.rs` (step 5) left with the kernel's decimals; steps 1-3, the QSL agreement half, are removed from this repository, recreation in agent-ix/quire-integration is planned under Linear IR-430 |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-430): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | 🚧 planned (IR-349 part 1, slice 1): the operations are `quire-exact`'s, and `tests/exact_arithmetic.rs` left with the runtime's copy |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9 | ✅ implemented |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | 🚧 steps 1–2 and 4–7 implemented in `tests/exact_collection.rs`; steps 3 and 5 cover the set and bag only, so FR-008-AC-10 through FR-008-AC-12's sequence and ordered-set cases are not yet tested |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | ✅ implemented |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | 🚧 partly evidenced: AC-1 for the charge points the runtime's own operations charge (`tests/exact_outcomes.rs`); AC-2 to AC-6 planned (IR-349 part 1, slice 1), the seam being the kernel `Meter`'s, with the two divergences recorded in the coverage table above |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | 🚧 partly evidenced: the quantity, short-circuit and unit-graph cases remain (`tests/exact_meter_state.rs`); AC-1 and AC-5 and the kernel halves of AC-3, AC-4 and AC-7 planned (IR-349 part 1, slice 1) |
| TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | ✅ implemented |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | 🚧 partly evidenced: AC-12 (quantity type-fault order) remains; AC-8 to AC-11 planned (IR-349 part 1, slice 1), their subjects being `quire-exact`'s |
| TC-035 | Pin boxed value and type structs' hand-written Debug rendering | Unit | P1 | FR-007-AC-13 | 🚧 partly evidenced: the pins of `CompoundUnit`, `EnumValue` and `ObjectReference` remain; the kernel scalars' pins left with the kernel (IR-349 part 1, slice 1) |
| TC-194 | Apply checked functions totally, before any charge | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7 | ✅ implemented |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | 🚧 planned (Linear IR-349): the kernel copy is deleted in part 1 (in slices; slice 1 deleted the scalars, `Meter` and `Outcome`) and the residue in part 2, after QSL-358 phase 2 |
| TC-198 | Fail the build on a dependency on a guarded QSL crate | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | ✅ implemented (IR-349 foundation slice): `deny.toml` entries and `make deny-mutations` |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | ✅ implemented (IR-349 foundation slice): the exact profile builds on 1.98.1 with `quire-exact` in the graph and the footprint graph holds none; the runtime's own copy is still deleted by later parts |

FR-009-AC-5 and FR-273-AC-4 are verified by `compile_fail` doctests, and TC-198 and
TC-199 by gate targets (`make deny-mutations`, `make test-features`, `make msrv`, `make size`;
see Evidence Locations). Every other row is backed by a `tc_NNN` Rust test; executable semantic
claims retain direct acceptance-criterion trace tags. Rows marked planned or partly evidenced above
are the exceptions. FR-010-AC-6's `compile_fail` doctest was on the runtime's `InjectedDenial`,
which is `quire-exact`'s now (IR-349 part 1, slice 1), so that AC has no evidence here.

`FR-010-AC-5` had evidence at both `check_injected` call sites, `Meter::charge` and
`Meter::charge_plan`, in the runtime's copy of the `Meter`. That copy is deleted, and the
`charge_plan` tests failed against `quire-exact`'s `Meter`, whose injected denial is not single-shot
(see the FR-010 row above), so the row is planned and the divergence is QSL's to settle.

## Evidence at the kernel move

FR-275 moves the exact value kernel to `quire-exact` and deletes the runtime's copy in two steps
(code steps IR-349 part 1 and part 2, after the foundation slice that adds the dependency, the
bans and the one-copy check and deletes nothing). Nothing in this matrix is deleted by either step
(FR-275-AC-14). Until IR-349 lands every row above keeps its status; when the files below are
deleted, IR-349 sets each affected row to planned with the reason in the last column, never removes
it. A leaving row will never be backed by RT: its evidence lives in the QSL repository. The matrix
status vocabulary in use here has no "verified upstream" status, and none is invented; the row
stays "planned" with the reason stated, and whether the vocabulary should gain one is a planner
question (raised in the PR). The runtime keeps no substitute test: no agreement test and no vendored vectors, and no test that
calls a QSL crate other than `quire-exact` (FR-275-AC-12). A test of an item `quire-exact` exports
leaves in step 1. A test of an interim-residue item stays only until QSL-358 phase 2 is merged
(temporary exception, owner approved 2026-10-01, see FR-275) and then leaves with the code to QSL
(FR-275-AC-16); the residue is not runtime-owned.

| Test file | Test cases | Disposition | Reason |
|---|---|---|---|
| `tests/exact_function_application.rs` | TC-194, TC-195 | stays until QSL-358 phase 2 | interim residue (FR-273, AD-002): the tests run over the `quire-exact` `Value`, `Meter` and `Outcome`, then leave with the code; rows stay planned until QSL's evidence exists |
| `tests/exact_negotiation.rs` | TC-030 | stays | the `negotiate_*` predicates are runtime-owned, not a QSL port (FR-009, FR-275) |
| `tests/exact_vocabulary.rs` | TC-033 | stays until QSL-358 phase 2 | interim residue unless `quire-exact` exports the vocabulary (FR-012) |
| `tests/exact_equality.rs` | TC-026 | stays until QSL-358 phase 2 | interim residue: `CheckedEquality` and the checking environment; its equality-plan assertions on a kernel item leave in step 1, with the slice that deletes the runtime's `Value` (IR-349 part 1, slice 1 left the file whole: its subject, the runtime's own `Value`, is still defined) |
| `tests/exact_composite.rs` | TC-024 | splits | construction of kernel `Value`s leaves in step 1, with the slice that deletes the runtime's `Value`; declaration-environment checks stay until QSL-358 phase 2 (slice 1 left the file whole for the same reason) |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018, TC-019, TC-023 | left (IR-349 part 1, slice 1) | the files test kernel behaviour (they also use `CHARGE_LOG_CAPACITY`, which `quire-exact` does not export, so that constant's assertions go with the kernel's accounting evidence): evidence belongs to the QSL repository, which this repository does not track |
| `tests/exact_outcomes.rs` | TC-016, TC-017, TC-031 | split (IR-349 part 1, slice 1) | the kernel `Meter`, `Outcome` and injected-denial tests left; the cases that use `TypeEnvironment`, `CheckedEquality`, `CheckedPackage`, `ObjectEnvironment`, `UnitGraph`, `EnumDeclaration` and `evaluate_quantity`, which `quire-exact` does not export, stay until QSL-358 phase 2 |
| `tests/exact_collection.rs` | TC-025 | splits | the collection-algebra and canonical-key cases leave in step 1, with the slice that deletes the runtime's collections; the cases built on `TypeEnvironment` and `CompositeDeclaration` stay until QSL-358 phase 2 (slice 1 left the file whole: its subject, the runtime's own collections, is still defined) |
| `tests/exact_meter_state.rs` | TC-032 | split (IR-349 part 1, slice 1) | `UnitGraph::admit`, `CompoundUnit`, `evaluate_quantity` and `evaluate_boolean_short_circuit` cases (FR-011-AC-7) stay until QSL-358 phase 2; the rest left |
| `tests/exact_semantics.rs` | TC-034 | split (IR-349 part 1, slice 1) | the `UnitGraph`, `Dimension` and `evaluate_quantity` cases stay until QSL-358 phase 2; the rest left |
| `tests/exact_debug_parity.rs` | TC-035 | split (IR-349 part 1, slice 1) | the `CompoundUnit`, `Dimension`, `EnumDeclaration` and `ObjectReference` Debug pins stay (the last until the slice that deletes the runtime's `ObjectReference`); the rest left |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031, TC-032 | left (IR-349 part 1, slice 1) | in-crate tests of kernel `Meter` and `Integer` internals the runtime no longer defines |

The runtime keeps, in the end state, only the consumption checks TC-197 to TC-199: that it uses the
QSL kernel, holds no ported code and guards its edges.

## Evidence Locations

- TC-198: `deny.toml` (the `[bans]` list and `[graph] all-features = true`) and
  `scripts/check_deny_bans.sh`, run by `make deny-mutations`, which adds each banned crate to a
  scratch copy of the tracked files and requires cargo-deny's `banned` error. TC-199:
  `scripts/run_feature_matrix.py` (row `build-exact-no-std-msrv`, `make test-features`),
  `make msrv` and `make size` (linked band, panic relocations and the footprint-graph check that
  `quire-exact` is absent). Neither has a Rust test.
- TC-194, TC-195: `tests/exact_function_application.rs` (`--features exact`), landed under
  agent-ix/quire-contract-runtime#34. FR-273-AC-4's evidence is a `compile_fail` doctest on
  `IeeeDisposition` (`src/exact/ieee.rs`).
- FR-273-AC-5 (TC-194's shared-corpus row): the QSL agreement oracle is removed from this repository;
  recreating it in agent-ix/quire-integration is planned under Linear IR-430. The removed oracle agreed on the closed `InputRefusal` vocabulary (`UnknownFunction`, `Arity`, `WrongValueKind`,
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
- TC-030: `tests/exact_negotiation.rs`; TC-032: `tests/exact_meter_state.rs` (the in-crate
  `src/exact_accounting_tests.rs` left with the kernel `Meter`);
  TC-033: `tests/exact_vocabulary.rs`; TC-034: `tests/exact_semantics.rs`; TC-035:
  `tests/exact_debug_parity.rs`.
- TC-031: `tests/exact_outcomes.rs` (the sweep over the charge points the runtime's own operations
  charge, `--features exact`). TC-016, TC-017, TC-023 and the allocation bounds of TC-018 and
  TC-019 have no evidence here since IR-349 part 1, slice 1: `tests/exact_arithmetic.rs`,
  `tests/exact_allocation.rs` and the in-crate accounting tests left with the kernel. The
  `compile_fail` doctest on the runtime's `InjectedDenial` (FR-010-AC-6) left with the type.
- TC-018 through TC-022: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned under Linear IR-430. No step of TC-018/TC-019 keeps evidence here since IR-349 part 1, slice 1.
- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`; TC-026:
  `tests/exact_equality.rs`. All run with `--features exact`.
