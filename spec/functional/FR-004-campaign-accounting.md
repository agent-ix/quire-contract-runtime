---
id: FR-004
title: "Account for every campaign outcome"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
---
# FR-004: Account for every campaign outcome

## Description

When a harness records a case, the runtime shall update a per-requirement campaign report containing
accepted, rejected, failed, and discarded counters.

## Inputs

- A verdict or an explicit external-framework discard event.

## Outputs

- A complete, saturating `CampaignReport` for one requirement and revision.

## Behavior

- Passed and failed cases shall increment accepted.
- Failed cases shall also increment failed.
- Rejected preconditions shall increment rejected.
- Framework discards shall increment discarded.
- Reports shall always serialize or format all four counters as one indivisible value.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | Mixed outcome sequences produce the specified four counters. | Test (TC-006) |
| FR-004-AC-2 | Counter overflow saturates and never panics or wraps. | Test (TC-006) |
| FR-004-AC-3 | No public report constructor can omit a metric. | Inspection |

## Dependencies

- **Upstream**: [FR-001](./FR-001-verdict-observation.md).
