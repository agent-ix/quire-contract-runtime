---
id: "SR-651"
title: "IR-342 gap analysis: remove the vendoring exception wording (PR 96)"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-runtime@fb2274498efc0b221d97c7e49024500740b4c0b2; spec/exact/matrix/tests.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/core/functional/interface-001-runtime-api.md, tests/exact_function_application.rs, src/exact/; base origin/main"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-651: IR-342 gap analysis of PR 96

## Summary

Ticket: IR-342. Plan completion: not assessed (planless). PR agent-ix/quire-contract-runtime#96
is spec-only and changes no code. This pass checks the matrix rows that changed against the
tagged tests and the source.

## Method

- Confirmed that no file under `src/`, `tests/`, `Cargo.toml`, `deny.toml`, `scripts/` or
  `.github/` changed, so no code lacks an owning requirement.
- Checked the FR-275 and TC-197 matrix rows. TC-197 is planned and has no `tc_197` tag, which is
  consistent. The 14 FR-275 ACs traced to TC-197 match TC-197's procedure, steps 1 to 8.
- Mapped the `tc_194` and `tc_195` tests in `tests/exact_function_application.rs` to the items
  they exercise, to check the rewritten disposition rows.
- Confirmed that `src/exact/mod.rs` still defines and re-exports the residue (`Frame`,
  `CheckedPackage` and the others). This matches the matrix: AC-16, AC-17 and AC-18 are planned
  and unmet.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The rewritten disposition row for `tests/exact_function_application.rs` (TC-194, TC-195) says the whole file "leaves with the residue". But TC-195's `tc_195_negotiate_ieee_takes_no_meter_by_signature`, `tc_195_undischargeable_function_capability_negotiates_unsupported` and `tc_195_dischargeable_requirements_negotiate_supported_or_requires_bound` exercise `negotiate_ieee`. That predicate is runtime-owned and stays, under FR-275-AC-19 and interface-001-AC-8, and TC-195 also verifies FR-009-AC-5. Following the row deletes the evidence for code the runtime keeps. The row should be "splits". This is pre-existing since #93 ("stays until QSL-358 phase 2"), and this PR rewrote the row without fixing it. | spec/exact/matrix/tests.md:103, tests/exact_function_application.rs:1206-1308 |

## Verdict

**Mergeable on gap analysis.** FND-001 is low and pre-existing, and can be deferred to IR-349,
which carries out the dispositions. Traceability for the changed rows is intact: every FR-275 AC
is still traced, no row or id was removed, and the planned and unmet statuses match the code at
head, where the residue is still present.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f28783b: the `tests/exact_function_application.rs` row is now "splits". The TC-194 function-application tests are residue and leave with the code. The TC-195 tests exercise `negotiate_ieee`, which is runtime-owned and stays (FR-275-AC-19, FR-009-AC-5). |
