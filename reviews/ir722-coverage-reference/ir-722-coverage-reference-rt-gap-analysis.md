---
id: SR-20261102072320
title: "IR-722 authoritative references and event-position quire-contract-runtime gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; original PR193; user ruling(b) source-reference coverage and FR276AC11 sequence"
review_set: subset
---

## Summary

Authoritative-main coverage remains FAIL until guard findings are fixed and actual accepted integration main is available. No copied binder or filtered coverage acceptance is substituted.

Same reviewer, scoped continuation of original RT193 / integration35 at RT90c7eb3133860fb094e2bd33286e0aec0f631031 and ITa2ba6f3c3bf40c60546d265826cc33481222e2c9. User ruling(b) requires authoritative sibling-main test sources by reference, with no mirrored/ported controls and no SHA/digest pin. No old source/SR ownership is replaced. Read actual Git diffs and existing repository/method guidance. No source/test edits, Cargo/heavy/proof/backend runs or merge/closure actions by reviewer. A small scratch Git fixture reproduced integrity failures only; it altered no reviewed tree.

The helper uses argument-vector subprocesses, TemporaryDirectory and file/directory symlinks; it neither executes external monitor tests nor copies their implementation or filters coverage JSON. Temporary cleanup unlinks references, leaving source files intact. Common-Git-directory discovery handles the ordinary RT worktree layout. Existing guards test reference identity/cleanup, feature/dirty-main rejection and an untracked substitute. However git branch/path existence plus ordinary diff do not prove source custody, and the alternate-checkout override lacks repository identity validation. See independently reproduced findings. Local-only reference prototype equivalence151/0 demonstrates preservation of local scan inputs, not accepted external coverage.

The new FR276AC11 control uses genuine emitted/read QSL declaration identities with matching authored source functions and explicitly controlled native ProgramParts bodies. It executes native State reads/construction and exact addition: init count0, baseline event delta0/count1, watermark/poll preserve1, gap reading65500 preserves1 and commits baseline, next65510 delta10/count2. Tick time independently goes0->65506->65516, and State has the four canonical required fields. There is no host event counter, copied temporal-status model or TC200 tag. The controlled executable body is not claimed to be actual temporal producer lowering; no full producer qualification is granted.

Preserved event-position logs show the new control and whole monitor_program86/0 plus touched Clippy PASS on the explicit interim coherent lock only; restored committed candidate cohort remains blocked by six driver adoption errors/IR1277. Real authoritative integration main currently lacks tests/monitor_program.rs, so the helper correctly fails closed with no branch fallback; rollout and actual strict reference computation remain owed after accepted main content exists. The whole-PR prior151unbacked/37introduced context is unchanged by claims here; no baseline114 waiver or current-cohort green is inferred. IEEE/CBMC is explicitly OUT OF SCOPE per ruling; no backend run/credit. Final native and own long-chain evidence from earlier carries is retained, not rerun.

Plan completion: not assessed

## Verdict

Scoped method verdict: **FAIL**; rollout/cohort readiness remains conditional.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The clean-content guard does not establish that the referenced bytes are the committed regular main file: git diff ignores assume-unchanged working-tree modifications, and a tracked symlink passes cat-file/diff/is_file while resolving to untracked outside content. Both cases are reproduced against the frozen helper in review scratch. Validate regular-file custody and actual contents against the tracked main blob without a persisted revision/digest pin; keep consuming the original file by reference. | tools/coverage_refs.py:main_test_source |
| FND-002 | medium | The explicit integration-dir override accepts any unrelated Git repository on a branch named main with a committed tests/monitor_program.rs. A scratch unrelated main repository is accepted by the frozen helper. Verify this is an authorized checkout of quire-integration, while retaining supported worktree/alternate-checkout discovery and avoiding source revision/digest pins. | tools/coverage_refs.py:main / main_test_source |

## Examined scope

```yaml
scope:
- id: coverage_refs.main_test_source
  path: tools/coverage_refs.py
  role: examined
  excerpt: 'def main_test_source(checkout):'
- id: coverage_refs.reference_scope
  path: tools/coverage_refs.py
  role: examined
  excerpt: 'with tempfile.TemporaryDirectory(prefix="rt-coverage-references-") as
    temporary:'
- id: make-spec
  path: Makefile
  role: examined
  excerpt: python3 -m unittest discover -s tools/tests
- id: reference-guard-controls
  path: tools/tests/test_coverage_refs.py
  role: examined
  excerpt: 'class CoverageReferences(unittest.TestCase):'
bindings: []
```

## Reference guard dispositions

Verified sourcee61d2cc9df495802da44946a2a836f03433a5898 on originalRT193; integrationa2ba6f3 remains unchanged. Both original integrity findings are fixed without widening coverage claims: helper validates canonical declared repository origin, main branch, regular-file tree modes/working path and actual bytes against both HEAD and fetched origin/main. It calculates/persists no digest, pins no source revision and still gives Quire original sources by reference. Seven preserved guard controls PASS, covering reference identity/cleanup, nonmain/ordinary dirty, missingtracked/untracked substitution, tracked symlink, hidden assume-unchanged, unrelated origin and unpushed local-main divergence.

The preserved make spec log runs seven guards and Quire validation, then correctly fails closed at the missing authoritative integration-main test file. Main rollout and actual accepted reference strict coverage are still owed; no branch fallback or false37-row closure. RT code scoped verdict nowPASS; scoped gap remainsFAIL for unavailable authoritative inputs/accepted discharge. Integration FR276AC11 sequence and whole86/0/touchedClippy interim-only evidence remain credited; restored candidate driver errors/current-cohort qualification remain held. IEEE/CBMC is OUT OF SCOPE. No Cargo/backend run, source/test edit, merge or closure by reviewer.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Actual referenced bytes must equal both main blobs. Both tree entries must have regular-file blob mode; the working source must not be a symlink. This removes reliance on index flags/git diff and rejects outside symlink content. Preserved guard tests include hidden assume-unchanged with git diff0 verified. |
| FND-002 | fixed | Declared origin URL is checked against canonical GitHub HTTPS/SCP-SSH/SSH forms for agent-ix/quire-integration; branchmain and both local/fetched-main regular blob contents are required. Unrelated origin and unpushed local-main controls refuse. No revision/digest pin or test copy added. |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "content = source.read_bytes()\n    if (content != regular_blob(checkout,\
    \ \"HEAD\")\n            or content != regular_blob(checkout, \"refs/remotes/origin/main\"\
    )):"
  reason: Actual referenced bytes must equal both main blobs. Both tree entries must
    have regular-file blob mode; the working source must not be a symlink. This removes
    reliance on index flags/git diff and rejects outside symlink content. Preserved
    guard tests include hidden assume-unchanged with git diff0 verified.
- fnd: FND-002
  outcome: fixed
  after_excerpt: raise ValueError("integration origin must identify agent-ix/quire-integration")
  reason: Declared origin URL is checked against canonical GitHub HTTPS/SCP-SSH/SSH
    forms for agent-ix/quire-integration; branchmain and both local/fetched-main regular
    blob contents are required. Unrelated origin and unpushed local-main controls
    refuse. No revision/digest pin or test copy added.
```
