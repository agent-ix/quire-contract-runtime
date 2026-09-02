---
id: SR-002
title: Shared assurance migration code review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime#8 at 00ae6f8; the domain work inherited from PR #7; FR-005 and the deletion of the local evidence framework"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-runtime/PLAN-002
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
---

# SR-002: Shared assurance migration code review

## Summary

This change carries two things at once. It inherits the domain work that lived
only on PR #7 — the seventh Kani harness, the `#[cfg(kani)]` accounting
constructor, the independent widened-arithmetic oracle, the symbolic division
harness, the full-width `usize` index harness, and the per-harness discharged-
obligation floors — and it migrates this repository's QA machinery onto the
released Engineering Assurance, Quire, and Quoin contracts. It supersedes PR #7.

The domain half is inherited rather than authored here. `git diff 0bb51fb -- src/
verification/ measurement/` shows one deletion and no additions: nothing in the
crate, the harnesses, or the footprint population changed. The review of that
half is therefore a review of PR #7's six rounds, and the question asked is not
"is it correct" but "did the migration quietly undo anything those rounds fixed".

The migration half replaces 4,387 lines of local evidence machinery with gates
that delegate: component versions to `engineering_assurance.compatibility`,
retained bytes to `map_pgm01_bytes`, and everything dynamic to Quoin's
change-assurance surface. The most substantive findings are that the old path was
never revision-portable — a fact only visible because both paths were run at one
revision before either was deleted — and that Kani publishes no machine-readable
result, so one transcript parser survives, inside the domain tool that owns Kani.

## Verdict

**CONDITIONAL.** Findings below; every one is dispositioned in SR-004 after the
independent adversarial review.

## Gates run at 00ae6f8

| Gate | Result |
| --- | --- |
| `make ci` | see SR-004; run at the exact final head |
| `make spec` (`quire validate --strict`-equivalent + `quire coverage --strict`) | exit 0 |
| `quire coverage` | 40/48 rows backed (83%); rust 22/22/22 bound/tagged/candidates; the 8 unbacked rows are `spec/evidence/suites.md`, a registry with no matrix rows of its own |
| Rust tests | 26 passed, 0 failed, 0 ignored (1 unit, 3 integration, 4 operator, 3 proptest, 3 release-contract, 7 shared-assurance, 5 doc) |
| Feature matrix producer | 9/9 rows pass (4 feature sets × domain targets, 4 doc lanes, 1 footprint package) |
| Kani | 7/7 harnesses pass, each at or above its declared obligation floor; suite-census row pass; `cargo-kani 0.67.0` |
| Kani mutations | 3/3 injected defects rejected by their owning proofs |
| Footprint | 907 bytes `.text`+`.rodata`, floor 500, ceiling 4096, 0 panic relocations, on rustc 1.75.0 for `thumbv7em-none-eabi` |
| MSRV | `rustup run 1.75.0 cargo check --locked --all-targets --all-features` clean |
| Shared pins | 4/4 compatible, 0 artifact digest mismatches, 0 mirror references; acceptance state `pending_human_acceptance`, reported and not gated on |
| Compatibility census | 16/16 cases matched, 42 retained envelopes, 3,412 evidence files read, 0 bytes moved this run, 0 uncommitted differences |
| Compatibility mutation probes | 5/5 detected |
| Assurance chain | 14 scenarios, 6 controls, 7 adapter probes, all matched; 12/12 verification outcomes demonstrated |
| Receipt | `incomplete`, reason `decision_missing` — the correct answer, because no attributed human decision event exists |
| `git diff 0bb51fb -- evidence/` | empty |
| Hosted CI | not dispatched |

## The dual run, recorded as observed

The migration contract requires the old and new paths to be run against the same
candidate revision before the old one is deleted. Both were. The result is not
parity and is not described as parity.

| Path | Revision | Result |
| --- | --- | --- |
| Old, in a pristine clone | `0bb51fb` (branch base) | `make ci` **exit 0**. 7 Kani harnesses, 3 mutation controls, 56 evidence-tool tests, `verify_evidence.py` verified 1 authoritative record / 124 checksums / 104 manifest artifacts, AA-001 anchor bound 29 outcomes as `conclusive`, footprint 907 bytes. |
| Old, at the migration revision | `c6e0e9a` (both paths present) | `verify_evidence.py` **exit 1**; `check_assurance_anchor.py` **exit 1**; `check_coverage_status.py` **exit 1**; `run_evidence_tests.py` **exit 1** (56 run, 20 failures, 3 errors); `check_failure_propagation.py` **exit 1**. |
| New, at the same revision | `c6e0e9a` and `00ae6f8` | every gate above, exit 0. |

The old path's baseline was green, and that is stated because a baseline that had
been red would have had to be stated the other way. Wave 0 of this campaign found
its old verifier had been exiting 1 on `main` for many commits; this repository's
had not.

The old path's failure at the migration revision is not a regression the new path
introduced. It is a structural property of the old design, and running it is what
made the property visible:

- `verify_evidence.py:356` runs `git diff --quiet <recorded-revision> HEAD -- . ':(exclude)evidence'`. **Any** change to any tracked file outside `evidence/` fails it. The only way to make it pass again is a fresh collection, which writes new bytes under `evidence/` — which this migration forbids. The old verifier could therefore only ever be green at the exact revision whose evidence it had sealed.
- `check_coverage_status.py` carries `EXPECTED_FUNCTIONAL_ROWS = 8` and a hand-copied eight-tuple of the matrix. Adding FR-005's six rows makes the census 14 and the gate fails. It was a second copy of the test matrix.
- `check_failure_propagation.py` asserts that the Makefile defines the targets it polices. Removing those targets fails it — a self-attestation asserting its own existence.
- `run_evidence_tests.py`'s suite asserts the shape of the domain scripts this change upgraded to publish structured results.

