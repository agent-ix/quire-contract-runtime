---
id: TC-007
title: "Audit runtime footprint"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: verifies
---
# TC-007: Audit runtime footprint

## Description

Verify the Rust 1.75 `thumbv7em-none-eabi` footprint consumer remains between the 500-byte
population floor and 4 KiB ceiling for linked `.text` plus `.rodata` and retains no runtime/harness
panic-path reference.

## Test Procedure

Run `make size` and the footprint crate's tests. A footprint-crate unit test executes the fixed population at two fixed inputs and compares
exact results. `make size` links the fixed-population consumer on the MSRV compiler for the
declared target and measures it.

## Expected Results

The footprint semantic test passes and `scripts/check_linked_footprint.sh` exits successfully only when the fixed-target
runtime/harness sections are at least 500 and no larger than 4,096 bytes and no runtime/harness
panic-path reference is linked.
