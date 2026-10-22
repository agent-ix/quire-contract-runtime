---
id: SR-15435
title: IR-590 PR 156 evidence review
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-runtime; spec/exact/functional/FR-273-exact-function-application.md,
  spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-194-function-application.md,
  spec/exact/matrix/TC-198-qsl-crate-edge-guard.md, spec/exact/matrix/TC-199-exact-no-std-and-footprint.md,
  spec/exact/matrix/tests.md
review_set: subset
---

## Summary

Ran the Quoin method advisor for the changed obligations and compared TC-194/TC-199 procedures to their criteria. One exact-profile measurement remains underdefined. Ticket: IR-590. Reviewed head: fe329007e535aa1f9ebeb49a690434ced393f9cc.

## Examined scope

- `FR-273-AC-12` (examined; `spec/exact/functional/FR-273-exact-function-application.md`): After AD-004 step 5, the RT consumer invokes both an admitted checked input and a CG-generated resumable body through the same QSL-owned evaluation-leaf interface with `std` disabled; for each shared input the outcome or typed refusal, source location and loss, consumed work and ordered charge log equal QSL's std reference. No RT-local evaluator body or compatibility route supplies either result.
- `FR-273-AC-13` (examined; `spec/exact/functional/FR-273-exact-function-application.md`): On a native thread with a 512 KiB stack, the checked decreasing `count(100000)` call through the consumed no_std evaluator and a generated-body call for the same checked input each return `100000` with work limit `w`, where `w` is that run's measured spend; each returns `Incomplete` naming `work_units` at `w-1` with no denied charge in the counter or log, then returns `100000` again at `w`. The ordered charge sequence at all three limits matches the QSL std reference, and no depth refusal or native stack overflow occurs.
- `FR-275-AC-5` (examined; `spec/exact/functional/FR-275-single-exact-kernel.md`): `Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`, and, after AD-004 step 5, the QSL-owned evaluation leaf.
- `FR-275-AC-7` (examined; `spec/exact/functional/FR-275-single-exact-kernel.md`): `deny.toml` admits only the `quire-exact`, `quire-semantic-value` and `quire-canonical` own-repository Git sources and the dev-only `ix-trace-rs` first-party tooling source before AD-004 step 5, then also the QSL-owned evaluation leaf's own-repository Git source; it keeps `unknown-git = "deny"` and never admits the QSL Git repository.
- `FR-275-AC-10` (examined; `spec/exact/functional/FR-275-single-exact-kernel.md`): The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi` contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`, and after AD-004 step 5 contains no evaluation leaf.
- `FR-275-AC-29` (examined; `spec/exact/functional/FR-275-single-exact-kernel.md`): After AD-004 step 5, with `exact` enabled and `std` disabled on Rust 1.98.1, the RT consumer and a generated-body consumer of the QSL-owned evaluation leaf compile for `thumbv7em-none-eabi` using one lock entry for that leaf; neither consumer resolves the QSL workspace Git source, an RT-local evaluator copy, or a compatibility alias. A host build of the same no_std profile executes the checked-input and generated-body cases of FR-273-AC-12 and AC-13.
- `FR-275-AC-30` (examined; `spec/exact/functional/FR-275-single-exact-kernel.md`): After AD-004 step 5, `make size` still measures the default fixed population at 500 bytes through 4 KiB linked `.text` plus `.rodata` with no evaluation leaf in its graph and no panic relocation. The acceptance receipt reports the exact-profile linked bytes and native deep-test peak memory separately; neither is compared with the default 4 KiB ceiling, and no exact-profile memory limit is inferred from this default measurement.
- `TC-194` (examined; `spec/exact/matrix/TC-194-function-application.md`): 15. After AD-004 step 5, invoke one admitted checked input and one CG-generated resumable body through the QSL-owned leaf with `std` disabled. Run the same inputs through QSL's std reference. Compare the complete outcome or typed refusal, location and losses, consumed work, and ordered charge log; use one negative input whose first denied charge preserves the admitted prefix (FR-273-AC-12). 16. On a native thread limited to a 512 KiB stack, run the checked decreasing `count(100000)` and the corresponding generated-body case.
- `TC-198` (examined; `spec/exact/matrix/TC-198-qsl-crate-edge-guard.md`): After AD-004 step 5, admit the evaluation leaf's own repository if it uses Git. Never admit the QSL workspace Git source (FR-275-AC-7).
- `TC-199` (examined; `spec/exact/matrix/TC-199-exact-no-std-and-footprint.md`): After AD-004 step 5, repeat the no-std MSRV target build with the shared evaluation leaf and one generated-body consumer. Check the lock has one evaluation-leaf entry, and neither target graph resolves the QSL workspace source, an RT-local evaluator, or a compatibility alias. Run TC-194's checked-input and generated-body host cases with `std` disabled. Record the exact-profile target's linked `.text` plus `.rodata` bytes and the native deep-test peak memory in the receipt (FR-275-AC-29, AC-30).
- `TestMatrix-exact` (examined; `spec/exact/matrix/tests.md`): TC-194 now lists FR-273-AC-12 and FR-273-AC-13, but its status still says resumable-body, explicit-frame and work-fuel cases are planned under IR-512. TC-199 now lists FR-275-AC-29 and FR-275-AC-30.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-275-AC-30 asks the receipt to report exact-profile linked bytes and native deep-test peak memory, but TC-199 step 4 names no fixed linked exact-profile consumer artifact or peak-memory metric and measurement method. A bare target build does not identify the linked population, so two acceptance runs can report incomparable values or no linked value. Specify the consumer image and metric/measurement boundary while leaving exact-profile ceilings unset. | spec/exact/functional/FR-275-single-exact-kernel.md:210; spec/exact/matrix/TC-199-exact-no-std-and-footprint.md:31-37 |

## Verdict

The advisor reports Test matches AC-12, AC-13, AC-29 and AC-30. Its generic Inspection mismatch for AC-7 is not a defect: TC-198 inspects deny.toml and separately exercises the source-rejection gate.
