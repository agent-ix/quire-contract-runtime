---
id: SUR-001
title: "Contract runtime v0.1 evidence suite registry"
type: SuiteRegistry
---

# Contract runtime v0.1 evidence suite registry

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-001 | Kani proof suite over the operator, accounting, and public-model surfaces | `python3 scripts/run_kani_gate.py --json` | cargo-kani 0.67.0 / CBMC | Analysis |
| SUITE-002 | Kani semantic mutation campaign | `python3 scripts/check_kani_mutations.py --json` | cargo-kani 0.67.0 / CBMC | Analysis |
| SUITE-003 | Feature-matrix build and test run | `python3 scripts/run_feature_matrix.py --json` | cargo / libtest | Integration |
| SUITE-004 | Governed linked-footprint measurement | `python3 scripts/measure_footprint.py --json` | rustc 1.75.0, binutils `size` and `objdump` | Static |
| SUITE-005 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' --summary` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-006 | Static specification and coverage export | `quire coverage --scope . --json` | quire 0.31.0 / quire-rs 0.46.0 | Static |
| SUITE-007 | Retained-evidence compatibility view | `.venv-assurance/bin/python scripts/legacy_evidence_view.py --json` | engineering-assurance 0.2.0 `map_pgm01_bytes` | Static |
| SUITE-008 | Shared assurance intake chain | `python3 scripts/assurance_chain.py --candidate-revision <sha>` | quoin 0.23.1 change-assurance and evidence surfaces | Integration |

## Notes

This registry is new with the shared-assurance migration. Before it, the retained
records named their commands only inside a collection manifest, which meant the
suite a result discharged an obligation for was a fact recoverable only by
reading the collector's shell script.

SUITE-001 is the suite whose run this repository transcribes into Quoin's
evidence store, because the Kani proofs are the crate's headline verification and
their rows carry per-harness trace bindings read from the harness source itself.

SUITE-005 through SUITE-008 were previously performed by the deleted collector:
schema validation, envelope conformance, the local traceability reimplementation,
and the local verifier. Every one of those concerns moved upstream. Quire is the
authority on static specification, obligation and coverage facts; Quoin owns
intake, retention, audit and receipts; Engineering Assurance owns the read-only
mapping of retained bytes.

`make ci` is deliberately not a suite. A suite whose command is "everything"
cannot say which obligation a result discharged, and `make ci` is a gate rather
than a producer of transcribable results.
