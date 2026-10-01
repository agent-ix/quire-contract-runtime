---
id: "SR-624"
title: "IR-322 spec review: RT spec restructure into subsystems (PR 91)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@b6514fa2b55d64fe7d1320f4d80dbe392f2d3bbe; spec/** (44 renames, spec/spec.md, spec/tests.md, four subsystem matrices), CLAUDE.md, reviews/ir-319-code-review.md; base agent-ix/quire-contract-runtime@75e5ed04d415f47a99bca2db994d1add888495e9"
relationships: []
---

# SR-624: IR-322 spec review of the RT subsystem restructure

## Summary

Ticket: IR-322 (folds in the IR-319 audit record SR-623). PR: agent-ix/quire-contract-runtime#91 at
b6514fa, two commits off main 75e5ed0: c9bb965 (renames only) and b6514fa (registry, matrices, link
fixes, audit review). The PR moves the kind-first RT spec into the subsystem layout of
`ix://agent-ix/quire-contract-ir/ADR-0056`: core, accounting, proptest_adapter and exact. Each
subsystem has a matrix, `spec/tests.md` is the root TestMatrixIndex (TM-005), and `spec/index.md`
becomes `spec/spec.md` with a Subsystem Registry.

This is the base checklist review plus a manual review of the renames and diff. Integrity
(SR-625) and scope-boundary (SR-626) each have their own artifact.

## Method

- `git diff -M --summary` per commit. c9bb965 is 44 pure renames at 100% similarity with zero
  content lines. b6514fa changes 11 files: three one-line `Upstream` link fixes (FR-003, FR-004,
  FR-006), the CLAUDE.md layout line, the registry section in `spec/spec.md`, the four matrices,
  the new root index and the added audit review.
- Ran `make spec` in a clean main worktree and at the head, then `quire coverage --scope . --strict`
  on both, and normalized away the paths before diffing.
- Probed the coder's alternative in a scratch copy of the head with the matrix header renamed
  `Coverage Status` -> `Status`. The probe was never committed.
- Grepped the repository outside `reviews/` and `plan/` for old spec paths, including
  `include_str!`, `include_bytes!` and `env!`. Read the registry against `src/lib.rs`, the
  workspace `Cargo.toml` and the AD-001/AD-002 frontmatter. Read the PR body.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The registry's Owner column reads "Runtime lane" on every row, and no repository document names that owner. AD-001 and AD-002, the ADs this registry links each subsystem to, both carry `owner: runtime-maintainers`. ADR-0056 asks for "the one lane or person that writes the subsystem's specifications". Use the documented `runtime-maintainers`, or cite where "Runtime lane" is defined. | spec/spec.md:68-71, spec/assurance/AD-001-runtime-architecture.md:6, spec/assurance/AD-002-function-application-boundary.md:6 |
| FND-002 | low | The "Owning crates/modules" union is not exactly the workspace's crates and top-level modules, as ADR-0056 requires. It omits the top-level module `kani_proofs` (`src/lib.rs:70-72`, `verification/kani.rs`). It also names `lib`, which is the crate root, and `snapshot_json`, which is a submodule of `accounting` (`src/accounting.rs:6-7`), as if they were top-level modules. | spec/spec.md:68-69, src/lib.rs:70-72, src/accounting.rs:6-7 |
| FND-003 | low | The new Exact row in the root index cites IR-430 as the place the QSL agreement evidence is tracked. SR-623 FND-009, folded into this same PR, records that IR-430 is Done and that the open owners are IR-355/IR-20. The new row repeats a pointer to a closed ticket. | spec/tests.md:19, reviews/ir-319-code-review.md:85 |
| FND-004 | low | The PR body says `make spec` has the "same failure set as main", but not what the header failure hides. The coverage report could not run status checks (status-column-matches-nothing), so 29 unbacked rows and 1 contradicted status (FR-003 ✅ with no backing symbol) are invisible, before and after this PR. A reader can take "same failure set" as harmless. Add one line saying so, and say that IR-365 owns the header fix that will expose them. | PR #91 body |

## Verdict

**Mergeable.** There are no high or medium findings, and the four low findings are text fixes.

- **The renames are clean (claim 8).** c9bb965 has 44 renames at R100 and no content change.
  b6514fa's content edits are small and match its description.
- **The id set is unchanged (claim 1).** Before and after there are the same 45 frontmatter ids
  and 96 distinct AC references. The only additions are TM-002, TM-003, TM-004 and TM-005, and
  there are no duplicate ids.
- **Links resolve (claim 2).** All 21 relative Markdown links in spec/, README, CLAUDE.md and
  AGENTS.md resolve at the head. On main, 20 of 20 resolved.
- **The matrix split is exact (claim 3).** See SR-625.
- **`make spec` is no worse (claim 4).** Before: 2 docs failed, 3 errors. After: 5 docs failed,
  6 errors. The causes are the same two: interface-001 lacks `id` and `features`, and the matrix
  header reads `Coverage Status` (IR-365). The second now appears once in each of 4 matrices.
  `quire coverage --strict` gives 92/124 rows backed both before and after. With paths normalized,
  the unbacked symbols are the same set and the strict failure is the same
  (status-column-matches-nothing), now reported once per matrix.
- **3 -> 6 is acceptable.** The count rises only because one cause is copied into each split
  matrix. No requirement, row or status changed.
- **Neither header option hides anything.** I reproduced the coder's alternative: renaming the
  header to `Status` clears the validate errors in all four matrices. It also turns the strict
  coverage failure into "29 unbacked row(s) and 1 contradicted status(es)", so `make spec` stays
  red but becomes honest about content. That is the better end state.
- **Keep the header as-is in this PR.** The rename belongs to IR-365, which owns it. ADR-0056
  scopes a restructure PR to `git mv` plus link fixes, and keeping the content unchanged makes
  the split checkable row by row. The rule "do not green by hiding" is not broken: nothing is
  removed, and the hiding is pre-existing, is IR-365's subject, and goes when the header is
  renamed.
- **The PR body states the 3 -> 6 replication.** It does not say what the header hides
  (FND-004).
- **Path references are fine (claim 5).** Outside `reviews/` and `plan/`, nothing names an old
  spec path. README line 62 says only "under `spec/`", which is still true, and the PR did not
  need to touch it, contrary to the claim that it was updated. CLAUDE.md's layout line is
  updated. Makefile globs `spec/**/*.md`. scripts/, measurement/, verification/, tests/,
  schemas/ and examples/ contain no spec path. No `include_str!`, `include_bytes!` or `env!`
  points into spec/. There is no `conformance/` directory at the head.
- **Historical records may keep old paths.** Every record in `reviews/` pins its own scope sha,
  including SR-623 at 75e5ed0, so its `spec/test-matrix.md:NN` refs resolve at that sha. Do not
  rewrite them.
- **The registry and the directory tree agree.** Four rows and four directories. The root
  index's Requirements column lists exactly the ids in each directory.
- **The added SR-623 file is byte-identical to the audit artifact** and validates.
- **`quire validate` on `reviews/**`** passes, 10 of 10 documents.
