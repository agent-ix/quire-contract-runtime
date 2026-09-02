---
id: TC-010
title: "Reach Quoin through the declared adapter with no producer executed"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-010: Reach Quoin through the declared adapter with no producer executed

## Description

Verify that the change-assurance record, the seven proof attestations, the intake of producer bytes,
and the verification receipt all travel through the pinned Quoin CLI; that each attested result is
read out of the bytes its producer wrote; and that neither Quoin nor Quire executes cargo, rustup,
rustc, or Kani.

## Test Procedure

Run `scripts/assurance_chain.py --candidate-revision <HEAD> --json` and require every scenario,
control, and adapter probe to match. Then run it twice more with a shimmed `PATH`: once with
`cargo`, `rustup`, and `rustc` replaced by shims that log every invocation and fail, requiring the
chain to succeed and the log to be empty; and once with `quoin` shimmed, requiring the chain to fail
and that log to be non-empty.

## Expected Results

Every scenario, control, and probe matches. No producer is asked to do work. The control run proves
the shims were reachable, so the empty log in the first run is a measurement rather than an
unconsulted `PATH`.
