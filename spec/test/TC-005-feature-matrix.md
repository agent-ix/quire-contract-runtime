---
id: TC-005
title: "Resolve and build the feature matrix"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/interface-001
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

Also execute isolated `--no-default-features --features snapshot-json` tests and doctests,
then compile that exact library profile for `thumbv7em-none-eabi` on Rust 1.75.0. Check
default dependency resolution separately. All-features includes std and does not prove
the codec's no_std independence; neither alloc nor std alone exposes the codec API.

Also compile `--no-default-features --features exact` for `thumbv7em-none-eabi` at the Rust
version `compatibility.msrv` declares, verifying interface-001-AC-13's no_std/MSRV build of the
`exact` module.

## Expected Results

Every supported combination compiles and passes; the default-profile compile-fail case proves
proptest symbols are absent from the default API.
