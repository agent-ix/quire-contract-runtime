---
id: SR-11545
title: "IR-65 evidence review"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-runtime@754c88851c53587c47009b6d4815fbb9cc1429d5; spec/accounting/functional/FR-004-campaign-accounting.md, spec/core/non-functional/NFR-002-panic-compatibility-license.md, spec/exact/functional/FR-006-exact-outcomes-and-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-65. Compared each new criterion's verification method with the evidence that already exists or could exist in the repo: `make test-features` (including its `build-exact-no-std-msrv` thumbv7em row), `make deny`, `make lint` and the compile_fail doctests. I also checked whether each Inspection criterion states a falsifiable procedure.

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
| NFR-002 Measurement | context_only | spec/core/non-functional/NFR-002-panic-compatibility-license.md | License policy violations | 0 | 0 | sca-sbom |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-006-AC-8's no_std half is marked Inspection, but inspecting the `#![no_std]` attribute cannot detect a dependency that pulls in `std`. That is the real way the obligation fails, for example `unicode-normalization` or `quire-exact` losing `default-features = false`. An executing gate already exists: the `build-exact-no-std-msrv` row of `make test-features` builds `--features exact` for `thumbv7em-none-eabi`. The criterion should be compile-time-check against that row. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:73 |
| FND-002 | medium | NFR-002-AC-5's license-policy half is marked Inspection, while NFR-002's own Measurement row assigns license policy to sca-sbom. Inspecting the Makefile confirms that `make deny` exists, not that it passes. If a GPL-only crate is added, `make deny` fails while AC-5 still passes by inspection. | spec/core/non-functional/NFR-002-panic-compatibility-license.md:47 |
| FND-003 | medium | FR-006-AC-8's "no intentional panic site" gives no inspection procedure. Neither the AC nor Verification says which constructs count: `panic!`, `unreachable!`, `unwrap`/`expect`, `assert!`, slice indexing, or overflowing arithmetic (the crate also denies `clippy::indexing_slicing` and `arithmetic_side_effects`). Two inspectors can reach opposite verdicts on the same code, for example over an `expect` on a branch the author considers infallible. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:73-81 |
| FND-004 | low | FR-006-AC-9's "child modules are not public paths" can be a compile-time check: a `compile_fail` doctest such as `use quire_contract_runtime::exact::equality;`, the same pattern `src/exact/mod.rs` already uses for `exact::Integer`. That would be executed evidence rather than an unbacked Inspection row. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:74 |

## Verdict

CHANGES REQUESTED for FND-001 through FND-003. Test is the right method for FR-004-AC-9, and `tc_015_actual_mixed_report_has_independent_exact_json` already compares the literal encoded bytes; only its trace tag is missing. Inspection is the right method for NFR-002-AC-4: nothing in `make deny` checks `publish`, as the Verification text states. FR-006-AC-7 is a valid Inspection with a stated subject (`src/exact/`). FR-004-AC-10's Inspection method is reasonable, but its pass criterion is weak; see SR-11542 FND-001.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The single bound doctest (`use quire_contract_runtime::exact::equality;`) cannot fail for 15 of the 16 child modules. FR-006-AC-9 claims every child path is inaccessible and that every public item comes from a private child. A regression to `pub mod boolean;`, or a `pub fn` defined directly in `mod.rs`, keeps that doctest green. Bind one `compile_fail` per child module, or add an inspection half for the 'every item re-exported from a private child' clause. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:74, 85-87 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | The fix introduced an uncatalogued verification method. `quire coverage --strict` at 70a0b8d emits `[uncatalogued-verification-method] 'compile-time-check and Inspection' is neither a declared verification_catalog method id nor a declared class, so nothing can say what discharging it means` for FR-006-AC-9. The row cannot be discharged or classified by tooling. Write one declared method in the cell, such as `compile-time-check`, and keep the mod.rs inspection in Verification, or split the inspection into its own criterion. | spec/exact/functional/FR-006-exact-outcomes-and-accounting.md:74 |

## Dispositions

Round 1, reviewed head 06d5f6799d826f5a2613f69b703ceb0ced718e16 (`git diff 754c888..06d5f67`). Each outcome was checked against the diff and the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-006-AC-8 is compile-time-check bound to the existing `build-exact-no-std-msrv` row (scripts/run_feature_matrix.py:83, `--features exact --target thumbv7em-none-eabi` on +1.98.1). |
| FND-002 | fixed | NFR-002-AC-7 is sca-sbom via `make deny`, which runs `cargo deny --workspace check licenses …`. That matches the NFR-002 Measurement row. |
| FND-003 | fixed | FR-006 Verification now gives a closed construct list for a panic site and excludes `#[cfg(test)]`. |
| FND-004 | fixed | FR-006-AC-9 is now compile-time-check through a `compile_fail` doctest. The doctest is not yet authored, which the TM-001 row discloses. Its coverage gap is recorded as FND-005. |
| FND-005 | fixed | Round 2 (70a0b8d): The 16 named children match the 16 `mod` declarations in src/exact/mod.rs at the head exactly: boolean, collection, composite, containment, definition, division, enumeration, equality, expression, ieee, key, node, quantity, reference, stop and unit. Verification requires comparing the list against mod.rs so a new child cannot be omitted. It adds an inspection for direct public definitions; mod.rs has none today. Each doctest must fail on privacy, not on absence. |
| FND-006 | fixed | Round 3 (c77472d): The cell is now the single id `compile-time-check`. That is a declared catalog method: `quire coverage --strict` at c77472d emits no `uncatalogued-verification-method` warning (0 occurrences; the only diff from 70a0b8d is that warning's removal), and the same id is already accepted on NFR-001-AC-1 and NFR-002-AC-3. No obligation was dropped. The per-child sentence moved out of the criterion cell, but the Verification prose still lists all 16 private children for individual `compile_fail` doctests, requires comparing that list against mod.rs, and keeps 'Inspect `mod.rs` for direct public definitions, including `pub fn` and `pub struct`; any such definition fails the re-export-only clause.' |
