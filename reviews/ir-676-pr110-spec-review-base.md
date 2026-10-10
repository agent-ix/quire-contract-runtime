---
id: SR-5071
title: "spec-review/base review of IR-676 RT transfer"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@bb12ae75d6615895f2378028990e0b49766ba298; spec/exact/functional/FR-010-injected-charge-denial-seam.md, spec/exact/matrix/TC-031-injected-charge-denial.md, spec/exact/matrix/TC-017-exact-metering.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-676. The owner references resolve and the status acknowledges RT’s older dependency revision. Evidence mapping findings are recorded by the evidence lens.

## Verdict

**PASS** — no findings in this lens.

## Scope

- FR-010-AC-1 — role: examined; path: spec/exact/functional/FR-010-injected-charge-denial-seam.md; excerpt: For every admitted charge point, an injected denial at occurrence 1 yields `Incomplete` naming that point with `limit_kind = WorkUnits`, every counter unchanged, and no entry appended to the admitted-charge log.
- FR-010-AC-2 — role: examined; path: spec/exact/functional/FR-010-injected-charge-denial-seam.md; excerpt: A denial reached through an RT exact-feature charge-point driver returns the `quire-exact` record specified and directly tested by `ix://agent-ix/quire-exact/FR-358-AC-12`; RT adds no second record definition.
- FR-010-AC-3 — role: examined; path: spec/exact/functional/FR-010-injected-charge-denial-seam.md; excerpt: An RT exact-feature charge-point driver uses the `quire-exact` injection-versus-ordinary-limit precedence specified and directly tested by `ix://agent-ix/quire-exact/FR-358-AC-13`; RT adds no second precedence rule.
- TC-031 — role: examined; path: spec/exact/matrix/TC-031-injected-charge-denial.md; excerpt: For each RT residue operation's admitted charge point, inject occurrence 1 under sufficient limits. Drive the operation and check the refusal names that point, changes no counter or admitted-charge log, and reports the meter's observed denied amount. This is the retained FR-010-AC-1 driver in `tests/exact_outcomes.rs`.
- TC-031 — role: examined; path: spec/exact/matrix/TC-031-injected-charge-denial.md; excerpt: Inspect the RT exact-feature meter dependency and its use by those drivers; check that the returned record and injection-versus-ordinary-limit precedence are the owner behavior in `ix://agent-ix/quire-exact/FR-358-AC-12` and `ix://agent-ix/quire-exact/FR-358-AC-13`. The owner verifies both through `ix://agent-ix/quire-exact/TC-906`; this RT procedure does not duplicate its cases.
- TC-017 — role: examined; path: spec/exact/matrix/TC-017-exact-metering.md; excerpt: The exact injected record and ordinary-limit precedence now have direct owner evidence under `ix://agent-ix/quire-exact/FR-358-AC-12` and `ix://agent-ix/quire-exact/FR-358-AC-13`; RT's older quire-exact dependency revision still needs a separate refresh.
- FR-010-AC-2 — role: examined; path: spec/exact/matrix/tests.md; excerpt: | FR-010 | FR-010-AC-2 | TC-031 | 🚧 owner quire-exact FR-358-AC-12 has direct public-meter record tests; the RT residue driver remains, but RT's lockfile still resolves the older kernel revision and needs a separate refresh |
- FR-010-AC-3 — role: examined; path: spec/exact/matrix/tests.md; excerpt: | FR-010 | FR-010-AC-3 | TC-031 | 🚧 owner quire-exact FR-358-AC-13 has direct public-meter ordinary-limit precedence tests; RT's lockfile still resolves the older kernel revision and needs a separate refresh |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
