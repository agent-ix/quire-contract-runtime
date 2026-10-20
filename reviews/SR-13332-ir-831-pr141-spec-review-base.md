---
id: SR-13332
title: "spec-review/base review of IR-831 PR 141"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/assurance/AD-004-runtime-crate-layout.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/matrix/TC-197-single-kernel-ownership.md; spec/exact/matrix/TC-201-exact-quantity-conversion.md"
review_set: subset
---
## Summary

Ticket: IR-831. Frozen review head: cae3c4dc71a74f57a65a135dd0a14c14c6da269e. The new criterion lacks its matrix ledger entry.

Examined: AD-004 step 4, FR-275-AC-23, TC-197, TC-201. Context: FR-007-AC-20/21/23, QSV `IdentifiedUnit`, `UnitScope`, `convert_quantity`, and exact `UnitId`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-275-AC-23 has no row in spec/exact/matrix/tests.md; the TC-197 and TC-201 inventory rows also omit it. The new planned criterion is invisible in the authored coverage ledger, so it cannot be tracked separately from the already green FR-007 checks. | spec/exact/functional/FR-275-single-exact-kernel.md:188 |

## Verdict

Changes required before merge.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fix head f49f56d0 adds an explicitly planned and unbacked FR-275-AC-23 matrix row mapped to TC-201 and TC-197, and adds AC-23 to both test-case inventory rows while preserving the green FR-007 rows. |
