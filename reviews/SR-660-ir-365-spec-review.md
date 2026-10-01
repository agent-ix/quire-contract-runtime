---
id: "SR-660"
title: "IR-365 spec review: Status column and interface-001 frontmatter (PR 97)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@186c1678d7245055c1a4949fb3ec932ad467ee9f; spec/accounting/matrix/tests.md, spec/core/matrix/tests.md, spec/exact/matrix/tests.md, spec/proptest_adapter/matrix/tests.md, spec/core/functional/interface-001-runtime-api.md; base origin/main 5d83267"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# SR-660: IR-365 spec review of PR 97

## Summary

Ticket: IR-365. PR agent-ix/quire-contract-runtime#97 changes only spec files, 5 of them. It
renames the `Coverage Status` matrix header to `Status` in the four subsystem matrices (core has
two tables, and both are renamed). It changes interface-001's frontmatter id from `interface-001`
to `interface_001`, and it adds the `## Features` table that the interface archetype requires. This
review covers integrity, schema conformance and id preservation.

## Method

- Ran `make spec` with quire 0.33.0 (engine 0.47.1) at base 5d83267 and at head 186c167. Base:
  `quire validate` fails 5 documents (four matrices with the `Coverage Status` header, and
  interface-001 missing `id` and `features`). `make spec` stops there, so the strict coverage step
  never runs. Head: validate passes (0 failed). Strict coverage reports 92/148 rows backed, 56
  unbacked rows and 5 contradicted statuses, and `make spec` exits 2.
- Ran `quire coverage --strict` directly at base. It gives the same 92/148, `status-column-matches-nothing`
  on all four matrices, and 2 contradicted statuses (TC-198 and TC-199) that the traces-to table
  already exposed. The PR newly exposes 3 contradictions: FR-275 at exact/matrix/tests.md:42 and
  :43, and FR-003 at proptest_adapter/matrix/tests.md:14.
- Diffed base and head. The changes are 4 header lines, 1 frontmatter line and 18 added lines. No
  row, AC, TC or id is removed, and strict coverage counts 148 rows at both base and head.
- Read the interface archetype id rule and the `status` vocabulary in the spec-artifacts-process
  manifest (v0.26.0). Base ids are `interface_\d+`. Status values are complete ✅, pending 🚧,
  failed ❌ and retired ⛔. Compared the change with quire-contract-codegen's interface-001
  (origin/main), which uses `id: interface_001`, an `[interface-001]` heading and the same
  `## Features` form.
- Checked that the 11 Features rows match, one for one and in order, the 11 `operations` in the
  Contract yaml.
- Grepped RT for both id spellings. `interface_001` appears only in the frontmatter, and every
  other citation uses `interface-001`.
- Ran probes in a scratch copy:
  - A dangling relationship target (`interface-999`) still passes `quire validate`, so
    relationship targets are not resolved.
  - Renaming the AC header `Criterion` to `Criteria` removes all 13 `obligation-row-states-nothing`
    notes.
  - Adding `spec/evidence/inspections.md` and `spec/evidence/suites.md` registries backs nothing.
    It adds 3 rows that are themselves unbacked, and TC-198 stays contradicted.
- Ran `git merge-tree` against open PRs #95 and #78. Both already conflict with main in the same
  files, and #97 adds no new conflicting file.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The ticket goal is not met. IR-365 says "the M5 gate requires make spec green", and `make spec` still exits 2 at head: strict coverage reports 56 unbacked rows and 5 contradicted statuses. The PR says so plainly and claims green nowhere. It is a strict improvement: validation is fixed and strict classification now runs. But IR-365 must not be closed as "make spec green" on merge. The remaining failures need a follow-up ticket. A probe showed that authoring the module's Inspections/SuiteRegistry registries backs no row in quire 0.33.0. Inspection-verified ACs therefore cannot be backed by spec edits alone: they need a quire engine or module change, or the evidence must be turned into tagged tests. | Makefile:102-105, spec/exact/matrix/tests.md:42-43, spec/proptest_adapter/matrix/tests.md:14 |
