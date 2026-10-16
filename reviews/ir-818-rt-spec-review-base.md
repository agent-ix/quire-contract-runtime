---
id: SR-11970
title: "IR-818 RT spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@c336b9bf9c17093ffdf8fb18f0badd7a6d51684b; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md, spec/exact/matrix/tests.md, spec/core/functional/interface-001-runtime-api.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
---

## Summary

Ticket: IR-818. PR quire-contract-runtime#126, spec-only. Reviewed the new FR-007 Behavior bullet, FR-007-AC-20 through AC-23, the verification note, TC-201, the matrix rows, the interface-001 residue entry and the spec.md range against `src/exact/quantity.rs` on RT `origin/main` (`convert_quantity`, `converts_to`, `convert`, `read_identities`, `charge_edges`, `events`, `charge_rational_target`, `charge_retain`), `rational_arithmetic_bits` and `magnitude_bits` in the locked `quire-exact`, `src/quantity.rs` in the locked `quire-semantic-value`, AD-004 steps 3 and 4, FR-275, and QSpec FR-142 and TC-187 on `quire-specification` `origin/main` (read only, nothing copied).

Verified true: the charge order (one `unit.identity-read`; all `unit.edge` charges for the source-to-root then root-to-target path before any rational event; two events per edge, multiply then add forward, subtract then divide in reverse; `unit.target-domain` with no size; `unit.result-retain`); `IncompatibleDimensions` returned before the identity read with zero charges; declared-to-declared conversion requiring one dimension node while a compound side compares maps; affine units convertible; the values for U01, U04, U22 `m2`, U25 second step and U26. Re-derived from the code's charge structure: U22 `cm2` (read 3 bits, one edge, subtract `max(3+1, 1+1)+1 = 5`, divide `bits(4)+bits(10000) = 17`, high-water `consumed 5`, `next_charge 17`, work 6); U26 (three edges, six events with peak 6 bits, work 12; second tuple denies edge 3 with consumed 2); U15 (two edges charged first, so `unit_edges` limit 1 denies edge 2). No criterion is renumbered, FR-007-AC-14 stays skipped, and the diff carries no hash, pin or copied bytes.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-007 Behavior bullet (IR-818) | examined | Exact quantity-to-quantity conversion (IR-818) is the thin function `convert_quantity_exact(source, unit, meter)` ... The function moves to `quire-semantic-value` with the `quantity` and `unit` modules in AD-004 step 4 ... and is deleted from RT in that step; it is not re-exported from RT and no second copy remains. |
| FR-007-AC-20 | examined | `convert_quantity_exact` between two units of equal dimension returns a completed `Quantity` in the target unit whose magnitude is `inverse(target_path)(source_path(v))` ... |
| FR-007-AC-21 | examined | `convert_quantity_exact` between declared units on distinct dimension nodes, or between units with different normalized base-dimension maps, returns `Err(IllTyped)` with cause `IncompatibleDimensions` before any `unit.identity-read` ... |
| FR-007-AC-22 | examined | `convert_quantity_exact` converts an affine source or target unit exactly while `Add`, `Subtract`, `Multiply`, `Divide` and `Power` on that unit still return `IllTyped` ... |
| FR-007-AC-23 | examined | `convert_quantity_exact` charges one `unit.identity-read`, then all `unit.edge` charges ... |
| FR-007 Verification notes (IR-818 paragraph) | examined | They cite QSpec `ix://agent-ix/quire-specification/FR-142-AC-1` through `FR-142-AC-5` and `FR-142-AC-10` and TC-187 rows ... |
| TC-201 | examined | Check `convert_quantity_exact`, the exact-only conversion that returns a `Quantity`, directly against the runtime. |
| interface-001 residue `quantity_residue_added_by_ir_818` | examined | quantity_residue_added_by_ir_818: [convert_quantity_exact]  # ... AD-004 step 4 moves it to quire-semantic-value with `quantity`, no second copy |
| spec/spec.md TC range | examined | TC-194–TC-195, TC-197–TC-199, planned TC-200 and planned TC-201 provide the verification matrix. |
| FR-275 residue rule | context_only | The residue in the runtime's `src/exact` is ported QSL code. It is **not authorized** ... the residue is to be deleted. |
| AD-004 step 4 | context_only | The `Value`-bearing set together: `composite`, ..., `quantity`, `unit`, `enumeration` |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The PR adds a new public item, `convert_quantity_exact`, to the RT `exact` residue, which interface-001 and FR-275 classify as "not authorized; vendored code to be deleted" (FR-275-AC-18: the residue list is empty). It grows the residue and adds new RT-local tests of residue behaviour, while the owning crate `quire-semantic-value` already exports `convert_quantity` (over `UnitQuantity`). This needs a lead or owner decision before the code slice: land the exact conversion in the owning crate and have CG consume it there, or record why RT's residue is its home. | spec/core/functional/interface-001-runtime-api.md:194, spec/exact/functional/FR-007-exact-scalar-families.md:84-101, spec/exact/functional/FR-275-single-exact-kernel.md:52-58 |
| FND-002 | medium | The note "moves to `quire-semantic-value` ... in AD-004 step 4" is not what AD-004 says. Step 4's RT change deletes RT's copies and consumers move to the shared crate's own items. The locked `quire-semantic-value` `src/quantity.rs` has `convert_quantity` and `evaluate_quantity_unit` but no `convert_quantity_exact`, and no QSL ticket is cited to add one. As written, deleting RT's copy in step 4 leaves the CG consumer with no function to call. Either name the QSL-owned ticket that adds the function to `quire-semantic-value`, or say that step 4 deletes it and CG repoints to the shared `convert_quantity` with `QuantityTarget::Exact`. | spec/exact/functional/FR-007-exact-scalar-families.md:98-101, spec/core/functional/interface-001-runtime-api.md:194, spec/assurance/AD-004-runtime-crate-layout.md:390 |
| FND-003 | low | IR-818's first acceptance criterion (ticket text, treated as data) asks for the conversion to use "the catalogued rounding mode". The spec excludes rounding ("takes no `RoundingMode`") but gives no reason. The reason exists and checks out: IR `FR-038-AC-213` (quire-contract-ir `origin/main`) admits only `{kind: rounding, value: exact}` for `quire.op.quantity.convert` with a quantity `type_argument`, and refuses `toward-zero` as `operation-mode-type-mismatch`. Cite it, so that CG's half and later readers see why no rounding parameter is needed. | spec/exact/functional/FR-007-exact-scalar-families.md:87 |
| FND-004 | medium | The QSpec citation list is wrong in both directions. It cites `FR-142-AC-3` (a lossy conversion returns its loss or refuses), which this exact-only slice excludes. It omits `FR-142-AC-7` (exact-bound accounting, and denial returning incomplete without a partial quantity), which is exactly what FR-007-AC-23 restates. The same list appears in TC-201's Description. | spec/exact/functional/FR-007-exact-scalar-families.md:150-153, spec/exact/matrix/TC-201-exact-quantity-conversion.md:14-16 |
| FND-005 | low | The name `convert_quantity_exact` is acceptable but redundant: every item here is under the `exact` feature, and `QuantityTarget::Exact` is already the selector it hard-codes. The shared crate's own pattern (`evaluate_quantity_unit`) suggests `convert_quantity_unit`. If FND-001 or FND-002 puts the function in `quire-semantic-value`, QSL chooses the name. | spec/exact/functional/FR-007-exact-scalar-families.md:85 |

