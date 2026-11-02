---
id: SR-20261102072294
title: "IR-722 scalar quire-contract-runtime spec-review/scope-boundary"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; IR-722; frozen scalar amendment and interface003 doc fix; interface_008, IT-018-SC-10"
review_set: subset
---

## Summary

Exact owns arithmetic, relations, charge schedules and native outcomes. RT owns explicit operating budgets, canonical computed storage and envelope/source retention. Semantic monitor capacities/outcome kinds, root/State and source identity remain unchanged.

Frozen scalar source on original RT193 f3927c041579b7ae678f0c431e8b5127ce98a318 and integration35 5a51d1dcca86f98f762a6726e244c0be2f75e67b. The later interface003-only documentation fix dbea2d55b7a925010a0c1bf709740ca034eb2756 is included for disposition; constructor spec/production continuation is excluded. Read frozen Git snapshots, repository guidance, Rust/code review and selected scoped interface methods. No worktree source edits, Cargo/heavy/proof jobs, merge/closure or permission actions performed. No applicable AssuranceProfile found.

No concrete arithmetic/order/type/ownership defect was found. Binary child frames are pushed right then left, hence execute left first; scalar handling pops right then left and delegates exactly to shared evaluate_integer_arithmetic/order_numbers. Integer/Int inputs and arithmetic result annotations are checked; ordering requires Boolean. The declared Int result interval is forwarded to the native operation. Native Meter is created once per invocation with required explicit ScalarLimits and reused across selected operations, including helpers; no host-width arithmetic or private numeric semantics appear.

Completed integers move into canonical Value storage and are addressed by a private append-only computed arena. Computed/projection/binding/argument/operand addresses all participate in scratch/native live-element accounting and clear after invoke; a canonical composite result is retained first. Native Boolean order results use existing canonical constants. Incomplete retains LimitKind, exact Integer next_charge and original native outcome/charge point; refusal/undefined retain native cause and source as Scalar invariant evidence. No partial scalar becomes a value or temporal outcome. Resource boxing and non-Copy exact requested fields are explicit public-shape changes; no fallback budget/default is invented. RefCell denial state is only borrowed after synchronous native reservation callbacks return.

Eight scalar controls cover independently authored >128-bit decimal expectations, twelve ordering rows, bounded-domain refusal, invalid annotations, native shortage/source/charge evidence, cumulative result units, inactive scalar branches, left-first native failure and computed scratch. Preserved whole monitor_program target has 52 passing tests and touched Clippy ONLY on the archived interim cohort. RT all-target/all-feature Clippy and three existing native controls pass on current Exact a3052598 / QSV 25535387. Restored committed cohort, our final dependency-head long chain, producer, external coverage acceptance and final aggregate remain unqualified. Same-Machine cleanup after stop remains explicitly open until dispatch. Full phase 2/event/watermark/gap/FR-276 remains unfinished; IEEE has NO backend replay credit.

Plan completion: not assessed

## Verdict

Initial scoped verdict: **PASS**. Current broader readiness remains CONDITIONAL; gap-analysis remains FAIL for unresolved acceptance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: interface-008-AC-1
  path: spec/monitor/functional/interface-008-monitor-scalar-operations.md
  role: examined
  excerpt: Arithmetic and all four order relations agree with independent exact expectations,
    including integers beyond host width and bounded result refusal.
- id: interface-008-AC-2
  path: spec/monitor/functional/interface-008-monitor-scalar-operations.md
  role: examined
  excerpt: Explicit native limits apply cumulatively across selected operations; inactive
    branches do not charge, and denial retains its exact request, native cause and
    source evidence without a value or temporal verdict.
- id: interface-008-AC-3
  path: spec/monitor/functional/interface-008-monitor-scalar-operations.md
  role: examined
  excerpt: Computed values consume scratch; ordinary interpreter stops retain their
    exact units, and a stopped invocation cannot commit state or contaminate later
    execution.
- id: interface_003
  path: spec/monitor/functional/interface-003-monitor-executor.md
  role: examined
  excerpt: 'contains `work: u64`, `stack_frames: usize`, `scratch_elements: usize`
    and

    `scratch_bytes: usize`; zero is a real zero ceiling.'
- id: src/monitor/budget.rs
  path: src/monitor/budget.rs
  role: examined
  excerpt: //! Operating limits, independent of the admitted plan's semantic capacities.
- id: src/monitor/executor.rs
  path: src/monitor/executor.rs
  role: examined
  excerpt: //! Monitor ownership and initializer execution. Entry dispatch shares
    the same machine.
- id: src/monitor/program.rs
  path: src/monitor/program.rs
  role: examined
  excerpt: //! The single retained executable representation, independent of compiler
    storage.
- id: src/monitor/validation.rs
  path: src/monitor/validation.rs
  role: examined
  excerpt: //! Executable integrity checks; source-language admission remains upstream.
- id: src/monitor/vm.rs
  path: src/monitor/vm.rs
  role: examined
  excerpt: //! Iterative execution of ordinary retained expressions, with no source
    evaluator.
- id: src/monitor/mod.rs
  path: src/monitor/mod.rs
  role: examined
  excerpt: //! Owned executable monitor programs (FR-276).
bindings: []
```

Current method verdict after the documented disposition: **PASS**. All broader qualification blockers remain explicit.
