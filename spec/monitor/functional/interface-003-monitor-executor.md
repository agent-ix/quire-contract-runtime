---
id: interface_003
title: "Owned monitor initializer and bounded entry machine"
type: interface
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-276
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface_002
    type: references
---
# [interface-003] Owned monitor initializer and bounded entry machine

## Contract

```yaml
name: MonitorExecutor
ownership: quire-contract-runtime
feature: exact
associated_types:
  - BuildPlan
  - StepBudget
  - StepResource
  - StepResourceStop
  - Monitor
  - MonitorBuildFailureCode
  - MonitorBuildFailure
  - ExecutionSite
  - ExecutorInvariantCode
  - ExecutorInvariantFailure
  - AdmissionFailure
operations:
  - name: build_monitor
    inputs: [plan]
    output: Monitor
    semantics: retain the fixed owner, prepare bounded scratch, and execute init once
  - name: inspect_monitor_state
    inputs: [monitor]
    output: Value
    semantics: borrow the complete canonical state returned by init
  - name: inspect_monitor_program
    inputs: [monitor]
    output: Program
    semantics: borrow the fixed retained owner without a rebind operation
  - name: inspect_step_budget
    inputs: [monitor]
    output: StepBudget
    semantics: inspect explicit operating limits independent of semantic premises
  - name: replace_step_budget
    inputs: [monitor, budget]
    output: Monitor
    semantics: replace operating scratch and limits atomically without changing state or owner
invariants:
  - the machine stores operand addresses rather than mirrored semantic values
  - no source package parser, formula reconstruction or host body callback
  - shared membership and shared type equality own conformance
  - execution visits the selected entry path, not a package-wide node census
  - operating stops are not temporal outcomes or executor invariant failures
```

## Operations

### build_monitor

`monitor::build(BuildPlan { program, budget })` returns `Result<Monitor,
MonitorBuildFailure>`. The input Program is the immutable owner of interface-002;
the reader/driver has already admitted and independently bound its complete program.
Build consumes no event, produces no obligation result and runs the selected init
entry once. Each successful build owns a separate mutable state replacement slot.
Immutable canonical Arc-backed values may share their underlying storage.

Failure returns no usable partial monitor. Allocation or budget exhaustion uses the
resource channel; a broken evaluated program uses the invariant channel; the shared
conformance boundary retains its native typed failure and resolvable path arena.
No arbitrary allocator-OOM recovery is promised.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| plan | BuildPlan | 1..1 | |

Returns: Monitor[1..1]

BuildPlan has exactly `program: Rc<Program>` and `budget: StepBudget`. StepBudget
contains `work: u64`, `stack_frames: usize`, `scratch_elements: usize` and
`scratch_bytes: usize`; zero is a real zero ceiling. These are operating controls,
not replacement capacities, finite semantic domains or host-width event counters.
The State value, including its exact Integer event_position, remains canonical.

One continuation dispatch costs one work unit. A nontrivial conformance walk charges
the shared supplied-membership events from the same remaining work budget. The
explicit continuation vector bounds stack use. Scratch elements bound the live
operand-address slots plus cumulative native membership element reservations;
scratch bytes bound cumulative native membership byte reservations. Actual native
reservation units are preserved. Scratch vectors are prepared at build; a controlled
reservation may stop later shared membership before it grows storage.

Immutable already-admitted literals and arguments retain their type facts. Shared
`same_type` equality for each relevant expression edge is cached once at build, so
identical-type helper calls do not repeatedly walk the entire State. A nonidentical
edge invokes shared bounded value admission; no second subtyping checker is defined.
The equality caches contain only booleans over retained edges and are not another
executable representation. Preparing them is linear in the supplied relevant nodes
and edges, apart from the shared type comparisons themselves. No per-step package
scan, key sort or whole-function census occurs.

The initial machine executes the existing Literal, owning Argument, Call and lazy If
operations iteratively. Recursive calls spend the same budget and cannot recurse on
the Rust stack. Further capabilities follow the actual producer closure (QSL #2106),
using ordinary shared operations; no append or temporal opcode fills a gap. Event,
watermark and gap dispatch remain the rest of IR-722; this initializer slice does not
claim the complete FR-276/TC-200 acceptance.

### inspect_monitor_state

`Monitor::state()` borrows the exact shared Value currently owned by this monitor.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| monitor | Monitor | 1..1 | |

Returns: Value[1..1]

### inspect_monitor_program

`Monitor::program()` borrows its Rc owner. There is no live rebind method.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| monitor | Monitor | 1..1 | |

Returns: Program[1..1]

### inspect_step_budget

`Monitor::step_budget()` returns the selected operating limits.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| monitor | Monitor | 1..1 | |

Returns: StepBudget[1..1]

### replace_step_budget

`Monitor::set_step_budget(budget)` prepares replacement scratch before changing any
live field. An allocation failure leaves the old budget, state and owner intact.
This is an operating-control change, not plan rebinding or source semantics repair.

| Param | Type | Multiplicity | Constraints |
| --- | --- | --- | --- |
| monitor | Monitor | 1..1 | |
| budget | StepBudget | 1..1 | |

Returns: Monitor[1..1]

The Rust method returns `Result<(), MonitorBuildFailure>` and mutates only on success.

## Features

| Feature | Kind |
| --- | --- |
| build_monitor | operation |
| inspect_monitor_state | operation |
| inspect_monitor_program | operation |
| inspect_step_budget | operation |
| replace_step_budget | operation |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| interface-003-AC-1 | A retained admitted program builds after preparation drops, executes its actual initializer and keeps its complete canonical State and fixed owner. Two builds have independent state slots, consume no event and produce no obligation result. | Test |
| interface-003-AC-2 | Zero or insufficient work, stack and scratch limits stop initialization through the resource channel and return no Monitor. A lazy If does not execute its unselected branch; a helper chain spends bounded explicit continuations without Rust recursion. | Test |
| interface-003-AC-3 | A dynamically wrong function result or operand reaches the shared conformance boundary and returns its typed failure rather than a false property result. Native limit/storage failures preserve the operating-stop channel. | Test |
| interface-003-AC-4 | Replacing operating limits leaves the canonical State and fixed Program unchanged; a failed replacement reservation preserves the old limits. No compatibility default or live rebind is exposed. | Test |
