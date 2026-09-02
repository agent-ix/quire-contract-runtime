---
id: Task-003
title: "Dual run at one revision, then delete the old path"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: references
---
# Task-003: Dual run at one revision, then delete the old path

## Scope

Run both paths against the same candidate revision, record what was observed, and only then remove
the generic machinery.

## Deliverables

- The old path's result at the branch base, taken in a pristine clone so that the new path's own
  working files cannot influence it.
- The new path's result at the same revision.
- A single deletion commit, made last.

## Completion Evidence

The old path at `0bb51fb` returned exit 0 with 7 Kani harnesses, 3 mutation controls, 56
evidence-tool tests, 124 checksums, 104 manifest artifacts, and a 907-byte linked footprint. The new
path reports the same domain facts through the shared contract. Both results are recorded as
observed; neither is described as parity, because the two paths do not compute the same thing and
saying they agreed would be claiming more than was measured.
