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

Check the typed outcome envelope, the closed reason and vocabulary enums, and the source policy of
the `exact` surface. Evidence: `tests/exact_outcomes.rs` (`--features exact`).

## Test Procedure

1. Build one outcome of each disposition, including completed `true` and `false`; compare every
   pair for equality and check that only completed outcomes yield a value.
2. Enumerate every `Refusal` and `Undefined` variant and check that codes are distinct.
3. Round-trip every `ChargePoint` and `LimitKind` through its spelling; check the eleven 5d88578
   families are present and `equality.plan-form` is absent.
4. Scan every `src/exact/` file for host floats, `std`, `unsafe`, `unwrap`, `expect`, `panic!`
   and indexing; check the crate root attributes.
5. Parse the module tree and check that the public item set equals the `pub use` set.

## Expected Results

Four distinct dispositions, closed vocabularies matching QSpec 5d88578, no forbidden source token
and no public item outside the private-module re-exports.
