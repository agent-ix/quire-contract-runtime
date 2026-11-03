---
id: SR-20261102049905
title: spec-review/base — IR-1293 staged runtime architecture
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime; IR-1293; spec/assurance/AD-001-runtime-architecture.md,
  spec/assurance/AD-002-function-application-boundary.md, spec/assurance/AD-003-codegen-runtime-seam.md,
  spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-275-single-exact-kernel.md,
  spec/exact/functional/FR-275-single-exact-kernel.md
review_set: subset
---

## Summary

Reviewed the four AD contracts and changed FR-275 interim prose against the installed schema, base checklist and actual source. Structural validation of all five documents succeeds. All 24 FR-275 AC rows are byte-for-byte unchanged from main, including AC13/17/18, explicitly unmet during the exception. No new requirement ID, copied owner type, behavioral criterion, digest ledger, pin or test trace is introduced. The current user assignment supplies the bounded exception decision, not this reviewer. Cross-reference defects are recorded separately in the integrity method. No applicable AssuranceProfile or enforced-run requirement was found.

## Verdict

PASS.

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
