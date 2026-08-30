---
id: MP-001
title: Runtime v0.1 measurement plan
type: MeasurementPlan
status: proposed
owner: runtime-maintainers
metric: runtime_conformance_and_footprint
definition_version: quire-contract-runtime.measurement-v1
stage: gate
statistical_design:
  population: every supported feature set and public semantic boundary in the source candidate
  sampling: exhaustive truth tables plus boundary and property-generated integer cases
  repetitions: 1
  estimator: exact pass/fail counts and compiled artifact bytes
  error_model: toolchain configuration and bounded proof exploration
  uncertainty: retain skipped unavailable and inconclusive tool states
  decision_rule: escalate any failed gate missing identity or unresolved material gap
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: measures
---
# Runtime v0.1 measurement plan

## Decision Use

The measurements inform the human v0.1 source-release decision; they do not approve release or confer
validation or accreditation.

## Population

The population is the exact source revision across core, alloc, std, and proptest features; all
Boolean truth-table rows; checked integer boundaries; verdict mappings; and accounting transitions.

## Collection Procedure

Run `scripts/collect_evidence.sh`. It records source and tool identities, feature-matrix tests, Clippy,
license and unsafe checks, Kani availability/result, dependency metadata, release rlib size, and output
digests beneath `evidence/`. Preserve failures and skips rather than deleting them.

## Interpretation

A green run supports only the bounded source candidate. A skipped Kani run, absent governance gate,
or open human review remains an explicit limitation. Artifact byte size is compared with the 256 KiB
ceiling and is not treated as target RAM/ROM utilization.
