# quire-contract-runtime

Small no_std runtime support for generated contract oracles and harness verdicts.

## Hash / digest / pin antipattern: do not introduce

Hashes, digests, SHAs, pins, checksum catalogs and records that track files, versions
or tools are an antipattern and have been removed from this repository. Do not
introduce any new use of it. If you find one, remove it as part of the change. The only hash that stays is a canonical
identity digest that binds a proof to the exact content it proved. Package versions
live in Cargo.toml / package.json and their lockfiles only; reports name the app
version they ran.

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
make deny             # cargo deny check licenses
make kani             # the proofs; an absent toolchain fails
make kani-mutations   # injected defects must fail their owning proofs
make ci               # all mandatory local gates; never dispatches hosted CI
```

## Safety scaffolding

- `clippy.toml` caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `verification/kani.rs` is compiled under `cfg(kani)`. Stable Clippy does not type-check that
  configuration; rustfmt and `make kani` are the controls for this boundary.

## Layout

```
src/lib.rs             # crate root
verification/kani.rs   # proof harnesses, compiled only under cfg(kani)
measurement/footprint/ # the linked-footprint population
tests/                 # integration, operator, proptest and exact-oracle tests
spec/                  # requirements, test cases, matrix
plan/                  # implementation plans
scripts/               # feature matrix, footprint and Kani mutation scripts
```
