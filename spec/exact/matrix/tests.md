---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1 | TC-016 | 🚧 planned: owner FR-362-AC-11 tests scalar stop dispositions, but the closed-reason type census and RT consumer check have no local binder |
| FR-006 | FR-006-AC-6 | — (Inspection) | 🚧 owner-backed by quire-exact FR-096-AC-8 for code/cause; RT consumes typed `Outcome::Refused(Refusal)` directly through `src/exact/mod.rs`, but no RT-local inspection binder remains after the stale four-code test was removed |
| FR-006 | FR-006-AC-2 | TC-020, TC-021, TC-022 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, and its evidence with it |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | 🚧 owner-backed by quire-exact FR-368-AC-1/2 for vocabulary and FR-359-AC-6/7 for meter ordering; local kernel tests removed |
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
| FR-008 | FR-008-AC-1, FR-008-AC-2 | TC-024 | ✅ implemented |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | TC-025 | ✅ implemented |
| FR-008 | FR-008-AC-5, FR-008-AC-6 | TC-026 | 🚧 current RT controls are tagged; AC-5 kernel behavior changes and TC-026 rewrite remain planned for IR-882; AC-6 current triple is implemented, kernel triple planned |
| FR-008 | FR-008-AC-7, FR-008-AC-8 | TC-024, TC-025, TC-026 | ✅ 63-point kernel census including `temporal.visit` pinned by RT; retained RT tests check the twelve added denial points and residue behavior |
| FR-008 | FR-008-AC-9 | TC-024, TC-025 | ✅ implemented against RT's current `Value`; kernel-valued test rewrite planned in IR-882 |
| FR-008 | FR-008-AC-13 | TC-024 | ✅ implemented: deep metadata clone, equality, formatting and drop in a small-stack thread and default-stack child process |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-025 | 🚧 AC-10 membership charges and AC-11 retention are tested for all four kinds in `tests/exact_collection.rs`; decimal tests check ordered-set first-occurrence retention and bag multiplicity using each member's first-supplied representation; AC-12 visiting order remains unbacked |
| FR-007 | FR-007-AC-8 | TC-034 | 🚧 RT-local tests cover all six literal spellings and their consumed `RoundingMode` mapping, the consumed enum's `Exact` default, all six modes on positive and negative ties, and explicit `Exact` refusal; compiler package omission selecting `Exact` remains unbacked here |
| FR-007 | FR-007-AC-9 | TC-034 | 🚧 owner-backed by quire-exact FR-366-AC-1..4 for IEEE NaNs, signed zero and order; RT-local kernel test removed |
| FR-007 | FR-007-AC-10 | TC-034 | 🚧 owner-backed by quire-exact FR-362-AC-12/13 for rational canonical form and membership; RT-local kernel test removed |
| FR-007 | FR-007-AC-11 | TC-034 | 🚧 owner-backed by quire-exact FR-363-AC-6/7 and `tests/ir673_decimal.rs`: equal normalized values, ordering size counters 8/3/1 versus 4/2/0, and one Boolean result unit in both comparisons; RT exact-feature binder remains planned after the local kernel test was removed |
| FR-007 | FR-007-AC-12 | TC-034 | 🚧 retained RT tests tagged with this criterion cover quantity type-fault order and cross-dimension arithmetic; single-member division exposure is in AC-19 and `mod` is in AC-18 |
| FR-007 | FR-007-AC-13 | TC-035 | ✅ implemented |
| FR-007 | FR-007-AC-15 | TC-034 | ✅ RT-local test checks all three quotient-sign vectors, rounded coefficients and exact loss |
| FR-007 | FR-007-AC-16 | TC-034 | ✅ RT-local test checks affine-unit `Power` refusal at zero and negative exponents with empty charges |
| FR-007 | FR-007-AC-17 | TC-034 | ✅ RT-local test checks `Multiply` and `Divide` on distinct non-affine units of the same dimension |
| FR-007 | FR-007-AC-18 | TC-034 | ✅ RT-local test checks all four Euclidean `mod` sign vectors and typed out-of-domain refusal |
| FR-007 | FR-007-AC-19 | TC-034 | ✅ RT-local test checks both admitted exposed members and both typed refusals under every division profile |
| FR-007 | FR-007-AC-20 | TC-201 | ✅ RT-local tests check the U01, U04, U22 `m2`, U25 second-step and U26 magnitudes and units, U09b, and an `i128` fraction sweep against an independent rational oracle |
| FR-007 | FR-007-AC-21 | TC-201 | ✅ RT-local test checks `IncompatibleDimensions` before any charge for `m` to `s`, the shape-alike dimension node, `N_m` to `J` and compound `m^2` to `m` |
| FR-007 | FR-007-AC-22 | TC-201 | ✅ RT-local test checks `0 degC` to `K`, `1 u2` to `u1`, an affine target and an affine-to-affine conversion are admitted exactly |
| FR-007 | FR-007-AC-23 | TC-201 | ✅ RT-local test checks the charge order and consumed amounts of U10, U15, U22 `m2` and `cm2`, U25 both steps and U26 |
| FR-007 | FR-007-AC-24 | TC-201 | ✅ RT-local test checks the U05 and U20 `AffineUnitArithmetic` refusals with zero charges and the non-affine `3 u2` sum |
| FR-007 | FR-007-AC-25 | TC-201 | ✅ RT-local test checks the four denial tuples' `Incomplete` fields, including the U10 `result_units: 0` tuple, which executes to the derived value |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented (compile_fail doctest on `IeeeDisposition`, `src/exact/ieee.rs`) |
| FR-010 | FR-010-AC-1 | TC-031 | ✅ retained RT residue charge-point driver; owner-backed by quire-exact FR-358-AC-1 for generic injection |
| FR-010 | FR-010-AC-4 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1 for named admitted occurrence; RT-local kernel test removed |
| FR-010 | FR-010-AC-5 | TC-031 | 🚧 owner-backed by quire-exact FR-358-AC-1/2 for one-shot retry; RT-local kernel test removed |
| FR-010 | FR-010-AC-6 | TC-031 | ✅ owner-backed by quire-exact FR-358-AC-3 Inspection of nonzero occurrence type |
| FR-010 | FR-010-AC-2 | owner TC-906; TC-031 RT driver only | 🚧 owner quire-exact FR-358-AC-12 has direct public-meter record tests; RT TC-031 does not repeat the two-limit case, and RT's lockfile still resolves the older kernel revision and needs a separate refresh |
| FR-010 | FR-010-AC-3 | owner TC-906; TC-031 RT driver only | 🚧 owner quire-exact FR-358-AC-13 has direct public-meter ordinary-limit precedence tests; RT TC-031 does not test competing refusals, and RT's lockfile still resolves the older kernel revision and needs a separate refresh |
| FR-011 | FR-011-AC-1 | TC-032 | 🚧 owner-backed by quire-exact FR-362-AC-11 for scalar stop prefixes; RT-local kernel test removed |
| FR-011 | FR-011-AC-4 | TC-032 | ✅ retained RT field-order test; owner-backed by quire-exact FR-359-AC-6 for complete meter scan |
| FR-011 | FR-011-AC-5 | TC-032 | 🚧 owner-backed by quire-exact FR-358-AC-8..11 for the first 4096 log entries, truncation and post-cap enforcement; RT-local kernel test removed |
| FR-011 | FR-011-AC-6 | TC-032 | 🚧 partly evidenced: retained RT `tests/exact_meter_state.rs` checks a quantity-power denial before computation, `next_charge > u64::MAX` and the admitted prefix, but does not assert the exact amount or every counter; quire-exact FR-359-AC-1/3/4/5 owns cumulative and exact over-`u64` refusal, while QSpec TC-187 U13 states the exact quantity vector |
| FR-011 | FR-011-AC-2, FR-011-AC-3, FR-011-AC-7, FR-011-AC-8 | TC-032 | ✅ RT quantity, lazy connective and graph-order tests remain; owner FR-359-AC-7 covers `consumed` for every `LimitKind::ALL` member and FR-364-AC-1 the IEEE flag subset |
| FR-012 | FR-012-AC-1 | — (retired) | 🚧 retired with deletion of RT-local `NodeKey` and its constructor tests in the IR-882 key-owner slice. Original criterion text and ID remain reserved. |
| FR-012 | FR-012-AC-6 | TC-033 | ✅ `tc_033_runtime_has_no_local_node_key_constructor` inspects RT's key module and public surface; the local key, constructors and their parser/round-trip tests are deleted. RT library code contains no key construction or hex decoder. |
| FR-012 | FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5 | TC-033 | ✅ implemented for the current RT vocabulary; evidence leaving with the residue will be reclassified in its cutover |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | ✅ implemented: AC-1's linked-only application is proved by `tc_194_kernel_check_is_refused_so_no_kernel_package_is_applicable` (a package `check` rejects under `CheckMode::Kernel` is never applicable — that is AC-6's inspection too) together with `tc_194_linked_package_applies_every_declared_function` and the rest of this corpus's `Linked`-application tests, which call only a package `check` admitted under `CheckMode::Linked`. AC-2/AC-3's "arity before any per-argument check, all before the `function.call` charge, before the body" ordering is covered for both `call` and `Frame::call` |
| FR-273 | FR-273-AC-8 | TC-194 | ✅ implemented: foreign expressions, absent functions requested within a checked body, and meter re-borrows retain their distinct typed checked-invariant causes. TC-194 asserts the foreign path admits no charge, while a missing function or meter re-borrow inside one admitted outer call retains exactly that one `function.call` charge and one work unit. |
| FR-273 | FR-273-AC-7, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11 | TC-194 | 🚧 planned (IR-512): resumable generated bodies, explicit call frames, bounded active transitions across deep calls, independently specified exact/one-less `work_units` fuel, and nested charge order have no implementation evidence yet. Three existing depth-limit tests assert superseded behavior; their former AC-7/TC-194 trace tags are removed until those tests are replaced. |
| FR-273 | FR-273-AC-5 | TC-194 | 🚧 partly evidenced: the QSL shared-corpus half is removed from this repository, recreation in agent-ix/quire-integration is planned with open quire-integration ticket IR-669 (IR-430 removal is Done); the runtime-only ordering tests remain. AC-5 quantifies over shared-corpus function-application vectors only, and the removed shared corpus historically agreed on all five of them — the closed `InputRefusal` vocabulary (with codes and causes), the charge count of one admitted call, and — via AP01–AP04's `charges == 0` assertions on each refusal path (the removed QSL shared-corpus vectors; see Evidence Locations) — that every refusal precedes the `function.call` charge, agreed on both sides. Relative order *among* the four checks themselves (arity, value kind, dangling reference, unknown function) is not something any vector needs to discriminate for AC-5 to be met, since each corpus vector isolates exactly one violation by design; that ordering is instead verified by the runtime-only tests in `tests/exact_function_application.rs` (see Evidence Locations), which AC-2/AC-3 already cover. Body semantics have no shared corpus either, for the same reason: AC-5 does not claim them. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/exact/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 planned (Linear IR-349): optional `quire-exact` and `quire-semantic-value` edges resolve once each; the transitive canonical crates also resolve once (AC-3 to AC-5). The separate TC-202 checks the one-copy gate against duplicate first-party Git lock entries (AC-6). The prior code slice removed duplicate scalar, Meter and Outcome definitions and tests. AC-1 remains unmet because RT still exports residue names such as `Value`, `ValueType` and `CollectionType` that quire-exact exports. AC-2 holds for definitions removed in that slice: RT has no alias or feature restoring its former scalar, Meter or Outcome definitions; its new QSV identity and declaration owner clauses are planned for IR-882 and have no acceptance evidence yet. The remaining residue is not an approved exception. AC-12 holds for the current test and dependency graph: RT tests invoke no second kernel and no QSL Git repository crate. AC-13, AC-16 and AC-18 remain unmet while RT evaluation and function-package residue and the `exact` module remain under open IR-349 step 2, IR-583 and IR-590 RT target acceptance; interface-001-AC-7 is likewise unmet while RT defines `Frame`, `Body`, `CheckedPackage`, `Evaluation` and `plan_call`. AC-14, AC-15, AC-17 and AC-19 require current-tree inspection; AC-17 names only `scalar` as the permitted exception. See the interface-001 rows in the core matrix |
| FR-275 | FR-275-AC-6 | TC-202 | ✅ `tests/deny_one_copy.rs` exercises `make deny` against duplicate lock entries for every resolved first-party Git crate and a synthetic first-party name, with the cargo-deny stage isolated |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | 🚧 `deny.toml` and `make deny-mutations` (`scripts/check_deny_bans.sh`) contain the QSL Git source guard and normal, build and dev dependency mutation checks; AC-7 is an inspection, while AC-8 and TC-198 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | 🚧 `make test-features`, `make size` and `make msrv` contain the exact no-std, footprint graph, size and toolchain checks; AC-20 is an inspection, while AC-9 through AC-11, AC-21 and TC-199 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
| FR-277 | FR-277-AC-1, FR-277-AC-2, FR-277-AC-3, FR-277-AC-4, FR-277-AC-5, FR-277-AC-6, FR-277-AC-7, FR-277-AC-8 | TC-203 | 🚧 Planned: IR-882 migrates to `quire_exact::Value`, deletes RT's pair walker and routes the real evaluator to the result-bearing kernel call; no AC-1 tag until that path runs; AC-8 inspects the direct call; AC-7 Kani lane owner titan:kani:plan |
| FR-278 | FR-278-AC-1, FR-278-AC-2 | TC-204 | 🚧 Planned: overlapping TC-194/TC-026 assertions exist, FR-278 tags and remaining typed-cause controls pending IR-497 |
## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3 | 🚧 planned: owner FR-362-AC-11 tests scalar stops and FR-368-AC-1/2 tests vocabulary; the complete RT consumer checks have no local binder |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | 🚧 local kernel meter tests removed; matching owner evidence is FR-358/359/368 |
| TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 local allocation tests removed; owner dividend-bit exact/one-under evidence exists, arithmetic `max(bits(a), bits(b))` remains unproved (IR-678), and two-sided agreement remains IR-669 work |
| TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 local allocation tests removed; owner FR-361-AC-9 tests the denied upscale, and two-sided agreement remains IR-669 work |
| TC-020 | Agree with the authority on IEEE profile vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-3, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-021 | Agree with the authority on text and enum vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-4, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-022 | Agree with the authority on quantity and unit vectors | Integration | P0 | FR-006-AC-2, FR-007-AC-5, FR-007-AC-6 | 🚧 planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done): the QSL agreement oracle is removed from this repository, no test evidences it here |
| TC-023 | Meter integer, rational, ordering and Boolean operations | Property | P0 | FR-007-AC-7 | 🚧 owner FR-362-AC-10 remains planned and untagged (IR-667); shared-authority agreement remains IR-669 work |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-13 | ✅ current RT construction tests remain; RT pins the 63-point kernel census and `temporal.visit`; kernel-value rewrite planned in IR-882; owner FR-368-AC-1/2 tests full vocabulary round trips |
| TC-025 | Construct collections and order them by the canonical key | Property | P0 | FR-008-AC-3, FR-008-AC-4, FR-008-AC-7, FR-008-AC-8, FR-008-AC-9, FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | 🚧 AC-10 and AC-11 are tested for all four kinds, including bag multiplicity and first-supplied member representations; AC-12 remains unbacked |
| TC-026 | Evaluate the equality matrix and terminal references | Unit | P0 | FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-8 | 🚧 current RT cases run; IR-882 kernel value and changed-result cases planned |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, FR-010-AC-6 | 🚧 local RT evidence is limited to the residue charge-point driver under AC-1; AC-2/3 owner evidence is FR-358-AC-12/13 and TC-906, while RT's older kernel revision still needs a separate refresh |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | 🚧 RT quantity, lazy connective and graph cases remain; owner tests cover matching kernel meter subsets |
| TC-033 | Carry the compiler vocabulary byte-exactly | Unit | P0 | FR-012-AC-2, FR-012-AC-3, FR-012-AC-4, FR-012-AC-5, FR-012-AC-6 | ✅ key-owner cutover inspection and remaining runtime vocabulary checks; FR-012-AC-1 retired with its local constructor tests. The wider Value cutover remains planned. |
| TC-034 | Pin the exact semantics the agreement corpus does not reach | Unit | P0 | FR-007-AC-8, FR-007-AC-9, FR-007-AC-10, FR-007-AC-11, FR-007-AC-12, FR-007-AC-15, FR-007-AC-16, FR-007-AC-17, FR-007-AC-18, FR-007-AC-19 | 🚧 RT tests cover AC-8 spellings, consumed enum default and directed rounding, AC-12 quantity, and AC-15 through AC-19; compiler package omission for AC-8 and AC-9 through AC-11 remain unbacked here; owner evidence includes quire-exact FR-357-AC-2/3/7, FR-362/363/365/366 |
| TC-035 | Pin boxed value and type structs' hand-written Debug rendering | Unit | P1 | FR-007-AC-13 | ✅ implemented |
| TC-194 | Apply checked functions with explicit frames and work fuel | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-5, FR-273-AC-6, FR-273-AC-7, FR-273-AC-8, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11 | 🚧 existing input and ordering evidence remains; resumable-body, explicit-frame and work-fuel cases are planned under IR-512 |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | 🚧 planned (Linear IR-349): the kernel copy is deleted in part 1 and the residue in part 2 (the IR-882 QSV owner inspection in step 1 remains planned and untagged; remaining RT deletion owned by open IR-349 step 2; function-application deletion awaits IR-590 and RT target acceptance after IR-583; QSL-358 extraction is Done); step 7 fails while the residue exists |
| TC-198 | Fail the build on a dependency from the QSL Git source | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | 🚧 source rejection gate exists in `deny.toml` and `make deny-mutations`; Quire has no test-symbol binder for TC-198 |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-006-AC-8, FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | 🚧 exact profile and footprint gates exist in `make test-features`, `make size` and `make msrv`; Quire has no test-symbol binder for TC-199 |
| TC-201 | Convert a quantity exactly between units of one dimension | Unit | P0 | FR-007-AC-20, FR-007-AC-21, FR-007-AC-22, FR-007-AC-23, FR-007-AC-24, FR-007-AC-25 | ✅ RT-local `tests/exact_quantity_conversion.rs` (IR-818); QSL agreement stays IR-669 (TC-022) |
| TC-202 | Reject duplicate first-party Git lock entries through make deny | Integration | P0 | FR-275-AC-6 | ✅ `tests/deny_one_copy.rs` checks the real Make target and its one-copy diagnostic in a scratch repository holding the working-tree gate files |
| TC-203 | Exercise checked equality through the kernel plan and runtime meter | Unit | P0 | FR-277-AC-1, FR-277-AC-2, FR-277-AC-3, FR-277-AC-4, FR-277-AC-5, FR-277-AC-6, FR-277-AC-7, FR-277-AC-8 | 🚧 Planned: IR-882 kernel-valued evaluator route and RT controls pending; AC-1 cannot be credited by a kernel-only test; AC-8 requires inspection; AC-7 remains the separate titan:kani:plan claim |
| TC-204 | Retag runtime checked-invariant producer controls | Unit | P0 | FR-278-AC-1, FR-278-AC-2 | 🚧 Planned: FR-278 tags and remaining controls; existing overlap is TC-194 and `src/exact/equality.rs` unit tests tagged FR-008-AC-5 |

