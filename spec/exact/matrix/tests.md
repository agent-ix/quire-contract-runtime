---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1 | TC-016 | 🚧 owner-backed by quire-exact FR-362-AC-11 for scalar outcome stops; RT-local TC-016 tests left with the copied kernel, and the RT row has no local binder |
| FR-006 | FR-006-AC-6 | — (Inspection) | ✅ ownership inspection: quire-exact FR-096-AC-8 owns code/cause; RT consumes typed `Outcome::Refused(Refusal)` directly through `src/exact/mod.rs` without a local mapping or stale four-code test |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | 🚧 owner-backed by quire-exact FR-368-AC-1/2 for vocabulary and FR-359-AC-6/7 for meter ordering; local kernel tests removed |
| FR-006 | FR-006-AC-4 | TC-017 | 🚧 owner-backed by quire-exact FR-358-AC-1/2 and FR-362-AC-11 for admitted-charge denial and scalar stops; local kernel test removed |
| FR-007 | FR-007-AC-1 | TC-018 | 🚧 owner-backed by quire-exact FR-361-AC-7/8 for dividend-bit admission only; arithmetic `max(bits(a), bits(b))` exact/one-under remains an owner gap, and QSL two-sided agreement remains IR-669 work |
| FR-007 | FR-007-AC-2 | TC-019 | 🚧 owner-backed by quire-exact FR-361-AC-9 for denied retain-upscale; RT-local allocation test removed and QSL two-sided agreement remains IR-669 work |
| FR-007 | FR-007-AC-3 | TC-020 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-4 | TC-021 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-5 | TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | 🚧 partly evidenced: the runtime-side Debug pins remain (TC-035); the QSL Debug-parity half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done) |
| FR-007 | FR-007-AC-7 | TC-023 | 🚧 owner-backed by quire-exact FR-362-AC-14..18 for matching scalar atoms; FR-362-AC-10 remains planned and untagged (IR-667), and QSL agreement remains IR-669 work |
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | ✅ implemented |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ owner FR-368-AC-1/2 tests the 62-point census and round trips; retained RT tests check the twelve added denial points and residue behavior |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-13 | TC-024 | ✅ implemented: deep metadata clone, equality, formatting and drop in a small-stack thread and default-stack child process |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-025 | 🚧 sequence and ordered-set cases not yet in tests/exact_collection.rs |
| FR-007 | FR-007-AC-8 | TC-034 | 🚧 owner-backed by quire-exact FR-365-AC-1/2 for rounding ties and default; RT-local kernel test removed |
| FR-007 | FR-007-AC-9 | TC-034 | 🚧 owner-backed by quire-exact FR-366-AC-1..4 for IEEE NaNs, signed zero and order; RT-local kernel test removed |
| FR-007 | FR-007-AC-10 | TC-034 | 🚧 owner-backed by quire-exact FR-362-AC-12/13 for rational canonical form and membership; RT-local kernel test removed |
| FR-007 | FR-007-AC-11 | TC-034 | 🚧 owner-backed by quire-exact FR-363-AC-6/7 for normalized decimal value versus retained representation; RT-local kernel test removed |
| FR-007 | FR-007-AC-12 | TC-034 | ✅ retained RT quantity and Euclidean residue assertions; matching kernel subsets remain owner-backed |
| FR-007 | FR-007-AC-13 | TC-035 | ✅ implemented |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`, `src/exact/ieee.rs`) |
| FR-010 | FR-010-AC-1 | TC-031 | ✅ retained RT residue charge-point driver; owner-backed by quire-exact FR-358-AC-1 for generic injection |
| FR-010 | FR-010-AC-4 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1 for named admitted occurrence; RT-local kernel test removed |
| FR-010 | FR-010-AC-5 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1/2 for one-shot retry; RT-local kernel test removed |
| FR-010 | FR-010-AC-6 | TC-031 | ✅ owner-backed by quire-exact FR-358-AC-3 Inspection of nonzero occurrence type |
| FR-010 | FR-010-AC-2 | TC-031 | 🚧 planned RT-owned gap (IR-676): no direct RT assertion for `equality.plan` `pairs + 2` reservation after kernel-test removal; owner FR-358-AC-2 tests only a narrower denial |
| FR-010 | FR-010-AC-3 | TC-031 | 🚧 planned RT-owned gap (IR-676): simultaneous injected-versus-real-limit precedence lacks a direct assertion; owner FR-358-AC-1/2 does not close it |
| FR-011 | FR-011-AC-1 | TC-032 | 🚧 owner-backed by quire-exact FR-362-AC-11 for scalar stop prefixes; RT-local kernel test removed |
| FR-011 | FR-011-AC-4 | TC-032 | ✅ retained RT field-order test; owner-backed by quire-exact FR-359-AC-6 for complete meter scan |
| FR-011 | FR-011-AC-5 | TC-032 | 🚧 owner-backed by quire-exact FR-358-AC-8..11 for the first 4096 log entries, truncation and post-cap enforcement; RT-local kernel test removed |
| FR-011 | FR-011-AC-6 | TC-032 | ✅ retained RT derived-over-u64 test; owner-backed by quire-exact FR-359-AC-1..5 for cumulative boundaries |
| FR-011 | FR-011-AC-2, FR-011-AC-3, FR-011-AC-7, FR-011-AC-8 | TC-032 | ✅ RT quantity, lazy connective and graph-order tests remain; owner FR-359-AC-7 covers the consumed-reader subset and FR-364-AC-1 the IEEE flag subset |
| FR-012 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033 | ✅ implemented |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented: AC-1's linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable` (a package `check` rejects under `CheckMode::Kernel` is never applicable — that is AC-6's inspection too) together with `tc_194_linked_package_applies_every_declared_function` and the rest of this corpus's `Linked`-application tests, which call only a package `check` admitted under `CheckMode::Linked`. AC-2/AC-3's "arity before any per-argument check, all before the `function.call` charge, before the body" ordering is covered for both `call` and `Frame::call` |
| FR-273 | FR-273-AC-7 | TC-194 | ✅ implemented: re-entry into a checked package through `CheckedPackage::call`, `CheckedPackage::evaluate` or `Frame::call` is bounded by `CheckingLimits::depth` on a shared counter — not only `Frame::call` — including the direct-re-entry attack a body holding its own `Rc<CheckedPackage>` could otherwise use to bypass it, proved by `tc_194_recursion_beyond_the_depth_limit_is_a_checked_invariant_refusal`, `tc_194_direct_reentrant_package_call_is_bounded_like_frame_call` and `tc_194_checking_limits_refuses_a_depth_above_the_maximum`. The bound is per-`CheckedPackage`, not universal: a host body that builds a *fresh* `CheckedPackage` at each hop gets a fresh budget and can still overflow the host stack — but so does a body that recurses without touching this crate's runtime at all, since under AD-002 a body is arbitrary host Rust and its own stack usage is the host's concern, not this crate's |
| FR-273 | FR-273-AC-5 | TC-194 | 🚧 partly evidenced: the QSL shared-corpus half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done); the runtime-only ordering tests remain. AC-5 quantifies over shared-corpus function-application vectors only, and the removed shared corpus historically agreed on all five of them — the closed `InputRefusal` vocabulary (with codes and causes), the charge count of one admitted call, and — via AP01–AP04's `charges == 0` assertions on each refusal path (the removed QSL shared-corpus vectors; see Evidence Locations) — that every refusal precedes the `function.call` charge, agreed on both sides. Relative order *among* the four checks themselves (arity, value kind, dangling reference, unknown function) is not something any vector needs to discriminate for AC-5 to be met, since each corpus vector isolates exactly one violation by design; that ordering is instead verified by the runtime-only tests in `tests/exact_function_application.rs` (see Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared corpus either, for the same reason: AC-5 does not claim them. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 planned (Linear IR-349; AC-16 and AC-18 are unmet while the residue exists, and the residue is not authorized: no exception, no expiry, no approval; AC-17 can be checked today): the runtime still holds its own copy of the kernel and of the residue. The IR-349 foundation slice (floor, dependency, bans, one copy; it deletes nothing) made AC-3 to AC-6 true (the optional `quire-exact` git dependency at `branch = "main"`, one lock entry, `make deny`'s one-copy check); AC-1, AC-2, AC-12 and AC-13 need the copy deleted; interface-001-AC-7 is likewise unmet while the runtime's copy of `Frame`, `Body`, `CheckedPackage`, `Evaluation` and `plan_call` exists. AC-1 and AC-16 now assert the end state of no `exact` module and no re-export (AD-004 step 1) and are unmet today, planned until the IR-349 code steps land; AC-17 restates its exception as the `scalar` items and stays checkable today. Per interface-001 criterion, see the interface-001 rows in the core matrix |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | ✅ implemented (IR-349 foundation slice; evidence is a gate script, not a `tc_NNN` test): `deny.toml` bans every listed QSL crate, and `make deny-mutations` (`scripts/check_deny_bans.sh`) adds `qsl-eval`, `qsl-replay`, `qsl-semantics` and `quire-spec-language` as normal and as dev dependencies in a scratch copy and requires cargo-deny's `banned` error for each |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | ✅ gate targets use the manifest-declared toolchain, build the exact profile with quire-exact and exclude it from the footprint graph |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3 | 🚧 local kernel tests and stale four-code tags removed; owner FR-096-AC-8, FR-362-AC-11 and FR-368-AC-1/2 test matching kernel behavior |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | 🚧 local kernel meter tests removed; matching owner evidence is FR-358/359/368 |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 local allocation tests removed; owner dividend-bit exact/one-under evidence exists, arithmetic `max(bits(a), bits(b))` remains unproved, and two-sided agreement remains IR-669 work |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 local allocation tests removed; owner FR-361-AC-9 tests the denied upscale, and two-sided agreement remains IR-669 work |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | 🚧 owner FR-362-AC-10 remains planned and untagged (IR-667); shared-authority agreement remains IR-669 work |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-13 | ✅ RT construction tests remain; owner FR-368-AC-1/2 tests the full charge-point census |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | 🚧 steps 1–2 and 4–7 implemented in `tests/exact_collection.rs`; steps 3 and 5 cover the set and bag only, so FR-008-AC-10 through FR-008-AC-12's sequence and ordered-set cases are not yet tested |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | ✅ implemented |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | 🚧 RT residue charge-point driver remains; AC-2/3 owner gaps remain IR-676 |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | 🚧 RT quantity, lazy connective and graph cases remain; owner tests cover matching kernel meter subsets |
| TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | ✅ implemented |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12 | 🚧 RT quantity cases remain; kernel semantic cases left and matching owner tests are FR-362/363/365/366 |
| TC-035 | Pin boxed value and type structs' hand-written Debug rendering | Unit | P1 | FR-007-AC-13 | ✅ implemented |
| TC-194 | Apply checked functions totally, before any charge | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7 | ✅ implemented |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-6, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | 🚧 planned (Linear IR-349): the kernel copy is deleted in part 1 and the residue in part 2 (remaining RT deletion owned by open IR-349 step 2 and IR-583; QSL-358 extraction is Done); step 7 fails while the residue exists |
| TC-198 | Fail the build on a dependency on a guarded QSL crate | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | ✅ implemented (IR-349 foundation slice): `deny.toml` entries and `make deny-mutations` |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | ✅ exact profile and footprint gates use the manifest-declared toolchain; the footprint graph excludes quire-exact |

