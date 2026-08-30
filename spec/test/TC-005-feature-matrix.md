---
id: TC-005
title: "Resolve and build the feature matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: verifies
---
# TC-005: Resolve and build the feature matrix

## Description

Verify the core builds alone, every declared optional feature combination resolves explicitly, and
the proptest API is unavailable without its opt-in feature. The reserved `alloc` and `std` rows are
resolver/build compatibility checks; they do not claim distinct runtime behavior.

## Test Procedure

Run tests with no default features, with `alloc`, with `std`, and with all features. Run the
feature-policy source test and the default-profile compile-fail doctest for `proptest_adapter`.

## Expected Results

Every supported combination compiles and passes; the default-profile compile-fail case proves
proptest symbols are absent from the default API.
