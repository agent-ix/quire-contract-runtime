---
id: TC-199
title: "Build the exact profile no_std and keep the default footprint"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-199: Build the exact profile no_std and keep the default footprint

## Description

Check that the runtime with `exact` and without `std` builds for `thumbv7em-none-eabi` against
`quire-exact`, and that the default profile and its footprint measurement do not resolve it.

## Test Procedure

1. Run the `make test-features` row `build-exact-no-std-msrv` (`exact`, no `std`, target
   `thumbv7em-none-eabi`); expect success (FR-275-AC-9).
2. Run `cargo tree -p quire-contract-runtime-footprint --target thumbv7em-none-eabi -i quire-exact`;
   expect cargo to report that the package is not in the graph (FR-275-AC-10).
3. Run `make size`; expect `.text` plus `.rodata` inside NFR-001-AC-3's 500 byte to 4 KiB band and no
   panic relocation (FR-275-AC-11).

## Expected Results

The exact profile builds on the bare-metal target; the footprint graph holds no kernel and its band
is unchanged.
