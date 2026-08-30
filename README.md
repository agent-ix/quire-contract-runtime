# Quire Contract Runtime

`quire-contract-runtime` is the small `no_std` support library linked by generated contract oracles.
It keeps successful checks, failed postconditions, and rejected preconditions distinct and observable.

```rust
use quire_contract_runtime::{
    ContractIdentity, ExecutionPoint, RequirementId, RevisionId, Verdict, VerdictContext,
};

let context = VerdictContext::new(
    ContractIdentity::new(RequirementId::new("REQ-42"), RevisionId::new("sha256:...")),
    ExecutionPoint::new("after-update"),
    &[],
);
let verdict = Verdict::passed(context);
```

There is intentionally no `Verdict -> bool` conversion. Callers must preserve `Passed`,
`FailedPostcondition`, and `RejectedPrecondition` rather than allowing a rejected input to become
successful evidence.

## Runtime surface

- Borrowed requirement/revision, execution-point, and clause identities.
- Allocation-free per-clause outcomes and structured failure details.
- Separately named short-circuit and total Boolean/implication operators.
- Checked option, slice-index, integer arithmetic, division, and remainder helpers.
- Complete, saturating accepted/rejected/failed/discarded campaign counts.
- An opt-in adapter to proptest's success/failure/rejection result.

The default feature set is empty. `alloc` and `std` are explicit opt-ins reserved for convenience
surfaces; `proptest` implies `std` and is intended only for development harnesses.

```toml
[dependencies]
quire-contract-runtime = { git = "https://github.com/agent-ix/quire-contract-runtime", default-features = false }

[dev-dependencies]
quire-contract-runtime = { git = "https://github.com/agent-ix/quire-contract-runtime", features = ["proptest"] }
```

## Contracts and limitations

- The core is safe Rust, performs no allocation or I/O, and has no required dependency.
- Undefined partial operations return `None`; counters saturate rather than panic.
- Public data enums are non-exhaustive for forward-compatible retention of future states.
- Exact type and release artifact sizes are target-dependent and retained per candidate. The v0.1
  release rlib ceiling is 256 KiB.
- The crate and generated customer-linked surface are `MIT OR Apache-2.0` and `publish = false` until
  the human v0.1 source-release decision.
- Release evidence can support a consuming project's validation or accreditation decision; it does
  not confer one.

## Verification

```bash
make ci
bash scripts/collect_evidence.sh
```

The checked-in Kani harnesses are a required CI job using pinned Kani 0.67.0. A local evidence run
records an unavailable Kani installation as skipped, never as passed. Requirements and assurance
artifacts live under `spec/`, with planning and review records under `planning/`.

Agent-assisted contributions remain subject to the same traceability, review, evidence, and human
release gates as every other contribution.

## License

Licensed under either Apache License, Version 2.0 or the MIT license at your option.
