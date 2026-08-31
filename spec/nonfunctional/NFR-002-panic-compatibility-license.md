---
id: NFR-002
title: "Panic, compatibility, and licensing contract"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: constrains
---
# NFR-002: Panic, compatibility, and licensing contract

## Statement

For all valid Rust values, public evaluation and accounting APIs shall avoid intentional panics; the
crate shall remain `publish = false`, dual-licensed `MIT OR Apache-2.0`, and forward-compatible by
using non-exhaustive public data enums where downstream exhaustive matching would impede evolution.

## Scope

Public v0.1 runtime APIs, optional dependency surfaces, and the local evidence-construction toolchain.

## Rationale

Generated code must not introduce avoidable panics or licensing surprises, while schema evolution
must be explicit to downstream users.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Intentional panic sites in library code | 0 | 0 | static-quality |
| License policy violations | 0 | 0 | sca-sbom |
| Registry publication enabled | false | false | inspection |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Valid public evaluation and accounting inputs encounter no intentional panic path. | property-based-testing (TC-003) |
| NFR-002-AC-2 | The manifest remains unpublished and declares `MIT OR Apache-2.0`. | Inspection (TC-007) |
| NFR-002-AC-3 | Public data enums whose evolution affects downstream matching remain non-exhaustive. | Inspection (TC-008) |
| NFR-002-AC-4 | The evidence collector records every invoked gate's transcripts and numeric exit status, fails closed on command or checksum-fixed-point failure, and is tested for status propagation; the builder verifies its vendored PGM-01 schema and raw Git commit identities, derives outcomes without contradicting retained transcripts, preserves failed or inconclusive outcomes, and is tested for identity, digest, role, extension, outcome truthfulness, and validation behavior. | Inspection (TC-007) |

## Verification

CI runs Clippy, unit/property tests, evidence-tool unit tests, manifest inspection, unsafe audit, and
cargo-deny. The public API documentation states its size, panic, feature, and compatibility contracts.

## Dependencies

- **Upstream**: [FR-002](../functional/FR-002-safe-operators.md).
