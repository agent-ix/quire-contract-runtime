---
id: SR-2026110705000811
title: "spec-review-ears of RT fallible declared-type adoption"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime; IR-1400; Cargo.lock, Cargo.toml, spec/exact/functional/FR-273-exact-function-application.md, spec/exact/matrix/TC-194-function-application.md, spec/monitor/functional/FR-276-consume-timed-monitor-plan.md, spec/monitor/matrix/TC-200-tick-monitor-consumer.md, src/exact/expression.rs, src/monitor/program.rs, src/monitor/validation.rs, tests/exact_type_check_allocation.rs"
review_set: subset
---

## Summary

Ticket: IR-1400. The added contract names actor, trigger and typed response. One wrapped retry statement produces new missing-subject/unclassifiable engine warnings; the old line-91 non-singular warning is baseline.

## Verdict

**PASS (scoped PR, disposition round 1)** — Every original finding is fixed at the inspected head. The native type-check adoption and actual denial controls are ready for the lead's final combined-tree gate; no whole-runtime acceptance claim.

## Scope

- `FR-273-AC-14` (`spec/exact/functional/FR-273-exact-function-application.md`), examined: | FR-273-AC-14 | Package function parameter/result checks and standalone-expression parameter/result checks preserve admitted types and the original semantic refusal cause, location and order through `try_check_type`. A dedicated allocator test denies actual owner worklist allocation after fixture construction at each of these four consumer positions; the result is the typed native reservation cause, retains `Allocation` with the owner's request unit and amount, publishes no checked result, and returns without aborting or allocating to project the error. An identical retry with allocation enab
- `FR-276-AC-13` (`spec/monitor/functional/FR-276-consume-timed-monitor-plan.md`), examined: | FR-276-AC-13 | `Program::try_new` takes only `ProgramParts`. In a dedicated test binary whose failing global allocator refuses the first allocation inside `try_new` (the first Layout type check's worklist), construction returns `ProgramRefusal` code `Membership` with the native reservation cause and no Program; refusing the first RT-owned scratch reservation returns code `Storage`; refusing an allocation inside a literal's membership walk returns `Membership` with the native `EnvironmentFailure`. An expression arena nested 20,000 levels deep constructs on a 512 KiB native stack. For build an
- `FR-276-AC-14` (`spec/monitor/functional/FR-276-consume-timed-monitor-plan.md`), examined: | FR-276-AC-14 | Actual allocator denials inside Layout, function parameter/result and expression type checks return `Membership` with the original native allocation class and `StorageRequest`, without allocating to construct the refusal or returning a Program; allocation-enabled retries admit the same valid parts. Semantic unknown-type and operator-ineligible controls retain the existing `TypeMismatch` behavior and validation precedence. Explicit native-error projection controls preserve `Allocation` versus `Capacity` and the original request unit and amount, with no synthetic configured limi
- `TC-194` (`spec/exact/matrix/TC-194-function-application.md`), examined: 17. For FR-273-AC-14, validate ordinary and nested declared types through both package
- `TC-200` (`spec/monitor/matrix/TC-200-tick-monitor-consumer.md`), examined: 12. For FR-276-AC-14, deny actual allocations inside the owner type-check walk at each

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new retry statement starts its wrapped line with SHALL after placing RT on the preceding line. Quire reports missing-subject and unclassifiable for this statement; keep the actor and SHALL together in the requirement statement. | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:176 |

## Verification

Inspected actual frozen source, owner TypeCheckError/StorageRequest/EnvironmentFailure and thread-local failing allocator implementation. The fixture denies one real allocation and then automatically disarms; it is not persistent allocator exhaustion. Read actual successful focused logs: `/tmp/ir1400-rt-check.log`, `/tmp/ir1400-rt-clippy.log`, and `/tmp/ir1400-rt-focused.log` (native projection 1, allocator/semantic controls 8, existing function-application 25). No reviewer Cargo gate was run; the lead owns the one final combined-tree gate. No CI workflow was changed. No applicable AssuranceProfile was found under this repository's spec. Capacity controls are explicit owner-error projection tests, not real overflow evidence. The resolved quire-exact NodeKey shallow-clones its Arc-backed NodeData and native request moves allocate no storage; exact diagnostic locus/vector capacity are prepared before the owner call. This does not claim recovery from arbitrary RT allocator exhaustion.

## Engine results

Changed-file validation passed with a baseline line-91 non-singular warning and new line-176 missing-subject/unclassifiable warnings. Only the new retry statement is a finding.

## Disposition verification

Inspected fix head `949cdaf0cc54c8b26cda671a232349322b3e5bd6` and the actual resolved Arc-backed NodeKey implementation. Read passing focused round-1 evidence: ten allocator/semantic tests, one private projection test, touched Clippy, formatting and diff checks. Fresh actual matrix declares 226 criteria (62 tagged, 122 baseline untagged, 42 method-without-symbol); new FR-273-AC-14 and FR-276-AC-14 are declared and tagged with six and five binders respectively. Existing FR-276-AC-13 remains tagged by three local controls. The old line-91 EARS warning remains baseline; the new warnings are gone. No reviewer aggregate Cargo gate ran.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The retry statement keeps RT and SHALL together; fresh changed-file validation reports only the pre-existing line-91 non-singular warning. |
