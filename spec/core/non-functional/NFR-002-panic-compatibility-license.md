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
The five runtime-owned scalar enums are closed under AD-004 Q-4 and exhaustive; core and
remaining expression-residue data enums retain their non-exhaustive compatibility contract.

## Scope

Public v0.1 runtime APIs and optional dependency surfaces.

The optional bounded snapshot codec returns structured limit and malformed-input errors.
Its allocating dependency paths can abort on allocator exhaustion; the Result API and
no-intentional-panic policy are not universal OOM recovery guarantees. Native memory-limit
qualification reports process failure separately from an actual codec error return.

The default linked-footprint panic check in NFR-001 covers its fixed population only.
The `exact` feature has a separate linked probe under TC-206. Its relocation result
speaks for the retained runtime-owned object files in that probe; it is not a claim
that every possible generic instantiation, unretained path or third-party object has
no panic symbol. QSL owns the public `quire-exact` integer API and its divisor
admission. The runtime must consume a kernel revision that enforces that admission
before this exact-profile qualification passes.

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
| NFR-002-AC-3 | Public core and remaining expression-residue data enums whose evolution affects downstream matching remain non-exhaustive, as the `compile_fail` doctests in `src/observation.rs` and `src/verdict.rs` show. The five runtime-owned scalar enums are closed and exhaustive under AD-004 Q-4. | compile-time-check |
| NFR-002-AC-5 | The crate manifest declares `license = "AGPL-3.0-or-later"`. | Inspection |
| NFR-002-AC-6 | The crate manifest declares `publish = false`, disabling registry publication for this package. | Inspection |
| NFR-002-AC-7 | `make deny` passes its license-policy check for the resolved dependency set. | sca-sbom (`make deny`) |
| NFR-002-AC-8 | On the declared MSRV and `thumbv7em-none-eabi`, the `exact`-enabled, `std`-disabled TC-206 probe retains calls into the runtime-owned `scalar` negotiators and Boolean connective and the runtime's remaining exact expression surface. The gate reports its artifact and inspected runtime-owned object population and fails if that population is empty, a required call is absent, or any retained runtime-owned object has a relocation to a recognized Rust panic entry point. This is a separate gate from the default NFR-001 footprint and imposes no 4 KiB ceiling on exact. | Test (TC-206) |
| NFR-002-AC-9 | A scratch probe with a reachable runtime-owned panic call produces a recognized panic relocation and makes the exact-profile gate fail; the same gate passes its unmodified probe when no recognized panic relocation is present. Missing build or inspection tools, missing artifacts, and an unparseable relocation report fail closed. | Test (TC-206) |
| NFR-002-AC-10 | The resolved `quire-exact` dependency exposes `NonZeroInteger::try_from(&Integer)` with typed zero-divisor refusal, and its public `exact_div`, `div_rem_truncating` and `div_mod_floor` require an admitted `NonZeroInteger`, not a plain `&Integer`. Direct zero-divisor admission returns `Err`; all three plain-borrow calls fail to compile. An older locked kernel without this contract does not qualify. | compile-time-check (TC-206) |

## Verification

CI runs Clippy, unit/property tests, and cargo-deny. Inspect the package fields in
`Cargo.toml` for NFR-002-AC-5 and NFR-002-AC-6; run `make deny` for NFR-002-AC-7.
Run the exact-profile probe and its injected panic control in TC-206 for
NFR-002-AC-8 and AC-9, and compile the direct public-kernel API controls for
AC-10. This gate supplements the existing exact no-std build in TC-199; a
successful build alone does not establish a retained panic-inspection population.
The public API documentation states its size,
panic, feature, and compatibility contracts.

## Dependencies

- **Upstream**: [FR-002](../functional/FR-002-safe-operators.md).
