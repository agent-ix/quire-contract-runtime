---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1 | TC-016 | 🚧 AC-1 has a tagged RT consumer test for all four dispositions, completed `false` and the closed kernel reason enums; TC-016's AC-3 vocabulary and accounting procedure remains open |
| FR-006 | FR-006-AC-6 | — (Inspection) | 🚧 owner-backed by quire-exact FR-096-AC-8 for code/cause; RT consumes typed `Outcome::Refused(Refusal)` directly through `src/exact/mod.rs`, but no RT-local inspection binder remains after the stale four-code test was removed |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | 🚧 TC-016 now requires the complete QSpec vocabulary, including `equality.plan-form`; quire-exact FR-368-AC-1/2 owns spelling tests and FR-359-AC-6/7 owns meter ordering, but the local kernel tests were removed and the complete TC-016/TC-017 procedure remains open |
| FR-006 | FR-006-AC-4 | TC-017 | 🚧 owner-backed by quire-exact FR-358-AC-1/2 and FR-362-AC-11 for admitted-charge denial and scalar stops; local kernel test removed |
| FR-006 | FR-006-AC-7 | — (Inspection) | 🚧 planned: RT-local host floating-point inspection has no bound evidence symbol |
| FR-006 | FR-006-AC-8 | TC-199 | 🚧 planned: `build-exact-no-std-msrv` exists but has no bound criterion evidence symbol |
| FR-006 | FR-006-AC-9 | — (`compile_fail` and Inspection) | 🚧 planned: per-child private-path doctests and direct-public-definition inspection are not yet bound |
| FR-006 | FR-006-AC-10 | — (Inspection) | 🚧 planned: RT-local non-test panic-site inspection has no bound evidence symbol |
| FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: quire-exact FR-361-AC-7/8 tests dividend-bit admission only; arithmetic `max(bits(a), bits(b))` exact/one-under is an owner evidence gap tracked by IR-678, and QSL two-sided agreement remains IR-669 work |
| FR-007 | FR-007-AC-2 | TC-019 | 🚧 owner-backed by quire-exact FR-361-AC-9 for denied retain-upscale; RT-local allocation test removed and QSL two-sided agreement remains IR-669 work |
| FR-007 | FR-007-AC-3 | TC-020 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-4 | TC-021 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-5 | TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-007 | FR-007-AC-6 | TC-018, TC-019, TC-020, TC-021, TC-022 | 🚧 partly evidenced: the runtime-side Debug pins remain (TC-035); the QSL Debug-parity half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done) |
| FR-007 | FR-007-AC-7 | TC-023 | 🚧 owner-backed by quire-exact FR-362-AC-14..18 for matching scalar atoms; FR-362-AC-10 remains planned and untagged (IR-667), and QSL agreement remains IR-669 work |
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | 🚧 RT duplicate composite and containment tests removed in IR-882; QSV and kernel owner evidence plus RT consumer mapping remain planned |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | 🚧 RT duplicate collection and key tests removed in IR-882; kernel owner evidence plus RT consumer mapping remain planned |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | 🚧 TC-203 and TC-026 RT consumer checks requiring local identities move to quire-integration IR-1175; owner evidence remains and the rest of the equality matrix is open. |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | 🚧 kernel FR-368-AC-1/2 owns the 63-point vocabulary; RT TC-031 covers remaining RT points and TC-203 covers QSV equality; the other denial and refusal evidence requires owner mapping |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | 🚧 RT duplicate Value removed; kernel deep Debug/Drop owner evidence requires mapping |
| FR-008 | FR-008-AC-13 | TC-024 | 🚧 RT duplicate ValueType removed; kernel deep metadata owner evidence requires mapping |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-025 | 🚧 Foreign-reference membership ordering from the removed RT fixture moves to quire-integration IR-1175; other kernel and consumer evidence remains open. |
| FR-007 | FR-007-AC-8 | TC-034 | 🚧 RT-local tests cover all six literal spellings and their consumed `RoundingMode` mapping, the consumed enum's `Exact` default, all six modes on positive and negative ties, and explicit `Exact` refusal; compiler package omission selecting `Exact` remains unbacked here |
| FR-007 | FR-007-AC-9 | TC-034 | 🚧 owner-backed by quire-exact FR-366-AC-1..4 for IEEE NaNs, signed zero and order; RT-local kernel test removed |
| FR-007 | FR-007-AC-10 | TC-034 | 🚧 owner-backed by quire-exact FR-362-AC-12/13 for rational canonical form and membership; RT-local kernel test removed |
| FR-007 | FR-007-AC-11 | TC-034 | 🚧 owner-backed by quire-exact FR-363-AC-6/7 and `tests/ir673_decimal.rs`: equal normalized values, ordering size counters 8/3/1 versus 4/2/0, and one Boolean result unit in both comparisons; RT exact-feature binder remains planned after the local kernel test was removed |
| FR-007 | FR-007-AC-12 | TC-034 | 🚧 QSV first-cause type-fault and cross-dimension arithmetic RT consumer controls move to quire-integration IR-1175; owner operation evidence still needs mapping. |
| FR-007 | FR-007-AC-13 | TC-035 | 🚧 RT's duplicate `CompoundUnit` is removed; `tests/exact_unit_owner.rs` pins the consumed QSV Debug rendering, while the complete nine-type obligation needs owner evidence mapping |
| FR-007 | FR-007-AC-15 | TC-034 | ✅ RT-local test checks all three quotient-sign vectors, rounded coefficients and exact loss |
| FR-007 | FR-007-AC-16 | TC-034 | 🚧 QSV affine-unit Power refusal and empty-charge control move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-17 | TC-034 | 🚧 QSV same-dimension Multiply/Divide and canonical unit controls move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-18 | TC-034 | ✅ RT-local test checks all four Euclidean `mod` sign vectors and typed out-of-domain refusal |
| FR-007 | FR-007-AC-19 | TC-034 | ✅ RT-local test checks both admitted exposed members and both typed refusals under every division profile |
| FR-007 | FR-007-AC-20 | TC-201 | 🚧 QSV conversion magnitudes, units and independent rational sweep move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-21 | TC-201 | 🚧 QSV IncompatibleDimensions before-charge controls move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-22 | TC-201 | 🚧 QSV admitted affine conversion controls move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-23 | TC-201 | 🚧 QSV conversion charge order and exact amounts move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-24 | TC-201 | 🚧 QSV affine arithmetic refusal and empty-charge controls move to quire-integration IR-1175. |
| FR-007 | FR-007-AC-25 | TC-201 | 🚧 QSV denied conversion tuples and Incomplete fields move to quire-integration IR-1175. |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`, `src/scalar/ieee.rs`) |
| FR-010 | FR-010-AC-1 | TC-031 | ✅ retained RT residue charge-point driver; owner-backed by quire-exact FR-358-AC-1 for generic injection |
| FR-010 | FR-010-AC-4 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1 for named admitted occurrence; RT-local kernel test removed |
| FR-010 | FR-010-AC-5 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1/2 for one-shot retry; RT-local kernel test removed |
| FR-010 | FR-010-AC-6 | TC-031 | ✅ owner-backed by quire-exact FR-358-AC-3 Inspection of nonzero occurrence type |
| FR-010 | FR-010-AC-2 | owner TC-906; TC-031 RT driver only | 🚧 owner quire-exact FR-358-AC-12 has direct public-meter record tests; RT TC-031 does not repeat the two-limit case, and RT's lockfile still resolves the older kernel revision and needs a separate refresh |
| FR-010 | FR-010-AC-3 | owner TC-906; TC-031 RT driver only | 🚧 owner quire-exact FR-358-AC-13 has direct public-meter ordinary-limit precedence tests; RT TC-031 does not test competing refusals, and RT's lockfile still resolves the older kernel revision and needs a separate refresh |
| FR-011 | FR-011-AC-1 | TC-032 | 🚧 owner-backed by quire-exact FR-362-AC-11 for scalar stop prefixes; RT-local kernel test removed |
| FR-011 | FR-011-AC-4 | TC-032 | 🚧 QSV quantity identity-read field order RT consumer control moves to quire-integration IR-1175; exact FR-359-AC-6 retains owner meter evidence. |
| FR-011 | FR-011-AC-5 | TC-032 | 🚧 owner-backed by quire-exact FR-358-AC-8..11 for the first 4096 log entries, truncation and post-cap enforcement; RT-local kernel test removed |
| FR-011 | FR-011-AC-6 | TC-032 | 🚧 QSV over-u64 denied quantity charge control moves to quire-integration IR-1175; exact FR-359-AC-1/3/4/5 retains owner atomicity evidence. |
| FR-011 | FR-011-AC-2, FR-011-AC-3, FR-011-AC-7, FR-011-AC-8 | TC-032 | 🚧 RT retains its lazy-connective control; QSV quantity zero stop, unit graph refusal and term order consumer controls move to quire-integration IR-1175. Kernel FR-359-AC-7 and FR-364-AC-1 retain owner evidence. |
| FR-012 | FR-012-AC-1 | — (retired) | 🚧 retired with deletion of RT-local `NodeKey` and its constructor tests in the IR-882 key-owner slice. Original criterion text and ID remain reserved. |
| FR-012 | FR-012-AC-6 | TC-033 | ✅ `tc_033_runtime_has_no_local_node_key_constructor` recursively inspects all RT Rust source, excluding test-only items; the local key, constructors and their parser/round-trip tests are deleted. RT library code contains no key construction or hex decoder. |
| FR-012 | FR-012-AC-2 | TC-033 (`quire-integration`) | 🚧 The `quire-integration` producer-origin Test still needs IR-1141, QSL-1282 and IR-1142 before it can drive RT's declaration, foreign-universe and equal-byte declared/compound behavioral controls from admitted identities. RT takes no QSL/IR Git dependency. |
| FR-012 | FR-012-AC-8 | Inspection (RT); TC-033 (`quire-integration`) | 🚧 RT source Inspection tc_033_rt_does_not_construct_semantic_identities confirms no test fixture constructs identity from bytes or constituent parts. Producer-origin TC-033 behavioral evidence moves to quire-integration IR-1175, blocked by QSL-1282. |
| FR-012 | FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033 | 🚧 RT retains AC-3 definition and refusal controls; AC-4 enum/graph admission and AC-5 canonical unit controls require producer-origin identities and move to quire-integration IR-1175. |
| FR-012 | FR-012-AC-7 | TC-033 | ✅ `tc_033_definition_refusals_have_only_the_qsv_owner` checks deletion of the RT module and its export, and checks QSV's three valid `invalid_package` causes; source inspection confirms no RT alias, re-export or adapter and direct QSV test imports. |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | 🚧 RT retains linked-only application, arity, wrong-kind, unknown-function and child-call controls. Identity-dependent dangling-reference and competing-refusal ordering move to quire-integration IR-1175. |
| FR-273 | FR-273-AC-8 | TC-194 | ✅ implemented: foreign expressions, absent functions requested within a checked body, meter re-borrows and direct host re-entry retain their distinct `ForeignCheckedExpression`, `UnknownCheckedFunction`, `MeterBorrowConflict` and `PackageReentry` checked-invariant causes. TC-194 asserts the foreign path admits no charge, a missing function or meter re-borrow inside one admitted outer call retains exactly that one `function.call` charge and one work unit, and host re-entry adds no nested charge. |
| FR-273 | FR-273-AC-12, FR-273-AC-13 | TC-194 | 🚧 PLANNED/UNRUN (IR-1009, blocked by IR-590; AD-004 step 5): no_std checked-input and generated-body consumer agreement and 100,000-call/512-KiB work-fuel acceptance. |
| FR-273 | FR-273-AC-7, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11 | TC-194 | ✅ RT-local scheduler evidence: `tc_194_explicit_frames_reach_4096_calls_on_64_kib_stack_with_exact_fuel` checks depths 1, 128 and 4,096, one active transition, exact and one-less `work_units` limits, charge counters and logs, and a retry after exhaustion. `tc_194_nested_arguments_charge_before_enclosing_body_and_stop_at_one_less` checks left/right/enclosing charge order and the denied enclosing charge. `tc_194_invalid_child_arguments_refuse_before_child_charge_or_body` and `tc_194_stopped_child_does_not_resume_parent` check child validation and stop propagation; the deep-call test checks the returned child value. The runtime scheduler in `src/exact/expression.rs` returns to its loop between transitions. CG-generated-body agreement and QSL-owned leaf consumption remain separate work under FR-273-AC-12/13. |
| FR-273 | FR-273-AC-5 | TC-194 | 🚧 QSL agreement corpus remains IR-669; identity-dependent runtime-only plan/refusal variants move to quire-integration IR-1175. RT retains non-identity application controls. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/scalar/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 RT now consumes the kernel Value, ValueType, CollectionType, equality planner and QSV checker directly, with no RT export for those names. TC-203 consumer-evaluator evidence moves to quire-integration IR-1175. IR-349 keeps independent AD-004 steps 1 through 4; IR-1009 receives step 5 and dependent empty-`exact` closure. AC-1, AC-13, AC-16 and AC-18 remain unmet while expression residue exists. AC-14 preserves TC-024 through TC-026 as planned artifacts after their duplicate source tests were removed. The footprint gate remains TC-199; TC-197 retains direct inspection of AC-5 lock entries. |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | 🚧 `deny.toml` and `make deny-mutations` (Rust `check_deny_source`) contain the QSL Git source guard and normal, build and dev dependency mutation checks; the traced Rust test checks diagnostic discrimination, while the real dependency injections remain the gate evidence. AC-7 is an inspection; IR-499 remains open for strict-coverage evidence |
| FR-275 | FR-275-AC-29, FR-275-AC-30 | TC-199, TC-194 | 🚧 PLANNED/UNRUN (IR-1009, blocked by IR-590; AD-004 step 5): evaluation-leaf no_std target/host consumers and separate default versus exact-profile footprint receipts. |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | 🚧 `make test-features`, `make size` and `make msrv` contain the exact no-std, footprint graph, size and toolchain checks; AC-20 is an inspection, while AC-9 through AC-11, AC-21 and TC-199 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
| FR-275 | FR-275-AC-22 | TC-205 | ✅ `tests/ir882_qsv_vocabulary.rs` rejects local semantic-node vocabulary definitions and aliases, including renamed copies |
| FR-275 | FR-275-AC-23 | TC-201, TC-197 | 🚧 QSV exact/non-exact handoff, declared/compound distinction and U25 two-step RT consumer controls move to quire-integration IR-1175; QSV FR-124-AC-4/TC-923 retains owner refusal evidence. |
| FR-277 | FR-277-AC-1, FR-277-AC-2, FR-277-AC-3, FR-277-AC-4, FR-277-AC-5, FR-277-AC-6, FR-277-AC-7, FR-277-AC-8 | TC-203 | 🚧 AC-8 RT owner-deletion inspection remains; AC-1 through AC-6 QSV evaluator consumer controls move to quire-integration IR-1175; AC-7 Kani proof remains separate. |
| FR-278 | FR-278-AC-1, FR-278-AC-2 | TC-204 | 🚧 RT function producer controls remain to be bound; collection and composite causes need shared-owner evidence; equality and quantity causes are owned by quire-semantic-value FR-122-AC-1 and AC-4 |
## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3 | 🚧 AC-1 has a tagged RT consumer test; AC-3 vocabulary round-trips and accounting still require complete owner and consumer evidence |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | 🚧 local kernel meter tests removed; matching owner evidence is FR-358/359/368 |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 local allocation tests removed; owner dividend-bit exact/one-under evidence exists, arithmetic `max(bits(a), bits(b))` remains unproved (IR-678), and two-sided agreement remains IR-669 work |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 local allocation tests removed; owner FR-361-AC-9 tests the denied upscale, and two-sided agreement remains IR-669 work |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | 🚧 owner FR-362-AC-10 remains planned and untagged (IR-667); shared-authority agreement remains IR-669 work |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-13 | 🚧 retained test case; RT duplicate tests removed, owner evidence and RT consumer mapping planned |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | 🚧 retained test case; RT duplicate tests removed, owner evidence and RT consumer mapping planned |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | 🚧 Composite, enum and membership pair-result consumer controls move to quire-integration IR-1175; remaining equality matrix and reference procedures remain open. |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | 🚧 local RT evidence is limited to the residue charge-point driver under AC-1; AC-2/3 owner evidence is FR-358-AC-12/13 and TC-906, while RT's older kernel revision still needs a separate refresh |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | 🚧 RT retains its lazy-connective control; QSV quantity stop, field order, over-u64 denial, unit graph and term-order controls move to quire-integration IR-1175. |
| TC-033 | Carry the compiler vocabulary byte-exactly | Integration | P0 | FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5, FR-012-AC-6, FR-012-AC-7, FR-012-AC-8 | 🚧 RT key/definition owner inspections and the AC-8 fixture inspection remain; AC-4/5 and other producer-origin identity controls move to quire-integration IR-1175. |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12, FR-007-AC-15, FR-007-AC-16, FR-007-AC-17, FR-007-AC-18, FR-007-AC-19 | 🚧 RT retains literal, rounding, mod and integer division controls; QSV quantity first-cause, affine-power and same-dimension arithmetic controls move to quire-integration IR-1175. |
| TC-035 | Pin boxed value and type structs' hand-written Debug rendering | Unit | P1 | FR-007-AC-13 | 🚧 `tests/exact_unit_owner.rs` pins QSV's `CompoundUnit` Debug rendering; full owner evidence needs mapping |
| TC-194 | Apply checked functions with explicit frames and work fuel | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7, FR-273-AC-8, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11, FR-273-AC-12, FR-273-AC-13 | 🚧 RT retains non-identity input, explicit-frame, deep-call and work-fuel cases; identity-dependent plan/refusal controls move to quire-integration IR-1175. Shared-corpus AC-5 remains IR-669. |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19, FR-275-AC-23 | 🚧 planned across IR-349 (independent steps 1–4) and IR-1009 (step 5 and dependent end state): FR-275-AC-22 has its own inspection in TC-205. Value, equality, quantity and unit owners are cut over; AC-23 producer-origin consumer evidence moves to quire-integration IR-1175. Function-application deletion awaits IR-590 target acceptance; step 7 fails while residue exists |
| TC-198 | Fail the build on a dependency from the QSL Git source | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | 🚧 source rejection gate exists in `deny.toml` and `make deny-mutations`; the traced Rust test checks the diagnostic discriminator, and the real mutation gate supplies dependency evidence |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-006-AC-8, FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21, FR-275-AC-29, FR-275-AC-30 | 🚧 exact profile and footprint gates exist in `make test-features`, `make size` and `make msrv`; Quire has no test-symbol binder for TC-199 |
| TC-201 | Convert a quantity exactly between units of one dimension | Unit | P0 | FR-007-AC-20, FR-007-AC-21, FR-007-AC-22, FR-007-AC-23, FR-007-AC-24, FR-007-AC-25, FR-275-AC-23 | 🚧 All RT QSV quantity conversion and handoff consumer controls requiring locally constructed identities move to quire-integration IR-1175. |
| TC-203 | Exercise checked equality through the kernel plan and runtime meter | Unit | P0 | FR-277-AC-1, FR-277-AC-2, FR-277-AC-3, FR-277-AC-4, FR-277-AC-5, FR-277-AC-6, FR-277-AC-7, FR-277-AC-8 | 🚧 RT retains AC-8 owner-deletion inspection; QSV equality evaluator consumer controls AC-1 through AC-6 move to quire-integration IR-1175; AC-7 Kani proof remains separate. |
| TC-204 | Runtime checked-invariant producer controls | Unit | P0 | FR-278-AC-1, FR-278-AC-2 | 🚧 bind RT function producer evidence and shared-owner collection/composite controls; inspect QSV FR-122 owner controls for equality/quantity |
| TC-205 | Inspect QSV semantic-node vocabulary ownership | Integration | P0 | FR-275-AC-22 | ✅ `tests/ir882_qsv_vocabulary.rs` inspects all runtime source for local semantic-node definitions and aliases and rejects representative copies |

FR-009-AC-5 and FR-273-AC-4 use `compile_fail` doctests; FR-010-AC-6 uses owner
FR-358-AC-3 Inspection of the nonzero occurrence type. TC-198 and
TC-199 use the gate targets named under Evidence Locations. The retired TC-016 four-code
Rust tests and tags are absent. FR-006-AC-6 is an ownership inspection of the direct
typed kernel outcome boundary; quire-exact FR-096-AC-8 owns the kernel code/cause census.
Inspection of `src/scalar/boolean.rs` and `src/exact/expression.rs` confirms that the runtime's
Boolean evaluator, `Body`, and `Evaluation.outcome` use `quire_exact::Outcome` and its
`Refusal` reasons directly; `CheckRefusal`, `InputRefusal`, and `EvaluationRefusal` report
checking or pre-evaluation input rejection rather than a second outcome-reason carrier.
`quire matrix` reports trace presence, not semantic coverage across repositories. The
retained RT charge-point driver binds FR-010-AC-1 and observes one-pair reservation
behavior. The complete injected record and simultaneous ordinary-limit precedence
are directly evidenced by quire-exact FR-358-AC-12/13, not by the retained RT test.
RT still needs to refresh its older quire-exact revision before the consumption claim
can be marked complete.

## Evidence at the kernel move

FR-275 requires RT to consume `quire-exact` directly. This code slice removes the
local scalar kernel and its duplicate tests. RT evaluation residue remains for open IR-349
step 2, IR-583 and IR-590 RT target acceptance. The status rows above describe the current tree: owner tests bind
matching kernel behaviors in quire-exact, while the RT coverage matrix only counts local
trace tags. FR-010-AC-2/3 now have direct owner evidence at the public meter under
quire-exact FR-358-AC-12/13 and TC-906. RT's dependency refresh is separate work;
this spec edit does not change Cargo.lock or prove that RT resolves the newer revision.
FR-362-AC-10 remains planned and untagged (IR-667), and QSL two-sided agreement
remains IR-669 work.

The retained TC-032 lazy connective test is RT-owned under FR-011-AC-3/AC-8. RT also
retains quantity, graph, declaration and function-package residue pending its assigned
follow-up work. No deleted kernel test is represented as an RT/QSL agreement test.
The table records what this slice actually changed; future dispositions are stated separately.

| Test file | Test cases | Disposition | Reason |
|---|---|---|---|
| `tests/exact_function_application.rs` | TC-194, TC-195 | retained | RT retains identity-free function application and backend negotiation tests; producer-origin argument and refusal controls move to IR-1175, while agreement remains IR-669 work |
| `tests/exact_negotiation.rs` | TC-030 | retained | RT backend negotiation stays |
| `tests/exact_vocabulary.rs` | TC-033 | retained | RT retains definition/selection controls and identity-free source inspections; enum/graph producer-origin controls move to IR-1175 |
| `tests/exact_equality.rs` | TC-026 | removed in IR-882 | duplicate RT evaluator tests retired with the local source; TC-026 retained as a planned test case and TC-203 supplies current QSV consumer evidence |
| `tests/exact_composite.rs` | TC-024 | removed in IR-882 | duplicate RT composite tests retired with local source; TC-024 retained pending owner and consumer mapping |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018, TC-019, TC-023 | removed | duplicate scalar kernel test files; matching owner cases are cited in the TC documents, while agreement and partial owner gaps stay open |
| `tests/exact_outcomes.rs` | TC-031 | one retained test | the residue charge-point driver remains; TC-016/017 kernel tests and stale four-code tags were removed |
| `tests/exact_collection.rs` | TC-025 | removed in IR-882 | duplicate RT collection tests retired with local source; TC-025 retained pending owner and consumer mapping |
| `tests/exact_meter_state.rs` | TC-032 | retained subset | RT's lazy connective test remains; quantity and graph cases left with QSV ownership |
| `tests/exact_semantics.rs` | TC-034 | retained subset | RT retains rounding, single-member division and Euclidean `mod`; QSV quantity consumer cases move to IR-1175 |
| `tests/exact_quantity_conversion.rs`, `tests/exact_quantity_owner.rs`, `tests/exact_unit_owner.rs` | TC-201, TC-033 | RT fixture cleanup IR-1162 | Identity-dependent QSV consumer controls move to quire-integration IR-1175; RT retains TC-035 dimensionless CompoundUnit Debug control |
| `tests/exact_debug_parity.rs` | TC-035 | removed | RT's boxed `CompoundUnit` owner was deleted; `tests/exact_unit_owner.rs` pins the consumed QSV rendering instead |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031, TC-032 | removed | in-crate tests of the deleted RT kernel implementation |

For the kernel, RT keeps the consumption checks TC-197 to TC-199 in the end state: direct
consumption, no copied code, guarded edges and direct inspection of AC-5 lock entries. RT also keeps evidence of its own lazy connective
in TC-032 and backend negotiation in TC-030/TC-195; kernel deletion shall not remove those checks.

## Evidence Locations

- TC-198: `deny.toml` (`[sources] unknown-git = "deny"`, the three shared own-repository
  `allow-git` entries, the dev-only first-party `ix-trace-rs` tooling entry and
  `[graph] all-features = true`) and the Rust `check_deny_source` binary, run by
  `make deny-mutations`, which adds QSL Git dependencies in normal, build and dev tables in scratch
  copies and requires cargo-deny's `source-not-allowed` error for that repository. TC-199:
  `tools/runtime-gates/` (row `build-exact-no-std-msrv`, `make test-features`),
  `make msrv` and `make size` (linked band, panic relocations and the footprint-graph check that
  `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive` are absent).
  Neither has a Rust test.
- TC-194, TC-195: `tests/exact_function_application.rs` (`--features exact`), landed under
  agent-ix/quire-contract-runtime#34. FR-273-AC-4's evidence is a `compile_fail` doctest on
  `IeeeDisposition` (`src/scalar/ieee.rs`), mirroring `InjectedDenial`'s.
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
- TC-201 (IR-818): `tests/exact_quantity_conversion.rs` (`--features exact`).
- TC-030: `tests/exact_negotiation.rs`; retained TC-032 RT quantity, lazy and graph
  cases: `tests/exact_meter_state.rs`; TC-033: `tests/exact_vocabulary.rs`;
  retained and extended TC-034 cases: `tests/exact_semantics.rs`; retained TC-035
  residue Debug cases: `tests/exact_debug_parity.rs`.
- TC-031: the retained residue charge-point driver in `tests/exact_outcomes.rs`.
  TC-016/017 kernel tests, TC-023 arithmetic tests and TC-018/019 allocation tests
  left with the RT kernel copy. Their matching owner evidence is in quire-exact;
  the RT/QSL and owner gaps identified above remain open.
- TC-018 through TC-022: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done). TC-018/TC-019 local allocation tests were removed in this slice. Owner tests cover matching kernel cases but do not close the remaining agreement gap. Owner mappings are in the individual TC documents.
- TC-024, TC-025 and TC-026 remain as test-case artifacts after IR-882 removed the duplicate RT source tests; TC-203 holds the current QSV equality consumer controls.
