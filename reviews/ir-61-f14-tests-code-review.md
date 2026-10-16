---
id: SR-11600
title: "IR-61 F14 semantic tests: code review with Rust lane"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@7afe79cb9eb671d50a8d9594a0a96c3e80852327 (base 5e64b66212fb7dea5d030a5677104a9f25e6a551); tests/exact_semantics.rs; spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-61. Branch `code/ir-61-f14-semantic-tests`, no PR yet. This is a code review of
`git diff origin/main...7afe79c`, with the Rust review folded in. The diff adds six
RT-local tests to `tests/exact_semantics.rs` for FR-007-AC-8 and AC-15 to AC-19, and
changes status wording in FR-007 and the exact matrix.

## Verdict

**PASS with four low findings.** No finding blocks a merge.

Each test checks exact typed values: decimal coefficients, `DecimalLoss::exact`,
`Outcome::Completed` and `Outcome::Refused` with the full refusal payload, and
`IllTypedCause`. None asserts only `is_err`. Each test's `Trace:` tag names the
criterion it checks. No assertion sits in a guard that never runs, and no test is
tautological. The AC-8 `Exact` arm asserts and then uses `continue`, so it does run.
Every expected value comes from the kernel's public API, not from a local stub.

Mutation testing used a scratch copy of quire-exact 6f4cc48, applied with a temporary
`[patch]` in a throwaway worktree only. The RT `src/exact/quantity.rs` was mutated in that
same worktree and then reverted. Fifteen non-equivalent mutants were built, and every one
was killed by the test for its own criterion:

- TowardZero always taking the floor, or always the ceiling: killed by AC-15 and AC-8
- `mod` using the truncating remainder, and `mod` using the floor remainder: killed by AC-18
- both division members checked: killed by AC-19
- TowardNegative or TowardPositive reading the sign: killed by AC-8
- the NearestAway tie sign inverted: killed by AC-15 and AC-8
- NearestEven picking the odd neighbour: killed by AC-8
- affine `Power` accepted at exponent 0, or at negative exponents: killed by AC-16
- a fault raised on same-dimension distinct units in `Multiply`/`Divide`: killed by AC-17
- a remainder refusal naming `Quotient`: killed by AC-19
- the loss recording the rounded value instead of the exact quotient: killed by AC-15
- a charge admitted before the type check: killed by AC-16 and two older tests

One mutant survived, and it is equivalent. It takes the TowardZero sign from the floor
neighbour instead of from the exact value. For a non-integer value, the floor is negative
exactly when the value is, so no input can tell the two apart.

`cargo test --locked --features exact --test exact_semantics` passes 9 of 9. `cargo clippy
--locked --features exact --all-targets -- -D warnings` and `cargo fmt --check` are clean.
The diff has no `#[allow]` and no `unsafe`. It changes nothing in `src/`, Cargo, the
Makefile or CI, and nothing in IR-349-owned `src/exact/quantity.rs` or `src/exact/unit.rs`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The module doc comment still says the "decimal rounding ... Euclidean `mod`" cases were left to the kernel in `quire-exact`. This file now holds RT tests for rounding (AC-8, AC-15), Euclidean `mod` (AC-18) and division exposure (AC-19), so the header contradicts its own body. Failure: a reader or agent trusts the header and concludes RT has no `mod` or rounding tests. They then either duplicate them or delete them as kernel residue. | tests/exact_semantics.rs:1-5 |
| FND-002 | low | The AC-8 oracle is computed with Rust's truncating `/` (`n / 2`, `(n + 1) / 2`, `(n - 1) / 2`, and a `signum` formula for NearestAway). Truncating division is itself one of the rounding families under test. These formulas give the floor and ceiling only for odd `n`. Failure: someone adds an even numerator (an exact case, such as `2 / 2`) or another tie, and the TowardNegative oracle `(2 - 1) / 2 = 0` is wrong, so the test fails for the wrong reason or gets "fixed" to match. A literal expected table per mode would read directly against the AC-8 text. | tests/exact_semantics.rs:69-90 |
| FND-003 | low | The AC-19 test repeats four near-identical `assert_eq!(divide(profile, member, a, b, &domain, &mut Meter::new(UNLIMITED)), expected, "{profile:?}")` blocks, 60 lines in all. It rebuilds each domain inside the profile loop, and writes each refused interval twice: once through `int_domain` and again as `IntegerInterval::new(..)` in the expected refusal. AC-18 does the same. A `(member, a, b, (lo, hi), expected)` table, or a small `divide_unmetered` helper, would halve the test and keep one source for each interval. This is a style note: no wrong result follows. | tests/exact_semantics.rs:398-477 |
| FND-004 | low | The loops over `RoundingMode::ALL` and `DivisionProfile::ALL` depend on hand-written arrays in quire-exact (`decimal.rs:185`, `division.rs:30`), not on arrays tied to the enums at compile time. AC-8 is tied anyway, because its `match (mode, numerator)` is exhaustive and a new variant fails to compile. AC-19 has no such tie. Failure: the kernel adds a fourth `DivisionProfile` to the enum but not to `ALL`, and AC-19's claim "under every profile" is still reported as backed although that profile never runs. An exhaustive `match profile {..}` inside the loop would add the compile-time tie. The completeness of `ALL` itself is kernel-owned. | tests/exact_semantics.rs:417-418 |

## Coverage

Examined:

