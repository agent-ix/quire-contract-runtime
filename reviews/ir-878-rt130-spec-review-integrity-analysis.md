---
id: SR-12905
title: IR-878 RT130 spec-review integrity-analysis
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; PR130 head 4917ef5337a35ac0b8bb0195621e0def49689372 vs main a0bbc149; spec/spec.md, spec/exact/matrix/TC-202-one-copy-deny-mutation.md, spec/exact/matrix/tests.md, spec/exact/matrix/TC-197-single-kernel-ownership.md"
review_set: subset
---

## Summary

Ticket: IR-878. PR: quire-contract-runtime#130. Structural integrity of the TC-202 split: one owner per criterion, matrix rows agreeing with TC documents and the FR table, and the registry listing every TC.

## Verdict

CONDITIONAL on one low finding. Atomicity and consistency hold: FR-275-AC-6 has exactly one TC (TC-202) in the FR table, in the requirement-to-TC rows and in the TC rows; TC-197 lists AC-1 to AC-5 and AC-12 to AC-19 in both tables and in its document; TC-202's frontmatter (`verifies` FR-275) follows the TC-197/TC-201 shape. The registry was not updated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The spec registry's verification-matrix enumeration lists "TC-197–TC-199, planned TC-200 and TC-201" and omits the new TC-202. | spec/spec.md:81-82 |

## Finding evidence

spec/spec.md is the subsystem registry (CLAUDE.md). The diff adds TC-202 but leaves this enumeration unchanged, so the registry no longer names every TC. Change it to include TC-202.

## Examined units

```yaml
scope:
- id: TC-202
  path: spec/exact/matrix/TC-202-one-copy-deny-mutation.md
  role: examined
  excerpt: "Reject duplicate first-party Git lock entries through make deny"
- id: TC-197
  path: spec/exact/matrix/TC-197-single-kernel-ownership.md
  role: examined
  excerpt: "Inspect that the runtime holds one kernel and no copy"
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: "`make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate."
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head 7a45ff8: spec/spec.md:82 now lists TC-202 among the verification-matrix test cases. |