| FND-002 | medium | Five ✅ rows are contradicted: FR-275 at exact :42 and :43, TC-198 :69, TC-199 :70, and FR-003-AC-2 at proptest_adapter :14. Two of them (TC-198 and TC-199) were already reported on main. The evidence is real, and the rows themselves say it is a gate script or a doctest, so these are not status lies in substance. But ✅ means "backed" to the engine. The honest minimal fix is not to flip the rows to 🚧: that would misreport delivered work and would still fail strict coverage. Instead: (a) FR-003-AC-2: `// Trace: FR-003-AC-2` at src/lib.rs:22 sits on a crate attribute and binds to no symbol. Move it to an evidence symbol, such as a tagged compile-fail test. (b) FR-275 and TC-198/199 gate rows: quire has no channel for this evidence today, so raise it with quire upstream and keep the rows as they are. Out of scope for this header-only PR. Defer to the follow-up ticket. | spec/proptest_adapter/matrix/tests.md:14, spec/exact/matrix/tests.md:42, spec/exact/matrix/tests.md:43, spec/exact/matrix/tests.md:69, spec/exact/matrix/tests.md:70, src/lib.rs:22 |
| FND-003 | low | interface-001's AC table header is `Criterion`, but the module's interface obligation declares `statement_column: Criteria`, and every other RT AC table uses `Criteria`. As a result, none of interface-001's 13 ACs mints an obligation record, and strict coverage prints 13 `obligation-row-states-nothing` notes. These notes exist on main too. The PR body blames them on the new Features table, which is wrong. A probe confirmed that renaming the header to `Criteria` clears all 13 notes and changes no other count. This is a one-word header fix of the same kind as the Status rename and in scope for this PR. Correct the PR body. | spec/core/functional/interface-001-runtime-api.md:223 |
| FND-004 | low | Five FR frontmatter relationships target `ix://agent-ix/quire-contract-runtime/interface-001`, but the document's frontmatter id is now `interface_001`. `quire validate` does not resolve relationship targets (a probe with `interface-999` passes), and codegen has the same pattern, so nothing breaks today. The reference only dangles if target resolution is ever enforced. Keep the targets consistent with codegen in this PR, and note the spelling split in the follow-up ticket. | spec/core/functional/FR-001-verdict-observation.md:8, spec/core/functional/FR-002-safe-operators.md:8, spec/proptest_adapter/functional/FR-003-proptest-adapter.md:8, spec/accounting/functional/FR-004-campaign-accounting.md:8, spec/exact/functional/FR-275-single-exact-kernel.md:8 |

## Verdict

**Mergeable once FND-003 is fixed. No high finding.** FND-001 and FND-002 are deferred to a
follow-up ticket, and IR-365 must not be closed as "make spec green" on merge.

What was checked and found sound:

- The id change is correct. The module's interface base id is `interface_\d+`, codegen's
  interface-001 uses the same form and validates, and the heading and the `interface-001-AC-N` ids
  are unchanged. Coverage still mints all 13 `interface-001-AC-N` rows.
- The Features table has 11 rows, one for each Contract operation in declaration order. Its form
  matches codegen's `| Feature | Kind |` / `operation`, and it validates.
- The header rename is complete. No `Coverage Status` is left in any TestMatrix. Every status
  cell already uses the ✅/🚧 vocabulary, and the column's meaning is unchanged.
- No row, AC, TC or id is removed, and strict coverage counts 148 rows at both base and head.
- The PR body and the commit are honest that `make spec` is still red. The PR changed no status
  and invented no tag.

Routing note, not a finding: draft PR #95 rewrites spec/exact/matrix/tests.md and still carries
the `Coverage Status` header. When it rebases, it must take `Status`, or it reintroduces the
validation failure. #95 and #78 already conflict with main in the same files regardless of this PR.
