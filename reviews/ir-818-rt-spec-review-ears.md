---
id: SR-11971
title: "IR-818 RT EARS review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime@c336b9bf9c17093ffdf8fb18f0badd7a6d51684b; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
---

## Summary

Ticket: IR-818. Examined FR-007-AC-20 through AC-23 and TC-201's procedure for three things: one subject, a stated condition, and an observable response that a wrong implementation would fail. Each named vector was checked against QSpec TC-187 on `quire-specification` `origin/main` (read only) and against `src/exact/quantity.rs` on RT `origin/main`. The criteria follow FR-007's existing declarative style (AC-17 to AC-19), which is accepted here; the findings concern falsifiability and accurate vector binding.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-007-AC-20 | examined | ... with no `DecimalLoss` and no rounding: `250 cm` to `m` is `5/2 m` (QSpec TC-187 U01), ... a distinct unit key under `L` with scale one converts to `m` exactly while keeping its own unit identity (U09b), ... |
| FR-007-AC-21 | examined | `convert_quantity_exact` between declared units on distinct dimension nodes, or between units with different normalized base-dimension maps, returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read` or other charge, leaving the meter unchanged: `1 m` to `s` (U03-style), ... (U09), and `1 N_m` to `J` (U22). |
| FR-007-AC-22 | examined | `convert_quantity_exact` converts an affine source or target unit exactly while `Add`, `Subtract`, `Multiply`, `Divide` and `Power` on that unit still return `IllTyped` with cause `AffineUnitArithmetic` with zero charges: `0 degC` converts to `K` while `1 u1 + 2 u1` and `1 u3 * 1 m` refuse (U04, U20 conversion side). |
| FR-007-AC-23 | examined | `convert_quantity_exact` charges one `unit.identity-read`, then all `unit.edge` charges ..., and a limit that denies a charge returns `Incomplete` ...: `100 cm` to `m` under the exact U10 tuple returns `1 m` and a denied `unit.result-retain` is `Incomplete` (U10); ... |
| TC-201 Test Procedure | examined | 1. Convert the FR-007-AC-20 vectors ...; check no `DecimalLoss` is reported. ... 4. ... run Add, Subtract, Multiply, Divide and Power on affine units (U20) ... |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-007-AC-22 names five refused operations but binds two: `1 u1 + 2 u1` (Add) and `1 u3 * 1 m` (Multiply). "On that unit" refers to the converted affine unit (`degC`), yet the refusal vectors use `u1` and `u3`. "U20 conversion side" mislabels U20, which is arithmetic only. TC-201 step 4 says U20 runs all five operations, but U20 holds only add and multiply. QSpec U05 is the vector that covers all five on `degC`/`degF`. Bind U05 (or name all five on one affine unit) and drop "conversion side". | spec/exact/functional/FR-007-exact-scalar-families.md:145, spec/exact/matrix/TC-201-exact-quantity-conversion.md:29-30 |
| FND-002 | low | In FR-007-AC-23, the U10 denial clause cannot fail on any field value. "A denied `unit.result-retain` is `Incomplete`" states no denying tuple and no `limit_kind`, `limit`, `consumed` or `next_charge`, although the same AC requires those fields and gives them for U15, U22 and U26. State the denying tuple, for example U10's tuple with `result_units: 0` or `work_units: 5`, and the expected fields. | spec/exact/functional/FR-007-exact-scalar-families.md:146 |
| FND-003 | low | FR-007-AC-23 carries two responses: the success charge order, and a denial that returns `Incomplete` with no partial quantity, each with its own vectors. It is testable as written, but a failure report cannot say which half broke. Splitting it into an order AC and a denial AC would make each row's failure point at one behaviour. AC-14 is skipped and the new AC would be appended, so nothing is renumbered. | spec/exact/functional/FR-007-exact-scalar-families.md:146 |
| FND-004 | low | "With no `DecimalLoss`" (FR-007-AC-20) and "check no `DecimalLoss` is reported" (TC-201 step 1) cannot fail: the function returns `Outcome<Quantity>`, which has no place to carry a loss record. Drop the clause, or restate it as the signature claim it is ("returns a `Quantity`, not a `Conversion`"). | spec/exact/functional/FR-007-exact-scalar-families.md:143, spec/exact/matrix/TC-201-exact-quantity-conversion.md:23-24 |
| FND-005 | low | In FR-007-AC-20, U09b's "keeping its own unit identity" is ambiguous: the result is in `m`, so it is unclear whose identity is kept. TC-187 U09b states the checkable property: source and target unit identities remain distinct. Say "the source unit is not equal to `m`, and the result unit is `m`". | spec/exact/functional/FR-007-exact-scalar-families.md:143 |

## Verdict

CHANGES REQUESTED (one medium, four low). FR-007-AC-21 is singular and falsifiable: one condition, one typed `Err`, a zero-charge meter check, and three named vectors that each match TC-187 (U03 is honestly labelled "U03-style", since TC-187's U03 is an add). FR-007-AC-20's values all check against TC-187 and the code: `5/2 m`; `5463/20 K` for both `0 degC` and `32 degF`; `4 m2`; `1 J`; `-9 u1`. Its `i128` oracle sweep is a well-formed universal property. FR-007-AC-23's quoted tuples for U15, U22 `cm2` and U26 are correct as stated.
