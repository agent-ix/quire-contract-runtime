---
id: "SR-627"
title: "IR-323 spec review: codegen to runtime seam AD (PR 92)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@74b40a20c31d73524dfb07f71bd8fd3a775db355; spec/assurance/AD-003-codegen-runtime-seam.md, spec/spec.md; base agent-ix/quire-contract-runtime@215e443f2de81f002336ad5d480a9c476744f4ed; cross-checked read-only against quire-contract-codegen origin/main 2fad745 and PR 215 (AD-004) at bfaaa84, PR 214 at d8d55ba, quire-contract-ir PR 241 at d784ff7, quire-spec-language origin/main d81193f9, quire-driver PR 11 at 218f51e"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---

# SR-627: IR-323 spec review of AD-003 (codegen to runtime seam)

## Summary

Ticket: IR-323. PR: agent-ix/quire-contract-runtime#92, two commits off main 215e443: 2825953 (the
AD and registry rows) and 74b40a2 (folds in the decisions that CG AD-004 step 1c deletes
`RUNTIME_REVISION`, withdraws R3-C5 and cites IR-352 for R3-C6). The review was started at 2825953
and completed at 74b40a2; every finding is stated at 74b40a2.

This is the base spec review (integrity, ArchitectureDescription structure, consistency with code
and with the sibling seam ADs). Gap analysis is SR-628.

## Method

- Re-measured every claim against origin/main of each repository after `git fetch` (RT 215e443,
  CG 2fad745, IR 0a889f9, QSL d81193f9) with `git show` and `git grep`, read-only. The AD was not
  trusted.
- Counted `unreachable!` per CG file and classified each arm by the enum it names.
- Diffed RT `ed0a04b..ccc722b -- src` (8 files, 12+/16-).
- Read CG PR 215 AD-004 (step 1c, the `RUNTIME_REVISION` row of Decisions taken), CG PR 214,
  IR PR 241 AD-005/AD-006 and quire-driver PR 11 for placement and routing-id collisions.
- Read QSL `tools/arch-lint/graph.rs` (`classify`, `fb05_violations`), the Makefile `ci:` target
  and ADR-011 line 950.
- Ran `make spec` at 2825953, at 74b40a2 and at origin/main 215e443, and `quire coverage --scope .
  --strict` at the head and base.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The AD says six generators write the runtime manifest. CG has three: `src/exact_scalar.rs:2817`, `src/composite_equality.rs:1573` and `src/exact_function.rs:1378` are the only `src/` sites that emit a `quire-contract-runtime` dependency line. The other `RUNTIME_REVISION` users are CG tests (harness, Kani, oracle, bound and witness fixtures). CG AD-004 (PR 215), which this AD now cites for the deletion, says "three emitters", so the two seam ADs contradict each other. The same conflation makes "Features codegen requests: ... `proptest` for generated harness crates" and the Kani crates describe CG test manifests, not generator output. | spec/assurance/AD-003-codegen-runtime-seam.md:62, spec/assurance/AD-003-codegen-runtime-seam.md:67-68, spec/assurance/AD-003-codegen-runtime-seam.md:176 |
