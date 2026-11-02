---
id: SR-20261102072234
title: "IR-722 lexical quire-contract-runtime gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; IR-722; frozen lexical amendment; interface_004, IT-018-SC-06, monitor source and companion tests"
review_set: subset
---

## Summary

The computed matrix over the exact frozen RT snapshot leaves all four interface-004 criteria locally untagged. Baseline FR-276 acceptance remains open. No new source stub or behavior outside the explicit lexical interface was found. Optional standalone semantic gap audit was not invoked.

Frozen lexical amendment of the original IR-722 PRs: RT193 2ec49a0c372a418b67d78388a098ce6623fde2c6 over 14b1767, integration35 742bff7417163bcf19d93398acabf1ec0d088520 over 31c384a. Superseded PR195/36 history is context only. All previous SR artifacts are retained untouched by this pass. Read actual frozen Git snapshots, applicable repository guidance, code-review/Rust style and selected scoped spec methods. No source edits, Cargo, proof/heavy jobs, merge/closure or permission actions performed. No applicable AssuranceProfile found.

Public shape changes are LocalId, Operation::Local/Let and typed InvalidBinding/Binding failure variants. ProgramParts/Layout/Function/Expression fields remain unchanged. Bindings::prepare derives a compact private address index in linear construction work; unique in-range positions among N declarations imply dense positions without host-size semantic caps. Validation propagates function ownership and rejects mixed/foreign binding closures. Local type facts derive from the retained initializer annotation; shared bounded admission handles a different Local result annotation and Let result edges. No mirrored Value/source-key type, synthetic function, callback, parser or new plan wire appears.

VM dataflow was traced: bound operands are canonical literal/supplied addresses; invocation tokens derive from already charged monotonic work, separating recursive invocations. Restore runs after the scoped body and before Let conformance, restoring caller activation and decrementing the active count. The active count includes shadowed operands saved in Restore frames and is included in slot/native reservation accounting. Touched flags cap cleanup to actually activated unique slots; both normal and stopped run results execute cleanup. These source traces are evidence of implementation structure, not substitutes for the missing AC-4 controls.

Supplied evidence: seven lexical controls and all twenty-nine monitor_program tests pass, and touched integration Clippy passes ONLY with the preserved interim old-leaf lock and current RT source. RT all-target/all-feature Clippy passes on Exact a3052598 / QSV 25535387. The committed new integration cohort remains blocked by counterpart driver adoption/CG707 integration; the requested final IR long chain has not run. Root IEEE owner evidence is source/compile-only, not CBMC replay. Prior coverage/cross-repository acceptance and final aggregate remain unresolved. Ordinary constructor/arithmetic/sequence/fold operations and event/watermark/gap execution remain unfinished outside this scoped lexical claim; no full FR-276 or temporal producer qualification is granted.

Plan completion: not assessed

## Verdict

**FAIL** — Findings require an explicit fix or scope disposition; committed-cohort/aggregate blockers remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The frozen RT computed matrix reports this new lexical criterion untagged locally. The companion integration carries an external criterion tag, but accepted cross-repository discharge has not been established. Preserve authentic external evidence and obtain the explicit acceptance/routing decision; do not manufacture a local binder. | spec/monitor/functional/interface-004-monitor-lexical-bindings.md:96-99 |
| FND-002 | medium | The frozen RT computed matrix reports this new lexical criterion untagged locally. The companion integration carries an external criterion tag, but accepted cross-repository discharge has not been established. Preserve authentic external evidence and obtain the explicit acceptance/routing decision; do not manufacture a local binder. | spec/monitor/functional/interface-004-monitor-lexical-bindings.md:96-99 |
| FND-003 | medium | The frozen RT computed matrix reports this new lexical criterion untagged locally. The companion integration carries an external criterion tag, but accepted cross-repository discharge has not been established. Preserve authentic external evidence and obtain the explicit acceptance/routing decision; do not manufacture a local binder. | spec/monitor/functional/interface-004-monitor-lexical-bindings.md:96-99 |
| FND-004 | medium | The frozen RT computed matrix reports this new lexical criterion untagged locally. The companion integration carries an external criterion tag, but accepted cross-repository discharge has not been established. Preserve authentic external evidence and obtain the explicit acceptance/routing decision; do not manufacture a local binder. | spec/monitor/functional/interface-004-monitor-lexical-bindings.md:96-99 |

## Examined scope

