---
id: "SR-1417"
title: "IR-349 spec review (base): AD-004 step 1 Group 1 amendments to FR-275 and interface-001 (PR 102)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@3524ef89238e11143afd57f6c9eabc6433561f05; spec/exact/functional/FR-275-single-exact-kernel.md, spec/core/functional/interface-001-runtime-api.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/matrix/tests.md (git diff origin/main...HEAD), read against spec/assurance/AD-004-runtime-crate-layout.md Group 1"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-197
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
---

# SR-1417: IR-349 base spec review of PR 102

## Summary

Ticket: IR-349 (AD-004 migration step 1, spec only). I checked every statement the PR amends
against AD-004's Group 1 list. I then re-read FR-275 and interface-001 in full at the PR head,
looking for any Group 1 statement left unamended. I also checked the touched TC-197 steps and the
matrix row for truthfulness against today's code.

Examined: FR-275 Description, Outputs, Behavior "One kernel", "The residue" and "The end state",
FR-275-AC-1, AC-16 and AC-17. In interface-001: the Exact kernel surface prose, the contract yaml
(`module`, `kernel.runtime_role`, `runtime_owned`, `core_items`, the `dependencies` lines and the
`residue.rule`), and interface-001-AC-1 to AC-6 and AC-10. Also TC-197 steps 1, 6 and 7, and the
FR-275 row of `spec/exact/matrix/tests.md`.

Context only: AD-004 (end state, Groups 1 to 3, Migration step 1, Owner answers), and the owner
answers the planner recorded on Linear IR-349 (Q-1 to Q-4). These are untrusted ticket text, and I
read them only to check scope. Also CG `origin/main`, read-only, to check present-tense claims.

Scope holds. The Group 2 statements are untouched: FR-275 Dependency spelling, One copy, Guarded
edges, AC-4, AC-7, AC-8, interface-001-AC-11, AC-14 and the `runtime -> quire-exact` line. The
Group 3 statements are untouched: FR-273, AD-002, the residue list, the yaml `residue` block and
AC-9. The relied-on criteria are byte-identical to main: FR-275-AC-13, AC-18 and
interface-001-AC-7.

The PR adds no SHA, pin or line citation, no compatibility or shim allowance, and no new
`check_one_copy` reference. It makes no decision beyond AD-004 and the recorded owner answers. Both
yaml blocks parse. The `scalar` items the PR names match today's source exactly:
`evaluate_boolean_short_circuit` and `ShortCircuitConnective` in `src/exact/numeric.rs`, the
division negotiator types in `division.rs`, and the IEEE ones in `ieee.rs`. Nothing else in the
repo references `outside_exact`, so the rename to `core_items` breaks nothing. No test carries a
tag for any AC whose meaning changed, so this is not a matrix backed by tests of the old meaning.

