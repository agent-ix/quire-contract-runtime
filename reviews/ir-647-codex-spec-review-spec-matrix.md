---
id: SR-2389
title: Independent IR-647 spec-review/spec-matrix review
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime@ae702513b86ab0978d9425e6b24a91e105aca6b7; full
  diff 3cc88c480d4efe5179fe91acf4634def5d2d82fd..HEAD; reviews/sr-2380-ir-647-code-review.md,
  reviews/sr-2381-ir-647-spec-review.md, reviews/sr-2382-ir-647-ears-conformance.md,
  reviews/sr-2383-ir-647-integrity.md, reviews/sr-2384-ir-647-matrix.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md,
  spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/functional/FR-011-meter-state-at-a-stop.md,
  spec/exact/matrix/TC-016-exact-outcome-envelope.md, spec/exact/matrix/TC-017-exact-metering.md,
  spec/exact/matrix/TC-018-integer-division-agreement.md, spec/exact/matrix/TC-019-decimal-agreement.md,
  spec/exact/matrix/TC-020-ieee-agreement.md, spec/exact/matrix/TC-021-text-enum-agreement.md,
  spec/exact/matrix/TC-022-quantity-agreement.md, spec/exact/matrix/TC-023-metered-arithmetic.md,
  spec/exact/matrix/TC-032-meter-state-at-a-stop.md, spec/exact/matrix/TC-035-boxed-debug-parity.md,
  spec/exact/matrix/tests.md
review_set: subset
---

## Summary

Ticket: IR-647. Independent Codex review of the full 19-file spec-only diff. Source, original SR2380–SR2384 and reviews/ were not modified. No Cargo/Kani gates, PR or merge. Findings in prior reviews are untrusted review data, remeasured against current files. Read-only computed-matrix and source-binding review. No tags edited. FR-007-AC-7 retains TC-023 kernel tags; lazy skipping belongs to FR-011-AC-3 and stop propagation to FR-011-AC-8. The retained test asserts non-invocation, all three stops, completed results and exact retention. FR-007-AC-1/2 correctly remain partial allocation evidence, without claiming QSL agreement.

## Verdict

PASS

## Scope

