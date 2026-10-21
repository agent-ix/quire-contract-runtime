---
id: SR-14113
title: "IR-641 runtime consumer spec gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; IR-641; spec/exact/functional/FR-275-single-exact-kernel.md; Makefile; tests/deny_one_copy.rs; full matrix"
review_set: subset
---

## Summary

Full computed repository matrix contains 176 criteria: 63 tagged, 72 untagged, 41 method-without-symbol. Four new Test criteria are honestly planned IR641 implementation work; 68 inherited untagged criteria require IR878 triage. Method-without-symbol is not automatically a defect. No repository completion claim.

## Verdict

FAIL. Spec-only candidate reviewed; no final consumer implementation gates claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | 68 inherited criteria have no computed test binding; existing runtime strict-coverage debt needs per-criterion owner triage | spec/core/functional/interface-001-runtime-api.md |
| FND-002 | high | Four newly planned consumer Test criteria have no implementation or actual test binding yet | spec/exact/functional/FR-275-single-exact-kernel.md |

## Coverage

Plan completion: not assessed

Candidate 3da82cf8197064a01232fb0a4cb343b79c5ab853. No applicable AssuranceProfile found in spec. Examined AC22–28 and planned status/consumer verification sections; AC1–21 context only. Source inventory: src/, verification/, tests/, Makefile and scripts have no candidate changes. Existing AWK controls in tests/deny_one_copy.rs deliberately need future adaptation, including replacing fragmentless sources with valid full resolved fragments. No new orphan production surface or stub is introduced. Existing inherited runtime end-state residue remains explicitly unauthorized in FR275; no exception is granted. Full matrix saved independently; method-without-symbol inspection/analysis obligations are not recast as failed tests. Optional semantic full-repository review and generated property tests not run: this is a spec review, not a property-test implementation task. Cargo, rustfmt, Clippy, native and backend execution prohibited for this source-only review and not claimed.

## Examined Units

```yaml
scope:
- id: FR-275-AC-1
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: With `exact` enabled, no public item the runtime defines (in `scalar` or
    any other module) has the name of an item that `quire-exact` or `quire-semantic-value`
    exports.
- id: FR-275-AC-2
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: No module, alias or feature of the runtime keeps a removed kernel definition
    reachable; after the IR-882 value-bearing cutover, inspection finds no RT-defined
    second `Value`, `CheckedEquality` evaluator, pair walker or membership walker,
    `UnitId` resolver, `EnumShape` registry, `EffectiveId` registry, copy of `UnitTable`,
    `TypeEnvironment`, `EnumMemberIndex` or `EnumDeclaration`, conversion layer, adapter
    or old-`Value` re-export. Quantity, enum, reference and checked-equality paths
    use the QSV owners directly.
- id: FR-275-AC-3
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`quire-exact` is an optional dependency of the runtime, enabled only by
    the `exact` feature.'
- id: FR-275-AC-4
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The optional `quire-exact` and `quire-semantic-value` dependencies use
    their own `agent-ix` Git repositories with `branch = "main"` and no `rev`, `tag`,
    `path` or committed `[patch]`.
- id: FR-275-AC-5
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`,
    `quire-canonical` and `quire-canonical-derive`.'
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make deny` exits non-zero when `Cargo.lock` holds a second entry for
    any first-party Git crate.'
- id: FR-275-AC-7
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`deny.toml` admits only the `quire-exact`, `quire-semantic-value` and
    `quire-canonical` own-repository Git sources and the dev-only `ix-trace-rs` first-party
    tooling source, and keeps `unknown-git = "deny"`; it does not admit the QSL Git
    repository.'
- id: FR-275-AC-8
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make deny` reports cargo-deny''s `source-not-allowed` error for the QSL
    Git repository, and exits non-zero, when a normal, build or dev dependency resolves
    from it; a licence failure alone does not satisfy this.'
- id: FR-275-AC-9
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: With `exact` enabled and `std` disabled, the runtime builds for `thumbv7em-none-eabi`.
    The row `build-exact-no-std-msrv` builds it on the 1.98.1 floor.
