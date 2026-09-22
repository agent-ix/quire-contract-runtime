---
type: master-requirements
name: quire-contract-runtime
org: agent-ix
component_type: rust-library
implementation_language: rust
tags: [contract-runtime, no-std, assurance]
depends_on:
  - ix://agent-ix/quire-contract-ir/PGM-01
standards_alignment: [iso-iec-ieee-29148]
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: depends_on
    cardinality: "1:1"
security_critical: false
---
# Master Requirements Specification

## Purpose

This specification defines the small runtime linked by generated contract oracles. It makes
precondition rejection, postcondition failure, clause evaluation, and campaign accounting explicit
without making a certification or accreditation claim.

## Scope

### In Scope

- Allocation-free verdict, identity, observation, and failure types for generated code.
- Panic-free Boolean, option, index, arithmetic, and division helpers.
- Optional property-test adaptation and complete campaign counters.
- Optional exact-oracle operators and typed outcome/accounting envelopes, under the `exact`
  feature, for the complete-V1 scalar, composite, collection, equality, containment-graph,
  reference, backend-negotiation and total-pure-function-application families.

### Out of Scope

- Contract parsing, canonicalization, code generation, campaign orchestration, and integration into
  Quoin or Quire.
- Project-specific validation, accreditation, certification, and human release approval.
- Mutable state: `modifies`/`creates`/`deletes` frame-obligation semantics and any `negotiate_frame`
  predicate beside `negotiate_ieee` and `negotiate_integer_division`. That construct is negotiated
  at the code generator's arrow, not this runtime's, and is blocked on IR lowering the frame node
  ([quire-contract-ir#109](https://github.com/agent-ix/quire-contract-ir/issues/109)).
- Temporal semantics, protocol encodings admitted by a backend profile, and replay. The language
  authority that defines them has not shipped
  ([quire-spec-language#121](https://github.com/agent-ix/quire-spec-language/issues/121)), and this
  runtime is an implementation of that definition, never a second semantic authority.

## System Overview

### System Description

The crate is an independent `no_std` library. Generated or hand-authored oracles evaluate clauses,
return a tri-state verdict, and optionally adapt that verdict to a property-testing framework.

### Intended Users

Generated customer code relies on the default core. Test harness authors may enable optional
adapters. Reviewers and release owners rely on the Quire static export, the sealed Quoin change-assurance
record, its proof attestations and its verification receipt. This repository retains no evidence of
its own.

## Requirements Architecture

Stakeholder requirement StR-001 is refined by functional requirements FR-001 through FR-012,
FR-273, and quality requirements NFR-001 and NFR-002. `interface-001` defines the language-neutral
runtime API contract implemented by those FRs. Test cases TC-001 through TC-034 and TC-194–TC-195
(TC-012, TC-027–TC-029 unused) provide the verification matrix. The assurance artifacts bind the
intended use, boundary, evidence, and open human decision.

## References

- [Program umbrella](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [PGM-01 governance gate](https://github.com/agent-ix/quire-contract-ir/issues/3), identified as
  `ix://agent-ix/quire-contract-ir/PGM-01`; this specification does not redefine it.
- [Runtime epic](https://github.com/agent-ix/quire-contract-runtime/issues/4).
