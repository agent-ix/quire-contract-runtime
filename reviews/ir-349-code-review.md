---
id: "SR-640"
title: "IR-349 code review: adopt quire-exact part 1 (PR 94)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@c5e0f2b9ef979706d127a518a0e1466fe8647c04; Cargo.toml, Cargo.lock, deny.toml, Makefile, clippy.toml, measurement/footprint/Cargo.toml, scripts/check_deny_bans.sh, scripts/run_feature_matrix.py, src/exact/collection.rs, src/exact/expression.rs, .github/workflows/ci.yml, README.md, CLAUDE.md; base agent-ix/quire-contract-runtime@e843e1bb196a440cb62c1064f8855cfd5ecbb43a; cross-checked read-only against quire-spec-language origin/main 0885a9ba"
---

# SR-640: IR-349 code review of PR 94 (rust-review lane folded in)

## Summary

Ticket: IR-349. PR agent-ix/quire-contract-runtime#94 head c5e0f2b, three commits over main
e843e1b (up to date with main). The code commit raises the Rust floor to 1.82, adds the optional
`quire-exact` git dependency behind `exact`, bans the 14 non-kernel QSL workspace crates in
`deny.toml`, adds `[graph] all-features = true`, and adds `scripts/check_deny_bans.sh`
(`make deny-mutations`, in `make ci`). The Rust change is one `repeat_n` call and one comment.

## Method

- Read the full diff (spec excluded here; see SR-642).
- Ran at head, CARGO_TARGET_DIR in the worktree: `make fmt-check lint test-features doc msrv size
  deny deny-mutations test`: all exit 0. `test-features` verified 14 feature sets, including
  `build-exact-no-std-msrv` (`+1.82.0`, thumbv7em-none-eabi, quire-exact compiled). `make size`:
  913 bytes, 0 panic references. `make msrv` compiled quire-exact on 1.82.
- `cargo deny check advisories`: ok. `cargo tree -d --all-features -e normal,dev,build`: nothing.
  `cargo tree -p quire-contract-runtime-footprint --target thumbv7em-none-eabi -i quire-exact`:
  no match.
- Negative check of the mutation script: in a scratch copy with the `qsl-eval` ban removed,
  `check_deny_bans.sh qsl-eval qsl-replay` reports FAIL for both qsl-eval cases and ok for
  qsl-replay, exit 1. Not vacuous (cargo-deny still exits non-zero there, on licences and
  transitive bans; the `error[banned]: crate 'qsl-eval = ` grep is what tells them apart). The
  `mktemp -d` dir is removed by the trap.
- One-copy: a scratch lock with a second `quire-exact` entry makes `check_one_copy.awk` exit 1.
- Compared the ban list with QSL origin/main workspace members: 15 members, package names
  `quire-spec-language`, `xtask`, `arch-lint`, `quire-exact` and 11 `qsl-*`; the list is the 14
  that are not `quire-exact`. quire-exact declares `rust-version = "1.82"` and depends on no
  banned crate.
- Lock delta: quire-exact 0.1.0 (git, branch main), thiserror 2.0.20, thiserror-impl 2.0.20,
  syn 3.0.6. Licences pass `make deny`; advisories pass.
- `git merge-tree` with the other open PR (#78): 15 conflicts, the same 15 it has against main.
  PR 94 adds none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `check_deny_bans.sh` copies the whole working tree with `tar`, nine times, not the tracked files. It excludes only `./target`, `./.git` and `*-target`, so ignored and untracked content goes into every copy: a `.cargo/config.toml` written by `make use-local` (which patches the git source the bans are about), `.worktrees/`, nested `target/` dirs such as `measurement/footprint/target`. A copy carrying the local patch tests a different graph from the committed one, and a large untracked tree makes `make ci` slow and disk-heavy. Copy `git ls-files` (or `git archive HEAD` plus the working-tree diff) instead. | scripts/check_deny_bans.sh:23-28 |

## Verdict

**Code: mergeable on code review alone; one low finding.** Every gate the brief named passes at
head; measurements reproduce (913 B, 14 feature sets, one lock entry, no duplicate in
`cargo tree -d`).

Rust lane: the `repeat_n` change keeps behaviour. `iter::repeat(member).take(n)` and
`iter::repeat_n(member, n)` yield the same n items; `repeat_n` moves the last one in place of a
clone. `Value::clone` is an `Rc` count change with no metering, and the multiplicity is at least 1,
so the result and the charges are the same. `repeat_n` is stable since 1.82.0, which matches the
new floor, and clippy's msrv 1.82 is what enabled `manual_repeat_n`. No panic, unsafe or integer
conversion surface changed.

Checked and clean:

- `quire-exact` is optional, enabled only by `exact`, `default-features = false`, git on the QSL
  repository with `branch = "main"`, no rev, tag, path or `[patch]`. `version = "=0.1.0"` is the
  crate's own version spelled as codegen spells its first-party deps; it records no commit. The
  spec-text conflict it raises is SR-642 FND-003.
- `[graph] all-features = true`: before it, cargo-deny resolved default features only, so the
  optional deps (num-*, unicode-normalization, serde, serde_json, proptest) were outside the
  licence and ban checks. It widens the checks, and they pass. The CI `licenses` job's
  `cargo deny check licenses` picks it up from deny.toml with no workflow change.
- `allow-git` names only the QSL repository; `unknown-git = "deny"` stays.
- No leftover 1.75 in Makefile, CI, README, Cargo.toml, clippy.toml, scripts or code comments.
  The remaining 1.75 mentions are history in old reviews and plans, and history sentences in spec
  (SR-642).

CI workflow edit (for the leader to put to the owner): the only change under `.github/` is
`.github/workflows/ci.yml`, job `msrv`, 2 lines: the job name `Rust 1.75 surface and footprint`
becomes `Rust 1.82 surface and footprint`, and `dtolnay/rust-toolchain@1.75.0` becomes `@1.82.0`.
No new job, step, trigger or permission. The workflow is `workflow_dispatch` only and never runs
`make ci`, so `deny-mutations` is wired into local `make ci` only, not into CI.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@ac43bf55a3d96d5ad6f2a375dd97c6dc662eb732.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ac43bf5: `copy_workspace` now copies `git ls-files -z --cached` through `tar --null -T -`, so untracked and ignored files (a `use-local` `.cargo/config.toml` included) never reach a scratch copy. `make deny-mutations` re-run at ac43bf5: 8 of 8 ok. |
