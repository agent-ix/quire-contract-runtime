---
id: SR-35015
title: "Integrity review of IR-349 RT PR 159 (AD-004 delivery split to IR-1009)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime; PR #159 at 56fd812cad256616ed5ca92a76f6fa66ef501320 against origin/main 754e2566003295c54a4c96b8bcc3928de3e6e8b2; spec/assurance/AD-004-runtime-crate-layout.md; spec/core/matrix/tests.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/matrix/TC-197-single-kernel-ownership.md; spec/exact/matrix/tests.md; spec/core/functional/interface-001-runtime-api.md (context)"
review_set: subset
---

## Summary

Ticket: IR-349. This checks whether the IR-349 to IR-1009 re-ownership is
applied consistently across the repository at head 56fd812. `quire validate`
exits 0 on the five changed files, and coverage is unchanged from main
(93 unbacked, 0 contradicted). The diff edits no AC row, and no matrix row's AC
or TC list changes.

Inside the diff, the change is consistent: AD-004, FR-275, TC-197 and both
matrix files all assign steps 1-4 to IR-349 and step 5 plus the end state to
IR-1009. Two places still say IR-349 owns end-state inspection, and the PR
did not update them.

## Scope examined

- AD-004 delivery paragraphs and migration rows 5/6: examined.
- FR-275 lines 37, 53-54, 71, 225-226, 266-273: examined.
- TC-197 Description: examined.
- spec/exact/matrix/tests.md FR-273 AC-12/13, FR-275 AC-1..19, FR-275 AC-29/30, TC-194, TC-197 rows: examined (AC/TC columns compared with main: identical).
- spec/core/matrix/tests.md interface-001 rows 23 and 24: examined.
- interface-001 lines 115 and 168: examined (outside diff).
- AD-004 Q-1 row (line 561) and TC-032 line 20: context only, consistent.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The second interface-001 row (AC-2/6/8/10/11/14), in the same matrix file the PR edits, still says "The other listed criteria require separate end-state inspection under IR-349". The PR moves end-state inspection to IR-1009, and AC-10 names the `exact` module, which only IR-1009 removes. Re-own that sentence the way row 23 was. | spec/core/matrix/tests.md:24 |
| FND-002 | low | interface-001 still labels the end state "(FR-275; planned, IR-349)" twice, in prose and in the contract yaml comment. The PR now splits that end state between IR-349 and IR-1009. | spec/core/functional/interface-001-runtime-api.md:115,168 |

## Verdict

Changes requested. Two small consistency edits outside the lines already
touched. AC preservation passes: no criterion is reworded, narrowed or
deleted, and the planned status of every affected row is kept.
