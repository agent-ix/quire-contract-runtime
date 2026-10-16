---
id: SR-11601
title: "IR-61 F14 semantic tests: gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@7afe79cb9eb671d50a8d9594a0a96c3e80852327 (base 5e64b66212fb7dea5d030a5677104a9f25e6a551); tests/exact_semantics.rs; spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/tests.md; spec/exact/matrix/TC-034-local-exact-semantics.md"
review_set: subset
---

## Summary

Ticket: IR-61. Branch `code/ir-61-f14-semantic-tests`, no PR yet. This analysis has no
plan, so plan completion was not assessed. It checks FR-007-AC-8 and AC-15 to AC-19 against
the computed Test Matrix, the six new tagged tests and the quire-exact 6f4cc48 kernel those
tests exercise.

## Verdict

**FAIL: one medium finding and two low findings.** AC-15 to AC-19 are fully backed. AC-8 is
partly backed, and the partial note does not name everything that is missing.

`quire matrix` changes six criteria from untagged to tagged and nothing else: FR-007-AC-8 and
AC-15 to AC-19. Coverage rises from 72/178 rows backed at base to 78/178 at head.
`quire coverage --strict` reports 112 unbacked rows at both base and head, and `quire
validate` reports 189 of 189 documents grammar-clean.

Every literal input and expected value matches TC-034 steps 1, 9, 10, 12, 13 and 14, and the
FR-007 criterion text. Two vectors were recomputed by hand from the kernel source:

- **AC-15:** `(1,0) / (-3,0)` under TowardZero. The exact value is -1/3, and
  `div_mod_floor(-1, 3)` gives floor -1 and ceiling 0. Because the exact value is negative,
  TowardZero takes the ceiling, 0. The loss record is `scaled(-1/3, 0)`, which is -1/3.
- **AC-18:** `-7 mod -3`. The floor quotient is 2 with remainder -1. The Euclidean step makes
  that quotient 3 and remainder 2. `7 mod -3` similarly gives -2, corrected to 1.

AC-17's unit graph matches the criterion exactly. `meter` is the root; `kilometer` targets
`meter` with scale 1000 and offset 0. The expected results are `6000 m²` and `2000/3`
dimensionless. AC-15's target is `DecimalType::new(-100, 100, 0, 0, mode)`, which is
coefficient bounds [-100,100] at scale 0. Each matrix row changed to ✅ belongs to a criterion
that a test kills under mutation; SR-11600 records the mutants.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-007-AC-8 is computed as `tagged`, so it counts as backed in the 78/178. The partial note in the matrix and in FR-007 lists only the omitted spelling as missing. The criterion's first sentence is also untested: the six spellings `exact` ... `nearest-away` "corresponding to the six named `RoundingMode` variants". The consumed kernel exposes that mapping as `RoundingMode::as_str` and `from_code` (`quire-exact src/decimal.rs:195-212`), and no RT test calls either. Failure: the kernel renames `toward-zero` or swaps two spellings, the RT suite stays green, and AC-8 is still reported as backed with a note that does not mention this clause. | spec/exact/matrix/tests.md:36; spec/exact/functional/FR-007-exact-scalar-families.md:114,128-131; tests/exact_semantics.rs:65-100 |
| FND-002 | low | The note calls AC-8's omitted-spelling clause "a compiler-side obligation". Yet the consumed `quire_exact::RoundingMode` encodes it directly: it derives `Default` with `#[default] Exact` (`quire-exact src/decimal.rs:165-170`), and the kernel's own doc comment states "an omitted spelling is strict `RoundingMode::Exact`". RT never constructs a defaulted mode (no `RoundingMode::default()` in `src/`), so the classification is defensible. Still, `RoundingMode::default() == RoundingMode::Exact` is an RT-observable assertion on consumed behaviour. Failure: the kernel default changes and nothing in RT notices, while the note sends readers to a compiler test that does not cover the kernel type. | spec/exact/matrix/tests.md:36; spec/exact/functional/FR-007-exact-scalar-families.md:130-131 |
| FND-003 | low | TC-034's "Ownership and evidence" paragraph still says "The kernel tests left with the copied implementation; RT's quantity cases remain in `tests/exact_semantics.rs`". The file now also holds RT rounding, quotient-sign, `mod` and division-exposure tests (TC-034 steps 1, 9, 10 and 12). The diff updates the TC-034 matrix row but not this paragraph. Failure: a reader of TC-034 concludes that steps 1, 9, 10 and 12 have only kernel-owned evidence. | spec/exact/matrix/TC-034-local-exact-semantics.md:32-33 |