```yaml
scope:
- id: interface-004-AC-1
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: examined
  excerpt: Missing, duplicate, sparse and foreign local declarations refuse construction,
    while compact function-qualified bindings retain their original source locations.
- id: interface-004-AC-2
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: examined
  excerpt: A selected Let evaluates its value once and returns its scoped body's result;
    a lazy unselected binding is not activated. A differently annotated local uses
    shared admission and preserves its native failure.
- id: interface-004-AC-3
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: examined
  excerpt: Nested calls and recursive invocations cannot read an outer invocation's
    local as their own. A bind restores the prior invocation's slot on return, and
    an inactive read is a typed invariant failure.
- id: interface-004-AC-4
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: examined
  excerpt: Binding operands respect the scratch ceiling, including a shadowed outer
    operand. A stopped invocation clears only touched slots and cannot leak a binding
    into a subsequent invocation; deep binding chains use the explicit stack.
- id: src/monitor/program.rs
  path: src/monitor/program.rs
  role: examined
  excerpt: //! The single retained executable representation, independent of compiler
    storage.
- id: src/monitor/bindings.rs
  path: src/monitor/bindings.rs
  role: examined
  excerpt: //! Compact indexes over the one retained lexical expression arena.
- id: src/monitor/validation.rs
  path: src/monitor/validation.rs
  role: examined
  excerpt: //! Executable integrity checks; source-language admission remains upstream.
- id: src/monitor/vm.rs
  path: src/monitor/vm.rs
  role: examined
  excerpt: //! Iterative execution of ordinary retained expressions, with no source
    evaluator.
- id: src/monitor/executor.rs
  path: src/monitor/executor.rs
  role: examined
  excerpt: //! Monitor ownership and initializer execution. Entry dispatch shares
    the same machine.
- id: src/monitor/mod.rs
  path: src/monitor/mod.rs
  role: examined
  excerpt: //! Owned executable monitor programs (FR-276).
- id: interface_001
  path: spec/core/functional/interface-001-runtime-api.md
  role: examined
  excerpt: Ordinary function-qualified lexical slots extend that same arena through
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | Authentic integration criteria tags remain external to RT. Accepted cross-repository discharge and committed-cohort qualification remain pending on open IR-722; no local fake binder or coverage exception is granted. |
| FND-002 | deferred | Authentic integration criteria tags remain external to RT. Accepted cross-repository discharge and committed-cohort qualification remain pending on open IR-722; no local fake binder or coverage exception is granted. |
| FND-003 | deferred | Authentic integration criteria tags remain external to RT. Accepted cross-repository discharge and committed-cohort qualification remain pending on open IR-722; no local fake binder or coverage exception is granted. |
| FND-004 | deferred | Authentic integration criteria tags remain external to RT. Accepted cross-repository discharge and committed-cohort qualification remain pending on open IR-722; no local fake binder or coverage exception is granted. |

Lexical disposition carry inspected current RT af4c3d8effd378cac9cd953b01404aee6f118981 and integration 7fcff282ce80a05527bcd4b3493837a61f74f12d. The new public lexical_recursive_shadow_counts_the_retained_caller_operand control distinguishes a four-slot refusal from a five-slot success and checks restored outer now=99; it closes the caller-shadow accounting portion of the original finding. It builds a fresh Machine for each attempt and therefore does not establish reuse of one stopped Machine. That remainder stays on the existing IR-722 entry-dispatch work; no API widening, acceptance narrowing or full AC-4 credit is granted.

The interface-004 preparation prose is corrected from visits-once to linear preparation, matching its fixed passes. No new lexical runtime defect found. Evidence remains interim-cohort only for integration; current RT Clippy passes on new leaves, but committed integration cohort, our final dependency-head long chain, QSL1463/2106 producer qualification, coverage acceptance and IEEE replay remain unresolved. No Cargo/heavy jobs or source edits by reviewer.

Current verdict remains **FAIL** for external/local criterion acceptance.

```yaml
dispositions:
- fnd: FND-001
  outcome: deferred
  reason: Authentic integration criteria tags remain external to RT. Accepted cross-repository
    discharge and committed-cohort qualification remain pending on open IR-722; no
    local fake binder or coverage exception is granted.
- fnd: FND-002
  outcome: deferred
  reason: Authentic integration criteria tags remain external to RT. Accepted cross-repository
    discharge and committed-cohort qualification remain pending on open IR-722; no
    local fake binder or coverage exception is granted.
- fnd: FND-003
  outcome: deferred
  reason: Authentic integration criteria tags remain external to RT. Accepted cross-repository
    discharge and committed-cohort qualification remain pending on open IR-722; no
    local fake binder or coverage exception is granted.
- fnd: FND-004
  outcome: deferred
  reason: Authentic integration criteria tags remain external to RT. Accepted cross-repository
    discharge and committed-cohort qualification remain pending on open IR-722; no
    local fake binder or coverage exception is granted.
```
