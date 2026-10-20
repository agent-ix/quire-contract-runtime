---
id: SR-13323
title: "IR-882 PR 138 scope-boundary analysis: RT, CG, QSL and QSV NodeKey and identity ownership"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime@96c1519ae6a6b149696b168f8d6c2e3b9c1d1a55 vs origin/main de63b6b4; spec/assurance/AD-004-runtime-crate-layout.md, spec/exact/functional/FR-275-single-exact-kernel.md, spec/core/functional/interface-001-runtime-api.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: reviews
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#138. This analysis checks the boundary allocation the PR records: who constructs `NodeKey`, who holds it, who resolves wire ids, and who owns unit, enum and object-type identity. It compares that allocation with the owning sources: quire-exact `1c6cbcb6` `src/node.rs` and `src/value.rs`, quire-semantic-value `9fd13752`, QSL main ADR-012 (lines 875, 1099), QSL FR-060 T12-B/T12-F, and the QSL #1070 draft diff.

The allocation holds together on these points:

- RT holds `NodeKey` only inside `quire_exact::Value`. That matches quire-exact `ValueType::Composite(NodeKey)` and `CompositeValue.declaration`.
- RT library code constructs none. On main, the only RT `NodeKey::from_bytes` call outside `node.rs` is in `#[cfg(test)]` (`src/exact/equality.rs:689`, test module from line 604).
- Test fixtures use `from_digest`, which T12-B does not scan (`src/` only, `#[cfg(test)]` excluded).
- No new constructor and no hex parser.
- Unit, enum and object-type lookups belong to QSV owner types, matching AD-004 rows 87, 89, 90 and 116.

## Examined criteria

- AD-004 lines 272-273 (examined): "emitted `rt::NodeKey::from_bytes([..])` literals become `WireNodeId` literals resolved by lookup in the admitted node table."
- AD-004 H3 row line 282, step-4 row line 390 and Q-5 row line 455 (examined): "generated `WireNodeId` literals resolve by lookup in the admitted node table."
- QSL ADR-012 line 1099 on main and as amended in #1070 (context_only): "Only QSL converts a `WireNodeId` to a `NodeKey`: ADR-011 E4 and the `replay` facade."
- FR-275 lines 217-224 and interface-001 lines 147-150 (examined): RT consumes `UnitTable::get`, `TypeEnvironment::object_type`, `EnumMemberIndex::resolve` and `EnumDeclaration` directly.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-004 says generated `WireNodeId` literals "resolve by lookup in the admitted node table" but names no owner for that table or the lookup. QSL ADR-012 (main and #1070) says only QSL converts a `WireNodeId` to a `NodeKey`, through ADR-011 E4 and the `replay` facade. Without that owner, the RT and CG halves of step 4 could each build their own lookup table. Name the QSL `replay` facade or E4 as the lookup owner | spec/assurance/AD-004-runtime-crate-layout.md:272-273,282,390,455 |

## Verdict

**PASS with one low finding.** The hold-only boundary matches the owning crates' sources and QSL's T12-B scope. The only gap is that the lookup step is not attributed to its owner (FND-001).

## Dispositions

Review pass only; no disposition round has run yet.

Round 1, verified against fix commit a9b45d77 on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-004 lines 273-274, the H3 row, the step-4 row, the Q-5 paragraph and the Q-5 row now say only QSL converts `WireNodeId` literals to `NodeKey`, through ADR-011 E4 and the `replay` facade |
