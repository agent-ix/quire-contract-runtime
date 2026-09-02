---
id: SR-007
title: Gap analysis — drop the retained legacy evidence and its compatibility machinery
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime#11 at cde591e; coverage arithmetic before and after; every row, obligation and gate the deletion touched"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/TM-001
    type: references
---

# SR-007: Gap analysis — drop the retained legacy evidence and its compatibility machinery

## Summary

Every task the deletion implied is done and every surviving test-matrix row is
backed by a real `tc_NNN` Rust test. The coverage arithmetic closes exactly:
backed 40/48 before, 37/44 after, `unbacked_rows` empty in both, and the 8 → 7
gap between `total` and `backed` is the suite registry in both measurements. No
row became unbacked as a side effect.

One gap was found and closed rather than accepted. `TC-013` asserted the twelve
verification outcomes over a *union* of two sources, and two of the twelve —
`unsupported` and `malformed` — came only from the source being deleted. Measured
per outcome on the pre-deletion tree, the chain alone reached ten. Deleting the
compatibility census would therefore have left `TC-013` green at ten rather than
red, which is a silently weakened gate and not a visible one.

## Coverage arithmetic

`quire coverage --scope . --json`, captured on the pre-deletion tree at `b3c0552`
and again at `cde591e`:

| | before | after | delta |
|---|---|---|---|
| `totals.backed` | 40 | 37 | −3 |
| `totals.total` | 48 | 44 | −4 |
| `totals.criteria` | 26 | 24 | −2 |
| `unbacked_rows` | `[]` | `[]` | — |
| `status_lies` | `[]` | `[]` | — |

The arithmetic closes exactly, per group:

| Group | before | after | what moved |
|---|---|---|---|
| `spec/evidence/suites.md` (suite) | 0/8 | 0/7 | SUITE-007 deleted — an unbacked row in both |
| `spec/functional/FR-005-…` (acceptance-criterion) | 6/6 | 5/5 | FR-005-AC-4 deleted |
| `spec/nonfunctional/NFR-002-…` (nfr-acceptance-criterion) | 4/4 | 3/3 | NFR-002-AC-4 deleted |
| `spec/test-matrix.md` (test-case) | 14/14 | 13/13 | TC-012 deleted |
| every other group | unchanged | unchanged | — |

backed −3 = FR-005-AC-4 + NFR-002-AC-4 + TC-012. total −4 = those three plus
SUITE-007, which was never backed. **No row became unbacked as a side effect.**
The 8 → 7 gap between `total` and `backed` is the suite registry in both
measurements, and only the suite registry: the eight suite rows had zero symbol
bindings before this change and the seven remaining ones have zero after it.

## Every row, and what backs it

| Row | Test | Symbol |
|---|---|---|
| FR-005-AC-1 | TC-009 | `tc_009_every_shared_pin_is_classified_by_the_packaged_matrix` |
| FR-005-AC-2 | TC-010 | `tc_010_the_chain_reaches_quoin_without_quoin_or_quire_executing_a_producer` and three siblings |
| FR-005-AC-3 | TC-011 | `tc_011_the_sealed_records_impact_snapshot_is_the_quire_export` |
| FR-005-AC-5 | TC-013 | `tc_013_all_twelve_verification_outcomes_are_demonstrated_and_paired_with_controls` |
| FR-005-AC-6 | TC-014 | `tc_014_no_local_evidence_framework_remains` |
| NFR-002-AC-2 | TC-007 | `tc_007_release_controls_are_mandatory` |

`spec/test-matrix.md` carries 13 test-case rows, all backed. FR-001 through
FR-004, NFR-001, and StR-001 are untouched by this change and their groups are
byte-identical before and after.

## Deleted rows, and why none is orphaned

