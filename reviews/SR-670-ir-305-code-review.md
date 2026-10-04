---
id: "SR-670"
title: "IR-305 code review: CI tool versions (PR 98)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@c3b1345ae53162b4e863969e679daa21354507a4; .github/workflows/ci.yml; base origin/main e88c068"
---

# SR-670: IR-305 code review of PR 98

## Summary

Ticket: IR-305. PR agent-ix/quire-contract-runtime#98 changes one line in one file:
`.github/workflows/ci.yml:33`. It moves the `npm install --global` line in the `Validate specification`
step from `@agent-ix/quire-cli@0.31.0`, `@agent-ix/quoin@0.23.1` and `ix-flow@0.0.4` to `0.33.0`,
`0.24.1` and `0.2.3`. The owner approved exactly that edit, versions only. This is a config-scope
code review plus a short integrity check. No `spec/`, `plan/` or production code changed, so
gap-analysis and spec-review do not apply.

## Method

- `git diff origin/main...c3b1345 --stat`: 1 file, 1 insertion, 1 deletion. `cat -A` shows LF
  endings and no trailing whitespace. No other workflow line changed, and there is one commit
  on top of the merge-base e88c068, which is the current origin/main.
- Read the whole workflow. It triggers on `workflow_dispatch` only. It has no `actions/setup-node`,
  no `registry-url`, no `NODE_AUTH_TOKEN`, and the repo has no `.npmrc`. So the runner's npm
  resolves every name against the default registry, registry.npmjs.org, with no auth. It does
  not use GitHub Packages.
- `npm view` against registry.npmjs.org, with the `@agent-ix` scope overridden because the local
  scope points at npm.ix:
  - `@agent-ix/quire-cli@0.33.0` is published (latest 0.36.0).
  - `@agent-ix/quoin@0.24.1` is published, with bin `quoin` (latest 0.28.0).
  - `@agent-ix/ix-flow@0.2.3` is published, with bin `ix-flow` (latest 0.2.3).
  - The old `0.31.0` and `0.23.1` are published too.
  - Unscoped `ix-flow` is E404, because no such package exists at any version.
- `gh api /orgs/agent-ix/packages?package_type=npm` lists `ix-flow` (internal) and no quire-cli or
  quoin. GitHub Packages is not the registry this workflow reads.
- Grepped the workflow, Makefile and repo for `ix-flow`. Its only occurrence is the install line.
- `make spec` at the head, in a detached worktree:
  - With the local quire 0.33.0, it exits 2: validate passes, then strict coverage reports
    "56 unbacked row(s) and 5 contradicted status(es)".
  - With quire-cli 0.31.0 (via npx from registry.npmjs.org), it also exits 2, with the same
    56/5 and an identical finding list. Only two module warnings differ.
- `gh pr checks 98` lists only the CLA checks. The CI workflow did not run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The unscoped name `ix-flow@0.2.3` does not exist on registry.npmjs.org (E404), which is the only registry this workflow can reach (no setup-node, no .npmrc). The published package is `@agent-ix/ix-flow` (0.2.3 is latest). Because `npm install --global` installs the whole line or nothing, a dispatched run fails at install, before `make spec`. This is pre-existing: `ix-flow@0.0.4` was equally unresolvable. The PR carries it forward and does not introduce it. Nothing in the workflow or the Makefile invokes ix-flow. The fix is outside the approved versions-only scope, and the owner must decide it: drop `ix-flow` from the line, or rename it to `@agent-ix/ix-flow@0.2.3`. | .github/workflows/ci.yml:33 |
| FND-002 | low | The PR body's caveats are inaccurate, though it claims nothing false about the diff. (a) It says the registry check "could not be done" and that GitHub Packages "does not list them". CI reads registry.npmjs.org, where quire-cli 0.33.0 and quoin 0.24.1 are published, and that was checkable. (b) It says "the PR's automatic checks are the evidence for whether these resolve". The workflow is `workflow_dispatch` only, so no PR check exercises the line. (c) It does not mention that the unscoped `ix-flow` is a 404 on the registry CI uses. | PR #98 body |

## Verdict

The diff is exactly the approved single-line, versions-only edit. It has nothing extra, no
line-ending change and no other workflow change. The title has no ticket id, and the body says
"Part of IR-305" with no `Closes`.

The `@agent-ix/quire-cli@0.33.0` and `@agent-ix/quoin@0.24.1` pins resolve on the registry CI
uses. The edit does not change the `make spec` outcome: on this tree, quire 0.31.0 and 0.33.0 both
exit 2 with the same 56 unbacked rows and 5 contradicted statuses (IR-499). The gaps were exposed
by the RT #97 header repair, not by the version change.

FND-001 is pre-existing and needs an owner decision, so it does not block this approved edit.
FND-002 is a body-text correction. The PR is mergeable as the approved change. A dispatched CI run
would still fail at install until FND-001 is resolved.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d5e3cb6: `.github/workflows/ci.yml:33` now installs `@agent-ix/ix-flow@0.2.3`, which is published on registry.npmjs.org with bin `ix-flow`; no other workflow line changed |
| FND-002 | fixed | d5e3cb6 (PR body edited on GitHub, not in git): every caveat now matches measurement (npmjs publishes quire-cli 0.33.0, quoin 0.24.1 and @agent-ix/ix-flow 0.2.3; unscoped ix-flow is absent; make spec is red with 0.31.0 too; dispatch-only, so `gh pr checks` shows only CLA) |