FR-009-AC-5, FR-010-AC-6 and FR-273-AC-4 use `compile_fail` doctests. TC-198 and
TC-199 use the gate targets named under Evidence Locations. The retired TC-016 four-code
Rust tests and tags are absent. FR-006-AC-6 is an ownership inspection of the direct
typed kernel outcome boundary; quire-exact FR-096-AC-8 owns the kernel code/cause census.
`quire matrix` reports trace presence, not semantic coverage across repositories. The
retained RT charge-point driver binds FR-010-AC-1 only; FR-010-AC-2/3 remain IR-676 gaps.

## Evidence at the kernel move

FR-275 requires RT to consume `quire-exact` directly. This code slice removes the
local scalar kernel and its duplicate tests. RT evaluation residue remains for open IR-349
step 2 and IR-583. The status rows above describe the current tree: owner tests bind
matching kernel behaviors in quire-exact, while the RT coverage matrix only counts local
trace tags. FR-010-AC-2/3 lack their exact owner assertions (IR-676), FR-362-AC-10 remains
planned and untagged (IR-667), and QSL two-sided agreement remains IR-669 work.

The retained TC-032 lazy connective test is RT-owned under FR-011-AC-3/AC-8. RT also
retains quantity, graph, declaration and function-package residue pending its assigned
follow-up work. No deleted kernel test is represented as an RT/QSL agreement test.
The table records what this slice actually changed; future dispositions are stated separately.

