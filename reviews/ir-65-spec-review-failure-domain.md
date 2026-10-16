---
id: SR-11542
title: "IR-65 failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-runtime@754c88851c53587c47009b6d4815fbb9cc1429d5; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/non-functional/NFR-001-no-std-footprint.md, spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-65. Looked for unstated failure modes, criteria that pass in the broken state they exist to catch, and edge cases on the new criteria. I checked each against `src/snapshot_json.rs`, `src/exact/mod.rs`, and the FR-275 end state in `interface-001`.

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-004-AC-9 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`, `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in that order inside `counts`; byte-for-byte comparison of encoded output observes this order. |
| FR-004-AC-10 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Inspection of the decoder and its parser dependency confirms that parser scratch grows only with the capped input, with no unbounded allocation for rejected nested metadata. |
| NFR-002-AC-4 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `publish = false`, disabling registry publication for this package. |
| NFR-002-AC-5 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `license = "AGPL-3.0-or-later"`, and the existing `make deny` gate checks the resolved dependency set against the license policy. |
| FR-006-AC-7 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Inspection of `src/exact/` finds no host `f32` or `f64` operations in the exact surface. |
| FR-006-AC-8 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | The `exact` feature remains available with crate-level `#![no_std]` and `#![forbid(unsafe_code)]`; inspection of non-test code in `src/exact/` finds no intentional panic site. |
| FR-006-AC-9 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Every public item exposed through `quire_contract_runtime::exact` is re-exported by `src/exact/mod.rs` from a private child module; the child modules are not public paths. |
| interface-001 FR-275 end state | context_only | spec/core/functional/interface-001-runtime-api.md | The target end state (FR-275; planned, IR-349) is that they reach them through the owning crates' own paths …; the runtime then defines no `quire_contract_runtime::exact` module and re-exports nothing |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-004-AC-10 is effectively unfalsifiable. Every decoder allocation is a finite function of a 64 KiB-capped input, so "grows only with the capped input" and "no unbounded allocation" hold for any parser, including a `serde_json::Value`-tree decoder with multiplicative overhead. Two inspectors would read the bound differently: scratch ≤ input length, or merely finite. "Rejected nested metadata" is also never parsed today, because `FieldVisitor` refuses the unknown key before its value is read, so the clause has no subject to inspect. | spec/accounting/functional/FR-004-campaign-accounting.md:98 |
| FND-002 | medium | FR-006-AC-9 conflicts with the planned FR-275 end state, under which RT "defines no `quire_contract_runtime::exact` module and re-exports nothing". Once IR-349 deletes `src/exact/`, AC-9 and AC-7/AC-8 become vacuous: inspecting an absent directory finds no floats, no panics and no public children. The new criteria bind permanent obligations to residue that `src/exact/mod.rs` itself calls "an open FR-275 defect". | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:74 |
| FND-003 | low | FR-006-AC-7 checks "operations", which is narrower than the Behavior bullet "uses no host floating point". A public `f64` field, an `as f64` cast or `f64::from_bits` can be read as a type use rather than an "operation", so two inspectors may disagree. AC-7 also excludes the consumed kernel, which the Behavior's "exact surface" arguably includes. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:72 |

## Verdict

CHANGES REQUESTED for FND-001 and FND-002. FR-004-AC-9 can fail: any reordering changes the literal bytes that `tc_015_actual_mixed_report_has_independent_exact_json` compares. NFR-002-AC-4 can fail if the manifest line is removed. None of the new criteria touch kernel meter behaviour owned by quire-exact FR-359.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-004-AC-10's stated procedure cannot tell key-first refusal from value-first decoding. `decode_campaign_snapshot` maps every serde error to `SnapshotError::Malformed`, through `refusal.get().unwrap_or(Malformed)`, and only `TextSeed` sets a distinct refusal. A decoder that fully decodes or skips the unknown member's invalid value first also returns Malformed. 'Receiving the same refusal' therefore holds in exactly the broken state the criterion exists to exclude. Likewise, a maximally nested value hits serde_json's recursion limit and also yields Malformed. The 'before its value is decoded' half needs an observation that differs, such as an internal seam or a value whose decoding would yield ResourceLimit. Otherwise it belongs in the AC-11 inspection. | spec/accounting/functional/FR-004-campaign-accounting.md:98 |

## Dispositions

Round 1, reviewed head 06d5f6799d826f5a2613f69b703ceb0ced718e16 (`git diff 754c888..06d5f67`). Each outcome was checked against the diff and the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The unfalsifiable scratch-bound criterion is gone. FR-004-AC-10 is now a Test of unknown-key refusal, and AC-11 is an Inspection whose fail condition is stated. The weak points of the replacement are recorded as FND-004 below and as SR-11541 FND-006. |
| FND-002 | fixed | FR-006 Verification now states that AC-7..AC-10 bind `src/exact/` only while it exists, retire with it under FR-275, and must never pass vacuously against an absent directory. |
| FND-003 | fixed | FR-006-AC-7 now covers types, fields, casts and operations, and names `f32` and `f64`. |
| FND-004 | fixed | Round 2 (70a0b8d): FR-004-AC-10 is now an Inspection, and no Test is claimed. The procedure names `FieldVisitor::visit_str`, `next_key::<Field>()`, the `next_value` and `next_value_seed` arms and `IgnoredAny`, and gives a concrete fail condition, so two inspectors reach the same verdict. At the head the code satisfies it: the unknown key returns `Err` from visit_str, `?` in `while let Some(field) = map.next_key::<Field>()?` (snapshot_json.rs:272, 333) propagates it before any value read, and there is no `IgnoredAny`. The Note honestly discloses that the public `Malformed` result cannot show key-first refusal. |
