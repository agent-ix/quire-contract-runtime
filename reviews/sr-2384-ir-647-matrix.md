---
id: SR-2384
title: "Matrix status and binding review of the IR-647 amendment"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@b81def151ee556aa69e1313eccf51097a35e3107; spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/functional/FR-011-meter-state-at-a-stop.md, spec/exact/matrix/TC-023-metered-arithmetic.md, spec/exact/matrix/TC-032-meter-state-at-a-stop.md, spec/exact/matrix/tests.md"
review_set: subset
---
 
## Summary

Ticket: IR-647. Read-only matrix pass (spec-matrix adds no tags here: the candidate is frozen and spec-only). Compared the changed status cells and evidence paragraphs with the tagged tests at the reviewed sha and with `quire coverage --scope . --strict`; that gate reports 56 unbacked rows and 5 contradicted statuses at both base and head, differing only in shifted line numbers, so the amendment adds no coverage regression. TC-016/017/023 stay ✅ implemented and their tests are present; FR-007-AC-1/2 and TC-018/019 "partly evidenced" matches present allocation tests and the removed agreement oracle.

## Verdict

**FAIL** — the RT-owned lazy-connective clause has no binding TC, and the planned agreement work is attributed to a closed ticket.

## Scope

- tests.md FR-007-AC-1 row — role: examined. Excerpt: | FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-430) |
- tests.md FR-007-AC-2 row — role: examined. Excerpt: | FR-007 | FR-007-AC-2 | TC-019 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-430) |
- tests.md FR-007-AC-7 row — role: examined. Excerpt: | FR-007 | FR-007-AC-7 | TC-023 | ✅ implemented |
- tests.md TC-018 row — role: examined. Excerpt: | TC-018 | Agree with the authority on integer division vectors | Integration | P0 | FR-007-AC-1, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-430) |
- tests.md TC-019 row — role: examined. Excerpt: | TC-019 | Agree with the authority on exact decimal vectors | Integration | P0 | FR-007-AC-2, FR-007-AC-6 | 🚧 partly evidenced: local allocation checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-430) |
- tests.md Evidence at the kernel move — role: examined. Excerpt: FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in IR-349, followed by evaluation-residue removal tracked under QSL-358. This amendment is spec-only: all current test files and production definitions remain present. The status columns describe current evidence only. TC-016/017/023 and their local kernel criteria remain implemented; TC-018/019 and FR-007-AC-1/2 are partly evidenced because their allocation checks exist but the shared-corpus agreement oracle is absent. Future deletion does not change those statuses before the code PR removes the tests. The e
- tests.md exact_meter_state.rs disposition — role: examined. Excerpt: | `tests/exact_meter_state.rs` | TC-032 | splits | `evaluate_boolean_short_circuit` lazy invocation/stop propagation (FR-011-AC-3/AC-8, step 5) stays in RT; already-decided Boolean and meter cases leave in step 1; `UnitGraph::admit`, `CompoundUnit` and `evaluate_quantity` cases leave with QSL-358 residue |
- tests.md end-state paragraph — role: examined. Excerpt: For the kernel, RT keeps the consumption checks TC-197 to TC-199 in the end state: direct consumption, no copied code and guarded edges. RT also keeps evidence of its own lazy connective in TC-032 and backend negotiation in TC-030/TC-195; kernel deletion shall not remove those checks.
- tests.md TC-018..TC-022 evidence location — role: examined. Excerpt: - TC-018 through TC-022: The QSL agreement oracle is removed from this repository; recreating it in agent-ix/quire-integration is planned under Linear IR-430. TC-018/TC-019 currently retain only local allocation-bound evidence; it leaves in IR-349 and does not close the IR-430 agreement gap. Owner mappings are in the individual TC documents.
- FR-007-AC-7 — role: examined. Excerpt: | FR-007-AC-7 | Integer arithmetic, rational arithmetic, ordering and Boolean connectives match an independent `i128` oracle and the QSpec TC-191 P11 and TC-190 Q11 atom charges, with denial behavior at every kernel point; RT-owned lazy connectives skip the right operand when the left decides the result and propagate a right-operand stop unchanged without retention; operand-derived arithmetic and normalize amounts are charged before any intermediate or result is allocated. | Test (TC-023) |
- FR-011-AC-3 — role: context_only. Excerpt: | FR-011-AC-3 | A connective that short-circuits admits exactly one `boolean.result-retain` charge, for every connective kind and every decided operand pair. | Test (TC-032) |
- FR-011-AC-8 — role: context_only. Excerpt: | FR-011-AC-8 | A connective whose right operand stops returns that stop unchanged, admits no `boolean.result-retain` charge and consumes no result unit. | Test (TC-032) |
- TC-023 Ownership and evidence — role: examined. Excerpt: Kernel arithmetic, ordering, atom charges and already-decided Boolean retention leave RT in IR-349: `ix://agent-ix/quire-exact/FR-362` owns those subsets; decimal ordering belongs to `ix://agent-ix/quire-exact/FR-363`, and the specified denied-work allocation bounds to `ix://agent-ix/quire-exact/FR-361`. These are owner references, not copied tests or a claim that the P11/Q11 expression workloads have been evaluated upstream. Lazy operand evaluation is RT-owned and remains in TC-032; step 3's caller simulation in this file is not its retained evidence. IR-430 shared-authority agreement remains
- TC-032 Ownership and evidence — role: examined. Excerpt: Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure when the left decides, propagating each right-operand stop unchanged with no retention, and retaining a completed connective result exactly once. Current evidence is `tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above (FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires. The already-decided truth-table test is kernel-owned; `ix://agent-ix/quire-exact/FR-362` does not test lazy closure invocation. Steps 1–3 and 6–8 and the kerne

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-007-AC-7 now adds an RT-owned lazy-connective clause but its Verification still names only TC-023, which the amendment says leaves RT in IR-349 and whose step 3 "is not its retained evidence", while tests.md says TC-032 is not a binder for FR-007-AC-7; after IR-349 the permanent RT-owned clause has no verifying TC. Add TC-032 to AC-7's verification (and tag the retained test) or split the lazy clause into its own criterion. | spec/exact/functional/FR-007-exact-scalar-families.md:101 |
| FND-002 | medium | Changed status cells and new ownership text say QSL shared-corpus agreement "remains planned (Linear IR-430)" and "stays separate under IR-430", but IR-430 is Done (2026-09-30) and tracked RT PR #88's removal of qsl-agreement, not its recreation; no open Linear ticket owns the quire-integration agreement oracle, so the planned status names a closed, non-owning ticket. | spec/exact/matrix/tests.md:17 |
