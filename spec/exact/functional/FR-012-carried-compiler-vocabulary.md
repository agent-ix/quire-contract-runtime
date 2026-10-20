---
id: FR-012
title: "Carry the compiler's semantic vocabulary without re-deciding it"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-141
    type: implements
  - target: ix://agent-ix/quire-specification/FR-142
    type: implements
---
# FR-012: Carry the compiler's semantic vocabulary without re-deciding it

## Description

When a generated oracle must report a decision the compiler already made — a node identity, a
definition-selection refusal, a package refusal, or the topology of a unit graph — the runtime shall
carry the compiler's exact spellings so the oracle reports what it was handed, and shall not
recompute, re-derive or invent any of them.

`src/exact/mod.rs` exports the remaining runtime vocabulary; `NodeKey` and
`NODE_KEY_DOMAIN` are consumed directly from `quire_exact`. Until this requirement
it was carried by no requirement and asserted by no acceptance criterion: roughly sixty public items
across `node.rs`, `definition.rs` and `unit.rs`, none of which `grep` finds anywhere in `spec/`.

## Inputs

- Compiler-admitted 32-byte `quire.checked-semantic-node/v1` node keys carried in shared values.
- Normative refusal spellings read from a definition lock.
- Compiler-admitted dimension nodes with their `(base dimension, exponent)` terms, and unit
  declarations with their dimension, optional target, scale and offset.

## Outputs

- The runtime-owned `COMPOUND_UNIT_DOMAIN` evaluator domain string.
- `SelectionRefusalCode`, `PackageRefusalCode`, `PackageCause` and `PackageRefusal` values with
  their normative `as_str` spellings.
- An admitted `UnitGraph`, `Unit`, `Dimension` and `CompoundUnit`, each unit's admitted `UnitEdge`
  path to its canonical root, or an `InvalidSemanticGraph` carrying one typed `SemanticGraphCause`.
- An `InvalidCompoundUnit` carrying one typed `CompoundUnitCause`, where a compound unit is refused
  at construction.

## Behavior

- **The runtime never computes or constructs a node key in library code.** It carries admitted
  `quire_exact::NodeKey` values in the current runtime values. The local `NodeKey::from_bytes` and
  `NodeKey::from_hex` constructors were deleted with the local definition in the IR-882 key-owner
  slice of step 4; hex decoding has no RT owner. Test-only fixtures may mint keys with the shared
  `NodeKey::from_digest([u8; 32])`, using a key from an admitted package when lookup must succeed.
  `NODE_KEY_DOMAIN` is `quire.checked-semantic-node/v1` and `COMPOUND_UNIT_DOMAIN` is
  `quire.value.compound-unit/v1`; both are carried verbatim and neither is constructed at runtime.
- **The refusal vocabularies are closed and byte-exact.** `SelectionRefusalCode::ALL` lists the
  lock's eight `selection_refusal_codes` in normative check order, `as_str` yields the lock's exact
  spelling for each, and `from_code` is its exact inverse: it resolves every listed spelling and
  resolves nothing else. `PackageRefusalCode::as_str` is `invalid_package`. A generated oracle
  reports a compiler refusal by carrying one of these; it never spells a code itself.
- **`SemanticGraphCause` is one closed vocabulary shared with the compiler, split by who raises
  each cause.** Three groups, and the split is a fact about this runtime's API shape, not a
  preference:
  - *Raised by the runtime over a graph it is handed*: zero exponent, duplicate term, unsorted
    terms, zero scale, non-identity root, duplicate node, unknown dimension, non-base dimension
    term, unknown target, cross-dimension target, missing root, duplicate root, target cycle — and
    undeclared case, raised where a case is read against its declaration rather than at graph
    admission.
  - *Raised by the runtime at its own declaration-admission point*: `EnumDeclaration::new` admits a
    declaration node from caller-supplied member strings, and that is a runtime admission, so it
    raises `NonCanonicalPreimage` for an empty, repeated or non-identifier member list and
    `UnsortedUnorderedMembers` for an unordered declaration whose cases are not sorted. Those two
    causes are raised there and nowhere else; no graph handed to the runtime raises either.
  - *Carried only, never raised here*: `OwnerNotSelected` and `StaleKey`, which only a compiler that
    computes keys and resolves a lock selection can decide; and `ForeignDeclaration` and
    `UnreducedRational`, which this API cannot present — a member node never carries a declaration
    key of its own for the runtime to compare, and `Rational`'s only public constructor reduces, so
    no unreduced scale or offset can reach `UnitDeclaration::check_semantics`. The runtime carries
    all four so a generated oracle can report a compiler refusal it was handed.
