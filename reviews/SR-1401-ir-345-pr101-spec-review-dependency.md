---
id: "SR-1401"
title: "IR-345 spec review (dependency): AD-004 step gating on IR-582 and IR-583 (PR 101)"
type: SpecReview
analysis: dependency
review_set: subset
scope: "agent-ix/quire-contract-runtime@e3c517ccc56eab1d72c6e797456537508ebfeec1; spec/assurance/AD-004-runtime-crate-layout.md (edge table, Migration steps 1-5, L-2); read against deny.toml, scripts/check_deny_bans.sh, FR-275, IR AD-007 on IR origin/main, Linear relations of IR-349"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1401: IR-345 dependency review of PR 101

## Summary

Ticket: IR-345. Measured clean:

- **Step 2 needs IR-582.** True: IR-582's description creates the repositories, and IR-582
  blocks IR-349.
- **Step 3 needs step 2 and not IR-583.** True: step 3 does not delete the `expression` items.
  The Interim paragraph says steps 3 and 4 only repoint them.
- **Step 5 needs IR-583.** True: IR-583 blocks IR-349, and its description says "Blocks IR-349
  steps 1 and 5".
- **The edge table drops the QSL-repository edge.** Step 2 removes the QSL `allow-git` entry and
  the 14 name bans, and `unknown-git = "deny"` then guards git sources. This matches IR AD-007
  O-1 option 3 ("RT's 14 bans and the QSL `allow-git` entry go"). It supersedes AD-007's
  recommendation to add `qsl-analyze` and `qsl-walk-grow`, consistently.
- **L-2 drops "and the one-copy check".** This does not regress anything: the owner decided on
  IR-346 to replace `check_one_copy.awk` with one owned tool (IR-581), and AC-5 still states the
  one-entry rule.

Two gating claims do not hold.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 1 claims "the rest need no ticket", and that covers the Q-6 amendments. It is false for them. FR-275-AC-4 is met today: it requires the `quire-exact` git source "on the QSL repository". AC-7 and AC-8 require the bans, which `make deny-mutations` exercises (TC-198). Restating these for the extracted repositories in step 1 does two bad things. It names repositories that do not exist until IR-582. And it leaves main with criteria its own `Cargo.toml` and `deny.toml` violate until step 2 lands, which itself waits on IR-582. Gate the Q-6 amendments on IR-582 and land them in step 2's PR, or say how the rows are kept planned in between. | AD-004 Migration step 1; FR-275-AC-4, AC-7, AC-8 |
| FND-002 | medium | Step 2 removes the 14 name bans but does not change `scripts/check_deny_bans.sh`. That script is `make deny-mutations` (FR-275-AC-8, TC-198). It adds `qsl-eval`, `qsl-replay`, `qsl-semantics` and `quire-spec-language` from the QSL git URL, and requires cargo-deny's `error[banned]` for each. With the bans gone, a sources error replaces `banned`, so the script fails. Gate G includes `deny-mutations`, so step 2 as written fails its own gate. Step 2's gate cell, "`make deny` fails on a git source outside the allow-list", names no check that proves it. Add to step 2 the script rewrite (it should assert the sources rejection), the TC-198 change, and the `CLAUDE.md` `deny.toml` description. | AD-004 Migration step 2; scripts/check_deny_bans.sh; Makefile deny-mutations |

## Verdict

**Fix both mediums before merge.** Each is a wording or gating change in the AD. No step as written
can break main silently, because gate G would catch FND-002. But a step cannot pass its own gate as
it is written.

## Dispositions

Disposition pass 1. I reviewed agent-ix/quire-contract-runtime at
7b25ebfddd762e668ca4f2d6fbcb976b58a225bf (fix commit 7b25ebf). I re-read deny.toml,
`scripts/check_deny_bans.sh`, the Makefile and FR-275.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7b25ebf. Step 1 now "needs no ticket" and amends Group 1 only, with "Groups 2 and 3 are not touched here". The Q-6 statements (Group 2: FR-275-AC-4, AC-7, AC-8 and the rest) are amended in step 2. That is the same PR that needs IR-582 and moves the dependency, so main never holds criteria its Cargo.toml or deny.toml violate. |
| FND-002 | fixed | 7b25ebf. Step 2 now rewrites `scripts/check_deny_bans.sh` to assert cargo-deny's sources rejection where it now asserts `error[banned]`. It also changes TC-198 and the `CLAUDE.md` `deny.toml` description in the same PR. Its gate is "G, with `deny-mutations` green on the rewritten script". The gate story holds. The script already runs `cargo deny check licenses bans sources`. Once the QSL `allow-git` entry is gone, `unknown-git = "deny"` rejects a crate added from the QSL git URL in the sources check. The unmodified-tree pre-check still passes, because the remaining git sources are allow-listed. Everything is one PR, so no merge sits between the ban removal and the script change, and main stays green. CG adopts RT's change only through its own lock bump, and IR-582 includes QSL switching its workspace to the extracted crates. So the bump does not give CG a second copy of `quire-exact`. |
