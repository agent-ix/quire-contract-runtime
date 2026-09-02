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

Public v0.1 runtime APIs and optional dependency surfaces.

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

## Verification

CI runs Clippy, unit/property tests, manifest inspection, unsafe audit, and
cargo-deny. The public API documentation states its size, panic, feature, and compatibility contracts.

## Qualification integrity limits

A green local `make ci` is evidence that the gates ran and passed on the tree as
committed. It is not evidence about a tree whose Makefile has been edited.
Measured on this repository: with a rustfmt violation, a failing test and a
renamed Kani harness all present, an unmodified tree exits 2 at `fmt-check` and a
tree with `.IGNORE:` prepended to the Makefile exits 0 from `make ci`, with six
of the fourteen prerequisites still printing a diagnostic and none of them
failing the build.

The gate that used to police this was deleted with the collector it protected,
and the structural replacement covers part of the class rather than all of it:
the chain derives every attested result from producer bytes, so a suppressed
producer yields an absent or unreadable input and an error, but the gates that
feed nothing into the chain are simply neutered. Tracked as
agent-ix/quire-contract-runtime#10 and stated here rather than claimed away.

## Dependencies

- **Upstream**: [FR-002](../functional/FR-002-safe-operators.md).
