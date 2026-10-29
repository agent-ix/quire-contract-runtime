---
id: TC-202
title: "Reserved: retired first-party Git lock-entry mutation gate"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-202: Reserved retired lock-entry mutation gate

## Description

**Retired; ID reserved.** This test case exercised the separate `quire check-lock`
gate and scratch duplicate-name and split-revision controls. The gate is being
removed because its lock-entry checks do not establish the runtime's source,
licence or ban policy. TC-197 retains direct inspection of the committed
lockfile under FR-275-AC-5. TC-198 retains cargo-deny source-policy evidence.

## Test Procedure

No active procedure. The former scratch-lock mutation procedure is retired
with FR-275-AC-6 and FR-275-AC-24 through FR-275-AC-28.

## Expected Results

No active result is claimed for this reserved test case. TC-197 and TC-198
retain their independent ownership and source-policy results.
