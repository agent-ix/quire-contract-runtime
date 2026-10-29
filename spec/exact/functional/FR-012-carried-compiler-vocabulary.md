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
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---
# FR-012: Carry the compiler's semantic vocabulary without re-deciding it

## Description

When a generated oracle must report a decision the compiler already made — a node identity, a
definition-selection refusal, a package refusal, or the topology of a unit graph — the runtime shall
carry the compiler's exact spellings so the oracle reports what it was handed, and shall not
recompute, re-derive or invent any of them.

`src/exact/mod.rs` no longer exports a runtime unit vocabulary; the quantity and unit cutover
consumes QSV's checked graph and units directly. The R-3 definition cutover consumes
`quire_semantic_value::definition` directly.
`NodeKey` values in the current shared interface are consumed directly from `quire_exact`;
`InvalidSemanticGraph` and `SemanticGraphCause` are consumed directly from
`quire_semantic_value::semantic_node`.

## Inputs

- Compiler-admitted semantic identities carried in shared values.
- Normative refusal spellings read from a definition lock.
- Compiler-admitted dimension nodes with their `(base dimension, exponent)` terms, and unit
  declarations with their dimension, optional target, scale and offset.

## Outputs

- Admitted declaration and unit identities, consumed without an RT digest-domain copy.
- QSV's definition vocabulary supplies `SelectionRefusalCode`, `PackageRefusalCode`,
  `PackageCause` and `PackageRefusal` with their normative spellings.
- A QSV-admitted `UnitGraph`, `Unit`, `Dimension` and `CompoundUnit`, each unit's admitted `UnitEdge`
  path to its canonical root, or a QSV `InvalidSemanticGraph` carrying one typed
  `SemanticGraphCause`.
- An `InvalidCompoundUnit` carrying one typed `CompoundUnitCause`, where a compound unit is refused
  at construction.

## Behavior

- **The runtime never computes or constructs a semantic identity in library code.** It carries
  identities from the checked package through the current shared types. RT's local `NodeKey`
  constructors and hex decoder were deleted in the IR-882 key-owner cutover. Tests that need
  declaration, reference-universe or unit identity use admitted values from their producer; they
  do not mint identity bytes or assert a fixed digest or digest-domain string. RT neither owns a
  digest-domain constant nor reconstructs a shared identity from constituent bytes.
- **The refusal vocabularies are closed and byte-exact.** `SelectionRefusalCode::ALL` lists the
  lock's eight `selection_refusal_codes` in normative check order, `as_str` yields the lock's exact
  spelling for each, and `from_code` is its exact inverse: it resolves every listed spelling and
  resolves nothing else. `PackageRefusalCode::as_str` is `invalid_package`. A generated oracle
  reports a compiler refusal by carrying one of these; it never spells a code itself.
- **R-3 retires three RT-only package causes.** RT's former local `PackageCause` included
  `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` without a producer,
  consumer or test outside the local definition. Those causes are not members of the
  `invalid_package` definition-admission subset: QSpec FR-272 assigns digest mismatches to
  `stale_dependency`, while QSV's definition vocabulary carries only `MissingMember`,
  `ConflictingDefinition` and `IncompatibleDefinition`. The R-3 cutover deleted RT's local
  `PackageCause`, `PackageRefusal` and `PackageRefusalCode` definitions, imports QSV's three-cause
  vocabulary at use sites, and adds no alias, re-export or compatibility adapter. QSV owns the
  valid code/cause pairing and its tests; RT keeps the existing selection and package-code
  spelling obligations in FR-012-AC-3.
