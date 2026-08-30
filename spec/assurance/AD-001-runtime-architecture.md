---
id: AD-001
title: Quire contract runtime architecture
type: ArchitectureDescription
status: proposed
owner: runtime-maintainers
system: quire-contract-runtime v0.1
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: realizes
---
# Quire contract runtime architecture

## System Boundary

The default boundary is this crate's safe Rust core: identity, observations, verdicts, operators,
checked integer helpers, and campaign counts. Generated code and callers are outside. The optional
proptest adapter is a development-only boundary; proptest itself is external and pinned.

## Views

The data view contains borrowed identities and fixed-size enums. The evaluation view calls pure or
lazy helpers and returns Boolean/`Option` values. The reporting view folds verdicts and explicit
framework discards into a complete four-counter report. No global state, I/O, heap, or unsafe code is
in the core.

## Decisions

Default features are empty and `#![no_std]` is unconditional. Borrowed details avoid allocation.
Separately named total and short-circuit functions make evaluation behavior visible. Checked integer
traits are sealed to preserve semantics. Counters saturate because evidence collection must not panic.

## Risks

The optional proptest mapping is coupled to a pinned external API. Source size is only a proxy for
target footprint. Kani coverage depends on an external proof tool. These limitations remain in the
measurement and gap reports.

