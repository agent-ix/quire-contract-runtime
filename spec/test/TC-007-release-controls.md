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
ceiling for linked `.text` plus `.rodata`, retains no runtime/harness panic-path reference, and publication and dual-license
controls remain explicit. Verify the evidence toolchain binds the vendored PGM-01 schema to the
recorded revision/digest, preserves semantic envelope identities, and fails closed when collection,
outcome, transcript, or checksum-fixed-point records disagree.

## Test Procedure

Run `make ci`. Inspect the `tc_007_release_controls_are_mandatory` result, default dependency tree,
unsafe audit, `cargo deny` result, and the enforced release-size output retained by MP-001.
The source-policy test parses the footprint harness and requires actual calls to every constructor,
both accounting mutations, and every operator family. A footprint-crate unit test executes that
population at two fixed inputs and compares exact results. The source-policy test also recursively
parses every shipped runtime source file and constrains accounting inherent and trait implementations,
private aliases, cross-file functions, and macros that could add a reset seam.
`scripts/measure_footprint.py` links the fixed-population consumer on the declared MSRV compiler for
the declared target and publishes the measurement as a structured document, so the numbers a reader
sees are the numbers a gate read. `tests/shared_assurance.rs` pins the four frozen artifacts under
`schemas/` by SHA-256 and asserts that no code, configuration, or workflow file in the repository
references any of them.

## Expected Results

The source policy and footprint semantic tests pass, the default normal dependency count and
unsafe-block count are zero, the license and publication gates pass, and
`scripts/check_linked_footprint.sh` exits successfully only when the fixed-target runtime/harness
sections are at least 500 and no larger than 4,096 bytes and no runtime/harness panic-path reference
is linked.

The four frozen artifacts under `schemas/` are byte-identical to the digests retained records name
them by, and altering one byte of any of them fails the census.
