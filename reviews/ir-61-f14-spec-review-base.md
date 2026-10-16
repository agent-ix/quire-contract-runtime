---
id: SR-11580
title: "IR-61 F14 FR-007 rounding, division-exposure and mod correction: base spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@2cde819ac7520f31b9b43dab5b25a158edbd4691 (base 40425522c8b3f962c543969d283e54acbccf00e3); spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/tests.md; context spec/exact/matrix/TC-034-local-exact-semantics.md"
review_set: subset
---

## Summary

Ticket: IR-61. Branch spec/ir-61-f14-rounding-mod, no PR. This review covers
`git diff origin/main...2cde819`, which changes FR-007 and the exact test matrix. Every
corrected claim was checked against its upstream source. The quire-exact sources were read at
origin/main d18927e and at the locked revision 6f4cc48; `round()` and `RoundingMode` are
byte-identical at both, and `division.rs` differs only in tests. QSpec FR-140 and FR-147 were
read at origin/main.

The five corrections are right. One new criterion is broken by Markdown table syntax, and
the verifying test case TC-034 still prescribes the old pair-refusal semantics.

## Verdict

**FAIL: two high findings and one medium finding.**

Each corrected claim was verified against its source:

1. The rounding modes match quire-exact `src/decimal.rs:167-203` exactly: `RoundingMode` has the
   six variants `Exact`, `TowardZero`, `TowardPositive`, `TowardNegative`, `NearestEven` and
   `NearestAway`; `ALL` has six members; and `as_str` gives the kebab spellings used in AC-8.
   QSpec FR-140 Behavior lists the same modes. `Floor` and `Ceiling` are not rounding modes.
2. The sign dependence matches `round()` at `decimal.rs:1136-1165`. `negative` is the sign of the
   reduced exact rational's numerator. Only `TowardZero` and the `NearestAway` tie read it;
   `TowardPositive` always takes the ceiling and `TowardNegative` the floor.
3. The single-member exposure matches `division.rs:77-87` and `:137-166`, where only the exposed
   member is checked, and QSpec FR-147 Behavior and AC-9/AC-10 at origin/main.
4. The `mod` law matches `modulo()` at `division.rs:89-98`, which takes no profile, and `:181`,
   which always uses the Euclidean law. QSpec FR-147-AC-5 and TC-192 DIV-03 give the
   admission refusal as `refused { code: invalid_package, cause: incompatible-definition }`.
5. The matrix overclaim is corrected. `tests/exact_semantics.rs` contains no `mod`, division
   or decimal rounding test.

Every vector was executed in a throwaway worktree with a scratch test against the locked
kernel. The test file has been deleted. Results:

| Vector | Result |
| --- | --- |
| `(-1,0)/(-3,0)` TowardZero | coefficient 0, loss exact 1/3 |
| `(1,0)/(-3,0)` TowardZero | coefficient 0, loss exact -1/3 |
| `(-1,0)/(-2,0)` NearestAway | coefficient 1, loss exact 1/2 |
| `-7 mod 3` | 2 |
| `7 mod -3` | 1 |
| `10 div 5` in `[1,10]`, every profile | 2 |
| `-10 rem -1` in `[-10,5]`, every profile | 0 |
| `Power(1 celsius, 0)` and `Power(1 celsius, -1)` | `AffineUnitArithmetic`, no charges |
| `2 km * 3 m` | 6000, compound `{meter: 2}` |
| `2 km / 3 m` | 2000/3, empty compound |

Every result matches its criterion.

Other checks:

- AC-14 was retired in 0096d37, and AC-15 to AC-18 are first used here (`git log -S`).
  AC-8 and AC-12 keep their ids.
- `quire extract` yields clean `FR-140` and `FR-147` targets, with no target ending in
  punctuation.
- The diff does not touch `src/exact/quantity.rs` or `src/exact/unit.rs`.
- `quire validate` is clean at both base and head: 183/183 documents.
- `quire coverage --strict` goes from 112 to 113 unbacked rows. The only new row is
  `FR-007-AC-18 ... has no backing symbol [verification]`, and there is a new
  `[uncatalogued-verification-method] 'b'` diagnostic.
- `quire matrix` goes from 173 to 177 rows. The four new rows, FR-007-AC-15 to AC-18, are
  untagged. Backed rows stay at 72.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-007-AC-18 contains `0 <= r < \|b\|` unescaped inside a table cell, and the GFM pipes split the row. quire parses the criterion as ending at `0 <= r <` and the Verification cell as `b`. The `mod`-takes-no-law clause, the admission clause and `Test (TC-034)` are lost from the computed matrix. `quire coverage --strict` adds `FR-007-AC-18 has no backing symbol [verification]` and `[uncatalogued-verification-method] 'b'`. Failure: a binder tagged FR-007-AC-18 would be checked against the truncated statement, and the admission obligation would never reach the matrix. Fix: write `abs(b)`, as QSpec FR-147 does, or escape the pipes. | spec/exact/functional/FR-007-exact-scalar-families.md:117 |
