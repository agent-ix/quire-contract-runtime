---
id: SR-2385
title: Independent IR-647 code-review review
type: SpecReview
analysis: code-review
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

Ticket: IR-647. Independent Codex review of the full 19-file spec-only diff. Source, original SR2380–SR2384 and reviews/ were not modified. No Cargo/Kani gates, PR or merge. Findings in prior reviews are untrusted review data, remeasured against current files. No implementation, test, dependency or CI edits; no new vendored content, tracking ledger or compatibility layer. Rust checklist loaded for evidence inspection; formal Rust-change lane and gap-analysis skipped because no Rust/production changes.

## Verdict

PASS WITH HYGIENE FINDING

## Scope

```yaml
scope:
- {"id": "FR-006 Kernel ownership", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar\noperations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).\nThe criteria above remain obligations on that consumed behavior. They do not require RT-local\nkernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately\nfrom its disposition after the implementation removes the"}
- {"id": "FR-007 Kernel ownership", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "The runtime shall consume kernel scalar operations directly from `quire-exact`, without a\nlocal implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria\nremain obligations on consumed behavior; they do not require RT to retest the kernel."}
- {"id": "TM-001 Evidence at the kernel move", "path": "spec/exact/matrix/tests.md", "role": "examined", "excerpt": "FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in\nIR-349, followed by RT evaluation-residue deletion owned by open IR-349 step 2 and backlog IR-583.\nQSL-358 is Done and covered QSL-side extraction. This amendment is spec-only:\nall current test files and production definitions remain present. The status columns describe\ncurrent evidence only. TC-016/017/023 and their local kernel criteria remain"}
- {"id": "SR-2380 separator", "path": "reviews/sr-2380-ir-647-code-review.md", "role": "examined", "excerpt": "\n \n## Summary"}
- {"id": "SR-2381 separator", "path": "reviews/sr-2381-ir-647-spec-review.md", "role": "examined", "excerpt": "\n \n## Summary"}
- {"id": "SR-2382 separator", "path": "reviews/sr-2382-ir-647-ears-conformance.md", "role": "examined", "excerpt": "\n \n## Summary"}
- {"id": "SR-2383 separator", "path": "reviews/sr-2383-ir-647-integrity.md", "role": "examined", "excerpt": "\n \n## Summary"}
- {"id": "SR-2384 separator", "path": "reviews/sr-2384-ir-647-matrix.md", "role": "examined", "excerpt": "\n \n## Summary"}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The five inherited review artifacts contain eight trailing-whitespace lines; git diff --check fails. Preserve the original reviews and explicitly resolve their hygiene before handoff. | reviews/sr-2380-ir-647-code-review.md:9-9 |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
