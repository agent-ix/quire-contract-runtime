---
id: "SR-1398"
title: "IR-345 spec review (scope-boundary): AD-004 keep/delete boundary (PR 100)"
type: SpecReview
analysis: scope-boundary
review_set: subset
scope: "agent-ix/quire-contract-runtime@5a09652802d2e0e13a6f233e9a2ad723d9bba16f; spec/assurance/AD-004-runtime-crate-layout.md (What this crate keeps, Open questions, QSL pieces); read against FR-275, AD-003 F, src/exact headers, QSL origin/main, IR AD-007"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/AD-003
    type: references
---

# SR-1398: IR-345 scope-boundary review of PR 100

## Summary

This review checks the boundary between what RT keeps and what moves to QSL. It also checks that
the owner questions decide nothing on the owner's behalf, and that the AD is consistent with IR
AD-007, where O-1 stays open and the AD is indifferent to it, which holds. The 11 kept scalar items
are absent from both shared crates, which I confirmed with a grep for `negotiate_` and
`ShortCircuit` in QSL. They are runtime-owned under FR-275 and interface-001-AC-8. The problem is
the 15 `expression` items.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The keep table says the 15 `expression` items are "not a copy", and the Q-1 recommendation is to keep them. But this code is a port by its own header and by merged spec. `src/exact/mod.rs` says the module "is a `no_std + alloc` port ... every type, field and variant keeps the authority's name and order". `expression.rs` says "AD-002 controls the boundary this module ports: ... the call surface of `quire_spec_language::value::expression`". QSL defines `CheckedPackage`, `PackageDeclarations`, `CheckedExpression`, `CheckCause`, `CheckRefusal` and `Evaluation` (in qsl-package, qsl-semantics and qsl-eval). FR-275-AC-13 defines a port as code that keeps the QSL authority's item names. FR-275 lists function application as unauthorized vendored code, citing the owner ruling "no vendoring ... Absolutely not allowed". AD-003 F says the same. The AD's argument, "absent from both shared crates", does not test for a port. In effect the recommendation asks the owner to approve keeping vendored code, which the org rule forbids proposing. The AD also leaves AC-13 and AD-003 F out of its list of contradicted statements. Restate Q-1 so that each option is compliant: QSL provides the call mechanism; or RT writes its own call surface that is not a port, with the reason it is runtime-owned; or deletion, with the stated CG cost. Drop "keep the existing code" as the recommendation. | AD-004 "What this crate keeps", Q-1 |
| FND-002 | medium | The AD treats `decode_admitted` as a rename ("RT's `from_bytes` calls and CG's emitted `from_bytes` become `decode_admitted`"). But QSL #590 adds `decode_admitted` together with "the T12-F reader allow-list". Its documented precondition is to call it only on bytes of a package that already passed the admitted-package identity check, and the guarantee rests on QSL's call-site allow-list. CG's emitted literal keys, and the generator's hex-decode path, fall outside that allow-list. The AD does not show they meet the precondition. Also, RT library code calls no `NodeKey` constructor today: every `from_bytes` call is in a test. Route this to QSL with Q-5: say whether consumer crates may call `decode_admitted` and under what precondition. | AD-004 "QSL pieces" H3 row, Q-5 |

## Owner questions

Q-1 to Q-6 are put to the owner or QSL, and no answer is assumed. Decision B ("no shim") rests on
the org rule and on AD-016 owner decision 2. That decision reached the AD through the IR-345
assignment text, which is relayed, and Q-2 correctly asks the owner to confirm it. The
recommendations for Q-2 to Q-6 are fair. The Q-1 recommendation is not (FND-001). The Kani risk is
stated honestly as "not measured". It just belongs at step 3 (SR-1397 FND-002). IR AD-007 is
consistent: 124 shared names, the same two missing bans, decision A, and O-1 left open (Q-6
defers to it).

## Verdict

**Not mergeable as is: FND-001 (high) must be fixed.** The fix is a rewording of the keep table and
Q-1, plus adding FR-275-AC-13 and AD-003 F to the contradiction list. No code is involved. Also fix
SR-1396 FND-001 and SR-1397 FND-001 and FND-002 (mediums) and this review's FND-002 (medium) before
merge. The lows (SR-1395 FND-001 to FND-003, SR-1396 FND-002, SR-1397 FND-003) are quick to fix in
the same round. Everything else in the AD measured true.

## Dispositions

Disposition pass 1, reviewed at agent-ix/quire-contract-runtime@c7087f953a24cc49774ed81c5914d5eef0f3fbd0 (fix commits 6a5d672, c7087f9).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c7087f9, building on 6a5d672. The keep table no longer lists the 15 `expression` items. The AD now says "Not kept by this AD", and that absence from the shared crates does not make them runtime-owned. It quotes both headers as describing a port and cites AC-13, the residue list with its owner ruling, and AD-003 F. Q-1 now has three compliant options and says "Keeping the existing code is never an option". The recommendation is (a), else (b), else (c). I re-verified its measurement at QSL origin/main: `qsl-eval`, `qsl-semantics` and `qsl-package` carry no `#![no_std]` (qsl-eval's only hit is a doc comment); they have 40, 273 and 38 `std::` lines; `qsl-semantics` enables `quire-canonical` `features = ["std", ...]`; QSL's Makefile builds only `quire-exact` and `quire-semantic-value` for `thumbv7em-none-eabi`; RT's `deny.toml` bans all three. No vendoring approval is asked for. See FND-003 for the Interim paragraph. |
| FND-002 | fixed | 6a5d672. The H3 row now says "not a rename of `from_bytes`". It names the T12-F allow-list and the admitted-package precondition (T12-B for `from_digest`, which I confirmed in QSL `quire-exact/src/node.rs`). It states that RT library code constructs no `NodeKey`. Q-5 is restated as whether a consumer may construct a `NodeKey` at all, step 4 is gated on it, and R-5 routes it. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The new "Interim" paragraph sets an interim policy that merged FR-275 leaves to the owner. It reads: "the items stay in the tree ... steps 3 and 4 only repoint what they must to keep the crate compiling". FR-275 says: "How the runtime handles its residue copy until QSL-358 lands (open, owner decision) ... no interim policy is stated." The paragraph grants no exception and asks for none, so it is not a vendoring approval request. But by saying that steps 3 and 4 maintain the ported code, it answers the owner's open interim question for the owner. Deleting it early under option (c) is not presented. The fix is wording only: cite FR-275's open interim question, and present "repoint to keep compiling" as what steps 3 and 4 do if the owner does not rule otherwise, not as this AD's policy. Or add it to Q-1 as a sub-question. | AD-004 "What this crate keeps" Interim paragraph; FR-275 Open questions |

### Dispositions, round 2

Disposition pass 2, reviewed at agent-ix/quire-contract-runtime@5777d53d8d6ea60cc609421e2545a1634f93dc34 (fix commit 5777d53).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | 5777d53. The Interim paragraph now quotes FR-275's Open question ("open, owner decision ... no interim policy is stated") and says "this AD does not decide it". It presents repoint-only as a default "which applies only if the owner rules nothing else". It presents Q-1 option (c), deletion early in step 3 or 4 with the CG cost, as the alternative. No exception, expiry or approval is granted or asked for. The contradiction list now also names the Open question, and that is honest: a stated default is more than FR-275's "no interim policy", and step 1 records the owner's answer. The treatment is honest and decides nothing for the owner. |
