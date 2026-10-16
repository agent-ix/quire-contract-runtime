---
id: SR-11582
title: "IR-61 F14 FR-007 correction: failure-domain analysis"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-runtime@2cde819ac7520f31b9b43dab5b25a158edbd4691 (base 40425522c8b3f962c543969d283e54acbccf00e3); spec/exact/functional/FR-007-exact-scalar-families.md"
review_set: subset
---

## Summary

Ticket: IR-61. This analysis looks for unstated failure modes, preconditions and edge cases in
the rewritten division and rounding Behavior and in FR-007-AC-12 and AC-15 to AC-18.

## Verdict

**PASS with four low findings.**

The stated success vectors hold when executed against the locked kernel. The gaps are refusal
paths and preconditions the criteria leave unstated; none makes a stated value wrong.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-007 Behavior and AC-12/AC-18 state the out-of-domain refusal for `div`/`rem` (`DivisionOutOfDomain`) but not for `mod`. The kernel refuses `mod` as the distinct `ModuloOutOfDomain` (`division.rs:184-187`; QSpec FR-147 `modulo_out_of_domain`). AC-18 chooses a domain "admitting both residues", so it tests only the success path. Failure: an RT binding that reports `mod` overflow as `DivisionOutOfDomain` satisfies FR-007. | spec/exact/functional/FR-007-exact-scalar-families.md:31-38; :117 |
| FND-002 | low | AC-15 gives the target scale (0) and the rounding mode but not the target's coefficient bounds. A `Decimal` target whose bounds exclude 0 or 1 refuses with `DecimalOutOfDomain` (`decimal.rs` `check_membership`) instead of yielding the stated coefficient. The scratch run used `[-100,100]`. | spec/exact/functional/FR-007-exact-scalar-families.md:114 |
| FND-003 | low | AC-17's "canonical results are `6000 meters²` and `2000/3` dimensionless" holds only when `meter` is the canonical root and `kilometer` is 1000 meters with zero offset. That is the `tests/exact_semantics.rs:46-97` fixture, which the criterion does not name. Under a graph rooted at `kilometer` the results are 6/1000 km² and 2000/3. | spec/exact/functional/FR-007-exact-scalar-families.md:116 |
| FND-004 | low | AC-18 checks two of the four operand-sign combinations. Base AC-12 required "every operand sign", and TC-034 step 9 requires all four under each `div`/`rem` law. `-7 mod -3` = 2 and `7 mod 3` = 1 were executed but are not stated. | spec/exact/functional/FR-007-exact-scalar-families.md:117 |

## Coverage

Examined:

- FR-007 Behavior lines 31-38 and 48-55
- FR-007-AC-12, AC-15, AC-16, AC-17 and AC-18

Read as context only:

- quire-exact `division.rs` and `decimal.rs`
- QSpec FR-147
- `tests/exact_semantics.rs`
- TC-034

## Dispositions

Round 1 checks head f7e1e02615e1069743f55801a5d3bd11f5fa2693 against `git diff 2cde819..f7e1e02`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Behavior text and AC-18 now state that an out-of-domain `mod` remainder is refused without a value as `ModuloOutOfDomain`. That matches quire-exact `division.rs:189` and QSpec FR-147. TC-034 step 9 adds a refusal vector: `-7 mod 3` under `Int[0,1]`, whose remainder 2 is outside the domain. |
| FND-002 | fixed | AC-15 now states the target coefficient interval `[-100,100]` at scale 0. That is the interval the vectors were executed against. |
| FND-003 | fixed | AC-17 now states the unit graph: canonical root `meter`, non-affine `kilometer = 1000 meters`, zero offset. That matches the `tests/exact_semantics.rs` fixture and the executed results. |
| FND-004 | fixed | AC-18 and TC-034 step 9 now state all four quadrants: `7 mod 3 = 1`, `-7 mod 3 = 2`, `7 mod -3 = 1` and `-7 mod -3 = 2`. All four agree with the earlier execution against the locked kernel and with owner FR-357-AC-7. |
