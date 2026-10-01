---
id: "SR-642"
title: "IR-349 spec review: approvals and 1.82 floor recorded (PR 94)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@c5e0f2b9ef979706d127a518a0e1466fe8647c04; spec commit d9ed159 and spec edits in 3f101b4: spec/exact/functional/FR-275-single-exact-kernel.md, spec/exact/functional/FR-273-exact-function-application.md, spec/assurance/AD-002-function-application-boundary.md, spec/assurance/AD-003-codegen-runtime-seam.md, spec/core/functional/interface-001-runtime-api.md, spec/core/non-functional/NFR-001-no-std-footprint.md, spec/core/matrix/TC-007-release-controls.md, spec/exact/matrix/TC-198-qsl-crate-edge-guard.md, spec/exact/matrix/TC-199-exact-no-std-and-footprint.md, spec/exact/matrix/tests.md"
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-275
    type: references
  - target: ix://agent-ix/quire-contract-runtime/NFR-001
    type: references
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: references
---

# SR-642: IR-349 spec review of PR 94

## Summary

Ticket: IR-349. The spec edits record the owner's 2026-10-01 approvals (the temporary
no-vendoring exception for the interim residue, expiry QSL-358 phase 2 merged; the single 1.82
floor), restate NFR-001-AC-3, TC-007, TC-199 and interface-001-AC-13 at 1.82, and move TC-198
and TC-199 to implemented.

## Method

- Read the spec diff in full. Diffed AC and TC ids: none removed.
- Checked each approval sentence against what the owner approved (the brief, and the relayed
  approval on IR-349): only the residue exception with that expiry, and 1.82. No sentence claims
  more; nothing claims the quire-canonical branch move.
- Grepped `spec/` for 1.75, "pending", "until IR-349" and "IR-349 part 1".
- Ran `make spec`: the same 5 pre-existing structural failures as main (IR-365), none new.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | "IR-349 part 1" now names two different things. FR-275 defines part 1 (step 1) as deleting every item quire-exact exports (lines 28, 210), and tests.md says "the kernel copy is deleted in part 1" (lines 68, 86). This PR deletes nothing, yet the spec credits its work to "IR-349 part 1" in TC-007:15, NFR-001:43 and 48, TC-199:21, FR-275:153 and 188, and tests.md:41-43, 69-70. A reader concludes part 1 has landed. The owner's approval excludes the quire-exact-covered kernel copy, and that copy is still in the runtime. Name this slice distinctly (for example "IR-349 slice 1", the PR body's step numbering), or restate step 1 as spanning several PRs and say the deletion is still to come. | spec/exact/functional/FR-275-single-exact-kernel.md:28, spec/exact/functional/FR-275-single-exact-kernel.md:210, spec/exact/matrix/tests.md:68, spec/core/non-functional/NFR-001-no-std-footprint.md:43 |
| FND-002 | low | NFR-001-AC-3 and FR-275-AC-9 now carry measurement and delivery history inside the acceptance criterion: "IR-349 part 1 re-measured it at 1.82 (913 bytes; 907 bytes at 1.75, the previous floor)" and "(IR-349 part 1); the 1.75 build could not pass". The byte count goes stale with any code change while the AC still asserts it. Keep the AC to the band and the target; put the measurement and history in Verification or the matrix row. | spec/core/non-functional/NFR-001-no-std-footprint.md:43, spec/exact/functional/FR-275-single-exact-kernel.md:153 |
| FND-003 | low | FR-275 says "The runtime records no version, commit or digest of `quire-exact` in its sources", but Cargo.toml:31 spells `version = "=0.1.0"`. The `=` requirement guards nothing on a branch-main git dependency. And when QSL bumps quire-exact's version, it breaks resolution, plus `make use-local` ("the sibling's version does not satisfy the requirement"). Either drop the version requirement, or reword FR-275 to allow the crate's own version in the IR-434 spelling and say it is informational. | spec/exact/functional/FR-275-single-exact-kernel.md:96-97, Cargo.toml:31 |

## Verdict

**Not mergeable on spec review until FND-001 is fixed.** It is a wording fix. The approvals are
recorded truthfully. FR-275, FR-273, AD-002, AD-003 F, interface-001 and the matrix each say the
owner approved the temporary exception on 2026-10-01 with the expiry unchanged, and confirmed the
1.82 floor. None overstates the approval: no rollover, no extension of the exception to the
quire-exact-covered kernel copy, no claim about QSL's quire-canonical. No requirement, AC or TC
was removed. The stale "until IR-349 moves it" sentences (interface-001-AC-13, FR-275-AC-9,
TC-199 step 1, the matrix row) were all updated, and no 1.75 floor statement remains. TC-197
stays planned with an accurate note.
