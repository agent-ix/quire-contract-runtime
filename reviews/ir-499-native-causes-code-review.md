---
id: SR-20261102049901
title: "IR-499 native cause/carrier code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime PR196; FR278AC1 native cause/fuel/carrier evidence leg only; five-file frozen diff"
review_set: subset
---

## Summary

Frozen original RT196 source5041aa33971def60186daf146eeebbaf21156a09 over main2f4c13a0. IR499 ticket, AGENTS/CLAUDE, actual five-file diff, FR278AC1/2, TC204 and relevant implementation/native controls were inspected. One code review with Rust lane plus one scoped gap pass. Only FR278 Status prose changes; normative ACs/relationships/contract are unchanged, so no new specification-authoring sub-analysis is required. No source/worktree edits, Cargo/Kani/CBMC/IEEE/backend runs or RT hosted-CI dispatch by reviewer. RT ci.yml unconditionally includes Kani and is explicitly not invoked. Native-only hosted disposition remains owner's decision, not implicit proof authorization.

Six tags attach FR278AC1 to existing discriminating native tests rather than to documentation stubs. Foreign checked expression is refused with exact ForeignCheckedExpression and no admitted charge (the allocator-reuse premise is explicitly asserted); missing generated child yields UnknownCheckedFunction with only the parent charge and no parent resumption; actual nested public Frame.meter reborrow yields MeterBorrowConflict. Invalid child arguments retain GeneratedArgumentShapeMismatch before child charge/body, and direct host re-entry retains PackageReentry with no nested charge. Explicit-frame calls reach depth4096 on64KiB native stack at exact fuel; N-1 yields OutcomeIncomplete with WorkUnits/FunctionCall, exact consumed/next charge and unchanged admitted-charge prefix, rather than CheckedInvariant. Existing package/function source producer branches were traced against these owning causes; unchanged defensive unreachable branches are not represented by fabricated native counterexamples.

The carrier clause is not discharged by a vocabulary grep or by the new positive tag alone. The same existing kernel-api-check package positively imports/constructs native UnknownCheckedFunction and separately has a compile_fail,E0432 import of CallDepthExceeded (import avoids assuming any historical variant payload). Existing MSRV Make gate executes both package unit tests and its doctests against the same resolved locked kernel; adding existing ix-trace-rs as dev-only dependency changes one package dependency-list lock line, not the kernel selection or production dependency graph. Together actual native cause/fuel controls plus positive/negative carrier compilation address the whole FR278AC1 criterion. The positive unit control by itself would not establish carrier absence or all native producer clauses.

Frozen computed matrix shows FR278AC1 tagged by seven actual functions, including the six native controls and kernel positive unit; its collective semantic evidence is assessed above. FR278AC2 remains untagged/open and TC204 as a whole remains planned: collection/composite ownership/evidence is not fabricated from former equality/quantity controls. FR278 Status correctly says PARTIAL and preserves that distinction. IR499's whole classification/strict-coverage acceptance and deferred evaluator/removal work remain open. No status downgrade, compatibility layer, copied fixture, fallback carrier, normative narrowing or fake Inspection/gate binder is introduced. The separate reference prototype and its false coarse-TC025/parked-inspection credit are outside this PR and not credited here; existing upstream ownership remains unchanged. RT193/IT35/IR1277 work is separate and untouched.

Preserved focused evidence: native exact_function_application25PASS, kernel package2unitPASS and5doctestPASS; touched-crate all-target/all-feature warnings-denied Clippy, formatting and changed-document validation pass. Reviewer read those logs without rerunning. Final native aggregate and hosted-native-only disposition remain owed. These are native/compile evidence only, never backend/proof credit. Scoped method verdict PASS with no findings; merge readiness CONDITIONAL on final gates, and IR499 remains OPEN.

Plan completion: not assessed

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: FR-278-AC-1
  path: spec/exact/functional/FR-278-runtime-checked-invariant-causes.md
  role: examined
  excerpt: '| FR-278-AC-1 | Every RT checked-package/function invariant producer returns
    its table cause: unknown name is `UnknownCheckedFunction`, a foreign expression
    is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`.'
- id: FR-278-AC-2
  path: spec/exact/functional/FR-278-runtime-checked-invariant-causes.md
  role: examined
  excerpt: 'FR-278-AC-2 remains planned: moving a producer out of RT does not make
    the'
- id: kernel-carrier-negative
  path: tools/kernel-api-check/src/lib.rs
  role: examined
  excerpt: //! use quire_exact::CheckedInvariantCause::CallDepthExceeded;
- id: kernel-carrier-positive
  path: tools/kernel-api-check/src/lib.rs
  role: examined
  excerpt: //! use quire_exact::CheckedInvariantCause::UnknownCheckedFunction;
- id: native-scheduler
  path: src/exact/expression.rs
  role: examined
  excerpt: let Ok(mut active_meter) = shared_meter.try_borrow_mut() else {
- id: native-cause-controls
  path: tests/exact_function_application.rs
  role: examined
  excerpt: fn tc_194_reentrant_meter_borrow_has_typed_cause() {
- id: carrier-gate
  path: Makefile
  role: examined
  excerpt: $(CARGO) +$(MSRV) test $(LOCKED) -p quire-contract-runtime-kernel-api-check
    --doc
bindings:
- test_id: tc_194_an_expression_from_a_dropped_package_is_refused_by_a_new_one
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tc_194_explicit_frames_reach_4096_calls_on_64_kib_stack_with_exact_fuel
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tc_194_reentrant_meter_borrow_has_typed_cause
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tc_194_invalid_child_arguments_refuse_before_child_charge_or_body
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tc_194_missing_child_has_typed_cause_and_does_not_resume_parent
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tc_194_direct_host_reentry_is_refused_before_nested_charge
  ac_id: FR-278-AC-1
  trace: correct
- test_id: tests::the_resolved_fault_carrier_has_the_public_function_cause
  ac_id: FR-278-AC-1
  trace: correct
```
