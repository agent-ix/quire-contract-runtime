---
id: SR-568103
title: "IR-1011 PR 175 Rust code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime#175@b5a8de084cf41346fed425f156db7ae63ba9df30; tests/exact_accounting_consumer.rs, tests/exact_function_application.rs, tests/exact_outcome_consumer.rs; FR-006-AC-3, TC-016, TC-017; quire-exact accounting.rs and QSpec value-accounting.md as read-only context"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
---

## Summary

Reviewed the three changed Rust tests at the assigned PR head. Rust style and repository conventions were applied. This static pass ran no builds or tests under the one-lane build order. The new meter assertions exercise real kernel behavior, but the limit vocabulary test copies QSpec's ordered field list and its negative case cannot exercise a decoder.

## Verdict

**FAIL** — an upstream vocabulary is duplicated locally and the negative spelling assertion cannot fail when an undeclared spelling is accepted by a consumer.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The local ten-entry `expected` table duplicates QSpec's normative `ScalarLimitsV1` field names and order in RT test code; obtain the authoritative vocabulary as a dependency instead of maintaining a second copy. | tests/exact_outcome_consumer.rs:184 |
| FND-002 | medium | The undeclared LimitKind loop checks only that three literals are absent from a set made from declared `as_str()` values; it calls no reverse conversion and cannot detect acceptance of an undeclared spelling. | tests/exact_outcome_consumer.rs:219 |

## Review scope and limitations

- `tests/exact_accounting_consumer.rs`: high-water, cumulative counters, first-short ordering and denied record; no separate code finding.
- `tests/exact_function_application.rs`: ordinary work-limit denial before a function body side effect; no separate code finding.
- `tests/exact_outcome_consumer.rs`: charge point QSpec vector check and new LimitKind census; findings above.
- `src/` and CI workflows are unchanged. `quire-exact::LimitKind` has `ALL` and `as_str` but no public `from_code` at the inspected dependency interface. The review does not claim the code compiles or the gate passes; neither was run.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The `expected` LimitKind table and its entire test were removed at 3ee6928; no QSpec field-list copy remains in this PR's diff. |
| FND-002 | fixed | The undeclared LimitKind loop was removed with that test; the PR no longer asserts rejection that it cannot observe. |

## Disposition verdict

**PASS for this partial code slice** — both code-review defects were removed. FR-006-AC-3 remains open and is not claimed by a trace tag. Static review only; final gates have not run.

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | The new QSpec LimitKind consumer test calls `LimitKind::from_code`, but RT's locked `quire-exact` revision 6ec67288 has no such method; this PR head cannot compile. | tests/exact_accounting_consumer.rs:29 |

## Dispositions (round 2)

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | still-open | RT Cargo.lock selects quire-exact 6ec67288, whose `LimitKind` implementation has only `ALL`, `as_str`, and `is_cumulative`; only `ChargePoint` has `from_code`. Wait for the kernel method to land, then refresh RT's lock and re-check this test. |

## Delta review at 8a9d044

The new test reads the QSpec `VALUE_ACCOUNTING_LIMIT_KIND_VECTORS` constant from the dependency, compares the declared order exactly with `LimitKind::ALL`, and checks declared and undeclared spellings through the intended reverse method. It copies no vector into RT and does not add a premature FR-006-AC-3 trace tag. Those design checks pass statically. The missing method on the selected kernel revision is a compile blocker. No build or test ran under the one-lane order.

## Dispositions (round 3)

| FND | outcome | reason |
| --- | --- | --- |
| FND-003 | fixed | RT Cargo.lock now selects quire-exact 03a9a7f, whose `LimitKind::from_code` public method resolves exact ScalarLimitsV1 spellings by scanning `ALL`. The two test calls now target an existing method. |

## Delta review at ecb66be

The QSpec vector dependency remains at 938452fb and the test still compares the ordered declared list against `LimitKind::ALL`, then checks every declared spelling and every QSpec undeclared spelling with the kernel reverse method. This addresses the static compile blocker; actual compilation and test execution remain unverified in this review. `rust-toolchain.toml` was deleted, while `Cargo.toml` still declares `rust-version = "1.98.1"`, Makefile's MSRV remains 1.98.1 and CI explicitly selects both stable and 1.98.1; no changed CI gate was found. FR-006-AC-3 remains untagged and TC-017 remains open.
