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
| Linked `.text` + `.rodata` | recorded baseline | 4 KiB | performance-benchmarking |
| Release rlib bytes | recorded observation | not enforced | performance-benchmarking |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | The default feature profile compiles without `std` and resolves no runtime dependencies. | compile-time-check (TC-005) |
| NFR-001-AC-2 | Library source contains no `unsafe` block. | Inspection (TC-007) |
| NFR-001-AC-3 | On Rust 1.75 for `thumbv7em-none-eabi`, MP-001's fixed-population static-library consumer has linked `.text` plus `.rodata` no greater than 4 KiB. | Inspection (TC-007) |

## Verification

CI checks every declared target and feature at the MSRV, builds the fixed bare-metal footprint
consumer, runs the unsafe audit, and retains the linked-section, observational rlib, and dependency
outputs identified by MP-001.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
