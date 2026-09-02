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
family. TC-007 parses the harness and requires those call expressions, executes the population at two
fixed inputs with exact expected results, and requires the linked artifact to stay above the fixed
500-byte population floor with no panic-path references from its runtime/harness objects. Changing or making the population unreachable
therefore requires an explicit measurement-plan and test update. The harness is a workspace member
and uses the root release profile (`lto = "thin"`, one codegen unit, and aborting panics), not a
private profile.

## Collection Procedure

Run `make assurance-inputs`. It is the only target that executes a producer, and each command it runs
is the exact argv the corresponding proof obligation declares in `assurance/change-assurance.json`; a
declared command that is not the executed command would be a lie in a sealed attestation.

Four producers belong to this repository and each publishes a declared structured result rather than
a transcript:

- `scripts/run_feature_matrix.py` publishes `runtime.feature-matrix/v1`. It runs nine rows — four
  feature sets over the crate's own test targets, their four doc-test lanes, and the footprint
  package — and decides each row from two structured channels: cargo's own
  `--message-format=json` `compiler-message` level for the build phase, and libtest's process exit
  status for the test phase. Build failure and test failure are different facts and are reported
  separately. Per-test granularity would require libtest's unstable JSON formatter and therefore a
  different compiler than the one the crate ships on; that limitation is stated rather than papered
  over by parsing `test result: ok.` out of human output.
- `scripts/run_kani_gate.py` publishes `runtime.kani-proof/v1`, one row per declared harness plus a
  suite-census row. Kani publishes no machine-readable result, so this producer is the single place in
  the repository where a transcript is parsed; every downstream consumer reads a field. A harness
  passes only when it was checked, verification succeeded, and it discharged at least its declared
  positive obligation floor. A harness that verified below its floor is `vacuous`, a declared harness
  the transcript never mentions is `not-computed`, and an absent `cargo-kani` makes every row
  `unavailable`. `make kani` exits non-zero on any of those, so a green local run means the proofs ran
  here.
- `scripts/check_kani_mutations.py` publishes `runtime.kani-mutation/v1`. It injects three
  representative Boolean, arithmetic, and accounting defects into a scratch copy of the source — never
  into the working tree — and requires the owning proof to reject each one. A non-zero exit that never
  reached a verification failure is reported as a failure, not as a control that held.
- `scripts/measure_footprint.py` publishes `runtime.footprint/v1`. It links the footprint staticlib on
  the declared MSRV compiler for `thumbv7em-none-eabi` and then measures it through
  `scripts/check_linked_footprint.sh`, which owns `size` and `objdump` and emits the same document.
  Nothing re-derives its numbers; a second implementation of a measurement is a second answer.

The declared obligation floors, and the counts measured at this candidate revision. They are equal
today by construction, not by any check: raising a floor above its measured count makes the gate
report `vacuous`, but lowering one is invisible to the gate, so the measurement is recorded here and
a lowered floor becomes a two-file edit rather than a one-line one.

| Harness | Declared floor | Measured |
|---|---|---|
| `tc_001_public_model_preserves_provenance` | 140 | 140 |
| `tc_002_boolean_truth_tables` | 136 | 136 |
| `tc_003_campaign_accounting_saturates` | 264 | 264 |
| `tc_003_checked_i8_arithmetic_matches_primitives` | 59 | 59 |
| `tc_003_i32_division_boundaries_are_undefined` | 43 | 43 |
| `tc_003_option_helpers_preserve_definedness` | 52 | 52 |
| `tc_003_slice_index_is_defined_exactly_in_bounds` | 24 | 24 |

`scripts/check_kani_harnesses.py` remains the cheap static half of the proof gate: it makes the seven
proof names and their TC-001/TC-002/TC-003 ownership an executable census, so a deleted, renamed, or
`cfg`-ed-out harness is caught without needing the model checker at all. It also reads each harness's
trace binding out of the harness source, so the published result document carries a binding that
cannot silently disagree with a second copy.

Two further inputs are not this repository's producers. `quire coverage --scope . --json` is the
static specification, obligation, and coverage export; Quire executes nothing.
`scripts/legacy_evidence_view.py` reads every retained evidence byte through
`engineering_assurance.verification_semantics.map_pgm01_bytes` from the pinned release and reports
what came back; it implements no mapping, digests the whole tree before and after so that read-only
is measured rather than asserted, and asks Git whether any retained byte differs from what was
committed.

`scripts/assurance_chain.py` then drives the official chain over those already-written bytes: it
seals the change-assurance record, seals one proof attestation per obligation, hands the producer's
bytes to Quoin's intake, and asks for a verification receipt. It reads every attested result out of
the bytes the producer wrote. It runs no producer, and that is asserted behaviourally by two runs —
one with cargo, cargo-kani, rustup and rustc replaced by logging stubs, requiring the log to be
empty, and a control that stubs `quoin` and requires the chain to fail, because an empty log and an
unconsulted `PATH` are otherwise the same observation.

Retention, integrity checking, audit, attestation, and receipts are Quoin's. Compatibility with the
retained record family is Engineering Assurance's. This plan no longer describes a local collector, a
local envelope builder, a local verifier, a local anchor census, or a Make recipe that polices its
own execution controls, because none of those exist here any more.

The following design extensions remain explicitly deferred beyond this source candidate: adding a
second independent Kani-to-coverage semantic oracle (FND-409), and expanding the representative
mutation set into exhaustive operator/verdict mutation coverage (FND-414). The current controls do
not claim those classes closed.

## Interpretation

A green run supports only the bounded source candidate. A skipped Kani run, absent governance gate,
or open human review remains an explicit limitation. The representative consumer's runtime/harness
`.text` plus `.rodata` is compared with the fixed 500-byte population floor and 4 KiB ceiling, and
its runtime/harness objects are rejected if they retain a panic-path reference. That value is not
treated as whole-application RAM/ROM utilization.

The observational release-rlib byte count is retired with this change. It gated nothing by design —
it varies with compiler metadata and build paths — and the deleted collector was its only caller, so
it was a number nobody could act on collected by a tool that no longer exists. The governed
measurement, which is the linked `.text` plus `.rodata` figure above, is unchanged in definition,
floor, ceiling, target, and compiler. Removing an observation is recorded here rather than left to be
noticed in a diff.

The seven Kani harnesses are bounded verification controls, not a whole-crate proof. Public-model
provenance and Boolean assertions gate constructors and dispatch, while i8 checked arithmetic uses
independent i16 widening oracles. Division/remainder uses symbolic invalid inputs, index
definedness quantifies over full `usize`, and campaign accounting drives the public record/discard
paths from symbolic near-overflow states to cover all five increments and the saturating total. The Kani
toolchain may differ from both the shipped stable compiler and the Rust 1.75 compatibility compiler;
that exact tool identity is retained rather than treated as compiler equivalence. Retained local
transcripts are source-bound and Git-tamper-evident, but they are not externally signed runner
attestations.

An empty `cargo fmt --check` transcript is its documented success form; its numeric zero status is
still bound to the source and tool digests. Historical local oracle records are corroborating audit
history only, never an independent attestation or substitute for fresh human review of this source
candidate.
