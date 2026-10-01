---
id: TC-006
title: "Account for campaign outcomes"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: verifies
---
# TC-006: Account for campaign outcomes

## Description

Verify every outcome updates the right complete counter set and saturation is safe.

## Test Procedure

Record mixed verdicts and discards into a report, then record events at `u64::MAX` boundaries.

## Expected Results

Accepted, rejected, failed, and discarded counts match the specification and remain saturated.

