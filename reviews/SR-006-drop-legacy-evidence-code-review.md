---
id: SR-006
title: Code review — drop the retained legacy evidence and its compatibility machinery
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime#11; the deletion of evidence/, schemas/, the compatibility reader, its fixtures, its obligation and its acceptance criterion; the two replacement outcome demonstrations; exact-head gates"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/PLAN-002
    type: references
---

# SR-006: Code review — drop the retained legacy evidence and its compatibility machinery

## Summary

This change deletes 3,432 tracked files and 106,865 lines. It is irreversible, so
the only question worth asking of it is **whether anything still needs what it
removed.** Two things did, and neither was visible from the deletion list.

The first is the twelve-outcome census. `FR-005-AC-5` requires pass, fail,
unavailable, unsupported, inconclusive, not-computed, malformed, partial, stale,
suspect, vacuous and tampered to stay distinguishable, and `TC-013` asserted them
over the **union** of the assurance chain's `states_demonstrated` and the
compatibility census's case kinds. Measured on the pre-deletion tree, per outcome
rather than in aggregate:

| Source | Outcomes supplied |
|---|---|
| assurance chain alone | pass, fail, unavailable, inconclusive, not-computed, partial, stale, suspect, vacuous, tampered — **10 of 12** |
| compatibility census only | **`unsupported`, `malformed`** |

Because the assertion took a union, deleting the census would have left `TC-013`
**passing at ten outcomes**. Not a red test — a quietly smaller one. Both
demonstrations were re-established on surfaces that read no retained byte, and
`TC-013` now reads the chain alone.