| Deleted | Its test | Its verification after |
|---|---|---|
| FR-005-AC-4 | TC-012 | none — the criterion and the test are deleted together, and TC-012's spec document with them |
| NFR-002-AC-4 | TC-007 (trace tag only) | none — the criterion described the deleted collector and the builder that validated against the vendored PGM-01 schema; its only substantive check was the frozen-schema digest census, which is deleted |
| SUITE-007 | never bound | none — the suite registry entry is deleted and its id is not reused |

Nothing points at a deleted row, and no deleted row's claim is restated in weaker
form anywhere. `agent-ix/engineering-assurance#21`, which existed only because the
pinned mapping refused this repository's retained family, closes as **moot** rather
than as fixed.

## Twelve-outcome census, measured per source

Measured on the pre-deletion tree at `b3c0552` by running the chain and the
compatibility census separately and intersecting their contributions:

| Outcome | chain (before) | census (before) | chain (after) |
|---|---|---|---|
| pass | yes | — | yes |
| fail | yes | — | yes |
| unavailable | yes | yes | yes |
| **unsupported** | **no** | **yes** | **yes** (`audit-reports-an-unsupported-method`) |
| inconclusive | yes | yes | yes |
| not-computed | yes | yes | yes |
| **malformed** | **no** | **yes** | **yes** (`adapter-carries-a-malformed-row-as-non-success`) |
| partial | yes | — | yes |
| stale | yes | yes | yes |
| suspect | yes | — | yes |
| vacuous | yes | — | yes |
| tampered | yes | yes | yes |

Chain alone: **10/12 before, 11/12 after.** Because `TC-013` took a *union*, the
deletion would have left it green at ten. That is the gap this analysis exists to
catch, and it is closed rather than accepted.

The twelfth is `malformed`, and it is deliberately not the chain's. A chain-side
probe for it must write the row itself, so what it asserts is that the adapter's
own lookup table maps `malformed` to something other than `pass` — which holds
while `scripts/check_kani_mutations.py`, the only producer here that can emit the
state, reports `pass` instead. An independent adversarial review demonstrated
exactly that against the first attempt. `TC-013` now drives the mutation campaign
into its malformed branch and reads back what it said, so the demonstration fails
when the producer stops producing the state.

**What "twelve demonstrated" does and does not mean.** It means each state was
produced and observed by the component that owns it. It does **not** mean twelve
distinct values reach the verification receipt: `unsupported` is a Quoin finding
over the specification and appears in neither `KANI_OUTCOMES` nor `ROW_RESULTS`,
and `malformed` reaches the receipt as `failed` because Quoin's attestation
vocabulary is passed, failed, unavailable and not_computed. Both facts predate
this change and are recorded rather than claimed away.

## Gate results at `cde591e`

