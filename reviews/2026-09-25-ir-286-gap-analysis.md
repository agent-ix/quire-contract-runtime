---
id: SR-011
title: "Gap analysis — IR-286 concrete-path representation fixes against RT's exact-scalar requirements"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@9f0071a12904081c7175730071d1c4bf65eb9c18; PR #79 (Linear IR-286) diff against origin/main 23fbb13, checked against FR-006, FR-007 (AC-6, AC-7, AC-10, AC-11), NFR-002 (Kani gate) and TC-016, TC-023, TC-034"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-023
    type: references
---

# SR-011: Gap analysis — IR-286 representation fixes against RT's exact-scalar requirements

## Summary

Ticket: IR-286. This PR has no plan bundle. IR-286 is a spike ticket, and `plan/` holds only
PLAN-001..003, none of which names this work. Step 1 (plan completion) therefore has nothing to
assert. The analysis is scoped to the requirements that own the types the PR touches: FR-006
(outcomes and accounting), FR-007 (exact scalar families) and NFR-002 (the Kani gate).

- **Matrix step.** `quire coverage --scope . --strict` exits 1 at the reviewed head with
  `coverage could not evaluate its declared input: status-column-matches-nothing`, and `make spec`
  fails structural validation on 3 documents, identically on `origin/main` 23fbb13. The engine's
  reconciliation is therefore unavailable, and this step fell back to a grep index over `tc_NNN`
  names and `Trace:` lines in `tests/`.
- **Criteria backed by tests that exercise the changed code.**
  - FR-007-AC-7 is backed by TC-023, including
    `tc_023_generated_integer_and_ordering_against_an_i128_oracle`. That test runs `+ - * neg` and
    ordering over 12 values, including `i64::MIN` and `i64::MAX`, and so crosses the promotion
    boundary.
  - FR-007-AC-10 and FR-007-AC-11 are backed by TC-034. The one test edit in this PR is TC-034's
    source-inspection assertion (`Decimal` has no structural `PartialEq`). It was updated to the
    boxed shape, and it still fails if `PartialEq` is derived on either `Decimal` or `DecimalFields`,
    because both derive lines are asserted verbatim.
  - FR-006-AC-3 and FR-006-AC-4 (charges precede work, injected denial) are backed by TC-017 and
    TC-031. Both went red under the reviewer's compare and bit-length mutants.
- **Semantic review.** This was run for the Integer triple only, as the dispatch brief asked: does
  the i128 oracle, the fast-path code and FR-007-AC-7's "independent `i128` oracle" agree?
  Eleven mutants were run and two survived. See FND-002 and SR-010.
- **Code→spec (underspecified code).** The representation constraints this PR introduces
  have no owning requirement or architecture decision. See FND-001.

## Verdict

**CONDITIONAL.** There are no high findings, and no task in any plan is left incomplete. Every
behavioural change is owned by FR-006 or FR-007 and is exercised by a traced test. Two gaps remain,
both medium. FR-007-AC-7's oracle is not independent outside `i64`. The Kani-provability layout
rules the PR adds are owned by no spec artifact and guarded by no check.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | Underspecified design constraints. The PR adds four representation rules whose only rationale is CBMC tractability: explicit `u64` enum tags, no inline `Value`/`ValueType` payload larger than `Integer`/`IntegerInterval`, `enter()` returning `Option`, and `Outcome` built without a `Result<_, Stop>` round trip. They are recorded only in code comments. No AD, NFR or FR owns them, and no test or harness fails when one is broken (IR-286 is a spike ticket, not a spec artifact). Failure scenario: a later change breaks one of them and every gate stays green, because the harness that needed them was excluded from this PR. Owning artifact: an AD-001 decision or an NFR-002 criterion. | src/exact/composite.rs:43-49, src/exact/composite.rs:144-150, src/exact/expression.rs:716, Cargo.toml:50-55 |
| FND-002 | medium | FR-007-AC-7 requires "an independent `i128` oracle", and TC-023's oracle is not independent for results outside `i64`. Its expected value is `Integer::from(i128)`, the same `big_from_i128` path under test. The reviewer's sign-dropping mutant of `big_from_i128` survived all exact-feature suites (SR-010 FND-001). Failure scenario: every promoted negative result becomes positive, and TC-023 stays green. | tests/exact_arithmetic.rs:46, tests/exact_arithmetic.rs:557, spec/functional/FR-007-exact-scalar-families.md |
| FND-003 | low | Pre-existing, not introduced by this PR. FR-007-AC-6's evidence (equal Debug renderings against the pinned authority) cannot run: all 9 `conformance/qsl-agreement` test binaries fail to compile with `E0004` on `origin/main` and at this head alike. The PR's "Debug output is unchanged" claim is therefore backed by no gate. The reviewer backed it by a byte-for-byte probe comparison with `origin/main` (76 lines, identical); see SR-010. | conformance/qsl-agreement/tests/support/mod.rs:165, conformance/qsl-agreement/tests/support/mod.rs:238 |

## Coverage

- Plan completion: n/a. No plan bundle targets IR-286.
- Matrix reconciliation: the engine is unavailable (pre-existing `status-column-matches-nothing`),
  so the grep fallback was used. The changed Integer, Rational, Decimal and call-path code is
  reached by TC-016, TC-017, TC-018, TC-019, TC-023, TC-026, TC-031 and TC-034. Each of these went
  red under at least one reviewer mutant.
- Semantic review: run for FR-007-AC-7 ↔ TC-023 ↔ `src/exact/integer.rs` only. Skipped for the
  other requirements; their changes are mechanical (`self.x` → `self.0.x`) and were read in full in
  SR-010.
- NFR-002 Kani gate: 8 of 8 declared harnesses pass at or above their obligation floors. Each was
  run alone under a 16G memory cap.