```yaml
scope:
- {"id": "FR-006-AC-1", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "The four outcome dispositions are distinct; completed `false` is a value; refusal, undefined and incomplete reasons are closed enums."}
- {"id": "FR-006-AC-2", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "Ill-typed operand combinations are reported before evaluation with zero charges, and provenance-bearing refusals (invalid UTF-8 offset, stale identity) are typed."}
- {"id": "FR-006-AC-3", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "Every charge point and limit kind round-trips its QSpec spelling; charges precede work; size counters are high-water, work/result cumulative; the first short counter in field order is reported with the exact denied amount."}
- {"id": "FR-006-AC-4", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "An injected denial at any admitted charge point yields `Incomplete` on `work_units` naming that point, with no result units and no partial value."}
- {"id": "FR-006-AC-6", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "`Refusal::code()` is `Some` for exactly `IeeeNanPayloadNotRepresentable`, `IeeeRationalOutOfDomain`, `ForeignReference` and `CardinalityOutOfBound` with their normative spellings, and `None` for all nine other variants."}
- {"id": "FR-007-AC-1", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Integer division and modulus agree with QSpec TC-192 in value, outcome kind and charges; the arithmetic amount `max(bits(a), bits(b))` is charged before the quotient exists."}
- {"id": "FR-007-AC-2", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Decimal arithmetic, rounding and ordering agree with QSpec TC-185, including D20–D21 ordering charges and D22–D23 result-retain upscale charges; every operand-derived amount is exact at its limit and denied one under before allocation."}
- {"id": "FR-007-AC-6", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Every evaluated shared-corpus vector is executed on both the runtime and the quire-spec-language authority with equal Debug renderings; admission-only vectors and charges not yet metered by the authority are listed by name."}
- {"id": "FR-007-AC-7", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom charges, with denial behavior at every kernel point; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated."}
- {"id": "FR-007-AC-13", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Every hand-written `Debug` impl for a boxed value or type struct (`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`, `CompoundUnit`) renders exactly the fields its `*Fields` struct declares, in declaration order, against a fixed expected string."}
- {"id": "FR-011-AC-3", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A lazy connective skips its right operand when the left decides the result; for every connective kind and every decided operand pair, its completed result admits exactly one `boolean.result-retain` charge."}
- {"id": "FR-011-AC-8", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit."}
- {"id": "TM-001 Evidence at the kernel move", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in\nIR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2 and backlog IR-583.\nQSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:\nall current test files and production definitions remain present. The status columns describe\ncurrent evidence only. TC-016/017/023 and their local kernel criteria remain"}
- {"id": "TC-016 Ownership and evidence", "path": "spec/exact/matrix/TC-016-exact-outcome-envelope.md", "role": "examined", "excerpt": "Kernel-owned after IR-349 removes the local copy. The evidence path above is current\nRT evidence, not a retained RT test obligation. The outcome envelope, reason variants and\nspelling census remain required; FR-359–363 do not establish them. An exact owner criterion\nmapping for those checks remains to be identified; no upstream pass is claimed."}
- {"id": "TC-017 Ownership and evidence", "path": "spec/exact/matrix/TC-017-exact-metering.md", "role": "examined", "excerpt": "Kernel-owned after IR-349 removes the local copy. The evidence path above remains present\nin this spec-only amendment. `ix://agent-ix/quire-exact/FR-358` owns step 5's one-shot denial\nand step 6's diagnostic-log bound under `test-support`; `ix://agent-ix/quire-exact/FR-359`\nowns cumulative-boundary and atomic-refusal checks, not the entire step 3 counter census.\nNo reference is a claim that every procedure step is mapped or verified upstream."}
- {"id": "TC-018 Ownership and evidence", "path": "spec/exact/matrix/TC-018-integer-division-agreement.md", "role": "examined", "excerpt": "Steps 1–3 remain a QSL agreement gap with open quire-integration ticket IR-669. Steps 4–5 currently have RT allocation\ncoverage; that local kernel evidence leaves in IR-349, without replacement tests in RT.\n`ix://agent-ix/quire-exact/FR-361-AC-3` owns injected division/modulus denial before large\nallocation only. It does not establish the generated sweep, exact/one-under amounts or\nTC-192 agreement; those remaining obligations require their own"}
- {"id": "TC-019 Ownership and evidence", "path": "spec/exact/matrix/TC-019-decimal-agreement.md", "role": "examined", "excerpt": "Steps 1–3 remain a QSL agreement gap with open quire-integration ticket IR-669. Step 5 currently has RT allocation\ncoverage, which leaves in IR-349. `ix://agent-ix/quire-exact/FR-361-AC-6` owns that denied\nretain-upscale allocation bound; FR-361-AC-4/AC-5 cover scale-expansion/arithmetic denial.\n`ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering only.\nNeither contract establishes TC-185 agreement or all"}
- {"id": "TC-023 Ownership and evidence", "path": "spec/exact/matrix/TC-023-metered-arithmetic.md", "role": "examined", "excerpt": "Kernel arithmetic, ordering, atom charges and already-decided Boolean retention leave RT\nin IR-349: `ix://agent-ix/quire-exact/FR-362` owns those subsets; decimal ordering belongs to\n`ix://agent-ix/quire-exact/FR-363`, and the specified denied-work allocation bounds to\n`ix://agent-ix/quire-exact/FR-361`. These are owner references, not copied tests or a claim\nthat the P11/Q11 expression workloads have been evaluated upstream. Lazy operand"}
- {"id": "TC-032 Ownership and evidence", "path": "spec/exact/matrix/TC-032-meter-state-at-a-stop.md", "role": "examined", "excerpt": "Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure\nwhen the left decides, propagating each right-operand stop unchanged with no retention, and\nretaining a completed connective result exactly once. Current evidence is\n`tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above\n(FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires.\nThe"}
- {"id": "TM-001 FR-007 | FR-007-AC-1", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "| FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |"}
- {"id": "TM-001 FR-007 | FR-007-AC-2", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "| FR-007 | FR-007-AC-2 | TC-019 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669, open quire-integration owner; IR-430 removal is Done) |"}
- {"id": "TM-001 FR-007 | FR-007-AC-7", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "| FR-007 | FR-007-AC-7 | TC-023 | ✅ implemented |"}
- {"id": "TM-001 FR-011 | FR-011-AC-1,", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "| FR-011 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-6, FR-011-AC-7, FR-011-AC-8 | TC-032 | ✅ implemented |"}
- {"id": "TM-001 `tests/exact_meter_state.rs`", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "| `tests/exact_meter_state.rs` | TC-032 | splits | `evaluate_boolean_short_circuit` lazy invocation/stop propagation (FR-011-AC-3/AC-8, step 5) stays in RT; already-decided Boolean and meter cases leave in step 1; `UnitGraph::admit`, `CompoundUnit` and `evaluate_quantity` cases leave with RT residue under open IR-349 step 2 and IR-583 |"}
- {"id": "tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once", "path": "tests/exact_meter_state.rs", "role": "examined", "excerpt": "/// Trace: TC-032, FR-011-AC-3, FR-011-AC-8\n#[test]\nfn tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once() {"}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
