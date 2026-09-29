---
id: StR-002
title: "Machine-checkable specification with unique identifiers"
type: StR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-274
    type: satisfied_by
---
# StR-002: Machine-checkable specification with unique identifiers

## Stakeholder Need

Reviewers and spec authors require that the specification's identifiers shall stay unique and
machine-checked, so that every trace tag, matrix row and cross-repository reference binds to exactly
one artifact.

## Rationale

A duplicated identifier silently unbinds the tests that trace to it, and a move that loses or
renumbers an identifier breaks every reference to it without any error.

## Validation Criteria

| ID | Criteria | Validation |
|----|----------|------------|
| StR-002-VC-1 | A specification change that duplicates an identifier, or loses one in a move relative to `origin/main`, fails `make spec`. | Test (TC-196) |

## Dependencies

- **Satisfied by**: [FR-274](../functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md).
