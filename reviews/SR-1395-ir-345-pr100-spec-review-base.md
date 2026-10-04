---
id: "SR-1395"
title: "IR-345 spec review (base): runtime crate layout AD-004 (PR 100)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@5a09652802d2e0e13a6f233e9a2ad723d9bba16f; spec/assurance/AD-004-runtime-crate-layout.md, spec/spec.md (registry rows and references); base origin/main fdab1a0"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
---

# SR-1395: IR-345 base spec review of PR 100

## Summary

Ticket: IR-345. PR agent-ix/quire-contract-runtime#100 adds AD-004 (347 lines, status proposed)
and adds AD-004 to the four subsystem rows and the references list of `spec/spec.md`. This is the
base lens. It covers the measurements, the rule checks and the gate baseline. The integrity,
dependency and scope-boundary lenses are SR-1396, SR-1397 and SR-1398. EARS does not apply,
because the AD adds no FR, NFR or StR statement. L-1 to L-5 are local labels, not requirements.

## Method

All measurements were taken read-only at RT head 5a09652 (identical to origin/main under `src/`),
at QSL `origin/main` and at CG `origin/main`, after a fetch. QSL and CG sources were exported with
`git archive` to the scratchpad. No other repository was edited.

- Layout: `wc -l` over every file. src/ has 34 files and 13,791 lines, and exact/ has 23 files and
  12,080 lines. All 23 per-file counts match. The bucket sums match: 5,845 / 728 / 3,463 / 978 / 907,
  plus `mod.rs` 159. The core files match. `tests/` has 16 files and 9,591 lines, of which 12
  `exact_*` files hold 8,686 lines. `verification/kani.rs` is 295 lines. The negotiator line
  ranges are right.
- Shared names: wrote and ran my own script, which strips comments, attributes, `pub` and module
  paths, then compares bodies. It reproduces 151 / 102 / 72 names, 124 shared (82 in QE, 49 in
  QSV, 7 in both), 49 same, 45 same apart from `non_exhaustive`, 30 different (= 22 different + 8
  boxed), and 27 RT-only types. The 22 "different" names match the AD's list exactly. Seven of the
  8 "boxed" names are confirmed by reading their private `*Fields` structs.
- Read in full: Meter/accounting (both), Integer, Decimal, Outcome/Refusal/Undefined, NodeKey,
  ObjectClosure vs ObjectEnvironment. Also read the declarations of Value, ValueType, Quantity,
  ObjectReference, Origin, Location, FieldDeclaration, PackageCause, CheckingLimits,
  ObjectTypeDeclaration, TypeEnvironment and CheckedEquality.
- Meter claims confirmed. QSL `check_injected` never clears the denial, and `admit` is not reached
  on a denied charge, so every later charge at that point is denied again. RT sets
  `self.denial = None` instead, which is what FR-010-AC-5 requires. QSL `occurrence: u64` against
  RT `NonZeroU64` is confirmed. QSL `ChargePoint::ALL` has 62 entries against RT's 52. RT's 52 are
  a subset of QSL's, and the extra ten are the ones the AD names.
- NodeKey: RT has `const fn from_bytes` and `from_hex`. QSL has `from_digest` and `decode_admitted`.
  `Display` and `Debug` are byte-identical.
- QSL: PR #590 and PR #608 are MERGED. The still-missing list is confirmed: `CheckingLimits` is
  `{nodes, input_bytes, work_budget}` with no depth field, `PackageCause` lacks the 3 variants,
  `Origin`/`Location` are defined in both `quire-exact/src/location.rs` and
  `quire-semantic-value/src/location.rs` and differ, and `FieldDeclaration` is defined in both and
  differs.
- CG: the `rt::` counts match (105/47, 112/31, 90/20, 9/4), as do the import and alias lines, the
  mirrors, `from_hex` / `from_bytes` and the manifest in `src/core/profile.rs`. CG uses no
  charge log.
- Gate baseline: ran `quire coverage --strict` at origin/main and at head. Both report 56 unbacked
  rows and 5 contradicted statuses, and the sorted outputs are identical (empty diff). `quire
  validate` passes 94/94 at head. Every frontmatter relationship target exists as a document id.
  The AD contains no commit SHA or version pin.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AD cites code in another repo by line number, and those numbers drift: CG `src/oracle/scalar/mod.rs` `:58`, `:2396`; `equality/mod.rs` `:79`, `:1417`, `:194-271`, `:1034`, `:1887`; `function/mod.rs` `:145`, `:1452`, `:413-429`; `kani/generate/scalar.rs` `:23`, `:82`; `Cargo.toml` `:22`, `:38`. All are correct today. But the kani `:23`/`:82` lines are doc comments, not the emitted `rt::Integer` text (that is at :386/:428). Cite by symbol (`render_key`, `oracle_crate_manifest`, the `use ... as rt` alias, the mirror type names) instead. | AD-004 "CG paths that change in the same step" |
