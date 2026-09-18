---
id: TC-030
title: "Dispose negotiation items independently and in input order"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: verifies
---
# TC-030: Dispose negotiation items independently and in input order

## Description

Check `negotiate_integer_division` and `negotiate_ieee` over every bound subset, every cause
position and the finite-proof seam. Evidence: `tests/exact_negotiation.rs` (`--features exact`).

## Test Procedure

1. Negotiate the empty slice for both negotiators; expect an empty result.
2. Negotiate a mixed slice, then the same slice reversed; expect the reversed dispositions.
3. Negotiate `Mathematical` and each of the eight `IntegerDivisionBounds` presence subsets.
4. Build IEEE items failing width, operation, rounding and exceptional policy singly, then in
   pairs; expect the earlier cause in every pair.
5. Negotiate a non-rounding operation with a rounding direction the backend lacks.
6. Negotiate an item that needs a finite proof the backend cannot discharge, alone and combined
   with a missing capability.

## Expected Results

One disposition per item in input order; the first failed check names the cause; `RequiresBound`
appears only when no `unsupported` cause applies; no `Meter` participates.
