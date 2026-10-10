---
id: TC-004
title: "Verify the optional proptest adapter"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-003
    type: verifies
---
# TC-004: Verify the optional proptest adapter

## Description

Verify the adapter is absent without `proptest`, present with it, and maps pass, postcondition
failure, and precondition rejection distinctly.

## Test Procedure

1. Run the `src/lib.rs` `compile_fail` doctest without default features and confirm that importing
   `quire_contract_runtime::proptest_adapter` fails because the module is absent.
2. Enable `proptest`, adapt each verdict through the public module, and inspect the result variant.

## Expected Results

The feature-off import fails for the missing module. With the feature enabled, pass becomes
success, failure becomes `Fail`, and rejection becomes `Reject`.
