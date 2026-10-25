---
id: SR-35033
title: "gap-analysis of IR-349 direct scalar dependency removal (quire-contract-runtime PR 162)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@7da6e84943911d16e537448fea37530395615052; Cargo.toml; Cargo.lock; clippy.toml; spec/assurance/AD-004-runtime-crate-layout.md; spec/exact/functional/FR-275-single-exact-kernel.md; spec/exact/functional/FR-006-exact-outcomes-and-accounting.md; spec/core/matrix/TC-007-release-controls.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# Gap analysis of the IR-349 direct dependency removal (PR 162)

## Summary

Ticket: IR-349. Planless. Plan completion: not assessed. Frozen head 7da6e849 against base
95a27602 (origin/main is now 4e639dc, whose two later commits #163/#164 touch no file in this
diff).

The diff carries out the "drop the four direct dependencies" clause of AD-004 step 3
(AD-004 migration table, step 3 row; and the PR #95 table's "remove four direct
dependencies"). It changes no spec, test or trace tag, so no AC text is narrowed and no test
binding moves.

Examined against head:

- FR-275-AC-3 (`quire-exact` optional, enabled only by `exact`): still true, `Cargo.toml:26`
  is now `exact = ["alloc", "dep:quire-exact", "dep:quire-semantic-value"]`.
- FR-275-AC-4 (dependency spelling, `branch = "main"`, no rev/tag/path/patch): unchanged lines.
- FR-275-AC-10 (footprint graph holds no shared exact crate on thumbv7em): `make size` graph
  check passes; default graph identical to base.
- FR-275 "One copy" and AC-24..28 (`quire check-lock` on the committed lock): `make deny`
  passes on the edited lock. AC-28's "byte-for-byte unchanged" applies to the check-lock
  adoption change, not to later dependency edits, so it does not constrain this diff.
- FR-006-AC-8 no_std half (`build-exact-no-std-msrv` row): `make test-features` passes,
  14 feature sets.
- NFR-001 / TC-007 footprint band: 973 bytes at head and base, 0 panic references.
- AD-004 step 3 dependency clause: delivered by this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-004's amendment list still states, in the present tense, that the four direct dependencies "are named today only by `integer.rs`, `rational.rs`, `ieee.rs` and `text.rs`" and "are removed in the step that deletes those files". After #160 those files no longer exist (`src/exact` holds only `expression.rs` and `mod.rs`), and after this PR the dependencies are gone, removed in a follow-up PR rather than the deleting step. A reader of AD-004 is told work is pending that is done, and of files that do not exist. Restate the bullet as delivered (step 3, PR 162) or drop it. | spec/assurance/AD-004-runtime-crate-layout.md:242-244 |

## Verdict

AC coverage is preserved and no criterion or gate is narrowed. Every examined AC still holds at
head under its executing gate. One stale AD-004 statement about the now-removed dependencies is
left; it is low severity and does not block merge.

## Dispositions

Round 1, checked at head 94dc772cc2dd3418163a33ada1130db93531b23e. The delta from 7da6e849 is one
commit touching `spec/assurance/AD-004-runtime-crate-layout.md` (prose bullet only) and adding the
two `reviews/` files; no Cargo.toml, Cargo.lock, clippy.toml, source, test or verification file
changed, and no acceptance criterion changed.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | spec/assurance/AD-004-runtime-crate-layout.md:242-245 now reads "The scalar source cutover left no runtime use of the direct dependencies `num-bigint`, `num-integer`, `num-traits` and `unicode-normalization`. IR-349 step 3 removed their direct `Cargo.toml` and root `Cargo.lock` edges in follow-up PR #162. The owning crates may still depend on them transitively." This matches the repo at head. |
