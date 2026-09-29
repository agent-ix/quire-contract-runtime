---
id: MP-001
title: Runtime v0.1 measurement plan
type: MeasurementPlan
status: proposed
owner: runtime-maintainers
metric: runtime_conformance_and_footprint
definition_version: quire-contract-runtime.measurement-v3
stage: gate
ground_truth_kind: mechanical
objective:
  direction: zero
statistical_design:
  population: every supported feature set and public semantic boundary in the source candidate
  sampling: exhaustive truth tables plus boundary and property-generated integer cases
  repetitions: 1
  estimator: count
  error_model: toolchain configuration and bounded proof exploration
  uncertainty: retain skipped unavailable and inconclusive tool states
  decision_rule:
    comparator: le
    threshold: 0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: measures
---
# Runtime v0.1 measurement plan

## Decision Use

The measurements inform the human v0.1 source-release decision; they do not approve release or confer
validation or accreditation.

## Decision Rule

The estimate is a `count` of escalation items in one run over the source candidate. Each of the
following is one item:

- a failed gate: any `make ci` prerequisite that exits non-zero, including the linked footprint
  falling outside its 500-byte floor and 4 KiB ceiling or retaining a panic-path reference; and
- an unresolved material gap: an open finding against this candidate that is neither closed nor
  explicitly deferred in this plan.

The rule `{ comparator: le, threshold: 0 }` holds only when that count is zero, which is the
`objective.direction: zero` reading: any non-zero count escalates to the human release decision.
The linked footprint's byte count is judged by the footprint gate; it is not the estimate, and the
byte count itself is never compared with the threshold.

## Population

The population is the exact source revision across core, alloc, std, and proptest features; all
Boolean truth-table rows; checked integer boundaries; verdict mappings; and accounting transitions.

The linked-footprint population's static-library consumer calls every public runtime constructor,
`CampaignReport::record_verdict`, `CampaignReport::record_discard`, and every Boolean, option, index,
and checked-integer operator family. TC-007 parses the harness and requires those call expressions,
executes the population at two fixed inputs with exact expected results, and requires the linked
artifact to stay above the 500-byte population floor with no panic-path references from its
runtime/harness objects. The harness is a workspace member and uses the root release profile
(`lto = "thin"`, one codegen unit, and aborting panics), not a private profile.

## Collection Procedure

Run `make ci`. The measurements come from these targets:

- `make test-features` runs `scripts/run_feature_matrix.py`: five feature sets (core, alloc, std,
  snapshot-json, all features) over the crate's own test targets, their five doc-test lanes, the
  footprint package, the `exact` oracle tests, and two no_std MSRV library builds (snapshot-json and
  exact). Each row's build phase is read from cargo's `--message-format=json` `compiler-message`
  level and its test phase from libtest's exit status, so a build failure and a test failure are
  reported separately.
- `make kani` runs `cargo kani --features exact` over the harnesses in `verification/kani.rs`. It
  exits non-zero when a proof fails or `cargo-kani` is absent.
- `make kani-mutations` runs `scripts/check_kani_mutations.py`. It injects five representative
  defects — Boolean, arithmetic, accounting, and two in the exact IEEE comparison — into a scratch
  copy of the source, never into the working tree, and requires the owning proof to reject each one.
  A harness that verifies the mutated source anyway is `fail`. A run that never reaches a
  verification result is `inconclusive`, so a broken build is never read as a hollow harness or as
  a control that held.
- `make size` links the footprint staticlib on the MSRV compiler for `thumbv7em-none-eabi` and
  measures it with `scripts/check_linked_footprint.sh`, which owns `size` and `objdump`.

The following design extensions remain explicitly deferred beyond this source candidate: adding a
second independent Kani-to-coverage semantic oracle (FND-409), and expanding the representative
mutation set into exhaustive operator/verdict mutation coverage (FND-414). The current controls do
not claim those classes closed.

## Interpretation

A green run supports only the bounded source candidate. A skipped Kani run, absent governance gate,
or open human review remains an explicit limitation. The representative consumer's runtime/harness
`.text` plus `.rodata` is compared with the 500-byte population floor and 4 KiB ceiling, and its
runtime/harness objects are rejected if they retain a panic-path reference. That value is not
treated as whole-application RAM/ROM utilization.

The Kani harnesses are bounded verification controls, not a whole-crate proof. Public-model
provenance and Boolean assertions gate constructors and dispatch, while i8 checked arithmetic uses
independent i16 widening oracles. Division/remainder uses symbolic invalid inputs, index
definedness quantifies over full `usize`, and campaign accounting drives the public record/discard
paths from symbolic near-overflow states to cover all five increments and the saturating total. The
Kani toolchain may differ from both the shipped stable compiler and the Rust 1.75 compatibility
compiler.
