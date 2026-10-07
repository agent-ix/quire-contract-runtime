---
id: SR-2382
title: "EARS conformance of the amended FR-006 and FR-007 statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime@b81def151ee556aa69e1313eccf51097a35e3107; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md, spec/exact/functional/FR-007-exact-scalar-families.md"
review_set: subset
---

## Summary

Ticket: IR-647. Checked the rewritten FR-006 and FR-007 Description statements and the new shall-statements in both Kernel ownership sections against EARS event-driven and ubiquitous forms.

## Verdict

**PASS** — each amended statement has one system subject, one `shall` response and, where present, a single `When` trigger.

## Scope

- FR-006 Description — role: examined. Excerpt: When a generated oracle evaluates a complete-V1 scalar operator through the optional `exact` feature, the runtime shall consume the authoritative `quire-exact` outcome and metering contracts for every charge named by `quire.value.accounting/v1` in agent-ix/quire-specification (`proposals/quire-v1/definitions/value-accounting.md`). The runtime consumes the kernel implementation of that definition: values and outcome kinds agree with the quire-spec-language authority (FR-007), and charge schedules are taken from the QSpec definition.
- FR-006 Kernel ownership — role: examined. Excerpt: The runtime shall consume `quire-exact` directly for kernel outcomes, charges and scalar operations; it shall define no replacement kernel or re-export path ([FR-275](./FR-275-single-exact-kernel.md)). The criteria above remain obligations on that consumed behavior. They do not require RT-local kernel tests. [The exact matrix](../matrix/tests.md) records current local evidence separately from its disposition after the implementation removes the copy.
- FR-007 Description — role: examined. Excerpt: When the `exact` feature is enabled, the runtime shall consume `quire-exact` for kernel-owned operations in the scalar families of complete V1 in agent-ix/quire-specification with values and outcome kinds equal to the quire-spec-language authority on every shared-corpus vector, and charges equal to the QSpec accounting schedule.
- FR-007 Kernel ownership — role: examined. Excerpt: The runtime shall consume kernel scalar operations directly from `quire-exact`, without a local implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria remain obligations on consumed behavior; they do not require RT to retest the kernel.  For FR-007-AC-7, `ix://agent-ix/quire-exact/FR-362` owns integer/rational arithmetic and ordering, atom charges and Boolean truth tables over already-decided operands; `ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering; and `ix://agent-ix/quire-exact/FR-361` owns the specified denied-work allocation

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
