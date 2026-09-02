---
id: AA-001
title: Runtime v0.1 assurance argument
type: AssuranceArgument
status: proposed
owner: human-release-owner
profile: ix://agent-ix/quire-contract-runtime/AP-001
top_claim:
  id: claim-runtime-v01
  statement: the identified runtime source candidate is acceptable for bounded v0.1 use
  subject: quire-contract-runtime v0.1 source candidate
  status: open
reasoning:
  - id: reasoning-semantic-conformance
    statement: evaluate requirement-tagged tests and measurements against the declared boundary
    supports: claim-runtime-v01
    sufficiency_criteria:
      - all CI and feature-matrix gates pass
      - no blocking specification or gap-review finding remains
assumptions:
  - id: assumption-consumer-validation
    statement: consuming projects validate the pinned crate for their own intended use
    owner: human-release-owner
    status: open
    review_by: "2026-12-31T00:00:00Z"
participants:
  - id: human-release-owner
    role: decision owner
    authority: accept or reject the bounded source candidate
    independence: reviews agent-assisted implementation and evidence
challenges:
  - id: challenge-governance
    target: claim-runtime-v01
    statement: PGM-01 and the human v0.1 decision must be closed outside automated implementation
    status: open
    owner: human-release-owner
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: references
---
# Runtime v0.1 assurance argument

## Claim

The bounded claim concerns only an identified source revision and feature matrix. It remains open
until the named human owner reviews the attested results and records a decision.

## Reasoning

Specification traceability, exhaustive small-domain tests, property and Kani harnesses, dependency
and license checks, unsafe audit, footprint measurement, and explicit gaps jointly address the known
failure scenarios without treating any one tool output as a release decision.

## Sufficiency Decision

No automated sufficiency decision is recorded. The human release owner must accept or reject the
candidate after PGM-01, code review, CI, and gap analysis are complete.

The automated consumer enforces this bounded input declaration before the human decision. It no
longer names a local anchor file, because retention is no longer this repository's job:

```yaml
evidence_binding:
  declaration: assurance/change-assurance.json
  pins: assurance/pins.json
  record: quoin change-assurance seal-record
  proof_obligations: 6
  attestation_result_source: the bytes each producer wrote, read as a structured field
  receipt: quoin change-assurance receipt
  expected_receipt_outcome: incomplete
  expected_receipt_reason: decision_missing
```

`incomplete` is the correct outcome and not a defect. The receipt is incomplete precisely because no
attributed human decision event exists, which is the fact this argument's top claim is waiting on. A
receipt that read `valid` without one would be asserting that a person looked.

## Challenges

The cross-repository governance issue and human decision are deliberately open.

A green `make ci` is bounded in a way worth stating in an assurance argument. It
attests that the gates ran and passed on the tree as committed; it does not
attest to a tree whose Makefile has been edited. Measured: `.IGNORE:` prepended
to the Makefile takes `make ci` from exit 2 to exit 0 with three real defects
present. The chain is unaffected for anything that feeds it — attested results
are derived from producer bytes, so a suppressed producer errors rather than
passes — but the gates that feed nothing into the chain are neutered. Tracked as
agent-ix/quire-contract-runtime#10.

Kani evidence that could not be produced is recorded as `unavailable` — one of the twelve
distinguishable states — and the Kani gate exits non-zero. An earlier form of this argument said such
a run "must then be recorded as skipped, not passed"; that was too weak. A gate that stands down when
its dependency is absent returns the same exit code as one that ran, so an absent model checker now
fails the gate and the absence is reported in the attested result rather than being absorbed into a
green run.
