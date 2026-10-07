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
`quire-exact` and `quire-semantic-value`, and that the default profile and its footprint measurement
do not resolve either shared crate or transitive `quire-canonical` crates.

## Test Procedure

0. Read the package's declared `rust-version` in `Cargo.toml` as the MSRV authority
   (FR-275-AC-20); do not duplicate the tool version in this procedure.
1. Run the `make test-features` row `build-exact-no-std-msrv` (`exact`, no `std`, target
   `thumbv7em-none-eabi`) at that MSRV; expect success (FR-275-AC-9). Run `make msrv` and
   expect it to use the same manifest-declared MSRV (FR-275-AC-21).
2. Run `make size`: after the build it lists the footprint graph with `cargo tree -p
   quire-contract-runtime-footprint --target thumbv7em-none-eabi` and fails when `quire-exact`,
   `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive` is in it; expect it to
   report that the graph holds no shared exact dependency (FR-275-AC-10). Enabling `exact` on the
   footprint dependency in a scratch copy makes that listing contain those crates.
3. Run `make size`; expect `.text` plus `.rodata` inside NFR-001-AC-3's 500 byte to 4 KiB band and no
   panic relocation (FR-275-AC-11).

## Expected Results

The exact profile builds on the bare-metal target; the footprint graph holds no kernel and its band
is unchanged.
