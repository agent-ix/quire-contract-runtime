---
type: master-requirements
name: quire-contract-runtime
org: agent-ix
component_type: rust-library
implementation_language: rust
tags: [contract-runtime, no-std]
standards_alignment: [iso-iec-ieee-29148]
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
adapters. Reviewers rely on the tests, proofs and measurements this repository
runs.

## Subsystems

### Subsystem Registry

Layout follows `ix://agent-ix/quire-contract-ir/ADR-0056`. Each subsystem directory under `spec/`
holds its requirements (`stakeholder/`, `functional/`, `non-functional/`) and, in `matrix/`, its one
matrix and the test cases that matrix declares. Interface requirements live in `functional/`.
`spec/tests.md` indexes the matrices; `spec/assurance/` holds the architecture descriptions.

| Subsystem | Path | Role | Owning crates/modules | ADs | Owner |
| --- | --- | --- | --- | --- | --- |
| Core | `spec/core/` | Verdict, identity and observation types; panic-free operators; the runtime API contract; the `no_std` footprint and panic/license quality requirements | `quire-contract-runtime`: `verdict`, `identity`, `observation`, `operators`, `kani_proofs` (`verification/kani.rs`, `cfg(kani)`); `quire-contract-runtime-footprint` | AD-001, AD-003 | runtime-maintainers |
| Accounting | `spec/accounting/` | Complete campaign counters and the bounded immutable campaign snapshot transport | `quire-contract-runtime`: `accounting` (including its snapshot transport, `src/snapshot_json.rs`) | AD-001, AD-003 | runtime-maintainers |
| Proptest adapter | `spec/proptest_adapter/` | Adaptation of verdicts to the proptest framework | `quire-contract-runtime`: `proptest_adapter` | AD-001, AD-003 | runtime-maintainers |
| Exact | `spec/exact/` | The exact-oracle operators and typed outcome/accounting envelopes: scalar, text, composite, collection, equality, expression and function application, backend negotiation, the injected-denial seam and the carried compiler vocabulary | `quire-contract-runtime`: `exact` (the runtime-owned items only; the value kernel is the `quire-exact` crate, FR-275) | AD-001, AD-002, AD-003 | runtime-maintainers |

## Requirements Architecture

Stakeholder requirement StR-001 is refined by functional requirements FR-001 through FR-004,
FR-006 through FR-012, FR-273, FR-275, and quality requirements NFR-001 and NFR-002. `interface-001` defines the language-neutral
runtime API contract implemented by those FRs. FR-275 states the end state of the exact kernel: one
`quire-exact` kernel, consumed and never copied. Test cases TC-001 through TC-004, TC-006, TC-007, TC-015 through
TC-035 (TC-027–TC-029 unused), TC-194–TC-195 and TC-197–TC-199 provide the verification matrix.

## References

- [Program umbrella](https://github.com/agent-ix/quire-contract-ir/issues/1).
- [Runtime epic](https://github.com/agent-ix/quire-contract-runtime/issues/4).
- [Codegen to runtime seam](assurance/AD-003-codegen-runtime-seam.md).