The gates match baseline. `quire validate` exits 0 with 102 of 102 documents grammar-clean.
`quire coverage --strict` reports 56 unbacked rows and 5 contradicted statuses, and its sorted
output equals `origin/main` once line numbers are stripped.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-275 Behavior "The residue" is a Group 1 bullet, and only its last sentence was amended. Its first sentence still says "The `exact` items `quire-exact` does not export, other than the negotiators, are still in the runtime's source ... a port of QSL code ... Each is a defect ... and is to be deleted". That classifies the lazy Boolean connective as unauthorized residue. The amended End state, AC-16, AC-17 and the yaml `runtime_owned` block all keep it as a runtime-owned `scalar` item. Say "other than the `scalar` items (the negotiators and the lazy Boolean connective)". | FR-275 Behavior "The residue"; FR-275-AC-16, AC-17 |
| FND-002 | medium | The interface-001 Exact kernel surface prose is Group 1, but it still gives the old rationale for the `exact` path: "replacing the runtime's own kernel implementation with the upstream `quire-exact` crate changes where kernel items are defined and leaves the seam itself unchanged". With no re-export, the consumer-facing paths do change. CG moves from `rt::` to `quire_exact::` and `quire_semantic_value::`, which is AD-004's "CG paths that change in the same step". Restate the seam as unchanged in ownership and items, with the paths moving to the owner crates. | interface-001 Exact kernel surface, first paragraph |
| FND-003 | medium | Two sentences of the amended interface-001 prose state the end state in the present tense, and both are false today. First: "Generated oracles and `quire-contract-codegen` reach exact semantic values through the owning crates' own paths ..., which generated source names directly". CG `origin/main` still names `quire_contract_runtime::exact` in `src/oracle/scalar`, `equality`, `function` and `src/kani/generate/scalar.rs`, and emits `use quire_contract_runtime::exact as rt;`. Second: "the runtime's own such items are in its `scalar` module". No `scalar` module exists; the items are in `src/exact/numeric.rs`, `division.rs` and `ieee.rs`. The next sentence does mark the end state as planned, but these two read as facts. Mark them as the end state, planned (IR-349). | interface-001 Exact kernel surface, first paragraph; yaml `runtime_owned.module` |
| FND-004 | medium | The FR-275 matrix row contradicts itself and the ACs. (a) It keeps "AC-17 can be checked today", yet the PR marks FR-275-AC-17 "PLANNED (IR-349)" and appends that AC-17 is "planned until the IR-349 code steps land". (b) It calls interface-001-AC-6 and AC-10 planned, but both hold today. A grep of `src/` outside the `*_tests.rs` files finds no `NodeKey::from_bytes`, `from_hex` or `from_digest` call. `Verdict`, `VerdictKind`, `Observation`, `ClauseOutcome`, `FailureDetail` and `CampaignReport` are defined in `verdict.rs`, `observation.rs` and `accounting.rs`. AC-10's own text carries no PLANNED mark, while AC-6's does. (c) interface-001's own row in `spec/core/matrix/tests.md` still says "pending adoption" for the same ACs. That gives two statuses for them in two matrices. State one status per AC that is true today. | spec/exact/matrix/tests.md FR-275 row; interface-001-AC-6, AC-10; spec/core/matrix/tests.md interface-001 row |
| FND-005 | medium | TC-197 step 1 is narrower than the FR-275-AC-1 it claims to verify. AC-1 covers every public item the runtime defines "in `scalar` or any other module". Step 1 lists only "the public items the runtime defines under the `exact` feature". So a core-module item named like a `quire-exact` or `quire-semantic-value` export passes TC-197 but violates AC-1. Measured today, there is no such collision: no `pub` item name in `src/*.rs` matches any in either crate at QSL `origin/main`. This is a test-procedure gap, not a current defect. List all of the runtime's public items, with `exact` enabled. | TC-197 step 1; FR-275-AC-1 |
| FND-006 | low | The yaml `residue.rule` still reads "every exact item the upstream quire-exact crate does not export, other than the runtime_owned negotiation items". The PR's new `runtime_owned` block adds the connective, so by this rule the connective is both runtime-owned and residue. The `residue` block is Group 3, so the reconciliation crosses a group boundary. Either make a one-word change ("other than the runtime_owned items") and say it is forced by the Group 1 `runtime_owned` change, or record that the Group 3 amendment resolves it. | interface-001 yaml `exact.residue.rule`; `exact.runtime_owned` |
| FND-007 | low | FR-275 Behavior "One kernel" says "The runtime defines none of them (interface-001-AC-2, AC-3 and AC-5 state the same rule per item)". The PR rewrote interface-001-AC-2 into the no-re-export rule, so AC-2 no longer states a per-item definition rule, and the cross-reference is now stale. | FR-275 Behavior "One kernel"; interface-001-AC-2 |
| FND-008 | low | The PR adds "PLANNED (IR-349)" inside the Criteria text of FR-275-AC-1, AC-16 and AC-17 and of interface-001-AC-1 to AC-6. No other AC in `spec/` carries a status in its text. This puts status into the requirement, duplicating the matrix, which owns status. The criterion text must then be edited when the code lands, and the mark is applied unevenly: AC-6 has it although it holds today, and AC-10 lacks it although the matrix row calls it planned. Keep the status in the matrix, and in the Description sentence that already says the end state is planned. | FR-275-AC-1; interface-001-AC-1 to AC-6 |

## Verdict

**Not mergeable as is.** Fix the four mediums first. FND-001 and FND-002 are Group 1 statements
left partly unamended, which step 1 exists to remove. FND-003 and FND-004 are truthfulness issues,
where present-tense or status text does not match today's code. FND-005 is a TC step that does not
test what the rewritten AC now says. All are wording fixes in the same four files. The lows can be
fixed in the same round.

What is right: the amendment set matches AD-004's Group 1 list. Groups 2 and 3 and the relied-on
criteria are untouched. The `scalar` item names are exact. The no-re-export rule is stated as a
direct, inspectable prohibition: no `exact` module and no `pub use` of the three shared crates. The
gates hold at baseline.

Out of scope, for the planner and not a finding against this PR: interface-001
`compatibility.enums: non-exhaustive` will contradict owner answer Q-4 once step 3 drops the
attribute from the kept `scalar` enums. AD-004 names that statement in none of its three groups.

## Dispositions