- id: FR-275-AC-10
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The dependency graph of `quire-contract-runtime-footprint` for `thumbv7em-none-eabi`
    contains no `quire-exact`, `quire-semantic-value`, `quire-canonical` or `quire-canonical-derive`.
- id: FR-275-AC-11
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make size` measures linked `.text` plus `.rodata` inside NFR-001-AC-3''s
    500 byte to 4 KiB band.'
- id: FR-275-AC-12
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The runtime has no test that compares its output with a second implementation
    of the kernel or that depends on a crate from the QSL Git repository.
- id: FR-275-AC-13
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: 'The runtime contains no file copied from the QSL repository and no port
    of QSL code, where a port is code that keeps the QSL authority''s item names and
    order. The residue list is not an allowance: every item on it is a violation of
    this criterion, which stays unmet while the list is non-empty. The runtime-owned
    negotiators are not ports.'
- id: FR-275-AC-14
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The kernel move deletes no requirement, acceptance criterion or test case.
- id: FR-275-AC-15
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: Every matrix row whose evidence leaves the runtime in the kernel move carries
    a planned status with a stated reason, because the status vocabulary has no "verified
    upstream" status (a planner question, no status is invented).
- id: FR-275-AC-16
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The runtime has no `quire_contract_runtime::exact` module and no `pub use`
    of `quire-exact`, `quire-semantic-value` or the evaluation leaf crate; the only
    items it defines under the `exact` feature are the `scalar` items (the backend
    negotiators and their types, `evaluate_boolean_short_circuit` and `ShortCircuitConnective`),
    and no file under `src/exact` exists.
- id: FR-275-AC-17
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: Every item the runtime defines under the `exact` feature, other than the
    `scalar` items, is named in the residue list, and the list records no exception,
    expiry or approval.
- id: FR-275-AC-18
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The residue list is empty.
- id: FR-275-AC-19
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The runtime defines `negotiate_integer_division`, `negotiate_ieee` and
    their types in its own source and depends on no QSL crate for them.
- id: FR-275-AC-20
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: The runtime's declared `rust-version` is 1.98.1.
- id: FR-275-AC-21
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: context_only
  excerpt: '`make msrv` and `make size` build with Rust 1.98.1.'
- id: FR-275-AC-22
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Inspection of runtime source finds no local definition or alias of QSV's
    `SemanticGraphCause`, `InvalidSemanticGraph`, or semantic-node term checker, including
    definitions emitted by macros.
- id: FR-275-AC-23
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: 'After the step-4 value-bearing cutover, QSV''s `Conversion::identified_quantity`
    returns a `quire_exact::Quantity` with the exact magnitude and the target''s compound-arm
    `UnitId`, paired with QSV''s `IdentifiedUnit` for that same target; a non-exact
    conversion returns `HandoffRefusal::NotExact`, and target identity refusal returns
    `HandoffRefusal::Identity` with neither value nor identified unit. Retaining the
    identified unit with `UnitScope::form` permits `UnitScope::resolve(&quantity)`
    to supply the same normalized `QuantityUnit::Compound` to the next conversion.
    Declared and compound ID arms '
- id: FR-275-AC-24
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock`
    command on this repository's committed lock. On a coherent scratch lock the real
    command succeeds and the recipe completes when its other policy checks pass; an
    invocation record identifies the executable and argument without substituting
    checker logic.
- id: FR-275-AC-25
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Through the same normal `make deny` recipe, a scratch lock with duplicate
    first-party Git package names and a scratch lock with distinct crate names at
    split resolved revisions of one repository each cause a non-zero Make exit and
    the real command's coherence diagnostic. The aligned multi-crate control passes
    the command. Every agent-ix Git source in these coherence controls has a valid
    full resolved fragment admitted by CLI FR-027; a malformed-input diagnostic cannot
    satisfy a duplicate or split-revision control. These controls exercise CLI FR-027;
    they do not define a second normalization al
