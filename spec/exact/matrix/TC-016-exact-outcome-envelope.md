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
RT evidence, not a retained RT kernel test obligation. Kernel refusal code and cause spellings
are tested by `ix://agent-ix/quire-exact/FR-096-AC-8`; public scalar outcome stops by
`FR-362-AC-11`; charge and limit spellings by `FR-368-AC-1` and `FR-368-AC-2`.
FR-006-AC-6 now requires RT refusal-record handling of the returned code and cause, including
the absent-code internal fault, which those one-crate tests do not execute. That RT evidence is
planned after the kernel copy leaves.

## Test Procedure

1. Build one outcome of each disposition, including completed `true` and `false`; compare every
   pair for equality and check that only completed outcomes yield a value.
2. For each kernel refusal covered by FR-096-AC-8, construct the RT refusal record and check
   its code, cause, category and locus; check `CheckedInvariant` takes the internal-fault path.
3. Round-trip every `ChargePoint` and `LimitKind` through its spelling; check the eleven QSpec
   families are present and `equality.plan-form` is absent.

## Expected Results

Four distinct dispositions and closed vocabularies matching QSpec. RT records carry the
kernel's code and cause with the caller's category and locus, while an internal fault receives
no invented language refusal.
