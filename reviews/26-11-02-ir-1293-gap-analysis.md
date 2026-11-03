---
id: SR-20261102049907
title: gap-analysis — IR-1293 staged runtime architecture
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-runtime; IR-1293; spec/assurance/AD-001-runtime-architecture.md,
  spec/assurance/AD-002-function-application-boundary.md, spec/assurance/AD-003-codegen-runtime-seam.md,
  spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md
review_set: subset
---

## Summary

Plan completion: not assessed

Computed strict coverage reports 112 unbacked rows, zero contradicted statuses and zero untracked symbols with the installed CLI; it is not green and this review is not a repository-wide coverage acceptance. The changed source is documentation only and all existing criterion rows and test tags are unchanged, so the PR introduces no test-backed claim or false closure. Existing deferred inspection/removal/monitor work remains with IR-499, IR-1009 and IR-722; no dummy tags or copied owner tests are added. Reverse-gap and hollow-evidence inspection of the changed behavior found no new implementation surface: the port remains explicit and unqualified. Full implementation/backend qualification is outside the architecture acceptance.

## Verdict

PASS (change scope only).

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
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
