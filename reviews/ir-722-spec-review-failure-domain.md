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
