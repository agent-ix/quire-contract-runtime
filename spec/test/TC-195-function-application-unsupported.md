---
id: TC-195
title: "Settle a function application unsupported when a called capability is undischargeable"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: verifies
---
# TC-195: Settle a function application unsupported when a called capability is undischargeable

## Description

Check that an undischargeable capability reached from inside a called function's body propagates out
of `CheckedPackage::call` as the `unsupported` provider disposition, naming the capability, rather
than as a refusal or a partial `Outcome`. Evidence: `tests/exact_function_application.rs`
(`--features exact`), planned for agent-ix/quire-contract-runtime#34.

## Test Procedure

1. Call a function whose body reaches an operator the backend negotiation of FR-009 disposes
   `Unsupported` (for example an IEEE operation with an unsupported rounding direction); check the
   application settles `unsupported`, naming the same capability cause FR-009 names for that operator.
2. Call a function whose body reaches only `Supported` or `RequiresBound` operators, with the
   required bound present; check the application completes normally and no `unsupported` disposition
   is produced.
3. Check that an `unsupported` disposition reached from a call is never represented as an
   `InputRefusal` and never as a `Refused` or `Undefined` `Outcome` variant.

## Expected Results

A called function's undischargeable capability settles the whole application `unsupported`, naming
the capability; the application is never refused, never charged past the point of settlement, and
never reported as a `Refused` or `Undefined` outcome for that cause.
