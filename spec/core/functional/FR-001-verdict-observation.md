---
id: FR-001
title: "Represent verdicts and clause observations"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: implements
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
| FR-001-AC-4 | The five clause outcomes — passed, failed, rejected, not-evaluated, undefined — are distinct through construction and pattern matching, and a failed or rejected clause retains its typed code, clause identity and optional borrowed detail. | Test (TC-001) |

## Planned revision identity replacement

This replacement is **Planned**, with no executable claim. FR-001-AC-2 and the current verdict
carrier remain active. A caller-supplied `RevisionId` distinguishes verdicts for different source
revisions of one requirement, but it does not authenticate source content. Before changing its
representation or removing it, the QSpec and QSL/code-generation contract must define an admitted
source-revision identity and how generated verdicts carry it. The replacement must still let a
consumer distinguish two revisions of one requirement and retain the identity in every verdict.

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-auditable-runtime.md).
