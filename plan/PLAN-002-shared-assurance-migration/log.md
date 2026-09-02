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
