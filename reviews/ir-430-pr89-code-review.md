---
id: "SR-621"
title: "IR-430 follow-up code review: PR #89 aligns use-local/use-remote with codegen"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@b3c18e4e2b49a659fa04320b6157e08e1e12e797; Makefile, scripts/check_one_copy.awk, .gitignore, CLAUDE.md"
relationships: []
---

# SR-621: IR-430 follow-up code review (PR #89)

## Summary

Ticket: IR-430 (follow-up PR agent-ix/quire-contract-runtime#89, branch chore/align-use-local).
The PR replaces RT's use-local/use-remote block with the one on agent-ix/quire-contract-codegen
main (b61c328). It drops `-F'"'` from the one-copy awk call by setting `FS` in a `BEGIN` block,
and gitignores the new `.cargo/Cargo.lock.pre-local` snapshot. It adds one fix that codegen does
not have: the Cargo.lock snapshot is taken after LOCAL_PATCHES is validated, so a bad entry leaves
no stray snapshot. This also resolves SR-613 FND-003 (use-remote now restores the lock).

## Method

- Diffed the section from "Local development against sibling" to the next banner against
  codegen origin/main. The only differences are RT's empty `LOCAL_PATCHES` and the moved
  snapshot line. `scripts/check_one_copy.awk` is byte-identical to codegen's.
- Round trip with the empty list: `make use-local` then `make use-remote`. The first wrote an
  empty config and a snapshot. The second removed the config and restored the lock.
  `git status --porcelain --ignored` was clean and `.cargo/` was empty afterwards.
- Bad entries: `bogus:only` (malformed), `nosuchrepo:crate:.` and
  `quire-spec-language:qsl-cst:qsl-cst` (not cloned). Each exited 1 and left `.cargo/` empty.
  `quire-contract-ir:quire-contract-ir:.` (a real sibling that RT does not depend on) hit the
  unused-patch branch. It exited 1, removed the config and the snapshot, and left the tree clean.
- awk on a scratch copy of Cargo.lock, without `-F`. With two appended `qsl-cst` entries from
  `git+https://github.com/agent-ix/...` it printed "one-copy: qsl-cst has 2 entries" and exited 1.
  With one entry it exited 0. The legacy `-F'"'` call gives the same result.
- `make deny` exited 0 (bans, licenses and sources ok, plus the awk loop).
- Tracked locks: `git ls-files` shows only the root `Cargo.lock`. `measurement/footprint` is a
  workspace member (root `[workspace] members`), so it shares the root lock. Restoring only the
  root lock is therefore correct. The deny loop over `*/Cargo.lock` still covers any future nested lock.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `make help` still says use-remote only drops the local patch file. It now also restores Cargo.lock from the snapshot. | Makefile:36 |
| FND-002 | low | A snapshot left by an earlier use-local is kept by later runs, and use-remote restores it. So a Cargo.lock change made after the first use-local (a pull or a deliberate `cargo update -p`) is silently reverted. Reproduced: use-local, edit the lock, use-local, use-remote; the edit was gone. This is inherited from codegen, and RT's empty list makes use-local otherwise a no-op. | Makefile:132, Makefile:162 |

### FND-002 detail

`[ -f .cargo/Cargo.lock.pre-local ] || cp Cargo.lock .cargo/Cargo.lock.pre-local` keeps the
oldest snapshot. `use-remote` then does `mv` over the live lock with no staleness check. The block
comment claims "a deliberate `cargo update -p` made without a patch is never lost". That holds
only if use-remote ran in between. The same code is on codegen main, so any fix belongs in both
repos. It should not diverge RT. Options: refuse to restore when the snapshot differs from
`git show HEAD:Cargo.lock` and the live lock differs from both, or warn on a pre-existing
snapshot in use-local.

## Verdict

Approve, with two low, non-blocking findings. The block matches codegen main except for RT's
list and the snapshot-order fix, and the fix is correct. With an empty list the round trip leaves
the tree clean. Bad entries fail and leave no `.cargo` files. The awk works without `-F` and
catches a duplicated agent-ix entry. `make deny` passes. Restoring only the root lock is correct,
because RT has exactly one tracked Cargo.lock. The unused-patch error also fires when the crate is
not a dependency at all. Its wording ("the sibling's version does not satisfy the requirement") is
misleading in that case, but it matches codegen, so it is not raised here.
