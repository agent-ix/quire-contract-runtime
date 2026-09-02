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

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| FND-601 | high | `TC-013` unioned the chain's states with the compatibility census's case kinds. `unsupported` and `malformed` came only from the census, so deleting it would have silently reduced a twelve-outcome guarantee to ten while the test still passed. | **FIXED** — `unsupported` is now demonstrated by Quoin reporting `unknown-method` against a criterion declaring a verification method its catalog does not have; `malformed` by a producer row carrying the outcome `check_kani_mutations.py` emits when a mutation anchor is no longer present exactly once. `TC-013` reads the chain alone and the chain reaches 12/12. Both probes were shown red: degrading them makes `assurance_chain.py` exit 1. |
| FND-602 | high | All four digest-pinned `consumed_artifacts` were read only by the deleted reader. Deleting them leaves the digest check asserting over an empty population. | **FIXED** — replaced with one live pin on `engineering_assurance/compatibility.py`. Probed red: appending one byte to the installed module takes `make pins` from exit 0 to exit 1 with `consumed artifact digest mismatch`. `TC-009` now also asserts the pinned population is non-empty. |
| FND-603 | medium | `NFR-002-AC-4` described the deleted evidence collector and the builder that validated against the vendored PGM-01 schema. Its only remaining verification was the frozen-schema digest census in `tc_014`, which this change deletes. Leaving it would have been a criterion bound to a test asserting nothing about it. | **FIXED** — the criterion is deleted, not weakened, together with its `TC-007` paragraphs and its `tc_007` trace tag. |
| FND-604 | medium | `make compat-view` ran two things: the census **and** a five-probe mutation gate. Deleting the target removes both. | **ACCEPTED** — all five probes (`collapse-non-success-states`, `repair-unreadable-outcome`, `accept-refused-schema`, `unbind-tamper-digest`, `drop-source-identity`) degraded checks inside the compatibility census itself. They guarded the deleted material and have nothing left to guard. The repository's mutation-probe census falls from 15 to 12; see "Mutation probes" below. No probe that guards surviving material was removed. |
| FND-605 | medium | Five documents beyond the named acceptance criterion argued from retained evidence: `spec/index.md`, `spec/assurance/CAC-001`, `spec/assurance/MP-001`, `spec/assurance/AA-001` and `spec/nonfunctional/NFR-002`. `AA-001`'s `evidence_binding` block gated the top claim on `retained_evidence:`. | **FIXED** — every one removed or restated to what is actually true, never restated more weakly about the deleted records. `AA-001` no longer names retained evidence in its binding and its `proof_obligations` count is 6. |
| FND-606 | low | `requirements-assurance.txt` said the Draft 7 validator it superseded "is frozen under `schemas/`". | **FIXED** — corrected; the validator was deleted. |
| FND-607 | low | `plan/PLAN-002`'s completion rule required every byte under `evidence/` to be unchanged, and Task-001 instructed "FREEZE — not deleted" for the four `schemas/` artifacts. | **ACCEPTED** — the plan records are left as written and a dated `log.md` entry states that they describe what was true when written, and that the constraint they served was released by the owner the following day. Rewriting a completed plan to match a later decision is backdating. |

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
| `assurance_chain.py` adapter/audit probes | 7 | 9 |
| **Total** | **15** | **12** |

The fall is −5 +2, and it is correct rather than a regression: the five that went
degraded checks inside the deleted compatibility census and had nothing left to
guard. The two added are the replacement demonstrations of `unsupported` and
`malformed`, and both were observed red before being relied on.

## What was deliberately not done

The Make execution-control guard was not re-added. Its absence is recorded, not
closed, by owner decision, and `agent-ix/quire-contract-runtime#10` carries the
measured numbers. Nothing in this change alters them.

No compatibility mapping, evidence envelope, manifest, anchor file, retention
store or aggregate verdict was reintroduced under a new name.
