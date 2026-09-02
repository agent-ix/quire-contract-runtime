---
id: SR-005
title: Closing gap analysis — shared assurance migration
type: SpecReview
analysis: gap-analysis
scope: "PLAN-002 at the final head; FR-005 and TC-009..TC-014; SR-003 findings FND-301..FND-306; the traceability census after the adversarial round"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/PLAN-002
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
---

# SR-005: Closing gap analysis — shared assurance migration

## Summary

PLAN-002's three tasks are complete and each names an artifact that exists.
FR-005's six acceptance criteria each have a matrix row, a test case artifact,
and at least one Rust test carrying the trace tag Quire's census reads. The
enforcement layer grew by two tests during the adversarial round — a
failure-direction test for the producers, and the `ci:` graph check that now
includes the test runner — and TC-010 is now backed by four symbols rather than
two.

The one gap this round closed that SR-003 did not see is the most important one:
**SR-003 asserted that the matrix rows are backed by symbols that run, without
asserting that anything runs them.** Deleting one word from the `ci:` prerequisite
list made every TC-009..TC-014 symbol unreachable from `make ci` with the whole
suite still green. Backing and reachability are different properties and only the
first was checked.

## Task completeness

| Task | Status | Named artifact | Present |
| --- | --- | --- | --- |
| Task-001 inventory and pins | done | `assurance/pins.json`, `scripts/check_shared_pins.py` | yes |
| Task-002 shared intake | done | four producers, `scripts/assurance_chain.py`, `scripts/legacy_evidence_view.py`, `tests/shared_assurance.rs` | yes |
| Task-003 dual run and deletion | done | the dual-run table in SR-002; the deletion commit `dace2e4` | yes |

No task is marked done while naming a file that does not exist. PLAN-001
Task-006, the human source-release decision, remains `not_started`.

## Matrix backing, and reachability

`quire coverage --scope . --json` at the final head:

```
Coverage: 40/48 rows backed (83%)
rust:   22/22/22 bound/tagged/candidates (100% read; 100% authored)
spec/functional/FR-005-shared-assurance-intake.md: 6/6 (100%)
spec/test-matrix.md:                              14/14 (100%)
spec/evidence/suites.md:                           0/8 (0%)
```

| Criterion | Test case | Rust symbols |
| --- | --- | --- |
| FR-005-AC-1 | TC-009 | `tc_009_every_shared_pin_is_classified_by_the_packaged_matrix` |
| FR-005-AC-2 | TC-010 | `tc_010_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer`, `tc_010_the_chain_never_executes_a_producer_and_the_probe_can_prove_it`, `tc_010_every_declared_proof_command_is_the_command_make_actually_runs`, `tc_010_the_producers_report_failure_when_the_prover_does` |
| FR-005-AC-3 | TC-011 | `tc_011_the_sealed_records_impact_snapshot_is_the_quire_export` |
| FR-005-AC-4 | TC-012 | `tc_012_retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte` |
| FR-005-AC-5 | TC-013 | `tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` |
| FR-005-AC-6 | TC-014 | `tc_014_no_local_evidence_framework_remains_and_the_frozen_schemas_bind_nothing` |

`cargo test --all-features` reports **27 passed, 0 failed, 0 ignored**. The
zero-ignored figure is taken from the test runner and not from the coverage
census, because `quire coverage` reads backing from source text and an
`#[ignore]`d test counts as backing.

**Reachability is now asserted separately from backing.** TC-014 runs
`make -n ci` and requires `cargo test --all-features` and the eight
shared-assurance scripts to appear in the plan Make would execute. Removing any
of them from the `ci:` prerequisite list fails the test with "defined but
unreachable".

## Code with no owning requirement

Unchanged from SR-003 except for the two new seams, both of which trace to
FR-005-AC-2:

| File | Owning requirement | Note |
| --- | --- | --- |
| `scripts/check_kani_mutations.py::prove` | FR-005-AC-2, NFR-002 | a named test seam, and the reason it exists is a finding: without it the campaign's failure direction cannot be exercised |
| `scripts/assurance_chain.py::require_measurements` | FR-005-AC-2, FR-005-AC-5 | refuses a document that states an outcome without the measurement behind it |
| `spec/evidence/suites.md` | — | a suite registry; it declares rather than verifies, and contributes the eight unbacked census rows |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-501 | medium | Backing and reachability are different properties. SR-003 checked the first and asserted the second. `make -n ci` now checks reachability; the general lesson is that a coverage census cannot tell you whether anything runs the symbols it counts | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-502 | medium | Four of the crate's six modules carry no Kani harness. Inherited from PR #7 and unchanged here | `verification/kani.rs` | correct-requirement-no-evidence |
| FND-503 | medium | `quire coverage --strict` cannot check the Coverage Status column: the installed module configures the `Status` header and the authored table uses `Coverage Status` | `spec/test-matrix.md` | wrong-requirement |
| FND-504 | low | `spec/evidence/suites.md` contributes 8 unbacked coverage rows. A SuiteRegistry has no acceptance criteria to back | `spec/evidence/suites.md` | wrong-requirement |
| FND-505 | low | The 42 retained envelopes are read but cannot be interpreted; the pinned mapping covers only `quire.pgm01-evidence` v1 and v2 | `assurance/pins.json` | wrong-requirement |
| FND-506 | low | The pinned compatibility matrix records `pending_human_acceptance` and ships no `human_acceptance_recorded` predicate; the acceptance exists only on `engineering-assurance` main | `requirements-assurance.txt` | wrong-requirement |
| FND-507 | low | No ix-flow decision event exists, so the verification receipt reads `incomplete` with reason `decision_missing` | `assurance/change-assurance.json` | correct-requirement-no-evidence |

## Dispositions

| ID | Disposition | Where |
| --- | --- | --- |
| FND-501 | FIXED | TC-014 asserts the `ci:` graph reaches the test runner and the eight gates. Probed: removing `test` fails it. |
| FND-502 | DEFERRED | `agent-ix/quire-contract-runtime#4`. Not introduced here; the proof scope is stated in MP-001 and the matrix. |
| FND-503 | DEFERRED | `agent-ix/quire-contract-ir#21`. |
| FND-504 | ACCEPTED | The population is named wherever the figure appears. |
| FND-505 | DEFERRED | `agent-ix/engineering-assurance#21`. |
| FND-506 | DEFERRED | `agent-ix/engineering-assurance#20`. Reported, never gated on, no branch head substituted for the tag. |
| FND-507 | OPEN | Owner `@kreneskyp`. It is the decision the whole chain exists to wait for. |

SR-003's FND-301..FND-306 are carried forward unchanged as FND-504, FND-502,
FND-503, FND-505, FND-506 and FND-507 respectively; none was closed by the
adversarial round and none was found to be misstated.

## What this analysis does not claim

It does not claim the crate's semantics are fully verified. It claims the matrix
rows that exist are backed by symbols that run, and — as of this round — that
something runs them. It does not claim 100% of anything without naming what the
percentage is over, and it does not claim the enforcement layer is complete: an
adversarial review found three ways to disarm it that a thirty-probe self-review
did not.