Disposition pass 1. Reviewed agent-ix/quire-contract-runtime@8a236d727d89aa32d632bb3d033865501b534db0
(fix commit 8a236d7 over 3524ef8; origin/main unchanged). Re-measured against `src/`, `Cargo.toml` and
CG `origin/main`, not taken on the author's word.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8a236d7. "The residue" now excludes "the runtime-owned `scalar` items (the negotiators and their types, `evaluate_boolean_short_circuit` and `ShortCircuitConnective`, which are not residue)". This agrees with the End state, AC-16, AC-17 and the yaml `runtime_owned` block. |
| FND-002 | fixed | 8a236d7. The prose now says replacing the kernel "leaves the seam's items and their ownership as stated here and moves the paths consumers name to the owning crates". |
| FND-003 | fixed | 8a236d7. The prose opens with "Today generated oracles and `quire-contract-codegen` reach exact semantic values through `quire_contract_runtime::exact`", which is true: CG origin/main still names that path in four emitters. The owner paths and the `scalar` module are now stated as the "target end state (FR-275; planned, IR-349)", "which does not exist yet". The yaml `module` and `runtime_owned.module` comments say the same. |
| FND-004 | fixed | 8a236d7. (a) "PLANNED" is gone from AC-17, and the row's "AC-17 ... stays checkable today" no longer contradicts it. (b) AC-6 and AC-10 are now in a core-matrix row "hold today by inspection". Verified: no `NodeKey` constructor call in non-test `src/`; `Verdict` and `VerdictKind` are in `verdict.rs`, `Observation`, `ClauseOutcome` and `FailureDetail` in `observation.rs`, and `CampaignReport` in `accounting.rs`. (c) The FR-275 row no longer gives interface-001 statuses; it points to the core matrix, so each AC has one status. See new FND-009 for one misplaced AC in the split. |
| FND-005 | fixed | 8a236d7. TC-197 step 1 now says "List every public item the runtime defines with `exact` enabled, in every module". |
| FND-006 | fixed | 8a236d7. `residue.rule` now reads "other than the runtime_owned items". This is a one-word consequence of the Group 1 `runtime_owned` change. It changes nothing IR-583 decides: the evaluation items, the `consumed` lists and the deletion tracker are untouched, so it need not wait for IR-583. |
| FND-007 | fixed | 8a236d7. "One kernel" now cites "interface-001-AC-3 and AC-5 state the same rule per item; AC-2 and AC-4 state that no path re-exports them". |
| FND-008 | still-open | "PLANNED (IR-349)" is removed from every AC, but FR-275-AC-16's criterion text still ends "Unmet while the residue exists (matrix)." That is a status remark in the requirement text, which the matrix row already carries ("AC-16 and AC-18 are unmet while the residue exists"). Delete the sentence from the AC. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | The new core-matrix split lists interface-001-AC-2 under "unmet today: the runtime still defines an `exact` module and the kernel and residue items". AC-2 forbids re-exporting, by `pub use`, alias, wrapper or feature, any item of `quire-exact`, `quire-semantic-value` or the evaluation leaf. No file under `src/` or `verification/` names `quire_exact` or `quire_semantic_value`, so nothing is re-exported today and AC-2 holds today by inspection. The runtime's own copies fall under AC-3 and AC-5, not AC-2. Move AC-2 to the "hold today" row. | spec/core/matrix/tests.md interface-001 rows; interface-001-AC-2 |

Disposition pass 1 also checked:
- The rewrite of AC-1 loses nothing. The CG half is owned by AD-004 step 3's paired CG PR, and CG's compile against the RT head (AD-003 T-2, AD-004 L-5) fails if any emitter still names `quire_contract_runtime::exact` once the module is gone.
- AC-4 is now a named-list inspection, defined or re-exported at any path. Like AC-5, it is a concrete instance, not an event trigger.
- The "hold today" claims for AC-8, AC-11 and AC-14 match `src/` and `Cargo.toml`: the negotiators are defined in `division.rs` and `ieee.rs`; `quire-exact` is optional and enabled only by `exact`; no other QSL crate is a dependency.
- The AD-004 step 3 addition, "interface-001's `compatibility.enums: non-exhaustive` statement is amended in this step to match", is accurate. It stays within Q-4 if the amendment is scoped to the kept `scalar` enums, since the core enums keep the attribute.
- The leftover sweep is clean. The only `check_one_copy` reference is the pre-existing Group 2 one, and "compatibility" appears only as a prohibition.
- The gates match baseline: `quire validate` exits 0, and `quire coverage --strict` gives 56/5 with sorted output equal to main once line numbers are stripped.

### Dispositions, round 2

Disposition pass 2 (closing), reviewed at
agent-ix/quire-contract-runtime@d8ca5f2c18a7e04c719038d5add6d4cf6c0f4104 (fix commit d8ca5f2;
origin/main unchanged).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | d8ca5f2. FR-275-AC-16 now ends "... and no file under `src/exact` exists." with no status remark. No AC row in FR-275 or interface-001 carries "PLANNED", "(matrix)" or any other status text. |
| FND-009 | fixed | d8ca5f2. interface-001-AC-2 is now in the core-matrix "hold today" row, whose reason reads "no file under `src/` or `verification/` names `quire_exact` or `quire_semantic_value`, so nothing is re-exported". I re-ran `git grep -E 'quire_exact\|quire_semantic_value' -- src verification` and it exits 1 with no match. No interface-001 AC is listed in both core-matrix rows. |

Closing sweep at d8ca5f2. No status remarks remain inside the AC text this PR touched. Each
interface-001 AC has exactly one status, and the FR-275 row points to the core matrix instead of
giving its own. Every remaining `quire_contract_runtime::exact` mention is either marked as today's
state or stated as a prohibition. The diff adds no SHA, `outside_exact`, shim, fallback or legacy
wording. "Compatibility" appears only as a prohibition, plus the existing `compatibility.enums`
reference in AD-004 step 3. The only `check_one_copy` mention is the unchanged Group 2 text. The
gates match main: `quire validate` exits 0 with 102 of 102 documents grammar-clean, and
`quire coverage --strict` reports 56 unbacked and 5 contradicted, with sorted output equal to
`origin/main` once line numbers are stripped. No new findings.
