---
id: PLAN-001
title: "Runtime v0.1 implementation and release preparation"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-contract-runtime/StR-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: references
---
# PLAN-001: Runtime v0.1 implementation and release preparation

## Scope

Implement and verify the dependency-free runtime core, optional proptest adapter, assurance
artifacts, traceability, and retained candidate evidence. Release authority remains outside the
implementation boundary and belongs to the named human owner.

## Dependency Graph

`Task-001 -> Task-002 -> Task-003 -> Task-004 -> Task-005 -> Task-006`

PGM-01 review and the merged manual-CI PR #6 base are external inputs to Task-006, not substitutes for human
authority.

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-foundation.md) | Foundation and assurance specification | done |
| [Task-002](./tasks/Task-002-runtime-model.md) | Runtime identity, observation, and verdict model | done |
| [Task-003](./tasks/Task-003-runtime-behavior.md) | Operators, adapters, and accounting | done |
| [Task-004](./tasks/Task-004-verification.md) | Tests, proofs, and traceability | done |
| [Task-005](./tasks/Task-005-evidence.md) | Evidence collection and gap review | done |
| [Task-006](./tasks/Task-006-human-release.md) | Human source-release decision | not_started |

## Completion Rule

Implementation completion means Tasks 001-005 are `done` and locally reproducible. PLAN-001 remains
`active` until protected manual checks, CODEOWNER review, upstream reconciliation, and Task-006 are
complete. Automation must not change Task-006 to `done`.
