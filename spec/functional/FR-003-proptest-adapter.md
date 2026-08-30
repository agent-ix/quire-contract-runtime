---
id: FR-003
title: "Adapt verdicts to proptest"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
---
# FR-003: Adapt verdicts to proptest

## Description

Where the optional `proptest` feature is enabled, the runtime shall map passed verdicts to success,
failed postconditions to test failure, and rejected preconditions to test rejection.

## Inputs

- A runtime verdict.

## Outputs

- A `proptest::test_runner::TestCaseResult`.

## Behavior

- The adapter shall preserve the tri-state meaning.
- The adapter shall map rejection only to a proptest rejection error.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | Each verdict maps to the corresponding proptest outcome. | Test (TC-004) |
| FR-003-AC-2 | The adapter is absent unless the opt-in feature is selected. | Test (TC-005) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-verdict-observation.md).
