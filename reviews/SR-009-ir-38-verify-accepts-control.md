---
id: SR-009
title: "Code review and gap analysis — verify-accepts-an-unedited-receipt now observes acceptance (IR-38)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime#45 (Linear IR-38); scripts/assurance_chain.py, the verify-accepts-an-unedited-receipt control (lines ~748-795), two hunks against origin/main ac90fb4"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/AP-001
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/TC-013
    type: references
  - target: ix://agent-ix/quire-contract-runtime/CAC-001
    type: references
---
# SR-009: Code review and gap analysis — verify-accepts-an-unedited-receipt now observes acceptance (IR-38)

## Summary

This is a two-hunk fix to `scripts/assurance_chain.py`. The `verify-accepts-an-unedited-receipt`
control previously asserted `verified_status != 2`, an exit-code check that cannot currently
distinguish acceptance from refusal: under the open upstream regression quoin#543, refusal exits 1,
and this scenario's own receipt (`incomplete`, `decision_missing`) also exits 1, so the control was
satisfied unconditionally regardless of which happened. The fix replaces the exit-code check with an
observation of the receipt itself — the control now asserts that `verify-receipt`'s stdout parses as
a JSON object carrying `"digest"`, which is structurally only true on the acceptance path (the
refusal path never emits JSON to stdout at all). This combined document covers both operations
`AP-001`'s `review_policy` requires for a change of this shape (`code-review`, `gap-analysis`): the
diff is one file, two hunks, and mirrors an already-reviewed sibling pattern exactly, so one document
records both lenses rather than splitting them as this repository's larger reviews (SR-002/003,
SR-004/005, SR-006/007) do.

**Verdict: APPROVE — no blocking findings.**

## Independent verification performed

- Read the installed `@agent-ix/quoin@0.23.1` `verify-receipt.js` directly (not simulated):
  confirmed the refusal path calls `this.error(...)` with exit 2, printing to stderr only and never
  emitting JSON to stdout; confirmed the acceptance path's only `--json` stdout emission is always a
  dict carrying `"digest"`.
- Reproduced empirically: `echo '{}' | quoin change-assurance verify-receipt --input - --json` exits
  2 with empty stdout and stderr prose — confirming the shape the fix depends on.
- Confirmed `Chain.verify_receipt` (`scripts/assurance_chain.py:444-455`) returns
  `(returncode, (stdout or stderr).strip())`, so a refusal hands the new predicate unparseable prose
  (`json.loads` raises → `None` → `False`) and an acceptance hands it the JSON envelope
  (`isinstance(x, dict) and "digest" in x` → `True`).
- Confirmed quoin#543 changes only the exit code (2→1), never the stream or the emitted shape, so the
  new predicate's discrimination is structurally immune to that regression — closing the gap in the
  original synthetic-only repro methodology that first surfaced this ticket.
- Live reproduction against real `target/assurance/*` inputs: both
  `verify-accepts-an-unedited-receipt` and the sibling control
  `the-same-revision-sealed-honestly-is-read-not-refused` report `ok`, with detail
  `{'exit': 1, 'read_back': True}` — exactly the case the old `!= 2` check could not distinguish from
  a refusal.
- Sibling-mirroring check: read both control blocks side by side (new at ~777-795, sibling at
  ~842-856) — character-for-character equivalent modulo variable names (same
  `json.JSONDecodeError` handling, same `None` fallback, same `isinstance(x, dict) and "digest" in x`
  check, same `{"exit": ..., "read_back": ...}` detail shape). No missed edge case.
- Verified the "no Python lint gate" premise the fix's simplicity relies on: no
  `pyproject.toml`/`setup.cfg`/`.ruff.toml`/`.flake8`/`tox.ini`/`.pre-commit-config.yaml` anywhere in
  the repository; zero hits for `ruff`/`black`/`flake8`/`mypy`/`pylint`/`isort`; the Makefile's
  `lint:` target is clippy only; the CI workflow runs rustfmt/clippy/make targets only.

## Gap analysis — FR-005-AC-5 traceability

