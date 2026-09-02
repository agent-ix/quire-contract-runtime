---
id: Task-002
title: "Producers, adapter, chain, compatibility view, and tests"
type: Task
status: done
track: A
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-005
    type: references
  - target: ix://agent-ix/quire-contract-runtime/MP-001
    type: references
---
# Task-002: Producers, adapter, chain, compatibility view, and tests

## Scope

Publish this repository's domain results in declared structured formats, drive the official chain
over those bytes without any tool executing a producer, and read retained evidence through the
pinned mapping.

## Deliverables

- Four domain producers: `run_feature_matrix.py`, `run_kani_gate.py`, `check_kani_mutations.py`, and
  `measure_footprint.py`, publishing `runtime.feature-matrix/v1`, `runtime.kani-proof/v1`,
  `runtime.kani-mutation/v1`, and `runtime.footprint/v1`.
- `scripts/assurance_chain.py`: the driver, the native adapter, thirteen scenarios, six positive
  controls, and seven adapter probes.
- `scripts/legacy_evidence_view.py`: the read-only compatibility view over all 42 retained envelopes,
  sixteen declared cases, and five mutation probes.
- `tests/shared_assurance.rs`: six traced tests that invoke the gates rather than reimplementing
  them, including the two-run producer-isolation probe with its control.

## Completion Evidence

Every scenario, control and probe matches; the twelve verification outcomes are each demonstrated by
a case that produced them; and the Kani transcript is parsed in exactly one place, by the domain tool
that owns Kani, because Kani publishes no machine-readable result.