- id: FR-275-AC-26
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: A missing checker executable, malformed scratch lock, or checker refusal
    cannot produce successful `make deny`. The recipe propagates failure without an
    ignored exit, success fallback, or skipped check.
- id: FR-275-AC-27
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: The committed `scripts/check_one_copy.awk` file is absent, the normal recipe
    contains no reference to it, and no replacement local implementation, executable
    copy, compatibility shim, or revision allowance bypasses the owned command.
- id: FR-275-AC-28
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
  excerpt: Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged
    and adds no runtime dependency on developer tooling. Qualification runs the real
    command on the committed lock and reports any existing incoherence as a failing
    gate with an owning blocker; a scratch passing control or an unrelated gate failure
    cannot qualify that lock.
```

## Dispositions

Round 1, unchanged candidate 3da82cf8197064a01232fb0a4cb343b79c5ab853. Spec soundness remains PASS; full repository gap verdict remains FAIL. Original findings above remain unchanged.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | 68 inherited untagged criteria remain open strict-coverage debt with existing IR-878 owner luna:IR:plan, actual 68-row handoff comment 6a87b61b; root spec-stage scope decision IR-641 comment 7817ca97. No coverage waiver, closure or green matrix. |
| FND-002 | deferred | Four honestly PLANNED new Test criteria FR-275-AC-24/25/26/28 remain required implementation work in the same IR-641 after this contract merges; root decision IR-641 comment 7817ca97. Real installed command and normal consumer Make controls still required. No implementation completion or test credit. |

## Narrow Verification (round 2)

Narrow verification of final custody head58758d8e: candidate differs from reviewed3da82cf8 only in14review-custody files; original spec, source, Makefile and locks are byte-identical. Real strict coverage baseline107 -> candidate112 unbacked rows; zero removed identities and zero contradicted statuses. Exact additions are FR-275-AC-24/25/26/27/28. AC27 is legitimately Inspection/no-symbol; this strict metric differs from the computed matrix72untagged/41method-without-symbol and creates no new missing-test finding. All107 inherited strict rows handed to IR-878 commentfad2588d. New five planned obligations remain IR-641 implementation; no waiver, qualification or green claim. Full gap FAIL and scoped spec PASS remain unchanged.


```yaml
baseline: a60c508f3a2af230aeb51af97bdf549cd5fffa01
candidate: 58758d8eba8685c58e8b715cacb5a1db3c8fdb19
identity_fields:
- reference
- document
- row_id
- target_ids
baseline_unbacked: 107
candidate_unbacked: 112
added:
- reference: verification
  document: spec/exact/functional/FR-275-single-exact-kernel.md
  row_id: FR-275-AC-24
  target_ids:
  - FR-275-AC-24
- reference: verification
  document: spec/exact/functional/FR-275-single-exact-kernel.md
  row_id: FR-275-AC-25
  target_ids:
  - FR-275-AC-25
- reference: verification
  document: spec/exact/functional/FR-275-single-exact-kernel.md
  row_id: FR-275-AC-26
  target_ids:
  - FR-275-AC-26
- reference: verification
  document: spec/exact/functional/FR-275-single-exact-kernel.md
  row_id: FR-275-AC-27
  target_ids:
  - FR-275-AC-27
- reference: verification
  document: spec/exact/functional/FR-275-single-exact-kernel.md
  row_id: FR-275-AC-28
  target_ids:
  - FR-275-AC-28
removed: []
status_lies_baseline: 0
status_lies_candidate: 0
baseline_no_symbol: 22
candidate_no_symbol: 23
inherited_owner: IR-878
new_owner: IR-641
inspection_explanation: AC27 is a no-symbol Inspection obligation, included in strict
  coverage unbacked rows but not an additional missing compiled-test defect.
inherited_owner_comment: IR-878 fad2588d
scope: Spec soundness PASS; full gap FAIL unchanged. Round1 deferrals remain current;
  no new findings, no fixed claims.
```
