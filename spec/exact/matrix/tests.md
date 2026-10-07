---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1, FR-006-AC-6 | TC-016 | ✅ implemented |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | ✅ implemented |
| FR-006 | FR-006-AC-4 | TC-017 | ✅ implemented |
| FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |
| FR-007 | FR-007-AC-2 | TC-019 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |
| FR-007 | FR-007-AC-3 | TC-020 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-4 | TC-021 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-5 | TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | 🚧 partly evidenced: the runtime-side Debug pins remain (TC-035); the QSL Debug-parity half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done) |
| FR-007 | FR-007-AC-7 | TC-023 | ✅ implemented |
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-13 | TC-024 | ✅ implemented: deep metadata clone, equality, formatting and drop in a small-stack thread and default-stack child process |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-025 | 🚧 sequence and ordered-set cases not yet in tests/exact_collection.rs |
| FR-007 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | TC-034 | ✅ implemented |
| FR-007 | FR-007-AC-13 | TC-035 | ✅ implemented |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`, `src/exact/ieee.rs`) |
| FR-010 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | TC-031 | ✅ implemented |
| FR-011 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | TC-032 | ✅ implemented |
| FR-012 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033 | ✅ implemented |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented: AC-1's linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable` (a package `check` rejects under `CheckMode::Kernel` is never applicable — that is AC-6's inspection too) together with `tc_194_linked_package_applies_every_declared_function` and the rest of this corpus's `Linked`-application tests, which call only a package `check` admitted under `CheckMode::Linked`. AC-2/AC-3's "arity before any per-argument check, all before the `function.call` charge, before the body" ordering is covered for both `call` and `Frame::call` |
| FR-273 | FR-273-AC-7 | TC-194 | ✅ implemented: re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by `CheckingLimits::depth` on a shared counter — not only `Frame::call` — including the direct-re-entry attack a body holding its own `Rc<CheckedPackage>` could otherwise use to bypass it, proved by `tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal`, `tc_194_direct_reentrant_package_call_is_bounded_like_frame_call` and `tc_194_checking_limits_refuses_a_depth_above_the_maximum`. The bound is per-`CheckedPackage`, not universal: a host body that builds a *fresh* `CheckedPackage` at each hop gets a fresh budget and can still overflow the host stack — but so does a body that recurses without touching this crate's runtime at all, since under AD-002 a body is arbitrary host Rust and its own stack usage is the host's concern, not this crate's |
| FR-273 | FR-273-AC-5 | TC-194 | 🚧 partly evidenced: the QSL shared-corpus half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done); the runtime-only ordering tests remain. AC-5 quantifies over shared-corpus function-application vectors only, and the removed shared corpus historically agreed on all five of them — the closed `InputRefusal` vocabulary (with codes and causes), the charge count of one admitted call, and — via AP01–AP04's `charges == 0` assertions on each refusal path (the removed QSL shared-corpus vectors; see Evidence Locations) — that every refusal precedes the `function.call` charge, agreed on both sides. Relative order *among* the four checks themselves (arity, value kind, dangling reference, unknown function) is not something any vector needs to discriminate for AC-5 to be met, since each corpus vector isolates exactly one violation by design; that ordering is instead verified by the runtime-only tests in `tests/exact_function_application.rs` (see Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared corpus either, for the same reason: AC-5 does not claim them. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 planned (Linear IR-349; AC-16 and AC-18 are unmet while the residue exists, and the residue is not authorized: no exception, no expiry, no approval; AC-17 can be checked today): the runtime still holds its own copy of the kernel and of the residue. The IR-349 foundation slice (floor, dependency, bans, one copy; it deletes nothing) made AC-3 to AC-6 true (the optional `quire-exact` git dependency at `branch = "main"`, one lock entry, `make deny`'s one-copy check); AC-1, AC-2, AC-12 and AC-13 need the copy deleted; interface-001-AC-7 is likewise unmet while the runtime's copy of `Frame`, `Body`, `CheckedPackage`, `Evaluation` and `plan_call` exists. AC-1 and AC-16 now assert the end state of no `exact` module and no re-export (AD-004 step 1) and are unmet today, planned until the IR-349 code steps land; AC-17 restates its exception as the `scalar` items and stays checkable today. Per interface-001 criterion, see the interface-001 rows in the core matrix |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | ✅ implemented (IR-349 foundation slice; evidence is a gate script, not a `tc_NNN` test): `deny.toml` bans every listed QSL crate, and `make deny-mutations` (`scripts/check_deny_bans.sh`) adds `qsl-eval`, `qsl-replay`, `qsl-semantics` and `quire-spec-language` as normal and as dev dependencies in a scratch copy and requires cargo-deny's `banned` error for each |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | ✅ implemented (IR-349 foundation slice; evidence is gate targets, not a `tc_NNN` test): `make test-features` row `build-exact-no-std-msrv` builds `exact` without `std` for `thumbv7em-none-eabi` on 1.98.1 with `quire-exact` in the graph; `make msrv` and `make size` run on 1.98.1; `make size` fails when the footprint graph holds `quire-exact` and when the linked size leaves the band |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3, FR-006-AC-6 | ✅ implemented |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | ✅ implemented |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | ✅ implemented |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-13 | ✅ implemented |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | 🚧 steps 1–2 and 4–7 implemented in `tests/exact_collection.rs`; steps 3 and 5 cover the set and bag only, so FR-008-AC-10 through FR-008-AC-12's sequence and ordered-set cases are not yet tested |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | ✅ implemented |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | ✅ implemented |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | ✅ implemented |
| TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | ✅ implemented |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | ✅ implemented |
| TC-035 | Pin boxed value and type structs' hand-written Debug rendering | Unit | P1 | FR-007-AC-13 | ✅ implemented |
| TC-194 | Apply checked functions totally, before any charge | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7 | ✅ implemented |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | 🚧 planned (Linear IR-349): the kernel copy is deleted in part 1 and the residue in part 2 (remaining RT deletion owned by open IR-349 step 2 and IR-583; QSL-358 extraction is Done); step 7 fails while the residue exists |
| TC-198 | Fail the build on a dependency on a guarded QSL crate | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | ✅ implemented (IR-349 foundation slice): `deny.toml` entries and `make deny-mutations` |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | ✅ implemented (IR-349 foundation slice): the exact profile builds on 1.98.1 with `quire-exact` in the graph and the footprint graph holds none; the runtime's own copy is still deleted by later parts |

