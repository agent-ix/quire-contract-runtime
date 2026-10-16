---
id: SR-11541
title: "IR-65 integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-runtime@754c88851c53587c47009b6d4815fbb9cc1429d5; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/non-functional/NFR-001-no-std-footprint.md, spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-65. Checked the changed statements and criteria for completeness, consistency with surrounding text and other requirements, atomicity, and preservation of prior obligations (the brief claims nothing was removed).

## Reviewed scope

| Unit | Role | Path | Excerpt |
| --- | --- | --- | --- |
| FR-004 nested-metadata clause | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Decode shall reject undeclared nested objects or arrays rather than silently accepting them as metadata; the decoded snapshot shall expose only the declared identity and counters. |
| FR-004 parser-scratch clause | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Parser scratch shall remain bounded by the input cap. Allocator exhaustion can still abort: these limits do not promise recoverable OOM. |
| FR-004-AC-9 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Encoded snapshots emit `schemaVersion`, `requirement`, `revision`, `counterSemantics`, `counts` in that order, with `accepted`, `rejected`, `failed`, `discarded` in that order inside `counts`; byte-for-byte comparison of encoded output observes this order. |
| FR-004-AC-10 | examined | spec/accounting/functional/FR-004-campaign-accounting.md | Inspection of the decoder and its parser dependency confirms that parser scratch grows only with the capped input, with no unbounded allocation for rejected nested metadata. |
| NFR-001 Statement | context_only | spec/core/non-functional/NFR-001-no-std-footprint.md | The default runtime shall compile without `std`, heap allocation, unsafe code, or required third-party dependencies, and its release artifact shall remain within the measured v0.1 footprint budget. |
| NFR-001 Scope (exact) | examined | spec/core/non-functional/NFR-001-no-std-footprint.md | The optional `exact` feature also enables `alloc` and uses allocating collections and values without requiring `std`. Its linked size is outside the default-profile footprint measurement. |
| NFR-002-AC-4 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `publish = false`, disabling registry publication for this package. |
| NFR-002-AC-5 | examined | spec/core/non-functional/NFR-002-panic-compatibility-license.md | The crate manifest declares `license = "AGPL-3.0-or-later"`, and the existing `make deny` gate checks the resolved dependency set against the license policy. |
| FR-006-AC-7 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Inspection of `src/exact/` finds no host `f32` or `f64` operations in the exact surface. |
| FR-006-AC-8 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | The `exact` feature remains available with crate-level `#![no_std]` and `#![forbid(unsafe_code)]`; inspection of non-test code in `src/exact/` finds no intentional panic site. |
| FR-006-AC-9 | examined | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md | Every public item exposed through `quire_contract_runtime::exact` is re-exported by `src/exact/mod.rs` from a private child module; the child modules are not public paths. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The change drops an obligation, so "nothing removed" is false. The deleted line forbade both ignored nested metadata and an "intermediate Value tree". The replacement covers only the metadata half. Nothing now forbids decoding through `serde_json::Value` and then validating, and AC-10 does not catch it either: such a tree is still bounded by the 64 KiB cap. | spec/accounting/functional/FR-004-campaign-accounting.md:64-66 |
| FND-002 | medium | NFR-001 now contradicts its own scope. The Statement forbids heap allocation, and the Scope covers "every symbol linked by generated customer code". FR-006 says generated oracles evaluate through `exact`, and the new paragraph says `exact` allocates. A generated oracle linking `exact` therefore puts allocating symbols inside the scope of a no-allocation statement. The paragraph excludes only `exact`'s linked size, not the allocation obligation. | spec/core/non-functional/NFR-001-no-std-footprint.md:19-27 |
| FND-003 | medium | NFR-002-AC-5 is compound and gets one Inspection verdict. It joins a manifest-field fact (`license = …`) to a claim about a gate's behaviour ("make deny checks the resolved dependency set"). One can hold while the other fails: the field is present while a newly added copyleft dependency makes `make deny` red. | spec/core/non-functional/NFR-002-panic-compatibility-license.md:47 |
| FND-004 | low | FR-006-AC-8 combines three independent obligations under one Inspection verdict: the feature builds under `#![no_std]`, the crate has `#![forbid(unsafe_code)]`, and there are no non-test panic sites. A partial failure cannot be reported against the right obligation. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:73 |
| FND-005 | low | The new FR-004 nested-metadata sentence repeats rules already present: a nested object or array under an undeclared key is an "unknown member", and one under a declared key is an "incorrect type". Two rules now govern the same input. The codec returns `Malformed` for both, so behaviour is not wrong, but the duplicate is a maintenance hazard. | spec/accounting/functional/FR-004-campaign-accounting.md:61-66 |

