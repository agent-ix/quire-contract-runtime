---
id: TC-197
title: "Inspect that the runtime holds one kernel and no copy"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: verifies
---
# TC-197: Inspect that the runtime holds one kernel and no copy

## Description

Check the end state of the kernel move: `quire-exact` resolves once, the runtime defines no item the
kernel exports, and no substitute for the removed copy exists. Evidence is produced by the code step
IR-349; it is planned until then.

## Test Procedure

1. List every public item the runtime defines with `exact` enabled, in every module, and the items
   `quire-exact` and `quire-semantic-value` export (`cargo doc` or `cargo public-api` over each); expect
   no name defined in the runtime that either crate exports (FR-275-AC-1, FR-275-AC-2), none defined twice
   under an alias, no `exact` module and no `pub use` of `quire-exact`, `quire-semantic-value` or the
   evaluation leaf crate (FR-275-AC-16).
2. Read `Cargo.toml`; expect `quire-exact` and `quire-semantic-value` optional, enabled only by `exact`,
   each using its own Git repository, `branch = "main"`, and no `rev`, `tag`, `path` or `[patch]`
   (FR-275-AC-3, FR-275-AC-4).
3. Run `make deny`; expect success, and `awk -f scripts/check_one_copy.awk Cargo.lock` to report one
   entry each for `quire-exact` and `quire-semantic-value`, plus one each for transitive
   `quire-canonical` and `quire-canonical-derive`. Add a second first-party source in a scratch copy and expect the gate to
   fail (FR-275-AC-5, FR-275-AC-6).
4. List the files under `tests/`, `src/` and any fixture directory; expect no test that runs a second
   kernel implementation, no use of the QSL Git repository, and no file copied from the
   QSL repository (FR-275-AC-12, FR-275-AC-13).
5. Diff the specification against its state before the move; expect no requirement, acceptance
   criterion or test case deleted, and every row whose evidence left the runtime marked planned with
   its reason (FR-275-AC-14, FR-275-AC-15).
6. Compare every item the runtime defines under the `exact` feature, other than the `scalar` items,
   with FR-275's residue list; expect each is listed, and none is recorded as an exception, with an expiry or with an
   approval (FR-275-AC-17).
7. Expect no `exact` module, the `scalar` module to define only the negotiators and their types and
   the lazy Boolean connective, nothing under `src/exact`, and an empty residue list (FR-275-AC-16, FR-275-AC-18). This step fails while the
   residue exists.
8. Expect `negotiate_integer_division`, `negotiate_ieee` and their types defined in the runtime's
   own source with no QSL crate behind them (FR-275-AC-19).

## Expected Results

One `quire-exact` copy, no kernel definition in the runtime, no agreement test, no deleted
requirement.
