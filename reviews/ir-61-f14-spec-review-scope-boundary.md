---
id: SR-11584
title: "IR-61 F14 FR-007 correction: scope-boundary analysis"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime@2cde819ac7520f31b9b43dab5b25a158edbd4691 (base 40425522c8b3f962c543969d283e54acbccf00e3); spec/exact/functional/FR-007-exact-scalar-families.md"
review_set: subset
---

## Summary

Ticket: IR-61. This analysis checks that each new obligation is allocated to a component that
can discharge it: the runtime, the quire-exact kernel, or the compiler and QSpec admission.

## Verdict

**FAIL: one medium finding.**

The quantity criteria AC-16 and AC-17 are correctly RT residue (`src/exact/quantity.rs`), and
the diff does not touch IR-349-owned code.

The new admission clause allocates compiler work to the runtime.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-007 Behavior and AC-18 make it an RT obligation, to be confirmed "by execution", that "package admission refuses a non-Euclidean `mod` selection as `IncompatibleDefinition`". RT does not perform admission: `src/exact/definition.rs:4-7` says admitting a selection and checking a definition closure are compiler work, and that the runtime only carries the spellings. No RT code path produces `PackageCause::IncompatibleDefinition`. The upstream refusal is also `refused { code: invalid_package, cause: incompatible-definition }` (QSpec TC-192 DIV-03), not a bare `IncompatibleDefinition`. Failure: no RT test can be written for the clause, so AC-18 cannot be backed in this repository, or a test checks only the vocabulary constant and claims the admission. Fix: restate the clause as owned by compiler admission and QSpec TC-192 DIV-03, and give RT, if anything, an inspection that the vocabulary carries `incompatible-definition`. | spec/exact/functional/FR-007-exact-scalar-families.md:33-34; :117; src/exact/definition.rs:4-7 |

## Coverage

Examined:

- FR-007 Behavior lines 31-38
- FR-007-AC-15, AC-16, AC-17 and AC-18
- FR-007 Kernel ownership section, lines 119-156

Read as context only:

- `src/exact/definition.rs`
- `src/exact/quantity.rs`
- QSpec FR-147 and TC-192

## Dispositions

Round 1 checks head f7e1e02615e1069743f55801a5d3bd11f5fa2693 against `git diff 2cde819..f7e1e02`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-18 no longer carries the admission clause. The Behavior text now names compiler admission and `invalid_package` / `incompatible-definition`, as in QSpec TC-192 DIV-03. The new AC-19 states that RT neither produces nor tests the refusal, and TC-034 delegates it, so the runtime no longer holds an obligation it cannot discharge. How AC-19 is verified is recorded as SR-11585 FND-003. |
