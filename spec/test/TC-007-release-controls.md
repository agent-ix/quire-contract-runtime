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
Python unit tests assemble a fixture evidence bundle, verify its digests, roles, extension block,
and schema pin, exercise accepted/rejected local validation, reject a mismatched PGM schema pin,
reject zero-status records contradicted by retained failure transcripts, and pin the validator
transcript exclusion names. The collector self-test executes the production status recorder and
status-word/fixed-point helpers, requiring nonzero commands to mark collection failure and changed
envelopes to fail their checksum comparison.

## Expected Results

The source policy and footprint semantic tests pass, the default normal dependency count and
unsafe-block count are zero, the license and publication gates pass, and
`scripts/check_linked_footprint.sh` exits successfully only when the fixed-target runtime/harness
sections are at least 500 and no larger than 4,096 bytes and no runtime/harness panic-path reference is linked. The rlib byte
count is retained separately as an unenforced compiler-sensitive observation.
The evidence builder fails before emitting an envelope if the vendored PGM schema differs from the
pinned digest, and the planning copies of the revision and digest agree with the executable pin. The
collector self-test fails if command-status propagation, status-word derivation, or envelope
fixed-point detection is weakened.
