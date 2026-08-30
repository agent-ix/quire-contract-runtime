---
id: TC-002
title: "Verify Boolean operator semantics"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: verifies
---
# TC-002: Verify Boolean operator semantics

## Description

Verify Boolean truth tables and the operand-evaluation distinction between short-circuit and total
operators.

## Test Procedure

Exercise every input pair while side-effect counters record operand evaluation order and count.

## Expected Results

Results match Boolean logic; short-circuit operands are skipped where specified and total operands
are each evaluated exactly once from left to right.

