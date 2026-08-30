---
id: AP-001
title: Quire contract runtime v0.1 decision profile
type: AssuranceProfile
status: proposed
owner: human-release-owner
profile_version: 0.2
profile_kind: general
scope: one identified quire-contract-runtime v0.1 source candidate
impact_assessments:
  - id: impact-vacuous-pass
    scenario: a rejected precondition is represented as successful evidence
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-contract-runtime/CAC-001
review_policy:
  mode: require
  operations: [code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: references
---
# Quire contract runtime v0.1 decision profile

## Decision Boundary

This profile covers one source revision and declared Cargo feature set. The crate supplies validation
evidence only; release does not confer project-specific validation, accreditation, or certification.

## Impact Scenarios

Material scenarios are a rejected precondition becoming a pass, a partial operation panicking, total
operators skipping work, counters omitting non-success cases, or a dependency/licensing change
expanding the trusted boundary.

## Evidence Policy

Evidence identifies source revision, tool version, feature selection, inputs, outputs, and digest.
Missing, inconclusive, rejected, discarded, or failed results remain visible. A human release owner
alone judges sufficiency.

## Exceptions

No standing exceptions exist. An exception requires an owner, rationale, affected requirement,
expiry, and explicit human acceptance under PGM-01.
