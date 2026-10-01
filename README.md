# Quire Contract Runtime

[![Discord](https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white)](https://discord.gg/6qsdhSPE)

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
- Exact type and release artifact sizes are target-dependent. The v0.1 gate fixes Rust 1.82 and
  `thumbv7em-none-eabi`, then limits the representative static-library fixed-population
  consumer's linked `.text` plus `.rodata` to 4 KiB with no panic relocation.
- The crate is `AGPL-3.0-or-later` and `publish = false`.

## Verification

```bash
make ci          # every mandatory local gate; hosted CI stays manual-only
```

The checked-in Kani harnesses run under `make kani`, which fails when a proof fails or `cargo-kani`
is absent. Requirements, test cases, the matrix and architecture descriptions live under `spec/`; plans
under `plan/`; and code reviews and gap analyses under `reviews/`.

Agent-assisted contributions remain subject to the same traceability and review as every other
contribution.

## License

Licensed under the GNU Affero General Public License, version 3 or (at your option) any later
version (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).
