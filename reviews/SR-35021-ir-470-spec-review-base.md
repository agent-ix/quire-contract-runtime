---
id: SR-35021
title: "IR-470 PR 163 base spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime; spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/matrix/TC-206-exact-panic-surface.md"
review_set: base
relationships:
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: reviews
---

## Summary

Ticket: IR-470. Reviewed the exact-profile panic and division-admission criteria and TC-206 against RT's current linked-gate scope and the resolved QSL kernel API at PR head 987e6a7b46a7a70e323bba9b00c438f7d2c633f1. No applicable AssuranceProfile was found under this repository's `spec/` tree. No source or CI files changed.

## Verdict

PASS for the two changed spec files. The retained-object population, required calls, fail-closed inspection, reachable injected-panic control, and direct kernel admission controls are explicit and can fail independently. The criteria preserve the default footprint band and exclude claims about unretained or third-party code. The PR description's Dependency paragraph incorrectly says the current lock predates `NonZeroInteger`; the lock resolves `quire-exact` 1c6cbcb6, a descendant of the admission merge 37aa1721, and the locked source exposes the typed API. Correct that PR metadata before merge; the spec itself uses a conditional clause and does not make that false current-state claim.

## Examined scope

- NFR-002 exact-profile scope paragraph (examined): "The default linked-footprint panic check in NFR-001 covers its fixed population only. The `exact` feature has a separate linked probe under TC-206."
- NFR-002-AC-8 (examined): "On the declared MSRV and `thumbv7em-none-eabi`, the `exact`-enabled, `std`-disabled TC-206 probe retains calls into the runtime-owned `scalar` negotiators and Boolean connective and the runtime's remaining exact expression surface."
- NFR-002-AC-9 (examined): "A scratch probe with a reachable runtime-owned panic call produces a recognized panic relocation and makes the exact-profile gate fail; the same gate passes its unmodified probe when no recognized panic relocation is present."
- NFR-002-AC-10 (examined): "The resolved `quire-exact` dependency exposes `NonZeroInteger::try_from(&Integer)` with typed zero-divisor refusal, and its public `exact_div`, `div_rem_truncating` and `div_mod_floor` require an admitted `NonZeroInteger`, not a plain `&Integer`."
- TC-206 (examined): "Exercise an exact-enabled, no-std runtime consumer separately from the default-footprint population. Check the retained runtime-owned objects for recognized Rust panic relocations, and check the QSL-owned integer division admission contract at the resolved `quire-exact` dependency."
- NFR-001-AC-3 and FR-275-AC-10 (context only): existing default-profile footprint and dependency-exclusion boundaries referenced by TC-206.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
