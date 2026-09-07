---
id: TC-015
title: "Bound immutable campaign snapshot transport"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: verifies
---
# TC-015: Bound immutable campaign snapshot transport

## Description

Test actual runtime reports and independently authored JSON. Decoding remains unqualified
accounting inspection; no generated terminal outcomes or shared assurance verdicts are tested.

## Test Procedure

1. Record pass, failure, rejection and discard; export, then record more events and verify
   snapshot immutability. Compare exact independently written JSON and decoded identity/counts.
2. Independently omit, duplicate, mistype and substitute every root/count member. Exercise
   escaped-equivalent duplicate keys, invalid UTF-8, trailing values, unsupported identities,
   negative zero, exponents, floats, overflow and failed greater than accepted.
3. Check actual private near-limit recording, exact maximum, one-event saturation and total
   saturation with no single counter at maximum. Never interpret at-limit as proven overflow.
4. Exercise exact/over wire and decoded identity limits, escaped Unicode at its decoded limit,
   hostile nesting and encoder refusal with no partial output. Run a native child under a real
   memory ceiling and distinguish process failure from a decoder's returned semantic error.
5. Compile-fail private field mutation, mutable report import, unchecked constructors, and
   JSON symbol use outside the feature. Execute stable/MSRV and isolated no_std qualification.

## Expected Results

Every valid complete report is retained exactly within declared bounds. Invalid data produces
structured refusal, never partial or authenticated results. Process/allocator failure remains
a separate unavailable observation. Ordinary recording and default dependency/footprint policy
remain unchanged.
