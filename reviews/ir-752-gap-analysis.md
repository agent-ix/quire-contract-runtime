---
id: SR-7521
title: "IR-752 exact quantity criterion-to-test gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@fe731da5c15aeae1de53bc370da3af0012870b4f; tests/exact_semantics.rs; src/exact/quantity.rs; spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/TC-034-local-exact-semantics.md; quire-specification FR-142-AC-5/6"
review_set: subset
---

## Summary

Ticket: IR-752. Reviewed the changed TC-034 quantity assertions against RT FR-007-AC-12 and QSpec FR-142-AC-5/6, then checked the same author's two-line fix at fe731da5c15aeae1de53bc370da3af0012870b4f. This is a diff-scoped criterion-to-test review; repository-wide matrix status and plan completion were not assessed.

## Verdict

**PASS after disposition** — Exact values, compound root identity, cancellation, `0^0`, undefined zero to a negative exponent, and affine refusal in both operand positions have direct oracles. FND-001 is fixed on the replacement head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `Multiply` and `Divide` test affine refusal only when Celsius is the left operand. If the right-operand affine check is removed, every new affine assertion still passes; add right-affine cases and assert `AffineUnitArithmetic` with zero admitted charges. | tests/exact_semantics.rs:249 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fe731da5c15aeae1de53bc370da3af0012870b4f — `Multiply(&kelvin, &celsius)` and `Divide(&kelvin, &celsius)` now use the common cause and zero-charge assertions. |

After excerpt (`tests/exact_semantics.rs:249`, replacement head):

```rust
QuantityOperation::Multiply(&celsius, &kelvin),
QuantityOperation::Multiply(&kelvin, &celsius),
QuantityOperation::Divide(&celsius, &kelvin),
QuantityOperation::Divide(&kelvin, &celsius),
QuantityOperation::Power(&celsius, &int(2)),
```

## Coverage

- Examined FR-007-AC-12 and TC-034 for per-operation type refusal and zero charges; examined QSpec FR-142-AC-6 for exact compound results and runtime power edges; read QSpec FR-142-AC-5 for affine arithmetic refusal.
- The two new test functions both carry `Trace: TC-034, FR-007-AC-12`, which correctly binds the RT criterion. The right-affine branch in `src/exact/quantity.rs` type checking is now tested for both `Multiply` and `Divide`.
- Plan completion: not assessed.
- `cargo test --features exact --test exact_semantics`: 3 passed, 0 failed. Full native gates, strict coverage, and Kani were not rerun in this scoped review.
