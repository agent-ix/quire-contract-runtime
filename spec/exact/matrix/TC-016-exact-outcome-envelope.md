---
id: TC-016
title: "Inspect the exact outcome envelope and vocabulary"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: verifies
---
# TC-016: Inspect the exact outcome envelope and vocabulary

## Description

Check the typed outcome envelope and the closed reason and vocabulary enums of the `exact`
surface. The former RT-local kernel tests left with the copied implementation.

## Ownership and evidence

Kernel-owned after IR-349 removes the local copy. RT FR-006-AC-1 remains open:
[quire-exact FR-362-AC-11](ix://agent-ix/quire-exact/FR-362) tests scalar stop dispositions but
does not establish the whole closed-reason census or RT consumption. Kernel refusal code and cause spellings
are tested by [quire-exact FR-096-AC-8](ix://agent-ix/quire-exact/FR-096); public scalar outcome stops by
`FR-362-AC-11`; charge and limit spellings by `FR-368-AC-1` and `FR-368-AC-2`.
FR-006-AC-6 is an ownership inspection, not a TC-016 test criterion. The two retired
four-code tests and their stale tags were removed with the local kernel tests. Owner tests
establish the kernel vocabulary; RT retains no duplicate census.

## Test Procedure

1. Build one outcome of each disposition, including completed `true` and `false`; compare every
   pair for equality and check that only completed outcomes yield a value.
2. Inspect the consumed kernel `Outcome::Refused(Refusal)` and `Outcome::Undefined(Undefined)`
   reason enums and the `Incomplete` record with its `LimitKind` and `ChargePoint` fields.
   Confirm the runtime exposes no separate reason carrier. Do not infer the kernel
   code/cause census from the retired local four-code tests.
3. Round-trip every `ChargePoint` and `LimitKind` through its spelling; check the eleven QSpec
   families are present and `equality.plan-form` is absent.

## Expected Results

Four distinct dispositions; only `Completed` yields a value. `Refusal` and `Undefined`
are closed reason enums, while `Incomplete` carries closed `LimitKind` and `ChargePoint`
reasons in a typed record. The charge and limit spellings match QSpec.
FR-006-AC-6's kernel code/cause ownership is inspected separately under FR-275.
