---
id: SR-004
title: Closing code review — shared assurance migration
type: SpecReview
analysis: code-review
scope: "PR #9; SR-002 findings FND-201..FND-209; the independent adversarial review's twelve findings; exact-head gates"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/PLAN-002
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
---

# SR-004: Closing code review — shared assurance migration

## Summary

An independent adversarial review was run against this change with a single
instruction: find false greens. It ran 42 probes and found twelve findings,
**three of them high**, none of which SR-002 had found.

The three highs are one problem seen from three angles: **a gate that can be
silently disarmed, with no other gate noticing.** SR-002 had congratulated itself
for replacing a Makefile-text assertion with something that "runs the gates"; the
adversarial review demonstrated that deleting the word `test` from the `ci:`
prerequisite list removes the entire enforcement layer for FR-005 and every test
still passes. That is the same class of defect this repository charged against
itself as FND-108 during PR #7, reintroduced by the change that claimed to close
it.

The most uncomfortable finding is the one the reviewer identified as **PR #7's
FND-402 undone**: the mutation campaign and the Kani gate had no
failure-direction test at all. Hollowing `run_mutation` to `return "pass", None`
made `make kani-mutations` exit 0 in 0.03 seconds, emit an all-pass document,
turn the chain green, and leave every Rust test passing. The tests that used to
pin that behaviour lived in `tests/test_evidence_tooling.py`, which this change
deletes. Deleting a test suite alongside the framework it tested silently deleted
five domain controls with it.

All three highs are fixed. Every fix was re-probed with the reviewer's own
mutation, and each now goes red.

## Verdict

**CONDITIONAL.** Twenty-one findings across both reviews are dispositioned below:
eleven FIXED, seven ACCEPTED with rationale, three DEFERRED to filed issues.

## What the adversarial review changed about this change

**Three things were wrong in a way that mattered.**

*The producers had no failure direction.* Nothing in the repository asked whether
`run_kani_gate.collect` or `check_kani_mutations.run_mutation` could report
anything other than success. `scripts/check_kani_mutations.py` now runs the model
checker through a named seam, `prove`, and
`tc_010_the_producers_report_failure_when_the_prover_does` supplies a prover that
accepts the injected defect and requires `fail`, a prover that reports
`VERIFICATION:- FAILED` and requires failing rows, a zero-exit empty transcript
and requires `not-computed`, and counts below the declared floor and requires
`vacuous`. Hollowing either producer now fails that test.

*`make ci` could be disarmed by one line.* `.IGNORE:` at the top of the Makefile,
a `-` prefix on a recipe line, or `SHELL := /bin/true` each turn a failing gate
into `make` exit 0 while the gate still prints its own failure. The 311-line
recipe-failure policer that caught exactly this went with the collector. What
replaces it is not another policer target — that would be the Makefile attesting
to itself again — but an assertion in `tests/shared_assurance.rs` that the file
declares none of `.IGNORE`, `.SILENT`, `.ONESHELL`, `.SHELLFLAGS`, `SHELL` or
`MAKEFLAGS`, and that no recipe line is prefixed with `-`.

*The `ci:` graph check named eight scripts and no test runner.* Because
`assurance-inputs` supplies every one of those scripts, deleting `test` from the
prerequisite list left `make -n ci` still mentioning all eight while TC-009
through TC-014 never ran. `cargo test --all-features` is now in the required list.

**Two more that mattered nearly as much.** A 642-byte hand-written
`runtime.kani-proof/v1` with eight all-`pass` rows and no measurement fields
produced a fully green chain with no Kani run; `require_measurements` now refuses
a document that states an outcome without the observed tool version, the
discharged-obligation count, and the floor, and refuses a `pass` row that sits
below its own declared floor. And TC-014's frozen-schema census used an inclusion
list of eight directories, so a reintroduced validator at the repository root or
under `assurance/` was invisible; it now walks the repository root with an
exclusion list, which found a real reference on its first run.