Only the first of those is unconditional. Even a migration that touched none of
the Python would have failed the source binding, because it changes source. That
is the honest reading and it is why "the old path passed too" is not claimable at
this revision by any implementation.

## Domain behaviour inherited from PR #7, re-checked

PR #7 ran six adversarial rounds and closed findings FND-001..FND-015,
FND-101..FND-114, FND-2xx, FND-3xx, FND-4xx. The remediation at `0bb51fb` was
pushed but never reviewed; round 6 left 1 high / 6 medium / 4 low open at the
previous head. The classes that matter to this change:

| Class closed by PR #7 | Still closed here | How |
| --- | --- | --- |
| Kani outcome must not be a free-text word | yes | `run_kani_gate.py` derives each row from the transcript's own harness census and check counts, and republishes them as fields. Nothing downstream reads text. |
| `skipped-unavailable` must never reach a success verdict | yes, and strengthened | `unavailable` is a first-class row outcome, `make kani` exits non-zero on it, and the chain attests `unavailable`, which makes `attested-results-are-read-from-producer-output` red. |
| A harness census, enforced | yes | `check_kani_harnesses.py` is `make kani-census` and a prerequisite of `make kani`; it catches a deleted or renamed harness without running Kani. |
| Per-harness positive obligation floors | yes | `EXPECTED_KANI_CHECK_FLOORS` is now consumed per row, and a harness that verifies below its floor is `vacuous`, not `pass`. |
| Independent oracles where possible | unchanged | The i8 widening oracle, symbolic division, and full-width index harnesses are inherited byte-for-byte. |
| Mutation controls must reject injected defects | yes | 3/3, and a non-zero exit that never reached a verification failure is now reported as a failure rather than as a control that held. |
| The footprint is a governed measurement | yes | Same floor, ceiling, target, compiler, and zero-panic-relocation rule; now published as a structured document instead of a printed line. |
| Makefile text assertions are not wiring assertions (FND-108) | improved | `tests/release_contract.rs:32` asserted that a literal string appeared in the Makefile, which the whole `ci:` list could be deleted without breaking. It is removed; wiring is proven by `tests/shared_assurance.rs`, which runs the gates. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-201 | medium | Kani publishes no machine-readable result, so one transcript parser survives the migration. It is confined to the domain tool that owns Kani and every downstream consumer reads a field, but it is a parser and it is stated as such rather than hidden | `scripts/run_kani_gate.py` | wrong-requirement |
| FND-202 | medium | The Kani proofs run on the compiler Kani 0.67.0 pins, not on the stable compiler the crate ships on nor the 1.75.0 compiler the footprint is measured on. Inherent to Kani; recorded as an accepted unknown rather than resolved | `assurance/change-assurance.json` | wrong-requirement |
| FND-203 | medium | The feature matrix's test-phase verdict is libtest's process exit status, at suite granularity. Per-test granularity needs libtest's unstable JSON formatter and therefore a different compiler than the one under test. Stated in MP-001 rather than worked around by parsing `test result: ok.` | `scripts/run_feature_matrix.py` | correct-requirement-no-evidence |
| FND-204 | medium | `map_pgm01_bytes` at the pinned release covers `quire.pgm01-evidence` v1 and v2 only. All 42 retained envelopes here are `quire.derivation-evidence/v1` and are refused. A census across the campaign found 142 such envelopes in six of the eight repositories | `assurance/pins.json`, `agent-ix/engineering-assurance#21` | wrong-requirement |
| FND-205 | medium | `quire coverage --strict` returns 0 while reporting `[status-column-matches-nothing]`, so a Coverage Status column that contradicts its own row is not caught. The local checker that used to compensate was a second traceability implementation with a hand-copied matrix and went with the rest of the generic machinery | `spec/test-matrix.md`, `agent-ix/quire-contract-ir#21` | wrong-requirement |
| FND-206 | low | The observational release-rlib byte count is removed. It gated nothing and the deleted collector was its only caller; the governed measurement is unchanged. Recorded in MP-001 rather than left to be found in a diff | `scripts/measure_rlib_size.sh` | correct-requirement-no-evidence |
| FND-207 | low | `spec/evidence/suites.md` contributes 8 unbacked rows to the coverage census. A suite registry has no test cases of its own; the headline 40/48 is quoted with its population rather than as 100% of a smaller one | `spec/evidence/suites.md` | correct-requirement-no-evidence |
| FND-208 | low | The chain consumes what `make assurance-inputs` wrote and cannot itself verify that Make ran the command it printed. Inherent: something must run the producer, and Quoin's contract is that the caller states the result. What is true is that the caller states what the bytes say | `Makefile`, `scripts/assurance_chain.py` | correct-requirement-no-evidence |

## What this review does not claim

It does not claim the crate is correct beyond leaving it unchanged. It does not
claim the seven harnesses prove the crate; they are bounded controls over the
operator, accounting, and public-model surfaces, and four of six modules still
carry none. It confers no qualification, certification, or accreditation, and it
is not a substitute for the human source-release decision, which remains open as
PLAN-001 Task-006.

It is also a self-review, which is the weaker kind. An independent adversarial
review of the same head was commissioned before the pull request was opened, on
the explicit ground that the sibling repository's self-review missed seventeen
false greens that an adversarial one found in a single pass. Its findings and
their dispositions are in SR-004.
