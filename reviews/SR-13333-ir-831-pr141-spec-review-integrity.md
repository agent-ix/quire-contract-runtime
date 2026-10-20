---
id: SR-13333
title: "spec-review/integrity review of IR-831 PR 141"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/matrix/TC-197-single-kernel-ownership.md; spec/exact/matrix/TC-201-exact-quantity-conversion.md"
review_set: subset
---
## Summary

Ticket: IR-831. Frozen review head: cae3c4dc71a74f57a65a135dd0a14c14c6da269e. The conversion-to-quantity handoff is unspecified and currently type-incompatible.

Examined: AD-004 step 4, FR-275-AC-23, TC-197, TC-201. Context: FR-007-AC-20/21/23, QSV `IdentifiedUnit`, `UnitScope`, `convert_quantity`, and exact `UnitId`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The U25 plan asks UnitScope::resolve to read the first conversion result, but QSV convert_quantity returns Outcome<Conversion> and UnitScope::resolve accepts &Quantity. Conversion exposes value and unit, not a Quantity. The spec must name the QSV-owned exact result-to-Quantity construction and retention path, or require a QSV API for it, before the two-step claim is executable without an RT adapter. | spec/exact/matrix/TC-201-exact-quantity-conversion.md:40-46 |

## Verdict

Changes required before merge.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fix head f49f56d0 states that QSV does not yet expose the handoff, specifies a QSV-owned exact Conversion to (Quantity, IdentifiedUnit) result with refusal and charge semantics, and requires that API before RT step 4. TC-201 now passes the returned Quantity to UnitScope::resolve. |
