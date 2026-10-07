---
id: SR-2391
title: Independent IR-647 evidence review
type: SpecReview
analysis: evidence
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

Ticket: IR-647. quoin advise --repo . --json exits 0. All 26 FR-006/FR-007/FR-011 obligations have authored Test methods matching at least one catalog recommendation; zero scoped mismatches, zero inconclusive. Example, invariant, round-trip and metamorphic shapes recommend unit/property or related Test-class methods; no fault-detection evidence escalation was inferred. Current local tests and future kernel evidence are distinct; an ownership link or static tag is not discharge. Method selection is PASS; substantive census/status findings remain in SR-2386/SR-2388. Initial quoin advise . --json exited 3 because this command takes --repo, not a positional repository; the documented invocation succeeds.

## Verdict

PASS for this method; overall candidate FAIL on SR-2386/SR-2388, with SR-2385 hygiene.

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
- {"id": "FR-007-AC-3", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "IEEE profile operations agree with QSpec TC-193."}
- {"id": "FR-007-AC-4", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Text and enum operations agree with QSpec TC-186."}
- {"id": "FR-007-AC-5", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Quantity and unit-graph operations agree with QSpec TC-187."}
- {"id": "FR-007-AC-6", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Every evaluated shared-corpus vector is executed on both the runtime and the quire-spec-language authority with equal Debug renderings; admission-only vectors and charges not yet metered by the authority are listed by name."}
- {"id": "FR-007-AC-7", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom charges, with denial behavior at every kernel point; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated."}
- {"id": "FR-007-AC-8", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "The six rounding spellings round every exact tie to the stated neighbour for both signs; an omitted spelling is `Exact` and refuses a discarded nonzero digit."}
- {"id": "FR-007-AC-9", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "IEEE NaN propagation is leftmost-wins with sign and payload preserved and the result quieted; `invalid` is raised when any operand is signaling; an unrepresentable NaN payload is refused, never truncated; `-0.0` and `+0.0` convert to the same exact value with `discarded_negative_zero` reported; `total_order_key` totally orders every bit pattern including both zeros and NaNs."}
- {"id": "FR-007-AC-10", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "`Rational` membership admits exactly the reduced pairs inside both intervals — including refusing a value whose numeric magnitude is inside the numerator interval but whose reduced denominator is outside the denominator interval — an absent domain decides no membership and retains, and every exposed rational is in canonical form with zero as `0/1`."}
- {"id": "FR-007-AC-11", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "`Decimal` value comparison is on the normalized representation and charges are sized on the retained one, demonstrated by a pair equal in value whose ordering and retain charges differ."}
- {"id": "FR-007-AC-12", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "`mod` returns the Euclidean remainder for every operand sign whatever `div`/`rem` law is selected; a quotient/remainder pair outside the consumer domain is refused as a pair naming which members were admitted, exposing neither; quantity `IllTyped` causes appear in the stated per-operation order with zero charges, and `Multiply`/`Divide`/`Power` raise no dimension fault."}
- {"id": "FR-007-AC-13", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Every hand-written `Debug` impl for a boxed value or type struct (`Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`, `CompoundUnit`) renders exactly the fields its `*Fields` struct declares, in declaration order, against a fixed expected string."}
- {"id": "FR-011-AC-1", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "For each of `Undefined`, `Refused` and `Incomplete`, the meter after the stop holds exactly the charges admitted before it: an `Undefined` division by zero retains the operands charge and no arithmetic charge; a refused result retains the arithmetic charge and no result unit; a denied charge retains neither."}
- {"id": "FR-011-AC-2", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A quantity `power` with zero base and negative exponent is `Undefined::DivisionByZero`, and a divide by zero is reported in preference to it when both hold."}
- {"id": "FR-011-AC-3", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A lazy connective skips its right operand when the left decides the result; for every connective kind and every decided operand pair, its completed result admits exactly one `boolean.result-retain` charge."}
- {"id": "FR-011-AC-4", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A charge whose second-scanned counter is short writes no counter, appends no log entry and advances no occurrence counter; and a charge presented with its size vector in either order reports the same first short counter in `ScalarLimitsV1` field order."}
- {"id": "FR-011-AC-5", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first 4096 points in admission order, `charge_log_truncated()` is true, and the counters are still exact and still enforced."}
- {"id": "FR-011-AC-6", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A cumulative counter at `u64::MAX - 1` denies rather than wraps; a derived amount that exceeds `u64::MAX` saturates and the resulting charge is denied rather than admitted."}
- {"id": "FR-011-AC-7", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "`Meter::consumed` answers for all ten `LimitKind` members with no panic path, and IEEE flag iteration, dimension and compound-unit term iteration, and the `UnitGraph::admit` and `check_terms` refusal orders are the stated ones for every input permutation."}
- {"id": "FR-011-AC-8", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit."}
- {"id": "FR-006 Kernel ownership", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "context_only", "excerpt": "The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar\noperations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).\nThe criteria above remain obligations on that consumed behavior. They do not require RT-local\nkernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately\nfrom its disposition after the implementation removes the"}
- {"id": "FR-007 Kernel ownership", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "context_only", "excerpt": "The runtime shall consume kernel scalar operations directly from `quire-exact`, without a\nlocal implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria\nremain obligations on consumed behavior; they do not require RT to retest the kernel."}
- {"id": "TM-001 Evidence at the kernel move", "path": "spec/exact/matrix/tests.md", "role": "context_only", "excerpt": "FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in\nIR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2 and backlog IR-583.\nQSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:\nall current test files and production definitions remain present. The status columns describe\ncurrent evidence only. TC-016/017/023 and their local kernel criteria remain"}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
