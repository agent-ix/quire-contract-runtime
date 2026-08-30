---
id: REV-004
title: "Runtime v0.1 plan-bundle migration"
type: Review
---

# Runtime v0.1 plan-bundle migration

The mechanically checkable implementation plan is retained at
`plan/PLAN-001-runtime-v01/plan.md`. Its task files distinguish the five completed implementation and
evidence-preparation tasks from the human-owned source-release decision, which remains open.

## Dependency DAG

```text
PGM-01 -> foundation (#2) -> verdicts/observations (#1) -> operators/adapters/accounting (#3)
                                            \---------------------------> evidence + gap review
```

## Historical step mapping

1. Validate the requirements and five assurance artifacts with the installed Quire modules.
2. Implement the dependency-free no_std identity, observation, verdict, operator, checked arithmetic,
   and accounting core for FR-001, FR-002, and FR-004.
3. Add the pinned optional proptest adapter for FR-003 and verify the feature matrix.
4. Add requirement-tagged unit, integration, property, and Kani harnesses.
5. Collect CI, dependency, size, panic/unsafe, licensing, and provenance evidence; perform gap review.
6. Present the candidate to the human release owner. Do not publish to crates.io.
