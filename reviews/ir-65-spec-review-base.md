---
id: SR-11540
title: "IR-65 base spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@754c88851c53587c47009b6d4815fbb9cc1429d5; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/non-functional/NFR-001-no-std-footprint.md, spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-65. Branch spec/ir-65-undercoverage-slice, head 754c88851c53587c47009b6d4815fbb9cc1429d5, base ac0efa9857a1cf56b73d1eee9540e06ac7301df6; no PR yet. Base checklist over `git diff origin/main...754c888`: ID format and uniqueness (including git history of each new id), hash/digest/pin antipattern, gate-existence claims, factual claims against `Cargo.toml`, `Makefile`, `deny.toml`, `src/lib.rs`, `src/snapshot_json.rs`, `src/exact/` and `tests/snapshot.rs`, and matrix status for the seven new criteria. `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md' --summary` exits 0 (174/174 grammar-clean). `quire coverage --scope . --strict` reports 94 unbacked rows at base and 101 at head, so the seven new criteria are all unbacked, as intended.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-004-AC-9 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`, `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in that order inside `counts`; byte-for-byte comparison of encoded output observes this order. |
| FR-004-AC-10 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Inspection of the decoder and its parser dependency confirms that parser scratch grows only with the capped input, with no unbounded allocation for rejected nested metadata. |
| FR-004 Verification | examined | spec/accounting/functional/FR-004-campaign-accounting.md | The existing `tc_015_actual_mixed_report_has_independent_exact_json` test compares encoded bytes for the member and counter order in FR-004-AC-9; its trace tag has not yet been updated for that new row. |
| NFR-001 Scope (exact) | examined | spec/core/non-functional/NFR-001-no-std-footprint.md | The optional `exact` feature also enables `alloc` and uses allocating collections and values without requiring `std`. Its linked size is outside the default-profile footprint measurement. |
| NFR-002-AC-4 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `publish = false`, disabling registry publication for this package. |
| NFR-002-AC-5 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `license = "AGPL-3.0-or-later"`, and the existing `make deny` gate checks the resolved dependency set against the license policy. |
| FR-006-AC-7 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Inspection of `src/exact/` finds no host `f32` or `f64` operations in the exact surface. |
| FR-006-AC-8 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | The `exact` feature remains available with crate-level `#![no_std]` and `#![forbid(unsafe_code)]`; inspection of non-test code in `src/exact/` finds no intentional panic site. |
| FR-006-AC-9 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Every public item exposed through `quire_contract_runtime::exact` is re-exported by `src/exact/mod.rs` from a private child module; the child modules are not public paths. |
| TM-003 | context_only | spec/accounting/matrix/tests.md | FR-004 rows list AC-1, AC-2, AC-4..AC-8 only. |
| TM-002 | context_only | spec/core/matrix/tests.md | NFR-002 rows list AC-1 and AC-3 only. |
| TM-001 | context_only | spec/exact/matrix/tests.md | FR-006 rows list AC-1..AC-4 and AC-6 only. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | NFR-002-AC-4 reuses a retired id. Commit 98e7b8a used NFR-002-AC-4 for "The evidence builder verifies its vendored PGM-01 schema pin…", and #12 (60749bc) removed it. The new row gives the same id to `publish = false`, so old review data and comments that cite NFR-002-AC-4 now resolve to a different obligation. | spec/core/non-functional/NFR-002-panic-compatibility-license.md:46 |
| FND-002 | medium | Accounting matrix TM-003 has no row for FR-004-AC-9 or FR-004-AC-10. The repo marks unbacked criteria with a 🚧 planned row, but these two new criteria appear in no matrix status, so a reader of the matrix cannot see that they are unbacked. | spec/accounting/matrix/tests.md:13-15 |
| FND-003 | medium | Core matrix TM-002 has no row for NFR-002-AC-4 or NFR-002-AC-5, so their unbacked Inspection status is recorded nowhere except the strict coverage count. | spec/core/matrix/tests.md:32-36 |
| FND-004 | medium | Exact matrix TM-001 has no row for FR-006-AC-7, FR-006-AC-8 or FR-006-AC-9, although it records every other FR-006 criterion with a 🚧 status, including Inspection-only AC-6. | spec/exact/matrix/tests.md:13-17 |
| FND-005 | low | The rise in strict-coverage unbacked rows from 94 to 101, measured here, is stated in neither the commit message (title only) nor the spec diff. Without it, the coverage regression in the PR goes undisclosed. | commit 754c888 |
| FND-006 | low | The FR-004 Verification prose carries transient status ("its trace tag has not yet been updated"). This becomes false when the tag lands and does not belong in the requirement. The status belongs in TM-003. | spec/accounting/functional/FR-004-campaign-accounting.md:104-106 |

