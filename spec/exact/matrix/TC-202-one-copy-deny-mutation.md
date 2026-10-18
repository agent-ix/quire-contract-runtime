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

Check the runtime's one-copy gate independently of cargo-deny's other checks.
TC-197 retains the broader kernel ownership and no-copy inspection.

## Test Procedure

1. Create a temporary Git worktree at the current commit. Run `make deny` with
   its cargo-deny stage replaced by a successful command; require the unchanged
   lockfile to pass the remaining one-copy check.
2. For each first-party Git crate resolved in that lockfile, add a second entry
   in the temporary lockfile and rerun `make deny` with the same isolated cargo
   stage. Add a separate pair for a synthetic first-party Git crate so the guard
   is checked beyond the current dependency names.
3. Require each mutated run to exit non-zero with the `one-copy` diagnostic
   naming the duplicated crate and two entries. Remove the temporary worktree.

## Expected Results

The unchanged lockfile passes, while every duplicate is rejected by the
one-copy gate. No source dependency or production runtime item changes.
