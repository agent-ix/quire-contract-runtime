---
id: "SR-623"
title: "IR-319 design audit (code-review with rust-review lane): quire-contract-runtime"
type: SpecReview
scope: "agent-ix/quire-contract-runtime@75e5ed04d415f47a99bca2db994d1add888495e9; whole repository: src/ (lib.rs, accounting, identity, observation, operators, verdict, proptest_adapter, snapshot_json, exact/ 23 files), tests/, verification/kani.rs, measurement/footprint, scripts/, spec/, plan/, Cargo.toml, clippy.toml, deny.toml, .github/workflows/ci.yml; cross-checked against agent-ix/quire-spec-language@c9e63aa4 (quire-exact) and agent-ix/quire-contract-codegen origin/main (Cargo.toml, Cargo.lock)"
relationships: []
---

# SR-623: IR-319 design audit of quire-contract-runtime

## Summary

Ticket: IR-319. This is a whole-repository design audit, not a PR review. It applies the
/rust-review method to quire-contract-runtime (RT) at origin/main 75e5ed0.

An earlier audit is posted as a comment on IR-319, written at RT ed0a04b. This review treats it
as unverified claims. Each item was re-measured on 75e5ed0 and is classed as still-live,
fixed or wrong (table below). Items not in that list are new.

RT changed a lot between ed0a04b and 75e5ed0. RT #83 to #87 removed tracking ceremony. RT #88
(IR-430) deleted `conformance/qsl-agreement`. RT #89 and #90 changed tooling. No Rust source
under `src/exact` changed except one doc line in `mod.rs`.

## Method

- Read the repository's own conventions (CLAUDE.md, AGENTS.md, clippy.toml, deny.toml,
  rustfmt.toml, Cargo.toml lints) and treated them as authority.
- Read `src/exact/expression.rs`, `outcome.rs`, `accounting.rs` and `mod.rs` in full. Read
  `composite.rs`, `collection.rs`, `equality.rs`, `key.rs`, `integer.rs`, `division.rs`,
  `rational.rs`, `enumeration.rs` and `snapshot_json.rs` at the cited regions.
- Ran greps across the whole crate: panic macros, `unwrap`/`expect`, `#[allow]`, `as` casts,
  every `Refusal::CheckedInvariant` site, and module imports (to build the `exact/` dependency
  graph).
- Checked tests by script: 196 `#[test]`/`#[kani::proof]` functions; 94 distinct ids cited in
  `Trace:` tags, each resolved against `spec/`.
- Gates run at 75e5ed0, with CARGO_TARGET_DIR inside the worktree (deleted afterwards):
  - `cargo clippy -p quire-contract-runtime --all-targets --all-features --locked -- -D warnings`: exit 0.
  - `cargo test --all-features --locked`: exit 0, all suites ok, 0 failed, 0 ignored.
  - `cargo fmt --all -- --check`: exit 0.
  - `cargo build --locked --lib --no-default-features --features exact --target thumbv7em-none-eabi`
    on stable: exit 0.
  - `quire validate` (the `make spec` arguments): 2 documents failed structural validation.
    `quire coverage --strict`: "coverage could not evaluate its declared input:
    status-column-matches-nothing".
- QSL: ran `git fetch`, then read `quire-exact/Cargo.toml`, `quire-exact/src/lib.rs` and
  ADR-011 at origin/main c9e63aa4. CG: read `Cargo.toml` and `Cargo.lock` at origin/main.

### Prior-audit claims re-measured (IR-319 comment at ed0a04b)

