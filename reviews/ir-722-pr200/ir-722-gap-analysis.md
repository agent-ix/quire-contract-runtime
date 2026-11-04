---
id: SR-20261104024411
title: "IR-722 PR 200 context-scoped source location gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-runtime; PR #200 at 799063023e00c81a748775c36b8b8eea9a7b92c9; interface-002-AC-3/5/6, TC-209, src/monitor/program.rs, src/monitor/validation.rs"
review_set: subset
---

## Summary

The computed matrix recognizes the new AC-5 and AC-6 construction controls, and their tests exercise the production validator. The new AC-3 trace tag marks a compound older criterion as tagged although this test covers only its foreign-argument branch.

## Verdict

**FAIL for current-head acceptance evidence.** FND-001 remains fixed, but removing the direct tests leaves AC-5 and AC-6 untagged and unverified on this PR head. Their real producer-to-reader-to-runtime controls are assigned to the IR-722 integration owner; no passing result is claimed here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new AC-3 tag claims the whole compound construction criterion, but this test asserts only foreign argument and local binding refusals. It does not test duplicate function keys, invalid edges, cycles, neutral sharing, distinct inline paths, or the 20,000-level small-stack case; the computed matrix now reports AC-3 as tagged from this sole binder. | src/monitor/validation.rs:1155 |

## Coverage

At the initial 799063023e00c81a748775c36b8b8eea9a7b92c9 head, `quoin matrix --repo . --json` reported 221 criteria: 59 tagged, 42 method-without-symbol, and 120 untagged. AC-3 then had one new binder, the partial foreign-argument/local test. AC-5 has the accepted two-function provenance test; AC-6 has the duplicate-context and foreign-binding tests. The latter two assertions can fail for an incorrect validator. The 120 pre-existing untagged criteria are a repository baseline, not new PR findings. No reverse ownership gap, stub, copied upstream fixture, or hand-written matrix row was found in this diff. Optional semantic review was not selected.

Plan completion: not assessed

## Examined scope

```yaml
scope:
- id: interface-002-AC-3
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Duplicate function keys and duplicate expression source locations in one
    validated execution context refuse `DuplicateKey`; invalid expression edges, direct
    expression cycles including unreachable arena nodes, and a reachable argument
    of another function refuse with their distinct typed codes. A finite shared neutral
    subexpression and distinct inline body paths under the same source node remain
    admissible; a 20,000-level shared inline arena validates on a 512 KiB native stack
    without recursive graph descent.
- id: interface-002-AC-5
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Two functions using the same admitted State parameter node, delta parameter
    node and parameter-dependent IntegerAdd node admit distinct expression arena addresses
    when each copy's transitive binding owner is its own `FunctionId`. Both copies
    retain the original, equal `key` and `body_path` through `Program::expression`;
    the functions retain their distinct original keys and operand associations.
- id: interface-002-AC-6
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Two copies of one source location with the same derived function owner,
    two neutral copies, or a neutral and a bound copy refuse `DuplicateKey`. Mixed
    binding owners do not gain a duplicate-location exception; a reachable foreign
    argument refuses `ForeignArgument`, and a reachable foreign local or mixed local
    binding refuses `InvalidBinding`, without changing the source key or body path.
- id: TC-209
  path: spec/monitor/matrix/TC-209-monitor-program-construction.md
  role: examined
  excerpt: Run direct Program::try_new construction controls in RT for the source-location
    rule and the complete monitor_program target in quire-integration.
- id: src/monitor/program.rs
  path: src/monitor/program.rs
  role: examined
  excerpt: Two function keys or two expression locations in one execution context
    collide.
- id: src/monitor/validation.rs
  path: src/monitor/validation.rs
  role: examined
  excerpt: The validator derives expression BindingOwner values, sorts exact source
    locations with their owners, rejects same-context, neutral and mixed collisions,
    and checks reachable foreign ownership.
```

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | The current PR head changes duplicate-location admission but has no behavioral test for AC-5's shared State, delta and IntegerAdd source locations across distinct function owners. The earlier synthetic test was deleted; matrix AC-5 is untagged. | src/monitor/validation.rs:286-318 |
| FND-003 | medium | The current PR head has no behavioral test for AC-6's same-context, neutral, mixed, foreign-argument and foreign-local refusal cases. The earlier synthetic test was deleted; matrix AC-6 is untagged. | src/monitor/validation.rs:286-331 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AC-3 tag was removed from the partial foreign-binding test while TC-209 and AC-6 remain; the computed matrix now reports AC-3 untagged and AC-5/AC-6 tagged. |
| FND-002 | deferred | IR-722 remains open; the integration owner is assigned real QSL to IR to RT AC-5 controls. No direct RT pass is claimed on this head. |
| FND-003 | deferred | IR-722 remains open; the integration owner is assigned real QSL to IR to RT AC-6 controls. No direct RT pass is claimed on this head. |

After excerpt from `src/monitor/validation.rs:1155`:

```rust
#[trace("TC-209", "interface-002-AC-6")]
```

No new issue appears in the one-line fix diff. The untagged AC-3 state is an existing whole-criterion evidence gap, not a new claim of coverage by this PR.

## Current-head evidence update

At dcbdb342141fd1b3a7d0f492373073ebebbff6c9, the synthetic test module has been removed; the production validator is unchanged from the first review. `quire matrix --scope . --format json` reports 221 criteria: 56 tagged, 42 method-without-symbol and 123 untagged. AC-3, AC-5 and AC-6 each have zero binders. The old focused three-test pass belongs only to the earlier head and is not current-head qualification. No new dependency, source key or body path was introduced by the deletion. Plan completion: not assessed.
