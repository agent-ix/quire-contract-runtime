---
id: SR-11581
title: "IR-61 F14 FR-007 correction: integrity analysis"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@2cde819ac7520f31b9b43dab5b25a158edbd4691 (base 40425522c8b3f962c543969d283e54acbccf00e3); spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-61. This analysis checks the completeness, consistency and atomicity of the
rewritten FR-007-AC-8 and AC-12, the new AC-15 to AC-18, and the rewritten Behavior bullets,
against the base text and the upstream sources.

## Verdict

**FAIL: two medium findings and two low findings.**

The corrections are consistent with quire-exact and QSpec. AC-8 and AC-12 keep their ids, and
no retired id is reused.

The rewrite of AC-8 narrows its obligation. AC-12 still bundles three behaviours, so the
computed matrix shows it as tagged by tests that check only one of them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Base AC-8 required that "the six rounding spellings round every exact tie to the stated neighbour for both signs". Rewritten AC-8 constrains ties for `NearestEven` and `NearestAway` only, and AC-15 adds `TowardZero` and `NearestAway` sign vectors. No FR-007 criterion now checks `TowardPositive` or `TowardNegative`, although Behavior lines 54-55 newly assert that they are sign-independent. Failure: an RT binding that sends `TowardNegative` to the ceiling for a negative value satisfies every FR-007 criterion. Owner test FR-365-AC-1 (`tests/ir673_decimal.rs:105`) already covers all six modes on positive and negative ties. Fix: keep AC-8's all-six-modes tie obligation and state each mode's neighbour. | spec/exact/functional/FR-007-exact-scalar-families.md:108; :48-55 |
| FND-002 | medium | AC-12 remains compound: `mod`, `div`/`rem` single-member exposure, and quantity IllTyped order. Its `mod` clause duplicates AC-18. The three tests tagged FR-007-AC-12 (`tests/exact_semantics.rs:106,151,225`) exercise only the quantity clause, but `quire matrix` computes AC-12 as `tagged`. Only the hand-written row at tests.md:40 admits that the division and `mod` clauses are untested. Failure: a computed-matrix consumer reads AC-12, including division exposure, as backed when no RT test checks it. Fix: move the division and `mod` clauses into their own criteria, as AC-15 to AC-18 do, and leave AC-12 as the quantity clause. | spec/exact/functional/FR-007-exact-scalar-families.md:112; spec/exact/matrix/tests.md:40 |
| FND-003 | low | AC-15 to AC-18 embed authoring instructions in normative criteria, such as "These vectors are derived from the kernel; confirm each result by execution and correct this criterion if a result differs, rather than changing the test to fit it". That is process guidance, not an acceptance condition, and it becomes stale once a test binds. Every vector was executed for this review and matches, so the hedge can go. | spec/exact/functional/FR-007-exact-scalar-families.md:114-117 |
| FND-004 | low | The Behavior text says `TowardZero` and the `NearestAway` tie "use the sign of the exact quotient, not the sign of the retained coefficient". Rounding also applies to `Add`, `Multiply` and `Round` when the result's scale exceeds the target scale. There the rounded value is `v/10^k`, or the `sign(v)/10` proxy at `decimal.rs:1068-1074`, not a quotient. "Retained coefficient" is also undefined here: which operand's coefficient, or the result's? Fix: write "the sign of the exact unrounded result". | spec/exact/functional/FR-007-exact-scalar-families.md:52-53 |

## Coverage

Examined:

- FR-007 Behavior lines 31-38 and 48-55
- FR-007-AC-8, AC-12, AC-15, AC-16, AC-17 and AC-18
- matrix rows 40 and 42-45

Read as context only:

- base FR-007-AC-8 and AC-12 text
- `tests/exact_semantics.rs`
- quire-exact `decimal.rs` and `tests/ir673_decimal.rs`

## New findings (disposition pass 1)

These were found while checking the round 1 fixes at
f7e1e02615e1069743f55801a5d3bd11f5fa2693.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | Rewritten AC-8 says `TowardZero` "selects the zero neighbour" on positive and negative exact ties, and TC-034 step 1 repeats the phrase. Only the ±1/2 tie has zero as a neighbour. For the 3/2 tie the neighbours are 1 and 2, and `round()` gives 1, the neighbour nearer zero. As written the phrase is undefined for every other tie. Fix: "the neighbour nearer zero". | spec/exact/functional/FR-007-exact-scalar-families.md:112; spec/exact/matrix/TC-034-local-exact-semantics.md:37-39 |

## Dispositions

