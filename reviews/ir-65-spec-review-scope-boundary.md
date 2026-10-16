---
id: SR-11544
title: "IR-65 scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime@754c88851c53587c47009b6d4815fbb9cc1429d5; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/non-functional/NFR-001-no-std-footprint.md, spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-65. Checked whether each new obligation sits with the right owner, and whether any new clause overlaps another FR or NFR in RT. I also checked overlap with kernel-owned behaviour in quire-exact, including FR-359 meter ownership.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-004-AC-9 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`, `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in that order inside `counts`; byte-for-byte comparison of encoded output observes this order. |
| FR-004-AC-10 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Inspection of the decoder and its parser dependency confirms that parser scratch grows only with the capped input, with no unbounded allocation for rejected nested metadata. |
| NFR-001 Scope (exact) | examined | spec/core/non-functional/NFR-001-no-std-footprint.md | The optional `exact` feature also enables `alloc` and uses allocating collections and values without requiring `std`. Its linked size is outside the default-profile footprint measurement. |
| NFR-002-AC-4 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `publish = false`, disabling registry publication for this package. |
| NFR-002-AC-5 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `license = "AGPL-3.0-or-later"`, and the existing `make deny` gate checks the resolved dependency set against the license policy. |
| FR-006-AC-7 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Inspection of `src/exact/` finds no host `f32` or `f64` operations in the exact surface. |
| FR-006-AC-8 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | The `exact` feature remains available with crate-level `#![no_std]` and `#![forbid(unsafe_code)]`; inspection of non-test code in `src/exact/` finds no intentional panic site. |
| FR-006-AC-9 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Every public item exposed through `quire_contract_runtime::exact` is re-exported by `src/exact/mod.rs` from a private child module; the child modules are not public paths. |
| NFR-001-AC-2 | context_only | spec/core/non-functional/NFR-001-no-std-footprint.md | The library compiles with no `unsafe` code. |
| NFR-002-AC-1 | context_only | spec/core/non-functional/NFR-002-panic-compatibility-license.md | Valid public evaluation and accounting inputs encounter no intentional panic path. |
| FR-006 owner evidence (FR-359) | context_only | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | `FR-359-AC-6` and `FR-359-AC-7` cover counter order and consumption; |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-006-AC-8 restates crate-wide properties that NFR-001-AC-2 (no `unsafe`, compile-time-check) and NFR-002-AC-1 (no intentional panic) already own. The `forbid(unsafe_code)` half is the same crate attribute, so two criteria now own one property. If ownership changes, one owner can be updated and the other left stale. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:73 |
| FND-002 | low | FR-004-AC-10 obliges RT to inspect the internals of a third-party crate (`serde_json`'s parser) that RT does not own. Once the removed "at the exact dependency pin" wording is gone, the subject changes on any `cargo update` without a spec change. An inspection recorded today does not say which parser version it covered. | spec/accounting/functional/FR-004-campaign-accounting.md:98-104 |

## Verdict

PASS with low findings. The new criteria do not overlap quire-exact FR-359 meter ownership. FR-004-AC-9's "counter order" refers to the campaign snapshot's four counters, not `ScalarLimits` field order. FR-006-AC-7/8/9 constrain only RT-local `src/exact/` code and add no meter or charge obligation. NFR-002-AC-4/5 sit in the NFR that already states `publish = false` and the license. The NFR-001 `exact` paragraph matches the existing `make size` exclusion under FR-275-AC-10.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The NFR-001 Scope rewrite narrows the footprint obligation as well as clarifying no-heap. The old scope was 'The default feature set and every symbol linked by generated customer code'. The footprint-size obligation now covers only the fixed-population consumer measured by `make size`. A default-profile generated oracle that links more operators than the fixed population is in scope for no-heap but out of scope for the 4 KiB budget. That is a normative scope decision made in a review fix round, while the owner note leaves only the `exact` half open. Restore 'symbols linked by default-profile generated code' to the footprint scope, or put the narrowing in the owner note as undecided. | spec/core/non-functional/NFR-001-no-std-footprint.md:19-21 |

## Dispositions

Round 1, reviewed head 06d5f6799d826f5a2613f69b703ceb0ced718e16 (`git diff 754c888..06d5f67`). Each outcome was checked against the diff and the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-006-AC-8 no longer restates `forbid(unsafe_code)`. FR-006 Verification hands crate-wide unsafe to NFR-001-AC-2 and public no-panic to NFR-002-AC-1. AC-10 is scoped to non-test `src/exact/` code. |
| FND-002 | fixed | No new criterion inspects `serde_json` internals. AC-10 and AC-11 now bind RT's own decoder symbols. |
| FND-003 | fixed | Round 2 (70a0b8d): The Scope sentence is byte-identical to base ac0efa9: `git diff ac0efa9 HEAD` on NFR-001 shows only the added Note. The Note is labelled an owner decision, says 'remains open', and decides nothing. |
