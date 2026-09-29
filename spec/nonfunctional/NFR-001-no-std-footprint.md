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

The optional `snapshot-json` codec uses bounded allocating storage and requires an
allocator, but not `std`. Its separate bare-metal compile is not a default-profile
footprint measurement. The governed fixed-population default footprint remains unchanged;
do not present that measurement as the linked size of the allocating codec.

## Rationale

Embedded and assurance-sensitive consumers need predictable resource use and a small trusted surface.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Default dependencies | 0 | 0 | compile-time-check |
| Unsafe blocks | 0 | 0 | compile-time-check |
| Linked `.text` + `.rodata` | 500 B population floor | 4 KiB ceiling | performance-benchmarking |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | The default feature profile compiles without `std` and resolves no runtime dependencies. | compile-time-check (`make msrv`, `make test-features`) |
| NFR-001-AC-2 | The library compiles with no `unsafe` code. | compile-time-check (`#![forbid(unsafe_code)]`, `make lint`) |
| NFR-001-AC-3 | On Rust 1.75 for `thumbv7em-none-eabi`, MP-001's fixed-population static-library consumer has linked `.text` plus `.rodata` between 500 bytes and 4 KiB and its runtime/harness objects retain no panic-path reference. | Test (TC-007, `make size`) |

## Verification

The build gates verify this requirement. `make msrv` and the no_std rows of `make test-features`
compile the default, `snapshot-json` and `exact` profiles without `std` on Rust 1.75.
`#![forbid(unsafe_code)]`, checked by `make lint`, rejects any `unsafe` code. `make size` builds the
fixed bare-metal footprint consumer and enforces the 500-byte floor, the 4 KiB ceiling and the
absence of panic relocations.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
