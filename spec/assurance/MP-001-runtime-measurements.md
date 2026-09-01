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

Run `scripts/collect_evidence.sh`. It records source and tool identities, feature-matrix tests, Clippy,
license and unsafe checks, Kani availability/result, dependency metadata, the Rust 1.75
`thumbv7em-none-eabi` linked footprint, observational release rlib bytes, and output digests beneath
`evidence/`. Every invoked gate retains stdout, stderr, and its numeric exit status; the builder
`scripts/build_evidence_envelope.py` derives manifest outcomes from the complete status-file census,
represents missing or uncorroborated records as inconclusive, and records zero-status transcript
contradictions as durable failed outcomes. Kani passes only when its numeric status is zero, every
declared harness is named successful, each harness discharges a positive recorded check count, the
exact complete-summary count is present, and no failure marker occurs. Its exact version is retained.
The collector's local self-test exercises nonzero propagation and checksum fixed-point detection;
the builder never manufactures a pass from a command name or transcript text. The builder derives and verifies
the digest of the vendored PGM-01 envelope schema, the byte-identical vendored validator, and the
identity of its vendored raw merge commit; verification never depends on the collector's temporary
external-checkout path;
TC-007 tests the builder and local validator semantics before evidence collection can pass. Preserve
failures and skips rather than deleting them.

`scripts/check_kani_harnesses.py` makes the seven proof names and their TC-001/TC-002/TC-003 ownership an
executable census before every Kani run. `scripts/check_kani_mutations.py` injects representative
Boolean, arithmetic, and accounting defects and requires the owning proofs to reject them. Absence
of `cargo-kani` is a failed local gate, not a skip.
The shared quantitative floors for the evidence-tool suite and mutation campaign live in
`scripts/evidence_policy.py`; both counts are retained in the manifest and independently re-derived.
`scripts/check_coverage_status.py` independently enforces the schema-valid `Coverage Status` column,
complete backing, and the absence of ignored trace-bearing tests while the installed module still
configures the incompatible `Status` header. `scripts/update_evidence_anchors.py` deterministically
regenerates the complete anchor census for diff review after collection.

`scripts/check_failure_propagation.py` rejects ambient or in-file Make failure suppression, probes
every mandatory recipe position, and verifies the exact local `cargo`, `python3`, `quire`, and Make
executables before composite local checks. `scripts/run_kani_gate.py` preserves Kani's numeric result
and rejects success unless every harness meets its proof-obligation floor; it uses the unavailable
exit channel when the mandatory tool is absent. `scripts/run_evidence_tests.py` enforces a minimum
behavioral-test census before running the evidence-tool suite.
`scripts/check_assurance_anchor.py` executes AA-001's declared authoritative-record, outcome-count,
and conclusive-result binding after evidence verification.

The collector's declared command list is generated in transcript order and bound to every
`run_and_retain` call site. Stable Cargo/rustc and MSRV rustc digests name the rustup-resolved
toolchain binaries rather than the rustup shims; the verifier re-executes their version identities.
A present but different local toolchain is verification-unavailable, not evidence tampering. Source
binding enumerates ignored and non-ignored untracked paths without consulting `.gitignore`, allowing
only retained `evidence/` and generated `target/` content. Historical disposition sidecars and the
legacy in-envelope form are parsed and checked against closed shapes by the verifier.

The following design extensions remain explicitly deferred beyond this source candidate: adding a
second independent Kani-to-coverage semantic oracle (FND-409), embedding self-referential record
names and digests in AA-001 (FND-412), and expanding the representative mutation set into exhaustive
operator/verdict mutation coverage (FND-414). The current controls do not claim those classes closed.

`scripts/verify_evidence.py` independently verifies the committed `evidence/ANCHORS` record-set
boundary, every flat-record checksum and artifact/link digest, the recursively anchored historical
tree, exact JSON Schema formats, the external merged-PGM schema digest, independent outcome-name
census, complete re-derived outcome values, result status/summary, and limitations. It also binds
the recorded non-evidence source tree and clean worktree to current `HEAD`. Missing anchors, an empty
record set, or unavailable schema tooling mean verification unavailable, never success; a JSON
status file preserves that channel across GNU Make's exit-code collapse. `scripts/validate_json_schema.py`
fails closed unless the exact packages in `requirements-evidence.txt` provide checkers for every
format named by the supplied schema.

## Interpretation

A green run supports only the bounded source candidate. A skipped Kani run, absent governance gate,
or open human review remains an explicit limitation. The representative consumer's runtime/harness
`.text` plus `.rodata` is compared with the fixed 500-byte population floor and 4 KiB ceiling, and
its runtime/harness objects are rejected if they retain a panic-path reference. The rlib byte count is retained only
as an observation because it varies with compiler metadata and build paths; neither value is treated
as whole-application RAM/ROM utilization.

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
