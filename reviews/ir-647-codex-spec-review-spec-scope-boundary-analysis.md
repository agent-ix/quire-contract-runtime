---
id: SR-2392
title: Independent IR-647 scope-boundary review
type: SpecReview
analysis: scope-boundary
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

Ticket: IR-647. RT core owns lazy right-closure invocation/stop propagation (FR-011-AC-3/AC-8) and backend negotiation; quire-exact core owns outcome, meter, numeric operations and already-decided Boolean retention. QSpec is the semantic/accounting dependency and QSL the independent authority. Kernel contracts are consumed/assumed at RT’s end state, rather than guaranteed by retained RT kernel tests; current RT binders record local pre-removal evidence only. IR-669 owns cross-repo agreement composition, currently absent, without a guarantee claim. IR-349 step 2/IR-583 own residue removal. No new external component or relationship edges are introduced; retained contract boundaries are explicit and no compatibility path is added.

## Verdict

PASS for this method; overall candidate FAIL on SR-2386/SR-2388, with SR-2385 hygiene.

## Scope

```yaml
scope:
- {"id": "FR-006 Kernel ownership", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar\noperations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).\nThe criteria above remain obligations on that consumed behavior. They do not require RT-local\nkernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately\nfrom its disposition after the implementation removes the"}
- {"id": "FR-007 Kernel ownership", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "The runtime shall consume kernel scalar operations directly from `quire-exact`, without a\nlocal implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria\nremain obligations on consumed behavior; they do not require RT to retest the kernel."}
- {"id": "FR-007-AC-7", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom charges, with denial behavior at every kernel point; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated."}
- {"id": "FR-011-AC-3", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A lazy connective skips its right operand when the left decides the result; for every connective kind and every decided operand pair, its completed result admits exactly one `boolean.result-retain` charge."}
- {"id": "FR-011-AC-8", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "examined", "excerpt": "A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit."}
- {"id": "TM-001 Evidence at the kernel move", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in\nIR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2 and backlog IR-583.\nQSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:\nall current test files and production definitions remain present. The status columns describe\ncurrent evidence only. TC-016/017/023 and their local kernel criteria remain"}
- {"id": "TC-016 Ownership and evidence", "path": "spec/exact/matrix/TC-016-exact-outcome-envelope.md", "role": "examined", "excerpt": "Kernel-owned after IR-349 removes the local copy. The evidence path above is current\nRT evidence, not a retained RT test obligation. The outcome envelope, reason variants and\nspelling census remain required; FR-359–363 do not establish them. An exact owner criterion\nmapping for those checks remains to be identified; no upstream pass is claimed."}
- {"id": "TC-017 Ownership and evidence", "path": "spec/exact/matrix/TC-017-exact-metering.md", "role": "examined", "excerpt": "Kernel-owned after IR-349 removes the local copy. The evidence path above remains present\nin this spec-only amendment. `ix://agent-ix/quire-exact/FR-358` owns step 5's one-shot denial\nand step 6's diagnostic-log bound under `test-support`; `ix://agent-ix/quire-exact/FR-359`\nowns cumulative-boundary and atomic-refusal checks, not the entire step 3 counter census.\nNo reference is a claim that every procedure step is mapped or verified upstream."}
- {"id": "TC-018 Ownership and evidence", "path": "spec/exact/matrix/TC-018-integer-division-agreement.md", "role": "examined", "excerpt": "Steps 1–3 remain a QSL agreement gap with open quire-integration ticket IR-669. Steps 4–5 currently have RT allocation\ncoverage; that local kernel evidence leaves in IR-349, without replacement tests in RT.\n`ix://agent-ix/quire-exact/FR-361-AC-3` owns injected division/modulus denial before large\nallocation only. It does not establish the generated sweep, exact/one-under amounts or\nTC-192 agreement; those remaining obligations require their own"}
- {"id": "TC-019 Ownership and evidence", "path": "spec/exact/matrix/TC-019-decimal-agreement.md", "role": "examined", "excerpt": "Steps 1–3 remain a QSL agreement gap with open quire-integration ticket IR-669. Step 5 currently has RT allocation\ncoverage, which leaves in IR-349. `ix://agent-ix/quire-exact/FR-361-AC-6` owns that denied\nretain-upscale allocation bound; FR-361-AC-4/AC-5 cover scale-expansion/arithmetic denial.\n`ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering only.\nNeither contract establishes TC-185 agreement or all"}
- {"id": "TC-023 Ownership and evidence", "path": "spec/exact/matrix/TC-023-metered-arithmetic.md", "role": "examined", "excerpt": "Kernel arithmetic, ordering, atom charges and already-decided Boolean retention leave RT\nin IR-349: `ix://agent-ix/quire-exact/FR-362` owns those subsets; decimal ordering belongs to\n`ix://agent-ix/quire-exact/FR-363`, and the specified denied-work allocation bounds to\n`ix://agent-ix/quire-exact/FR-361`. These are owner references, not copied tests or a claim\nthat the P11/Q11 expression workloads have been evaluated upstream. Lazy operand"}
- {"id": "TC-032 Ownership and evidence", "path": "spec/exact/matrix/TC-032-meter-state-at-a-stop.md", "role": "examined", "excerpt": "Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure\nwhen the left decides, propagating each right-operand stop unchanged with no retention, and\nretaining a completed connective result exactly once. Current evidence is\n`tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above\n(FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires.\nThe"}
- {"id": "interface-001 scalar lazy operation", "path": "spec/core/functional/interface-001-runtime-api.md", "role": "context_only", "excerpt": "      operations: [evaluate_boolean_short_circuit]"}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