| FND-002 | low | "(D-4)" is a dangling reference. The AD's decisions are lettered A to E; there is no D-4 in it, and the only nearby D-4 (IR AD-005) is about model-crate exports. Probably Decision C or D is meant. | spec/assurance/AD-003-codegen-runtime-seam.md:89 |
| FND-003 | low | Decision C still says "(open question for the form)", but the open-question row now says the form is settled in CG AD-004 step 1c (git URL and branch, as CG's own `Cargo.toml`). The AD contradicts itself; Decision C should cite AD-004 instead. | spec/assurance/AD-003-codegen-runtime-seam.md:129-130, spec/assurance/AD-003-codegen-runtime-seam.md:198 |
| FND-004 | low | The 37 count is right (26 + 9 + 2) but it is described wrongly. Most arms are lowering matches that pick an `rt::` path to emit (for example `IeeeWidth`, `OrderingOperator`, `ComparisonOperator` in `exact_scalar.rs`), not conversions to serializable mirrors. The two `exact_function.rs` arms are inside generated source, so the failure row's reporter "the generator" is wrong for them. R3-C6 reads as 37 plus two more (39), when the two are part of the 37. | spec/assurance/AD-003-codegen-runtime-seam.md:120, spec/assurance/AD-003-codegen-runtime-seam.md:179-184, spec/assurance/AD-003-codegen-runtime-seam.md:213 |
| FND-005 | low | The FB-05 rule is stated more narrowly than QSL's lint implements it. `fb05_violations` exempts every normal CG edge into QSL, not only the `qsl-replay` edge; the replay-only restriction is the separate T12-A api-surface rule. The conclusion for an RT edge to `quire-exact` (an FB-05 violation) still holds. | spec/assurance/AD-003-codegen-runtime-seam.md:107 |
| FND-006 | low | The direction row names `scripts/check_one_copy.awk` as holding "runtime to codegen, IR or QSL: no". It is a duplicate-revision check and does not forbid an edge. Also, `make deny` is listed as the live guard, but `ci:` runs `spec` second and `make spec` fails on main, so `make ci` never reaches `deny` today. | spec/assurance/AD-003-codegen-runtime-seam.md:104 |
| FND-007 | low | Under "Invariants a test can check", T-8 is stated as not testable today, and the first half of T-6 ("returns exactly one of the four `Outcome` variants") is true of any enum value and cannot fail. The real property is that the operator returns rather than panics. Reword T-6 and move T-8 to a target, or rename the heading. | spec/assurance/AD-003-codegen-runtime-seam.md:138, spec/assurance/AD-003-codegen-runtime-seam.md:152-153, spec/assurance/AD-003-codegen-runtime-seam.md:156-157 |
| FND-008 | low | The decreases-discharge open question names QSpec as a co-owner, but Routed gaps says "To QSpec: none added by this AD", and R3-Q7 routes it only to QSL. Either route it to QSpec too, or drop QSpec as an owner. | spec/assurance/AD-003-codegen-runtime-seam.md:200, spec/assurance/AD-003-codegen-runtime-seam.md:221-224 |
| FND-009 | low | "Measured at runtime origin/main 215e443 and codegen origin/main 2fad745" is a provenance stamp. This repository's CLAUDE.md says not to add new SHA records. Either mark it once as informational (the cross-repo `file:line` cites rot with it), or drop it and let the commit date carry it. The ed0a04b/ccc722b mentions describe the defect and are fine. | spec/assurance/AD-003-codegen-runtime-seam.md:51 |

## Verdict

**Not mergeable at 74b40a2 until FND-001 is fixed.** It is a wrong measured count, and it
contradicts the sibling AD this one now cites. The eight low findings are text fixes and should
land in the same round. Merge stays gated on QSL sign-off (planner), and the PR is still a draft.

Verified correct at current origin/main:

- AD numbering: AD-001 and AD-002 exist; AD-003 is next. The AD validates clean.
- Pin facts: `RUNTIME_REVISION = ed0a04b...` at CG `src/oracle.rs:13`. CG `Cargo.toml:18` is
  `branch = "main"`, `version = "=0.1.0"`, and the lock resolves RT at ccc722b. RT `src/` differs
  by 8 files and 28 changed lines between the two. The compile tests use the old revision
  (`tests/it/exact_scalar_generation.rs:1424` and others).
- CG AD-004 (PR 215) step 1c deletes `RUNTIME_REVISION` and names the runtime by git URL and
  branch, as the revised text says. No contradiction on direction, and the AD recommends no
  revision or digest record (Decision D forbids one).
- `unreachable!` count: 37 arms name RT `#[non_exhaustive]` enums, at `exact_scalar.rs` 26,
  `composite_equality.rs` 9 and `exact_function.rs` 2 (`:1331`, `:1363`, both emitted). The
  typed `UnknownRuntimeVariant` exists (`generation.rs:85`) and is used at `exact_scalar.rs:1221`
  and `:1269`. IR-352 is the matching ticket.
- `measure_discharged: true` at `exact_function.rs:935` (linked check) and `:1303` (emitted).
  FR-273 states "Nothing links a runtime `CheckedPackage` to an authority proof, and a body is
  arbitrary host Rust".
- There are no `negotiate_ieee` or `negotiate_integer_division` callers in CG `src/`.
- RT `verification/kani.rs:1` imports `crate::exact` unconditionally. CG's
  `tests/it/kani_generation.rs:304` comment records the same.
- RT has 72 `#[non_exhaustive]` sites and `src/lib.rs` documents the Compatibility rule. The
  wire strings, `UnsupportedVersion`, the `InputRefusal` variants, `CheckRefusal`,
  `IdentityMismatch` and `at_limit` exist as stated. `exact/` is 12,080 lines.
- `deny.toml` has `unknown-git = "deny"` and no `allow-git`.
- QSL ADR-011:950 lists "RT -> `quire-exact`" as New. `classify` maps any source containing
  `quire-spec-language` to QSL, and `arch-lint-direction` is outside QSL `ci:`.
- Every CG file:line cited in the crossing table resolves to the stated construct.
- SR-623's FND-001/002/003/007/011 are cited, not restated, and the ticket mapping matches SR-623.
- Placement: RT is a leaf and owns the contract. IR AD-006 (PR 241) points at "quire-contract-runtime
  AD-003" for this seam. CG PR 214 and driver PR 11 do not contradict it. Routing ids R3-C6..C8
  and R3-Q6..Q7 continue IR AD-005/006's R3-C1..C4 and R3-Q1..Q5 with no collision. The gap left
  by the withdrawn R3-C5 is acceptable: the id is retired, not reused. Routed gaps state needs
  with owners and decide nothing for CG or QSL.
- Confidentiality: no research findings, roadmaps, milestones or quire-research internals.
- `make spec`: exits 1 on the same 5 documents at 2825953, at 74b40a2 and at origin/main 215e443.
  These are four `Coverage Status` matrix headers plus interface-001 `id`/`features`. Doc count
  is 77 -> 78, and the new AD is clean. `quire coverage --strict` gives the same
  `status-column-matches-nothing` on head and base. The PR adds no finding.
- The 5 errors are RT's own pre-existing breakage. IR-365 already owns them, but its text
  describes `spec/test-matrix.md` and MP-001. After RT #91 the error is in four subsystem matrices
  and MP-001 no longer fails, so update IR-365's description; no new ticket is needed.
