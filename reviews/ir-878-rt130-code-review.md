---
id: SR-12902
title: IR-878 RT130 code-review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; PR130 head 4917ef5337a35ac0b8bb0195621e0def49689372 vs main a0bbc149; tests/deny_one_copy.rs, spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/matrix/TC-202-one-copy-deny-mutation.md, spec/exact/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-878. PR: quire-contract-runtime#130. Code review with the Rust lane (rust-review) over the one new integration test `tests/deny_one_copy.rs` and the four spec/matrix edits. No production code changed. The test runs the real `make deny` target (cargo-deny stage replaced by `CARGO=true`) in a scratch Git worktree, appends a second lock entry for each resolved first-party Git crate plus a synthetic first-party pair, and requires a non-zero exit with the exact `one-copy: <name> has 2 entries in Cargo.lock` diagnostic.

## Verdict

CONDITIONAL. The test genuinely enforces FR-275-AC-6 at the committed tree: three committed mutations of the gate each turn it red. One medium finding: it checks the committed HEAD tree, not the working tree, so a local gate run over an uncommitted gate regression stays green. One low finding on repository side effects. Tagging uses `use ix_trace_rs::trace;` with `#[trace("TC-202", "FR-275-AC-6")]`; no `/// Trace:` lines, no vendored files, no compatibility layer, no `#[allow]`, no unsafe.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The scratch worktree is created from committed `HEAD`, so the test verifies the committed Makefile, `scripts/check_one_copy.awk` and `Cargo.lock`, not the working tree; with the gate disabled by an uncommitted edit the test still passes, and local `make ci` is the repo's only gate. | tests/deny_one_copy.rs:24-29 |
| FND-002 | low | The test registers a Git worktree in the developer's repository during `cargo test`; Drop ignores the removal status, and an aborted run (SIGKILL, timeout) leaves a stale registered worktree under the temp dir. | tests/deny_one_copy.rs:24-29; tests/deny_one_copy.rs:49-58 |

## Finding evidence

FND-001, reproduced: in a scratch clone at 4917ef5 the awk `END` action was edited to `exit 0` without committing; `cargo test --all-features --test deny_one_copy` reported `ok. 1 passed`. The same edit committed (branch m1) failed the test with `duplicate ix-trace-rs was accepted`. `.github/workflows/ci.yml` states "Manual only ... Local `make ci` is the gate", and agents run that gate in worktrees that may carry uncommitted edits, so a green local gate can coexist with a disabled one-copy check until commit. Fix: overlay the working-tree `Makefile`, `scripts/check_one_copy.awk` and `Cargo.lock` onto the scratch worktree after `git worktree add`, or build the scratch directory from working-tree copies.

FND-002: `ScratchWorktree::new` runs `git worktree add` against `CARGO_MANIFEST_DIR`; `Drop` discards the `git worktree remove --force` status. Unwinding panics clean up (verified: no worktree remained after the failing mutation runs), but a killed process leaves a prunable entry. Correct but poorer than a plain temp copy; no wrong verdict.

## Coverage

Vacuity checks, each on a committed mutation in a scratch clone, all made the test fail: (m1) awk always `exit 0`; (m2) the awk line removed from the `deny` recipe; (m3) the first-party predicate narrowed to `quire-*` names (caught via `ix-trace-rs` and the synthetic name). The baseline assertion requires the unmodified lock to pass; the duplicate assertion requires both a non-zero exit and the named diagnostic, so an unrelated failure cannot satisfy it. The crate discovery uses the same `git+https://github.com/agent-ix/` prefix as the awk, and the synthetic pair guards against a gate narrowed to current names. Resolved first-party crates found: ix-trace-rs, quire-canonical, quire-canonical-derive, quire-exact, quire-semantic-value.

Rust lane: repo CLAUDE.md and Cargo.toml lints read; `unwrap`/`expect` appear only in test code; deterministic `BTreeSet` order; unique temp dir by pid plus nonce with `create_dir` claim (no probe-then-act); no clock assertions; no async; external tools are git, make and awk, which the repo's gate already requires.

Gates run by the reviewer (scoped): `cargo test --all-features --test deny_one_copy` passed; `cargo clippy -p quire-contract-runtime --all-features --test deny_one_copy -- -D warnings` clean; `cargo fmt --all -- --check` clean; real `make deny` passed (bans ok, licenses ok, sources ok). Full `make ci`, `make kani` and `make kani-mutations` were not run per the dispatch brief; the author's Kani log reports 8 harnesses verified and 3 mutation controls, an author claim not re-measured here.

## Examined units

```yaml
scope:
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: "`make deny` exits non-zero when `Cargo.lock` holds a second entry for any first-party Git crate."
- id: tests/deny_one_copy.rs
  path: tests/deny_one_copy.rs
  role: examined
  excerpt: "#[trace(\"TC-202\", \"FR-275-AC-6\")] fn deny_rejects_a_second_first_party_git_lock_entry"
- id: TC-202
  path: spec/exact/matrix/TC-202-one-copy-deny-mutation.md
  role: context_only
  excerpt: "Reject duplicate first-party Git lock entries through make deny"
```
