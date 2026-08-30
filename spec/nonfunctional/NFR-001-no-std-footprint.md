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
| Default dependencies | 0 | 0 | `cargo metadata --no-default-features` |
| Unsafe blocks | 0 | 0 | unsafe audit |
| Release rlib size | recorded baseline | 256 KiB | reproducible release build |

## Verification

CI builds with no default features, runs the unsafe audit, and retains the size and dependency output
identified by MP-001.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
