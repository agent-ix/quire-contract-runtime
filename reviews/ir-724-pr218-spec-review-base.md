---
id: SR-21801
title: IR-724 PR218 spec-review-base review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime; PR218; spec/exact/matrix/tests.md; spec/monitor/matrix/TC-209-monitor-program-construction.md; spec/monitor/matrix/tests.md; spec/spec.md
review_set: subset
---

## Summary

Frozen head 64fb48904fdb1a971f3f0661d6484348db8799ed; ticket IR-724, RT rows owned by IR-499, PR218. No applicable AssuranceProfile was used; the review set is base plus integrity. The check is truthfulness: each changed status row was read against the cited test on quire-integration and quire-exact origin/main. Per the owner ruling, a row backed by real tests in another repo counts as backed when it names the repo; strict coverage staying unchanged is expected and is not a finding.

## Verdict

CHANGES REQUESTED, on one medium status gap and one medium unchanged row. Rows that hold as written: interface-002-AC-1 (one low wording gap), interface-004-AC-1, interface-006-AC-3 (one low citation gap), interface-010-AC-3 (one low citation gap), interface-011-AC-3, interface-012-AC-1, FR-010-AC-6, FR-009-AC-5 (one low command gap) and the spec/spec.md Tick monitor row. interface-007-AC-3 is the weak credit: the two named tests do not exercise its continuation or no-copy clauses. FR-276-AC-11 should stay Implemented but its evidence text should be corrected in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface-007-AC-3 is credited to two tests that exercise scratch, static source location and scan cost, but neither exercises explicit continuations or the absence of subtree copies | spec/monitor/matrix/tests.md:74-76 |
| FND-002 | medium | FR-276-AC-11 is Implemented citing only the producer test, which never asserts the no-extra-field clause, and the file's own text at lines 109-111 says the count sequence does not grant AC-11 | spec/monitor/matrix/tests.md:23 |
| FND-003 | low | interface-006-AC-3 cites a final strong count of one as its no-copy evidence, which a rebuilt subtree would also satisfy; the distinguishing assertion is in an uncited test | spec/monitor/matrix/tests.md:71-73 |
| FND-004 | low | interface-010-AC-3 row names only the native-charge tests and omits the tests carrying its explicit scratch limit clause; no Work or Stack limit stop appears in the tagged tests read | spec/monitor/matrix/tests.md:77-80 |
| FND-005 | low | FR-009-AC-5 says cargo test --doc runs the doctest, but the scalar module is gated on the exact feature and default features are empty | spec/exact/matrix/tests.md:63 |
| FND-006 | low | interface-002-AC-1 evidence claims shared nominal declarations, but the test only checks that one declaration lookup is present | spec/monitor/matrix/TC-209-monitor-program-construction.md:32-33 |

## Finding detail

FND-001. record_member_addresses_consume_scratch_before_returning_state (quire-integration tests/monitor_program.rs:2777) proves the scratch charge and compares key and body path on the Program arena. record_dynamic_declaration_uses_its_own_order_and_charges_every_inspected_field (:2946) proves exact scan work. No record test has depth, so a record-only recursive path would pass both. The no-copy evidence that exists is record_members_retain_declared_identity_and_original_value_owners (:2700, strong count 2 at :2718), tagged AC-1 and uncited. The shared machine is real: src/monitor/vm.rs:854 pushes Frame::TupleElement for both TupleElement and RecordField, so the deep tuple test at :2564 reaches the same frame. Fix: cite :2700 for no-copy and state that continuation evidence is the shared frame exercised by the deep tuple test, or keep the criterion Planned until a deep record projection test exists. Source evidence is read only from the static arena; no test checks a stop's site for a projection.

FND-002. step_9_event_position_counts_ingested_events_through_the_producer_plan (tests/monitor_tick_producer.rs:1568) proves the count sequence on a real producer plan. It reads the field by name, so an extra State field passes it. The no-extra-field assertion is event_position_counts_ingested_events_and_not_watermarks_polls_or_gap_faults (tests/monitor_program.rs:7009, slots length 4 at :7041) over a controlled arena, and Program::try_new refuses a nonconforming State (fixture state-extra-field). Lines 109-111 of spec/monitor/matrix/tests.md still say the manual arena and the count sequence alone do not grant AC-11, against the Implemented row. Recommendation: correct in this PR, keeping Implemented: cite both tests and the construction refusal, and rewrite lines 109-111 so the SC-15 arena caveat no longer reads as withholding AC-11.

FND-003. tuple_deep_projections_use_explicit_continuations_without_subtree_copies asserts strong_count(owner) == 1 at the end (:2654); a deep rebuild leaves the same count. tuple_projected_state_outlives_the_temporary_owner_in_declared_position_order (:2452, count 2 at :2476) distinguishes. Fix: cite it.

FND-004. Scratch is carried by sequence_fold_temporary_addresses_are_bounded_by_explicit_scratch (:5467) and dispatch_stopped_fold_restores_slots_before_same_machine_retry (:6254), neither named. Native WorkUnits stops are present, but in the tagged tests read no StepResource::Work or Stack stop appears for a sequence operation. Fix: name the scratch tests and name or narrow the work and stack clauses.

FND-005. src/lib.rs:71-72 gates scalar on feature exact; Cargo.toml default is empty; make test passes --all-features. A plain cargo test --doc does not collect the doctest. Fix: name the feature or make test.

FND-006. The test's only declaration check is second.types().composite(key("state")).is_some() (:943). A Program that retained only that one declaration passes. Fix: say one declaration lookup, or add an assertion.

## Coverage

