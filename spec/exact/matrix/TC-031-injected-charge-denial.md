---
id: TC-031
title: "Fire one injected denial with a limit-independent record"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-010
    type: verifies
---
# TC-031: Fire one injected denial with a limit-independent record

## Description

Check the injection seam's precedence, record contents, occurrence counting and single-shot
behavior. Evidence: `tests/exact_outcomes.rs` (`--features exact`).

## Ownership and evidence

`ix://agent-ix/quire-exact/FR-358-AC-1` through `FR-358-AC-3` test named occurrence,
one-shot retry, `equality.plan` denial and the nonzero type. They leave the AC-2
`pairs + 2` reservation amount and AC-3 simultaneous-limit precedence unproved by owner
tests; IR-676 tracks both. The current RT test for AC-2 covers an ordinary point only.
The current RT test for AC-3 covers one short size counter, not every competing limit.

## Test Procedure

1. Inject at each admitted charge point at occurrence 1 under generous limits; check the record,
   every counter and the admitted-charge log.
2. Run the same injection under two different `work_units` limits; check `limit` and `consumed`
   are identical and equal to the work spent, not to either configured limit; also check
   `equality.plan` reports its `pairs + 2` reservation as `next_charge`.
3. Set limits short enough that the same charge would be denied on a real counter; check the
   injected record is returned.
4. Interleave charges at other points and charges the limits deny at the injected point; check the
   occurrence counts only admitted charges at the injected point.
5. Continue charging after the injection fires; check later charges meter normally and no second
   injection occurs.

## Expected Results

Exactly one charge is denied, its record is independent of the configured limits, and the
occurrence counter advances only on admitted charges at the injected point.