## Coverage

Examined:

- FR-007-AC-8: "The six rounding spellings are `exact`, `toward-zero`, `toward-positive`, `toward-negative`, `nearest-even` and `nearest-away`, corresponding to the six named `RoundingMode` variants. For positive and negative exact ties, ... An omitted spelling is `Exact` and refuses a discarded nonzero digit."
- FR-007-AC-15, decimal division under TowardZero and NearestAway with exact loss: clean
- FR-007-AC-16, affine `Power` at exponents 0 and -1 with zero charges: clean
- FR-007-AC-17, same-dimension `Multiply` and `Divide`: clean
- FR-007-AC-18, Euclidean `mod` in four quadrants and `ModuloOutOfDomain`: clean
- FR-007-AC-19, single-member division exposure under every profile: clean
- TC-034 steps 1, 9, 10, 12, 13 and 14, against the test literals
- matrix rows `spec/exact/matrix/tests.md:36,44-48,89` and FR-007 verification notes at lines 128-131

Read as context only:

- the FR-007 Behavior text on rounding, division and `mod`, and on quantity cause order
- quire-exact 6f4cc48 `src/decimal.rs` and `src/division.rs`, and RT `src/exact/quantity.rs`
- `quire matrix --format tsv` and `quire coverage --strict` output at base 5e64b66 and at head 7afe79c

## Dispositions

Round 1 checks head a2d3fd71e8d00fc3588b4a0fddd50f4ba4c016ab against `git diff 7afe79c..a2d3fd7`.
`quire validate` passes 189 of 189. `quire matrix` tags AC-8 (tests/exact_semantics.rs:66 and :109)
and AC-15 to AC-19, coverage is 78/178, and strict unbacked is 112, the same as base. The diff changes
nothing in `src/`, Cargo or CI.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The new `tc_034_rounding_mode_spellings_and_default` checks that each mode maps to the literal spellings `exact`, `toward-zero`, `toward-positive`, `toward-negative`, `nearest-even` and `nearest-away` through an exhaustive match, and that `as_str` and `from_code` round-trip each one. A scratch kernel that renamed `toward-zero` failed this test. The matrix row and FR-007 note now name the spellings and their mapping as backed. |
| FND-002 | fixed | The test asserts `RoundingMode::default() == RoundingMode::Exact`, and a scratch kernel with `#[default]` moved to `TowardZero` failed it. FR-007:130-133 and tests.md:36 now name only the compiler-side clause as remaining: "a compiler package that omits the rounding spelling must still be tested to select `Exact`". |
| FND-003 | fixed | TC-034:31-35 now reads "RT-local `tests/exact_semantics.rs` now backs the consumed rounding behavior in AC-8, quotient-sign rounding and exact loss in AC-15, affine power refusal in AC-16, same-dimension quantity arithmetic in AC-17, Euclidean `mod` in AC-18, and division exposure in AC-19." The stale tests.md disposition rows at :137 and :184 were updated as well. |

Round 2 (head a3135e7cfa35fd99307d4bfd0da3a7c0c012baed) added no new gap findings and left no open
ones. The matrix still tags AC-8 and AC-15 to AC-19, coverage is 78/178, strict unbacked is 112, and
`quire validate` passes 189 of 189.
