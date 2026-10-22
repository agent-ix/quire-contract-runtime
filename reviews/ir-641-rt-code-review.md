---
id: SR-14114
title: "IR-641 RT consumer code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; IR-641; Makefile, tests/deny_one_copy.rs, docs and full repository matrix"
review_set: subset
---

## Summary

Independent review of RT PR153. FAIL pending correction of FND-001.

## Verdict

FAIL pending correction of FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Unquoted Git filename expansion splits tracked lock paths and passes Git-quoted escape text to the checker | Makefile:182 |

## Coverage

Candidate e8cc22f301a556bce8a1ad20c8936502e05538b9, exact base 3d2d8201dfda896fde0e960a05cfc26d44c5542f. No applicable AssuranceProfile exists under spec. No Cargo/native/fmt/backend/provision/aggregate command was run by this reviewer. Actual author focused05 raw log has six tests passed, no ignored tests, exit0, 16GiB/swap0, peak75100160B, zero OOM events and vanished scope. Existing final Clippy/fmt evidence is read, not rerun. Root retains normal aggregate gates. Cargo.toml, Cargo.lock, runtime/kernel sources and CI are unchanged. Deleted AWK logic is consumed via the real distributed CLI; no local algorithm, executable, shim or revision allowance was introduced. CLI check_lock source requires one or more explicit arguments; zero locks therefore fail rather than silently skip. Missing executable/malformed input cannot satisfy duplicate/split controls, whose full fragments and diagnostic assertions were inspected. Source inventory covers src/, scripts/, verification/ and tests/; candidate adds no orphan runtime API or stub. Exact Makefile substitution was independently observed with shell printf only: tracked space dir/Cargo.lock became two arguments, while é/Cargo.lock became literal Git quote/escape text. This observation did not replace or run checker logic. Real filename-boundary regression tests must use the same genuine CLI. Scope records include all reviewed units, including clean bindings. Rust lane folded into this artifact: fixture cleanup/unique dirs, real process boundary, no unsafe/async change, no new allow/ignore, no weakened Cargo policy; CARGO=true isolates only unrelated cargo-deny in scratch trees. Searches found no retained AWK copy or alternative checker.

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
  role: examined
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
  role: examined
  excerpt: '`Cargo.lock` holds exactly one entry each for `quire-exact`, `quire-semantic-value`,
    `quire-canonical` and `quire-canonical-derive`.'
- id: FR-275-AC-6
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
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
  role: examined
  excerpt: The runtime's declared `rust-version` is 1.98.1.
- id: FR-275-AC-21
  path: spec/exact/functional/FR-275-single-exact-kernel.md
  role: examined
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
bindings:
- test_id: deny_checks_the_unchanged_committed_lock_with_the_owned_command
  ac_id: FR-275-AC-5
  trace: correct
- test_id: deny_rejects_duplicate_first_party_package_names_with_valid_sources
  ac_id: FR-275-AC-6
  trace: correct
- test_id: deny_checks_the_unchanged_committed_lock_with_the_owned_command
  ac_id: FR-275-AC-24
  trace: correct
- test_id: deny_checks_every_tracked_lock_in_one_owned_invocation
  ac_id: FR-275-AC-24
  trace: correct
- test_id: deny_rejects_duplicate_first_party_package_names_with_valid_sources
  ac_id: FR-275-AC-25
  trace: correct
- test_id: deny_accepts_aligned_crates_and_refuses_distinct_name_repository_splits
  ac_id: FR-275-AC-25
  trace: correct
- test_id: deny_checks_every_tracked_lock_in_one_owned_invocation
  ac_id: FR-275-AC-25
  trace: correct
- test_id: deny_accepts_aligned_crates_and_refuses_distinct_name_repository_splits
  ac_id: FR-275-AC-26
  trace: correct
- test_id: deny_checks_every_tracked_lock_in_one_owned_invocation
  ac_id: FR-275-AC-26
  trace: correct
- test_id: deny_propagates_malformed_input_and_missing_checker_failures
  ac_id: FR-275-AC-26
  trace: correct
- test_id: owned_gate_has_no_local_checker_copy_or_fallback
  ac_id: FR-275-AC-27
  trace: correct
- test_id: deny_checks_the_unchanged_committed_lock_with_the_owned_command
  ac_id: FR-275-AC-28
  trace: correct
