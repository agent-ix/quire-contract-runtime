---
id: SR-12006
title: "IR-818 RT gap analysis of PR 127"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime#127@b68e5c6219fd9f6aca967afa707fd06f8ef4b3f0; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md, spec/exact/matrix/tests.md, spec/spec.md, tests/exact_quantity_conversion.rs; context src/exact/quantity.rs (unchanged), QSpec origin/main FR-142 and TC-187 rows U01, U03, U04, U05, U09, U09b, U10, U15, U20, U22, U25, U26 (read-only)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
---

## Summary

Ticket: IR-818. Planless gap analysis of PR 127. It checks the criteria-to-matrix-to-tests-to-code chain for FR-007-AC-20 through AC-25 and TC-201, and whether the status rows are truthful. Plan completion: not assessed. `quire matrix --scope .` moves all six criteria from `untagged` on `origin/main` to `tagged` at b68e5c6, and no other row changes. `src/` is unchanged, so there is no reverse gap and no stub.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-007-AC-20 .. FR-007-AC-25 | examined | the six TC-201 criteria (see SR-12005 for the text) |
| FR-007 Verification notes | examined | `convert_quantity` has no RT test today. `Decimal` and `Integer` targets and any `RoundingMode` are not specified here. |
| spec/exact/matrix/tests.md rows FR-007-AC-20..25, TC-201, file disposition, evidence list | examined | ✅ RT-local tests check ... |
| TC-201 Description | examined | Evidence (IR-818): `tests/exact_quantity_conversion.rs` (`--features exact`). |
| spec/spec.md TC list | examined | planned TC-200 and TC-201 provide the verification matrix. |
| QSpec TC-187 rows | context_only | read to confirm the vectors are restated, not copied |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-007 verification note now contradicts itself. The PR rewrites its first sentence to say AC-20 through AC-25 "are backed by RT-local tests of the existing `convert_quantity`" in `tests/exact_quantity_conversion.rs`. Eight lines later the same paragraph still says "`convert_quantity` has no RT test today." That sentence was true before this PR and is false after it. Fix: delete the stale sentence. | spec/exact/functional/FR-007-exact-scalar-families.md:154-163 |

## Verdict

**CONDITIONAL**: one low finding. All six criteria are tagged by tests that assert them. The test-intent gaps in AC-20, AC-23 and AC-24 evidence are in SR-12005 and are not repeated here: multi-edge path order is unpinned, and the right-operand affine case is untested. The tests.md rows, the TC-201 row and the file-disposition row (`added (IR-818)`) truthfully describe what the file asserts. The spec.md TC list is consistent. The vectors are restated in FR-007's own words, and no QSpec bytes are copied into the test or the spec diff.

## Coverage

- Criteria in scope: 6. Tagged: 6 (`tests/exact_quantity_conversion.rs:244, 381` for AC-20, `:414` AC-21, `:448` AC-22, `:480, 632` AC-23, `:650` AC-24, `:692` AC-25). Untagged in scope: 0.
- Repo-wide `quire matrix --strict` exits 1 on 55 untagged criteria. Every one of them is also untagged on `origin/main` (61 there): this PR adds none and closes six.
- Reverse gap: none; no source change.
- Stubs and coverage inflation: none. All eight tests run real code with literal expected values.
- Semantic review: focused oracle-strength check per the dispatching brief (reviewer mutants; results in SR-12005). No full fan-out.
- Plan completion: not assessed

## Dispositions

Round 1, head 2ba10e4f630e39c27bc3dd8ff23977e95e1dd1e6, verified against the fix commit.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The stale sentence is deleted. The FR-007 verification note now reads "`Decimal` and `Integer` targets and any `RoundingMode` are not specified here. Shared-corpus agreement stays IR-669 (TC-022)." |