FR-009-AC-5 and FR-273-AC-4 use `compile_fail` doctests; FR-010-AC-6 uses owner
FR-358-AC-3 Inspection of the nonzero occurrence type. TC-198 and
TC-199 use the gate targets named under Evidence Locations. The retired TC-016 four-code
Rust tests and tags are absent. FR-006-AC-6 is an ownership inspection of the direct
typed kernel outcome boundary; quire-exact FR-096-AC-8 owns the kernel code/cause census.
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
| `tests/exact_function_application.rs` | TC-194, TC-195 | retained | RT function-package residue and backend negotiation tests remain; FR-273 agreement is still IR-669 work |
| `tests/exact_negotiation.rs` | TC-030 | retained | RT backend negotiation stays |
| `tests/exact_vocabulary.rs` | TC-033 | retained | RT vocabulary residue remains for later migration |
| `tests/exact_equality.rs` | TC-026 | retained | RT checking-environment and equality-plan tests remain in this slice |
| `tests/exact_composite.rs` | TC-024 | retained | RT construction and declaration-environment tests remain in this slice |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017, TC-018, TC-019, TC-023 | removed | duplicate scalar kernel test files; matching owner cases are cited in the TC documents, while agreement and partial owner gaps stay open |
| `tests/exact_outcomes.rs` | TC-031 | one retained test | the residue charge-point driver remains; TC-016/017 kernel tests and stale four-code tags were removed |
| `tests/exact_collection.rs` | TC-025 | retained | all eight collection and canonical-key tests remain in this slice; their RT `Value` and type behavior remains residue |
| `tests/exact_meter_state.rs` | TC-032 | retained subset | nine RT quantity, lazy connective and graph-order tests remain; kernel-only meter tests were removed |
| `tests/exact_semantics.rs` | TC-034 | retained and extended | RT quantity type-fault order and cross-dimension arithmetic cases remain; RT-local tests also cover consumed rounding, single-member division and Euclidean `mod` behavior |
| `tests/exact_quantity_conversion.rs` | TC-201 | added (IR-818) | new RT-local file testing the existing `convert_quantity` and `evaluate_quantity`; RT's copy is deleted in AD-004 step 4 and the test rebinds to the shared crate's `convert_quantity` |
| `tests/exact_debug_parity.rs` | TC-035 | retained subset | residue Debug cases remain; duplicate kernel Debug cases were removed |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-023, TC-031, TC-032 | removed | in-crate tests of the deleted RT kernel implementation |

