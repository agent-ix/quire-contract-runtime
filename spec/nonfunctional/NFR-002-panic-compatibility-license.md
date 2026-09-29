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
crate shall remain `publish = false`, licensed `AGPL-3.0-or-later`, and forward-compatible by
using non-exhaustive public data enums where downstream exhaustive matching would impede evolution.

## Scope

Public v0.1 runtime APIs and optional dependency surfaces.

The optional bounded snapshot codec returns structured limit and malformed-input errors.
Its allocating dependency paths can abort on allocator exhaustion; the Result API and
no-intentional-panic policy are not universal OOM recovery guarantees. Native memory-limit
qualification reports process failure separately from an actual codec error return.

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

## Verification

CI runs Clippy, unit/property tests, and cargo-deny. The public API documentation states its size, panic, feature, and compatibility contracts.

## Dependencies

- **Upstream**: [FR-002](../functional/FR-002-safe-operators.md).
