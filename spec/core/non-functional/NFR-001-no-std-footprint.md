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
| Unsafe blocks | 0 | 0 | compile-time-check |
| Linked `.text` + `.rodata` | 500 B population floor | 4 KiB ceiling | performance-benchmarking |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-001-AC-1 | The default feature profile compiles without `std` or an allocator. | compile-time-check (`make size`) |
| NFR-001-AC-2 | The library compiles with no `unsafe` code. | compile-time-check (`#![forbid(unsafe_code)]`, `make lint`) |
| NFR-001-AC-3 | On Rust 1.82 for `thumbv7em-none-eabi`, the fixed-population static-library consumer in `measurement/footprint/` has linked `.text` plus `.rodata` between 500 bytes and 4 KiB and its runtime/harness objects retain no panic-path reference. IR-349 part 1 re-measured it at 1.82 (913 bytes; 907 bytes at 1.75, the previous floor); the floor change is specified in FR-275-AC-20 and AC-21. | Test (TC-007, `make size`) |

## Verification

`make size` builds the footprint crate, which depends on the runtime with
`default-features = false`, for `thumbv7em-none-eabi` on Rust 1.82 (the floor of FR-275-AC-20; re-measured by IR-349 part 1), so the default profile compiles
without `std` or an allocator; it then enforces the 500-byte floor, the 4 KiB ceiling and the
absence of panic relocations. `make lint` runs Clippy on the runtime and the footprint crate, and
`#![forbid(unsafe_code)]` rejects any `unsafe` code. No gate checks that the default profile
resolves no required third-party dependency.

## Dependencies

- **Upstream**: [FR-001](../functional/FR-001-verdict-observation.md).
