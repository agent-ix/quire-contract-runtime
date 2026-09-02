---
id: FR-005
title: "Adopt the shared assurance intake path"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: depends_on
---
# FR-005: Adopt the shared assurance intake path

## Description

When verification results are recorded for a candidate revision, quire-contract-runtime shall hand
its own tools' declared structured results to the released Engineering Assurance, Quire, and Quoin
contracts rather than to a repository-local evidence framework.

## Inputs

- The accepted Engineering Assurance compatibility matrix and the component versions it pins.
- Structured results produced by this repository's own tools: the feature-matrix runner, the Kani
  proof gate, the Kani semantic-mutation campaign, and the governed footprint measurement.
- The Quire static export of specification, obligation, and coverage facts.

## Outputs

- A Quoin change-assurance record sealed from `assurance/change-assurance.json`.
- One Quoin proof attestation per declared proof obligation, over bytes a producer already wrote.
- A Quoin verification receipt.

## Behavior

- `engineering_assurance.compatibility` shall classify every observed component version.
- quire-contract-runtime shall observe its own toolchain without restating the compatibility matrix.
- Quire shall export static specification, obligation, and coverage facts without executing a
  producer.
- Quoin shall transcribe declared structured results without executing a producer.
- `make assurance-inputs` shall be the only target that executes a producer.
- Each downstream gate shall report an absent producer input as an error.
- The native adapter shall transcribe the one protocol it names.
- The native adapter shall refuse a stream declaring any other protocol.
- The native adapter shall refuse an outcome its declared vocabulary does not name.
- No gate shall recover a verdict from a process's output stream while that process emits a
  structured result.
- The Kani producer shall publish `runtime.kani-proof/v1` from the Kani transcript.
- No component other than the Kani producer shall read the Kani transcript.
- While `cargo-kani` is absent, the Kani producer shall report every declared harness as unavailable.
- While `cargo-kani` is absent, the Kani gate shall exit non-zero.
- quire-contract-runtime shall keep pass, fail, unavailable, unsupported, inconclusive,
  not-computed, malformed, partial, stale, suspect, vacuous, and tampered distinguishable from one
  another.
- quire-contract-runtime shall report no non-success outcome as a success.
- quire-contract-runtime shall retain no generic runner, evidence envelope, manifest, tool-identity
  framework, retention store, audit store, anchor file, recipe-failure policer, or aggregate
  verdict.
- The published crate shall depend on neither Quire nor Quoin at runtime.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | Every pinned component is classified by the packaged compatibility matrix, no consumed artifact digest differs from its pin, and no internal mirror registry is named anywhere in the repository. | Test (TC-009) |
| FR-005-AC-2 | The feature-matrix, Kani proof, Kani mutation, and footprint results are structured, are produced by this repository's tools, and reach Quoin through the declared adapter without Quoin or Quire executing a producer. | Test (TC-010) |
| FR-005-AC-3 | Static specification, obligation, and coverage facts for a candidate revision come from the Quire export named by the sealed record's impact snapshot. | Test (TC-011) |
| FR-005-AC-5 | Each of the twelve verification outcomes is demonstrated by a case that produced it, each negative case is paired with a positive control that was observed to be accepted, and an absent Kani toolchain is reported as unavailable rather than as a pass. | Test (TC-013) |
| FR-005-AC-6 | No script, Make target, or test in the repository implements a generic evidence envelope, manifest, retention store, tool-identity lock, anchor file, or aggregate verdict, and the frozen evidence schemas are referenced by nothing. | Test (TC-014) |

## Dependencies

- **Upstream**: [NFR-002](../nonfunctional/NFR-002-panic-compatibility-license.md) and
  [NFR-001](../nonfunctional/NFR-001-no-std-footprint.md). Constrained by the accepted shared
  release pins recorded in `assurance/pins.json` and by the migration contract at
  `agent-ix/engineering-assurance#10`.
