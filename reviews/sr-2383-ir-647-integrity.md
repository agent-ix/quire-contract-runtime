---
id: SR-2383
title: "Integrity review of the IR-647 kernel evidence ownership amendment"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@b81def151ee556aa69e1313eccf51097a35e3107; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md, spec/exact/functional/FR-007-exact-scalar-families.md, spec/exact/functional/FR-011-meter-state-at-a-stop.md, spec/exact/matrix/TC-032-meter-state-at-a-stop.md, spec/exact/matrix/tests.md"
review_set: subset
---
 
## Summary

Ticket: IR-647. Checked the amended FR-006/FR-007 text and TC ownership sections for consistency with unchanged requirements (FR-011, FR-275, interface-001) and with the tracker state of the tickets they cite (read from Linear: IR-349 In Progress, QSL-358 Done 2026-10-02, IR-430 Done 2026-09-30).

## Verdict

**FAIL** — FR-006 now forbids what unchanged FR-011-AC-5 requires and current code implements.

## Scope

- FR-006 Behavior (meter storage) — role: examined. Excerpt: - The kernel owns meter storage and accounting. Its bounded diagnostic charge log is exposed under `quire-exact`'s `test-support` feature; RT shall not require a production log or define a local `CHARGE_LOG_CAPACITY`. Consumed counters remain exact.
- FR-006 Outputs — role: context_only. Excerpt: - `Outcome<T>`: `Completed(T)`, `Undefined(Undefined)`, `Refused(Refusal)` or `Incomplete(Incomplete)`, plus the admitted charge sequence and consumed counters on the `Meter`.
- FR-006 Behavior (FR-011 deferral) — role: context_only. Excerpt: - What the meter holds at an `Undefined` or `Refused` stop, the atomicity of one charge, the field-order scan over a charge's own size vector, the value and truncation behavior of `CHARGE_LOG_CAPACITY`, and which derived amounts saturate are FR-011.
- FR-006 Kernel ownership (IR-349/QSL-358/IR-430) — role: examined. Excerpt: This amendment deletes no implementation or test. The remaining local kernel copy is IR-349 work; non-kernel evaluation residue is QSL-358 work. Neither is RT-owned by remaining present. The missing QSL agreement evidence stays separate under IR-430.
- FR-011-AC-5 — role: context_only. Excerpt: | FR-011-AC-5 | Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first 4096 points in admission order, `charge_log_truncated()` is true, and the counters are still exact and still enforced. | Test (TC-032) |
- FR-007 Kernel ownership — role: examined. Excerpt: The runtime shall consume kernel scalar operations directly from `quire-exact`, without a local implementation or re-export ([FR-275](./FR-275-single-exact-kernel.md)). The criteria remain obligations on consumed behavior; they do not require RT to retest the kernel.  For FR-007-AC-7, `ix://agent-ix/quire-exact/FR-362` owns integer/rational arithmetic and ordering, atom charges and Boolean truth tables over already-decided operands; `ix://agent-ix/quire-exact/FR-363` owns retained-representation decimal ordering; and `ix://agent-ix/quire-exact/FR-361` owns the specified denied-work allocation 
- TC-032 Ownership and evidence — role: examined. Excerpt: Step 5's lazy connective behavior remains RT-owned after IR-349: skipping the right closure when the left decides, propagating each right-operand stop unchanged with no retention, and retaining a completed connective result exactly once. Current evidence is `tc_032_ac3_ac8_short_circuit_propagates_a_stop_and_retains_exactly_once` in the file above (FR-011-AC-3 and FR-011-AC-8). Its future API home is `scalar`, as interface-001 requires. The already-decided truth-table test is kernel-owned; `ix://agent-ix/quire-exact/FR-362` does not test lazy closure invocation. Steps 1–3 and 6–8 and the kerne
- tests.md Evidence at the kernel move — role: examined. Excerpt: FR-275 requires RT to consume `quire-exact` directly and remove its local kernel copy in IR-349, followed by evaluation-residue removal tracked under QSL-358. This amendment is spec-only: all current test files and production definitions remain present. The status columns describe current evidence only. TC-016/017/023 and their local kernel criteria remain implemented; TC-018/019 and FR-007-AC-1/2 are partly evidenced because their allocation checks exist but the shared-corpus agreement oracle is absent. Future deletion does not change those statuses before the code PR removes the tests. The e

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-006 Behavior now states unconditionally that RT shall not require a production log or define a local `CHARGE_LOG_CAPACITY`, contradicting unchanged FR-011-AC-5 (Past `CHARGE_LOG_CAPACITY` admitted charges the log holds exactly the first 4096 points), FR-006's own Outputs (admitted charge sequence on the `Meter`) and FR-006 Behavior line 60, and the current code (`pub const CHARGE_LOG_CAPACITY` at src/exact/accounting.rs:478) whose rows stay ✅ implemented; condition it on IR-349 completion or amend FR-011-AC-5 consistently. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:47-49 |
| FND-002 | low | The amendment assigns the remaining RT evaluation residue to QSL-358 work ("residue is QSL-358 work", "remain QSL-358 residue until removed", "removal tracked under QSL-358"), but QSL-358 is Done (2026-10-02) and covered QSL-side extraction; RT's own residue deletion has no open owner named. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:89-91 |
