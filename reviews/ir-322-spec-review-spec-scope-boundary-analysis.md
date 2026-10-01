---
id: "SR-626"
title: "IR-322 scope-boundary analysis: RT subsystem boundaries (PR 91)"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-runtime@b6514fa2b55d64fe7d1320f4d80dbe392f2d3bbe; spec/spec.md Subsystem Registry, spec/{core,accounting,proptest_adapter,exact}/**, spec/assurance/AD-001, AD-002, src/lib.rs, src/accounting.rs, src/exact/, verification/kani.rs, Cargo.toml workspace"
relationships: []
---

# SR-626: RT subsystem boundaries

## Summary

Ticket: IR-322. PR: agent-ix/quire-contract-runtime#91 at b6514fa. This review asks whether the
four subsystems (core, accounting, proptest_adapter, exact) follow the module architecture as
ADR-0056 rules 3 and 4 require, and whether any requirement or test case sits in the wrong
subsystem. It also judges the ticket's expectation that exact would be split into scalar, text,
composite, expression and outcome.

## Method

I read the module declarations in `src/lib.rs` and `src/accounting.rs`, the workspace members in
`Cargo.toml` and the `src/exact/` tree (23 files under one `mod.rs`). Each requirement's
`relationships` and `Dependencies` were checked against the subsystem that holds it. I also
checked which subsystem's behaviour each `verification/kani.rs` harness actually proves.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Core TC-003 carries evidence for two other subsystems. Its harness `tc_003_campaign_accounting_saturates` proves FR-004 campaign counters (accounting), and `tc_003_exact_ieee_numeric_equal_matches_nan_unordered` proves exact IEEE equality (exact). The core matrix's Kani bullet describes the campaign record/discard saturation too. So accounting and exact behaviour is evidenced under a core TC, against ADR-0056 rule 4. This already existed on main; the split makes it visible. A follow-up ticket should retag the two harnesses to accounting and exact TCs. It is out of scope for a git-mv restructure. | verification/kani.rs:201-203, verification/kani.rs:274-277, spec/core/matrix/tests.md:43-48 |

## Verdict

**The boundaries are sound.** The one low finding existed on main and belongs in a follow-up.

- **Exact stays one subsystem (claim 7).** ADR-0056 rule 3 says one crate or module maps to
  exactly one subsystem. `src/exact` is one top-level module (`pub mod exact`, a single
  `mod.rs`), so splitting it into scalar, text, composite, expression and outcome would break
  that rule. The ticket text expected the split, but it was written before the ADR applied. The
  coder's reading is correct and the PR body explains it.
- **FR-004 covers the snapshot transport and sits in accounting.** That is right because
  `snapshot_json` is a `#[path]` submodule of `accounting` (`src/accounting.rs:6-7`).
- **FR-273 (function application) sits in exact.** It is implemented in `src/exact`
  (`// Implements: FR-006, FR-007, FR-273` on `pub mod exact`) and governed by AD-002. The
  registry links exact to AD-001 and AD-002.
- **FR-006 (exact outcomes and accounting) sits in exact.** That is right: it is
  `src/exact/accounting.rs`, not top-level `accounting`.
- **interface-001 sits in core/functional/.** ADR-0056 rule 2 puts interface requirements in
  `functional/`. Its contract spans every feature (default, proptest, snapshot-json, exact), so
  it is a cross-subsystem property. Rule 4 puts that in core, and it has no `relationships` edge
  to another subsystem's requirement.
- **The rest of core's requirements belong there.** NFR-001 and NFR-002 constrain FR-001 and
  FR-002 and are crate-wide. StR-001's only edge is `satisfied_by` FR-001. FR-001 (verdict,
  identity, observation) is consumed by accounting, proptest_adapter and exact. FR-002 is the
  upstream of exact's FR-006.
- **The footprint crate (`measurement/footprint`) belongs to core.** TC-007 audits it for NFR-001.
- **Dependencies run one way.** No core requirement depends on a non-core requirement. FR-003,
  FR-004 and FR-006 depend upward on core's FR-001 and FR-002, and their links were fixed to the
  new paths.
