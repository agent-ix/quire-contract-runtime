---
id: PLAN-002
title: "Adopt the shared assurance contract and retire the local evidence framework"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AA-001
    type: references
---
# PLAN-002: Adopt the shared assurance contract and retire the local evidence framework

## Scope

Move this repository from its own evidence collector, envelope builder, verifier, anchor census, and
Makefile self-attestation onto the released Engineering Assurance 0.2.0, quire-cli 0.31.0 (engine
0.46.0), quoin 0.23.1, and ix-flow 0.0.4 contracts, while keeping every byte of retained evidence
and every domain behaviour this crate owns.

The migration issue is `agent-ix/quire-contract-runtime#8`; the contract is
`agent-ix/engineering-assurance#10`.

## Dependency Graph

`Task-001 -> Task-002 -> Task-003`

The deletion in Task-003 is deliberately last. Both paths coexist until the old path has been run at
the same candidate revision and its result recorded as observed, because deleting a verifier before
running it is how a repository loses the ability to say whether the two ever agreed.

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-inventory-and-pins.md) | Keep/replace/delete/defer inventory and the accepted pins | done |
| [Task-002](./tasks/Task-002-shared-intake.md) | Producers, adapter, chain, compatibility view, and tests | done |
| [Task-003](./tasks/Task-003-dual-run-and-deletion.md) | Dual run at one revision, then delete the old path | done |

## Completion Rule

Complete when `make ci` passes on the new path at the exact head, the old path's result at that same
head is recorded as observed rather than as parity, the generic machinery is deleted, and every byte
under `evidence/` is unchanged. Human release authority is not in scope and remains PLAN-001
Task-006.
