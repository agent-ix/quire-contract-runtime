---
id: "SR-613"
title: "IR-430 code review (incl. rust-review lane): one copy of first-party crates in RT"
type: SpecReview
scope: "agent-ix/quire-contract-runtime@2eb0b5299cbf61e1c1cc421580384c82fdb9dfc5; Makefile, deny.toml, .gitignore, CLAUDE.md, scripts/run_feature_matrix.py, tests/exact_debug_parity.rs, conformance/qsl-agreement (removed)"
relationships: []
---

# SR-613: IR-430 code review

## Summary

Ticket: IR-430 (created by the reviewer; the PR carried no ticket). PR:
agent-ix/quire-contract-runtime#88, head 2eb0b52, diffed against origin/main 5b6275a.

The PR deletes the `conformance/qsl-agreement` workspace and its `make conformance` target and
`ci` entry. It adds `make use-local` / `make use-remote`, which write or delete a gitignored
`.cargo/config.toml` `[patch]` driven by `FIRST_PARTY_GIT_DEPS` (empty in RT). It adds 18
`[[bans.deny]] deny-multiple-versions = true` entries to deny.toml, and `make deny` now runs
`cargo deny --workspace check licenses bans`. The spec and plan evidence pointers are covered by
SR-614 (gap analysis).

## Method

I read the whole non-deletion diff and the plan it implements. I ran the gates in the worktree.
On scratch copies of the head (worktree untouched) I tested the deny gate with a real second copy
of `quire-contract-runtime` (a `file://` git clone of the head, so same name and version,
different source) added three ways: as a normal dependency, as a dev-dependency only, and as an
optional dependency behind a non-default feature. I ran `make use-local` / `make use-remote` from
a scratch consumer crate that depends on `github.com/agent-ix/quire-contract-runtime` by
`branch = "main"`, with an empty list, a missing sibling and a real sibling, and checked the
generated file with a TOML parser and with `cargo metadata`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The one-copy bans do not see duplicates that arrive only through dev-dependencies: cargo-deny's `bans.multiple-versions-include-dev` defaults to false. Reproduced: a dev-dep second copy of quire-contract-runtime gives `bans ok`, exit 0. | deny.toml:22-24 |
| FND-002 | medium | The bans do not see duplicates behind a non-default feature: cargo-deny resolves default features only unless `[graph] all-features = true`. Reproduced: an optional second copy behind feature `extra` gives `bans ok`, exit 0. | deny.toml:22-24, Makefile:146 |
| FND-003 | medium | `make use-local` rewrites Cargo.lock to source-less path entries, and `make use-remote` only deletes the config. The lockfile left behind fails `--locked` builds, and it can be committed by mistake. | Makefile:137-138 |
| FND-004 | low | A `FIRST_PARTY_GIT_DEPS` entry with fewer than three `:` fields is accepted silently. `repo:crate` sets `dir` to the crate name, and a bare `repo` sets crate and dir to the repo name. | Makefile:118, Makefile:130 |
| FND-005 | low | The manual CI workflow's license job still runs only `cargo deny check licenses`, so the new bans gate exists only in `make deny`. | .github/workflows/ci.yml:74-75 |
| FND-006 | low | CLAUDE.md, a line this PR edits, says deny.toml "denies unknown registries/git sources", but `make deny` does not run the `sources` check, so those `[sources]` settings are never enforced. | CLAUDE.md:39, Makefile:146 |

### FND-001 detail

With the second copy as a normal dependency, `cargo deny --workspace check bans` gives
`error[duplicate]: found 2 duplicate entries for crate 'quire-contract-runtime'`, exit 2. So the
per-crate ban does catch a same-version, different-source copy. Moved to `[dev-dependencies]`, the
graph still holds both copies (`cargo tree -e normal,dev` shows the path copy and the
`file://…?branch=main` copy) but the output is `bans ok`, exit 0. This is the plan's main risk
case, because the qsl-agreement cycle and the IR/QSL composition tests arrived as dev-deps.

Fix: add `multiple-versions-include-dev = true` under `[bans]`. Verified: the same dev-dep
duplicate then fails with exit 2.

### FND-002 detail

An optional second copy enabled by feature `extra` gives `bans ok` with the default config. It
fails, exit 2, with either `cargo deny --workspace --all-features check bans` or
`[graph] all-features = true` in deny.toml. RT gates its whole `exact` surface behind a feature,
so a future authority dependency would most likely arrive this way.

Fix: add a `[graph]` table with `all-features = true` to deny.toml. Verified on a clean copy of
the head with both fixes applied: `cargo deny --workspace check licenses bans` passes (and
`sources` passes too).

### FND-003 detail

Reproduced in a scratch consumer. After `make use-local`, the `quire-contract-runtime` entry in
Cargo.lock has no `source` line. After `make use-remote`, `cargo metadata --locked` exits 101
with "cannot update the lock file … because --locked was passed". RT's list is empty, so nothing
breaks in RT today, but this is the snippet the plan has each repo copy.

Fix: have `use-remote` also restore the lockfile (`git checkout -- Cargo.lock`), or run
`cargo update -p <crate>` for each listed crate. At minimum, print a warning not to commit
Cargo.lock while patched.

### FND-004 detail

`dir=$${rest#*:}` returns `rest` unchanged when it contains no `:`. Fix: reject any entry whose
field count is not exactly 3, with a message naming the entry.

## Verdict

Request changes on FND-001, the high finding: as written, the gate this PR adds for the plan
cannot see the dev-dependency duplicates the plan exists to remove. FND-002 and FND-003 are
one-line fixes worth making in the same round.

What is right:

- Removing `conformance/qsl-agreement` leaves no dangling build reference. `git grep` over the
  tracked tree finds no `make conformance`, `QSL_AGREEMENT_TOOLCHAIN` or `conformance/` path
  outside historical `reviews/` files and the Task-003 prose. `.github/workflows/` never
  referenced it, and `ci:` no longer lists it.
- `make deny` now runs `bans` across the workspace. The `quire-contract-runtime-footprint` member
  appears in its output (the wildcard warning), and a normal-dependency duplicate fails it.
- use-local: the generated patch parses as TOML
  (`{'patch': {'https://github.com/agent-ix/quire-contract-runtime': {...}}}`), and `cargo
  metadata` then resolves the dependency to `path+file://…/quire-contract-runtime#0.1.0`, so the
  config-relative `../<repo>` path resolves correctly. A missing sibling fails with exit 2 and
  "use-local: ../quire-spec-language/crates/qsl-cst/Cargo.toml not found; clone
  agent-ix/quire-spec-language next to this repo". An empty list writes a header-only file, which
  is valid, and exits 0. `.cargo/config.toml` is in `.gitignore:15` (unanchored), and RT tracks
  no `.cargo/config.toml` that `use-remote` could delete.
- No Rust source changed except one doc comment. RT now has no agent-ix git dependency, so no
  `rev`/`tag` is left to switch.

Gates at 2eb0b52 (CARGO_TARGET_DIR inside the worktree, deleted afterwards): `make fmt-check`
exit 0. `cargo test --locked --workspace --all-targets` exit 0 (all suites ok, 0 failed; with
default features, so the `exact` suites are not in this run). `make deny` exit 0 (`bans ok,
licenses ok`, with one wildcard warning and one `license-not-encountered` warning).
