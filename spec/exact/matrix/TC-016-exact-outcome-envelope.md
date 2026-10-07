---
id: TC-016
title: "Inspect the exact outcome envelope and vocabulary"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: verifies
---
# TC-016: Inspect the exact outcome envelope and vocabulary

## Description

Check the typed outcome envelope and the closed reason and vocabulary enums of the `exact`
surface. Evidence: `tests/exact_outcomes.rs` (`--features exact`).

## Ownership and evidence

Kernel-owned after IR-349 removes the local copy. The evidence path above is current
RT evidence, not a retained RT test obligation. The outcome envelope, reason variants and
spelling census remain required; FR-359–363 do not establish them. An exact owner criterion
mapping for those checks remains to be identified; no upstream pass is claimed.

## Test Procedure

1. Build one outcome of each disposition, including completed `true` and `false`; compare every
   pair for equality and check that only completed outcomes yield a value.
2. Enumerate every `Refusal` and `Undefined` variant and check that codes are distinct.
3. Round-trip every `ChargePoint` and `LimitKind` through its spelling; check the eleven QSpec
   families are present and `equality.plan-form` is absent.

## Expected Results

Four distinct dispositions and closed vocabularies matching QSpec.
