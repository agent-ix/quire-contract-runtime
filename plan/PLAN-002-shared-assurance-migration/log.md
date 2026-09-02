---
type: log
title: "PLAN-002 - Update log"
description: "Chronological changes to the shared-assurance migration bundle."
---
# PLAN-002 - Update log

## History

- **2026-09-01** - Opened the bundle against issue #8. Branched from the head of PR #7 rather than
  from `main`, so that the domain work that PR carries — the seventh Kani harness, the `#[cfg(kani)]`
  accounting constructor, the widened arithmetic oracle, the symbolic division and full-width index
  harnesses — is retained and so that the old generic path is present to be run against the same
  candidate revision before it is deleted.
- **2026-09-01** - Recorded the old path's result at `0bb51fb` as observed: `make ci` exit 0, 7 Kani
  harnesses, 3 mutation controls, 56 evidence-tool tests, 124 checksums, 104 manifest artifacts,
  linked footprint 907 bytes. It was green, and this is stated because a dual run whose baseline was
  red must be reported that way rather than manufactured green.
- **2026-09-02** - The entries above are left as written; they describe what was true when written.
  The retained evidence they preserved was deleted the following day under
  `agent-ix/quire-contract-runtime#11`, after the repository owner released the evidence-preservation
  constraint for the pre-stable phase (`agent-ix/engineering-assurance#7`, section "Preservation
  constraint released for the pre-stable phase"). PLAN-002's completion rule spoke of leaving every
  byte under `evidence/` unchanged; it was satisfied as written for the duration of this plan, and
  those bytes were then removed rather than carried forward. Nothing was rewritten, backdated or
  re-sealed. Task-001's "FREEZE — not deleted" instruction for the four artifacts under `schemas/`
  and Task-002's delivery of `scripts/legacy_evidence_view.py` are historical records of what this
  plan did, not statements about the current tree.
