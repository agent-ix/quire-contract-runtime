# quire-contract-runtime

Small no_std runtime support for generated contract oracles and harness verdicts.

## Hash / digest / pin antipattern: do not introduce

Hashes, digests, SHAs, pins, checksum catalogs and records that track files, versions or
tools are an antipattern and have been removed from this repository. Do not introduce
any new use of them. If you find one, remove it as part of the change. The only hash
that stays is a canonical identity digest that binds a proof to the exact content it
proved. Package versions live in Cargo.toml / package.json and their lockfiles only;
reports name the app version they ran.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # the full test suite with all features
make test-features    # the crate's feature matrix
make doc              # warning-denied docs for runtime and footprint
make msrv             # all-target MSRV compatibility check
make size             # thumbv7em linked footprint and panic-relocation gate
make spec             # Quire validation and coverage
make build            # release build
make clean            # cargo clean
make deny             # cargo deny licenses and one-copy bans
make deny-mutations   # each banned QSL crate added in a scratch copy must fail cargo-deny with `banned`
make use-local        # patch first-party git deps to sibling checkouts via a gitignored .cargo/config.toml (none today); snapshots Cargo.lock to .cargo/Cargo.lock.pre-local; fails if cargo metadata fails or a patch is unused
make use-remote       # delete the patch config and restore Cargo.lock from that snapshot (no snapshot: lock untouched)
make kani             # the proofs; an absent toolchain fails
make kani-mutations   # injected defects must fail their owning proofs
make ci               # all mandatory local gates; never dispatches hosted CI
```

## Safety scaffolding

- `clippy.toml` caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources, and `make deny` fails if any agent-ix git crate resolves more than once in a Cargo.lock (`scripts/check_one_copy.awk`); it bans every QSL workspace crate except `quire-exact` by name (FR-275)
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `verification/kani.rs` is compiled under `cfg(kani)`. Stable Clippy does not type-check that
  configuration; rustfmt and `make kani` are the controls for this boundary.

## Layout

```
src/lib.rs             # crate root
verification/kani.rs   # proof harnesses, compiled only under cfg(kani)
measurement/footprint/ # the linked-footprint population
tests/                 # integration, operator, proptest and exact-oracle tests
spec/                  # spec.md (subsystem registry), tests.md, <subsystem>/{functional,matrix,...}
plan/                  # implementation plans
scripts/               # feature matrix, footprint and Kani mutation scripts
```