| Prior item | Status at 75e5ed0 | Evidence |
| --- | --- | --- |
| H1 shared kernel not adopted | still-live | FND-002 |
| H1 conformance crate compares against superseded `src/value` | superseded (crate deleted by RT #88, IR-430). Worse now: RT has no agreement evidence at all | FND-001 |
| H1 blockers: MSRV 1.98 vs 1.75, std/Arc vs no_std/Rc, thiserror 2, no Kani gate, dropped cross-unit quantity and equality-conversion table | still-live, every one | FND-002 |
| Ticket text: "CG's lock resolves two quire-exact copies" | fixed: CG origin/main Cargo.lock has one `quire-exact` and one `quire-contract-model` | CG Cargo.lock |
| Ticket text: "exact/ (24 files)" | wrong: 23 files (22 modules plus mod.rs) | `ls src/exact` |
| H2 CheckedInvariant covers five failures | still-live; grown to at least ten conditions at 16 sites | FND-003 |
| M1 dependency cycles in exact/ | still-live, and two more cycles found | FND-005 |
| M2 Evaluation.location/losses always empty | still-live | FND-011 |
| M3 Meter allocates per charge, production log of 4096 entries | still-live | FND-006 |
| M4 linear function lookup, nested RefCell per call level | still-live | FND-012 |
| M5 one of eight Kani harnesses reaches exact | still-live | FND-007 |
| L dead_code ported ahead of callers | still-live | FND-017 |
| L tautological census test tc_185_exact_decimals.rs:21 | fixed-by RT #88 (file deleted); a similar census remains | FND-018 |
| L snapshot field list kept by hand in three places | still-live | FND-019 |
| L test files mounted by #[path] at src root | still-live | FND-020 |
| L usize in Origin/Location | still-live | FND-021 |
| L Integer invariant by convention (deliberate, for CBMC) | still-live, accepted; no finding row | integer.rs:20-33 |
| Clean: no unwrap/expect/panic!/todo! in src outside tests | confirmed | grep: 0 hits |
| Clean: forbid(unsafe_code), deny arithmetic_side_effects/indexing_slicing | confirmed | lib.rs:19-22 |
| Clean: every test tc_NNN-named with Trace tags | confirmed: 196/196 tc_-named and traced; all 94 cited ids resolve | script |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | RT's exact semantics are checked against no external authority, but the module doc still claims a shared-corpus test that does not exist. The doc says RT is a port of `quire_spec_language::value`, kept name-for-name so that a shared-corpus test compares the two by `Debug` rendering. That path is gone from QSL main, and RT #88 deleted the test. `quire coverage` reports FR-006-AC-2 and FR-007-AC-3/4/5 with no backing symbol. Scenario: a QSL kernel semantic change (e.g. QSL-245 sum out-of-domain, QSL-280 float rounding mode) lands only in quire-exact. CG's generated oracles call `rt::` and diverge from the QSL replay verdict, and nothing in RT turns red. | src/exact/mod.rs:6-11, spec/test-matrix.md:25, spec/test-matrix.md:30-32 |
| FND-002 | high | The shared kernel (AD-016 owner decision 2) is not adopted. RT keeps its own 23-file, 12,080-line kernel and has no quire-exact dependency. CG reaches quire-exact through qsl-replay and RT exact through `features = ["exact"]`, so two exact implementations sit in CG's graph. All the measured adoption blockers are still live: quire-exact needs rust-version 1.98 while RT declares 1.75 (NFR-001-AC-3 also pins 1.75); quire-exact uses `std::sync::Arc` and is not no_std; it uses thiserror =2.0.20; it has no Kani gate; it drops cross-unit quantity and the equality-conversion table, which RT exports (`convert_quantity`, `admits_equality_conversion`). Scenario: a semantic fix lands in one kernel only, and CG oracle verdicts and QSL replay verdicts disagree on the same input. | Cargo.toml:4, Cargo.toml:30-36, src/exact/mod.rs:59-61 |
| FND-003 | medium | `Refusal::CheckedInvariant` stands for at least ten distinct conditions at 16 sites: depth bound, an unknown function inside `Frame::call`, three Meter re-borrow sites, a foreign expression, an equality-schedule mismatch, a collection-kind mismatch, an element type not admitted, an unkeyed sort, and a deferred value not admitted. `evaluate` also throws away the typed `EvaluationRefusal::ForeignExpression`. Scenario: a CG host body that re-borrows the meter, and a package that hits the depth bound, give byte-identical transcripts. Tests can only assert `Refused(CheckedInvariant)`. | src/exact/outcome.rs:143, src/exact/expression.rs:796, src/exact/expression.rs:802, src/exact/expression.rs:830, src/exact/expression.rs:919-933, src/exact/expression.rs:959, src/exact/equality.rs:195, src/exact/equality.rs:311, src/exact/collection.rs:211, src/exact/collection.rs:384, src/exact/composite.rs:1409 |
| FND-004 | medium | In `exact`, panic-freedom depends on preconditions written in comments, and no gate measures it. `Integer::exact_div`, `div_rem_truncating` and `div_mod_floor` call num-bigint operations that panic on a zero divisor ("`divisor` is nonzero"). clippy.toml exempts BigInt from `arithmetic_side_effects`. The panic-relocation gate builds only the default-feature footprint crate, never `exact`. Scenario: a new caller passes a zero divisor (say, a quantity conversion edge). num-bigint panics, the release profile's `panic = "abort"` aborts the process, and no gate fails. | src/exact/integer.rs:228-243, clippy.toml:5, measurement/footprint/Cargo.toml:13, scripts/check_linked_footprint.sh:15 |
| FND-005 | medium | `exact/` has module dependency cycles: outcome<->collection, outcome<->ieee, collection<->composite, composite<->reference, collection->key->composite->collection, and collection->equality->composite->collection. composite.rs is 1585 lines, holds Value/ValueType and imports 13 siblings. Scenario: the IR-349 split into scalar/text/composite/expression/outcome cannot place `outcome` below `collection` without first moving `CardinalityBound`/`CollectionKind` out of collection.rs. | src/exact/outcome.rs:8-10, src/exact/collection.rs, src/exact/composite.rs:1-37, src/exact/key.rs, src/exact/reference.rs |
| FND-006 | medium | The production Meter allocates on every sized charge. `Charge.sizes` is a Vec pushed per size, and `charge` allocates a second Vec. The Meter also keeps a charge log of up to 4096 entries in production. The QSL kernel already moved that log behind its test-support feature (QSL-206), so RT has diverged from the kernel it is meant to adopt. `consumed()` hides a bad index behind `unwrap_or(0)`. Scenario: each metered arithmetic step costs two heap allocations and a log push, on the target and under CBMC (the IR-340 tractability cost). | src/exact/accounting.rs:431-456, src/exact/accounting.rs:478-492, src/exact/accounting.rs:520, src/exact/accounting.rs:597-612 |
| FND-007 | medium | Only one of eight Kani harnesses reaches `exact` (`compare_ieee`). It runs under limits that are all `u64::MAX`, so it proves no refusal or `Incomplete` path. The harness comment cites CG `src/exact_scalar.rs:1370` and `:1677`. On CG main those lines are elsewhere: the `rt` import is at :2344 and `compare_ieee` at :2649. Scenario: a regression in metering refusal, or in any non-IEEE exact operator, passes `make kani`. | verification/kani.rs:13-15, verification/kani.rs:33-37, verification/kani.rs:275-277 |
| FND-008 | medium | `make spec` fails on main. interface-001 is missing the required `id` and `features`. The test-matrix header says "Coverage Status" where the schema asserts "Status", so `quire coverage --strict` cannot evaluate. Scenario: `make ci` fails at its second step, so every later gate in `make ci` goes unrun as a whole. | spec/interface/interface-001-runtime-api.md, spec/test-matrix.md:9 |
| FND-009 | low | The matrix, the TC files and the plan cite IR-430 (Done) as tracking the recreation of the agreement oracle. The open owners are IR-355 and IR-20, which duplicate each other. IR-355's premise cites `conformance/qsl-agreement/Cargo.toml:14`, which RT #88 deleted. Scenario: a reader following the matrix lands on a closed ticket, and the coder picking up IR-355 finds no crate to repoint. | spec/test-matrix.md:25, spec/test-matrix.md:30-33, spec/test-matrix.md:50, spec/test-matrix.md:71-75, spec/test/TC-020-ieee-agreement.md:14, plan/PLAN-003-exact-scalar-oracles/plan.md:21 |
| FND-010 | medium | AD-001 covers only the default core ("no global state, I/O, heap"). `exact` is 88% of src lines (12,080 of 13,791). It allocates and holds a process-global `static NEXT_PACKAGE_ID: AtomicUsize`, and AD-002 covers only its call boundary; no other architecture description covers it. Scenario: the IR-349 refactor has no reviewed layout or invariants to refactor against. | spec/assurance/AD-001-runtime-architecture.md:14-25, src/exact/expression.rs:104 |
| FND-011 | medium | `Evaluation.location` and `Evaluation.losses` are public, but no producer can ever fill them: `call`/`evaluate` always build `None` and empty. Decimal rounding and IEEE losses inside a body are therefore dropped at the function-application boundary. Scenario: a body rounds a decimal under half-even and returns `Completed` with `losses == []`. A consumer that reports loss sees an exact result. | src/exact/expression.rs:504-521, src/exact/expression.rs:778-782, src/exact/expression.rs:829-841 |
| FND-012 | low | Every `Frame::call` looks the function up by linear string comparison and nests one more `RefCell<&mut Meter>` per call level. Scenario: a recursion 128 deep with a 50-function package makes up to 6,400 string compares, plus 128 nested RefCell layers for CBMC to track. | src/exact/expression.rs:690-695, src/exact/expression.rs:917-941 |
| FND-013 | low | `check_type` says it checks that "every named declaration exists", but `ValueType::Enum(key)` is never checked. TypeEnvironment has no enum registry. Scenario: a package whose parameter is `Enum(stale_key)` passes `check`, and every call is then refused `WrongValueKind` at runtime instead of at admission. | src/exact/composite.rs:939-942, src/exact/composite.rs:1018-1031, src/exact/composite.rs:1074-1111 |
| FND-014 | low | `sort_by_key` sorts with a comparator that returns `Equal` for unkeyed pairs, which is not a total order. Since Rust 1.81 the standard sort may panic when it detects such an order, before the intended `CheckedInvariant` refusal is returned. RT builds on stable. Scenario: an invariant-violating mixed-kind set aborts instead of refusing. It is unreachable for checked programs. | src/exact/collection.rs:375-386 |
| FND-015 | low | `NEXT_PACKAGE_ID` uses `AtomicUsize::fetch_add`, which needs atomic CAS. That contradicts the module contract "no `target_has_atomic` requirement". The id also wraps at 2^32 on 32-bit targets (documented). Scenario: building `--features exact` for a target without CAS (e.g. thumbv6m-none-eabi) fails to compile. | src/exact/expression.rs:90-104, src/exact/expression.rs:374, src/exact/mod.rs:59-61 |
| FND-016 | low | `CheckingLimits::nodes` is accepted by `new` and returned by `nodes()`, but nothing enforces it. Scenario: a caller sets `nodes = 10` expecting a checking budget and gets none. | src/exact/expression.rs:106-143 |
| FND-017 | low | Code ported ahead of its callers stays behind `#[allow(dead_code)]`: `TypeEnvironment::composites` and `form_grouped`, both for the out-of-scope FR-145 machine. Scenario: the two drift from the authority untested until a caller arrives. | src/exact/composite.rs:1009, src/exact/collection.rs:271 |
| FND-018 | low | `ChargePoint::ALL` (52 entries), `LimitKind::ALL` (10) and `consumed: [u64; 10]` are hand-listed beside exhaustive matches. The census test `ChargePoint::ALL.len() == 52` still passes when a new variant is left out of ALL. Scenario: a 53rd charge point compiles because `as_str` is forced and ALL is not. It is silently skipped by the TC-031 denial sweep that iterates ALL. | src/exact/accounting.rs:78, src/exact/accounting.rs:264, src/exact/accounting.rs:486, tests/exact_composite.rs:46 |
| FND-019 | low | The snapshot field list lives in three hand-kept places: the `write!`-built encoder, the decoder's field match, and the JSON schema. Emitted JSON is not a `Serialize` type, although serde is already a dependency of that feature. Scenario: a field added to the decoder and schema but not the encoder makes encode/decode round-trip asymmetric. Only the existing round-trip tests would notice. | src/snapshot_json.rs:95-101, src/snapshot_json.rs:207-215, schemas/campaign-snapshot-v1.schema.json:8-22 |
| FND-020 | low | Unit-test files sit at the src root and are mounted by `#[path]` from their modules, rather than placed beside the code. Scenario: a reader of src/ sees three test files that look like crate modules. | src/accounting_tests.rs, src/exact_accounting_tests.rs, src/exact_integer_tests.rs, src/exact/accounting.rs:683-685 |
| FND-021 | low | The function-application records `Origin.index`, `Location.path` and `InputRefusal::Arity` carry `usize`, so their width depends on the platform. Scenario: a record rendered on the 32-bit target and on a 64-bit host differs in type width. Any future serialized transcript would need a conversion. | src/exact/expression.rs:200, src/exact/expression.rs:207, src/exact/expression.rs:220, src/exact/expression.rs:439-441 |

### Triage tags

- FND-001: IR-355 (rescope: the crate is already gone; recreate the agreement check against
  quire-exact plus qsl-eval in agent-ix/quire-integration). IR-20 duplicates it.
- FND-002: IR-342 (spec), then IR-345 (AD) and IR-349 (code move).
- FND-003, FND-014: IR-356 (scope has grown from five conditions to at least ten).
- FND-004: NEW: needs ticket.
- FND-005, FND-010, FND-015: IR-345.
- FND-006: IR-345 and IR-349; feeds IR-340.
- FND-007, FND-012: IR-340; the code half goes to IR-349.
- FND-008: IR-365.
- FND-009: IR-355 rescope (close IR-20 as a duplicate, repoint the matrix away from IR-430).
- FND-011: NEW: needs ticket (loss reporting at the call boundary; FR-273 or IR-331 decides
  whether it is required).
- FND-013: NEW: needs ticket.
- FND-016, FND-017, FND-018, FND-019, FND-020, FND-021: IR-349.

## Verdict

Request changes at the design level: two high findings, eight medium, eleven low.

The high findings are both about the kernel's relation to QSL. RT has no agreement evidence
left, and the shared-kernel decision is unimplemented. The decision cannot be implemented as a
plain dependency switch until IR-342 settles MSRV (1.98 vs 1.75), std vs no_std, and the
dropped quantity and equality-conversion shapes.

What holds up:

- No `unwrap`, `expect`, `panic!`, `todo!` or `unreachable!` in non-test src.
- `forbid(unsafe_code)`, and denied `arithmetic_side_effects`/`indexing_slicing`.
- Every value walk is iterative over an explicit worklist (key, equality, argument validation,
  type refusal).
- Every `as` cast is a deliberate widening or digit extraction.
- The snapshot codec is bounded.
- All 196 tests are `tc_`-named and traced, and all 94 cited ids resolve.
- clippy, test and fmt pass at 75e5ed0.

Not verified (not run or not measurable here):

- `make kani` and `make kani-mutations`.
- `make msrv` and `make size`: the 1.75 toolchain is not installed on this machine, and the
  hosted CI workflow has never been dispatched.
- `make deny`.
- `exact`'s linked panic relocations: the release rlib is thin-LTO bitcode, which objdump cannot
  read.
- FND-015's thumbv6m build failure: the target is not installed. The claim rests on
  `fetch_add` requiring CAS.
- That Value's hand-written `Drop`/`Debug` are fully iterative.
- The IR-340 Kani tractability attribution.
