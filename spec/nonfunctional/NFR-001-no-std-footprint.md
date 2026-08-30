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
| Release rlib size | recorded baseline | 256 KiB | performance-benchmarking |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | The default feature profile compiles without `std` and resolves no runtime dependencies. | compile-time-check (TC-005) |
| NFR-001-AC-2 | Library source contains no `unsafe` block. | Inspection (TC-007) |
| NFR-001-AC-3 | The retained release rlib measurement is no greater than 256 KiB. | Inspection (TC-007) |

## Verification

CI builds with no default features, runs the unsafe audit, and retains the size and dependency output
identified by MP-001.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
