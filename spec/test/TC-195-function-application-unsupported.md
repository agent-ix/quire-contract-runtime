---
id: TC-195
title: "Negotiate a function's undischargeable capability as unsupported"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-273
    type: verifies
---
# TC-195: Negotiate a function's undischargeable capability as unsupported

## Description

Check that a function whose declared operator requirements no registered backend can discharge
negotiates the `unsupported` provider disposition, naming the required capability, before any
application, and that the disposition stays outside the evaluation path. Evidence:
`tests/exact_function_application.rs` (`--features exact`), planned for
agent-ix/quire-contract-runtime#34.

## Test Procedure

1. Negotiate the declared requirements of a function whose body declares an operator requirement a
   backend cannot discharge (for example an IEEE operation at an unsupported width), through the
   FR-009 negotiator; check the function negotiates `Unsupported`, carrying the same capability cause
   FR-009 names for that requirement.
2. Negotiate a function whose declared requirements are all `Supported`, and one whose requirements
   are `RequiresBound` with every required bound present; check each negotiates that disposition and
   that neither is reported `Unsupported`.
3. Check the negotiation takes no `Meter` and leaves every counter unchanged, and that no disposition
   is convertible into an `Outcome` variant or an `InputRefusal`, as FR-009-AC-5 already requires for
   the bare operators.
4. Apply a function whose requirements negotiate `Supported`; check the application completes
   normally and produces no disposition of its own.

## Expected Results

A function whose declared operator requirements no registered backend can discharge negotiates
`unsupported` with the capability cause named, before any application and with no `Meter`
participation; a function whose requirements are dischargeable negotiates `Supported` or
`RequiresBound` and applies normally; and no disposition appears as an `Outcome` variant or an
`InputRefusal`.
