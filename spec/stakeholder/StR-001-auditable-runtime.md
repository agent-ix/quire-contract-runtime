---
id: StR-001
title: "Auditable generated-oracle runtime"
type: StR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: satisfied_by
---
# StR-001: Auditable generated-oracle runtime

## Stakeholder Need

Assurance engineers require that the runtime shall preserve every non-success state for generated
contract harnesses through a small, inspectable, permissively licensed boundary.

## Rationale

Conflating rejected inputs with successful evidence creates vacuous confidence. A small independent
runtime limits the trusted surface and allows downstream projects to validate it for their own use.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-001-VC-1 | A harness distinguishes success, failed postconditions, and rejected preconditions. | Demonstration |
| StR-001-VC-2 | The default linked surface compiles with `#![no_std]`, invokes no allocator, and resolves no runtime dependency. | Inspection |

## Dependencies

The governing compatibility, provenance, evidence, and qualification policy is PGM-01 at
`ix://agent-ix/quire-contract-ir/PGM-01`.
