---
id: SR-21802
title: IR-724 PR218 spec-review-integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-runtime; PR218; spec/monitor/matrix/tests.md; spec/monitor/matrix/TC-209-monitor-program-construction.md; spec/exact/matrix/tests.md; spec/spec.md
review_set: subset
---

## Summary

Frozen head 64fb48904fdb1a971f3f0661d6484348db8799ed; ticket IR-724, PR218. Internal consistency of the status text after the PR's edits, within the four changed files.

## Verdict

CHANGES REQUESTED. The PR moves interface-002-AC-1 and TC-209 procedure 1 to backed, but two passages it left in place still state the old evidence gap, so each edited file now contradicts itself. The exact matrix edits and the spec/spec.md row are consistent. The counts hold: the 29 listed criteria sum correctly, and seven rows moved leave one Planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | tests.md still says procedure 1 lacks independent retained-selection-byte assertions and that this keeps TC-209 Planned, directly contradicting the PR's new interface-002-AC-1 entry eight lines below | spec/monitor/matrix/tests.md:55-57 |
| FND-002 | medium | TC-209 Evidence Status still refers to missing retention assertions and says all FR-276 temporal criteria remain Planned, contradicting the PR's new text above it and the Implemented FR-276 rows | spec/monitor/matrix/TC-209-monitor-program-construction.md:41-44 |

## Finding detail

FND-001. Lines 55-57 read "TC-209's procedures 2 through 4 are backed, but procedure 1 lacks independent retained-selection-byte assertions, so the complete TC-209 case remains Planned." Line 64-67 now cites literal-oracle assertions for those same bytes, and the test does hold them (quire-integration tests/monitor_program.rs:862-873 against literals at :769-771). A reader of the M5 gate takes the wrong reason for TC-209's status. TC-209 lines 33-34 already give the true one, interface-002-AC-5/AC-6. Fix: replace the sentence with that reason.

FND-002. Line 41 "It does not fill the missing retention assertions" is now false for the bytes, and lines 42-43 "TC-200 and all FR-276 temporal criteria remain Planned independently" conflict with tests.md:13-24, where FR-276 rows are Implemented or Partial. Fix: drop the retention clause and state TC-200 as partial with a pointer to tests.md.

## Coverage

Integrity lens over the diff and the unchanged prose in the same files that restates the changed rows' status. Requirement structure, IDs and relationships are unchanged.

## Verification limits

No tests or gates were run. The analysis covers only the four changed files.

## Examined units

```yaml
scope:
- id: TC-209
  path: spec/monitor/matrix/TC-209-monitor-program-construction.md
  role: examined
  excerpt: Procedure 1 and interface-002-AC-1 are backed in quire-integration tests/monitor_program.rs by retained_program_outlives_preparation_and_keeps_four_entry_keys
- id: TM-006
  path: spec/monitor/matrix/tests.md
  role: examined
  excerpt: TC-209's procedures 2 through 4 are backed, but procedure 1 lacks independent retained-selection-byte assertions, so the complete TC-209 case remains Planned.
- id: TC-031
  path: spec/exact/matrix/tests.md
  role: examined
  excerpt: Fire one injected denial with a limit-independent record
- id: Tick-monitor-row
  path: spec/spec.md
  role: examined
  excerpt: Consumer of QSL's admitted executable `TimedMonitorPlan`; bounded counter steps and typed results, without temporal-semantic ownership
```
