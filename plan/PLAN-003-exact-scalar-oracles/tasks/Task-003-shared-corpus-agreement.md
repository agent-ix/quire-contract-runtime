---
id: Task-003
title: "Shared-corpus agreement against the authority"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---
# Task-003: Shared-corpus agreement against the authority

## Scope

A separate `conformance/qsl-agreement` package executes each shared vector twice, once on
quire-spec-language d9d5273 and once on the runtime, and compares Debug renderings. It is never a
dependency of the runtime. `make conformance` runs it.

## Known upstream lag

d9d5273 does not meter `ordering.*`, `integer-arithmetic.*`, `rational-arithmetic.*` or
`boolean.result-retain`. TC-185 D20 and D21 compare values against the authority and charges
against QSpec 5d88578 only, and are listed in `CHARGES_PENDING_QSL_119`. Runtime charges are never
adjusted to match the authority.

## Completion Evidence

TC-018 through TC-022: 12, 21, 33, 17 and 30 evaluated vectors respectively, with admission-only
vectors named in each test.