| Gate | Result |
|---|---|
| `make ci` | exit 0 |
| `cargo test` toolchain dependency | `tc_013` now requires `cargo-kani`, because the mutation producer checks for it before reaching its own predicate. Fail-closed; `make ci` already required it, plain `cargo test` did not |
| `quire validate` | 56/56 docs grammar-clean, 0 findings (54 at the `make ci` run, plus these two review artifacts) |
| `quire coverage --strict` | exit 0; 37/44 backed, 0 unbacked rows, 0 status lies |
| `cargo test --all-features` | 27 tests, 0 failed (1 unit + 3 integration + 4 operators + 3 proptest + 3 release-contract + 8 shared-assurance + 5 footprint) |
| Kani | 7 harnesses checked, each at or above its declared obligation floor |
| Kani semantic mutations | 3/3 controls verified |
| assurance chain | 14 scenarios, 6 controls, 9 probes, all matched; 12/12 states demonstrated |
| shared pins | 4/4 components compatible, 0 artifact mismatches, 0 mirror references |
| footprint | within the governed 500-byte floor and 4,096-byte ceiling, 0 panic relocations |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-701 | high | `TC-013`'s twelve-outcome assertion took a union of the chain's `states_demonstrated` and the compatibility census's case kinds; `unsupported` and `malformed` were supplied only by the census, so its deletion would have left the test passing at ten of twelve | `tests/shared_assurance.rs`, `spec/test/TC-013-verification-outcomes.md` | correct-requirement-no-evidence |
| FND-702 | medium | `NFR-002-AC-4` was bound to `tc_007` by a trace tag alone; that test asserted nothing about the collector or the vendored schema the criterion described, and the criterion's only substantive check was the frozen-schema digest census being deleted | `spec/nonfunctional/NFR-002-panic-compatibility-license.md` | wrong-requirement |
| FND-703 | low | `SUITE-007` was an unbacked suite-registry row before and after; deleting it changes the denominator but no backing | `spec/evidence/suites.md` | correct-requirement-no-evidence |
| FND-704 | high | **Independent adversarial review.** The first replacement demonstration of `malformed` asserted a dict lookup rather than a produced state, and stayed green while the producer that owns the state was hollowed out to report `pass`. Closing FND-701 with an unfalsifiable check reads 12/12 exactly as a real one does | `scripts/assurance_chain.py`, `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-705 | medium | **Independent adversarial review.** `FR-005-AC-6`'s frozen-schema clause asserted over a population this change deleted; coverage still counted the criterion as backed | `spec/functional/FR-005-shared-assurance-intake.md` | wrong-requirement |
| FND-707 | medium | **Independent adversarial review, second round.** The deleted-name census matched files by extension, so the extensionless `Makefile` — the one file a reintroduced Make target could live in — was never scanned, and the `compat-view` name the remediation added was unenforceable | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-708 | low | **Independent adversarial review, second round.** The malformed probe drove only the `count == 2` side of the producer's `count != 1` predicate; the `count == 0` side was unguarded | `tests/shared_assurance.rs` | correct-requirement-no-evidence |
| FND-706 | medium | **Independent adversarial review.** Two dangling trace ids (`NFR-002-AC-4`, `PGM-01`) entered the static export through a comment in `tc_007`'s annotation block; `quire coverage --strict` exits 0 without reporting `unmatched_tags` | `tests/release_contract.rs` | correct-requirement-no-evidence |

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-701 | **FIXED** — both demonstrations re-established on surfaces that read no retained byte, and `TC-013` now reads the chain alone, which reaches 12/12. |
| FND-702 | **FIXED** — the criterion is deleted together with its trace tag and its `TC-007` paragraphs. |
| FND-703 | **ACCEPTED** — recorded so the 48 → 44 denominator change is not mistaken for lost backing. |
| FND-704 | **FIXED** — probe deleted, demonstration moved to the producer, and the reviewer's exact defect now turns `tc_013` red. |
| FND-705 | **FIXED** — the clause now names the population `tc_014` walks. |
| FND-706 | **FIXED** — comment removed from the annotation block; `unmatched_tags` empty again. |
| FND-707 | **FIXED** — `collect_sources` now collects the extensionless `Makefile` and `.yaml`; probed red by reintroducing the deleted `compat-view` target. |
| FND-708 | **FIXED** — both sides of the predicate are driven; probed red by weakening `!= 1` to `> 1`. |

## Residual and deferred

| Item | Status |
|---|---|
| `[status-column-matches-nothing]` from `quire coverage --strict` | **DEFERRED** — pre-existing, unchanged by this work; tracked as `agent-ix/quire-contract-ir#21` and recorded as `UNKNOWN-coverage-status-column-unchecked` |
| `engineering-assurance` v0.2.0 records `pending_human_acceptance` with no predicate | **DEFERRED** — pre-existing; `agent-ix/engineering-assurance#20` |
| No ix-flow decision event; the receipt reads `incomplete` with `decision_missing` | **ACCEPTED** — correct, and only the repository owner may create one |
| Make is not a trust root; `.IGNORE:` neuters the gates that feed nothing into the chain | **ACCEPTED** — recorded, not closed, by owner decision; `agent-ix/quire-contract-runtime#10` carries the measured numbers and this change does not alter them |
| `agent-ix/engineering-assurance#21` | closes as **moot** — the records it was filed about no longer exist |