- **`SemanticGraphCause` is one closed vocabulary shared with the compiler, split by who raises
  each cause.** QSV owns the graph and enum checks; RT consumes their typed results directly:
  - *Raised by QSV's checked-node and graph boundaries*: zero exponent, duplicate term, unsorted
    terms, zero scale, non-identity root, unknown dimension, non-base dimension term, unknown
    target, cross-dimension target, missing root, duplicate root and target cycle. `DuplicateNode`
    is carried in the shared vocabulary but is unrepresentable at QSV's keyed-map graph boundary.
    QSV raises `UndeclaredCase` when `EnumDeclaration::member` reads a case outside its admitted
    declaration; RT tests consume that owner path directly.
  - *Raised by the QSV declaration-admission point consumed by the runtime*: `EnumDeclaration::new` admits a
    declaration node from caller-supplied member strings, so it
    raises `NonCanonicalPreimage` for an empty, repeated or non-identifier member list and
    `UnsortedUnorderedMembers` for an unordered declaration whose cases are not sorted. Those two
    causes are raised there and nowhere else; no QSV graph input raises either.
  - *Carried only, never raised here*: `OwnerNotSelected` and `StaleKey`, which only a compiler that
    computes keys and resolves a lock selection can decide; and `ForeignDeclaration` and
    `UnreducedRational`, which this API cannot present — a member node never carries a declaration
    key of its own for the runtime to compare, and `Rational`'s only public constructor reduces, so
    no unreduced scale or offset can reach `UnitNode::checked`. The runtime carries
    all four so a generated oracle can report a compiler refusal it was handed.
- **Every QSV unit-graph refusal is `invalid_semantic_graph` with one typed cause naming the first
  failed check**, after per-node checks by `DimensionNode::checked` and `UnitNode::checked` and the
  topology order of `UnitGraph::from_checked_nodes`. The keyed maps admit no duplicate key.
  RT consumes the result without repairing or rebuilding the graph.
- **A `UnitEdge` is one exact affine edge `target = scale × source + offset`.** QSV's
  `UnitNode::checked` admits it after refusing a zero scale or a non-identity root; each `Unit`'s
  path to its canonical root and its own canonical edge are `UnitEdge` values, carried unchanged.
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
| FR-012-AC-2 | RT library code and identity fixtures construct no semantic identity from bytes or constituent parts, retain no digest-domain constant or fixed digest oracle, and consume admitted declaration, reference-universe and unit identities without reminting them. Distinct declaration, foreign-universe and declared-versus-compound unit identities remain distinguishable and retain their existing inequality/refusal behavior. | Test (TC-033) |
| FR-012-AC-3 | `SelectionRefusalCode::ALL` is the eight codes in normative check order, `as_str` yields the lock spelling for each, `from_code` resolves exactly those eight spellings and nothing else, and `PackageRefusalCode::as_str` is `invalid_package`. | Test (TC-033) |
| FR-012-AC-4 | Each of the twelve graph causes reachable through QSV's `DimensionNode::checked`, `UnitNode::checked` or `UnitGraph::from_checked_nodes` is raised by a malformed input; `DuplicateNode` is unrepresentable because the graph accepts key-unique `BTreeMap`s, rather than a list with repeated keys. `UndeclaredCase` arises by reading a case against its declaration; no graph input raises any of the four compiler-admission causes. `EnumDeclaration::new` raises `NonCanonicalPreimage` and `UnsortedUnorderedMembers` for the malformed member lists named above; `OwnerNotSelected`, `StaleKey`, `ForeignDeclaration` and `UnreducedRational` have no RT raise site. | Test (TC-033) |
| FR-012-AC-5 | QSV's `Dimension` and `CompoundUnit` hold no zero exponent, iterate ascending by node key, and compare equal exactly when they denote the same unit, for every insertion order of the same admitted graph nodes; compound term input is ascending, and an unsorted term input is refused. | Test (TC-033) |
| FR-012-AC-6 | At the IR-882 step-4 cutover, inspection finds no RT-local `NodeKey`, no RT `from_bytes` or `from_hex` constructor, and no TC-033 parser or round-trip test of those local constructors. RT library code has no `NodeKey` construction or hex decoder. | Inspection |
| FR-012-AC-7 | After the R-3 cutover, inspection finds no RT-local `PackageCause`, `PackageRefusal` or `PackageRefusalCode` definition, type alias, public re-export or adapter. The RT consumers of selection and package refusals use QSV's definition types directly; `RevisionMismatch`, `DigestDomainMismatch` and `ByteDigestMismatch` are absent from RT's `invalid_package` surface, while FR-012-AC-3's eight selection spellings and `invalid_package` code remain available from QSV. | Inspection (TC-033) |

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
