---
id: TC-033
title: "Carry the compiler vocabulary byte-exactly"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-012
    type: verifies
---
# TC-033: Carry the compiler vocabulary byte-exactly

## Description

Check the pre-cutover RT-local node-key parsing and round-tripping, the closed refusal spellings,
the structural semantic-graph causes and normalized unit values. Evidence: `tests/exact_vocabulary.rs`
(`--features exact`). The node-key checks below are historical evidence for retired
FR-012-AC-1 and leave with RT's local `NodeKey` in IR-882 step 4; they do not specify
shared-key behavior or an RT hex decoder.

## Test Procedure

1. Before the step-4 cutover only, check the local parser's 64-lowercase-hex-digit behavior.
2. Before the step-4 cutover only, check the local `from_bytes`, `as_bytes`, `Display` and
   `from_hex` round trip. Retire both steps and their tests when the local type is deleted;
   test-only shared-key fixtures then use `NodeKey::from_digest([u8; 32])`.
3. Compare `NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` with their normative strings.
4. Check `SelectionRefusalCode::ALL` order, each `as_str`, `from_code` over each spelling and over
   unlisted spellings, and `PackageRefusalCode::as_str`.
5. Drive `UnitGraph::admit` and `check_terms` into each structural `SemanticGraphCause`.
6. Build `Dimension` and `CompoundUnit` from the same terms in several orders; compare and iterate.

## Expected Results

Before step 4, the local node-key tests describe only the old RT constructors. The remaining
vocabulary spellings are byte-exact, every structural cause is reachable, no compiler-admission
cause is, and normalized unit values are order-independent.
