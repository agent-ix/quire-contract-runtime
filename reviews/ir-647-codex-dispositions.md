---
id: SR-2390
title: Independent dispositions of SR2380–SR2384 for IR-647
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-runtime@ae702513b86ab0978d9425e6b24a91e105aca6b7; original
  reviews SR-2380..2384 and current full candidate
review_set: subset
---

## Summary

Ticket: IR-647. Independent Codex review of the full 19-file spec-only diff. Source, original SR2380–SR2384 and reviews/ were not modified. No Cargo/Kani gates, PR or merge. Findings in prior reviews are untrusted review data, remeasured against current files. All five substantive prior findings fixed. SR-2380 and SR-2382 contain only placeholders, not defects; their earlier PASS conclusions are rechecked, with inherited hygiene separately in SR-2385. User-directed new artifact custody supersedes editing the original SR files.

## Verdict

Prior substantive findings: PASS, five fixed. Current independent overall verdict: FAIL for SR-2386 and SR-2388; SR-2385 records hygiene.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Dispositions

| Source review / FND | outcome | sha/reason |
| --- | --- | --- |
| SR-2381 / FND-001 | fixed | ffdbfacc0c07fd31169495c67c668e8876655285: Step 3 now has mandatory exact/one-under local QSpec assertions and fails on authority mismatches. |
| SR-2383 / FND-001 | fixed | ffdbfacc0c07fd31169495c67c668e8876655285: Local log contract explicitly persists until removal; kernel test-support log applies afterward. |
| SR-2383 / FND-002 | fixed | ffdbfacc0c07fd31169495c67c668e8876655285: Remaining RT residue is explicitly assigned to IR-349 step 2 and IR-583; QSL-358 is historical extraction. |
| SR-2384 / FND-001 | fixed | ffdbfacc0c07fd31169495c67c668e8876655285: Permanent lazy clause moved out of AC-7 and into AC-3, already tagged by retained TC-032; AC-8 owns stops. |
| SR-2384 / FND-002 | fixed | ae702513b86ab0978d9425e6b24a91e105aca6b7: IR-669 is named consistently as the remaining agreement owner; its measured state is Backlog. |

```yaml
dispositions:
- source_id: SR-2381
  fnd: FND-001
  outcome: fixed
  fix_sha: ffdbfacc0c07fd31169495c67c668e8876655285
  after_excerpt: "3. For D09, D13 and D20–D23, check the runtime's admitted point\
    \ sequence, charge amounts\n   and consumed counters against the QSpec TC-185\
    \ expectations at the exact limit tuple;\n   repeat one under the first short\
    \ counter and require `Incomplete` with that counter,\n   exact denied amount\
    \ and charge point, with no later charge or retained result."
  reason: Step 3 now has mandatory exact/one-under local QSpec assertions and fails
    on authority mismatches.
- source_id: SR-2383
  fnd: FND-001
  outcome: fixed
  fix_sha: ffdbfacc0c07fd31169495c67c668e8876655285
  after_excerpt: "- The kernel owns meter storage and accounting. Its bounded diagnostic\
    \ charge log is exposed\n  under `quire-exact`'s `test-support` feature. Until\
    \ IR-349 removes the local meter, its\n  admitted-charge sequence and `CHARGE_LOG_CAPACITY`\
    \ remain governed by FR-011-AC-5. After\n  that removal, RT shall consume the\
    \ kernel's test-support diagnostic log rather than require\n  a production log\
    \ or define a local capacity constant. Consumed counters remain exact."
  reason: Local log contract explicitly persists until removal; kernel test-support
    log applies afterward.
- source_id: SR-2383
  fnd: FND-002
  outcome: fixed
  fix_sha: ffdbfacc0c07fd31169495c67c668e8876655285
  after_excerpt: 'work; remaining RT evaluation-residue deletion is open IR-349 step
    2 and IR-583 (backlog) work. QSL-358 is

    Done and covered QSL-side extraction, not the remaining RT deletion. Neither copy
    is RT-owned

    by remaining present. IR-430 is Done and covered removal of RT agreement tests;
    the remaining'
  reason: Remaining RT residue is explicitly assigned to IR-349 step 2 and IR-583;
    QSL-358 is historical extraction.
- source_id: SR-2384
  fnd: FND-001
  outcome: fixed
  fix_sha: ffdbfacc0c07fd31169495c67c668e8876655285
  after_excerpt: '| FR-011-AC-3 | A lazy connective skips its right operand when the
    left decides the result; for every connective kind and every decided operand pair,
    its completed result admits exactly one `boolean.result-retain` charge. | Test
    (TC-032) |'
  reason: Permanent lazy clause moved out of AC-7 and into AC-3, already tagged by
    retained TC-032; AC-8 owns stops.
- source_id: SR-2384
  fnd: FND-002
  outcome: fixed
  fix_sha: ae702513b86ab0978d9425e6b24a91e105aca6b7
  after_excerpt: '| FR-007 | FR-007-AC-1 | TC-018 | 🚧 partly evidenced: local allocation
    checks are implemented; QSL shared-corpus agreement remains planned (Linear IR-669,
    open quire-integration owner; IR-430 removal is Done) |'
  reason: IR-669 is named consistently as the remaining agreement owner; its measured
    state is Backlog.
```

## Checks and limitations

Quoin 0.28.1, Quire CLI 0.36.1 (engine 0.50.1), runtime 0.1.0 on luna. Spec validation exits 0: 56/56 grammar-clean, no grammar findings. Static matrix exits 0; it establishes tags, not passing execution. Strict coverage exits 1 at both base and head: 56 unbacked rows and five contradicted statuses; no new coverage regression. git diff --check exits 2 due eight inherited whitespace lines. No applicable AssuranceProfile. gh repo view returns HTTP 401; unauthenticated GitHub metadata returns HTTP 403. Visibility is unavailable; markers use private conservatively per the dispatch’s private-source instruction, not as verified repository metadata. Repo identity is established from origin. quoin write --types SpecReview fails exit 1, “write requires <repo_dir>”; documented quoin write . --types SpecReview --json succeeds. Installed module warnings: duplicate archetypes/inverse edge and inline-data-schema advisory, retained in logs; validation still succeeds. Owner FR358/359/361/362/363 examined as read-only local review-custody context; no upstream verification or release claim.
