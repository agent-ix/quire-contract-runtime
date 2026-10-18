---
id: SR-12904
title: IR-878 RT130 spec-review
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; PR130 head 4917ef5337a35ac0b8bb0195621e0def49689372 vs main a0bbc149; spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/matrix/TC-202-one-copy-deny-mutation.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-878. PR: quire-contract-runtime#130. Spec review of the TC-202 split. Sub-analyses run under their own ids: spec-integrity-analysis (SR-12905) and spec-evidence-analysis (SR-12906). Skipped as out of scope: spec-ears-analysis (no requirement or AC statement text changed), spec-object-review (no domain object edited), spec-dependency-analysis (only one `verifies` edge added, mirroring TC-197), spec-criterion-strength-analysis (AC text unchanged).

## Verdict

CONDITIONAL on one low cross-reference finding. The split is consistent where it matters: FR-275-AC-6 names Test (TC-202) in the FR table, the tests.md requirement and TC rows both move AC-6 from TC-197 to TC-202, TC-197 step 3 no longer claims the duplicate mutation, and TC-202's procedure matches what the test does. `quire validate` on the four files and the full `spec/**` `plan/**` `reviews/**` set exits 0 (204/204 grammar-clean).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The tests.md end-state paragraph still says RT keeps "the consumption checks TC-197 to TC-199", and Evidence Locations has no TC-202 entry, although TC-202 now carries the one-copy guard that used to sit in TC-197. | spec/exact/matrix/tests.md:151-153; spec/exact/matrix/tests.md:155 |

## Finding evidence

Before this PR the one-copy mutation was part of TC-197, so "TC-197 to TC-199 ... guarded edges" covered it. After the split a reader of tests.md:151 would conclude the duplicate-lock guard is not among the checks retained in the end state. Add TC-202 to that sentence and an Evidence Locations bullet naming `tests/deny_one_copy.rs` and `scripts/check_one_copy.awk`.

## Examined units

```yaml
scope:
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: "`make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate."
- id: FR-275-AC-5
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: "`Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`, `quire-canonical` and `quire-canonical-derive`."
- id: TC-197
  path: spec/exact/matrix/TC-197-single-kernel-ownership.md
  role: examined
  excerpt: "Inspect that the runtime holds one kernel and no copy"
- id: TC-202
  path: spec/exact/matrix/TC-202-one-copy-deny-mutation.md
  role: examined
  excerpt: "Reject duplicate first-party Git lock entries through make deny"
```
