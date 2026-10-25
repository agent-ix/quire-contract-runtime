---
id: SR-35034
title: "code-review of IR-499 FR-006-AC-1 consumer evidence (quire-contract-runtime PR 166)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@ce96ab304819f3bf37abe6ce4c31cc1fc69393d0; tests/exact_outcome_consumer.rs; base agent-ix/quire-contract-runtime@4e639dcb7f576a029d06a8ec5b5b94e55f34635c; cross-checked src/scalar/boolean.rs, src/exact/mod.rs, src/exact/expression.rs, Cargo.toml, Cargo.lock (quire-exact 1c6cbcb6)"
---

# Code review of PR 166: FR-006-AC-1 exact outcome consumer test (rust-review lane folded in)

## Summary

Ticket: IR-499. PR agent-ix/quire-contract-runtime#166 (draft), frozen head ce96ab30, two
commits over 4e639dc (91af45c superseded by the fixture correction ce96ab3; only the head is
reviewed). The Rust change is one new integration test file, `tests/exact_outcome_consumer.rs`,
gated on `feature = "exact"`, with one `#[test]` tagged `#[trace("TC-016", "FR-006-AC-1")]`
and four private exhaustive-match helpers over the kernel `Refusal`, `Undefined`, `LimitKind`
and `ChargePoint` enums. No production source, Cargo manifest, lockfile, CI workflow, lint or
deny configuration changed.

## Method

- Read the test against `src/scalar/boolean.rs` (`evaluate_boolean_short_circuit`, which
  round-trips stops through `quire_semantic_value::stop::{outcome_into_stop, outcome_from_stop}`
  and charges `boolean.result-retain` via `quire_exact::retain_boolean`).
- Tautology check, per assertion: a runtime that conflated `Undefined` and `Refused` in the Stop
  round trip, turned a completed `false` into a stop, dropped the `boolean.result-retain`
  charge, or reported a different `Incomplete` record (limit kind, limit, consumed, next charge,
  charge point) fails one of the `assert_eq!`s. The pairwise `assert_ne!` over five outcomes and
  the `completed()` checks fail if any two dispositions compare equal or a stop yields a value.
- Closedness check: the four helpers match external-crate enums with no wildcard arm. Adding a
  variant or `#[non_exhaustive]` to any of them in `quire-exact` breaks compilation of this
  test, so the "closed enum" clause is a real, failing check, not a tautology.
- Fixture: the refusal reason is `Refusal::CheckedInvariant { cause:
  CheckedInvariantCause::ForeignCheckedExpression }`. The diff has no
  `CheckedInvariantCause::CallDepthExceeded` (retired by IR-356/IR-497); the remaining
  occurrences are pre-existing in `src/exact/expression.rs`, outside this diff.
- Trace tag: `#[trace(...)]` from `ix-trace-rs`, the repo's current form; no `/// Trace:` or
  `Tracing:` line is added (the pre-existing `Trace:` in `src/exact/mod.rs` is outside the diff).
- Panic surface: `panic!` appears only inside the never-called right-operand closures of
  short-circuited cases, which is the intended detector; test code only.
- Gates run at head (worktree target dir, RUST_TEST_THREADS=4): `cargo fmt --all --check` pass;
  `cargo clippy --locked --offline --features exact --test exact_outcome_consumer -- -D warnings`
  pass; `cargo test --locked --offline --features exact --test exact_outcome_consumer` 1 passed.
  Aggregate gate, `make deny` and Kani not run (lead's single final gate; brief).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean for the Rust change. The test exercises the real runtime Boolean path, each runtime
assertion can fail under a plausible regression, the closedness helpers are compile-enforced,
the fixture uses the stable `ForeignCheckedExpression` cause, and the tag uses `#[trace]`. The
trace tag's coverage side effect (the `TC-016` id also backs FR-006-AC-3 rows) is recorded once,
in the gap-analysis artifact SR-35035, not here.
