---
id: AD-002
title: Function application boundary
type: ArchitectureDescription
status: proposed
owner: runtime-maintainers
system: quire-contract-runtime v0.1
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: realizes
---
# Function application boundary

## System Boundary

The optional `exact` feature retains the existing function-application implementation
in `src/exact/expression.rs`, exposed by `src/exact/mod.rs`. Exact values, operations,
outcomes and metering belong to `quire-exact`; type environments, object closures and
stop conversion belong to `quire-semantic-value`. RT is not a second semantic authority
for those shared values.

The remaining expression implementation is a temporary staged exception under
[IR-1009](https://linear.app/agent-ix/issue/IR-1009), recorded by IR-1293. It preserves
existing function application while the usable shared `no_std + alloc` evaluator is
deferred. IR-590 and IR-584 are not newly planned or staffed by this decision. This
exception authorizes no new evaluator copy, semantic expansion or compatibility layer.

## Views

The caller supplies declarations and generated resumable body states. Admission through
`PackageDeclarations::check` refuses unsupported checking mode, duplicate names and
undischarged measure declarations before publishing a callable package. RT does not
inspect an upstream AST or prove an arbitrary host body pure, terminating or defined;
those are the upstream admission obligations. `CheckingLimits::nodes` is carried but
cannot count a generated body that exposes no AST.

`plan_call` and `plan_evaluation` validate arguments and the checked-expression/package
association without taking a meter. `CheckedPackage::call` validates first, charges one
`function.call` work unit, then starts the generated body. `evaluate` starts its root
without that call charge and charges requested child calls. Both retain typed
`InputRefusal` before execution and the shared `Outcome` after execution.

Execution owns suspended body states in an explicit heap vector and resumes one
transition at a time. A body transition requests a child call or returns an outcome;
it does not synchronously invoke another checked body. The shared meter is borrowed
through `RefCell`; package reentry or an incompatible borrow is a typed invariant
refusal. Heap allocation is part of this opt-in `alloc` feature. There is no promise
that allocator exhaustion becomes a recoverable result. RT introduces no ambient I/O.

## Decisions

Function calls are distinct from the runtime-owned lazy Boolean connective. Backend
negotiation remains in `scalar` under FR-009 and outside function evaluation. Only
`CheckMode::Linked` admits a callable package; `Kernel` is refused. Caller-supplied
upstream-discharge markers are admission inputs, not proofs reconstructed by RT.

The existing port and its public path remain only within IR-1009's recorded exception.
The exception expires when the shared evaluator implementation/ownership lands and
passes RT's governed `no_std` target acceptance for checked-input and generated-body
execution. The RT/paired-CG adoption removes the local expression implementation and
old public path in that cutover. An interface alone does not satisfy this expiry.
The end state has one shared owner; it introduces no alias, fallback or compatibility
wrapper. FR-273 and FR-275's deferred end-state criteria retain their meaning and
remain open until their actual evidence lands.

Runtime-maintainers own the exception review when this source, a dependency or shared
evaluator availability changes. AD-003 defines the reviewed dependency-direction check.

## Risks

A generated host body is not an admitted source AST, so the runtime cannot independently
establish its semantic proof obligations. The current `Evaluation` paths return no
location and an empty losses list; this architecture acceptance does not claim otherwise.
The package-instance counter is finite and wraps at its integer width; it is not a
permanent content identity. These limits remain visible during the IR-1009 staged period.

The explicit scheduler and shared payloads have native controls under FR-273/FR-278;
architecture review is not a Kani or IEEE qualification. Former CBMC-oriented
`DepthGuard` and inline-value-size descriptions do not describe the current source.
