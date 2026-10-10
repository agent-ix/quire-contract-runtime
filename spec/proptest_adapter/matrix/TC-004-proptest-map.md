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

1. Without `proptest`, compile a separate path-dependent client that imports a known public type,
   then compile the same client package's `proptest_adapter` import and require rustc `E0432` at
   that exact import. Use a distinct stable worktree-owned Cargo target to avoid a nested build
   lock. Also run the `src/lib.rs` `compile_fail` doctest without default features.
2. Enable `proptest`, adapt each verdict through the public module, and inspect the result variant.

## Expected Results

The ordinary feature-off import compiles, while the adapter import fails at `E0432` for the
missing module; the compile-fail doctest agrees. With the feature enabled, pass becomes
success, failure becomes `Fail`, and rejection becomes `Reject`.
