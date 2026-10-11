---
id: SR-7160
title: "Spec review of IR-750 decimal ordering charge correction"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@bf693a903a965d08d9443e06b6183be49c57b5cc; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-034-local-exact-semantics.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-750. Reviewed the changed FR-007 decimal behavior and AC-11, TC-034 ownership and procedure, and matrix row against quire-exact origin/main FR-363-AC-6/7 and its traced decimal test. The exact values, charge ordering, and planned RT binder status agree; no defect found.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-007 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | *Ordering size charges* use the retained operand representations, never their normalized forms, so an equal-value pair can have different ordering size counters. The Boolean `ordering.result-retain` charge remains one result unit for either pair. |
| FR-007-AC-11 | examined | spec/exact/functional/FR-007-exact-scalar-families.md | Retained decimals `(110, 2)` and `(11, 1)` compare equal in value and both normalize to `(11, 1)` without changing their retained representations. On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both return false; their ordering size counters (`integer_bits`, `decimal_digits`, `scale_expansion`) are respectively `8/3/1` and `4/2/0`, while both comparisons consume `value_occurrences/work_units/result_units = 2/3/1`, including one Boolean `ordering.result-retain` result unit. |
| FR-007 kernel ownership | examined | spec/exact/functional/FR-007-exact-scalar-families.md | For FR-007-AC-11, `ix://agent-ix/quire-exact/FR-363-AC-6` and `FR-363-AC-7` specify the normalized-value and retained-operand-charge distinction; the kernel's `tests/ir673_decimal.rs` tests the equal values, retained representations and exact `8/3/1` versus `4/2/0` size counters with one result unit in each comparison. The RT exact-feature obligation remains planned in TC-034 until RT-local evidence binds it. |
| TC-034 ownership | examined | spec/exact/matrix/TC-034-local-exact-semantics.md | `FR-363-AC-6` and `FR-363-AC-7`, traced by kernel `tests/ir673_decimal.rs`, test decimal normalized values versus retained ordering size charges and the unchanged Boolean result unit. |
| TC-034 procedure 8 | examined | spec/exact/matrix/TC-034-local-exact-semantics.md | Compare retained `(110, 2)` against `(11, 1)` and order each of `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` on a fresh meter. Check equal normalized values and false ordering; check `integer_bits/decimal_digits/scale_expansion` of `8/3/1` versus `4/2/0`, with `value_occurrences/work_units/result_units = 2/3/1` for both comparisons. The Boolean `ordering.result-retain` point consumes one result unit in each case. |
| FR-007-AC-11 matrix row | examined | spec/exact/matrix/tests.md | 🚧 owner-backed by quire-exact FR-363-AC-6/7 and `tests/ir673_decimal.rs`: equal normalized values, ordering size counters 8/3/1 versus 4/2/0, and one Boolean result unit in both comparisons; RT exact-feature binder remains planned after the local kernel test was removed |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the scoped spec correction. The revised criterion is falsifiable: it names both concrete comparisons, their Boolean results, six meter counters, and retained normalization behavior. Quire validated all three changed files; no Rust, Cargo, Kani, or full CI gate was run in this review.
