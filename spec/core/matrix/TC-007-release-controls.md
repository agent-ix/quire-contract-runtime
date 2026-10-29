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

Verify the root `Cargo.toml` `rust-version` `thumbv7em-none-eabi` footprint consumer remains between the 500-byte
population floor and 4 KiB ceiling for linked `.text` plus `.rodata` and retains no runtime/harness
panic-path reference (FR-275-AC-20, AC-21).

The band's 500-byte floor proves the fixed population is linked. The footprint crate keeps its
verdicts and report opaque through `core::hint::black_box` so the measured population remains
observable to the compiler.

## Test Procedure

Run `make size` and the footprint crate's tests. A footprint-crate unit test executes the fixed population at two fixed inputs and compares
exact results. `make size` links the fixed-population consumer on the MSRV compiler for the
declared target and measures it.

## Expected Results

The footprint semantic test passes and the Rust `check_linked_footprint` binary exits successfully only when the fixed-target
runtime/harness sections are at least 500 and no larger than 4,096 bytes and no runtime/harness
panic-path reference is linked.
