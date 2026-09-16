---
type: log
title: "PLAN-003 - Update log"
description: "Chronological changes to the exact scalar oracle bundle."
---
# PLAN-003 - Update log

## History

- **2026-09-16** - Opened the bundle for the scalar slice of issue #16 against
  agent-ix/quire-specification@a25c93c, then moved to @5d88578, which adds the metered
  `ordering.*`, `integer-arithmetic.*`, `rational-arithmetic.*` and `boolean.result-retain` rows and
  TC-185 D20–D21. The agreement package stays on quire-spec-language d9d5273; D20–D21 charges are a
  named upstream lag pending quire-spec-language#119. Composite families remain blocked on #119.
- **2026-09-16** - Slice 2 moved the pin to agent-ix/quire-specification@7d7943a (QSpec PR #75):
  arithmetic, normalize, rounding, retain-upscale, unit-event and target-domain amounts are derived
  from operands and charged before allocation, replacing result-size bracketing. TC-185 adds
  D22–D23. Charges not yet metered by quire-spec-language d9d5273 are listed in
  `CHARGES_PENDING_QSL_119` (TC-185: D09, D13, D20–D23; TC-187: U10, U13, U15, U16, U19, U20,
  U22–U24, U26, U28, U29).