For the kernel, RT keeps the consumption checks TC-197 to TC-199 and the one-copy gate check TC-202 in the end state: direct
consumption, no copied code, guarded edges and the duplicate-entry lock gate. RT also keeps evidence of its own lazy connective
in TC-032 and backend negotiation in TC-030/TC-195; kernel deletion shall not remove those checks.

## Evidence Locations

- TC-198: `deny.toml` (`[sources] unknown-git = "deny"`, the three shared own-repository
  `allow-git` entries, the dev-only first-party `ix-trace-rs` tooling entry and
  `[graph] all-features = true`) and `scripts/check_deny_bans.sh`, run by
  `make deny-mutations`, which adds QSL Git dependencies in normal, build and dev tables in scratch
  copies and requires cargo-deny's `source-not-allowed` error for that repository. TC-199:
  `scripts/run_feature_matrix.py` (row `build-exact-no-std-msrv`, `make test-features`),
  `make msrv` and `make size` (linked band, panic relocations and the footprint-graph check that
  `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive` are absent).
  Neither has a Rust test. TC-202: `tests/deny_one_copy.rs`, which copies the working-tree
  `Makefile`, `scripts/check_one_copy.awk` and `Cargo.lock` into a scratch repository and runs
  `make deny` against duplicate first-party Git lock entries.
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
- TC-024: `tests/exact_composite.rs`; TC-025: `tests/exact_collection.rs`; TC-026:
  `tests/exact_equality.rs`. All run with `--features exact`.
