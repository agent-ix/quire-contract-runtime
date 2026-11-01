---
id: TC-208
title: "Isolate Kani verification from the optional exact dependency"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: verifies
---
# TC-208: Isolate Kani verification from the optional exact dependency

## Description

`quire-exact` is an optional dependency enabled only by the `exact` feature. A default-feature
dependent of the runtime compiles `verification/kani.rs` under `cfg(kani)`, so every item there
that names `quire_exact` must be compiled only under `feature = "exact"` (interface-001-AC-15).

## Test Procedure

Run `kani_module_gates_every_quire_exact_reference_on_the_exact_feature`. It parses
`verification/kani.rs` and fails if a `use` or item that mentions `quire_exact`, or an item that
mentions a name imported from it, lacks a `#[cfg(feature = "exact")]` attribute. Compile-only
confirmation: `cargo kani --only-codegen` without `--features exact`, and with it.

## Expected Results

The test passes; the default-feature `--only-codegen` run reports no `E0432`.