- `tests/exact_semantics.rs` at 7afe79c: the new helpers `int_domain`, `decimal_target` and `decimal_divide`, and the six new tests at lines 65-122 and 347-477
- the existing `graph()` and `quantity()` helpers the AC-16 and AC-17 tests use (lines 124-186)
- the status wording in `spec/exact/functional/FR-007-exact-scalar-families.md:128-131` and `spec/exact/matrix/tests.md:36,44-48,89`

Read as context only:

- quire-exact 6f4cc48 `src/decimal.rs` (`RoundingMode`, `round`, the division rounding path and the loss record) and `src/division.rs` (`divide`, `modulo`, `member_of`, `euclidean_remainder`)
- RT `src/exact/quantity.rs` (`evaluate_quantity`, `type_check`)
- FR-007 at base and TC-034 at base

## New findings (disposition pass 1)

These were found while checking the round 1 fixes at
a2d3fd71e8d00fc3588b4a0fddd50f4ba4c016ab (`git diff 7afe79c..a2d3fd7`).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | `assert!(profile_index < DivisionProfile::ALL.len())` can never fail. `profile_index` comes from the exhaustive match and is always 0, 1 or 2, and line 440 has already asserted that `ALL.len()` is 3. The only purpose of the binding is to make the match exhaustive, so the assert only appears to check something. Failure: a reader counts it as an index or completeness oracle. In fact the compile-time tie and the length check do that work, and this assert would pass even if both were removed. A bare `match profile { DivisionProfile::Truncating \| DivisionProfile::Floor \| DivisionProfile::Euclidean => {} }` states the intent without a vacuous assertion. | tests/exact_semantics.rs:442-447 |

## Dispositions

Round 1 checks head a2d3fd71e8d00fc3588b4a0fddd50f4ba4c016ab against `git diff 7afe79c..a2d3fd7`.
The checks ran in a throwaway worktree with a scratch quire-exact 6f4cc48 applied through a
temporary `[patch]`.

- **Tests and lint:** `exact_semantics` passes 10 of 10. Clippy with `-D warnings` and `cargo fmt --check` are clean.
- **Mutants:** all 15 original non-equivalent mutants are still killed. The equivalent TowardZero floor-sign mutant still survives, as expected.
- **New mutants:** a spelling rename (`toward-zero` to `towards-zero`) and a `#[default]` moved to `TowardZero` are both killed by `tc_034_rounding_mode_spellings_and_default`. `DivisionProfile::ALL` dropping `Euclidean` is killed by AC-19.
- **Compile-time ties:** a new `DivisionProfile::Ceiling` added to the enum, with or without `ALL`, fails to compile the RT test (E0004, non-exhaustive). A new `RoundingMode::HalfDown` fails to compile both AC-8 tests (E0004).
- **Spec tooling:** `quire validate` passes 189 of 189. `quire matrix` still tags AC-8 (at lines 66 and 109) and AC-15 to AC-19. Coverage is 78/178, and strict unbacked is 112.
- **Scope:** the diff changes nothing in `src/`, Cargo, the Makefile or CI.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The module doc now reads "FR-007 exact rounding, division, Euclidean `mod`, and quantity semantics ... IEEE exceptional, rational membership, and decimal normalization cases are kernel-owned." |
| FND-002 | fixed | The AC-8 oracle is now a literal table, one row per numerator and one column per mode, with columns chosen through an exhaustive `match mode`. I checked every value against floor, ceiling, half-even and half-away for ±1/2 and ±3/2. Any added numerator, even ones included, must now carry literal expected values, so a formula can no longer be wrong for even inputs. |
| FND-003 | fixed | AC-19's four assert blocks are now one `(member, a, b, domain, expected)` table with a single `divide` call, and every expected outcome is visible as a literal. Each refused interval is still written twice (in `int_domain(..)` and in `IntegerInterval::new(..)`), and AC-18 is unchanged. That remainder is cosmetic and is accepted as part of this fix. |
| FND-004 | fixed | AC-19 now has an exhaustive `match profile` and `assert_eq!(DivisionProfile::ALL.len(), 3)`. A variant added to the enum, whether or not it is in `ALL`, fails to compile the test (verified), and a variant dropped from `ALL` fails it at run time (verified). AC-8 asserts `RoundingMode::ALL.len() == 6`, and a new variant fails to compile (verified). |
| FND-005 | still-open | This finding is new in this round. The vacuous `assert!(profile_index < DivisionProfile::ALL.len())` at tests/exact_semantics.rs:447 has had no fix round yet. |

Round 2 checks head a3135e7cfa35fd99307d4bfd0da3a7c0c012baed against `git diff a2d3fd7..a3135e7`.
The only change is to `tests/exact_semantics.rs` (4 lines added, 6 removed).

- **Tests and lint:** `exact_semantics` passes 10 of 10. Clippy with `-D warnings` and `cargo fmt --check` are clean.
- **Compile-time tie:** a scratch `DivisionProfile::Ceiling` fails to compile the RT test with E0004 at the match on line 443, whether or not the variant is also in `ALL`.
- **AC-19 mutants:** dropping a profile from `ALL`, checking both members, and a refusal that names `Quotient` are each still killed.
- **Spec tooling:** `quire validate` passes 189 of 189. The matrix tags AC-8 and AC-15 to AC-19, and strict unbacked is 112.

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | The vacuous index binding and its assert are replaced by a bare exhaustive match with no wildcard arm and a comment stating its purpose. The rest of the AC-19 test is unchanged. |
