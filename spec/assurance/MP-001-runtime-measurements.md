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

The v0.1 linked-footprint population is versioned with this plan. Its static-library consumer calls
every public runtime constructor, `CampaignReport::record_verdict`,
`CampaignReport::record_discard`, and every Boolean, option, index, and checked-integer operator
family. TC-007 parses the harness and requires those call expressions, so changing the population
requires an explicit measurement-plan and test update. The harness is a workspace member and uses
the root release profile (`lto = "thin"`, one codegen unit, and aborting panics), not a private profile.

## Collection Procedure

Run `scripts/collect_evidence.sh`. It records source and tool identities, feature-matrix tests, Clippy,
license and unsafe checks, Kani availability/result, dependency metadata, the Rust 1.75
`thumbv7em-none-eabi` linked footprint, observational release rlib bytes, and output digests beneath
`evidence/`. The builder derives and verifies the digest of the vendored PGM-01 envelope schema;
TC-007 tests the builder and local validator semantics before evidence collection can pass. Preserve
failures and skips rather than deleting them.

## Interpretation

A green run supports only the bounded source candidate. A skipped Kani run, absent governance gate,
or open human review remains an explicit limitation. The representative consumer's runtime/harness
`.text` plus `.rodata` is compared with the fixed 4 KiB ceiling. The rlib byte count is retained only
as an observation because it varies with compiler metadata and build paths; neither value is treated
as whole-application RAM/ROM utilization.
