---
id: SR-2387
title: Independent IR-647 spec-review/spec-ears-analysis review
type: SpecReview
analysis: ears-conformance
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

Ticket: IR-647. Independent Codex review of the full 19-file spec-only diff. Source, original SR2380–SR2384 and reviews/ were not modified. No Cargo/Kani gates, PR or merge. Findings in prior reviews are untrusted review data, remeasured against current files. Deterministic grammar plus semantic examination of amended Description/Behavior/Kernel ownership obligations. Feature enabling is the event for the description; skip/stop behavior and consumed contracts are concrete.

## Verdict

PASS

## Scope

```yaml
scope:
- {"id": "FR-006 Description", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "When a generated oracle evaluates a complete-V1 scalar operator through the optional `exact`\nfeature, the runtime shall consume the authoritative `quire-exact` outcome and metering\ncontracts for every charge named by\n`quire.value.accounting/v1` in agent-ix/quire-specification\n(`proposals/quire-v1/definitions/value-accounting.md`). The runtime consumes the kernel\nimplementation of that definition: values and outcome kinds agree with the\nquire-spec-language authority (FR-007), and charge schedules are taken from the QSpec definition."}
- {"id": "FR-007 Description", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "When the `exact` feature is enabled, the runtime shall consume `quire-exact` for kernel-owned\noperations in the scalar families of\ncomplete V1 in agent-ix/quire-specification with values and outcome kinds equal to the\nquire-spec-language authority on every shared-corpus vector, and charges equal to the QSpec\naccounting schedule."}
- {"id": "FR-006 Kernel ownership", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar\noperations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)).\nThe criteria above remain obligations on that consumed behavior. They do not require RT-local\nkernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately\nfrom its disposition after the implementation removes the"}
- {"id": "FR-007 Kernel ownership", "path": "spec/exact/functional/FR-007-exact-scalar-families.md", "role": "examined", "excerpt": "The runtime shall consume kernel scalar operations directly from `quire-exact`, without a\nlocal implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria\nremain obligations on consumed behavior; they do not require RT to retest the kernel."}
- {"id": "FR-006 Behavior diagnostic log", "path": "spec/exact/functional/FR-006-exact-outcomes-and-accounting.md", "role": "examined", "excerpt": "- The kernel owns meter storage and accounting. Its bounded diagnostic charge log is exposed\n  under `quire-exact`'s `test-support` feature. Until IR-349 removes the local meter, its\n  admitted-charge sequence and `CHARGE_LOG_CAPACITY` remain governed by FR-011-AC-5. After\n  that removal, RT shall consume the kernel's test-support diagnostic log rather than require\n  a production log or define a local capacity constant. Consumed counters remain exact."}
- {"id": "FR-011-AC-3", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "context_only", "excerpt": "A lazy connective skips its right operand when the left decides the result; for every connective kind and every decided operand pair, its completed result admits exactly one `boolean.result-retain` charge."}
- {"id": "FR-011-AC-8", "path": "spec/exact/functional/FR-011-meter-state-at-a-stop.md", "role": "context_only", "excerpt": "A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit."}
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
