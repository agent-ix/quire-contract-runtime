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

Check the absence of RT-local node-key ownership after the IR-882 key-owner cutover, the closed
refusal spellings, the structural semantic-graph causes and normalized unit values. Evidence:
`tests/exact_vocabulary.rs` (`--features exact`). The retired parser and round-trip steps are
preserved below as historical FR-012-AC-1 text.

## Test Procedure

1. Run `tc_033_runtime_has_no_local_node_key_constructor`. Walk every Rust source recursively
   under `src/`, `verification/`, `examples/`, `measurement/` and `scripts/`, excluding only
   `#[cfg(test)]` items. Inspect for a local `NodeKey` type or alias, a public re-export, any
   constructor reference or call (including function pointers, qualified paths and macro token
   streams), and local `NodeKey` definitions inside macros. Flag named hex decoding paths and
   functions taking `&str` or `&[u8]` and returning `[u8; 32]` (also through `Option` or
   `Result`). Confirm the retired parser and round-trip tests are absent.
2. Compare the kernel's `NODE_KEY_DOMAIN` and RT's `COMPOUND_UNIT_DOMAIN` with their normative strings.
3. Check `SelectionRefusalCode::ALL` order, each `as_str`, `from_code` over each spelling and over
   unlisted spellings, and `PackageRefusalCode::as_str`.
4. Drive `UnitGraph::admit` and `check_terms` into each structural `SemanticGraphCause`.
5. Build `Dimension` and `CompoundUnit` from the same terms in several orders; compare and iterate.

## Retired Procedure (FR-012-AC-1)

1. Before the step-4 cutover only, check the local parser's 64-lowercase-hex-digit behavior.
2. Before the step-4 cutover only, check the local `from_bytes`, `as_bytes`, `Display` and
   `from_hex` round trip. Retire both steps and their tests when the local type is deleted;
   test-only shared-key fixtures then use `NodeKey::from_digest([u8; 32])`.

## Expected Results

RT has no local `NodeKey` type, alias, constructor, hex decoder or public re-export; its library
source contains no `NodeKey` construction. The retired parser and round-trip tests are absent.
The remaining vocabulary spellings are byte-exact, every structural cause is reachable, no
compiler-admission cause is, and normalized unit values are order-independent.
