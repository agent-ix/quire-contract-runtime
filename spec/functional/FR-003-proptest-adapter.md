---
id: FR-003
title: "Adapt verdicts to proptest"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: implements
---
# FR-003: Adapt verdicts to proptest

## Description

Where the optional `proptest` feature is enabled, the runtime shall map passed verdicts to success,
failed postconditions to test failure, and rejected preconditions to test rejection.

## Inputs

- A runtime verdict.
- For `adapt_recording`, additionally the `CampaignReport` the verdict is to be recorded in.

## Outputs

- A `proptest::test_runner::TestCaseResult`.

## Behavior

- The adapter shall preserve the tri-state meaning.
- The adapter shall map rejection only to a proptest rejection error.
- The stateless `adapt` shall record no campaign counts.
- The recording `adapt_recording` shall record the verdict in its matching report before adapting
  it, so that a recorded case is counted exactly once.
- Where a verdict's identity does not match the report's, `adapt_recording` shall return a proptest
  failure retaining both the expected and the actual identity. An identity mismatch is a fourth
  state that proptest's tri-state cannot carry; it is a harness wiring error, and the adapter shall
  map it to neither success nor rejection, because a mismatched verdict was neither accepted nor
  rejected by this campaign and a silent rejection would erase the case.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | Each verdict maps to the corresponding proptest outcome. | Test (TC-004) |
| FR-003-AC-2 | The adapter is absent unless the opt-in feature is selected. | Test (TC-005) |
| FR-003-AC-3 | `adapt` records nothing; `adapt_recording` records a matching verdict exactly once and maps it as `adapt` does; a mismatched identity yields a proptest failure naming both identities and leaves the report's counters unchanged. | Test (TC-004) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-verdict-observation.md).
