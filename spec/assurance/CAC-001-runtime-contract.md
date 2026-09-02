---
id: CAC-001
title: Runtime component assurance contract
type: ComponentAssuranceContract
status: proposed
owner: runtime-maintainers
kind: deterministic
responsibility: preserve contract-oracle outcomes without panic or conflation
inputs: [borrowed identities, clause outcomes, Boolean operands, checked integer operands]
outputs: [tri-state verdicts, observations, defined values, complete counters]
invariants: [rejection is never success, undefined operations never panic]
failure_behaviors: [return a typed failure or None, saturate counters]
version_pins:
  rust-msrv: "1.75"
  governance: agent-ix/quire-contract-ir#3
controls:
  surfaces: [Cargo features, CI, requirement-tagged tests, sealed proof attestations]
  fallback: disable optional adapters and use the dependency-free core
  abstention: retain rejected discarded and inconclusive states
  escalation: human release owner reviews unresolved gaps
isolation: no dependency on Quoin Quire or code-generation repositories
replacement: preserve public semantics and evidence identities
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: references
---
# Runtime component assurance contract

## Component Boundary

The component owns only the linked runtime and opt-in adapter. Parsers, generators, test campaign
orchestration, proof engines, and release decisions remain outside.

## Required Behavior

Every oracle terminal state remains one of pass, failed postcondition, or rejected precondition.
Clause evidence preserves identity. Operator evaluation follows its named contract. Every report
contains accepted, rejected, failed, and discarded counters.

## Failure Handling

Undefined value operations return `None`; counter overflow saturates. Adapter failures and rejections
remain distinct. The core performs no I/O and has no intentional panic path.

## Controls

Feature-matrix CI, requirement-tagged tests, cargo-deny, unsafe audit, API documentation, and the
governed footprint measurement sealed into a Quoin proof attestation constrain changes. Human review
is mandatory for release. This repository retains no measurement envelope of its own.

## Replacement

A replacement must pass the same conformance vectors, preserve all terminal states and identities,
meet the no_std/footprint boundary, and record a new human decision.

