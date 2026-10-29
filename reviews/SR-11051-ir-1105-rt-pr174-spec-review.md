---
id: SR-11051
title: "IR-1105 RT PR 174 specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime PR 174: AGENTS.md, CLAUDE.md, plan/campaign-snapshot-implementation.md, spec/assurance/AD-003-codegen-runtime-seam.md, spec/assurance/AD-004-runtime-crate-layout.md"
review_set: base
---

## Summary

Reviewed the Markdown-only diff at 11c63e15 against the stated removal of frozen revision guidance and measured CG's current manifest emitter. The deleted tolerance sections and revised dependency text are consistent, but AD-003 retains stale instructions and claims about a removed runtime revision constant.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AD-003 still says `RUNTIME_REVISION` is to be deleted and that CG has two runtime revisions and a stale SHA, although CG main emits a Git branch with no `rev`; remove or restate all three stale rows. | AD-003:133-135, 170-171, 224-226; CG `src/core/profile.rs:9` |

## Evidence

CG main `src/core/profile.rs:9` states the generated dependency table has `git` and `branch` keys, with no `rev`; its TC-024 at line 88 checks that behavior. The PR itself changes AD-003's manifest row and dependency-selection row to this current behavior at lines 69 and 89-91. The retained decisions, invariant, and open-question row contradict those changed lines and can misdirect future work into treating a removed frozen revision as live.

Scoped `quire validate --scope .` on the two changed AD documents and the changed plan exited successfully. No Rust or full build gate applies to this review.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At d4e4196d, AD-003 decision C and T-2 name the current Git URL and branch selection; the obsolete open-question row is deleted. |
