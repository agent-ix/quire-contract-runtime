---
id: SR-11972
title: "IR-818 RT integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@c336b9bf9c17093ffdf8fb18f0badd7a6d51684b; spec/exact/matrix/tests.md, spec/exact/matrix/TC-201-exact-quantity-conversion.md, spec/core/functional/interface-001-runtime-api.md, spec/spec.md, spec/exact/functional/FR-007-exact-scalar-families.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/TC-201
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
---

## Summary

Ticket: IR-818. Checked structural consistency across the five changed files. This covered: TC-201's frontmatter and `verifies` edge; the four new requirement-to-TC rows; the TC table row; the file-to-TC row; the evidence line; the spec.md TC range; the interface-001 contract yaml; and FR-007's AC numbering. TC-201 is the next free id (TC-200 is the monitor TC). FR-007-AC-14 stays skipped and nothing is renumbered. All four AC rows, the TC row, the file row and the evidence line say the same thing: planned, IR-818, `tests/exact_quantity_conversion.rs`, `--features exact`. The changed files contain no hash, pin or digest.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| tests.md FR-007-AC-20..23 rows | examined | `| FR-007 | FR-007-AC-20 | TC-201 | 🚧 planned (Linear IR-818): ...` (and AC-21..23) |
| tests.md TC-201 row | examined | `| TC-201 | Convert a quantity exactly between units of one dimension | Unit | P0 | FR-007-AC-20, FR-007-AC-21, FR-007-AC-22, FR-007-AC-23 | 🚧 planned ...` |
| tests.md file-to-TC row | examined | `| tests/exact_quantity_conversion.rs | TC-201 | planned (IR-818) | new RT-local file ...` |
| TC-201 frontmatter | examined | id: TC-201, type: TC, verifies ix://agent-ix/quire-contract-runtime/FR-007 |
| interface-001 residue key | examined | quantity_residue_added_by_ir_818: [convert_quantity_exact] |
| spec/spec.md TC range | examined | ... planned TC-200 and planned TC-201 provide the verification matrix. |
| FR-275 residue list table | context_only | Containment and unit graphs, enumeration declarations / FR-008, FR-007 / `src/exact/containment.rs`, `unit.rs`, `enumeration.rs` |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The interface-001 residue gains a new key, `quantity_residue_added_by_ir_818`, beside the existing `quantity` list. A contract key named after a ticket is provenance, not structure: the item belongs in the `quantity:` list, with the ticket in the comment. FR-275's own residue list table names no `quantity.rs` item at all (a gap that predates this PR), and the new item is not added there either. FR-275-AC-17 reads "named in the residue list", so the two lists now disagree by one more item. | spec/core/functional/interface-001-runtime-api.md:193-194, spec/exact/functional/FR-275-single-exact-kernel.md:182-187 |
| FND-002 | low | The TC-201 row in the TC table sits between TC-035 and TC-194, which breaks the table's ascending id order. It belongs after TC-199. | spec/exact/matrix/tests.md:95 |

## Verdict

APPROVE WITH NITS (two low). The cross-file trace is consistent, and every planned row names the same ticket, test file and feature. `quire validate` passes on all five files, and `make spec` grammar is 192/192 clean. The `--strict` coverage failure (120 unbacked rows) was already failing before this PR; the PR adds the four planned rows it declares.

## Dispositions

Round 1, head d1ebadd943a15db39c75a3200eb3f23cd0882728, judged against the spec text.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The ticket-named key is removed. interface-001 matches origin/main and no residue item is added, so FR-275's list and interface-001 diverge by nothing new. |
| FND-002 | fixed | The TC-201 row now follows TC-199. |