## Verdict

CHANGES REQUESTED. One high, two medium, two low. The behavioural claims check out against the code: the charge schedule, refusal before any charge, declared-node versus compound-map admission, affine conversion, and every quoted value and charge tuple I re-derived (U22 `cm2`, U26, U15) match `src/exact/quantity.rs` and TC-187. The compound-target gap is stated in the open (see SR-11973 for its precision). What blocks the PR is placement and ownership: the function is added to a residue the spec itself calls unauthorized, and the plan for removing it in step 4 cites a move into a crate that does not have the function. The matrix rows, the file-to-TC row, the evidence line and the spec.md range agree with each other. `quire validate` is clean on all five files. `make spec` fails `--strict` with 120 unbacked rows, a pre-existing failure; this PR adds the four planned FR-007-AC-20..23 rows to that count.

## Dispositions

Round 1, head d1ebadd943a15db39c75a3200eb3f23cd0882728, judged against the spec text.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | No new RT symbol: interface-001 is back to origin/main (the residue key is gone) and FR-275's residue is unchanged; the criteria are pinned on the existing convert_quantity with QuantityTarget::Exact. |
| FND-002 | fixed | Step 4 now deletes RT's convert_quantity and consumers switch to the shared crate. Checked with git show at the quire-semantic-value revision in RT's Cargo.lock: src/quantity.rs exports convert_quantity(source: UnitQuantity, unit, target: &QuantityTarget, meter) -> Result<Outcome<Conversion>, IllTyped>, and its QuantityTarget has Exact. |
| FND-003 | fixed | Cites IR FR-038-AC-213, which admits only the exact mode for quire.op.quantity.convert. |
| FND-004 | fixed | FR-142-AC-3 is now excluded and FR-142-AC-7 is cited, in FR-007 and in TC-201. |
| FND-005 | fixed | No new symbol, so nothing to name: the function name is gone. |
