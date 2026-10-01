---
id: "SR-650"
title: "IR-342 spec review: remove the vendoring exception wording (PR 96)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@fb2274498efc0b221d97c7e49024500740b4c0b2; spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-273-exact-function-application.md, spec/exact/functional/FR-009-i13-backend-negotiation.md, spec/exact/matrix/TC-197-single-kernel-ownership.md, spec/exact/matrix/tests.md, spec/core/functional/interface-001-runtime-api.md, spec/assurance/AD-002-function-application-boundary.md, spec/assurance/AD-003-codegen-runtime-seam.md, spec/spec.md; base origin/main"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# SR-650: IR-342 spec review of PR 96 (no vendoring exception)

## Summary

Ticket: IR-342. PR agent-ix/quire-contract-runtime#96 is spec-only (9 files). It removes the
wording, added by PRs 93 and 94, that called the runtime's ported QSL residue a "temporary
exception" with an expiry date and owner approval. It says instead that the residue is vendored
code, not authorized, and to be deleted. How the runtime handles its copy until QSL-358 lands is
left as an open owner decision. Checks: integrity, EARS, id preservation, matrix, traceability,
and consistency with the owner's ruling.

## Method

- Grepped the whole tree at head, excluding `reviews/` and `.git`, for
  `temporar|exception|expir|interim|approv|phase 2|until qsl-358`. Also grepped outside `spec/`
  for `QSL-358|residue|semantic-value|vendor`. The tree covers README.md, CLAUDE.md, Makefile,
  scripts/, deny.toml, Cargo.toml, measurement/, src/ and .github/. No exception, expiry, approval
  or "phase 2" wording about the residue is left. The remaining hits are unrelated: the IEEE
  "exceptional policy", the AGPL `exceptions` in deny.toml, "licence exception", "human release
  approval", "are the exceptions", AD-003's "interim gap" about agreement evidence, and the new
  negations.
- Diffed the set of FR, NFR, US, TC, AD, interface and AC ids under `spec/` at base and head: the
  sets are identical. Changed table rows: 8 removed and 8 re-added (interface-001-AC-7 and AC-9,
  FR-275-AC-13, AC-16, AC-17 and AC-18, and the FR-275 and TC-197 matrix rows). No row was deleted.
