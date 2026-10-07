---
id: SR-2380
title: "Code review baseline of the IR-647 spec-only ownership amendment"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@b81def151ee556aa69e1313eccf51097a35e3107; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md, spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-647. Frozen candidate b81def151ee556aa69e1313eccf51097a35e3107 over base main 3cc88c480d4efe5179fe91acf4634def5d2d82fd (merge base equals base; one commit). The diff touches nine Markdown files under spec/exact and no source, test, build, manifest or lockfile, so the code-review baseline checked only that no code, test or build surface changed and that no content was copied in; rust-review and gap-analysis do not apply (no .rs/Cargo change, no production code). Spec content findings are in SR-2381, SR-2383 and SR-2384.

## Verdict

**PASS** — no code defect: the change is spec-only, removes no implementation or test, and contains no copied file, vendored vector or digest/pin.

## Scope

- FR-006 Kernel ownership — role: examined. Excerpt: The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar operations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)). The criteria above remain obligations on that consumed behavior. They do not require RT-local kernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately from its disposition after the implementation removes the copy.
- FR-007 Kernel ownership — role: examined. Excerpt: The runtime shall consume kernel scalar operations directly from `quire-exact`, without a local implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria remain obligations on consumed behavior; they do not require RT to retest the kernel.  For FR-007-AC-7, `ix://agent-ix/quire-exact/FR-362` owns integer/rational arithmetic and ordering, atom charges and Boolean truth tables over already-decided operands; `ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering; and `ix://agent-ix/quire-exact/FR-361` owns the specified denied-work allocation
- tests.md Evidence at the kernel move — role: examined. Excerpt: FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in IR-349, followed by evaluation-residue removal tracked under QSL-358. This amendment is spec-only: all current test files and production definitions remain present. The status columns describe current evidence only. TC-016/017/023 and their local kernel criteria remain implemented; TC-018/019 and FR-007-AC-1/2 are partly evidenced because their allocation checks exist but the shared-corpus agreement oracle is absent. Future deletion does not change those statuses before the code PR removes the tests. The e

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
