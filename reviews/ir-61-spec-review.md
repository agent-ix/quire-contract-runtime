---
id: SR-11562
title: "IR-61 residue slice upstream link-spelling spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@170c5c15e1b94fff2e1334120b77919b10f0b174 (base 0fcccfc70f58407e1a25c041a0f6b8639f94911f); spec/exact/functional/FR-007-exact-scalar-families.md; spec/exact/functional/FR-008-composite-collection-and-equality.md; spec/exact/matrix/TC-018..TC-023"
review_set: subset
---

## Summary

Ticket: IR-61. Branch code/ir-61-residue-slice, no PR yet. This review covers the F16 spec edits
only. They convert bare upstream QSpec ids to `ix://agent-ix/quire-specification/<id>` in FR-007,
FR-008 and TC-018 through TC-023. The edits change link spellings, not criteria, so the EARS,
integrity and strength sub-analyses do not apply.

## Verdict

**PASS, clean.** A word-level diff confirms that each change replaces a bare id with its qualified
form and changes nothing else. The one exception is `FR-149/FR-144`, which became
`.../FR-149 / .../FR-144` with spaces around the slash. No criterion id, verification cell or other
wording changed.

Every converted id resolves at QSpec origin/main 3e51cf9 to the intended artifact:

| Id | Artifact |
| --- | --- |
| FR-141 | text and enumerations |
| FR-142 | quantities and units |
| FR-144 | collection algebra |
| FR-149 | equality matrix |
| FR-272 | typed diagnostic cause |
| TC-185 | decimal, D01-D25 |
| TC-186 | text and enum |
| TC-187 | quantity |
| TC-190 | contains Q11 |
| TC-191 | contains P11 |
| TC-192 | integer division |
| TC-193 | IEEE |

`quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md' --summary` gives the
same result at base and head: 180/180 documents grammar-clean.

`quire coverage --strict` reports 112 unbacked rows at both base and head. It exits 1 at both, so
the failure is pre-existing. Its only differences are the criterion text of FR-007-AC-1 to AC-5
and AC-7.

`quire matrix` has 147 rows at both base and head and loses no row.

No FR-275 migration, FR-007-AC-8 or AC-12 text change, Floor/Ceiling vocabulary change, or
Cargo, Makefile or CI change is in the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## New findings (disposition pass 1)

The review pass missed this finding. I found it while checking the FR-008 fix at
129c9d6ee790321368052e8d4f003c64bfb5a9fa.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Three FR-007 criteria end with a bare ix link followed by a period: FR-007-AC-3 (`TC-193.`), AC-4 (`TC-186.`) and AC-5 (`TC-187.`). `quire extract` harvests their edge targets as `TC-193.`, `TC-186.` and `TC-187.`, including the period, so those three edges point at ids that do not exist. `quire validate` still passes. A trailing comma, as in `TC-185,`, is stripped correctly. Wrap each link in backticks or angle brackets. A local probe shows either form yields a clean target. Main had no edges for these ids because they were bare text. | spec/exact/functional/FR-007-exact-scalar-families.md:99-101 |

## Dispositions

Round 2 checks head 09a37a09462af03878ccb45711f0700f248f73b2 against
`git diff 129c9d6..09a37a0`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | FR-007-AC-3, AC-4 and AC-5 now end with `` `ix://agent-ix/quire-specification/TC-193`. `` and the equivalent links for TC-186 and TC-187. I ran `quire extract` on FR-007, FR-008 and TC-018 to TC-023. No edge target ends in punctuation or contains a backtick. Every intended edge is present: TC-185 to TC-187, TC-190 to TC-193, FR-141, FR-142, FR-144, FR-149, FR-272, FR-368-AC-1, FR-368-AC-2 and FR-369. Only the link delimiters changed in the criterion text. |

## Coverage

- Examined the following criteria, each of whose cells changed only by link spelling:
  - FR-007-AC-1 to FR-007-AC-5 and FR-007-AC-7
  - FR-008 Behavior bullets citing FR-141, FR-142, FR-144, FR-149 and FR-272
  - TC-018 to TC-023 Description, Test Procedure and Expected Results
- `quire extract` now harvests `references` edges to FR-141, FR-142, FR-144, FR-149 and FR-272
  from FR-008's body links. Base harvested none. The edges are printed by bare id. The same is
  true of the existing `ix://agent-ix/quire-exact/FR-361-AC-6` link on main, so this is existing
  tool output behaviour, not a defect in this change.