| Test file | Test cases | Disposition | Reason |
|---|---|---|---|
| `tests/exact_function_application.rs` | TC-194, TC-195 | retained | RT function-package residue and backend negotiation tests remain; FR-273 agreement is still IR-669 work |
| `tests/exact_negotiation.rs` | TC-030 | retained | RT backend negotiation stays |
| `tests/exact_vocabulary.rs` | TC-033 | retained | RT vocabulary residue remains for later migration |
| `tests/exact_equality.rs` | TC-026 | retained | RT checking-environment and equality-plan tests remain in this slice |
| `tests/exact_composite.rs` | TC-024 | retained | RT construction and declaration-environment tests remain in this slice |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018, TC-019, TC-023 | removed | duplicate scalar kernel test files; matching owner cases are cited in the TC documents, while agreement and partial owner gaps stay open |
| `tests/exact_outcomes.rs` | TC-031 | one retained test | the residue charge-point driver remains; TC-016/017 kernel tests and stale four-code tags were removed |
| `tests/exact_collection.rs` | TC-025 | retained | all eight collection and canonical-key tests remain in this slice; their RT `Value` and type behavior remains residue |
| `tests/exact_meter_state.rs` | TC-032 | retained subset | nine RT quantity, lazy connective and graph-order tests remain; kernel-only meter tests were removed |
| `tests/exact_semantics.rs` | TC-034 | retained subset | RT quantity and Euclidean residue cases remain; duplicate kernel semantic tests were removed |
| `tests/exact_debug_parity.rs` | TC-035 | retained subset | residue Debug cases remain; duplicate kernel Debug cases were removed |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031, TC-032 | removed | in-crate tests of the deleted RT kernel implementation |

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
- TC-030: `tests/exact_negotiation.rs`; retained TC-032 RT quantity, lazy and graph
  cases: `tests/exact_meter_state.rs`; TC-033: `tests/exact_vocabulary.rs`;
  retained TC-034 quantity/residue cases: `tests/exact_semantics.rs`; retained TC-035
  residue Debug cases: `tests/exact_debug_parity.rs`.
- TC-031: the retained residue charge-point driver in `tests/exact_outcomes.rs`.
  TC-016/017 kernel tests, TC-023 arithmetic tests and TC-018/019 allocation tests
  left with the RT kernel copy. Their matching owner evidence is in quire-exact;
  the RT/QSL and owner gaps identified above remain open.
- TC-018 through TC-022: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done). TC-018/TC-019 local allocation tests were removed in this slice. Owner tests cover matching kernel cases but do not close the remaining agreement gap. Owner mappings are in the individual TC documents.
- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`; TC-026:
  `tests/exact_equality.rs`. All run with `--features exact`.
