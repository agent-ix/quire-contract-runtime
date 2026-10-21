---
id: TC-202
title: "Reject duplicate first-party Git lock entries through make deny"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-202: Reject duplicate first-party Git lock entries through make deny

## Description

Check the runtime's shared lock coherence gate independently of cargo-deny's other checks.
The checker is the real distributed `quire` command selected through `QUIRE`,
never a replacement implementation.
TC-197 retains the broader kernel ownership and no-copy inspection.

## Test Procedure

1. Create a temporary Git worktree at the current commit. Run `make deny` with
   its cargo-deny stage replaced by a successful command; require the unchanged
   lockfile to pass the remaining one-copy check.
2. For each first-party Git crate resolved in that lockfile, add a second entry
   in the temporary lockfile and rerun `make deny` with the same isolated cargo
   stage. Add a separate pair for a synthetic first-party Git crate so the guard
   is checked beyond the current dependency names.
3. Use valid full resolved Git fragments in each synthetic package source. Require
   each duplicated-name run to exit non-zero with the owned coherence diagnostic
   naming the affected crate; malformed input alone does not satisfy this control.
4. Run an aligned multi-crate control and a distinct-name, split-revision control
   for one repository. Require success for the former and a non-zero Make exit
   naming the repository, both crates and both revisions for the latter.
5. Add a second tracked lock under a nested directory and require its refusal
   through the same single checker invocation. Verify malformed lock input and a
   missing executable both fail without a successful coherence report. Remove
   the temporary worktree.

## Expected Results

The unchanged lockfile and aligned control pass, while duplicate names and
repository revision splits are rejected by the shared gate. No source dependency or production runtime item changes.
