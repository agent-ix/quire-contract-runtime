---
id: TC-013
title: "Demonstrate all twelve outcomes and pair every negative with a positive control"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-013: Demonstrate all twelve outcomes and pair every negative with a positive control

## Description

Verify that pass, fail, unavailable, unsupported, inconclusive, not-computed, malformed, partial,
stale, suspect, vacuous, and tampered are each demonstrated by a case that produced them and matched,
and that each negative case names a positive control which was observed to be accepted. A refusal
that has never been seen to accept is indistinguishable from a step that never worked.

## Test Procedure

Union the chain report's `states_demonstrated` with the compatibility census's case kinds and require
all twelve. Require every declared negative scenario to be named by some control's `pairs_with`, and
require the chain to refuse a control naming a scenario that does not exist.

## Expected Results

Twelve of twelve demonstrated. Every negative paired. `unavailable` in particular is demonstrated by
the Kani producer's own vocabulary, so an absent model checker is a reported state and not a skip.
