---
id: TM-001
title: "Exact subsystem test matrix"
type: TestMatrix
---

# Exact subsystem test matrix

The producer-origin RT consumer cases removed by IR-1162 run in
[`quire-integration` TC-096](https://github.com/agent-ix/quire-integration/pull/31).
Its `producer_origin_identity`, `producer_origin_quantity_residual`,
`producer_origin_equality`, and `producer_origin_function` test targets obtain
identities from a live QSL checked-package producer and the IR reader. The PR's
moved-symbol map names each restored assertion and its observed result. The
RT source inspection `tc_033_rt_does_not_construct_semantic_identities` remains
here. Rows below retain 🚧 where a separate criterion or owner mapping is open.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-006 | FR-006-AC-1 | TC-016 | ✅ `consumed_exact_outcomes_keep_false_distinct_from_all_stops` checks all four dispositions, completed `false` and closed kernel reason enums through the RT consumer. |
| FR-006 | FR-006-AC-6 | — (Inspection) | 🚧 owner-backed by quire-exact FR-096-AC-8 for code/cause; RT consumes typed `Outcome::Refused(Refusal)` directly through `src/exact/mod.rs`, but no RT-local inspection binder remains after the stale four-code test was removed |
| FR-006 | FR-006-AC-3 | TC-016, TC-017 | ✅ RT consumer tests compare every public `ChargePoint` and `LimitKind` with QSpec vectors, including distinct `equality.plan-form`, and check the corrected 130-bit arithmetic denial, high-water and cumulative counters, first-short field order, refusal before retention, and exact counters past the charge-log cap. Each test is traced to AC-3; quire-exact FR-368-AC-1/2 and FR-359-AC-6/7 own the kernel behavior. |
| FR-006 | FR-006-AC-4 | TC-017 | ✅ `every_named_charge_point_can_be_denied_without_mutating_meter` injects a work-unit denial at every public charge point, checks the exact `Incomplete` record and unchanged meter; `charge_log_truncation_preserves_counters_and_late_denials` checks a denial after the log cap. quire-exact FR-358-AC-1/2 and FR-362-AC-11 retain owner evidence. |
| FR-006 | FR-006-AC-7 | — (Inspection) | 🚧 planned: RT-local host floating-point inspection has no bound evidence symbol |
| FR-006 | FR-006-AC-8 | TC-199 | 🚧 planned: `build-exact-no-std-msrv` exists but has no bound criterion evidence symbol |
| FR-006 | FR-006-AC-9 | — (`compile_fail` and Inspection) | 🚧 planned: per-child private-path doctests and direct-public-definition inspection are not yet bound |
| FR-006 | FR-006-AC-10 | — (Inspection) | 🚧 planned: RT-local non-test panic-site inspection has no bound evidence symbol |
| FR-008 | FR-008-AC-3, FR-008-AC-4 | — | 🚧 RT collection and canonical-key consumer evidence remains planned; owner mapping is outstanding |
| FR-008 | FR-008-AC-13 | TC-024 | 🚧 RT duplicate ValueType removed; kernel deep metadata owner evidence requires mapping |
| FR-008 | FR-008-AC-10, FR-008-AC-11, FR-008-AC-12 | TC-096 (`quire-integration`) | 🚧 TC-096 `admitted_model_roots_set_the_reference_universe_and_distinct_owner_result` covers the foreign-reference owner/result control; RT consumer evidence in `tests/exact_collection_consumer.rs` covers AC-10, AC-11 and AC-12 for all four collection kinds; remaining kernel evidence is open. |
| FR-007 | FR-007-AC-8 | TC-034 | 🚧 RT-local tests cover all six literal spellings and their consumed `RoundingMode` mapping, the consumed enum's `Exact` default, all six modes on positive and negative ties, and explicit `Exact` refusal; compiler package omission selecting `Exact` remains unbacked here |
| FR-007 | FR-007-AC-13 | TC-035 | ✅ `tests/exact_unit_owner.rs` pins the consumed QSV `CompoundUnit` Debug rendering; other boxed exact-kernel Debug obligations are specified by quire-exact. |
| FR-007 | FR-007-AC-15 | TC-034 | ✅ RT-local test checks all three quotient-sign vectors, rounded coefficients and exact loss |
| FR-007 | FR-007-AC-18 | TC-034 | ✅ RT-local test checks all four Euclidean `mod` sign vectors and typed out-of-domain refusal |
| FR-007 | FR-007-AC-19 | TC-034 | ✅ RT-local test checks both admitted exposed members and both typed refusals under every division profile |
| FR-009 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | TC-030 | ✅ implemented |
| FR-009 | FR-009-AC-5 | TC-195 | ✅ implemented: the `compile_fail` doctest on `IeeeDisposition` in `src/scalar/ieee.rs` fails to compile a conversion into `Outcome`, and `cargo test --doc --features exact` runs it (plain `cargo test --doc` does not collect it); Quire has no doctest binder, so `quire coverage` lists the row as unbacked although the check is delivered |
| FR-010 | FR-010-AC-1 | TC-031 | ✅ retained RT residue charge-point driver; owner-backed by quire-exact FR-358-AC-1 for generic injection |
| FR-010 | FR-010-AC-6 | TC-031 | ✅ Implemented: backed by quire-exact test `injected_denial_occurrence_requires_nonzero` (`src/accounting.rs`, tagged FR-358-AC-3), which asserts `NonZeroU64::new(0)` is `None` and `InjectedDenial::occurrence` has type `NonZeroU64`; therefore occurrence zero cannot be constructed |
| FR-011 | FR-011-AC-3, FR-011-AC-8 | TC-032 | 🚧 RT retains its lazy-connective control; QSV quantity zero stop, unit graph refusal and term order consumer controls now run in quire-integration TC-096. Kernel FR-359-AC-7 and FR-364-AC-1 retain owner evidence. |
| FR-012 | FR-012-AC-6 | TC-033 | ✅ `tc_033_runtime_has_no_local_node_key_constructor` recursively inspects all RT Rust source, excluding test-only items; the local key, constructors and their parser/round-trip tests are deleted. RT library code contains no key construction or hex decoder. |
| FR-012 | FR-012-AC-8 | Inspection (RT); TC-033 (`quire-integration`) | ✅ RT source inspection `tc_033_rt_does_not_construct_semantic_identities` rejects byte- and part-built fixture identities; TC-096's producer-origin tests obtain identities from a live QSL package admitted by IR. |
| FR-012 | FR-012-AC-3 | TC-033; TC-096 (`quire-integration`) | 🚧 RT retains AC-3 definition and refusal controls. |
| FR-012 | FR-012-AC-7 | TC-033 | ✅ `tc_033_definition_refusals_have_only_the_qsv_owner` checks deletion of the RT module and its export, and checks QSV's three valid `invalid_package` causes; source inspection confirms no RT alias, re-export or adapter and direct QSV test imports. |
| FR-273 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6 | TC-194 | 🚧 RT retains linked-only application, arity, wrong-kind, unknown-function and child-call controls. Identity-dependent dangling-reference and competing-refusal ordering now run in quire-integration TC-096. |
| FR-273 | FR-273-AC-8 | TC-194 | ✅ implemented: foreign expressions, absent functions requested within a checked body, meter re-borrows and direct host re-entry retain their distinct `ForeignCheckedExpression`, `UnknownCheckedFunction`, `MeterBorrowConflict` and `PackageReentry` checked-invariant causes. TC-194 asserts the foreign path admits no charge, a missing function or meter re-borrow inside one admitted outer call retains exactly that one `function.call` charge and one work unit, and host re-entry adds no nested charge. |
| FR-273 | FR-273-AC-12, FR-273-AC-13 | TC-194 | 🚧 PLANNED/UNRUN (IR-1009, blocked by IR-590; AD-004 step 5): no_std checked-input and generated-body consumer agreement and 100,000-call/512-KiB work-fuel acceptance. |
| FR-273 | FR-273-AC-7, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11 | TC-194 | ✅ RT-local scheduler evidence: `tc_194_explicit_frames_reach_4096_calls_on_64_kib_stack_with_exact_fuel` checks depths 1, 128 and 4,096, one active transition, exact and one-less `work_units` limits, charge counters and logs, and a retry after exhaustion. `tc_194_nested_arguments_charge_before_enclosing_body_and_stop_at_one_less` checks left/right/enclosing charge order and the denied enclosing charge. `tc_194_invalid_child_arguments_refuse_before_child_charge_or_body` and `tc_194_stopped_child_does_not_resume_parent` check child validation and stop propagation; the deep-call test checks the returned child value. The runtime scheduler in `src/exact/expression.rs` returns to its loop between transitions. CG-generated-body agreement and QSL-owned leaf consumption remain separate work under FR-273-AC-12/13. |
| FR-273 | FR-273-AC-4 | TC-195 | ✅ implemented: `negotiate_ieee(&[IeeeItemRequirement], &IeeeBackendCapabilities)` receives no `Meter` at all, so no application-time charge is reachable from it by construction — the evidence is that signature plus the `compile_fail` doctest on `IeeeDisposition` (`src/scalar/ieee.rs`) proving no conversion path from a disposition into `Outcome`/`InputRefusal` exists. `tc_195_negotiate_ieee_takes_no_meter_by_signature` inspects that signature and confirms negotiation still runs and reports one disposition per requirement; it carries no `Meter` assertion of its own, since a `Meter` never passed to `negotiate_ieee` cannot be evidence of anything the call did |
| FR-275 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | TC-197 | 🚧 RT now consumes the kernel Value, ValueType, CollectionType, equality planner and QSV checker directly, with no RT export for those names. TC-203 consumer-evaluator evidence now runs in quire-integration TC-096. IR-349 keeps independent AD-004 steps 1 through 4; IR-1009 receives step 5 and dependent empty-`exact` closure. AC-1, AC-13, AC-16 and AC-18 remain unmet while expression residue exists. AC-14 preserves TC-024 through  as planned artifacts after their duplicate source tests were removed. The footprint gate remains TC-199; TC-197 retains direct inspection of AC-5 lock entries. |
| FR-275 | FR-275-AC-7, FR-275-AC-8 | TC-198 | 🚧 `deny.toml` and `make deny-mutations` (Rust `check_deny_source`) contain the QSL Git source guard and normal, build and dev dependency mutation checks; the traced Rust test checks diagnostic discrimination, while the real dependency injections remain the gate evidence. AC-7 is an inspection; IR-499 remains open for strict-coverage evidence |
| FR-275 | FR-275-AC-29, FR-275-AC-30 | TC-199, TC-194 | 🚧 PLANNED/UNRUN (IR-1009, blocked by IR-590; AD-004 step 5): evaluation-leaf no_std target/host consumers and separate default versus exact-profile footprint receipts. |
| FR-275 | FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21 | TC-199 | 🚧 `make test-features`, `make size` and `make msrv` contain the exact no-std, footprint graph, size and toolchain checks; AC-20 is an inspection, while AC-9 through AC-11, AC-21 and TC-199 have no Quire test-symbol binder. IR-499 remains open for strict-coverage evidence |
| FR-275 | FR-275-AC-22 | TC-205 | ✅ `tests/ir882_qsv_vocabulary.rs` rejects local semantic-node vocabulary definitions and aliases, including renamed copies |
| FR-277 | FR-277-AC-7, FR-277-AC-8 | TC-203 | 🚧 AC-8 RT owner-deletion inspection remains; AC-7 Kani proof remains separate. |
| FR-278 | FR-278-AC-1 | — | 🚧 RT function producer controls remain to be bound. |
## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-016 | Inspect the exact outcome envelope and vocabulary | Unit | P0 | FR-006-AC-1, FR-006-AC-3 | ✅ AC-1 retains its tagged RT consumer outcome test; AC-3 has tagged RT `ChargePoint` and `LimitKind` vector comparisons, round-trips and undeclared-spelling refusals, with kernel owner evidence in FR-368. |
| TC-017 | Meter charges before work and deny them without effect | Unit | P0 | FR-006-AC-3, FR-006-AC-4 | ✅ `tests/exact_accounting_consumer.rs` checks all six steps through the public kernel API, including the 130-bit denial, counter semantics, first-short record, refusal ordering, injected denial per charge point, and log cap behavior. |
| TC-024 | Construct composite values and their declaration environment | Unit | P0 | FR-008-AC-13 | 🚧 retained test case; kernel deep metadata owner evidence requires mapping |
| TC-030 | Dispose negotiation items independently and in input order | Unit | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4 | ✅ implemented |
| TC-031 | Fire one injected denial with a limit-independent record | Unit | P0 | FR-010-AC-1, FR-010-AC-6 | 🚧 local RT evidence is limited to the residue charge-point driver; quire-exact owns the generic nonzero occurrence control. |
| TC-032 | Read a determinate meter state at every stop | Unit | P0 | FR-011-AC-3, FR-011-AC-8 | 🚧 RT retains its lazy-connective control. |
| TC-033 | Carry the compiler vocabulary byte-exactly | Integration | P0 | FR-012-AC-3, FR-012-AC-6, FR-012-AC-7, FR-012-AC-8 | 🚧 RT definition/refusal controls, constructor absence, owner-only refusal vocabulary and identity-fixture inspection remain. |
| TC-034 | Pin the retained exact scalar semantics | Unit | P0 | FR-007-AC-8, FR-007-AC-15, FR-007-AC-18, FR-007-AC-19 | 🚧 RT retains rounding, mod and integer-division consumer controls. |
| TC-035 | Pin the consumed CompoundUnit Debug rendering | Unit | P1 | FR-007-AC-13 | ✅ `tests/exact_unit_owner.rs` pins the consumed QSV `CompoundUnit` Debug rendering. |
| TC-194 | Apply checked functions with explicit frames and work fuel | Unit | P0 | FR-273-AC-1, FR-273-AC-2, FR-273-AC-3, FR-273-AC-6, FR-273-AC-7, FR-273-AC-8, FR-273-AC-9, FR-273-AC-10, FR-273-AC-11, FR-273-AC-12, FR-273-AC-13 | 🚧 RT retains linked-only input, explicit-frame, deep-call and work-fuel controls; identity-dependent plan/refusal controls now run in quire-integration TC-096. |
| TC-195 | Negotiate a function's undischargeable capability as unsupported | Unit | P0 | FR-273-AC-4, FR-009-AC-5 | ✅ implemented |
| TC-197 | Inspect that the runtime holds one kernel and no copy | Integration | P0 | FR-275-AC-1, FR-275-AC-2, FR-275-AC-3, FR-275-AC-4, FR-275-AC-5, FR-275-AC-12, FR-275-AC-13, FR-275-AC-14, FR-275-AC-15, FR-275-AC-16, FR-275-AC-17, FR-275-AC-18, FR-275-AC-19 | 🚧 planned across IR-349 (independent steps 1–4) and IR-1009 (step 5 and dependent end state): FR-275-AC-22 has its own inspection in TC-205. Function-application deletion awaits IR-590 target acceptance; step 7 fails while residue exists |
| TC-198 | Fail the build on a dependency from the QSL Git source | Integration | P0 | FR-275-AC-7, FR-275-AC-8 | 🚧 source rejection gate exists in `deny.toml` and `make deny-mutations`; the traced Rust test checks the diagnostic discriminator, and the real mutation gate supplies dependency evidence |
| TC-199 | Build the exact profile no_std and keep the default footprint | Integration | P0 | FR-006-AC-8, FR-275-AC-9, FR-275-AC-10, FR-275-AC-11, FR-275-AC-20, FR-275-AC-21, FR-275-AC-29, FR-275-AC-30 | 🚧 exact profile and footprint gates exist in `make test-features`, `make size` and `make msrv`; Quire has no test-symbol binder for TC-199 |
| TC-203 | Inspect checked equality ownership | Unit | P0 | FR-277-AC-7, FR-277-AC-8 | 🚧 AC-8 RT owner-deletion inspection remains; AC-7 Kani proof remains separate. |
| TC-205 | Inspect QSV semantic-node vocabulary ownership | Integration | P0 | FR-275-AC-22 | ✅ `tests/ir882_qsv_vocabulary.rs` inspects all runtime source for local semantic-node definitions and aliases and rejects representative copies |

FR-009-AC-5 and FR-273-AC-4 use `compile_fail` doctests; FR-010-AC-6 is backed by the
quire-exact test `injected_denial_occurrence_requires_nonzero` (FR-358-AC-3). TC-198 and
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
trace tags. FR-362-AC-10 remains planned and untagged (IR-667), and QSL two-sided agreement
remains IR-669 work.

The retained TC-032 lazy connective test is RT-owned under FR-011-AC-3/AC-8. RT also
retains quantity, graph, declaration and function-package residue pending its assigned
follow-up work. The table records the current source dispositions.

| Test file | Test cases | Disposition | Reason |
|---|---|---|---|
| `tests/exact_function_application.rs` | TC-194, TC-195 | retained | RT retains identity-free function application and backend negotiation tests; producer-origin argument and refusal controls now run in quire-integration TC-096, while agreement remains IR-669 work |
| `tests/exact_negotiation.rs` | TC-030 | retained | RT backend negotiation stays |
| `tests/exact_vocabulary.rs` | TC-033 | retained | RT retains definition/selection controls and identity-free source inspections; enum/graph producer-origin controls now run in quire-integration TC-096 |
| `tests/exact_equality.rs` | — | removed in IR-882 | duplicate RT evaluator tests retired with the local source; current QSV consumer evidence is in quire-integration TC-096 |
| `tests/exact_composite.rs` | TC-024 | removed in IR-882 | duplicate RT composite tests retired with local source; TC-024 retained pending owner and consumer mapping |
| `tests/exact_arithmetic.rs`, `tests/exact_allocation.rs` | TC-016, TC-017 | removed | duplicate scalar kernel test files; matching owner cases are in quire-exact |
| `tests/exact_outcomes.rs` | TC-031 | one retained test | the residue charge-point driver remains; TC-016/017 kernel tests and stale four-code tags were removed |
| `tests/exact_collection.rs` | — | removed in IR-882 | duplicate RT collection tests retired with the local source |
| `tests/exact_meter_state.rs` | TC-032 | retained subset | RT's lazy connective test remains; quantity and graph cases left with QSV ownership |
| `tests/exact_semantics.rs` | TC-034 | retained subset | RT retains rounding, single-member division and Euclidean `mod`; QSV quantity consumer cases now run in quire-integration TC-096 |
| `tests/exact_quantity_conversion.rs`, `tests/exact_quantity_owner.rs`, `tests/exact_unit_owner.rs` | TC-033 | RT fixture cleanup IR-1162 | Identity-dependent QSV consumer controls now run in quire-integration TC-096; RT retains TC-035 dimensionless CompoundUnit Debug control |
| `tests/exact_debug_parity.rs` | TC-035 | removed | RT's boxed `CompoundUnit` owner was deleted; `tests/exact_unit_owner.rs` pins the consumed QSV rendering instead |
| `src/exact_accounting_tests.rs`, `src/exact_integer_tests.rs` | TC-031, TC-032 | removed | in-crate tests of the deleted RT kernel implementation |

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
- TC-030: `tests/exact_negotiation.rs`; retained TC-032 RT quantity, lazy and graph
  cases: `tests/exact_meter_state.rs`; TC-033: `tests/exact_vocabulary.rs`;
  retained TC-034 cases: `tests/exact_semantics.rs`; TC-035 `CompoundUnit` Debug evidence:
  `tests/exact_unit_owner.rs`.
- TC-031: the retained residue charge-point driver in `tests/exact_outcomes.rs`.
  TC-016/017 kernel tests left with the RT kernel copy. Their matching owner evidence is in quire-exact.