## Gates at the exact final head

| Gate | Result |
| --- | --- |
| `make ci` on a clean tree | exit 0 |
| `make ci` from a fresh clone, own `CARGO_TARGET_DIR`, `.venv-assurance` built from scratch | exit 0 |
| `quire validate` | 53/53 documents grammar-clean, 0 structural failures |
| `quire coverage --strict` | 40/48 rows backed (83%); rust 22/22/22; the 8 unbacked rows are `spec/evidence/suites.md` |
| Rust tests | 27 passed, 0 failed, 0 ignored |
| Feature matrix | 9/9 rows |
| Kani | 7/7 harnesses, 0 failures, each at its declared obligation floor, `cargo-kani 0.67.0` |
| Kani mutations | 3/3 injected defects rejected |
| Footprint | 907 bytes, floor 500, ceiling 4096, 0 panic relocations, rustc 1.75.0, `thumbv7em-none-eabi` |
| Shared pins | 4/4 compatible, 0 artifact mismatches, 0 mirror references |
| Compatibility census | 16/16 cases, 42 envelopes, 3,412 files read, 0 bytes moved, 0 uncommitted |
| Compatibility mutation probes | 5/5 detected |
| Assurance chain | 14 scenarios, 6 controls, 7 adapter probes, all matched |
| Receipt | `incomplete`, reason `decision_missing` |
| `git diff 0bb51fb HEAD -- evidence/` | empty |
| `git diff 0bb51fb HEAD -- src/ verification/ measurement/` | empty |
| Hosted CI | not dispatched |

## Findings

Residual after this round. Nothing new was found that is not dispositioned below.

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-401 | medium | The obligation floors remain load-bearing in one direction only. Raising a floor above its measured count makes the gate report `vacuous`; lowering one is invisible. MP-001 now records the measured counts beside the declared floors, so a weakened floor is a two-file edit rather than a one-line one, but nothing compares them mechanically | `scripts/check_kani_harnesses.py`, `spec/assurance/MP-001-runtime-measurements.md` | correct-requirement-no-evidence |
| FND-402 | low | The frozen-artifact reference census excludes Markdown. `planning/pgm-01-reconciliation.md` names the PGM-01 envelope schema because it is a record of it, and prose cannot validate anything — but the exclusion is a scoping decision, stated here rather than left implicit | `tests/shared_assurance.rs` | wrong-requirement |
| FND-403 | low | The chain consumes what `make assurance-inputs` wrote and cannot verify Make ran the command it printed. TC-010 now checks the declared argv against `make -n assurance-inputs`, which closes the declaration half; the execution half is inherent | `Makefile`, `scripts/assurance_chain.py` | correct-requirement-no-evidence |

## Dispositions

### SR-002 findings

| ID | Severity | Disposition | Where |
| --- | --- | --- | --- |
| FND-201 | medium | ACCEPTED | Kani publishes no machine-readable result. The parser is confined to the domain tool that owns Kani; every downstream consumer reads a field; recorded as an open unknown in the sealed record. |
| FND-202 | medium | ACCEPTED | Inherent to Kani, which pins its own nightly. Recorded as `UNKNOWN-kani-compiles-a-third-compiler`, disposition `accepted`, and stated in MP-001. |
| FND-203 | medium | ACCEPTED | Stable libtest publishes a process exit status, not per-test JSON. Build and test phases are separated so a compile error is never reported as a test failure. |
| FND-204 | medium | DEFERRED | `agent-ix/engineering-assurance#21`. |
| FND-205 | medium | DEFERRED | `agent-ix/quire-contract-ir#21`. |
| FND-206 | low | ACCEPTED | Recorded in MP-001 and NFR-001 with its reason; the governed measurement is unchanged. |
| FND-207 | low | ACCEPTED | The population is named everywhere the figure appears. |
| FND-208 | low | FIXED | TC-010 compares every declared `command.argv` against `make -n assurance-inputs`. |
| FND-209 | medium | ACCEPTED | The lossy step is the attestation *summary*; the producer's document, carrying the finer state, is what intake retains. Confirmed independently by the adversarial review as forced by Quoin's four-value `result` enum and fail-closed. |

