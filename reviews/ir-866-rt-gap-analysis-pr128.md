---
id: SR-12331
title: "IR-866 RT gap analysis of PR 128"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime#128@559313a01ea94446f4cb9ab32699ed4e0ee770d2; deny.toml, Cargo.toml, measurement/footprint/Cargo.toml, every converted test file; spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/matrix/TC-198-qsl-crate-edge-guard.md, spec/exact/matrix/tests.md, CLAUDE.md (unchanged)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-198
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-008
    type: references
---

## Summary

Ticket: IR-866 (RT child of IR-210). This gap analysis covers PR 128 and is planless: it reads `quire coverage` and `quire matrix` rather than a plan. Plan completion: not assessed. The scope is `git diff origin/main...HEAD`, one commit at 559313a, against merge base 561691e. Base and head were both measured in throwaway detached worktrees, using quire 0.36.2 (engine 0.50.2) and module spec-artifacts-process@b82e42e.

Coverage measurements, with base and head identical except for one binder:

- `quire coverage --scope . --strict`: 86/185 rows backed (46%) at both base and head; rust 155/155/155 bound/tagged/candidates at both; the same 111 unbacked rows, 0 contradicted, and byte-identical output.
- `quire matrix --format tsv`: 66 tagged / 55 untagged / 36 method-without-symbol / 1 status at both base and head. No criterion changes status.
- The binder sets are identical apart from one row. FR-008-AC-9 gains a fifth binder, `tc_025_p6_canonical_key_comparison_at_depth_has_no_stack_overflow` (tests/exact_collection.rs:320). At base, its doc line wrapped `FR-008-AC-9 (the deep values' ...` across two lines with an open parenthesis, and that tag did not bind. The conversion recovers an id the author meant to bind. `tc_025_p8_debug_exact_string_both_forms` was already a binder at base (tests/exact_collection.rs:477), so it is not the new binder.
- Warnings: one `[tag-on-non-binding-symbol]` warning (FR-009-AC-5 on `IeeeDisposition`, src/exact/ieee.rs) at both base and head. It is pre-existing, on a line this PR leaves alone, and no new warning appears.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-275-AC-7 | examined | `deny.toml` admits only the `quire-exact`, `quire-semantic-value` and `quire-canonical` own-repository Git sources and keeps `unknown-git = "deny"`; it does not admit the QSL Git repository. |
| FR-275-AC-8 | examined | `make deny` reports cargo-deny's `source-not-allowed` error for the QSL Git repository, and exits non-zero, when a normal, build or dev dependency resolves from it; a licence failure alone does not satisfy this. |
| interface-001-AC-14 | examined | The runtime's normal, build and dev dependency graphs shall include no crate from the `agent-ix/quire-spec-language` Git repository. |
| FR-008-AC-9 | examined | binder set grows by tc_025_p6 (see Summary) |
| NFR-002-AC-3 | context_only | Public data enums whose evolution affects downstream matching remain non-exhaustive, as the `compile_fail` doctests in `src/observation.rs` and `src/verdict.rs` show. |
| all 152 converted test fns | examined | the same ids before and after (the per-function script in SR-12330) |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The PR makes FR-275-AC-7 false. AC-7, verified by Inspection, says `deny.toml` "admits only the `quire-exact`, `quire-semantic-value` and `quire-canonical` own-repository Git sources". The PR adds a fourth `allow-git` entry, `https://github.com/agent-ix/ix-trace-rs`, and changes none of the statements that list the admitted sources: FR-275 "One copy" ("admits only their three own-repository Git sources"), FR-275 "Guarded edges" ("`allow-git` admits only the three own-repository sources"), TC-198 Test Procedure step 1 (it expects `allow-git` entries for exactly those three), the TC-198 Evidence Locations in spec/exact/matrix/tests.md ("the three own-repository `allow-git` entries"), and CLAUDE.md ("allow-lists licenses and the three shared Git repositories"). An inspector following TC-198 step 1 against this `deny.toml` would record a failure. The guard itself (AC-8) is not weakened: QSL is still not admitted. Only the specified inventory is wrong. Fix in this PR, spec first: amend FR-275-AC-7 and the two FR-275 prose bullets, TC-198 step 1, the tests.md evidence line and CLAUDE.md to also admit `ix-trace-rs` as a dev-only, first-party tooling source. Alternatively, reword AC-7 to cover the actual invariant (only agent-ix own-repository sources, never the QSL repository). | deny.toml:44-49, spec/exact/functional/FR-275-single-exact-kernel.md:112-118, spec/exact/functional/FR-275-single-exact-kernel.md:158, spec/exact/matrix/TC-198-qsl-crate-edge-guard.md:19-20, spec/exact/matrix/tests.md:155-156, CLAUDE.md:42 |

## Verdict

**NOT MERGEABLE as is; one high finding, a small spec-only fix.** The traceability migration itself is faithful and loses nothing. It keeps the 86/185 backed rows and the 66/55/36 matrix split, and one wrapped tag that the doc-line parser had dropped now binds. No `#[trace]` sits on a non-test symbol. The evidence for that is the placement script, not the warning count: as measured in SR-12330, quire emits no tag-on-non-binding-symbol warning for the attribute form, even when it sits on a production item. Interface-001-AC-14 and FR-275-AC-8 still hold, because the new source is not the QSL repository and `unknown-git = "deny"` stays.

The seven `Trace:` doc lines left on types and the `exact` module are correct to leave in this PR (measurements in SR-12330). The criteria they name, NFR-002-AC-3, interface-001-AC-2, interface-001-AC-4 and FR-009-AC-5, are unbacked at both base and head, although real `compile_fail` evidence exists for them. That is a pre-existing gap: quire binds trace ids only on evidence symbols, and a doctest on a type is not one. It needs its own ticket, for example a test-fn or trybuild home for those compile-fail checks. It is not a defect of this PR.

Code is otherwise complete for IR-866's stated scope. The ticket text (data) asked for the nine TC-201 lines in tests/exact_quantity_conversion.rs and every other non-Kani line. All nine are converted, and no `Trace:` line remains on a test function anywhere in the tree.
