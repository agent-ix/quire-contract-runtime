---
id: SR-2026110705000806
title: "Failure-domain review of RT #209 bounded monitor state and budget boundary"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-runtime; spec/monitor/functional/FR-276-consume-timed-monitor-plan.md, spec/monitor/matrix/TC-200-tick-monitor-consumer.md"
review_set: subset
---
# Failure-domain review of RT #209

## Summary

Ticket: IR-722. Looked for unstated failure modes in the construction and budget
boundary (FR-276 lines 138-145, AC-13) and in the bounded state check (lines 128-137,
AC-12), against `validation.rs`, `construction.rs`, `dispatch.rs` and the shared
TypeEnvironment walks that `try_new` calls. The commit-before-check path is sound. Every
dispatch route replaces `self.state` only after `step_values`, so a check placed there
commits nothing and moves neither the baseline nor the progress. The construction bound
does not cover every walk that `try_new` performs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `try_new` bound lists functions, expressions, operand edges, bindings, State fields and buffers only. Two walks are left out. (1) Literal membership (`TypeEnvironment::admits`) scales with literal size, and its reservation failure refuses `Membership`, not `Storage`. (2) Per-signature `check_type` re-walks the type graph with infallible `vec!` | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:140-143, src/monitor/validation.rs:37-62 |
| FND-002 | medium | AC-13 needs "a forced validation scratch reservation failure", but `try_new` takes only `ProgramParts`, and no spec line says how the failure is forced (a test-only reservation seam, or a failing global allocator in a dedicated test binary). The coder must choose | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:181, spec/monitor/matrix/TC-200-tick-monitor-consumer.md:86 |

## Verdict

Both findings block. FND-001: either list literal elements and type declarations, with
their reservation refusal code, or narrow the claim to RT-owned scratch and say so. A
plan with a large literal sequence is currently constructed at a cost the stated part
counts do not bound, and a literal-walk reservation failure returns `Membership`, which
contradicts AC-13's `Storage`.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | AC-13's control cannot pass. `validate_layout`, the first walk, calls `check_type` (infallible `vec!` in semantic-value) before any RT `try_reserve`, so a failing allocator refusing the first `try_new` allocation aborts and never returns `Storage`. The table row "Layout premises ... none allocated" is false, and "sweep covers only allocations before the first check_type" covers none | spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:154-168, spec/monitor/functional/FR-276-consume-timed-monitor-plan.md:220, src/monitor/validation.rs:428-436 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | The walk table, the literal `Membership` mapping and the RT `Storage` scratch rows fix the enumeration. The `check_type` gap is still open: RT FR-276 places a SHALL on semantic-value and adds an "until the owner provides it ... aborts" carve-out. Semantic-value must expose a public fallible type check that returns a typed reservation failure; its private `type_refusal_admission` already uses a `ReservationPolicy`. RT must route every `check_type` call through it. The dependency belongs in Dependencies as a real cross-owner requirement, without the transitional clause |
| FND-002 | fixed | A forced failure is now produced only by a failing global allocator in a dedicated integration test binary, after the `measurement/exact-panic` precedent; `forbid(unsafe_code)` applies to the library only |
| FND-001 | fixed | 7e3e779: every `try_new` type check (the five Layout bindings, then each parameter, result and expression type) goes through semantic-value's fallible check-type entry and maps to `Membership` with the native cause; Dependencies states the requirement on semantic-value; the transitional abort clause is removed |
| FND-003 | fixed | 7e3e779: the first-allocation control now expects `Membership` (the first Layout type check's worklist), and a separate RT-scratch control expects `Storage`. The Layout row excludes the type checks; the remaining Layout calls (`same_type`, unit lookup, composite lookup) allocate nothing |