Round 1 checks head f7e1e02615e1069743f55801a5d3bd11f5fa2693 against `git diff 2cde819..f7e1e02`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-8 again constrains all six spellings on positive and negative exact ties: NearestEven even, NearestAway greater magnitude, TowardZero the zero neighbour, TowardPositive ceiling, TowardNegative floor, and omitted Exact refuses. This matches `round()` and owner FR-365-AC-1. The "zero neighbour" wording is recorded as new FND-005. |
| FND-002 | still-open | The `mod` duplication is fixed: AC-12 drops its `mod` clause and points to AC-18. The hand-written rows at `tests.md:40` and `:89` now say the tagged tests cover only the quantity clause. AC-12 still bundles the division-exposure clause with the quantity clause, though, and `quire matrix` at f7e1e02 still computes AC-12 as `tagged` from the three quantity-only tests (`tests/exact_semantics.rs:106,151,225`). Tooling that consumes the computed matrix therefore still reads division exposure as backed. Remaining fix: move the `div`/`rem` single-member clause to its own new criterion, and leave AC-12, with its id kept, as the quantity clause. |
| FND-003 | fixed | AC-15 to AC-18 no longer carry authoring directives. The guidance moved to a non-criterion `## Verification notes` section. |
| FND-004 | fixed | The Behavior text now says "the sign of the exact value being rounded (for division, the exact quotient)". It defines the retained coefficient as "the integer coefficient of the result at the target scale". Both match `round()` and `in_target_units`, including the scaled-intermediate path. |
| FND-002 | fixed | Round 2, checked at 71f0cb7b69f4690acda358cb2860890cd112da6d. AC-12, with its id kept, is now only the quantity clause: "Quantity `IllTyped` causes appear in the stated per-operation order with zero charges, and `Multiply`/`Divide`/`Power` raise no dimension fault". `quire matrix` computes it as `tagged` by `tests/exact_semantics.rs:106,151,225`, which test Add/Subtract cause order; cross-dimension Multiply/Divide/Power; and zero power plus affine refusal with zero charges. So the computed status now matches what those tests cover. The division clause moved to FR-007-AC-19, which `quire matrix` shows as untagged and the matrix row marks 🚧 planned. `git log -S 'FR-007-AC-19' origin/main` is empty, so on main the id is new, not reused. The `mod` clause is in AC-18. One old AC-12 obligation was narrowed: the out-of-domain remainder refusal. That is recorded as new FND-006. |
| FND-005 | fixed | Round 2, checked at 71f0cb7. AC-8 and TC-034 step 1 now both say "the neighbour nearer zero", which matches `round()` for every tie. The vectors are unchanged. |
| FND-006 | fixed | Round 3, checked at eb97f99aa8a20cb7498da755bd6723603b6b0153 against `git diff 71f0cb7..eb97f99`, which changes only FR-007-AC-19, TC-034 step 10 and the AC-19 matrix row. Both AC-19 and step 10 add "`9 rem 10` in `Int[0,5]` refuses without a value as `DivisionOutOfDomain` naming the Remainder member although its quotient is `0`". I ran it in a scratch test, now deleted, against the locked kernel 6f4cc48 under all three profiles. Truncating, Floor and Euclidean each return `Refused(DivisionOutOfDomain { domain: [0,5], member: Remainder })`, and `9 div 10` in `Int[0,5]` completes with `0` under each. The member spelling matches the kernel's `DivisionMember::Remainder`. The quotient vector and its Quotient naming are unchanged; `-10 div -1` in `Int[-10,5]` is refused naming `Quotient` under all three profiles. The matrix row now lists the same four vectors. `quire validate` passes 183/183, `quire matrix` keeps the same rows and statuses as 71f0cb7, and `--strict` still reports 112 unbacked rows. The only other changed text is a non-normative execution note in TC-034 step 10. |

## New findings (disposition pass 2)

Found while checking the round 2 fix at 71f0cb7b69f4690acda358cb2860890cd112da6d.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | Moving the division clause into FR-007-AC-19 dropped the remainder-refusal obligation. Base AC-12, and f7e1e02 AC-12 and TC-034 step 10, required that "an out-of-domain quotient or remainder instead yields `DivisionOutOfDomain` naming that exposed member". AC-19 and TC-034 step 10 now give only the quotient refusal (`-10 div -1` in `Int[-10,5]` names Quotient). No FR-007 criterion or TC-034 step now requires `rem` to refuse an out-of-domain remainder naming the Remainder member. Failure: an RT binding that reports every division refusal as `member: Quotient`, or that lets an out-of-domain remainder through, satisfies FR-007. Owner FR-357-AC-3 tests it, but RT no longer states it. Fix: add a profile-independent remainder vector to AC-19 and TC-034 step 10, for example `9 rem 10` in `Int[0,5]` gives remainder 9 under every profile and refuses as `DivisionOutOfDomain` naming Remainder. | spec/exact/functional/FR-007-exact-scalar-families.md:124; spec/exact/matrix/TC-034-local-exact-semantics.md:58-61 |