```

## Dispositions

Round1, Original FND-001 fixed. Newly discovered FND-002 remains open; code/Rust remains FAIL until same-PR correction and disposition.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | NUL-delimited Bash array and quoted argv preserve filenames; verified genuine RED-to-seven-PASS evidence. |
| FND-002 | fixed | Completed Git enumeration is checked before the same single owner checker invocation; genuine old-recipe RED101 and fixed eight-control PASS0 establish the failure scenario and its correction. |

## Narrow Verification (round 1)

Exact published candidate c8609adbba0ddfa000d11f310fe1be7d88153bd8; original same reviewer/run ae2ff59d-21b1-4119-a583-3a8435154c50. Read-only git/Linear/GitHub and preserved raw logs only; no new Cargo/native/Kani/full gate, lock change, rebase, branch write or merge. All six original typed/raw review files in candidate are byte-exact to the original reviewer custody. Relative to test-first75ed338b, production change is only the Make recipe and Rust change is regression-test formatting. The seventh real process control had actual RED101 on75ed338b (Git-quoted café path, verification retained) and all7 controls PASS0 after the fix using actual ownerCLI2005; no ignored tests or checker substitute. Four filename families are space, Unicode, newline and glob characters; the real incoherent lock is placed at each path independently. Existing duplicate/split/malformed/missing-checker cases remain. Raw fmt/lint exit files are0. The author reports16GiB/swap0/jobs2 scopes; focused log names its actual scope, but new-scope memory peak/caps are not independently remeasured by this source-only disposition. Root owns caps/final-gate verification. RT154 and RT155 are still OPEN at readback; RT153 final rebase/aggregate/acceptance/merge remain HELD pending RT154 thenRT155 ordering. No final mergeable claim is made against unrebased main.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | Git producer failure after partial filename output is lost, allowing successful coherence validation of an incomplete tracked-lock set | Makefile:182 |

The `git ls-files` status belongs to a process-substitution child and is neither waited nor checked before exec. `mapfile` can succeed on a prefix followed by EOF even when Git exits nonzero (for example, Git receives SIGTERM after writing the root lock path). The checker then sees a nonempty proper subset and can report success while a later tracked lock was never supplied. Empty/no-output producer failures do fail because CLI requires at least one path; that does not establish fail-closed behavior for partial output. This is source-traced, not a newly executed reproduction. Require successful Git enumeration before invoking the checker, preserve NUL filename boundaries, and add a producer-failure-after-output control against the genuine checker. Normal filename and seven existing successful/error-path controls remain required. FND002 has no disposition yet and blocks acceptance.

## Narrow Verification (round 2)

Scoped code/Rust PASS at the reviewed head: FND001 remains fixed from round1; FND002 is fixed in round2; no new findings. Original finding rows, round1 wording and previous outcomes remain unchanged as history.

Original reviewer/run ae2ff59d-21b1-4119-a583-3a8435154c50, published exact head 0c37ad083f2254b0588942920ad7ee194785e0c2, RT PR153 still OPEN. Relative c8609adb production delta only changes Makefile enumeration/completion handling; test delta adds one external Git-boundary fault control and passes PATH explicitly, with the genuine owner checker unchanged. All six prior round1 custody files match reviewer source bytes. No Cargo manifest/lock, runtime/kernel source or CI change. Reviewer ran only source/readback/Quire operations, no new Cargo/native/full gate/Kani/lock/rebase/merge. Raw producer-red.log independently shows the old recipe returned Make success after valid root Cargo.lock NUL prefix plus Git exit42, causing the new test RED101. The actual fixed recipe is exercised by all8controls PASS0, no ignored tests; producer-fmt and producer-clippy exit0. OwnerCLI2005 is the same real installed CLI; the only test double is the external Git producer, not the checker or lock algorithm. The test first demonstrates actual refusal of the incoherent tracked sibling without injection, then injects prefix/failure and requires nonzero Make, producer diagnostic, and no checker success report. Previous filename spaces/Unicode/newlines/globs and all real aligned/duplicate/split/malformed/missing-executable controls remain. Author-reported16GiB/swap0/jobs2 scope settings are not independently remeasured here; raw logs show actual scope invocation IDs and root owns cap/final-gate verification. RT154 remains OPEN at this readback; RT154 merge SHA and required lane ordering, followed by final rebase/gates/review, still hold RT153. No final mergeable claim is made on unrebased main.

`mktemp` failure, Git failure and mapfile failure each exit before the checker. NUL-delimited data survives through the checked temporary file and quoted Bash array. Empty lock enumeration reaches the CLI required-argument refusal rather than succeeding. The checker remains the final command; EXIT cleanup removes the temporary file while preserving the command failure status. No ignored exit, policy fallback or additional checker copy was introduced. Linux/Bash matches the existing Ubuntu workflow scope; no additional platform acceptance is claimed.

## Narrow Test Bindings (round 2)

```yaml
bindings:
- test_id: deny_preserves_tracked_filename_boundaries_for_the_real_checker
  ac_id: FR-275-AC-24
  trace: correct
- test_id: deny_preserves_tracked_filename_boundaries_for_the_real_checker
  ac_id: FR-275-AC-25
  trace: correct
- test_id: deny_preserves_tracked_filename_boundaries_for_the_real_checker
  ac_id: FR-275-AC-26
  trace: correct
- test_id: deny_rejects_a_git_producer_failure_after_a_nonempty_prefix
  ac_id: FR-275-AC-24
  trace: correct
- test_id: deny_rejects_a_git_producer_failure_after_a_nonempty_prefix
  ac_id: FR-275-AC-26
  trace: correct
```