FR-009-AC-5, FR-010-AC-6 and FR-273-AC-4 are verified by `compile_fail` doctests, and TC-198 and
TC-199 by gate targets (`make deny-mutations`, `make test-features`, `make msrv`, `make size`;
see Evidence Locations). Every other row is backed by a `tc_NNN` Rust test; executable semantic
claims retain direct acceptance-criterion trace tags. Rows marked planned or partly evidenced above
are the exceptions.

`FR-010-AC-5` is verified at both `check_injected` call sites: `Meter::charge`
(`tc_031_further_charges_after_the_injected_denial_meter_normally`,
`tc_031_work_accounting_is_correct_before_and_after_the_injected_denial`) and
`Meter::charge_plan`
(`tc_031_further_charge_plan_calls_after_the_injected_denial_meter_normally`,
`tc_031_charge_plan_reservation_is_unaffected_by_the_injected_denial`).

## Evidence at the kernel move

FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in
IR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2 and backlog IR-583.
QSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:
all current test files and production definitions remain present. The status columns describe
current evidence only. TC-016/017/023 and their local kernel criteria remain implemented;
TC-018/019 and FR-007-AC-1/2 are partly evidenced because their allocation checks exist but
the shared-corpus agreement oracle is absent. Future deletion does not change those statuses
before the code PR removes the tests.
The existing vocabulary has no "verified upstream" status; no such status or upstream pass is
claimed. Requirements, criteria and TC IDs remain, including obligations not yet mapped to an
exact owner criterion. The computed `quire matrix` still finds current local trace binders;
`tagged` reports their presence, not a semantic pass or future ownership. TC-032 is not a
binder for FR-007-AC-7: its retained lazy connective evidence is directly tagged only to
FR-011-AC-3/AC-8, which own the permanent lazy behavior. FR-007-AC-7 owns kernel
operations and keeps TC-023's current evidence disposition; it has no permanent lazy clause.

