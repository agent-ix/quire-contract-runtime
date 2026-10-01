---
id: "SR-630"
title: "IR-342 gap analysis: single exact kernel FR-275 (PR 93)"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-runtime@4985ce341b742e8c736be535c48493b76f258e16; spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-197..TC-199, spec/exact/matrix/tests.md, spec/assurance/AD-003-codegen-runtime-seam.md, spec/core/functional/interface-001-runtime-api.md; base agent-ix/quire-contract-runtime@9e07f7a21def0970f7f2b1040f4830ea47de7f54"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-630: IR-342 gap analysis of FR-275

## Summary

Ticket: IR-342. Plan completion: not assessed (planless). PR agent-ix/quire-contract-runtime#93
is spec-only. This pass compares the PR with IR-342's acceptance and with the scope set by the
IR-323 review. IR-342's acceptance reads: "Specify the ownership, the crate boundary and the
migration end state in the owning repos (touches RT, CG, IR); the code move is an M4 ticket". The
IR-323 scope is: delete RT `src/exact`, depend on `quire-exact`, keep no shared-corpus agreement
test, with AD-003 decision F as the source. It also checks the matrix: every new AC is traced and
every row is preserved.

## Method

- Checked each part of IR-342's acceptance against the diff: ownership (FR-275 One kernel, What
  the runtime keeps), crate boundary (Dependency spelling, Guarded edges, interface-001), migration
  end state (Evidence at the kernel move, FR-275-AC-14 and AC-15), and code move deferred to
  IR-349 (Backlog).
- Checked each FR-275 AC against the matrix: AC-1..AC-6 and AC-12..AC-15 go to TC-197, AC-7 and
  AC-8 to TC-198, AC-9..AC-11 to TC-199. All 15 are traced, and each TC file names the ACs it
  verifies.
- Checked that the matrix rows are preserved: the diff adds rows only.
- Checked that no production code changed, so no code lacks an owning requirement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | IR-342's acceptance names three owning repos (RT, CG, IR). This PR specifies the RT part only. The CG part (CG's lock resolves two `quire-exact` copies and three `quire-contract-model` revisions, plus CG's own `quire-exact` edge) is "Routed" to the codegen owner (CG AD-004 step 1d) with no CG PR or ticket id cited. IR is stated as "nothing" without a cited check. IR-342 is not done when this PR merges: name the CG ticket or PR that carries the CG part, and record why IR needs nothing, or keep IR-342 open. | spec/exact/functional/FR-275-single-exact-kernel.md:139-141 |
| FND-002 | medium | The IR-323 scope says delete RT `src/exact` with no vendoring. FR-275 specifies deleting only the items `quire-exact` exports, so a port of QSL code survives the migration end state (about 5,500 of the 12,080 lines sit in `containment.rs`, `definition.rs`, `enumeration.rs`, `expression.rs`, `unit.rs`, `composite.rs`, `mod.rs` and `ieee.rs`, though not all of those are residue). The gap from the ticket is stated openly in Open questions, but no ticket or expiry condition is attached. The policy half is SR-629 FND-001. Here the gap is that IR-342's "migration end state" leaves vendored residue unbounded. | spec/exact/functional/FR-275-single-exact-kernel.md:120-126 |

## Verdict

**Not mergeable on gap analysis alone until FND-001 is answered (a cross-repo carrier for the CG
part) and SR-629 FND-001 is ruled on.** Traceability is otherwise complete:

- All 15 FR-275 ACs map to TC-197, TC-198 or TC-199.
- The matrix and summary rows exist with planned status and a reason (IR-349).
- No row was removed and no id lost.
- `spec.md` (registry and Requirements Architecture) and `spec/tests.md` list FR-275 and
  TC-197..TC-199.
- AD-003 decision F points at FR-275.
- No production code changed.
- The shared-corpus agreement test is excluded (FR-275-AC-12), and so is a `qsl-eval`
  conformance edge.
