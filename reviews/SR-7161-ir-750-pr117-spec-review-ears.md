---
id: SR-7161
title: "EARS review of IR-750 decimal behavior statement"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime@bf693a903a965d08d9443e06b6183be49c57b5cc; spec/exact/functional/FR-007-exact-scalar-families.md"
review_set: subset
---

## Summary

Ticket: IR-750. The edited FR-007 decimal behavior clarifies the existing requirement's value-versus-charge response without adding a second obligation, ambiguous trigger, or unmeasurable result. The scoped Quire grammar check reports no EARS finding.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-007 description | context_only | spec/exact/functional/FR-007-exact-scalar-families.md | When the `exact` feature is enabled, the runtime shall consume `quire-exact` for kernel-owned operations in the scalar families of complete V1 in agent-ix/quire-specification with values and outcome kinds equal to the quire-spec-language authority on every shared-corpus vector, and charges equal to the QSpec accounting schedule. |
| FR-007 decimal behavior | examined | spec/exact/functional/FR-007-exact-scalar-families.md | *Ordering size charges* use the retained operand representations, never their normalized forms, so an equal-value pair can have different ordering size counters. The Boolean `ordering.result-retain` charge remains one result unit for either pair. Decimal arithmetic's result-retain upscale charges are a separate operation (AC-2). |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the changed FR-007 statement. The changed behavior is specific, measurable, and consistent with the existing `When ... shall ...` requirement; TC-034 supplies a concrete witness.
