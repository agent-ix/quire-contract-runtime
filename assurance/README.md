# Shared assurance

Two files and no evidence.

`change-assurance.json` is what this repository *states* about the change under
[issue #8](https://github.com/agent-ix/quire-contract-runtime/issues/8): the
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
matrix, the linked-footprint measurement, the MSRV build, `quire coverage`, and
the retained-evidence compatibility view, and writes their structured output to
`target/assurance/`.

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

## The compatibility answer, stated plainly

This repository never retained a `quire.pgm01-evidence` record. Its 42 retained
envelopes are `quire.derivation-evidence/v1` — a different schema family, which
the PGM-01 programme governed but did not define. The pinned mapping
`engineering_assurance.verification_semantics.map_pgm01_bytes` therefore answers
`incompatible` for every one of them, with the reason "unknown PGM-01 schema
version".

That is the mapping declining to interpret a shape it has never seen, which is
exactly what it should do and is one of the twelve states this migration is
required to keep distinguishable. It is not a pass, it is not a failure of these
records, and it is not a licence to write a local mapper that would return a
friendlier answer. The gap is filed upstream as
`agent-ix/engineering-assurance#21`, which records 142 such envelopes across six
of the eight campaign repositories.

The mapping is shown to accept as well as refuse: the pinned release's own
`fixtures/verification-semantics/pgm01-v1.json` and `pgm01-v2.json` are read as
positive controls in the same run. A refusal that has never been seen to accept
is indistinguishable from a step that never worked.
