---
id: SR-13321
title: "IR-882 PR 138 spec review: Q-5 hold-only NodeKey boundary and QSV ownership"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-runtime@96c1519ae6a6b149696b168f8d6c2e3b9c1d1a55 vs origin/main de63b6b4; spec/assurance/AD-004-runtime-crate-layout.md, spec/core/functional/interface-001-runtime-api.md, spec/exact/functional/FR-275-single-exact-kernel.md; leftover sweep over spec/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: reviews
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#138 (spec only, 3 files, +35 -22). Reviewed in a fresh detached worktree at the PR head; scope is `git diff origin/main...HEAD`.

Verified against sources, not the PR text:

- quire-exact `1c6cbcb6` (RT `Cargo.lock` revision and current quire-exact main) `src/node.rs`: public `NodeKey::from_digest([u8; 32])` and `NodeKey::decode_admitted([u8; 32])` exist. The module doc says only the checking stage calls `from_digest` in production and `decode_admitted` runs only after the admitted-package identity check; tests may call `from_digest` freely. No hex parser exists.
- quire-exact `src/value.rs`: `ValueType::Composite(NodeKey)` (line 380) and `CompositeValue { declaration: NodeKey, .. }` (line 1500). The kernel `Value` does carry `NodeKey`, so "RT holds `NodeKey` at type level" is accurate.
- quire-semantic-value `9fd13752` (locked and current main): `UnitTable::get(&self, UnitId) -> Option<&QuantityUnit>` (quantity.rs:195), `TypeEnvironment::object_type(&self, EffectiveId) -> Option<&ObjectTypeDeclaration>` (declaration.rs:2444), `EnumMemberIndex::resolve(&self, VariantId) -> Option<&EnumValue>` (enumeration.rs:231), and `EnumDeclaration` (enumeration.rs:38). All four exist with those signatures.
- QSL `tools/arch-lint/api_surface.rs` walks `src/` only and excludes `#[cfg(test)]` items, so "T12-B scans shipped code only" holds for `tests/exact_*.rs` fixtures.
- `WireNodeId` exists (`qsl_foundation::digest::WireNodeId`), and CG main already uses it in `src/replay/`.
- Leftover sweep over `spec/`: no statement says Q-5 is open, and none cites QSL-530 or `parse_hex32`. FR-275 no longer allocates registries to RT. Q-1 to Q-4 and Q-6 rows and the "Q-1 to Q-4 and Q-6 are answered" paragraph are unchanged. The PR edits no QSL ADR-012 row; AD-004 says the rows "need amendment in the QSL repository", which is QSL draft PR #1070 (open, draft, head e1b5ec81, edits ADR-012 lines 875 and 1099 only).
- Closed PR #137 findings: SR-13314 FND-001 (FR-275 prose versus AD-004 ownership) is resolved by the FR-275 and interface-001 amendments, which now match AD-004 rows 87, 89, 90 and 116. The other SR-13313 and SR-13314 findings were about `src/exact/identity_registry.rs` and `tests/exact_kernel_registries.rs`. PR #137 is CLOSED and neither file exists on main or at the head, so they are moot. SR-13314 FND-004 (no AC owns the registry rule) carries forward as SR-13322 FND-001.
- `make spec` at the head and at origin/main: 228/228 grammar-clean, "Coverage: 90/198 rows backed", "124 unbacked row(s) and 0 contradicted status(es)". Sorted outputs differ only by interface-001 AC line numbers (+5 lines).

Sub-analyses run: integrity (SR-13322) and scope-boundary (SR-13323), plus gap-analysis (SR-13324). EARS analysis was skipped because the PR adds or edits no requirement statement or acceptance criterion; it changes only prose and table cells. Rust review and Kani were not applicable (spec only).

## Examined criteria

