---
id: "SR-1400"
title: "IR-345 spec review (integrity): AD-004 contradiction list after the owner answers (PR 101)"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-runtime@e3c517ccc56eab1d72c6e797456537508ebfeec1; spec/assurance/AD-004-runtime-crate-layout.md (end state, Migration step 1); read against merged FR-275 and interface-001"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-1400: IR-345 integrity review of PR 101

## Summary

Ticket: IR-345. I checked the new step 1 text against merged FR-275. Step 1 names five FR-275
statements that mention the QSL repository and its bans: Dependency spelling, Guarded edges,
AC-4, AC-7 and AC-8. Each of them does name the QSL repository or its ban list, so these five
references are accurate (FR-275 lines 93-116 and 149-153). The contradiction list that SR-1396
fixed is unchanged, so that fix has not regressed. The PR adds new amendments, and those are not
reconciled with the list.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Step 1 says it amends "every statement listed under the end state". It then adds, in a parenthesis, the Q-6 statements (FR-275 Dependency spelling, Guarded edges, AC-4, AC-7, AC-8) and the evaluation-item ones (FR-273, AD-002). None of these was added to the end state's "Merged statements this AD contradicts" list. The Q-6 set is also incomplete. FR-275 "One copy" says `deny.toml` "admits that one git source (`allow-git`)", which is the QSL repository. The FR-275 Open question "Upkeep of the ban list" is moot once the bans go. Both are contradicted by the end state and neither is named. Add the Q-6 and evaluation-item statements, plus "One copy" and the ban-list Open question, to the contradiction list, so that step 1's "every statement listed" is complete. | AD-004 end state "Merged statements this AD contradicts"; Migration step 1; FR-275 One copy, Open questions |
| FND-002 | medium | The end state contradicts step 1. The end state says "relies on and must not weaken: ... FR-275's residue list ..., AC-18 (... step 1 leaves it as written) ... and interface-001-AC-7". Step 1 then says "The amendments that name the evaluation items (FR-273, AD-002, interface-001-AC-7, the residue list) need the surface IR-583 provides". Under option (a), interface-001-AC-7 ("The runtime shall not define `Frame`, `Body`, `CheckedPackage`, `Evaluation` or `plan_call`") is already correct and needs no change. The AD does not say what change the residue list needs, for example repointing its deletion tracker from QSL-358 to IR-583. Either drop AC-7 and the residue list from step 1, or state the wording change and that it weakens neither. | AD-004 end state "relies on" bullet; Migration step 1 |

## Verdict

**Fix both mediums before merge.** They are wording fixes in the AD only.

## Dispositions

Disposition pass 1. I reviewed agent-ix/quire-contract-runtime at
7b25ebfddd762e668ca4f2d6fbcb976b58a225bf (fix commit 7b25ebf) and re-read the contradiction lists
against merged FR-275 and interface-001.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7b25ebf. The contradiction list now has three groups, and each one is tied to the step that amends it. Group 1 (no-shim and kernel statements) is amended in step 1. Group 2 is amended in step 2 with IR-582. It covers FR-275 Dependency spelling, "One copy", Guarded edges, AC-4, AC-7 and AC-8, the ban-list Open question, interface-001-AC-11, AC-14 and the `runtime -> quire-exact` line, plus `check_deny_bans.sh`, TC-198 and `CLAUDE.md`. Group 3 is amended when IR-583 states its surface. It covers FR-273, AD-002, the residue list, the yaml `residue` block and interface-001-AC-9. Step 1, step 2 and step 5 each name exactly the group they amend. |
| FND-002 | fixed | 7b25ebf. The "relies on" bullet now keeps only the owner ruling the residue list quotes, AC-18, AC-13, AD-003 F and interface-001-AC-7. It says option (a) already satisfies AC-7. The residue list's tracker change moves to Group 3. That change repoints the tracker and does not weaken AC-18, which still requires an empty list. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Group 1 is introduced as "hold today; amended in step 1", but it still lists "the Open question on the interim copy (left open: see Interim above)". Step 1 says "The interim-copy Open question stays open", and the Interim paragraph no longer decides anything. So that open question is no longer contradicted, and it is not amended in step 1. Drop it from Group 1, or move it to a note that it stays open. This is a wording fix only. | AD-004 end state Group 1; Migration step 1 |

### Dispositions, round 2

Disposition pass 2, reviewed at agent-ix/quire-contract-runtime@3e96b68bb65abe9fa4f1e9f9bf679b2734194bda (fix commit 3e96b68).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 3e96b68. Group 1 no longer lists the interim-copy Open question. It now ends at AC-16 and AC-17 and goes straight on to the interface-001 prose. Step 1 still says "The interim-copy Open question stays open (Interim)", and the Interim paragraph quotes FR-275's open question, so the three places agree. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Step 1 says "Amend Group 1 only", but it also lists FR-275 "step 2". That statement is not in Group 1, and it says QSL-358 places the residue "(function application, ...)" in `quire-semantic-value`. Under the owner's Q-1 answer, function application goes to the IR-583 leaf instead, so that clause belongs to Group 3 and needs IR-583's surface. Either add FR-275 step 2 to the groups, with its exact-path and kernel parts in Group 1 and its function-application clause in Group 3, or drop it from step 1's list. This is a wording fix only. It changes no gating and no gate result. | AD-004 Migration step 1; end state Groups 1 and 3; FR-275 "Step 2, the residue" |

### Dispositions, round 3

Disposition pass 3, reviewed at agent-ix/quire-contract-runtime@a714b5d0049c5137649bc38fe78809f7c6f8dc0d (fix commit a714b5d).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | a714b5d. Step 1 no longer lists FR-275 "step 2". Group 3 now names the function-application clause of FR-275 step 2: QSL-358 places it in `quire-semantic-value`, but under Q-1 it goes to the IR-583 leaf. Group 3 also says the rest of that step is already delivered and not contradicted. That holds on QSL main: `quire-semantic-value/src` has `checking`, `declaration`, `object_closure`, `containment`, `unit`, `enumeration`, `definition` and `semantic_node`. I re-checked the placement of every statement steps 1, 2 and 5 name. Step 1's FR-275 statements (Description, Outputs, both Behavior bullets, AC-1, AC-16, AC-17) and its interface-001 items (prose, the yaml `module`, `runtime_role`, `runtime_owned` and `outside_exact` blocks, the `exact` dependency line, AC-1 to AC-6 and AC-10) are all in Group 1. Step 2's items are all in Group 2. Step 5 amends Group 3. AC-13, AC-18 and interface-001-AC-7 are listed as relied on. No statement sits in two groups. |
