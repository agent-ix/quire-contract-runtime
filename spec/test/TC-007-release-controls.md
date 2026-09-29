---
id: TC-007
title: "Audit runtime footprint and release controls"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: verifies
---
# TC-007: Audit runtime footprint and release controls

## Description

Verify the default linked boundary remains dependency-free and unsafe-free, the Rust 1.75
`thumbv7em-none-eabi` footprint consumer remains between the 500-byte population floor and 4 KiB
ceiling for linked `.text` plus `.rodata`, retains no runtime/harness panic-path reference, and publication and `AGPL-3.0-or-later`
license controls remain explicit.

## Test Procedure

Run `make ci`. Inspect the default dependency tree, the `cargo deny` result, and the `make size`
output. A footprint-crate unit test executes the fixed population at two fixed inputs and compares
exact results. `make size` links the fixed-population consumer on the MSRV compiler for the
declared target and measures it.

## Expected Results

The footprint semantic test passes, the default normal dependency count is zero, the license gate
passes, and `scripts/check_linked_footprint.sh` exits successfully only when the fixed-target
runtime/harness sections are at least 500 and no larger than 4,096 bytes and no runtime/harness
panic-path reference is linked.