Base checklist over the diff: no ID, trace-tag or Verification-cell change; no hash, digest, pin or recorded revision added. Requirement statements are unchanged, so EARS, dependency, object, failure-domain, risk and scope analyses do not apply. Integrity ran separately as SR-21802. The per-row result is in the verdict.

## Verification limits

Tests were read on origin/main of quire-integration and quire-exact and not executed. No cargo, make ci, Kani or mutation gate was run, per the brief. The compile_fail doctest failure reason was inferred from exported symbols, not compiled. No test was edited.

## Examined units

```yaml
scope:
- id: interface-002-AC-1
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: After construction consumes the preparation parts and one Rc owner drops, another owner retains the complete root key with nested operand keys, selection bytes, all four entry associations, literal bodies and shared nominal declarations. Invalid arena lookups return absent. No monitor execution credit is granted by this construction control.
- id: interface-004-AC-1
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: examined
  excerpt: Missing, duplicate, sparse and foreign local declarations refuse construction, while compact function-qualified bindings retain their original source locations.
- id: interface-006-AC-3
  path: spec/monitor/functional/interface-006-monitor-tuple-projection.md
  role: examined
  excerpt: Retained projection addresses are charged to scratch; deep projections use the explicit machine, and source keys/body paths remain available without copying a subtree.
- id: interface-007-AC-3
  path: spec/monitor/functional/interface-007-monitor-record-projection.md
  role: examined
  excerpt: Projection addresses consume scratch and explicit continuations preserve source evidence without subtree copies or repeated declaration scans.
- id: interface-010-AC-3
  path: spec/monitor/functional/interface-010-monitor-sequence-operations.md
  role: examined
  excerpt: Native element/visit/formation/retain charges and explicit work/stack/scratch limits stop before later selected work, retain exact typed evidence, expose no partial value and leave later invocation state clean.
- id: interface-011-AC-3
  path: spec/monitor/functional/interface-011-monitor-unions-and-equality.md
  role: examined
  excerpt: Earlier stopped payloads prevent later execution; native retention/equality shortages and scratch denial retain exact typed evidence without a partial result or state commit.
- id: interface-012-AC-1
  path: spec/monitor/functional/interface-012-monitor-entry-dispatch.md
  role: examined
  excerpt: Changed package/clause/subject/occurrence/clock/counter/unit/width/profile/order or invalid required values refuse before entry execution, with state and reading/progress baselines unchanged.
- id: FR-010-AC-6
  path: spec/exact/functional/FR-010-injected-charge-denial-seam.md
  role: examined
  excerpt: A 0-based `occurrence` cannot be constructed, so an injected denial can never silently match no charge and degrade into "no fault injected".
- id: FR-009-AC-5
  path: spec/exact/functional/FR-009-i13-backend-negotiation.md
  role: examined
  excerpt: No `IeeeDisposition` is convertible into an `Outcome` variant, as the `compile_fail` doctest on `IeeeDisposition` (`src/scalar/ieee.rs`) shows.
- id: Tick-monitor-row
  path: spec/spec.md
  role: examined
  excerpt: Consumer of QSL's admitted executable `TimedMonitorPlan`; bounded counter steps and typed results, without temporal-semantic ownership
- id: FR-276-AC-11
  path: spec/monitor/functional/FR-276-consume-timed-monitor-plan.md
  role: examined
  excerpt: Build observes event_position 0 from init. With width 16 and maximum gap 100, the baseline event reading 65530 calls on_event(0) and leaves event_position 1; a watermark and poll leave it 1; reading 65500 faults through on_gap and leaves it 1; admitted reading 65510 calls on_event(10) and leaves it 2. The unwrapped exact Integer count is independent of tick time, and State keeps it after activations with no extra field.
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | The fix round states no test exercises an explicit StepResource::Work or Stack stop over a sequence operation, but exhausting_work_stack_or_scratch_stops_map_and_filter_as_it_stops_a_fold does so for fold, map and filter | spec/monitor/matrix/tests.md:108-109 |

FND-007. quire-integration tests/monitor_program.rs:7918 (tagged interface-010-AC-5) runs fold, map and filter under work 40, stack_frames 4 and scratch_elements 6, asserts MonitorFailureCode::Resource with StepResource::Work, Stack and ScratchElements (:7949-7953), and completes the same programs under a full budget (:7956-7959). The stated reason for Planned is therefore false. Original FND-004 led to this by limiting its search to tests tagged interface-010-AC-3. Scenario: the M5 gate records a missing test and schedules work that already exists. Planned can still be truthful on a different reason: that test asserts only the failure code and resource kind, not exact limit and spent evidence, and retries with freshly built programs. Fix: cite the test and either back the Work and Stack clauses or state that remaining gap.

## Dispositions

Round 1, head 4727c06d76572c4f8cc1c71f33970f349a414ab9.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | interface-007-AC-3 is Planned and names the absent deep record projection test; no RecordField occurs in any depth loop in monitor_program.rs |
| FND-002 | fixed | Evidence text cites the producer test, the four-slot assertion and the State refusal; closed_state_refuses_missing_optional_mistyped_extra_and_misordered_fields (:1232) refuses state-extra-field; the row at line 23 is unchanged and the citation is in the Partial section |
| FND-003 | fixed | Row cites tuple_projected_state_outlives_the_temporary_owner_in_declared_position_order |
| FND-004 | fixed | Row names both scratch tests and addresses the work and stack clauses; the regression in that wording is FND-007 |
| FND-005 | fixed | Row states cargo test --doc --features exact |
| FND-006 | fixed | TC-209 now says one nominal declaration lookup |
| FND-007 | still-open | Found this round; the false negative claim at tests.md:108-109 needs correcting |