- AD-004 NodeKey row (examined): "same 32 bytes, `Display` and `Debug`. RT has a local constructor; `quire-exact` provides `from_digest` and `decode_admitted`. The local definition is deleted in the cutover; RT library code constructs no shared key."
- AD-004 H3 row, line 282 (examined): "Q-5 answered, option A, hold-only: RT holds shared keys through `quire_exact::Value` but constructs none in library code. CG constructs none ..."
- AD-004 step-4 row, line 390 (examined).
- AD-004 Q-5 paragraph, lines 434-447 (examined), and Q-5 table row, line 455 (examined): "Q-5. May RT or CG construct a `NodeKey`? | planner-relayed owner decision | answered, option A, hold-only ..."
- AD-004 Routed gaps R-5, line 464 (examined).
- AD-004 CG paragraph, lines 272-273 (examined): "Two uses require repointing in CG ... become `WireNodeId` literals resolved by lookup in the admitted node table."
- FR-275 "Shared identity and declaration owners", lines 217-224 (examined).
- interface-001 lines 147-150 (examined).
- FR-012 Behavior line 48 and FR-012-AC-1 (examined): "`NodeKey::from_hex` accepts exactly the 64-lowercase-hex-digit inputs ..."
- TC-033 step 2 (examined): "Round-trip digests through `from_bytes`, `as_bytes`, `Display` and `from_hex`."
- AD-004 Q-1 to Q-4 and Q-6 rows (context_only, unchanged).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR removes `NodeKey::from_hex` from AD-004's "Still missing" list, and the NodeKey row now says the local definition "is deleted in the cutover". But FR-012 Behavior and FR-012-AC-1 still require RT's `NodeKey::from_hex` and the `from_bytes`/`from_hex` round trip, and TC-033 step 2 still tests them. The step-4 row names no amendment or retirement of FR-012-AC-1 or TC-033, so after IR-882 deletes the local `NodeKey`, an AC remains that no code can satisfy and hex decoding has no stated owner. The NodeKey row's "RT has a local constructor" also hides that RT has two (`from_bytes`, `from_hex`) | spec/exact/functional/FR-012-carried-compiler-vocabulary.md:48,98; spec/exact/matrix/TC-033-carried-compiler-vocabulary.md:20; spec/assurance/AD-004-runtime-crate-layout.md:127,291,390 |
| FND-002 | medium | The Q-5 row's Owner cell says "planner-relayed owner decision" and cites no source; every other answered row cites its ticket ("IR-349 answer"). The only record found is an IR-882 comment, which attributes the answer to draco:QSL:code via luna:IR:plan, not to the owner. The owning QSL spec, ADR-012 lines 875 and 1099 on QSL main, still says "RT holds no `NodeKey`", and the fix (QSL #1070) is an open draft. RT would record Q-5 as "answered" while the owning decision record says the opposite. Cite the source, and land this PR with or after QSL #1070 | spec/assurance/AD-004-runtime-crate-layout.md:434-447,455 |

## Verdict

**FAIL: not mergeable as is (two medium findings).** The substance is verified. The quire-exact constructors and `Value`'s `NodeKey` fields, the four QSV owner APIs at the locked revisions, `WireNodeId`, and the T12-B shipped-code-only scope all match the text. No leftover in `spec/` says Q-5 is open, or cites QSL-530 or `parse_hex32`. Q-1 to Q-4 and Q-6 are untouched. The PR does not claim to edit ADR-012. The PR #137 ownership finding is addressed. To fix: settle FR-012-AC-1 and TC-033's `from_hex` fate in the step-4 row or amend them (FND-001), and give Q-5 a citable source and ordering against QSL #1070 (FND-002). Also see SR-13322 FND-001.

## Dispositions

Review pass only; no disposition round has run yet.

Round 1, verified against fix commit a9b45d77 on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The step-4 row now retires FR-012-AC-1's RT-local constructors and TC-033's parser and round-trip steps. FR-012 Behavior says hex decoding has no RT owner, and the NodeKey row names both local constructors. The fix introduced a new defect in how FR-012-AC-1 is re-specified; see SR-13322 FND-003 and SR-13324 FND-002 |
| FND-002 | fixed | The Q-5 Owner cell now reads "QSL lane technical answer from `draco:QSL:code`, relayed by `luna:IR:plan` on 2026-10-19; tracked by IR-882". The paragraph says it is not a first-hand owner ruling and that QSL #1070 must land with or before this PR |