- **Every unit-graph refusal is `invalid_semantic_graph` with one typed cause naming the first
  failed check**, in the order fixed by FR-011: per-node semantics, then duplicate keys, then graph
  topology. The runtime admits a graph or refuses it; it never repairs one and never admits a graph
  with a repaired node.
- **A `UnitEdge` is one exact affine edge `target = scale × source + offset`.** `UnitDeclaration`
  admits it, as `check_semantics`'s `Ok` result, after refusing a zero scale or a non-identity
  root; each `Unit`'s path to its canonical root and its own canonical edge are `UnitEdge` values,
  carried unchanged.
- **A `CompoundUnit` construction that fails is refused as `InvalidCompoundUnit`, carrying one
  typed `CompoundUnitCause`.** The five causes are `NonCanonicalPreimage` (compiler-reader
  preimage only), `ZeroExponent`, `DuplicateTerm`, `UnsortedTerms` and `NotRootUnit`.
- **`Dimension` and `CompoundUnit` are normalized value identities.** Exponents are never zero, the
  empty map is the sole dimensionless value, terms are ascending by node key, and structural
  equality is value equality — two compound units are equal exactly when they denote the same unit.
- The whole vocabulary is inert: carrying it charges nothing, and no item in it reads or writes a
  `Meter`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-012-AC-1 | `NodeKey::from_hex` accepts exactly the 64-lowercase-hex-digit inputs and returns `None` for every wrong length, uppercase digit and non-hexadecimal byte; `from_bytes`/`as_bytes`/`from_hex`/`Display` round-trip every digest unchanged. Retired at IR-882 step 4; this ID remains reserved with its original criterion text. | Test (TC-033; pre-cutover only) |
| FR-012-AC-2 | `NODE_KEY_DOMAIN` and `COMPOUND_UNIT_DOMAIN` equal their normative strings, and no code path constructs either from parts. | Test (TC-033) |
| FR-012-AC-3 | `SelectionRefusalCode::ALL` is the eight codes in normative check order, `as_str` yields the lock spelling for each, `from_code` resolves exactly those eight spellings and nothing else, and `PackageRefusalCode::as_str` is `invalid_package`. | Test (TC-033) |
| FR-012-AC-4 | Each of the thirteen graph causes is raised by an admissible input to `UnitGraph::admit`, and `UndeclaredCase` by reading a case against its declaration; no graph input raises any of the four compiler-admission causes; `EnumDeclaration::new` raises `NonCanonicalPreimage` and `UnsortedUnorderedMembers` for exactly the malformed member lists named above; and `OwnerNotSelected`, `StaleKey`, `ForeignDeclaration` and `UnreducedRational` have no raise site in the crate. | Test (TC-033) |
| FR-012-AC-5 | `Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending by node key, and compare equal exactly when they denote the same unit, for every construction order of the same terms. | Test (TC-033) |
| FR-012-AC-6 | At the IR-882 step-4 cutover, inspection finds no RT-local `NodeKey`, no RT `from_bytes` or `from_hex` constructor, and no TC-033 parser or round-trip test of those local constructors. RT library code has no `NodeKey` construction or hex decoder. | Inspection |

## Kernel ownership

The behaviour above that `quire-exact` implements (interface-001, "Exact kernel surface") is the
behaviour of that one kernel, which the runtime consumes and does not copy ([FR-275](./FR-275-single-exact-kernel.md)).
This requirement binds the `exact` feature as a whole except for the local-constructor
contract in FR-012-AC-1, retired in the IR-882 key-owner slice of step 4. Its original text and ID remain in the record;
FR-012-AC-6 inspects the cutover. Where other evidence
moves to the kernel's repository, the matrix keeps its row and states the status.

## Dependencies

- **Upstream**: [FR-006](./FR-006-exact-outcomes-and-accounting.md), [FR-011](./FR-011-meter-state-at-a-stop.md);
  `ix://agent-ix/quire-specification`.