### Adversarial findings

| ID | Severity | Disposition | Where |
| --- | --- | --- | --- |
| RA-001 | **high** | **FIXED** | `scripts/check_kani_mutations.py` gains the `prove` seam; `tc_010_the_producers_report_failure_when_the_prover_does` exercises five failure directions across both producers. Re-probed with the reviewer's own mutations: hollowing `run_mutation` and hollowing `run_kani_gate.collect` both fail the test. |
| RA-002 | **high** | **FIXED** | TC-014 asserts the Makefile declares no failure-suppressing directive and prefixes no recipe line with `-`. Re-probed: `.IGNORE:`, a `-` prefix, and `SHELL := /bin/true` each fail it. |
| RA-003 | **high** | **FIXED** | `cargo test --all-features` added to the `make -n ci` required list. Re-probed: removing `test` from `ci:` fails TC-014 with "defined but unreachable". |
| RA-004 | medium | **FIXED** | `require_measurements` refuses a `runtime.kani-proof/v1` document with no observed tool version, or rows without `dischargedObligations`/`floor`, or a `pass` row below its own floor. Re-probed with the 642-byte forgery and with a forgery that supplies floors it does not meet: both refused, exit 2. |
| RA-005 | medium | **FIXED** | The census walks the repository root with an exclusion list. Re-probed: a validator at the root and one under `assurance/` are both caught. Widening it immediately found a real Markdown reference — see FND-402. |
| RA-006 | medium | ACCEPTED | Historical. `00ae6f8` widened `quire validate` to `reviews/**/*.md` one commit before `reviews/` existed, so `make spec` could not pass at that head. Fixed by `dbe39a4`, and worth recording that SR-002's own head was not one on which `make ci` returned 0. |
| RA-007 | medium | **FIXED** | `declared_schema_version` reports `unknown` for a record that declares none or is unreadable, instead of raising `TypeError` on `sorted({..., None})`. A malformed retained record is one of the twelve states; it does not get to be a traceback. |
| RA-008 | medium | **FIXED** | TC-007's Test Procedure and Expected Results, `spec/test-matrix.md`'s TC-007 evidence line, and NFR-001's metric table and Verification section no longer name deleted machinery. |
| RA-009 | low | ACCEPTED, mitigated | MP-001 now records the measured obligation counts beside the declared floors. Carried as FND-401. |
| RA-010 | low | **FIXED** | `check_kani_mutations` publishes the observed `cargo-kani --version` string, or `null`. The placeholder `"observed"` in a field named `version` is gone. |
| RA-011 | low | **FIXED** | SUR-001's SUITE-005 restates the full argv the Makefile runs. |
| RA-012 | low | **FIXED** | The three adapter refusals now carry `state: None`. `unsupported`, `malformed` and `vacuous` are demonstrated by cases that produced them — the compatibility view against real records, and Quoin's own audit against a run it read — not by the adapter declining to transcribe. |

**One stale claim in SR-002 is corrected here.** SR-002's inherited-classes table
said the removed Makefile-text assertion was fine because "wiring is proven by
`tests/shared_assurance.rs`, which runs the gates". RA-003 showed that was false
for `test`: the gates it ran were reachable through `assurance-inputs` whether or
not the test target was in `ci:`. It is true now, and it was not true when it was
written.

## What this review does not claim

It does not claim the crate is correct beyond leaving it unchanged. It does not
claim the seven harnesses prove the crate; they are bounded controls and four of
six modules carry none. It does not claim the probe set is exhaustive — the
adversarial review found three highs that a 30-probe self-review did not, which is
the strongest available evidence that a self-review's probe set is not the same
thing as an adversarial one.

It confers no qualification, certification or accreditation, and it is not a
substitute for the human source-release decision, which remains open.
