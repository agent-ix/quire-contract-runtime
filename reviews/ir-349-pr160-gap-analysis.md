---
id: SR-35018
title: "gap-analysis of IR-349 RT scalar move (quire-contract-runtime PR 160)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@3f6ef45d; spec/exact/functional/FR-009-i13-backend-negotiation.md; spec/exact/matrix/tests.md; spec/core/matrix/tests.md; spec/core/functional/interface-001-runtime-api.md; src/scalar; tests/exact_negotiation.rs; tests/exact_meter_state.rs; tests/exact_function_application.rs; tests/exact_outcomes.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-009
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# Gap analysis of the IR-349 RT scalar move (PR 160)

## Summary

Ticket: IR-349. Planless. Plan completion: not assessed. Frozen head 3f6ef45d against origin/main fe673bef.

Examined: FR-009-AC-1..4 (TC-030, `tests/exact_negotiation.rs`, all 31 tests still tagged and passing), FR-009-AC-5 and FR-273-AC-4 (TC-195, the `IeeeDisposition` compile_fail doctest, now at `src/scalar/ieee.rs`, now non-vacuous; see SR-35017), FR-011-AC-3/AC-8 (TC-032, the lazy connective test in `tests/exact_meter_state.rs`), interface-001-AC-8 (the runtime defines the negotiators and their types in its own source: true, in `src/scalar/{division,ieee}.rs`), and FR-275-AC-19 (defined in own source, no QSL crate: true). No AC text changed. FR-009-AC-5 changed only its file path. `quire coverage --strict` is identical at head and main (93 unbacked, 0 contradicted), so no trace tag was lost in the move.

Out of diff, for the lead: AD-004 step 3 also says "drop the four direct dependencies". `num-bigint`, `num-integer`, `num-traits` and `unicode-normalization` are still enabled by the `exact` feature in `Cargo.toml:26`, but no file under `src/` or `verification/` names them. This was already true before this PR. It is remaining IR-349 step-3 work, not a defect of this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The interface-001 matrix row that carries AC-8 was not updated for the move this PR makes. It still reads "Runtime-owned negotiators and core items remain. IR-349 owns their scalar move", but the scalar move is now done. AD-004 step 6 has IR-349 close the matrix dispositions for the source it changes. Record that AC-8's negotiators now live in `scalar`, and keep the row 🚧 for the other ACs. | spec/core/matrix/tests.md:24 |

## Verdict

AC coverage is preserved and no criterion is narrowed. Every examined AC keeps its tagged evidence and passes. One stale matrix disposition is left.

## Dispositions

Round 1, checked at head e3f2a07889fd3dbe0a536ac69fec989e7d66c387. The delta from 3f6ef45d touches only spec files and reviews/; no Rust, test or manifest file changed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | spec/core/matrix/tests.md:24 now reads "The runtime-owned negotiators and lazy Boolean connective now live in `scalar`, satisfying AC-8's current named items. IR-349 still owns cleanup and matrix disposition for its remaining steps". The row stays 🚧 and its AC list is unchanged. |
