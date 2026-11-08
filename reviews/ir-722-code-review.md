---
id: SR-202611080241211
title: "Code and Rust review of RT #211 bounded State enforcement"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; src/monitor/executor.rs, src/monitor/dispatch.rs, src/monitor/vm.rs, spec/monitor/matrix/tests.md"
review_set: subset
---
# Code and Rust review of RT #211

## Summary

Ticket: IR-722. PR quire-contract-runtime#211 at d8ea370f78bac73f0a5bd6457c5a1f65c75e048e,
scope `git diff origin/main...HEAD`. Code-review with the rust-review lane folded in.
Examined `state_within_bounds`, its call in `build` after `init`, its call in
`step_values` for `on_event`, `on_watermark` and `on_gap`, the new
`ExecutorInvariantCode::StateBound` and `vm::Broken::StateBound` mapping, and the three
unit tests. Ran `cargo test --features exact --lib state_bound` (3 pass) and
`cargo test --features exact --test exact_vocabulary tc_033_rt_does_not_construct_semantic_identities`
(fails on this head, naming only src/monitor/executor.rs).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new test module's `fn key() -> NodeKey { NodeKey::new("package", ...) }` fixture breaks the existing TC-033 / FR-012-AC-8 inspection test `tc_033_rt_does_not_construct_semantic_identities`, which forbids RT test fixtures that build a `NodeKey`. The `exact`-feature suite goes red on this head | src/monitor/executor.rs:427-434, tests/exact_vocabulary.rs:551-581 |
| FND-002 | low | A State that is not a record, or lacks a planned sequence slot, returns false and is reported as `StateBound`. FR-276 reserves `StateBound` for the length-bound condition. The branch is unreachable after result membership admission, so it should map to the existing shape class or be documented as unreachable, not tested as a StateBound case | src/monitor/executor.rs:281-291, src/monitor/executor.rs:493-509 |
| FND-003 | low | `Integer::from(u64::try_from(counted).unwrap_or(u64::MAX))` saturates silently. `Integer::from(usize)` is already used in this crate, so the conversion and its fallback are unnecessary | src/monitor/executor.rs:294 |

## Verdict

Not mergeable: FND-001 turns the `exact`-feature test gate red. The bound predicate
itself is right. It reads lengths only, it maps `buffer_<n>` slots by plan index (as
validation already fixes the State record order), it uses `len - 1 <= capacity`, which
equals `capacity + 1`, and it caps `activations` at `activation_capacity`. Atomicity holds.
In `step_values` the check runs before `self.state`, `self.baseline` or `self.progress` is
assigned, and `failed` sets `terminated` for the `Invariant` code. In `build` the check runs
before the `Monitor` is constructed. Suggested fix for FND-001: test an identity-free core,
for example a function over capacities and slot lengths, so no `NodeKey`, `Buffer.owner` or
tuple key is needed.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The fix adds a heap `Vec` in `state_within_bounds`, and `try_reserve_exact(...).ok()?` maps an allocator refusal to `None`, which `state_defect` reports as `Broken::Projection`. A refused allocation therefore terminates the monitor as an executor invariant failure. FR-276 says allocation or resource failure is not an invariant defect. The allocation is also not charged to `StepBudget` scratch. It is unneeded: iterate `buffer_plan` and check each `length(index)?` in place, with no allocation | src/monitor/executor.rs:300-304, src/monitor/executor.rs:315-321 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | f6d3cff tests only `lengths_within_bounds` over (len, capacity) pairs with no `NodeKey`; tc_033 and both state_bound tests pass on f6d3cff |
| FND-002 | fixed | `state_within_bounds` returns `Option<bool>`; `state_defect` maps `None` to `Broken::Projection` and only `Some(false)` to `StateBound`; the shape test is removed |
| FND-003 | fixed | `quire_exact::Integer::from(len.saturating_sub(1))` and `Integer::from(activations.0)` replace the u64 conversion |