| FND-002 | high | TC-034, the test case that verifies FR-007-AC-12, still says at step 10 "Refuse a quotient/remainder pair with each admitted-member combination". That is the pair-refusal semantics this change corrects; FR-147 and `division.rs` check only the exposed member. Failure: a coder who implements TC-034 as written asserts behaviour that contradicts corrected AC-12, and the test fails against the kernel or is bent to fit. | spec/exact/matrix/TC-034-local-exact-semantics.md:48 |
| FND-003 | medium | The matrix maps FR-007-AC-15 to AC-18 to TC-034, but TC-034's Description and Test Procedure have no step for quotient-sign rounding vectors, affine `Power` at exponent 0 or -1, same-dimension `Multiply`/`Divide`, or `mod` admission. Its Ownership section also omits the existing division and `mod` owner evidence, quire-exact FR-357-AC-2, AC-3 and AC-7. Failure: TC-034 can be fully implemented as written without testing any of the four new criteria. | spec/exact/matrix/TC-034-local-exact-semantics.md:13-49; spec/exact/matrix/tests.md:42-45 |

## Coverage

Examined, for the criteria, Behavior bullets and matrix rows changed by the diff:

- FR-007 Behavior, division/mod bullet (lines 31-38)
- FR-007 Behavior, rounding bullet (lines 48-55)
- FR-007-AC-8
- FR-007-AC-12
- FR-007-AC-15
- FR-007-AC-16
- FR-007-AC-17
- FR-007-AC-18
- `spec/exact/matrix/tests.md` rows 40 and 42-45, TC-034 row 88, the disposition row at 136
  and the evidence-location line

Read as context only:

- TC-034
- `src/exact/definition.rs`
- `src/exact/quantity.rs`
- `tests/exact_semantics.rs`
- quire-exact `src/decimal.rs` and `src/division.rs`, plus owner tests
  `tests/ir673_decimal.rs` and `tests/ir673_core.rs`
- QSpec FR-140, FR-147 and TC-192

## Dispositions

Round 1 checks head f7e1e02615e1069743f55801a5d3bd11f5fa2693 against `git diff 2cde819..f7e1e02`.
The diff touches only FR-007, TC-034 and `tests.md`; IR-349 code is untouched. FR-007 ids are
AC-1 to AC-13 and AC-15 to AC-19. No id was removed or renumbered, AC-14 is still unused, and
AC-19 is first used in f7e1e02.

Gate results at f7e1e02:

- `quire validate` passes: 183/183 documents.
- `quire extract` yields no target that ends in punctuation; the new FR-147-AC-5 target is
  clean.
- `quire coverage --strict` reports 114 unbacked rows (base 112, previous head 113). The
  uncatalogued-method `b` diagnostic and the AC-18 `[verification]` row are gone. Two rows are
  new: `FR-007-AC-19 [verification]` and `FR-007 (tests.md:46) [functional-coverage]`, both
  from the delegated AC-19 (see SR-11585 FND-003).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-18 now reads `0 <= r < abs(b)`. `quire matrix` parses its full statement, and AC-18's Verification cell is `Test (TC-034)`. `quire coverage --strict` no longer emits `[uncatalogued-verification-method] 'b'` or an AC-18 `[verification]` row. |
| FND-002 | fixed | TC-034 step 10 now states the single-member rule. Under every profile, `10 div 5` gives 2 in `Int[1,10]` and `-10 rem -1` gives 0 in `Int[-10,5]`. Only an out-of-domain exposed member is refused, as `DivisionOutOfDomain` naming that member. This matches `division.rs` and FR-147, and the earlier execution. |
| FND-003 | fixed | New TC-034 steps cover each new criterion: step 12 is AC-15, step 13 is AC-16, step 14 is AC-17, and step 9 is AC-18, with four quadrants and the `ModuloOutOfDomain` refusal. A note delegates AC-19. The Ownership section now cites FR-357-AC-2, AC-3 and AC-7 and FR-365-AC-1 and AC-2. Each id exists in quire-exact origin/main d18927e and is traced: `division.rs:282` and `:296`, `tests/ir673_core.rs:71`, and `tests/ir673_decimal.rs:103` and `:145`. |
