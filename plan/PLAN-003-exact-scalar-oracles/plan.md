---
id: PLAN-003
title: "Exact complete-V1 scalar oracle operators and typed outcomes"
type: Plan
status: active
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---
# PLAN-003: Exact complete-V1 scalar oracle operators and typed outcomes

## Scope

The scalar slice of `agent-ix/quire-contract-runtime#16`: an optional `exact` feature carrying
typed outcomes, `quire.value.accounting/v1` metering and the scalar operator families of complete V1
at agent-ix/quire-specification@5d88578. The runtime implements that definition and is checked
against it; quire-spec-language d9d5273 remains the value authority.

Out of scope: composites, collections, equality plans and function calls
(agent-ix/quire-spec-language#119), model domains (#120) and replay (#121).

## Dependency Graph

`Task-001 -> Task-002 -> Task-003`

## Task File Mapping

| Task | Scope | Status |
|---|---|---|
| [Task-001](./tasks/Task-001-outcomes-and-accounting.md) | Outcome envelope, meter and vocabularies (FR-006) | done |
| [Task-002](./tasks/Task-002-scalar-families.md) | Scalar operator families (FR-007) | done |
| [Task-003](./tasks/Task-003-shared-corpus-agreement.md) | Shared-corpus agreement against the authority | done |

## Completion Rule

Complete for the scalar slice when every TC-016 through TC-023 test passes, every evaluated TC-185,
TC-186, TC-187, TC-192 and TC-193 vector agrees with the authority in value and outcome kind, and
every charge not yet metered by the authority is named in its test. Issue #16 stays open for the
composite families.
