---
id: SR-13297
title: "IR-882 PR 135 EARS analysis: RT migration onto quire_exact::Value"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-runtime@1f744539a5842f92bd378f2fc2fe85c6c3b37567; requirement statements and acceptance criteria edited in spec/exact/functional/FR-008-composite-collection-and-equality.md, FR-275-single-exact-kernel.md, FR-277-checked-equality-evaluation.md"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#135. This analysis checked EARS conformance
of the edited requirement statements and acceptance criteria. All three statements keep a
ubiquitous or optional-feature `shall` frame. The edits add declarative sentences inside the
statements that carry no `shall`, and one AC mixes a test property with an inspection property.

## Examined criteria

- FR-008 statement (examined): "When the `exact` feature is enabled, the runtime shall construct composite and collection values ... using `quire_exact::Value` directly ... the runtime consumes the shared kernel: `quire_exact` owns `Value`, `ValueType`, collection formation, membership, canonical keys and equality planning. RT's checked environment and containment entry points use those owner types directly and define no second representation or walker."
- FR-275 statement (examined): "Where the `exact` feature is enabled, the runtime shall take the exact value kernel from the one `quire-exact` crate ... The end state is that the runtime holds no distinct `Value` enum, `planned_equality`, `plan_pairs` or collection membership walker; ... There is no conversion layer or old-`Value` re-export."
- FR-277 statement (examined): "The runtime SHALL route `CheckedEquality::evaluate` in `src/exact/equality.rs` through `quire_exact::planned_equality_with_plan` ..." (conformant, ubiquitous).
- FR-008-AC-9 (examined): "The consumed `quire_exact::Value` has iterative `Debug` and nested-node `Drop` ... RT defines no second `Value`."
- FR-277-AC-1 (examined): event-shaped and conformant, apart from its verification method (see SR-13298 FND-004).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-275 statement adds "The end state is that the runtime holds no distinct `Value` enum ... There is no conversion layer or old-`Value` re-export." These are present-tense assertions with no `shall`, set inside the `Where ... shall` statement, so the prohibition is not normative. Fix: fold it into the response, for example "... and shall define no `Value` enum, pair walker or membership walker of its own, and no conversion layer or re-export of a removed `Value`". | spec/exact/functional/FR-275-single-exact-kernel.md:19-23 |
| FND-002 | low | The FR-008 statement now asserts what another crate owns ("`quire_exact` owns `Value`, `ValueType`, ...") instead of a runtime response, and "use those owner types directly and define no second representation" has no `shall`. Fix: restate it as a runtime obligation, for example "the runtime shall use `quire_exact`'s `Value`, `ValueType`, collection, membership, key and equality-plan operations and shall define no second value representation or walker". | spec/exact/functional/FR-008-composite-collection-and-equality.md:17-22 |
| FND-003 | low | FR-008-AC-9 is compound. It is a Test criterion about iterative `Debug`/`Drop`, but its last sentence, "RT defines no second `Value`", is a structural property that only inspection (FR-275-AC-2/TC-197) can establish. Fix: drop the sentence from AC-9 and rely on the FR-275 one-copy ACs, or add it as a separate inspection criterion. | spec/exact/functional/FR-008-composite-collection-and-equality.md:170 |

## Verdict

**PASS with findings.** All three statements keep a valid EARS frame. The added
prohibitions should become `shall` clauses, and AC-9's structural sentence should move to an
inspection criterion.
