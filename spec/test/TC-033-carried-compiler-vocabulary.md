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

Check node-key parsing and round-tripping, the closed refusal spellings, the structural
semantic-graph causes and normalized unit values. Evidence: `tests/exact_vocabulary.rs`
(`--features exact`).

## Test Procedure

1. Parse 64 lowercase hex digits; reject short, long, uppercase and non-hex inputs.
2. Round-trip digests through `from_bytes`, `as_bytes`, `Display` and `from_hex`.
3. Compare `NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` with their normative strings.
4. Check `SelectionRefusalCode::ALL` order, each `as_str`, `from_code` over each spelling and over
   unlisted spellings, and `PackageRefusalCode::as_str`.
5. Drive `UnitGraph::admit` and `check_terms` into each structural `SemanticGraphCause`.
6. Build `Dimension` and `CompoundUnit` from the same terms in several orders; compare and iterate.
7. Inspect the `node.rs`, `definition.rs` and `unit.rs` re-exports for a `Meter` parameter or
   return, and for an item this requirement does not name.

## Expected Results

Every spelling is byte-exact, every structural cause is reachable, no compiler-admission cause is,
normalized unit values are order-independent, and nothing in the vocabulary touches a meter.
