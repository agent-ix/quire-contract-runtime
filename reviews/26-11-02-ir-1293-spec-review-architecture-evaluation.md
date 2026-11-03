---
id: SR-20261102049904
title: spec-review/architecture-evaluation — IR-1293 staged runtime architecture
type: SpecReview
analysis: architecture-evaluation
scope: agent-ix/quire-contract-runtime; IR-1293; spec/assurance/AD-001-runtime-architecture.md,
  spec/assurance/AD-002-function-application-boundary.md, spec/assurance/AD-003-codegen-runtime-seam.md,
  spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md, Makefile
review_set: subset
---

## Summary

The implemented default core has borrowed/fixed state and no allocator; optional exact/snapshot dependencies remain explicit. Function admission, planning, root/child charging, explicit frames, typed refusal and shared outcomes match source. Recovery returns typed stops without conflating them with failed verdicts; allocation failure is not promised recoverable. Change scenarios preserve shared semantic ownership and require maintainer review. All sourceNone metadata packages were checked to be inside their owner workspaces; independent normal/build traversal reproduces RT55, IR178, QSL252 and CG263 package closures. RT has no IR/CG/QSL edge. Package-distinct wider repository quotient loops do not establish package cycles or a universal DAG. Runtime-maintainers are explicitly responsible for the manual check, including unavailable-input handling. IR-1009 has a bounded retained scope and observable cutover expiry; IR-590/584 stay deferred. AD-001 and AD-002 are suitable for staged acceptance; AD-003 requires FND-001 closure; AD-004 requires FND-002 closure. Full FR-275 and monitor/proof qualification remain open.

## Verdict

CONDITIONAL.

## Examined scope

```yaml
- id: AD-001
  path: spec/assurance/AD-001-runtime-architecture.md
  role: examined
  excerpt: 'The default boundary is this crate''s safe Rust core: identity, observations,
    verdicts, operators,

    checked integer helpers, and campaign counts. Generated code and callers are outside.
    The optional

    proptest adapter is a development-only boundary; proptest itself is external.'
- id: AD-002
  path: spec/assurance/AD-002-function-application-boundary.md
  role: examined
  excerpt: 'The optional `exact` feature retains the existing function-application
    implementation

    in `src/exact/expression.rs`, exposed by `src/exact/mod.rs`. Exact values, operations,

    outcomes and metering belong to `quire-exact`; type environments, object closures
    and

    stop conversion belong to `quire-semantic-value`. RT is not a second semantic
    authority

    for those shared values.'
- id: AD-003
  path: spec/assurance/AD-003-codegen-runtime-seam.md
  role: examined
  excerpt: 'This description covers codegen as a consumer of runtime''s public surface.
    Runtime is

    a leaf relative to codegen, IR and QSL. It owns default-core identity, observations,

    verdicts, campaign accounting and checked operators, plus opt-in scalar negotiation
    and

    lazy Boolean evaluation. Shared semantic values and operations have their own
    owners.

    Other consumer seams and the pending monitor executor are outside this description.'
- id: AD-004
  path: spec/assurance/AD-004-runtime-crate-layout.md
  role: examined
  excerpt: 'In scope: the files under `src/`, `verification/`, `measurement/` and
    `tests/`; the features; the

    dependency edges and `deny.toml` guards; the owner of every `exact` item; the
    paths in CG that name

    this crate''s `exact` module. Out of scope: kernel behaviour (QSL''s, specified
    by FR-006 to FR-012 as

    behaviour of QSL''s crates), CG''s own layout, IR, and where QSL''s two crates
    live (O-1 of AD-007 in

    `quire-contract-ir`: the owner answered, both are extracted into their own repositories,
    IR-346 and

    IR-582; this crate follows, see the end state).'
- id: FR-275
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: 'The end state remains a runtime with no ported QSL code. This interim
    decision does not

    amend or satisfy the acceptance criteria below. In particular, AC-13, AC-17 and
    AC-18

    remain unmet while the port and its temporary exception remain.'
- id: FR-275-AC-13
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: 'The runtime contains no file copied from the QSL repository and no port
    of QSL code, where a port is code that keeps the QSL authority''s item names and
    order. The residue list is not an allowance: every item on it is a violation of
    this criterion, which stays unmet while the list is non-empty. The runtime-owned
    negotiators are not ports.'
- id: FR-275-AC-17
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Every item the runtime defines under the `exact` feature, other than the
    `scalar` items, is named in the residue list, and the list records no exception,
    expiry or approval.
- id: FR-275-AC-18
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The residue list is empty.
- id: Makefile
  path: Makefile
  role: examined
  excerpt: "deny:\n\t$(CARGO) deny --workspace check licenses bans sources"
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The relevant deny target never invokes the released owner-issued quire check-lock command required by IR-1293 comment 3ff32139; cargo-deny alone does not satisfy that added acceptance. Wire the owned command and demonstrate the scoped gate, reconciling FR-275 prose that currently says it is removed. | Makefile:200-201 |
| FND-002 | low | AD-004's system boundary still describes the exact feature as 23 files, while its current layout and actual source have two expression and four scalar files. Remove the stale population from the current boundary; historical migration counts may remain explicitly historical. | spec/assurance/AD-004-runtime-crate-layout.md:7 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Make deny now invokes the released owned lock command before cargo-deny, with no copied checker. Direct owner command passes; positive whole deny receipt passes and the false-command control stops the target before cargo-deny. |
| FND-002 | fixed | AD-004 system now describes the kept scalar modules and remaining expression port without the obsolete file population. |

## Disposition verdict

PASS. The fix round resolves every finding in this method without a new defect. Recommend changing AD-001, AD-002, AD-003 and AD-004 from proposed to accepted for the staged architecture. This is not approval of the deferred single-owner end state, monitor executor, coverage baseline or backend proofs. The bounded IR-1009 exception and all unchanged FR-275 criteria remain visible. All 24 FR-275 AC rows still exactly match main. Five-document structural validation passes.