## Verdict

CHANGES REQUESTED for FND-001 through FND-003. FR-004-AC-9 is atomic and matches the stated member order and the codec's encoder. NFR-002-AC-4 is atomic and consistent with the NFR-002 Statement and its Measurement row "Registry publication enabled | inspection". FR-006-AC-7 and AC-9 restate the existing FR-006 Behavior bullet without contradicting it.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The FR-004 parser-scratch clause again has no criterion. At 754c888 FR-004-AC-10 bound it. The fix replaced AC-10 with the unknown-key test and added AC-11 for the value tree, but neither bounds parser scratch. IR-65 is an AC under-coverage sweep, so the slice leaves one of its own target clauses uncovered. Either sharpen it to a falsifiable bound (for example, escaped-string scratch never exceeds input length) or state that it is covered by FR-004-AC-7's byte limit. | spec/accounting/functional/FR-004-campaign-accounting.md:76-77 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | Restoring the base Scope sentence also dropped the slice's `exact` paragraph. NFR-001 again does not say that `exact` allocates. The no-heap reading of 'every symbol linked by generated customer code' for an oracle linking `exact` or `snapshot-json` is back to its base ambiguity. The owner note leaves only the footprint half open, not no-heap. This was pre-existing at ac0efa9, so the slice is no worse than base, but intent (2) of the slice, that NFR-001 scope names `exact`, is no longer delivered. Extend the note to cover the no-heap half as well. | spec/core/non-functional/NFR-001-no-std-footprint.md:19, 26-28 |

## Dispositions

Round 1, reviewed head 06d5f6799d826f5a2613f69b703ceb0ced718e16 (`git diff 754c888..06d5f67`). Each outcome was checked against the diff and the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The value-tree prohibition is restored as an observable obligation, plus FR-004-AC-11 with a stated failing condition (any generic JSON tree before member validation). Undeclared nested metadata stays rejected by the existing 'unknown members' and 'incorrect types' clauses. |
| FND-002 | fixed | NFR-001 Scope no longer puts 'every symbol linked by generated customer code' under the no-heap statement; `exact` sits outside it and the exact footprint question is a labelled owner note. The narrowing this introduces is recorded as SR-11544 FND-003. |
| FND-003 | fixed | Split into NFR-002-AC-5 (manifest field, Inspection) and NFR-002-AC-7 (`make deny` license pass, sca-sbom). |
| FND-004 | fixed | FR-006-AC-8 is now the single no_std build obligation. Panic sites move to AC-10 and unsafe to NFR-001-AC-2 (named in Verification). |
| FND-005 | fixed | The redundant 'Decode shall reject undeclared nested objects or arrays…' sentence is removed. The 'unknown members' and 'incorrect types' clauses at FR-004:61-63 still cover the input. |
| FND-006 | fixed | Round 2 (70a0b8d): New FR-004-AC-12 binds the 'Parser scratch shall remain bounded by the input cap' clause again. Its Verification names `decode_campaign_snapshot`, `MAX_BYTES`, serde_json's escaped-string buffer and nesting controls, and `TextSeed`'s identity copies, which are checked against `MAX_IDENTITY_BYTES` at snapshot_json.rs:246 before `String::from`. It gives concrete fail conditions: an unbounded per-member allocation, an unbounded nesting stack, or growth unrelated to the cap. 'Proportional' now excludes the merely-finite reading. |
| FND-007 | fixed | Round 3 (c77472d): The owner note now leaves both halves open: no-heap for generated oracles linking `exact` or `snapshot-json`, and the separate footprint-size scope for `exact`. It decides neither. The Scope sentence 'The default feature set and every symbol linked by generated customer code.' is byte-identical to ac0efa9: `git diff ac0efa9 c77472d` on NFR-001 shows only the added Note. |