| FND-002 | low | The CG name split (78 = 50 + 8 + 14 + 6) does not reproduce, and it understates the edits. I get 79 distinct names: 39 struct/enum names in the same/same-except-`non_exhaustive` classes plus 12 free functions, 8 boxed, 14 different and 6 RT-only. `convert_quantity` changed signature (`&Quantity` became `UnitQuantity<'_>`), but the 14 "use-site change" names do not include it. The 23 names in the `non_exhaustive`-only class are called "path change only", but dropping `#[non_exhaustive]` makes CG's wildcard and `unreachable!` arms dead patterns, which the AD's own Risks section says. The shared `ValueType` also changes payloads: `Enum(EnumShape)`, `Float(FloatType)`, `Quantity(UnitId)`, `Reference(EffectiveId)` and a new `Population(Option<u64>)`. `Value` gains `Population(PopulationId)` and `Enum(EnumMember)`. CG's exhaustive `ValueType` renderers will meet all of these, and none is named. | AD-004 "CG paths that change in the same step", "The 78 distinct names" |
| FND-003 | low | Several measurement statements are inexact. (a) It says "the three line sums by target" and then lists five. (b) It says "the two QSL crates 1" `#[non_exhaustive]` site, but the only hit is a `finish_non_exhaustive()` call: QSL has 0 attribute sites. (c) `Integer` is put in "same fields, boxed in RT", but its RT declaration is `{small: i64, big: Option<Box<BigInt>>}` against QSL's `(BigInt)`, so it is a different representation, not the same fields boxed. The AD's own Integer row says so. (d) The RT-only tally "27 types, 7 free items" leaves out `pub type Body`, which the 15-item `expression` keep-list counts. (e) It calls `Refusal` "a unit-variant `Copy` enum in RT", but three RT variants carry fields (`DivisionPairOutOfDomain`, `IeeeNotExact`, `CardinalityOutOfBound`). (f) It calls `UniverseIdentity` and `ObjectIdentity` "successors under another name", but they change representation: `Box<[u8]>` becomes a 32-byte digest, and object bytes become a string id. (g) `measurement/footprint` 124 counts only `.rs` files (137 with the manifest). | AD-004 "Current layout", "Shared names: classification", "What this crate keeps" |

## Verdict

**Base lens: no high finding; three lows.** The measurement work is careful and reproducible: every
headline count re-measured exactly, the Meter single-shot defect is real and correctly described,
QSL status claims are true, CG counts are exact, and the strict baseline is not raised (identical
sorted output, 56 unbacked / 5 contradicted at both). No SHA, pin or version, no vendoring
proposal, no quire-research internals; per-repo sequencing only. Mergeability across all four
lenses is stated in SR-1398's verdict.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@c7087f953a24cc49774ed81c5914d5eef0f3fbd0 (fix commits 6a5d672, c7087f9). Each item was re-measured by the reviewer.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6a5d672. A grep for `:NNN` and `.rs:NNN` finds no CG line citation left. CG paths are now cited by symbol: the generator's `use ...exact::{..}`, the emitted `as rt` alias, `oracle_crate_manifest`, `oracle/equality`. The Kani row says the doc comments also name the path. The only line ranges left point into this repository's own files (`division` 180-233 and similar), which the rule does not cover. |
| FND-002 | fixed | 6a5d672. The AD now gives 79 names = 16 + 23 + 12 free functions + 8 + 14 + 6, which matches my recount exactly. It names the `convert_quantity` signature change. The 23 names are marked as producing dead wildcard and `unreachable!` arms. The `ValueType` and `Value` payload changes and the `Population` variants are named. |
| FND-003 | fixed | 6a5d672. All seven parts are corrected: "five line sums"; QSL has no `#[non_exhaustive]` attribute and its one textual hit is `finish_non_exhaustive()`; `Integer` has its own "same value, different representation" class and boxed is now 7; the RT-only tally includes `Body`; the `Refusal` fields are named; the `UniverseIdentity`/`ObjectIdentity` representation change is stated; footprint is "124 (.rs; 137 with the manifest)". |