When code PR #95 removes the local kernel tests, its matrix update shall mark the affected
rows planned with a stated reason under FR-275-AC-15, keeping any remaining partial evidence
explicit. The file dispositions below describe that future state; no deletion or upstream
verification is claimed by this amendment.

The owner mappings in TC-016/017/018/019/023 identify only matching contracts in
`ix://agent-ix/quire-exact`. RT shall retain no substitute kernel tests, copied vectors or
agreement tests (FR-275-AC-12). The lazy connective in TC-032 is RT-owned behavior, with
FR-011-AC-3/AC-8 evidence retained; already-decided Boolean truth tables are kernel-owned.
QSL-358 is Done (QSL-side extraction); open IR-349 step 2 and IR-583 own remaining RT residue deletion.
IR-430 is Done (RT agreement-test removal). The remaining QSL agreement oracle in
`quire-integration` has open quire-integration ticket IR-669; an upstream
arithmetic or allocation test does not close that gap. Kernel ownership mappings leave both workstreams distinct.

| Test file | Test cases | Disposition | Reason |
|---|---|---|---|
| `tests/exact_function_application.rs` | TC-194, TC-195 | splits | the TC-194 function-application tests are residue (FR-273, AD-002; not authorized): they run over the `quire-exact` `Value`, `Meter` and `Outcome`, then leave with the code; rows stay planned until QSL's evidence exists. The TC-195 tests exercise `negotiate_ieee`, which is runtime-owned and stays (FR-275-AC-19, FR-009-AC-5) |
| `tests/exact_negotiation.rs` | TC-030 | stays | the `negotiate_*` predicates are runtime-owned, not a QSL port (FR-009, FR-275) |
| `tests/exact_vocabulary.rs` | TC-033 | leaves with the residue | residue unless `quire-exact` exports the vocabulary (FR-012) |
| `tests/exact_equality.rs` | TC-026 | leaves with the residue | residue: `CheckedEquality` and the checking environment; its equality-plan assertions on a kernel item leave in step 1 |
| `tests/exact_composite.rs` | TC-024 | splits | construction of kernel `Value`s leaves in step 1; declaration-environment checks leave with the residue |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018, TC-019, TC-023 | leaves in step 1 | the files test kernel behaviour (they also use `CHARGE_LOG_CAPACITY`, which `quire-exact` does not export, so that constant's assertions go with the kernel's accounting evidence): evidence belongs to the QSL repository, which this repository does not track |
| `tests/exact_outcomes.rs` | TC-016, TC-017, TC-031 | splits | the kernel `Meter`, `Outcome` and injected-denial tests leave in step 1; the cases that use `TypeEnvironment`, `CheckedEquality`, `CheckedPackage`, `ObjectEnvironment`, `UnitGraph`, `EnumDeclaration` and `evaluate_quantity`, which `quire-exact` does not export, leave with the residue |
| `tests/exact_collection.rs` | TC-025 | splits | the collection-algebra and canonical-key cases leave in step 1; the cases built on `TypeEnvironment` and `CompositeDeclaration` leave with the residue |
| `tests/exact_meter_state.rs` | TC-032 | splits | `evaluate_boolean_short_circuit` lazy invocation/stop propagation (FR-011-AC-3/AC-8, step 5) stays in RT; already-decided Boolean and meter cases leave in step 1; `UnitGraph::admit`, `CompoundUnit` and `evaluate_quantity` cases leave with RT residue under open IR-349 step 2 and IR-583 |
| `tests/exact_semantics.rs` | TC-034 | splits | the `UnitGraph`, `Dimension` and `evaluate_quantity` cases leave with the residue; the rest leave in step 1 |
| `tests/exact_debug_parity.rs` | TC-035 | splits | the `CompoundUnit`, `Dimension` and `EnumDeclaration` Debug pins leave with the residue; the rest leave in step 1 |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031, TC-032 | leaves in step 1 | current in-crate tests of kernel `Meter` and `Integer` internals that IR-349 will remove |