`spec/functional/FR-005-shared-assurance-intake.md:66` (FR-005-AC-5) requires "each negative case is
paired with a positive control that was observed to be accepted." The old code observed only "did
not exit 2" — under quoin#543 that observes nothing useful. The new code observes the receipt being
read back and returned, satisfying the literal text.

Enforcement chain verified end to end, not just at the unit level: `scripts/assurance_chain.py`
returns exit 1 unless every declared control matches; `tests/shared_assurance.rs::chain_report()`
asserts `code == 0` for the chain run; `tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls`
is the test traced to FR-005-AC-5 per `spec/test-matrix.md:74`. The strengthened predicate is
load-bearing in the Rust suite, not decorative — a regression in this control fails a real,
traced test.

## Findings

| ID      | Severity | Summary                                                                                                                                                                                                                          | Refs                                          |
| ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| FND-001 | medium   | `AP-001`'s `review_policy` requires a committed SpecReview artifact for a change matching its `impact-vacuous-pass` scenario ("a rejected precondition is represented as successful evidence") — exactly this fix's subject. This artifact is that requirement being satisfied. Escape cause: correct-requirement-no-evidence — the policy was right, nothing had produced the required evidence trail until now. | AP-001, this file                              |
| FND-002 | low      | The neighbouring scenario `re-verify-the-sealed-receipt` (`scripts/assurance_chain.py:771-776`) still asserts `verified_status == status` (`1 == 1`), which a real quoin#543 collapse satisfies for a refusal exactly as for an acceptance — the same defect class this ticket removes, one code block above. Out of IR-38's stated scope (the tamper-vs-acceptance control, not this comparison); flagged as a follow-up ticket candidate, not filed yet. Escape cause: correct-requirement-no-evidence. | scripts/assurance_chain.py:771-776             |
| FND-003 | low      | The predicate accepts any dict carrying a `"digest"` key rather than this receipt's own digest specifically. `verified_read_back["digest"] == receipt["digest"]` would pin it more tightly at zero cost, but the sibling control uses the identical weaker form, and diverging here would break the intentional mirroring this change relies on. Not a defect against the currently pinned quoin version (verified above); no escape cause applies. If ever strengthened, both controls should move together. | scripts/assurance_chain.py:777-795, 842-856    |
| FND-004 | low      | The new comment states the assertion "survives quoin#543" — true for this control specifically, but the paired negative scenario nearby (see FND-002) would still go red under an actual #543 collapse. Fail-closed and harmless, but the comment describes this control, not the whole chain. No escape cause applies (documentation nuance, not a defect). Mirrors the pre-existing sibling comment's wording, so left as written. | scripts/assurance_chain.py                     |
| FND-005 | low      | Environmental, no action on this diff: the assurance chain as a whole is red on this host for unrelated pre-existing reasons (`ix-flow` observed at 0.2.3 against the pinned 0.0.4 — see Gate results), so the live "ok" reproduction above happened inside an overall-red chain run, not a fully green one. Does not affect the correctness of this specific fix. No escape cause applies (environmental, pre-existing). | tests/shared_assurance.rs                      |

No high-severity findings. No vendoring/duplication concerns — the predicate is a deliberate,
ticket-mandated mirror of an adjacent idiom within the same function. No stubs, no suppressed
warnings, no weakened assertions, and no test mocking the thing under test. The change strictly
strengthens an assertion and removes one that was structurally blind.

## Dispositions

| ID      | Disposition                                                                                                    |
| ------- | ----------------------------------------------------------------------------------------------------------------- |
| FND-001 | **RESOLVED BY THIS ARTIFACT** — this SpecReview document is the required artifact.                                |
| FND-002 | **DEFERRED** — real defect, same class, but outside IR-38's scope. Recorded here as a follow-up candidate.         |
| FND-003 | **ACCEPTED** — deliberate parity with the sibling control; not exploitable under the currently pinned quoin.       |
| FND-004 | **ACCEPTED** — accurate about this control; mirrors pre-existing sibling wording.                                  |
| FND-005 | **ACCEPTED** — environmental, pre-existing, unrelated to this diff; see Gate results.                              |

