---
id: "SR-641"
title: "IR-349 gap analysis: adopt quire-exact part 1 (PR 94)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@c5e0f2b9ef979706d127a518a0e1466fe8647c04; FR-275 AC-3..AC-11, AC-20, AC-21 against Cargo.toml, deny.toml, Makefile, scripts/check_deny_bans.sh, scripts/run_feature_matrix.py, spec/exact/matrix/tests.md, TC-197..TC-199, and the PR body's decomposition plan; cross-checked read-only against quire-spec-language origin/main 0885a9ba"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
---

# SR-641: IR-349 gap analysis of PR 94

## Summary

Ticket: IR-349. Plan completion: not assessed. Checks each FR-275 AC the PR claims (AC-3 to AC-11,
AC-20, AC-21) against code and gates at head c5e0f2b, the matrix rows it moves to implemented
(TC-198, TC-199), and the PR body's decomposition plan against quire-exact at QSL main.

## Method

- AC-3, AC-4: read Cargo.toml:26,31. AC-5, AC-6: lock count and a scratch two-entry lock against
  `check_one_copy.awk`. AC-7: ban list against QSL workspace members. AC-8: ran
  `make deny-mutations` (8 of 8 ok) and a negative copy with the qsl-eval ban removed (FAIL).
  AC-9: `make test-features` row `build-exact-no-std-msrv` passed on +1.82.0. AC-10: ran the
  `cargo tree -i quire-exact` check by hand. AC-11: `make size` 913 B. AC-20: Cargo.toml:5.
  AC-21: Makefile MSRV 1.82.0, `make msrv` and `make size` ran on 1.82.
- Grepped for `tc_198`, `tc_199`, `TC-198`, `TC-199` across code and scripts.
- Read quire-exact's `src/lib.rs` exports at QSL 0885a9ba and compared with the plan's per-file
  assignments.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-275-AC-10 (Test, TC-199) is marked implemented, but nothing runs its check. TC-199 step 2 is a manual `cargo tree -p quire-contract-runtime-footprint --target thumbv7em-none-eabi -i quire-exact`; no Makefile target, script or test runs it, and `make size` would not catch the regression (an unused `exact` in the footprint crate is removed at link time). Scenario: the footprint crate gains `features = ["exact"]`; every gate stays green while AC-10 is false. Add the check to `make size` (or a script it calls) and fail when cargo finds the package, or mark AC-10 partly evidenced. | spec/exact/matrix/tests.md:43, spec/exact/matrix/TC-199-exact-no-std-and-footprint.md:24-25, Makefile:89 |
| FND-002 | low | The feature-matrix row the matrix cites as TC-199's evidence does not trace to it. `build-exact-no-std-msrv` carries `["TC-016", "FR-006", "NFR-001"]`, with no `TC-199` or `FR-275`, so a trace consumer of the feature-matrix report never ties the row to FR-275-AC-9. Add `TC-199` and `FR-275` to its trace ids. | scripts/run_feature_matrix.py:81, spec/exact/matrix/tests.md:43 |
| FND-003 | low | The matrix's own statement is now false: "Every other row is backed by a `tc_NNN` Rust test ... Rows marked planned or partly evidenced above are the exceptions". TC-198 and TC-199 are now implemented with make-target evidence and no `tc_198`/`tc_199` test, and Evidence Locations has no entry for either. Name them as exceptions there and add Evidence Locations entries (`make deny-mutations`, `scripts/check_deny_bans.sh`; `make test-features` row, `make size`). | spec/exact/matrix/tests.md:72-74, spec/exact/matrix/tests.md:116 |
| FND-004 | low | The decomposition plan keeps `reference.rs` as residue until step 5, but quire-exact exports `ObjectReference` (`quire-exact/src/lib.rs:200`), which RT's `reference.rs:77` defines. FR-275 says an item quire-exact exports is not residue and is deleted in step 1 (AC-1), and the relayed approval says the quire-exact-covered part of `src/exact` gets no exception. The plan should delete RT's `ObjectReference` in its step 1 or 2 and keep only `ObjectEnvironment`/`ValueGraph` as residue. | src/exact/reference.rs:77, spec/exact/functional/FR-275-single-exact-kernel.md:177 |

## Verdict

**Not mergeable on gap analysis until FND-001 is fixed or AC-10 is restated as partly evidenced.**
The other claimed ACs hold at head: AC-3, AC-4 (optional, `exact` only, git branch main, no
rev/tag/path/patch), AC-5, AC-6 (one lock entry; the awk check fails a second), AC-7 (14 bans,
exactly QSL main's non-kernel members), AC-8 (8 of 8 cases report `error[banned]`, and the script
fails when a ban is removed), AC-9, AC-11, AC-20, AC-21. TC-197 correctly stays planned. The
decomposition plan is otherwise sound: the Arc vs Rc, NodeKey, Quantity (228 lines in
quire-exact) and missing equality-conversion table claims match quire-exact at QSL 0885a9ba, and
step ordering by coupling is reasonable; its step 2 (Rc to Arc, Send/Sync on `Body`) is rightly
called the risky one.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@ac43bf55a3d96d5ad6f2a375dd97c6dc662eb732.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ac43bf5: `make size` now lists `cargo tree -p quire-contract-runtime-footprint --target thumbv7em-none-eabi --edges normal,build,dev --prefix none` and fails on a `^quire-exact ` line. Mutation re-measured: with `features = ["exact"]` on the footprint dependency in a scratch copy the same listing finds quire-exact; unmodified it does not. |
| FND-002 | fixed | ac43bf5: row `build-exact-no-std-msrv` traces `["TC-016", "TC-199", "FR-006", "FR-275", "NFR-001"]`. |
| FND-003 | fixed | ac43bf5: tests.md names TC-198 and TC-199 as gate-target exceptions, and Evidence Locations has a TC-198/TC-199 entry. |
| FND-004 | fixed | ac43bf5: the FR-275 residue list splits `reference.rs` (only `ObjectEnvironment` and its refusal types stay; identities and `ObjectReference` go in step 1) and `node.rs` (only `SemanticGraphCause`/`InvalidSemanticGraph` stay), and the PR body plan deletes them in its Value slice. Checked: quire-exact exports no `ObjectEnvironment`. |
