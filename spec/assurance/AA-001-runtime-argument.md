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
until the named human owner reviews the retained evidence and records a decision.

## Reasoning

Specification traceability, exhaustive small-domain tests, property and Kani harnesses, dependency
and license checks, unsafe audit, footprint measurement, and explicit gaps jointly address the known
failure scenarios without treating any one tool output as a release decision.

## Sufficiency Decision

No automated sufficiency decision is recorded. The human release owner must accept or reject the
candidate after PGM-01, code review, CI, and gap analysis are complete.

## Challenges

The cross-repository governance issue and human decision are deliberately open. Kani evidence may be
unavailable in a local environment and must then be recorded as skipped, not passed.

