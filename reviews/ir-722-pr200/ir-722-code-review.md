---
id: SR-20261104024410
title: "IR-722 PR 200 context-scoped source location code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime; PR #200 at 799063023e00c81a748775c36b8b8eea9a7b92c9; src/monitor/program.rs, src/monitor/validation.rs, interface-002-AC-3/5/6"
review_set: subset
---

## Summary

Reviewed the complete Rust diff and direct construction tests for the RT source-location boundary in IR-722. The changed validator derives owners through the existing iterative graph pass, then compares exact source keys and body paths with the derived function context.

## Verdict

**PASS for production code inspection at the current head.** No production defect was found. Direct RT behavioral tests were removed after the initial review; AC-5/AC-6 evidence remains Planned in the gap-analysis artifact. Final aggregate gates belong to the lead.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

At the initial 799063023e00c81a748775c36b8b8eea9a7b92c9 head, examined `src/monitor/program.rs`, all changed production and test lines in `src/monitor/validation.rs`, the merged interface-002 AC-3/5/6 text, the TC-209 procedure, `Program::try_new`, lexical binding preparation and the existing BindingOwner merge. No CI workflow, lockfile, copied artifact, source-key synthesis, unsafe code, or compatibility path changed. The comparator makes same-location rows adjacent, including any neutral or mixed row and repeated function owner; three distinct focused tests passed on that earlier head. Scoped Clippy passed there; this reviewer did not rerun the aggregate gate. No applicable AssuranceProfile was found. Optional semantic gap review was not selected.

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

## Current-head update

At dcbdb342141fd1b3a7d0f492373073ebebbff6c9, the 420-line synthetic NodeKey test module is removed. The changed production validator and refusal documentation are byte-identical to the previously reviewed code. The three direct construction tests reported in the initial review ran on an earlier head only; they are not evidence on this head. No direct RT behavioral test or dependency change remains in this PR. Real QSL to IR to RT controls are assigned to the IR-722 integration owner; the code review does not claim their result.
