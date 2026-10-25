---
id: SR-35024
title: "IR-472 PR 164 base spec review"
type: SpecReview
analysis: base
review_set: base
scope: "agent-ix/quire-contract-runtime; spec/exact/functional/FR-008-composite-collection-and-equality.md, spec/exact/matrix/TC-024-composite-construction.md"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-024
    type: references
---

## Summary

Ticket: IR-472. Reviewed RT PR #164 at e213ef1d647bc42b19c43d449b8a2f67d4fc9440 against RT, QSV and QSL origin/main. FR-008's acceptance criteria and TC-024's existing procedure obligations are unchanged. No applicable AssuranceProfile is present under RT spec/.

## Scope

- FR-008 Behavior, TypeEnvironment paragraph: examined. The added text assigns enum declaration admission to the QSL package checker, names MissingName on absent authored type names, and names the separate IllTyped(TypeMismatch) shape guard.
- TC-024 Description: examined. It places the requested direct missing-enum control at QSL and retains PLANNED status.
- TC-024 procedure 3: examined. It retains reference and IEEE checks and adds the shared environment lookup and QSL enum admission checks.
- FR-008-AC-1: context only. It limits the TypeEnvironment's closed declaration set to records, tuples and object types; unchanged.
- FR-008-AC-2: context only. The graph-building criterion remains unchanged.

## Method

RT origin/main removed its duplicate `src/exact/composite.rs` through IR-882; `src/exact/mod.rs` now consumes the QSV and kernel composite/equality surfaces. QSV origin/main's `src/declaration.rs` stores composites and object types, treats ValueType::Enum as a leaf in `type_refusal_admission`, and its `check_type` doc still says every named declaration exists. Its `check_equality` accepts a checker-provided enum-member index. QSL origin/main's `qsl-semantics/src/check/check.rs` ScopeIndex includes enum bindings in named types, `type_form.rs::named_type` refuses an absent name as MissingName, and `Typer::check_declared_type` refuses an enum shape missing from the admitted bindings through `mismatch(location)`. Existing QSL tests exercise generic missing names but no direct absent enum type-form control was found; TC-024 requests that control and does not count it as present. The added text introduces no hash, digest, pin, or copy. No criterion text changed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this spec diff. The ownership statement matches the measured source. The direct QSL enum control and correction to current QSV public `check_type` documentation remains implementation work before IR-472 can be accepted; this review gives no acceptance credit for them.
