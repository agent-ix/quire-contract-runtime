---
id: "SR-628"
title: "IR-323 gap analysis: codegen to runtime seam AD (PR 92)"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-runtime@74b40a20c31d73524dfb07f71bd8fd3a775db355; spec/assurance/AD-003-codegen-runtime-seam.md, spec/spec.md; base agent-ix/quire-contract-runtime@215e443f2de81f002336ad5d480a9c476744f4ed"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---

# SR-628: IR-323 gap analysis of AD-003

## Summary

Ticket: IR-323. PR agent-ix/quire-contract-runtime#92 is spec-only. It adds no requirement, test
or production code. Plan completion: not assessed (planless). This pass checks traceability: that
the AD's links resolve, that its cited requirements are the ones it relies on, and that it mints
nothing a matrix would need to cover.

## Method

- Resolved every `relationships` target and every FR, AD and SR id named in the body against
  `spec/**` and `reviews/**` at the head.
- Checked that no requirement id, matrix row or TC was minted (T-1..T-8 are local labels).
- Checked each un-prefixed `path:line` in the AD for which repository it belongs to.
- Checked the `spec/spec.md` registry rows and the References link.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Un-prefixed paths mix two repositories inside one RT document. `src/kani_obligations.rs:2060-2071` in the failure table is a CG file; RT has no such file. `src/spine_replay.rs:16`, `src/oracle.rs:13` and `src/generation.rs` in Current state are CG files, while `src/exact/mod.rs:5-11` beside them is RT. A reader resolves these against RT and fails. Prefix the CG paths (for example "CG `src/...`"), as the dependency table already does for `Cargo.toml:18`. | spec/assurance/AD-003-codegen-runtime-seam.md:121, spec/assurance/AD-003-codegen-runtime-seam.md:175-176, spec/assurance/AD-003-codegen-runtime-seam.md:184 |
| FND-002 | low | The AD relies on FR-001 (Verdict, T-4), FR-007 and FR-008 (exact operators and composite equality in the crossing table) but does not list them in `relationships`, while it does list FR-004, FR-006, FR-009, FR-012 and FR-273. The traceability graph therefore misses three requirements this seam crosses. | spec/assurance/AD-003-codegen-runtime-seam.md:8-22, spec/assurance/AD-003-codegen-runtime-seam.md:55-58, spec/assurance/AD-003-codegen-runtime-seam.md:149 |

## Verdict

**No blocker from gap analysis.** Two low traceability fixes.

- All seven `relationships` targets resolve (AD-001, AD-002, FR-004, FR-006, FR-009, FR-012 and
  FR-273). SR-623 resolves to `reviews/ir-319-code-review.md`.
- No requirement id, matrix row or TC is minted. T-1..T-8 are labelled local, so no matrix
  coverage is owed. T-4, T-5 and T-6 point at existing FRs.
- The registry adds AD-003 to the Core, Accounting, Proptest adapter and Exact rows. All four
  subsystems have items on the seam (verdict, campaign report, proptest harness surface, exact).
  The References link `assurance/AD-003-codegen-runtime-seam.md` resolves from `spec/spec.md`.
- No production code changes, so there is no code without an owning requirement in this diff.