- Read interface-001-AC-7 in its history: 215e443 (before #93), e843e1b (#93) and 35ed597 (#94).
- Checked the owner quote and the attributions against the brief.
- Read the open PR #95 diff for overlap.
- Ran `make spec`: grammar 89/89 clean. The same 5 documents fail structural validation as
  before: four matrix "Coverage Status" column headers, and interface-001 missing `id` and
  `features`. All are pre-existing (IR-365) and none is new.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | interface-001's seam YAML still names the residue as runtime-owned. Its `runtime_owned` block has `owner: quire-contract-runtime` and the rule "every exact item the upstream quire-exact crate does not export". It also has `always_runtime: [Frame, Body, CheckedPackage, Evaluation, plan_call, ...]`, and it lists function_application, checking, equality, enumeration and quantity items under `consumed`. This contradicts the rewritten interface-001-AC-7 in the same file ("shall not define `Frame`, `Body`, `CheckedPackage`, `Evaluation` or `plan_call`") and FR-275 ("The residue is not runtime-owned"). The prose at 117-118 ("describe the residue and are restated when it is deleted") puts off the correction instead of making it. Fix: move the residue items out of `runtime_owned`. Either add a `residue` entry (QSL-owned, not authorized, to be deleted, QSL-358 as relayed), or name them by the FR-275 residue list. Leave `runtime_owned` and `always_runtime` holding only the negotiators. | spec/core/functional/interface-001-runtime-api.md:160-169, spec/core/functional/interface-001-runtime-api.md:117-118, spec/core/functional/interface-001-runtime-api.md:208 |
| FND-002 | medium | The rewritten FR-275-AC-17 can only pass vacuously, and the PR drops a check that was passable. "Where the runtime still defines an `exact` item other than the negotiators, that item is ... named in the residue list and deleted" contradicts itself for an item that exists, so whenever its trigger holds it cannot be met. It then says the same as AC-16 and AC-18. The old AC-17 had one honest, passable half: every residue item the runtime still defines is on the list. That half is lost. TC-197 step 6 inherits the problem ("This step fails while any such item exists"), so step 6 can no longer find an item that is missing from the list. Suggested form, which states no policy: "Every `exact` item the runtime defines, other than the negotiators, is named in the residue list, and the list records no exception, expiry or approval." Then drop the "fails while" sentence from step 6. AC-16 and AC-18 already carry the end state that is not yet met. | spec/exact/functional/FR-275-single-exact-kernel.md:162, spec/exact/matrix/TC-197-single-kernel-ownership.md:33-35 |
| FND-003 | low | Flipping interface-001-AC-7 to the negative is the right correction. Before #93 it read "The runtime shall define `Frame` ... in its own source", a claim that the runtime owns these items, which the owner's ruling negates. FR-275-AC-14 forbids deleting the AC, so the negation is the smallest truthful change. Two problems remain in the new text. It has two shall clauses ("shall not define ...: the `exact` module shall expose the QSL-owned definitions"). The second clause requires QSL to publish items under these exact names in a crate that does not exist yet, and that crate's shape is relayed, not verified. interface-001-AC-1 already requires the module to expose every item that consumers import. Keep the negative clause only. | spec/core/functional/interface-001-runtime-api.md:208 |
| FND-004 | low | Two passages state the future end state as present fact, without a "relayed" label. AD-002 says "QSL owns it in `quire-semantic-value` (QSL-358) and this crate consumes it", and FR-273 says "QSL owns it (QSL-358) and the runtime consumes it". `quire-semantic-value` does not exist yet, and its placement is relayed from QSL through the planner. Reword to "is to own ... (QSL-358, as relayed)", as FR-275 step 2 and interface-001:114 already do. | spec/assurance/AD-002-function-application-boundary.md:21-22, spec/exact/functional/FR-273-exact-function-application.md:124-125 |
| FND-005 | low | Four criterion cells carry status text ("Planned and unmet while the residue exists" / "while the runtime's copy exists"): FR-275-AC-16, AC-17 and AC-18, and interface-001-AC-7. Status belongs in the matrix, which already records it (tests.md FR-275 row), and this text goes stale in the requirement once the residue is deleted. Move it to the matrix only. | spec/exact/functional/FR-275-single-exact-kernel.md:161-163, spec/core/functional/interface-001-runtime-api.md:208 |

## Verdict

**Not mergeable as is: one high finding (FND-001).** The correction is otherwise sound:

- No exception, expiry, approval or "phase 2" permission wording is left anywhere in the tree
  outside `reviews/`, which is correctly left as the historical record.
- No interim policy is invented. Every "until QSL-358" sentence names the handling as an open
  owner decision. interface-001-AC-9's "While a residue operation exists" is a conditional duty
  on code that exists, not a permission. AD-003's "interim gap" (line 206) is about agreement
  evidence and is fine to leave.
- FR-275-AC-13 no longer carries the residue allowance.
- AC-16 and AC-18 are testable and consistent with AC-1 and AC-13.
- The owner's quote is verbatim in FR-275:47-48, and AD-003:160 quotes it with correct ellipses.
- FB-05 YES and `quire-semantic-value` are both labelled "relayed via the planner".
- The 1.98.1 floor stays attributed to the owner's first-hand decision.
- Ids and rows are preserved, and `make spec` shows no new failure.
- The tests.md disposition wording (coder doubt 1) is acceptable. It is repetitive but accurate,
  and it states no policy.

Routing note, not a finding on this PR: open PR #95 rewrites the same tests.md rows, FR-275, AD-002
and interface-001 with the old wording ("stays until QSL-358 phase 2", "interim residue", "AC-16
and AC-18 also need QSL-358 phase 2"). Whichever PR merges second must keep this PR's wording and
must not bring back the old text.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | Nine disposition cells in tests.md repeat "(interim handling is an open owner decision)". It grants nothing, so it is not a permission, but it reads as if an interim period exists. Paragraph 98-99 above the table already says once that how residue tests are handled before deletion is an open owner decision. Drop the parenthetical from the cells and keep the single statement in that paragraph. This is optional and does not block merge. | spec/exact/matrix/tests.md:103, spec/exact/matrix/tests.md:105-113 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f28783b: interface-001's seam YAML now has a `residue` block. It is marked QSL-owned (QSL-358, as relayed), not runtime-owned and not authorized, with no exception, expiry or approval. It lists the function_application items (now with Body, Evaluation and plan_call), checking, equality, enumeration and quantity. `runtime_owned` keeps only the negotiators (`always_runtime: [negotiate_integer_division, negotiate_ieee]`), and the YAML parses. The "restated when it is deleted" prose is gone. No other file calls the residue runtime-owned: I grepped spec, src, README, CLAUDE.md and the rest of the tree. |
| FND-002 | fixed | f28783b: FR-275-AC-17 now reads "Every `exact` item the runtime defines, other than the negotiators, is named in the residue list, and the list records no exception, expiry or approval." It can pass today. TC-197 step 6 no longer has the "fails while" sentence, and only step 7 keeps one. |
| FND-003 | fixed | f28783b: interface-001-AC-7 is now a single negative clause: "The runtime shall not define `Frame`, `Body`, `CheckedPackage`, `Evaluation` or `plan_call` in its own source (FR-275; no exception, no expiry)." |
| FND-004 | fixed | f28783b: AD-002:23-24 and FR-273:124-125 now say "QSL is to own it ... (QSL-358, as relayed) and this crate / the runtime is to consume it". |
| FND-005 | fixed | f28783b: the status text is gone from the FR-275-AC-16, AC-17 and AC-18 cells and from the interface-001-AC-7 cell. The FR-275 matrix row now carries it: AC-16 and AC-18 are unmet while the residue exists, AC-17 can be checked today, and interface-001-AC-7 is unmet. |
