---
id: SR-11561
title: "IR-61 residue slice criterion-to-test gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@170c5c15e1b94fff2e1334120b77919b10f0b174 (base 0fcccfc70f58407e1a25c041a0f6b8639f94911f); tests/integration.rs; tests/exact_vocabulary.rs; tests/exact_meter_state.rs; src/exact/unit.rs; src/accounting.rs; spec/accounting/functional/FR-004-campaign-accounting.md; spec/exact/functional/FR-008-composite-collection-and-equality.md; spec/exact/functional/FR-011-meter-state-at-a-stop.md; spec/exact/functional/FR-012-carried-compiler-vocabulary.md"
review_set: subset
---

## Summary

Ticket: IR-61. Branch code/ir-61-residue-slice, no PR yet. This is a diff-scoped criterion-to-test
and test-oracle-strength review of the residue slice for IR-61 F12, F15 and F16 and the
FR-004-AC-8 nit. Plan completion: not assessed.

## Verdict

**PASS with one medium finding and three low findings.**

FR-004-AC-8 is now a falsifiable control. A mismatch that clears nonzero prior counters fails the
test. On main that mutant passed. Both the wrong-requirement and the wrong-revision paths are
discriminated.

The F12 `assert_ne!` controls compare genuinely distinct values. With an always-true `PartialEq`
on `Dimension`, the test fails at `tests/exact_meter_state.rs:302` and
`tests/exact_vocabulary.rs:506`. With an always-true `PartialEq` on `CompoundUnit`, it fails at
`tests/exact_vocabulary.rs:543`. None of the three controls repeats an existing assertion.

However, the FR-012-AC-5 "exactly when" clause is still open to an exponent-blind equality mutant
(FND-001).

Matrix: identical row count (147) at base and head. The FR-004-AC-8 row is tagged at
`tests/integration.rs:156`. `quire coverage --strict` reports 112 unbacked rows at both base and
head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-012-AC-5 requires `Dimension` and `CompoundUnit` to "compare equal exactly when they denote the same unit". The new `assert_ne!` controls use values whose key sets differ: `low^2*high^3` against `low^2`, and a two-term unit against dimensionless. A `PartialEq` that compares only the node keys and ignores exponents therefore passes the whole `--all-features` suite (all 18 test binaries). For example, `m*s` would compare equal to `m^2*s^3`. Add a control with equal keys and different exponents, such as `low^2*high^3` against `low^3*high^2`, for both types. | tests/exact_vocabulary.rs:506,543 |
| FND-002 | low | `assert_ne!(d1, d2)` carries `Trace: TC-032, FR-011-AC-7`, but FR-011-AC-7 states only an iteration order and makes no equality claim. The `terms == [k1, k2, k3]` assertion at :310 already decides the iteration claim. The only oracle this line adds is `PartialEq` discrimination, which FR-012-AC-5 owns. The line is a harmless precondition check, but it does not strengthen its traced criterion. | tests/exact_meter_state.rs:302 |
| FND-003 | low | F12 named `assert_eq!(CompoundUnit::dimensionless(), CompoundUnit::default())` as a literal tautology, and it remains. `dimensionless()` is `Self::default()` at `src/exact/unit.rs:452-454`, so the assertion cannot fail. This slice does not fully close F12: delete the line, or replace it with an assertion that can fail. | tests/exact_vocabulary.rs:553 |
| FND-004 | low | F16-class residue remains in a file this slice edited. FR-008 still cites kernel requirements as bare ids qualified only by prose: `kernel FR-369` at :131 and `Quire-exact FR-368-AC-1/2` at :175. Other references in the same spec set use `ix://agent-ix/quire-exact/...`. RT has no local FR-368 or FR-369 today, so nothing resolves wrongly now. A future local mint would repeat the F15 FR-009 ambiguity. | spec/exact/functional/FR-008-composite-collection-and-equality.md:131,175 |

## New findings (disposition pass 1)

These findings come from checking fix head 129c9d6ee790321368052e8d4f003c64bfb5a9fa.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The new same-key controls change both exponents at once: `low^2*high^3` against `low^3*high^2`, and units `[2,3]` against `[3,2]`. A `PartialEq` that ignores the exponent of only one term still passes all 18 test binaries under `--all-features`. This was reproduced for `Dimension` with the first term ignored and with the last term ignored, and for `CompoundUnit` with the first term ignored. Such a mutant would make `m^2*s^3` equal `m^5*s^3`. Add one control per type that differs in exactly one exponent, for example `low^2*high^3` against `low^3*high^3`. | tests/exact_vocabulary.rs:507,546 |
| FND-006 | low | The FND-004 fix writes `from kernel ix://agent-ix/quire-exact/FR-369.` at the end of a sentence. `quire extract` harvests a second `references` edge with target `FR-369.`, including the period, which does not exist, beside the correct `FR-369` edge. Wrap the link in backticks or angle brackets. A local probe shows either form yields a clean `FR-369` target. | spec/exact/functional/FR-008-composite-collection-and-equality.md:131 |

