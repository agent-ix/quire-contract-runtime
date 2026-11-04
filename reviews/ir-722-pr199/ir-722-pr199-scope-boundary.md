---
id: SR-20261104024408
title: "IR-722 PR 199 RT and source-admission ownership boundary"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-runtime; IR-722; PR #199; interface-002 and TC-209 diff, interface-004 context"
review_set: subset
---

## Summary

Independent review of RT spec PR #199 at `8d43b96c504970092293eb83e33e1dc5196cd633`. RT and source-admission ownership boundary.

## Verdict

**PASS** — No findings in this scoped method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined Scope

```yaml
scope:
- id: interface-002-AC-3
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Duplicate function keys and duplicate expression source locations in one validated execution
    context refuse `DuplicateKey`; invalid expression edges, direct expression cycles including unreachable
    arena nodes, and a reachable argument of another function refuse with their distinct typed codes.
    A finite shared neutral subexpression and distinct inline body paths under the same source node remain
    admissible; a 20,000-level shared inline arena validates on a 512 KiB native stack without recursive
    graph descent.
- id: interface-002-AC-5
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Two functions using the same admitted State parameter node, delta parameter node and parameter-dependent
    IntegerAdd node admit distinct expression arena addresses when each copy's transitive binding owner
    is its own `FunctionId`. Both copies retain the original, equal `key` and `body_path` through `Program::expression`;
    the functions retain their distinct original keys and operand associations.
- id: interface-002-AC-6
  path: spec/monitor/functional/interface-002-monitor-program.md
  role: examined
  excerpt: Two copies of one source location with the same derived function owner, two neutral copies,
    or a neutral and a bound copy refuse `DuplicateKey`. Mixed binding owners do not gain a duplicate-location
    exception; a reachable foreign argument refuses `ForeignArgument`, and a reachable foreign local or
    mixed local binding refuses `InvalidBinding`, without changing the source key or body path.
- id: TC-209
  path: spec/monitor/matrix/TC-209-monitor-program-construction.md
  role: examined
  excerpt: Run direct Program::try_new construction controls in RT for the source-location rule and the
    complete monitor_program target in quire-integration for the existing producer/reader boundary.
- id: interface-004
  path: spec/monitor/functional/interface-004-monitor-lexical-bindings.md
  role: context_only
  excerpt: The program keeps the same ProgramParts, Layout, Function and Expression fields. There is no
    synthetic helper declaration, new source identity, callback, closure DTO or source parser.
```

## Analysis

RT owns representation and integrity validation of supplied expression arenas. The source reader and driver own semantic admission and original NodeKey supply; the spec neither asks RT to parse source nor grants it a new key/path. TC-209 separates direct RT construction controls from integration producer/reader controls and does not claim FR-276 qualification.

Targeted `quire validate` exited 0 with existing unresolved-type warnings. `quoin matrix --repo . --json` recognizes AC5 and AC6 as untagged with no run evidence, consistent with their Planned status. No applicable AssuranceProfile was found under `spec/`. No Cargo, Kani, or aggregate gate was run for this spec-only PR.
