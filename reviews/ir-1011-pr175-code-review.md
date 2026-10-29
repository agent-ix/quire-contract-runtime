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