## Dispositions

Round 1 checks fix head 129c9d6ee790321368052e8d4f003c64bfb5a9fa against
`git diff 170c5c1..129c9d6`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `tests/exact_vocabulary.rs:507` adds `assert_ne!(forward, low.power(&int(3)).multiply(&high.power(&int(2))));`. `:546` adds `assert_ne!(build([1, 2], [1, 2], [2, 3]), build([2, 1], [2, 1], [3, 2]));`. The fully exponent-blind (keys-only) `PartialEq` mutant is now killed at `:507` for `Dimension` and at `:546` for `CompoundUnit`. Mutating `Dimension` alone is also killed at `:507`. The one-term residual is recorded as FND-005. |
| FND-002 | fixed | `assert_ne!(d1, d2)` was removed from `tc_032_ac7_dimension_terms_ascend_by_node_key_for_any_construction_order`. Its remaining assertions check term iteration order, which FR-011-AC-7 claims. FR-012-AC-5 equality remains verified by the two `tc_033` tests in `tests/exact_vocabulary.rs`. |
| FND-003 | fixed | The `dimensionless() == default()` tautology was deleted. It had no oracle, so nothing was lost. `assert!(CompoundUnit::dimensionless().terms().next().is_none())` still checks that the dimensionless unit has no terms, and `assert_ne!(forward, CompoundUnit::dimensionless())` checks that it differs from a real unit. |
| FND-004 | fixed | Line 131 now reads `from kernel ix://agent-ix/quire-exact/FR-369`. The FR-008-AC-5 cell reads `its ix://agent-ix/quire-exact/FR-369 cause`. Line 175 reads `Quire-exact (ix://agent-ix/quire-exact/FR-368-AC-1 and ix://agent-ix/quire-exact/FR-368-AC-2)`. Only spelling changed. All three resolve at quire-exact origin/main: FR-368 is the meter charge and limit vocabulary, with AC-1 for ChargePoint and AC-2 for LimitKind, and FR-369 is the typed checked-invariant causes. The trailing-period regression is recorded as FND-006. |
| FND-005 | fixed | Round 2, head 09a37a09462af03878ccb45711f0700f248f73b2. Single-exponent controls were added at `tests/exact_vocabulary.rs:508-509` (`low^3*high^3` and `low^2*high^2` against `low^2*high^3`) and `:549-550` (units `[3,3]` and `[2,2]` against `[2,3]`). Each one-term exponent-blind mutant now fails at its intended line: Dimension ignoring the first term at `:508`, Dimension ignoring the last term at `:509`, CompoundUnit ignoring the first term at `:549`, and CompoundUnit ignoring the last term at `:550`. Both fixtures have two terms, so there is no middle term to mutate. |
| FND-006 | fixed | Round 2. Line 131 now reads ``from kernel `ix://agent-ix/quire-exact/FR-369`.``. `quire extract` on FR-008 yields exactly one `FR-369` target, with no target that ends in punctuation and no backtick in any target. The `FR-368-AC-1` and `FR-368-AC-2` edges are still present. |

## Coverage

- Examined FR-004-AC-8: "A verdict whose requirement or revision differs from the report's
  increments no counter, leaves a prior snapshot unchanged, and returns an `IdentityMismatch`
  carrying both identities; a report that recorded a mismatch has the same counters as one that
  never saw the verdict."
- Examined FR-012-AC-5: "`Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending
  by node key, and compare equal exactly when they denote the same unit, for every construction
  order of the same terms."
- Examined FR-011-AC-7, the dimension and compound-unit term-iteration clause.
- Mutants run in a throwaway worktree:
  - Clearing counters on mismatch: killed.
  - Clearing counters on a revision-only mismatch: killed.
  - Counting the refused verdict: killed.
  - `snapshot()` returning zero counts: survives `tests/integration.rs`, killed by `tc_015`
    (SR-11560 FND-001).
  - Always-true `Dimension` and `CompoundUnit` equality: killed.
  - Exponent-blind equality on both types: survives the whole suite.
- Bindings checked: `tc_006_mismatched_identity_leaves_every_counter_and_snapshot_untouched` binds
  FR-004-AC-8 correctly. `tc_033_dimension_multiply_is_order_independent_ascending_and_zero_free`
  and `tc_033_compound_unit_is_independent_of_admission_order_and_ascending` bind FR-012-AC-5
  correctly. `tc_032_ac7_dimension_terms_ascend_by_node_key_for_any_construction_order` binds
  FR-011-AC-7 correctly for its iteration assertions.
