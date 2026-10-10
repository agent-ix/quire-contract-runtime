---
id: SR-5076
title: "spec-review/evidence review of IR-676 RT transfer"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-runtime@bb12ae75d6615895f2378028990e0b49766ba298; spec/exact/functional/FR-010-injected-charge-denial-seam.md, spec/exact/matrix/TC-031-injected-charge-denial.md, spec/exact/matrix/TC-017-exact-metering.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-676. The owner tests prove the upstream meter behavior, but the RT verification cells and retained TC-031 procedure overstate the RT-local evidence.

## Verdict

**FAIL** — three evidence descriptions need correction.

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
| FND-001 | medium | AC-2 names Test (TC-031) as its verification, but Quire matrix reports AC-2 untagged and the retained RT test does not compare the two configured work limits. TC-031 step 2 itself says RT does not duplicate owner cases. Name the owner TC-906 Test and RT dependency Inspection separately, and keep the RT pin gap explicit. | spec/exact/functional/FR-010-injected-charge-denial-seam.md:57 |
| FND-002 | medium | AC-3 names Test (TC-031), but Quire matrix reports AC-3 untagged and the retained RT test never puts injection and an ordinary short counter in competition. TC-031 step 2 assigns this proof to owner TC-906. Correct the verification mapping so it cannot be read as an RT test claim; preserve the unrefreshed RT dependency status. | spec/exact/functional/FR-010-injected-charge-denial-seam.md:58 |
| FND-003 | medium | TC-031 step 1 claims the retained RT driver checks that no counter or admitted-charge log changes. Its sole tagged test checks WorkUnits and ResultUnits, derives expected work from log length, and checks only that the denied point is absent from the log; it does not snapshot all counters, admission count, or the entire log. Attribute full atomicity to owner evidence or narrow the RT-local claim, while keeping AC-1 coverage honest. | spec/exact/matrix/TC-031-injected-charge-denial.md:31-34 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | AC-1 now quantifies only over RT residue charge points already driven in TC-031. A new RT-owned charge point omitted from that test falls outside the requirement, whereas the previous AC required every admitted charge point. Quantify over every RT-owned residue point independent of the current test inventory; keep the narrowed local observation and owner atomicity allocation. | spec/exact/functional/FR-010-injected-charge-denial-seam.md:56 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1d1c420f78aebc25cca49546817f2f89d9e954c1 |
| FND-002 | fixed | 1d1c420f78aebc25cca49546817f2f89d9e954c1 |
| FND-003 | fixed | 1d1c420f78aebc25cca49546817f2f89d9e954c1 |
| FND-004 | still-open | AC-1 quantifier depends on the current test inventory. |
| FND-004 | fixed | 5c40a39d4ac7d10626051cf7b9eb0c76952c827a |
