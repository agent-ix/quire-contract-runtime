---
id: TC-004
title: "Map verdicts to proptest outcomes"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-003
    type: verifies
---
# TC-004: Map verdicts to proptest outcomes

## Description

Verify the optional adapter maps pass, postcondition failure, and precondition rejection distinctly.

## Test Procedure

Enable `proptest`, adapt each verdict, and inspect the result variant.

## Expected Results

Pass becomes success, failure becomes `Fail`, and rejection becomes `Reject`.

