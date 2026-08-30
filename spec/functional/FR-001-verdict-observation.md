---
id: FR-001
title: "Represent verdicts and clause observations"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/StR-001
    type: satisfies
---
# FR-001: Represent verdicts and clause observations

## Description

When an oracle completes, the runtime shall return exactly one of `Passed`,
`FailedPostcondition`, or `RejectedPrecondition` with requirement identity, revision, execution
point, and per-clause observations.

## Inputs

- Borrowed requirement, revision, execution-point, and clause identifiers.
- Clause outcomes and optional structured failure details.

## Outputs

- A typed tri-state verdict and allocation-free observation records.

## Behavior

- The runtime shall expose the three verdict variants without a Boolean conversion.
- A failure or rejection shall retain a typed code, clause identity, and optional borrowed detail.
- Observations shall distinguish passed, failed, rejected, not-evaluated, and undefined clauses.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | The three verdicts remain distinct through construction and pattern matching. | Test (TC-001) |
| FR-001-AC-2 | Every verdict carries requirement, revision, execution-point, and observation identity. | Test (TC-001) |
| FR-001-AC-3 | Rejection has no API that converts it to successful evidence. | Inspection |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-auditable-runtime.md).

