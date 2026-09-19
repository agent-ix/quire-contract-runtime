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

FR-273 extends the `exact` feature's boundary to include `quire_spec_language::value::CheckedPackage`
as the call surface for total pure functions. The runtime authors no execution engine of its own for
function bodies: `PackageDeclarations::check` proves purity, termination and definedness once,
statically, and `CheckedPackage::{call, evaluate}` alone run checked code under a `Meter`, matching
the shape FR-006 through FR-008 already establish for scalar, composite, collection and equality
operators.

## Views

The evaluation view gains one call path: a checked package and a function name or checked expression,
argument values, an `ObjectEnvironment` and a `Meter`, in, one `Result<Evaluation, InputRefusal>` out.
No new stored state, allocator use or I/O crosses the boundary; the call is metered exactly as every
other exact operator is.

## Decisions

Function application is total, never short-circuiting, so it is a separately visible path from the
short-circuit Boolean connectives AD-001 already names. An undischargeable capability reached from
inside a called function's body is reported as the same `unsupported` provider disposition FR-009
defines for its own operators, not as a new disposition kind: one closed vocabulary for "no
registered backend can discharge this," wherever in a call graph it is reached. `CheckMode::Kernel`
application stays out of scope, matching FR-008's own deferral of the unlinked evaluation path.

## Risks

The termination and definedness proof burden for every applied function lives entirely in
`PackageDeclarations::check`, outside this crate's own Kani proof surface; this crate's assurance
depends on that upstream proof rather than reproducing it. The call surface is pinned to
quire-spec-language by commit sha, so a later quire-spec-language release that changes
`CheckedPackage`'s signature requires a coordinated re-pin, the same dependency FR-008 already
carries for composite, collection and equality evaluation.