For the kernel, RT keeps the consumption checks TC-197 to TC-199 in the end state: direct
consumption, no copied code and guarded edges. RT also keeps evidence of its own lazy connective
in TC-032 and backend negotiation in TC-030/TC-195; kernel deletion shall not remove those checks.

## Evidence Locations

- TC-198: `deny.toml` (the `[bans]` list and `[graph] all-features = true`) and
  `scripts/check_deny_bans.sh`, run by `make deny-mutations`, which adds each banned crate to a
  scratch copy of the tracked files and requires cargo-deny's `banned` error. TC-199:
  `scripts/run_feature_matrix.py` (row `build-exact-no-std-msrv`, `make test-features`),
  `make msrv` and `make size` (linked band, panic relocations and the footprint-graph check that
  `quire-exact` is absent). Neither has a Rust test.
- TC-194, TC-195: `tests/exact_function_application.rs` (`--features exact`), landed under
  agent-ix/quire-contract-runtime#34. FR-273-AC-4's evidence is a `compile_fail` doctest on
  `IeeeDisposition` (`src/exact/ieee.rs`), mirroring `InjectedDenial`'s.
- FR-273-AC-5 (TC-194's shared-corpus row): the QSL agreement oracle is removed from this repository;
  recreating it in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done). The removed oracle agreed on the closed `InputRefusal` vocabulary (`UnknownFunction`, `Arity`, `WrongValueKind`,
  `DanglingReference`) with its codes and causes, and the charge count of one admitted call (AP05).
  That historical agreement did **not** establish check *ordering*: each of its five vectors (AP01 through AP05) triggers
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
  absent body-semantics corpus expands AC-5's shared-corpus scope. The removed oracle
  historically agreed on all five supplied vectors; that result does not establish current
  agreement evidence. AC-5 is currently partly evidenced: local ordering tests remain, while
  recreating two-sided shared-corpus evidence is open IR-669 work.
- TC-030: `tests/exact_negotiation.rs`; TC-032 (current evidence; lazy FR-011-AC-3/AC-8 stays): `tests/exact_meter_state.rs` and the in-crate
  `src/exact_accounting_tests.rs` for the cumulative-counter boundary no public operator can
  reach;
  TC-033: `tests/exact_vocabulary.rs`; TC-034: `tests/exact_semantics.rs`; TC-035:
  `tests/exact_debug_parity.rs`.
- TC-016, TC-017, TC-031: `tests/exact_outcomes.rs` and, for TC-031's check-before-mutate ordering
  across every counter, the in-crate `src/exact_accounting_tests.rs` (needs `Charge`'s
  crate-private builders to construct a charge that moves every counter at once, so is reachable
  only from inside the crate); TC-023: `tests/exact_arithmetic.rs`; allocation bounds for
  TC-016, TC-018, TC-019 and TC-023: `tests/exact_allocation.rs`. All run with
  `--features exact`. FR-010-AC-6's evidence is a `compile_fail` doctest on `InjectedDenial`
  (`src/exact/accounting.rs`): `occurrence: 0` does not compile, so the malformed request cannot be
  written.
- TC-018 through TC-022: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done). TC-018/TC-019 currently retain only local allocation-bound evidence; it leaves in IR-349
  and does not close the remaining agreement gap. Owner mappings are in the individual TC documents.
- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`; TC-026:
  `tests/exact_equality.rs`. All run with `--features exact`.
