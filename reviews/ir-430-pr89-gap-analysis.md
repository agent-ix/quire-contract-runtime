---
id: "SR-622"
title: "IR-430 follow-up gap analysis: PR #89 build-tooling alignment"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@b3c18e4e2b49a659fa04320b6157e08e1e12e797; Makefile, scripts/check_one_copy.awk, .gitignore, CLAUDE.md"
review_set: subset
relationships: []
---

# SR-622: IR-430 follow-up gap analysis (PR #89)

## Summary

Ticket: IR-430 (follow-up PR agent-ix/quire-contract-runtime#89). The diff is build tooling
only: Makefile, one awk script, .gitignore and CLAUDE.md. It does not touch `spec/`, `plan/`,
`src/` or `tests/`. Plan completion: not assessed (planless).

## Method

- I checked that CLAUDE.md's use-local/use-remote/deny lines match the Makefile's behaviour, by
  running each target (see SR-621).
- I grepped `spec/` and `plan/` for use-local, one-copy and Cargo.lock requirements. None owns
  this tooling. RT has no counterpart to codegen's FR-030. This PR does not change requirement
  coverage.
- I checked the one-copy gate against RT's real lock. It has no agent-ix git entries (RT has no
  first-party git deps), so the gate passes trivially today. That is expected, and the gate is
  wired to catch the first duplicate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The documentation matches the behaviour. No requirement or matrix row is affected, and no
stub or coverage change is introduced. The tooling has no owning requirement in RT. That predates
this PR (PR #88 added the gate), so it is recorded here as context and not as a finding.
