---
id: SR-012
title: "Spec review — FR-274, TC-196 and StR-002 repository-local ID and relocation-map check"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@98566b105e98504cbb648bbb44b4066ce9e22159; PR #80 diff against origin/main ed0a04b: spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md, spec/test/TC-196-spec-artifact-id-and-relocation-check.md, spec/stakeholder/StR-002-machine-checkable-specification.md, spec/test-matrix.md, spec/index.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-274
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/TC-196
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/StR-002
    type: reviews
---

# SR-012: Spec review — FR-274, TC-196 and StR-002

## Summary

The PR is spec-only. It adds StR-002, FR-274 with 22 ACs, and TC-196. It also adds one coverage
row and one Test Case Summary row to `spec/test-matrix.md` and edits a paragraph of `spec/index.md`.
No ticket id appears in the branch name or the PR title.

Sub-analyses run: EARS conformance, integrity (ACs against Behavior and against the peer
requirement), and evidence (TC-196 steps against the ACs).

Gates, run by the reviewer on clean detached worktrees:

- `make spec` exits 2 on both the base and the head, with the same 3 pre-existing structural
  failures: MP-001, interface-001 and the test-matrix `Coverage Status` column. Grammar is clean at
  the head: 102/102 docs.
- `quire coverage --scope . --strict` exits 1 on both (pre-existing
  `status-column-matches-nothing`). Backed rows go from 127/152 to 127/176. The 24 new unbacked rows
  are FR-274-AC-1..22, StR-002-VC-1 and TC-196.

Identifiers FR-274, TC-196 and StR-002 are unused on `origin/main` and in the other open PR (#78).
The diff has no dates, ticket ids, work-package tokens or private-repo paths. The index's TC range
(TC-001..TC-036, TC-194..TC-196) matches the files. The matrix rows are marked `🚧 planned`, which
is honest. The pre-existing `Coverage Status` column mismatch means `--strict` skips status
classification entirely, so the new rows' status is not checked by the tool. That is not introduced
by this PR.

## Verdict

**NOT MERGEABLE until FND-001 is fixed.** FR-274-AC-21 contradicts FR-274's own definition of the
identifier set, so an implementation that follows the Behavior section fails TC-196 step 12. The
medium findings (FND-002..FND-006) are real gaps and should be fixed in this PR as well.

Examined and clean: FR-274-AC-1..11, AC-13, AC-14, AC-17..19, the StR-002 need and its trace
relationships, the `spec/index.md` paragraph, and both matrix rows. The ACs pass EARS grammar.
Every AC has a TC-196 step.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | high | FR-274-AC-21 contradicts FR-274's own identifier-set definition. The Description defines the set as the *union* of frontmatter ids and TC ids that lead Test Case Summary rows. A `TC` whose summary row is removed while its `TC-###` file remains is still in that union through the file's frontmatter. So the set-equality rule in Behavior passes, but AC-21 requires a failure "even when that TC's artifact file remains". No Behavior clause produces AC-21. A faithful implementation fails TC-196 step 12. Fix: state two sets, as ADR-0056 restructure-gate rule 4 does. Frontmatter-id equality applies under map substitution. Separately, the live-TC set (TC ids that lead summary rows) must be identical, with each TC declared once. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:32-33, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:73-81, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:110 |
| FND-002 | medium | StR-002-VC-1 claims that a change which "loses one [identifier] in a move fails `make spec`". FR-274 checks moves only when `SPEC_BASE` is set, and nothing states that `make spec` sets it: AC-20 and the Description only say `make spec` runs the check. A plain `make spec` on a structural PR therefore never runs AC-12/13/15/16/21, which leaves unchecked exactly the silent loss StR-002 exists to catch. TC-196 step 15 exercises only the duplicate-id half of VC-1 through `make spec`. No step shows a lost identifier failing `make spec`. The "before it merges" clause also names no gate that enforces timing. Fix: either state how `make spec` supplies `SPEC_BASE` (for example the merge base with `origin/main` when the caller sets none) and add a TC-196 step for it, or narrow VC-1 to what the FR delivers. | spec/stakeholder/StR-002-machine-checkable-specification.md:26, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:109, spec/test/TC-196-spec-artifact-id-and-relocation-check.md:63-65 |
| FND-003 | medium | FR-274 says the check "behaves as `quire-contract-ir:FR-345` does over that repository's tree", and Dependencies says FR-345 "specifies the same check". FR-274 deliberately diverges in four ways. It validates `new_path`/`new_id` only for added maps, while FR-345-AC-4 validates them on every map. It excludes `planning/`. It puts summary-row TC ids in the identifier set. It reports `path:line` rather than paths. Two implementers would disagree on which text governs, for example on a historical map whose `new_path` has since moved (AC-22 versus FR-345-AC-4). Fix: drop the "behaves as" and "same check" claims, or name the differences. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:24, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:118-119 |
| FND-004 | medium | FR-274-AC-20 ("When the check finds any defect, `make spec` exits non-zero") is already true today for any tree: `make spec` exits 2 at both base and head because of 3 pre-existing structural failures. As worded, no test can fail it until those are repaired. The recipe also stops at the first failing command, so a check placed after `quire validate` would never run while validate is red. TC-196 step 15 adds "whose output carries the check's finding", which the AC does not require. It also says "run `make spec` over a tree" without saying how this repository's `make spec` targets a scratch repository. Fix: make AC-20 require that `make spec` runs the check and that its output carries the check's finding. State how step 15 points `make spec` at the scratch tree. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:109, spec/test/TC-196-spec-artifact-id-and-relocation-check.md:63-64, Makefile:157-160 |
| FND-005 | medium | FR-274-AC-16 has two branches: a renamed file with no row, or with more than one row across the added maps. TC-196 step 11 seeds only the no-row branch ("omits ... a renamed file"). One rename listed in two different added maps passes AC-11 (repeats within one map only), and no step exercises it. Fix: add a step that lists one rename in two added maps. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:105, spec/test/TC-196-spec-artifact-id-and-relocation-check.md:52-55 |
| FND-006 | medium | "Renamed under `spec/` between `SPEC_BASE` and the working tree" (AC-16 and Behavior) does not define what a rename is. Git rename detection is similarity-based. ADR-0056 lets a moved file edit its link targets, so a short file with several link edits can fall below the similarity threshold and show as a delete plus an add. Its frontmatter id is unchanged, so ID-set equality (AC-15) also passes, and a move with no map row goes unreported. Paths that leave `spec/` (for example into root `reviews/`) are also ambiguous under "renamed under `spec/`". Fix: define a rename. One option is a path under `spec/` at `SPEC_BASE` that is absent from the working tree while its frontmatter id or content appears at a new path; another is to name the detection rule and threshold. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:78-80, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:105 |
| FND-007 | low | FR-274-AC-22 covers only maps "present at `SPEC_BASE`". The Behavior clause's second half ("or of any relocation map when `SPEC_BASE` is not set") has no AC, yet TC-196 step 10 tests that case under AC-22. Fix: extend AC-22 to the unset case, or add an AC for it. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:72-73, spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:111, spec/test/TC-196-spec-artifact-id-and-relocation-check.md:48-51 |
| FND-008 | low | The Behavior clause "The check SHALL NOT report a defect as a warning" has no AC. It is only implied by the exit-status ACs, and no TC-196 step checks that a defect is never emitted with a warning label and a zero exit. | spec/functional/FR-274-check-spec-artifact-ids-and-relocation-maps.md:84 |