## Assurance Context

**Assurance program.** `spec/assurance/AP-001-runtime-release.md` ("Quire contract runtime v0.1
decision profile"). Its `review_policy` (`mode: require`, `operations: [code-review, gap-analysis]`)
governs this change because its `impact-vacuous-pass` scenario — "a rejected precondition is
represented as successful evidence" — is exactly the defect class this fix removes: an unfalsifiable
control that could not distinguish a refused receipt from an accepted one.

**Evaluated revision.** Working-tree diff in the worktree
`/Users/peter/dev/quire-contract-runtime/worktrees/ir-38-verify-accepts-control`, branch
`peter/ir-38-runtime45-verify-accepts-an-unedited-receipt-cannot-observe`, base `origin/main` at
`ac90fb4`. One file changed, `scripts/assurance_chain.py`, two hunks. `/rust-review` was not run — the
diff is Python only, zero Rust. `/spec-review` is not applicable — no spec file is touched by this
change.

**Trust inputs.** engineering-assurance 0.2.0, quire-cli 0.31.0 (engine 0.46.0), quoin 0.23.1 —
unchanged by this change. `ix-flow` is observed at 0.2.3 on this host against the pinned 0.0.4; see
Gate results below. This is a pre-existing environmental condition, not introduced by this diff.

**Unavailable context.** The assurance chain as a whole did not run fully green on this review
machine: `tc_009`, both `tc_010` chain-execution tests, `tc_011` and `tc_013` in
`tests/shared_assurance.rs` fail for the `ix-flow` version-pin drift described above, a pre-existing
condition unrelated to this diff (see Gate results). The live reproduction of the fixed control's
correct behavior (FND-005's "ok" observation) was therefore performed inside that overall-red run,
not inside a fully green one. This does not affect the correctness of the fix itself, which was also
verified by direct source reading and an isolated CLI probe, independent of the Rust harness.

**Failure posture.** Unchanged. The chain still reports a producer's own verdict and still exits
non-zero when a declared control does not match.

## Gate results

Run fresh on the evaluated revision (working tree, `scripts/assurance_chain.py` modified,
`origin/main` at `ac90fb4`):

| Gate                                                       | Result                                                                                           |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------- |
| `make fmt-check`                                              | exit 0                                                                                               |
| `make lint` (clippy, workspace + footprint target, `-D warnings`) | exit 0                                                                                            |
| `cargo test --all-features --no-fail-fast -- --test-threads=1` | 168 passed, 5 failed, 0 ignored (excluding the 12 doc-tests, all passing). Failures confined to `tests/shared_assurance.rs`: `tc_009_every_shared_pin_is_classified_by_the_packaged_matrix`, `tc_010_the_chain_never_executes_a_producer_and_the_probe_can_prove_it`, `tc_010_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer`, `tc_011_the_sealed_records_impact_snapshot_is_the_quire_export`, `tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` — all five failing on the same pre-existing cause, `ix-flow` observed 0.2.3 vs. pinned 0.0.4 (`"versions_compatible": false`), matching this repository's established baseline exactly. Nothing regressed. |
| `tests/exact_function_application.rs` under default parallelism | Not run with default threading in this session's final pass; run explicitly with `--test-threads=1` per this repository's documented pre-existing allocator-reuse flake, where it passes 29/29. |

`make spec` (Quire validate + `quire coverage --strict`) was run against the tree including this
artifact; see the commit for the result at the point this file was finalized.

## What was deliberately not done

The sibling gap recorded as FND-002 (`re-verify-the-sealed-receipt`'s `verified_status == status`
comparison) was not touched. It is a real instance of the same defect class, but it is outside
IR-38's stated scope (the `refuse-an-edited-receipt` / `verify-accepts-an-unedited-receipt` pairing),
and fixing it here would broaden this change beyond what was reviewed and approved. It is recorded
as a follow-up candidate, not filed as a new ticket by this document.

The digest-pinning tightening recorded as FND-003 was not applied, to preserve the intentional
mirroring between the two sibling controls that this fix itself relies on for its own correctness
argument.
