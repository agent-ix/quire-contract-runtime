# Shared assurance

Two files and no evidence.

`change-assurance.json` is what this repository *states* about the change under
[issue #11](https://github.com/agent-ix/quire-contract-runtime/issues/11): the
requirements it claims to meet, the things it promises not to break, the proofs
it offers, and the questions it cannot answer. `pins.json` is the Engineering
Assurance release it adopts and the digests of the artifacts it actually reads
from that release.

## Why there is no evidence in here

Because retention is Quoin's job. `make assurance` seals the declaration into a
Quoin change-assurance record, seals a proof attestation over each producer's
already-written result file, hands those bytes to Quoin's intake, and asks for a
verification receipt. The record, the attestations, the retained bytes, and the
receipt all live in Quoin's store under `target/`, which is ignored.

The repository that produced a result does not also get to be the place that
result is kept, digested, and pronounced upon. That arrangement is the thing
this migration removed — 4,046 lines of collector, envelope builder, verifier,
anchor writer, tool-identity lock, and Makefile self-attestation — and putting a
smaller version of it back under a new directory name would be the same mistake
in a nicer font.

## What runs what

One target produces:

```
make assurance-inputs
```

It runs the Kani proof suite, the Kani semantic-mutation campaign, the feature
matrix, the linked-footprint measurement, the MSRV build and `quire coverage`,
and writes their structured output to `target/assurance/`.

Everything downstream consumes those files. `scripts/assurance_chain.py` refuses
to run a producer; if an input is missing it says so and names the target that
makes it. Quire exports and does not execute. Quoin transcribes and does not
execute. That separation is asserted by a test with a control, not just described
here.

## Kani, and the one place a transcript is still read

`cargo kani` emits no machine-readable result document. It prints a human
transcript and returns a process status. The migration contract forbids
recovering a verdict from stdout *when structured output exists*; for Kani it
does not exist, so the parsing has to live somewhere.

It lives in the domain tool. `scripts/run_kani_gate.py` is this repository's
Kani producer: it owns the transcript, applies the declared harness census and
the per-harness discharged-obligation floors, and publishes
`runtime.kani-proof/v1` — a structured result with one row per harness. Every
downstream consumer reads a field. Neither Quoin nor Quire ever sees the
transcript, and neither of them runs Kani.

When `cargo-kani` is not installed, the producer emits `unavailable` rows and
the chain attests `unavailable`. It does not emit an empty stream, it does not
emit `pass`, and `make ci` does not go green on a machine where the proofs never
ran: `make kani` exits non-zero when the toolchain is absent. *Could not check*
and *checked and passed* are different facts and this repository spent six review
rounds learning to keep them apart.

## The decision that is not here

A verification receipt for this change reads `incomplete`, and the reason it
gives is that no human decision event exists. That is correct. An ix-flow
decision is an attributed human act; only the repository owner can create one,
and an agent that synthesized one would be forging the single field in the whole
chain that exists to say a person looked.

## The retained records, and where they went

This repository held 42 `quire.derivation-evidence/v1` envelopes under
`evidence/` — 3,412 files, 10.2 MB. It never retained a `quire.pgm01-evidence`
record, so the pinned mapping answered `incompatible` for every one of them with
the reason "unknown PGM-01 schema version". That refusal was correct and was
reported as it stood, never converted into a pass.

They are gone. The repository owner decided on 2026-09-02 to release the
evidence-preservation constraint for the pre-stable phase; the epic's completion
criterion and its mandatory control were amended before the deletion, which is
recorded in `agent-ix/engineering-assurance#7` and executed under
`agent-ix/quire-contract-runtime#11`. Nothing was rewritten, backdated or
re-sealed. `agent-ix/engineering-assurance#21` closes as moot rather than as
fixed. The constraint re-applies unchanged at the move toward stable releases.

Two of the twelve states this migration keeps distinguishable — `unsupported` and
`malformed` — were demonstrated only by that compatibility census. Measured on
the pre-deletion tree, the chain alone reached ten of twelve. Both were
re-established on surfaces that read no retained byte: Quoin naming a declared
verification method its catalog does not have, and a producer row carrying the
outcome the mutation campaign emits when a mutation anchor is no longer present
exactly once. The census was not quietly reduced to ten.
