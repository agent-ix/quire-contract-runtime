---
id: SR-11973
title: "IR-818 RT gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@c336b9bf9c17093ffdf8fb18f0badd7a6d51684b; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md, src/exact/quantity.rs (origin/main)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
---

## Summary

Ticket: IR-818. Planless gap analysis of a spec-only PR. It compares what FR-007-AC-20..23 and TC-201 claim with what the code they describe (`convert_quantity` on RT `origin/main`) does, and with what QSpec TC-187 requires. Plan completion: not assessed. The matrix rows are planned: no test exists yet, and the spec says so. `convert_quantity` has no direct RT test today (its only caller is `src/exact/equality.rs`), as the verification note states.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-007 Behavior bullet, compound-target sentence | examined | A **compound unit as the conversion target** is not specified or tested by this slice (a compound *source* is); that gap is stated rather than filled, and the QSpec cases that need it (TC-187 U25's first step) are not bound here. |
| FR-007-AC-21 | examined | ... between declared units on distinct dimension nodes, or between units with different normalized base-dimension maps, returns `Err(IllTyped)` with cause `IncompatibleDimensions` ... |
| `QuantityUnit::converts_to`, `QuantityUnit::path`, `convert` | context_only | `_ => self.has_dimension_of(target)`; `Self::Compound(_) => &[]`; `unit.path().iter().rev()` |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The compound-target gap is stated honestly, but it leaves a public input with no specified behaviour rather than an unsupported one. `convert_quantity_exact` is specified as a thin wrapper over `convert_quantity`, and that function already admits a compound target: `converts_to` compares base-dimension maps when either side is compound, and a compound target's `path()` is empty, so the canonical value is returned in the compound unit with no edge. The function will therefore accept U25's first step, with no AC saying it should, and no reason is given for leaving it out. TC-187 U25 step 1 is one vector with a known result (one read, target admission, retention, no edge). Either bind it in FR-007-AC-20 and FR-007-AC-23, or specify that a compound target is refused and how. | spec/exact/functional/FR-007-exact-scalar-families.md:97-99, src/exact/quantity.rs:52-59, src/exact/quantity.rs:66-71, src/exact/quantity.rs:505-519 |
| FND-002 | low | FR-007-AC-21 names two refusal branches but binds a vector for only one. Its three vectors (`m` to `s`, the alternate `L`, `N_m` to `J`) all exercise the declared-to-declared node check. The second branch, a compound side whose base-dimension map differs (for example compound `m^2` to `m`), is untested, although it is the branch `converts_to` takes for every compound operand. | spec/exact/functional/FR-007-exact-scalar-families.md:144, src/exact/quantity.rs:52-59 |

## Verdict

CHANGES REQUESTED (one medium, one low). Every behaviour the ACs claim exists in `convert_quantity` as written: exact rational composition through the root, refusal before any charge, affine conversion, the edge-first charge schedule, `unit.target-domain` with no size, and denial through `Outcome::from_stop` with no partial value. The wrapper therefore needs no new kernel logic. The compound-target scope cut leaves the one input `convert_quantity` already admits but no AC specifies, and the compound refusal branch has no vector.
