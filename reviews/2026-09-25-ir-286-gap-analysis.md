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

## Dispositions

Disposition pass 2026-09-26 at `ab7a9302fcf1b5bd88a6aee597a8ba87a9117c47`. Each outcome was checked
against the code and the fix commits.

| FND | Outcome | sha/reason |
|-----|---------|------------|
| FND-001 | fixed | 5a4aefc, rewritten in ab7a930. AD-002 now owns the `Value`/`ValueType` layout rule (explicit tag, no inline payload wider than `Integer`/`IntegerInterval`), and compile-time assertions in `src/exact/composite.rs` guard it. AD-002 still validates. The two remaining rules are recorded as FND-004 |
| FND-002 | fixed | 3c2c95d: TC-023's expected value no longer runs through `Integer::from(i128)`. The sign-drop mutant of `big_from_i128` now fails `tc_023_generated_integer_and_ordering_against_an_i128_oracle` |
| FND-003 | deferred | Pre-existing on `origin/main` 23fbb13 and out of scope for this PR: the conformance crate fails to compile with E0004 on both trees. The risk this PR added is covered by `tests/exact_debug_parity.rs` (see SR-010 FND-011) |

### New findings in the fix round

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-004 | low | Two of the four CBMC-driven rules from FND-001 are still owned by no spec artifact and guarded by no check: `CheckedPackage::enter` returning `Option`, and building `Outcome` directly instead of going through `Result<_, Stop>`. The AD-002 amendment covers the layout only. Failure scenario: a refactor puts `enter()?` back on `Result<_, Stop>`, and every gate stays green | src/exact/expression.rs:716, src/exact/numeric.rs:242-244, spec/assurance/AD-002-function-application-boundary.md:50-53 |

**Verdict after the fix round: CONDITIONAL.** FND-001 and FND-002 are fixed. FND-003 is deferred as
pre-existing. The one new finding is low.

### Dispositions, round 2

Disposition pass 2026-09-26 at `272af361cd9f69076bf69b9c8db2b2ed91160a5f`.

| FND | Outcome | sha/reason |
|-----|---------|------------|
| FND-004 | fixed | 272af36. AD-002's Risks section now owns both rules. `enter_signature_is_option` pins `enter`'s return type, and `tc_kani_layout_integer_arithmetic_builds_outcome_directly` checks `evaluate_integer_arithmetic`'s source. Mutants: putting `Outcome::from_stop` back fails the source test. Changing `enter` to return `Result<_, Stop>` fails to compile, though the three existing callers (`let Some(guard) = … else`) already fail on their own, so the pin adds a check only against a coordinated rewrite of `enter` and its callers. A `Result` round trip that avoids the literal `from_stop` survives (FND-005) |

### New findings, round 2

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-005 | low | The source check is narrower than the rule AD-002 states ("its body never reintroduces one", meaning a `Result<Integer, Stop>` round trip). The test matches only two literals: no `Outcome::from_stop`, and at least one `Outcome::Completed(result)`. The reviewer rewrote the tail as `let staged: Result<Integer, Stop> = Ok(result); match staged { Ok(result) => Outcome::Completed(result), Err(Stop::…) => … }`. That is a real `Result` round trip, and the test passed | tests/exact_arithmetic.rs:1119-1141, spec/assurance/AD-002-function-application-boundary.md:59-61 |
| FND-006 | low | The source check is wired up inconsistently. AD-002 says the test lives in `tests/exact_outcomes.rs`, but it is in `tests/exact_arithmetic.rs`. The test's name has no `tc_NNN` id, and its only trace is `Trace: AD-002`, which does not bind: `quire coverage --json` lists `tc_kani_layout_integer_arithmetic_builds_outcome_directly` under `untracked_symbols`. Failure scenario: a reader following AD-002 finds no such test in the named file, and coverage never counts the guard | spec/assurance/AD-002-function-application-boundary.md:61, tests/exact_arithmetic.rs:1117-1119 |

**Verdict after round 2: CONDITIONAL, low findings only.** FND-001, FND-002 and FND-004 are fixed.
FND-003 stays deferred as pre-existing. FND-005 and FND-006 are low.
