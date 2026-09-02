---
id: TC-013
title: "Demonstrate all twelve outcomes and pair every negative with a positive control"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: verifies
---
# TC-013: Demonstrate all twelve outcomes and pair every negative with a positive control

## Description

Verify that pass, fail, unavailable, unsupported, inconclusive, not-computed, malformed, partial,
stale, suspect, vacuous, and tampered are each demonstrated by a case that produced them and matched,
and that each negative case names a positive control which was observed to be accepted. A refusal
that has never been seen to accept is indistinguishable from a step that never worked.

## Test Procedure

Read the chain report's `states_demonstrated`. Then drive `scripts/check_kani_mutations.py` into its
malformed branch in a scratch copy and read back the outcomes it reports. Its predicate is
`count(anchor) != 1`, which has two sides, so both are driven: every anchor duplicated (count 2) and
every anchor removed (count 0). Either way no anchor occurs exactly once and no prover runs. Union the
observations with the chain's states and require all twelve. Require every declared negative scenario
to be named by some control's `pairs_with`, and require the chain to refuse a control naming a
scenario that does not exist.

## Expected Results

Twelve of twelve demonstrated. Every negative paired. `unavailable` in particular is demonstrated by
the Kani producer's own vocabulary, so an absent model checker is a reported state and not a skip.

Eleven come from the chain. `unsupported` is demonstrated by Quoin naming a declared verification
method its catalog does not have. The twelfth, `malformed`, is demonstrated by the mutation campaign
actually emitting it, because a chain-side probe for it could only rewrite a row's outcome itself and
assert that the adapter's own lookup table carried it — a tautology that would stay green while the
producer that owns the state was hollowed out to report `pass`. Both states previously came from the
deleted retained-evidence compatibility census.

## Toolchain dependency

The producer checks for `cargo-kani` before it reaches its own predicate, so on a machine without the
model checker this test reports `unavailable` and fails. That is fail-closed and deliberate — `make
ci` already requires Kani — but it means plain `cargo test` now requires it too, which it did not
before this probe existed. An absent toolchain is named in the failure message rather than left to
read as a mismatch.

## Limitations

`unsupported` appears in neither the adapter's nor the chain's outcome vocabulary; it is a finding
Quoin reports over the specification, not a state travelling the intake path. `malformed` is a
producer state, and the chain maps it to `failed` because Quoin's attestation vocabulary is passed,
failed, unavailable and not_computed. Both facts predate this test's current form and are stated
rather than claimed away: "twelve demonstrated" means each state was produced and observed by the
component that owns it, not that twelve distinct values reach the verification receipt.
