---
id: SR-13324
title: "IR-882 PR 138 gap analysis: spec-only Q-5 and QSV ownership amendments"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime@96c1519ae6a6b149696b168f8d6c2e3b9c1d1a55 vs origin/main de63b6b4; spec/assurance/AD-004-runtime-crate-layout.md, spec/core/functional/interface-001-runtime-api.md, spec/exact/functional/FR-275-single-exact-kernel.md"
review_set: subset
---

## Summary

Ticket: IR-882. PR agent-ix/quire-contract-runtime#138. This is a planless gap analysis; plan completion was not assessed. The diff is spec only and changes no source, test, AC row or matrix status.

Measured:

- `make spec` at the head and at origin/main both report "228/228 docs grammar-clean", "Coverage: 90/198 rows backed" and "124 unbacked row(s) and 0 contradicted status(es) (--strict)". The sorted outputs differ only by five-line shifts in interface-001 AC locations. The PR adds no unbacked row and changes no status.
- No code exists for the ownership the old FR-275 prose allocated to RT. `src/exact/identity_registry.rs` and `tests/exact_kernel_registries.rs` are absent at the head and on main, and PR #137 is CLOSED. Every SR-13313 finding (FND-001 to FND-006) and SR-13314 FND-002 and FND-003 (mis-tagged registry tests) therefore have nothing left to bind to. SR-13314 FND-001 is resolved by the FR-275 and interface-001 amendments. SR-13314 FND-004 (no AC for the registry rule) is carried forward as SR-13322 FND-001.
- RT library code constructs no `NodeKey` on main today. The one call outside `src/exact/node.rs` is `src/exact/equality.rs:689`, inside `#[cfg(test)]` (from line 604). This is consistent with the hold-only answer.

## Examined criteria

- FR-275-AC-2 (examined): "after the IR-882 value-bearing cutover, inspection finds no RT-defined second `Value`, pair walker or membership walker and no conversion layer or old-`Value` re-export."
- FR-275 lines 217-224 (examined); interface-001 lines 147-150 (examined).
- FR-277-AC-1 (context_only): stays PLANNED and untagged, unchanged by this PR.
- FR-012-AC-1 and TC-033 (context_only; see SR-13321 FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS.** The PR changes no code, test or matrix status, and strict coverage equals the origin/main baseline. The two findings from the closed PR #137 that still matter are covered elsewhere: the ownership conflict is resolved, and the missing AC is tracked in SR-13322 FND-001. The spec defects are in SR-13321 and SR-13322.

## Dispositions

Review pass only; no disposition round has run yet.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | Two tests still carry `#[trace("TC-033", "FR-012-AC-1")]`: `tc_033_from_hex_accepts_exactly_64_lowercase_hex_digits` and `tc_033_digest_round_trips_through_bytes_hex_and_display`. Both assert RT-local `from_hex` and `from_bytes` behaviour. `quire trace --id FR-012-AC-1` credits both as verifying the rewritten AC-1, an Inspection criterion that requires those tests and constructors to be absent, and `make spec` reports FR-012 at 5/5 (100%). The matrix row says AC-1 is retired, but coverage counts it as backed by tests that contradict it | tests/exact_vocabulary.rs:369,395; spec/exact/functional/FR-012-carried-compiler-vocabulary.md:98; spec/exact/matrix/tests.md:66 |

Round 1, verified against fix commit a9b45d77 on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | still-open | New this round; awaiting a fix round |

Round 2, verified against fix commit 36cb2cee on the PR head:

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | The two tests keep `#[trace("TC-033", "FR-012-AC-1")]`, and AC-1 again states the RT-local `from_hex`/`from_bytes` criterion they assert. The bindings are now correct, and `quire trace --id FR-012-AC-1` resolves both. The new FR-012-AC-6 (Inspection) has no claim, and its matrix row is truthfully "planned". Strict coverage moves from 124 to 126 unbacked rows: the new AC-6 verification row and its FR-012 matrix row, both planned inspections. There are 0 contradicted statuses |
