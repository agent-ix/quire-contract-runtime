---
id: SR-14117
title: "IR-641 RT PR153 independent final-head review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; IR-641; PR153 9a5f499; Makefile, tests/deny_one_copy.rs, changed spec and review artifacts"
review_set: subset
---

## Summary

Independent non-author review of PR153 at 9a5f499. The lock gate behavior passes the focused consumer controls using the real Quire owner CLI. The committed review custody introduces whitespace that makes `git diff --check origin/main...HEAD` fail.

## Verdict

FAIL pending removal of two trailing spaces in changed review artifacts. No new code, Rust, gap, or specification behavior defect was found. Existing SR-14114 code/Rust findings FND-001 and FND-002 remain fixed; SR-14115 retains inherited full-repository gap FAIL with three deferred outcomes; SR-14116 scoped base review remains PASS. This review makes no final aggregate-gate or merge claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two committed review lines have trailing whitespace, causing `git diff --check origin/main...HEAD` to exit 2 | reviews/ir-641-rt-gap-analysis.md:1192; reviews/ir-641-rt-spec-review-base.md:218 |

## Coverage

The exact PR head and base were read from GitHub and fetched locally. `git diff --check origin/main...HEAD` independently exits 2 on the two lines cited above; `git diff --check HEAD^ HEAD` isolates both to the final custody commit 9a5f499. No other changed line is reported. The code diff removes the copied AWK gate and invokes one owner `check-lock` over a NUL-delimited, successfully completed Git enumeration. The previous failure modes recorded in SR-14114 are corrected by the checked temporary file and quoted array. The real owner CLI is `/home/peter/dev/quire-cli/target/debug/quire`, built from quire-cli main 178d6c83 and exposing `check-lock`; installed `/home/peter/.local/bin/quire` lacks it. The focused `cargo +1.98.1 test --locked -j 2 --test deny_one_copy -- --test-threads=1` run with `QUIRE` set to the owner CLI passed all eight tests, no ignored tests. Cargo.toml, Cargo.lock, runtime source and CI workflow are unchanged. The changed FR-275, TC-197, TC-202, tests.md and AD-003 prose was checked against AC-24 through AC-28 and current Make behavior; no new normative criterion or relationship edge was introduced. Existing SR-14115 full-repository inherited gaps and their deferred outcomes were inspected, not converted to a green claim. No full `make ci`, Kani, or merge was run.

## Examined Units

```yaml
scope:
  - id: FR-275-AC-24
    path: spec/exact/functional/FR-275-single-exact-kernel.md
    role: examined
    excerpt: "The normal `make deny` recipe invokes the released `quire check-lock Cargo.lock` command on this repository's committed lock. On a coherent scratch lock the real command succeeds and the recipe completes when its other policy checks pass; an invocation record identifies the executable and argument without substituting checker logic."
  - id: FR-275-AC-25
    path: spec/exact/functional/FR-275-single-exact-kernel.md
    role: examined
    excerpt: "Through the same normal `make deny` recipe, a scratch lock with duplicate first-party Git package names and a scratch lock with distinct crate names at split resolved revisions of one repository each cause a non-zero Make exit and the real command's coherence diagnostic. The aligned multi-crate control passes the command. Every agent-ix Git source in these coherence controls has a valid full resolved fragment admitted by CLI FR-027; a malformed-input diagnostic cannot satisfy a duplicate or split-revision control. These controls exercise CLI FR-027; they do not define a second normalization algorithm."
  - id: FR-275-AC-26
    path: spec/exact/functional/FR-275-single-exact-kernel.md
    role: examined
    excerpt: "A missing checker executable, malformed scratch lock, or checker refusal cannot produce successful `make deny`. The recipe propagates failure without an ignored exit, success fallback, or skipped check."
  - id: FR-275-AC-27
    path: spec/exact/functional/FR-275-single-exact-kernel.md
    role: examined
    excerpt: "The committed `scripts/check_one_copy.awk` file is absent, the normal recipe contains no reference to it, and no replacement local implementation, executable copy, compatibility shim, or revision allowance bypasses the owned command."
  - id: FR-275-AC-28
    path: spec/exact/functional/FR-275-single-exact-kernel.md
    role: examined
    excerpt: "Adoption leaves the committed Cargo manifests and lock byte-for-byte unchanged and adds no runtime dependency on developer tooling. Qualification runs the real command on the committed lock and reports any existing incoherence as a failing gate with an owning blocker; a scratch passing control or an unrelated gate failure cannot qualify that lock."
  - id: reviews/ir-641-rt-gap-analysis.md
    path: reviews/ir-641-rt-gap-analysis.md
    role: examined
    excerpt: "Full-gap FAIL retained. All three latest outcomes remain deferred from round1"
  - id: reviews/ir-641-rt-spec-review-base.md
    path: reviews/ir-641-rt-spec-review-base.md
    role: examined
    excerpt: "Scoped base/spec-conformance PASS retained"
```

## Dispositions

Pending author fix of FND-001 on the same PR; the reviewer will inspect the next head.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Exact head 76c4c146 removes only the trailing spaces at `reviews/ir-641-rt-gap-analysis.md:1192` and `reviews/ir-641-rt-spec-review-base.md:218`; `git diff --check origin/main...origin/pr153` exits 0. |

## Narrow Verification (round 1)

At published PR head `76c4c146a00c8ba5420b24afe1c088aed9325923`, the only delta from reviewed `9a5f4998638e7ece5d9d86e5ae1c340500558dda` is the two review-file whitespace removals named in FND-001. No product, test, manifest, lock, spec, or CI file changed. GitHub reports the same exact head. The correction was inspected directly; `git diff --check origin/main...origin/pr153` exits 0. The focused owner-CLI 8/8 result at the preceding head carries because the executable code and tests are byte-identical. No new Cargo, Quire checker, Kani, or aggregate gate was run. FND-001 is fixed and this supplementary review is PASS for its narrow scope; preexisting SR-14115 repository-wide gap FAIL remains deferred as recorded, and final acceptance remains with the RT owner after required ordering and exact-head gates.