The second is `assurance/pins.json`. All four of its digest-pinned consumed
artifacts — `verification_semantics.py`, `pgm01-compatibility-view-v1.schema.json`
and the two PGM-01 fixtures — were read **only** by the deleted reader. Removing
them would have left `check_shared_pins.py::artifact_digest_mismatches` iterating
an empty list and reporting no mismatch because there was nothing to compare. The
pin is now `engineering_assurance/compatibility.py`, the module the pin checker
imports for every version verdict, and `TC-009` gained a probe requiring the
pinned population to be non-empty and every falsified digest to be reported.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-601 | high | `TC-013` unioned the chain's states with the compatibility census's case kinds. `unsupported` and `malformed` came only from the census, so deleting it would have silently reduced a twelve-outcome guarantee to ten while the test still passed | `tests/shared_assurance.rs`, `scripts/assurance_chain.py` | correct-requirement-no-evidence |
| FND-602 | high | All four digest-pinned `consumed_artifacts` were read only by the deleted reader. Deleting them leaves `artifact_digest_mismatches` iterating an empty list and reporting no mismatch because there is nothing to compare | `assurance/pins.json`, `scripts/check_shared_pins.py` | correct-requirement-no-evidence |
| FND-603 | medium | `NFR-002-AC-4` described the deleted evidence collector and the builder that validated against the vendored PGM-01 schema. Its only remaining verification was the frozen-schema digest census this change deletes | `spec/nonfunctional/NFR-002-panic-compatibility-license.md`, `tests/release_contract.rs` | wrong-requirement |
| FND-604 | medium | `make compat-view` ran two things: the compatibility census and a five-probe mutation gate. Deleting the target removes both | `Makefile` | correct-requirement-no-evidence |
| FND-605 | medium | Five documents beyond the named acceptance criterion argued from retained evidence, including `AA-001`'s `evidence_binding` block, which gated the top claim on `retained_evidence:` | `spec/index.md`, `spec/assurance/CAC-001-runtime-contract.md`, `spec/assurance/MP-001-runtime-measurements.md`, `spec/assurance/AA-001-runtime-argument.md`, `spec/nonfunctional/NFR-002-panic-compatibility-license.md` | wrong-requirement |
| FND-606 | low | `requirements-assurance.txt` said the Draft 7 validator it superseded is frozen under `schemas/`; the validator was deleted | `requirements-assurance.txt` | wrong-requirement |
| FND-607 | low | `plan/PLAN-002`'s completion rule required every byte under `evidence/` to be unchanged, and Task-001 instructed FREEZE — not deleted for the four `schemas/` artifacts | `plan/PLAN-002-shared-assurance-migration/` | correct-requirement-no-evidence |
| FND-608 | high | **Independent adversarial review.** The first `malformed` replacement probe was a tautology. It rewrote rows in `runtime.kani-proof/v1` — a protocol whose producer cannot emit `malformed` — and its predicate reduced to `KANI_OUTCOMES["malformed"] != "pass"`, a dict lookup 980 lines away. The reviewer hollowed `check_kani_mutations.py:111` from `return "malformed"` to `return "pass"`, so the campaign reported a proof held when it no longer described the source, and the census still showed 12/12 with `make ci` green | `scripts/assurance_chain.py`, `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-609 | medium | **Independent adversarial review.** `FR-005-AC-6`'s second clause still asserted that "the frozen evidence schemas are referenced by nothing". There are no frozen evidence schemas; the clause had an empty population and could not fail | `spec/functional/FR-005-shared-assurance-intake.md` | wrong-requirement |
| FND-610 | medium | **Independent adversarial review.** The explanatory comment added above `tc_007` put the literals `NFR-002-AC-4` and `PGM-01` in the symbol's attached annotation block, so Quire read them as trace bindings and the static export gained two `unmatched_tags` naming a criterion this change deleted. `quire coverage --strict` exits 0 without printing them, so `make ci` was blind to it | `tests/release_contract.rs` | wrong-requirement |
| FND-611 | low | **Independent adversarial review.** Three stale claims survived in live files: `run_feature_matrix.py`'s naming rationale appealed to "the old retained records", `assurance/README.md` still said "digests" plural, and the `assurance-record` comment described writing into `spec/evidence/` | `scripts/run_feature_matrix.py`, `assurance/README.md`, `Makefile` | wrong-requirement |
| FND-612 | low | **Independent adversarial review.** `tc_014`'s `inspected > 30` floor was inherited, not re-derived: the census fell from 55 non-markdown files to 37, cutting headroom from 25 to 7, and the deleted-name list omitted three of the seven `schemas/` files and the mapping symbols | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-613 | low | **Independent adversarial review.** "Twelve of twelve across the intake path" is a thinner claim than it sounds: `unsupported` is in neither `KANI_OUTCOMES` nor `ROW_RESULTS` and is a Quoin finding over the specification, and `malformed` reaches the receipt as `failed` because Quoin's attestation vocabulary has four results | `spec/test/TC-013-verification-outcomes.md`, `assurance/README.md` | correct-requirement-no-evidence |
| FND-614 | medium | **Independent adversarial review, second round.** `collect_sources` matched by extension only, and `Makefile` has no extension — so the deleted-name census never scanned the one file a reintroduced Make target could live in. The reviewer appended the deleted `compat-view` target verbatim to a scratch Makefile and `tc_014` still passed. `FR-005-AC-6` ("no code, configuration, or workflow file") and `TC-014`'s own description (which names `Makefile`) both claimed otherwise. `.yaml` was missing too, and GitHub accepts `.github/workflows/*.yaml` | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-615 | low | **Independent adversarial review, second round.** The malformed probe drove only one side of the producer's `count(anchor) != 1` predicate. Weakening it to `> 1` left `tc_013` green while a *missing* anchor stopped reporting malformed — `text.replace` becomes a no-op, the prover runs on unmutated source, and its success reads as "Kani accepted the injected defect" | `tests/shared_assurance.rs` | correct-requirement-no-evidence |

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-601 | **FIXED** — `unsupported` is now demonstrated by Quoin reporting `unknown-method` against a criterion declaring a verification method its catalog does not have; `malformed` by a producer row carrying the outcome `check_kani_mutations.py` emits when a mutation anchor is no longer present exactly once. `TC-013` reads the chain alone and the chain reaches 12/12. Both probes were shown red: degrading them makes `assurance_chain.py` exit 1. |
| FND-602 | **FIXED** — replaced with one live pin on `engineering_assurance/compatibility.py`. Probed red: appending one byte to the installed module takes `make pins` from exit 0 to exit 1 with `consumed artifact digest mismatch`. `TC-009` now also asserts the pinned population is non-empty. |
| FND-603 | **FIXED** — the criterion is deleted, not weakened, together with its `TC-007` paragraphs and its `tc_007` trace tag. |
| FND-604 | **ACCEPTED** — all five probes (`collapse-non-success-states`, `repair-unreadable-outcome`, `accept-refused-schema`, `unbind-tamper-digest`, `drop-source-identity`) degraded checks inside the compatibility census itself. They guarded the deleted material and have nothing left to guard. The repository's mutation-probe census falls from 15 to 12; see "Mutation probes" below. No probe that guards surviving material was removed. |
| FND-605 | **FIXED** — every one removed or restated to what is actually true, never restated more weakly about the deleted records. `AA-001` no longer names retained evidence in its binding and its `proof_obligations` count is 6. |
| FND-606 | **FIXED** — corrected; the validator was deleted. |
| FND-607 | **ACCEPTED** — the plan records are left as written and a dated `log.md` entry states that they describe what was true when written, and that the constraint they served was released by the owner the following day. Rewriting a completed plan to match a later decision is backdating. |
| FND-608 | **FIXED** — the probe is deleted, not repaired. `malformed` is now demonstrated where it is produced: `tc_013` drives `check_kani_mutations.py` into its malformed branch in a scratch copy, with every mutation anchor doubled so no anchor occurs exactly once and no prover runs, and asserts the producer answers `malformed`. Re-running the reviewer's exact defect now fails `tc_013`: `got ["pass"]`, `right: ["malformed"]`. The chain contributes eleven states and this observation the twelfth. |
| FND-609 | **FIXED** — the clause now names the population `tc_014` actually walks: the deleted retained-evidence tree, its reader, its fixtures, and the schema family it named by digest. |
| FND-610 | **FIXED** — the comment is removed from the annotation block; the rationale lives in this review and in `TC-007`. `unmatched_tags` is empty again. |
| FND-611 | **FIXED** — all three corrected. |
| FND-612 | **FIXED** — floor re-derived and raised to `>= 35` against a measured 37, with the derivation recorded beside it; the deleted-name list gained `pgm01-merged-commit.txt`, `pgm01-validator-blob.txt`, `pgm01-compatibility-view-v1.schema.json`, `map_pgm01_bytes`, `verification_semantics` and `SUITE-007`. |
| FND-613 | **ACCEPTED** — stated rather than claimed away, in a new Limitations section in `TC-013` and in `assurance/README.md`. Both facts predate this change: `ROW_RESULTS` mapped `malformed` to `failed` on `main`, and no `unsupported` ever travelled this chain. "Twelve demonstrated" means each state was produced and observed by the component that owns it, not that twelve distinct values reach the receipt. |
| FND-614 | **FIXED** — `collect_sources` now also collects the extensionless `Makefile` and `.yaml`. Probed red with the reviewer's own defect: appending the deleted `compat-view` target to the Makefile now fails `tc_014` with `Makefile still references the deleted legacy_evidence_view`. The census floor is re-derived to 38 and the claims in `FR-005-AC-6` and `TC-014` are now true. |
| FND-615 | **FIXED** — both sides of the predicate are driven: every anchor duplicated (count 2) and every anchor removed (count 0), each required to report `malformed`. Probed red: weakening `!= 1` to `> 1` now fails `tc_013` with the reviewer's exact observation, `the campaign reported ["fail"], not ["malformed"]`. |

## The independent adversarial review

An independent reviewer was run against this change with one instruction: find
false greens. It ran a baseline first, probed in a scratch copy, and returned six
findings — **one high** — none of which this review had found. It also verified
the coverage arithmetic independently against an extracted `b3c0552`, confirmed no
`make ci` prerequisite became unreachable, confirmed the `TC-009` pin change is a
strengthening, and found no surviving unsatisfiable release gate in `spec/`.

The high finding is the uncomfortable one, and it is the same class of mistake
this change was written to catch. FND-601 correctly identified that `malformed`
had lost its only demonstration; the replacement written for it **was itself a
false green**. It asserted a lookup in a dict literal, not a state, and it stayed
green while the one producer that can emit `malformed` was hollowed out to report
`pass`. Finding the gap and then filling it with something unfalsifiable is worse
than leaving the gap visible, because the census reads 12/12 either way.

The fix does not repair the probe. It deletes it and moves the demonstration to
the producer that owns the state, where the reviewer's exact injected defect now
turns the test red.

The second round is the reason a re-review on the exact remediation head is worth
running rather than assumed: six fixes in one commit is precisely when a new false
green gets introduced, and one of the two findings it returned (FND-614) was made
*sharper* by the remediation — the fix added `compat-view` to a name census that
could not see Makefiles, so the newly added name was unenforceable exactly where a
reintroduction would live.

## Assurance Context

**Claim boundary.** This change claims that the retained records, their reader,
their fixtures, the schemas they named by digest, and every gate and specification
row that existed to serve them were deleted; that nothing was rewritten, backdated
or re-sealed to look as though it still verifies; that the twelve-outcome
demonstration and the consumed-artifact digest check both survive on surfaces that
do not read a retained byte; and that `make ci` passes at the exact head. It claims
nothing about the deleted records' contents and confers no qualification.

**Authoritative policy.** `agent-ix/engineering-assurance#7`, section
"Preservation constraint released for the pre-stable phase" — a decision made by
the repository owner on 2026-09-02 and transcribed there by an agent. The epic's
completion criterion and its mandatory control were amended before this work.

**Trust inputs.** engineering-assurance 0.2.0 (git tag), quire-cli 0.31.0 (engine
0.46.0), quoin 0.23.1, ix-flow 0.0.4 — unchanged by this change. One consumed
artifact is pinned by digest: `compatibility.py` at
`62829251d7697d279364eeb2395a1e86a81ef116703c4b89ec754f701475f654`.

**Failure posture.** Unchanged. An absent `cargo-kani` still produces `unavailable`
rows and a non-zero gate. The chain still reports a producer's own verdict and
still exits 2, not 1, when it cannot read an input.

**Execution boundary.** Unchanged. `make assurance-inputs` remains the only target
that runs a producer, and it now runs one fewer.

**Retained-output identity.** This repository retains nothing. Quoin retains the
producer bytes under `target/`, bound by digest into each sealed attestation.

## Mutation probes

| Gate | Before | After |
|---|---|---|
| `legacy_evidence_view.py --mutation-probes` | 5 | 0 (deleted) |
| `check_kani_mutations.py` semantic defects | 3 | 3 |
| `assurance_chain.py` adapter/audit probes | 7 | 8 |
| **Total, in the gates that publish a count** | **15** | **11** |
| plus: producer-degradation probe inside `tc_013` | 0 | 1 |

The fall is correct rather than a regression. The five that went degraded checks
*inside* the deleted compatibility census and have nothing left to guard. One
adapter probe was added — `audit-reports-an-unsupported-method` — and one
producer-degradation probe was added inside `tc_013`, which doubles every mutation
anchor in a scratch copy and requires `check_kani_mutations.py` to answer
`malformed`.

Every added probe was observed red before being relied on:

| Probe | Degradation applied | Observed |
|---|---|---|
| `audit-reports-an-unsupported-method` | skip the criterion edit, so no unknown method exists | `MISMATCH`, chain exit 1 |
| `tc_013` malformed observation | `check_kani_mutations.py:111` `return "malformed"` → `return "pass"` | `got ["pass"]`, `right: ["malformed"]`, test FAILED |
| `tc_013` malformed observation, absent side | `count(old) != 1` → `count(old) > 1` | `the campaign reported ["fail"], not ["malformed"]`, test FAILED |
| `tc_014` deleted-name census | append the deleted `compat-view` target to the Makefile | `Makefile still references the deleted legacy_evidence_view`, test FAILED |
| `compatibility.py` digest pin | append one byte to the installed module | `consumed artifact digest mismatch`, `make pins` exit 1 |

The first version of the `malformed` probe was **not** observed red in the right
place — it was degraded at the adapter rather than at the producer — and that is
how it survived as a tautology until the independent review. The lesson is
recorded rather than smoothed over: a probe must be degraded at the component that
owns the state, not at the one that transcribes it.

## What was deliberately not done

The Make execution-control guard was not re-added. Its absence is recorded, not
closed, by owner decision, and `agent-ix/quire-contract-runtime#10` carries the
measured numbers. Nothing in this change alters them.

No compatibility mapping, evidence envelope, manifest, anchor file, retention
store or aggregate verdict was reintroduced under a new name.
