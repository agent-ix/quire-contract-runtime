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
refusal spellings, and QSV's structural semantic-graph causes and normalized unit values. RT evidence:
`tests/exact_vocabulary.rs` and `tests/exact_unit_owner.rs` (`--features exact`).
`quire-integration` owns step 2's producer-origin Test because RT's source policy excludes QSL
and IR dependencies; RT owns the source Inspection in step 1 and FR-012-AC-8. The retired
parser and round-trip steps are
preserved below as historical FR-012-AC-1 text.

## Test Procedure

1. Run `tc_033_runtime_has_no_local_node_key_constructor`. Walk every Rust source recursively
   under `src/`, `verification/`, `examples/`, `measurement/` and `tools/`, excluding only
   `#[cfg(test)]` items. Inspect for a local `NodeKey` type or alias, a public re-export, any
   constructor reference or call (including function pointers, qualified paths and macro token
   streams), and local `NodeKey` definitions inside macros. Flag named hex decoding paths and
   functions taking `&str` or `&[u8]` and returning `[u8; 32]` (also through `Option` or
   `Result`). Confirm the retired parser and round-trip tests are absent.
2. In `quire-integration`, emit QSL checked packages and admit their same bytes through the
   checked-package reader. Select the declaration, foreign reference-universe and declared and
   compound unit node ids from the admitted values, then obtain their `NodeKey` values through
   QSL's read-only admitted-node lookup on the corresponding package. Drive RT consumers with
   those identities and confirm the
   retained declaration identity, foreign-universe refusal and cross-domain inequality, including
   the equal-byte declared/compound control. The test does not remint or pin an identity in RT.
3. Check `SelectionRefusalCode::ALL` order, each `as_str`, `from_code` over each spelling and over
   unlisted spellings, and `PackageRefusalCode::as_str`.
4. Drive QSV `DimensionNode::checked`, `UnitNode::checked`, `UnitGraph::from_checked_nodes`,
   `EnumDeclaration::new` and `EnumDeclaration::member` into their structural
   `SemanticGraphCause` outcomes. The graph's `BTreeMap` input cannot hold a repeated node key.
   Give one graph several topology faults at once and assert the first cause in QSV's
   cross-dimension, missing-root, duplicate-root and cycle order.
5. Build QSV `Dimension` and `CompoundUnit` from graphs whose nodes were inserted in several
   orders; compare their identities and ascending, zero-free terms.
6. After the R-3 definition cutover, inspect RT library source for local definitions, aliases,
   re-exports or adapters of `PackageCause`, `PackageRefusal` and `PackageRefusalCode`; check that
   RT's selection and package refusal callers import QSV's definition owner directly. Confirm the
   retired `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` variants do not
   occur on RT's `invalid_package` surface, and repeat step 3 through the QSV types.

## Retired Procedure (FR-012-AC-1)

1. Before the step-4 cutover only, check the local parser's 64-lowercase-hex-digit behavior.
2. Before the step-4 cutover only, check the local `from_bytes`, `as_bytes`, `Display` and
   `from_hex` round trip. Retire both steps and their tests when the local type is deleted;
   subsequent identity fixtures consume admitted values rather than constructing digest bytes.

## Expected Results

RT has no local `NodeKey` type, alias, constructor, hex decoder or public re-export; its library
source contains no `NodeKey` construction. The retired parser and round-trip tests are absent.
The admitted-identity controls retain their foreign and cross-domain behavior. The remaining
refusal vocabulary spellings are byte-exact. QSV owns the complete structural-cause
reachability and normalized unit ordering obligations; RT has no second unit graph.
The R-3 step is implemented: RT has no local definition refusal vocabulary, its tests import QSV
directly, and the three retired causes cannot be paired with `invalid_package`. The
`quire-integration` producer-origin test obtains the same admitted identities QSL supplies and
does not require RT to depend on QSL or IR.