## Verdict

CHANGES REQUESTED, driven by FND-001: a retired criterion id is reused. Clean on the other base checks. No requirement or criterion id is removed or renumbered, and FR-004-AC-9/10, NFR-002-AC-5 and FR-006-AC-7/8/9 have never appeared in history. The diff adds no hash, digest, pin or provenance record and removes the old "exact dependency pin" wording. Every gate it names exists: `make deny` runs `cargo deny --workspace check licenses bans sources` against the `deny.toml` license allow-list. The factual claims hold at head:
- `Cargo.toml` has `publish = false` and `license = "AGPL-3.0-or-later"`.
- `exact = ["alloc", …]`.
- `src/lib.rs` has `#![no_std]` and `#![forbid(unsafe_code)]`.
- `src/exact/mod.rs` declares only private `mod` children with `pub use` re-exports.
- No `f32`/`f64` appears in `src/exact/`.
- The only `panic!`/`unwrap`/`assert` sites in `src/exact/` are under `#[cfg(test)]` in `equality.rs`.
- `tc_015_actual_mixed_report_has_independent_exact_json` compares literal encoded bytes.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | Branch-local ids were re-meant at or below the review-head high-water the author cites (FR-004 10, NFR-002 5, FR-006 9). FR-004-AC-10 changed from a parser-scratch inspection to an unknown-key refusal test. FR-006-AC-8 changed from no_std plus unsafe plus panic inspection to a build-row check. NFR-002-AC-5 dropped its `make deny` half. None of these ids reached main, so no published id is reused. However, SR-11540..11545 and their IR-65 comments cite the old meanings. Anyone mining that review data will attach findings to criteria that now say something else. | spec/accounting/functional/FR-004-campaign-accounting.md:98 |
| FND-008 | low | Coverage rows and Test Case Summary disagree. TM-003's FR-004-AC-9 row cites TC-015, but TC-015's Test Case Summary 'Traces To' still lists only AC-4..AC-7. Likewise, TM-001's FR-006-AC-8 row cites TC-199, but TC-199's summary row (spec/exact/matrix/tests.md:90) traces only FR-275 criteria. A reader working from the TC side will not find these criteria. | spec/accounting/matrix/tests.md:16, 25 |

## Dispositions

Round 1, reviewed head 06d5f6799d826f5a2613f69b703ceb0ced718e16 (`git diff 754c888..06d5f67`). Each outcome was checked against the diff and the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | NFR-002-AC-4 no longer appears anywhere in the head tree (`git grep` 0 hits); the publication obligation is now NFR-002-AC-6, an id never used on main (`git log -S` 0 commits; main high-water NFR-002-AC-4). |
| FND-002 | fixed | TM-003 now has 🚧 planned rows for FR-004-AC-9, AC-10 and AC-11. |
| FND-003 | fixed | TM-002 gains a Non-functional Requirement Coverage table with planned rows for NFR-002-AC-5, AC-6 and AC-7. |
| FND-004 | fixed | TM-001 now has planned rows for FR-006-AC-7 through AC-10. |
| FND-005 | fixed | Commit 06d5f67 states the counts; re-measured with `quire coverage --scope . --strict`: 94 at ac0efa9, 101 at 754c888, 110 at 06d5f67. The 16-row rise is the ten listed criterion rows plus six matrix rows (TM-003:17-18, TM-001:18-21). |
| FND-006 | fixed | The transient 'trace tag has not yet been updated' sentence is gone from FR-004 Verification; the status now lives in the TM-003 AC-9 row. |
| FND-007 | accepted-no-change | Round 2 (70a0b8d): Verified that none of FR-004-AC-10, FR-006-AC-8 or NFR-002-AC-5 appears on origin/main under any meaning (`git log -S` returns 0 commits for each). The re-meaning happened only inside this unmerged branch. Every IR-65 reviewer record carries its `reviewed=` SHA, so the meaning a record cites can be resolved. Renumbering now would only add churn. Acceptable. |
| FND-008 | fixed | Round 2 (70a0b8d): TC-015's Traces To now lists FR-004-AC-9, and its Test Procedure step 1 names the byte-for-byte member-order check. `tc_015_actual_mixed_report_has_independent_exact_json` asserts the literal encoded bytes, so it exercises AC-9. TC-199's Traces To now lists FR-006-AC-8. Its step 1 runs the `build-exact-no-std-msrv` row (`--features exact`, no `std`, thumbv7em, manifest MSRV), which is exactly AC-8. The coverage rows and summaries agree. |
