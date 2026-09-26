---
id: AD-002
title: Function application boundary
type: ArchitectureDescription
status: proposed
owner: runtime-maintainers
system: quire-contract-runtime v0.1
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: realizes
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: realizes
---
# Function application boundary

## System Boundary

FR-273 extends the `exact` feature's boundary to include `quire_contract_runtime::exact::CheckedPackage`
as the call surface for total pure functions: a port of the quire-spec-language authority's own type,
carrying its name and order, the shape FR-006 through FR-008 already establish for scalar, composite,
collection and equality operators. The runtime is no second semantic authority for function bodies:
the authority's `PackageDeclarations::check` proves purity, termination and definedness once, statically, and
`CheckedPackage::{call, evaluate}` alone run checked code under a `Meter`.

## Views

The evaluation view gains one call path: a checked package and a function name or checked expression,
argument values, an `ObjectEnvironment` and a `Meter`, in, one `Result<Evaluation, InputRefusal>` out.
No new stored state, allocator use or I/O crosses the boundary; the call is metered exactly as every
other exact operator is.

## Decisions

Function application is total, never short-circuiting, so it is a separately visible path from the
short-circuit Boolean connectives AD-001 already names. A function whose declared operator
requirements no registered backend can discharge is negotiated by FR-009's own negotiators, before
any application, and reported as the `unsupported` provider disposition FR-009 already defines: one
closed vocabulary for "no registered backend can discharge this," and one negotiation seam for it,
whether the requirement is declared by a bare operator or by a function that reaches one. Provider
dispositions stay outside the evaluation path, as AD-005 and FR-009 keep them. `CheckMode::Kernel`
application stays out of scope, matching FR-008's own deferral of the unlinked evaluation path.

## Risks

The termination and definedness proof burden for every applied function lives entirely in
`PackageDeclarations::check`, outside this crate's own Kani proof surface; this crate's assurance
depends on that upstream proof rather than reproducing it. The call surface is pinned to
quire-spec-language by commit sha, so a later quire-spec-language release that changes
`CheckedPackage`'s signature requires a coordinated re-pin and re-port, the same dependency FR-008
already carries for composite, collection and equality evaluation. `Value` and `ValueType`'s layout
on this crate's own Kani proof surface (an explicit tag, and no inline payload wider than `Integer`
or `IntegerInterval`) is guarded by compile-time assertions in `src/exact/composite.rs`, not by
construction discipline alone.

Two more representation rules on this crate's Kani proof surface have no owning FR or NFR of their
own, same as the layout rules above: `CheckedPackage::enter` returns `Option<DepthGuard>` rather than
`Result<DepthGuard, Stop>`, because `Stop`'s niche is where CBMC cannot fold a written discriminant
back (`src/exact/expression.rs`); a signature check next to `enter` pins its exact return type.
`evaluate_integer_arithmetic` builds its `Outcome<Integer>` directly rather than through an inner
`Result<Integer, Stop>` round-trip, for the same reason (`src/exact/numeric.rs`); a source-inspection
test (`tests/exact_outcomes.rs`) pins that its body never reintroduces one.
