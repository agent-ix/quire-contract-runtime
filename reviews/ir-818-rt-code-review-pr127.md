---
id: SR-12005
title: "IR-818 RT code review (Rust lane) of PR 127"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime#127@b68e5c6219fd9f6aca967afa707fd06f8ef4b3f0; tests/exact_quantity_conversion.rs, spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md, spec/exact/matrix/tests.md, spec/spec.md; context src/exact/quantity.rs, src/exact/unit.rs (unchanged)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
---

## Summary

Ticket: IR-818. Code review with the `rust-review` lane folded in, over `git diff origin/main...HEAD` of PR 127 (one commit, b68e5c6). The PR adds eight `/// Trace: TC-201, FR-007-AC-2x` tests on the existing `convert_quantity` + `QuantityTarget::Exact` and `evaluate_quantity`, plus status rows. `src/` is untouched (`git diff --stat origin/main...HEAD -- src` is empty), so no RT symbol is added. Oracle strength was measured with 22 reviewer mutants in a throwaway worktree on top of the coder's six: 16 killed (m20 after its compile-error first form was re-run as a variant) and 6 survived. Two of the survivors (m02, m03) are equivalent under the public meter (see Verdict). The other four are real gaps: edge order on multi-edge paths (m01, m19) and right-operand affine refusal (m14, m15).

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-007-AC-20 | examined | `convert_quantity` with `QuantityTarget::Exact` between two units of equal dimension returns a completed `Conversion` whose `value()` is the `ConvertedValue::Exact` variant holding `inverse(target_path)(source_path(v))` over exact rationals and whose `unit()` is the target unit ... and for every numerator and denominator in an `i128` sweep of scale and offset fractions the magnitude equals an independent `i128` rational oracle. |
| FR-007-AC-21 | examined | ... returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read` or other charge, leaving the meter unchanged ... |
| FR-007-AC-22 | examined | `convert_quantity` with `Exact` admits an affine source or target unit and converts it exactly ... |
| FR-007-AC-23 | examined | ... then all `unit.edge` charges for the source-to-root path followed by the root-to-target path with no shortcut, then two `unit.rational-arithmetic` events per edge, then `unit.target-domain` with no size amount, then `unit.result-retain` ... |
| FR-007-AC-24 | examined | `evaluate_quantity` `Add`, `Subtract`, `Multiply` and `Divide` of `1 degC` with `2 degC`, and `Power` ... each return `IllTyped` with cause `AffineUnitArithmetic` with zero charges ... |
| FR-007-AC-25 | examined | When a charge of `convert_quantity` with `Exact` is denied, the result is `Incomplete` carrying the denied limit kind, limit, consumed amount, next charge and charge point, with no partial quantity ... |
| tests/exact_quantity_conversion.rs | examined | eight tests, `Fraction` oracle, unit-graph fixture |
| src/exact/quantity.rs `convert`, `events`, `type_check` | context_only | unchanged code under test |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The order of edges within a multi-edge path is never pinned, so a wrong traversal order passes. Mutant m01 (drop `.rev()` on the target path in `convert`) and mutant m19 (traverse the source path root-first) both survive all eight tests. In the fixture, the only multi-edge paths are `u2` -> `u1` -> `K` (scale 1, offsets -10 then +10) and `u3` -> `degC` -> `K` (scale 1, offsets 0 then 5463/20). Both compose commutatively. No target unit has more than one edge, and the i128 sweep uses one-edge paths only. The AC-20 expression `inverse(target_path)(source_path(v))` and AC-23's "source-to-root path followed by root-to-target path" both depend on that order. Scenario: a unit `X` targets `degF` with scale 2 and offset 3. Converting `1 X` to `K` and `5463/20 K` to `X` gives a different value under either mutant, and no test would notice. Fix: add one two-edge chain whose edges do not commute (non-unit scale and nonzero offset) and convert it both to and from the root. Alternatively, extend the sweep to two-edge source and target chains. | tests/exact_quantity_conversion.rs:101-136, tests/exact_quantity_conversion.rs:367-377, src/exact/quantity.rs:328-334, src/exact/quantity.rs:511-517 |
| FND-002 | low | Affine refusal is asserted only with an affine left operand. Mutants m14 (`Multiply`/`Divide` check `a.is_affine()` only) and m15 (`Add`/`Subtract` check `a.is_affine()` only) survive. Every AC-24 vector puts the affine operand on the left (`degC op degC`, `u1 + u1`, `u3 * m`). FR-007's Behavior prose refuses affine-unit arithmetic on either operand. Scenario: `1 m * 1 degC` would evaluate instead of being refused, and `1 K + 1 degC` would return `DistinctUnits` instead of `AffineUnitArithmetic`. Fix: add those two vectors. | tests/exact_quantity_conversion.rs:655-669, src/exact/quantity.rs:363-373 |
| FND-003 | low | The sweep's doc comment is false. It says the numerators span "a value past `i32`", but the largest is `1_000_003`, which is below `i32::MAX` (2147483647). The input fractions therefore never exceed about 1e6, and only the intermediate products grow, so the AC-20 "i128 sweep" claim rests on small inputs. Fix: add at least one numerator above `i64::MAX`, for example `10^20 + 3`, and correct the comment. | tests/exact_quantity_conversion.rs:349-357 |

## Verdict

**CONDITIONAL**: one medium and two low findings. All are additive test changes, and no source change is needed. No vendoring and no copied QSpec bytes: the vectors are restated from FR-007's own ACs, and the comments paraphrase them. Fixtures are built with RT's `UnitGraph::admit`, `graph.unit` and `compound_unit`. Importing `quire_exact` directly from tests matches the ten other `tests/exact_*.rs` files. Each tag sits on a test that asserts its clause: `quire matrix` moves FR-007-AC-20..25 from untagged to tagged, and adds no untagged criterion. The `Fraction` oracle is independent of the code under test. It has its own i128 gcd and checked arithmetic and implements the spec formula `(s_s*v + o_s - o_t)/s_t`, not RT's `events`. Expected values are the spec's literals, not values derived by the code.

Gates run at b68e5c6: `cargo fmt --check` clean, `cargo clippy -p quire-contract-runtime --all-targets --all-features -- -D warnings` clean, `cargo test --features exact --test exact_quantity_conversion` 8/8 pass. `cargo deny` was not run because Cargo.toml and Cargo.lock are unchanged.

Mutants killed (reviewer set): m04 target-domain decimal-digits size, m05 identity read with no bits size, m06 and m07 edge numbering starting at 0 and at 2 (off-by-one in the tight-limit denials), m08 exact value returning the root value (wrong result unit and magnitude), m09 result unit set to the source unit, m10 reverse edge multiplying (forward/reverse confusion, including negative scales through the sweep's -3 scales and `rev`), m11 reverse edge adding the offset, m12 retain before target, m13 compound dimension check skipped, m16 retain charging 2 results, m17 target edges traversed forward, m18 affinity decided by the first edge rather than the composed offset, m20 `canonical()` returning the target value, m21 identity-read bits +1, m22 source edge count capped at 1. There is no `DecimalLoss` or rounding path on the `Exact` target (`Target::Exact` returns `ConvertedValue::Exact` directly). The non-integer expected values (5/2, 127/50, 7/3, sweep fractions) would kill any truncation.

Known limitation, judged as accepted, not a finding. Mutants m02 (target-domain sized `value_occurrences 1`) and m03 (target-domain sized `integer_bits 1`) survive. Size counters on the pinned `quire-exact` meter are high-water marks (`slot = max(slot, amount)`), and the public meter exposes only `consumed`, `admission_count` and the `ChargePoint`-only `admitted_charges`. Any integer-bits size of an exact result is at most the amount the last rational event or identity read already charged, and retain already sizes `value_occurrences` 1. Such a mutant is therefore observationally equivalent on every input through the public meter. No cheap stronger assertion exists there; `with_injected_denial` reports `work_units`, not sizes. The test does kill every size kind whose limit is 0 in the tuples (decimal digits, scale expansion; m04 killed). For the same reason, U15's third and fourth event amounts (15, 14) are only bounded by the high-water of 15. The first two are pinned by the 13- and 14-bit denials.

Mergeability: fix FND-001 in this PR before merge. FND-002 and FND-003 are cheap and belong in the same fix round.

## Dispositions

Round 1, head 2ba10e4f630e39c27bc3dd8ff23977e95e1dd1e6, verified against the fix commit and by re-running the reviewer mutants on it (9/9 tests pass; debug run 3.6 s, release 0.3 s).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The fixture now has units X (to degF, scale 2, offset 3) and W (to X, scale 3, offset -1), whose adjacent edges do not commute. New test `tc_201_exact_conversion_orders_the_edges_of_multi_edge_paths` converts X and W to and from K, X to and from W, W to U2 and W to degC, and checks `canonical()` and the value. The expected values come from the oracle's own `apply_path`/`unapply_path` over edge fractions restated literally (not read from the graph), plus the literal `1 X = 5163/20 K` and its inverse. m01 and m19 are now killed by this test; m10 and m17 are still killed. |
| FND-002 | fixed | `tc_201_affine_arithmetic_is_refused_with_zero_charges` adds `1 m * 1 degC`, `1 m / 1 degC`, `1 K + 1 degC` and `1 K - 1 degC`, each expecting `AffineUnitArithmetic` with zero charges under both limit tuples. m14 and m15 are now killed. |
| FND-003 | fixed | The sweep adds `BIG = 10^20 + 3` (above `i64::MAX`) as a scale and an offset numerator, which gives 80 edges and 80x80x3 conversions, all asserted. The doc comment is corrected, and the oracle uses checked i128 arithmetic, so an overflow would panic rather than pass. |
