---
id: "SR-1418"
title: "IR-349 spec review (EARS conformance): rewritten FR-275 and interface-001 statements (PR 102)"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-runtime@3524ef89238e11143afd57f6c9eabc6433561f05; statements changed by git diff origin/main...HEAD in spec/exact/functional/FR-275-single-exact-kernel.md and spec/core/functional/interface-001-runtime-api.md"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1418: IR-349 EARS review of PR 102

## Summary

Ticket: IR-349. The engine check passes: `quire validate --summary` reports 102 of 102 documents
grammar-clean with 0 grammar findings, the same as `origin/main`. FR-275's one `shall` statement,
in the Description, is unchanged by this PR. The amended Description, Outputs and Behavior
sentences are declarative end-state prose, in the style the FR already used.

Semantic pass over the seven rewritten interface-001 criteria: AC-2, AC-3, AC-5, AC-6 and AC-10 are
clean ubiquitous or state-driven `shall` statements. FR-275-AC-1, AC-16 and AC-17 keep the FR's
existing declarative AC style, which this lens does not re-flag. Two findings, both on
interface-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | interface-001-AC-1 is non-singular. It carries two `shall` clauses with two different subjects: "The runtime shall define no `quire_contract_runtime::exact` module, and generated oracle source and `quire-contract-codegen` shall name the owning crates' own paths". The second subject is another repository and its output, so the criterion cannot map to one runtime obligation or one inspection in this repo. Split it. Keep the runtime half here and leave the consumer half to AD-004 step 3's paired CG PR. | interface-001-AC-1 |
| FND-002 | low | interface-001-AC-4 uses an event trigger for what is a standing prohibition: "When an item's owner changes from the runtime to `quire-exact`, the runtime shall keep no path for that item." The event is a development-history change, not a system event, and in the end state there is no `exact` path to keep. As a ubiquitous rule the same obligation is already interface-001-AC-2 (no re-export). Restate it as ubiquitous, or fold it into AC-2. | interface-001-AC-4 |

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@8a236d727d89aa32d632bb3d033865501b534db0 (fix commit 8a236d7). The engine still reports 0 grammar findings.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8a236d7. interface-001-AC-1 is now the single runtime obligation "The runtime shall define no `quire_contract_runtime::exact` module." The consumer half is left to AD-004 step 3's paired CG PR. |
| FND-002 | fixed | 8a236d7. interface-001-AC-4 is now ubiquitous, with no event trigger: "The runtime shall expose no item named in the `kernel.consumed` lists of the `exact` contract block at any `quire_contract_runtime` path, whether defined or re-exported there." |
