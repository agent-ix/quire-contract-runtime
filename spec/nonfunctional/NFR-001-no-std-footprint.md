---
id: NFR-001
title: "Allocation-free no_std core"
type: NFR
quality_attribute: performance_efficiency
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: constrains
---
# NFR-001: Allocation-free no_std core

## Statement

The default runtime shall compile without `std`, heap allocation, unsafe code, or required third-party
dependencies, and its release artifact shall remain within the measured v0.1 footprint budget.

## Scope

The default feature set and every symbol linked by generated customer code.

## Rationale

Embedded and assurance-sensitive consumers need predictable resource use and a small trusted surface.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Default dependencies | 0 | 0 | compile-time-check |
| Unsafe blocks | 0 | 0 | inspection |
| Linked `.text` + `.rodata` | 500 B population floor | 4 KiB ceiling | performance-benchmarking |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | The default feature profile compiles without `std` and resolves no runtime dependencies. | compile-time-check (TC-005) |
| NFR-001-AC-2 | Library source contains no `unsafe` block. | Inspection (TC-007) |
| NFR-001-AC-3 | On Rust 1.75 for `thumbv7em-none-eabi`, MP-001's fixed-population static-library consumer has linked `.text` plus `.rodata` between 500 bytes and 4 KiB and its runtime/harness objects retain no panic-path reference. | Inspection (TC-007) |

## Verification

The local composite gate checks every declared target and feature at the MSRV, builds and tests the
fixed bare-metal footprint consumer, runs the unsafe and panic audits, and publishes the
linked-section and panic-relocation measurements identified by MP-001 as a structured result.

The observational release-rlib byte count is retired. It gated nothing, it varied with compiler
metadata and build paths, and the collector that recorded it no longer exists; MP-001's Interpretation
records the retirement and its reason. The governed measurement is unchanged.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
