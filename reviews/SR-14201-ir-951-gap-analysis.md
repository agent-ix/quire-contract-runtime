---
id: SR-14201
title: "IR-951 runtime gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; NFR-001-AC-3; FR-275-AC-8; TC-007; TC-198; Makefile; src/bin"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: reviews
---

## Summary

Ticket: IR-951. Exact frozen head `d0af2fe1c75d394aefe73095b357d25d96b9ebaf`. Planless diff-scope trace check of the two gate ports against NFR-001-AC-3 and FR-275-AC-8, computed Test Matrix, and the changed tests. Both criteria have tagged tests; the real Make gates remain integration evidence.

## Verdict

PASS for the changed scope. Existing unrelated planned and untagged criteria are outside this diff. Plan completion: not assessed.

## Examined scope

- NFR-001-AC-3 (examined; spec/core/non-functional/NFR-001-no-std-footprint.md): On Rust 1.98.1 for `thumbv7em-none-eabi`, the fixed-population static-library consumer in `measurement/footprint/` has linked `.text` plus `.rodata` between 500 bytes and 4 KiB and its runtime/harness objects retain no panic-path reference.
- FR-275-AC-8 (examined; spec/exact/functional/FR-275-single-exact-kernel.md): `make deny` reports cargo-deny's `source-not-allowed` error for the QSL Git repository, and exits non-zero, when a normal, build or dev dependency resolves from it; a licence failure alone does not satisfy this.
- TC-007 (examined; spec/core/matrix/TC-007-release-controls.md): The footprint semantic test passes and the Rust `check_linked_footprint` binary exits successfully only when the fixed-target runtime/harness sections are at least 500 and no larger than 4,096 bytes and no runtime/harness panic-path reference is linked.
- TC-198 (examined; spec/exact/matrix/TC-198-qsl-crate-edge-guard.md): `make deny-mutations` (Rust `check_deny_source`) runs steps 2 to 4 in scratch copies and requires the QSL source diagnostic for each case.
- AD-004 (examined; spec/assurance/AD-004-runtime-crate-layout.md): The Rust `check_deny_source` binary (`make deny-mutations`) guards the QSL Git source; the Rust gate proves the source is rejected.
- spec/exact/matrix/tests.md (examined; spec/exact/matrix/tests.md): TC-198 now names the Rust check_deny_source binary and its traced diagnostic discriminator test; the real mutation gate supplies dependency evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. Computed matrix reports NFR-001-AC-3 and FR-275-AC-8 tagged; the parser unit tests and real Make gates give complementary evidence. Optional semantic audit was not selected.
