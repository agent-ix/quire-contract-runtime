---
id: SR-003
title: Shared assurance migration gap analysis
type: SpecReview
analysis: gap-analysis
scope: "PLAN-002 tasks 001-003; FR-005 and TC-009..TC-014; the traceability census at 00ae6f8"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/PLAN-002
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
---

# SR-003: Shared assurance migration gap analysis

## Summary

PLAN-002's three tasks are complete and each names an artifact that exists.
FR-005's six acceptance criteria each have a matrix row, a test case artifact,
and a Rust test that carries the trace tag Quire's census reads. The
traceability census is 40/48 rows backed, and the eight unbacked rows are all in
`spec/evidence/suites.md`, which is a suite registry and has no test cases of its
own — that number is quoted with its population rather than as 100% of a smaller
one.

## Task completeness

| Task | Status | Named artifact | Present |
| --- | --- | --- | --- |
| Task-001 inventory and pins | done | `assurance/pins.json`, `scripts/check_shared_pins.py` | yes |
| Task-002 shared intake | done | four producers, `scripts/assurance_chain.py`, `scripts/legacy_evidence_view.py`, `tests/shared_assurance.rs` | yes |
| Task-003 dual run and deletion | done | the dual-run table in SR-002; the deletion commit | yes |

No task is marked done while naming a file that does not exist. PLAN-001
Task-006, the human source-release decision, remains `not_started` and is not
touched by this change.

## Matrix backing

`quire coverage --scope . --json` at `00ae6f8`:

```
Coverage: 40/48 rows backed (83%)
rust:   22/22/22 bound/tagged/candidates (100% read; 100% authored)
spec/functional/FR-005-shared-assurance-intake.md: 6/6 (100%)
spec/test-matrix.md:                              14/14 (100%)
spec/evidence/suites.md:                           0/8 (0%)
```

Every FR-005 acceptance criterion is backed by a symbol that exists in
`tests/shared_assurance.rs` and carries a `/// Trace:` tag:

| Criterion | Test case | Rust symbol |
| --- | --- | --- |
| FR-005-AC-1 | TC-009 | `tc_009_every_shared_pin_is_classified_by_the_packaged_matrix` |
| FR-005-AC-2 | TC-010 | `tc_010_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer`, `tc_010_the_chain_never_executes_a_producer_and_the_probe_can_prove_it` |
| FR-005-AC-3 | TC-011 | `tc_011_the_sealed_records_impact_snapshot_is_the_quire_export` |
| FR-005-AC-4 | TC-012 | `tc_012_retained_evidence_is_read_through_the_shared_mapping_without_moving_a_byte` |
| FR-005-AC-5 | TC-013 | `tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` |
| FR-005-AC-6 | TC-014 | `tc_014_no_local_evidence_framework_remains_and_the_frozen_schemas_bind_nothing` |

No test in the repository is `#[ignore]`d: `cargo test --all-features` reports
26 passed, 0 failed, **0 ignored**. That matters here specifically, because
`quire coverage` reads backing from source text and an `#[ignore]`d test counts
as backing — a repository can disable its entire tagged suite and keep 100%
coverage. The local checker that used to close that gap was deleted with the
rest of the generic machinery, so the count is stated from the test runner
rather than from the coverage census.

## Code with no owning requirement

Every file added or materially changed by this migration traces to FR-005 except
where noted:

| File | Owning requirement | Note |
| --- | --- | --- |
| `scripts/check_shared_pins.py` | FR-005-AC-1 | |
| `scripts/run_feature_matrix.py` | FR-005-AC-2, NFR-001 | also carries the TC-005 feature-matrix rows |
| `scripts/run_kani_gate.py` | FR-005-AC-2, NFR-002 | the harness rows carry TC-001/002/003 read from the harness source |
| `scripts/check_kani_mutations.py` | FR-005-AC-2, NFR-002 | |
| `scripts/measure_footprint.py`, `scripts/check_linked_footprint.sh` | FR-005-AC-2, MP-001, NFR-001 | |
| `scripts/assurance_chain.py` | FR-005-AC-2, AC-3, AC-5 | |
| `scripts/legacy_evidence_view.py` | FR-005-AC-4 | |
| `assurance/change-assurance.json`, `assurance/pins.json` | FR-005 | the declaration and the pins |
| `tests/fixtures/legacy-compat/*` | FR-005-AC-4, AC-5 | every constructed fixture is one named edit to pinned release bytes, re-derived by the gate |
| `spec/evidence/suites.md` | — | a suite registry; it declares rather than verifies, and contributes the eight unbacked census rows |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-301 | medium | `spec/evidence/suites.md` contributes 8 unbacked coverage rows. A SuiteRegistry has no acceptance criteria to back, so this is a property of the census rather than missing verification, and the headline is quoted with its population everywhere it appears | `spec/evidence/suites.md` | wrong-requirement |
| FND-302 | medium | Four of the crate's six modules — `verdict.rs`, `observation.rs`, `identity.rs`, and most of `operators.rs` beyond the proved families — carry no Kani harness. Inherited from PR #7 and unchanged here; the harnesses are bounded controls and MP-001 says so | `verification/kani.rs` | correct-requirement-no-evidence |
| FND-303 | medium | `quire coverage --strict` cannot check the Coverage Status column here: the installed module configures the `Status` header and the authored table uses `Coverage Status`. A row declaring itself unbacked while being backed would not be caught | `spec/test-matrix.md` | wrong-requirement |
| FND-304 | low | The 42 retained envelopes are read but cannot be interpreted: the pinned mapping covers only `quire.pgm01-evidence` v1 and v2. The compatibility answer for this repository is a refusal, reported as one | `assurance/pins.json` | wrong-requirement |
| FND-305 | low | The compatibility matrix this change gates on records `pending_human_acceptance` and ships no `human_acceptance_recorded` predicate; the acceptance exists only on `engineering-assurance` main as `ae50e13`, with no release carrying it | `requirements-assurance.txt` | wrong-requirement |
| FND-306 | low | No ix-flow decision event exists, so the verification receipt reads `incomplete` with reason `decision_missing`. Only the repository owner can create one; synthesising it would forge the single field in the chain that exists to say a person looked | `assurance/change-assurance.json` | correct-requirement-no-evidence |

## Dispositions

| ID | Disposition | Where |
| --- | --- | --- |
| FND-301 | ACCEPTED | The census population is named in SR-002, this document, and the pull-request body. A registry with no criteria cannot be backed and is not treated as a defect. |
| FND-302 | DEFERRED | `agent-ix/quire-contract-runtime#4`. Not introduced by this change; the proof scope is stated in MP-001 and the test matrix. |
| FND-303 | DEFERRED | `agent-ix/quire-contract-ir#21`. The engine half is toolchain-owned; the local compensating checker was a second traceability implementation with a hand-copied matrix and was deleted with the rest of the generic machinery. |
| FND-304 | DEFERRED | `agent-ix/engineering-assurance#21`. 142 such envelopes across six of the eight campaign repositories. |
| FND-305 | DEFERRED | `agent-ix/engineering-assurance#20`. Reported, never gated on, and no branch head substituted for the tag. |
| FND-306 | OPEN | Owner `@kreneskyp`. It is the decision the whole chain exists to wait for. |

## What this analysis does not claim

It does not claim the crate's semantics are fully verified; it claims the matrix
rows that exist are backed by symbols that run. It does not claim the eight
unbacked rows are a defect; it claims they are a registry. And it does not claim
100% of anything without naming what the percentage is over.
