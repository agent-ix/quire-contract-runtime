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

Verify the default linked boundary remains dependency-free and unsafe-free, the release rlib remains
within 256 KiB, and publication and dual-license controls remain explicit.

## Test Procedure

Run `make ci`. Inspect the `tc_007_release_controls_are_mandatory` result, default dependency tree,
unsafe audit, `cargo deny` result, and the enforced release-size output retained by MP-001.

## Expected Results

The source policy test passes, the default normal dependency count and unsafe-block count are zero,
the license and publication gates pass, and `scripts/check_rlib_size.sh` exits successfully only when
the measured rlib is no larger than 262,144 bytes.
