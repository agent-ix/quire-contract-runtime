# quire-contract-runtime

Small no_std runtime support for generated contract oracles and harness verdicts.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # the full test suite with all features
make test-features    # the crate's feature matrix
make doc              # warning-denied docs for runtime and footprint
make msrv             # exact Rust 1.75 all-target compatibility check
make size             # thumbv7em linked footprint and panic-relocation gate
make spec             # Quire validation and coverage
make build            # release build
make clean            # cargo clean
make deny             # cargo deny check licenses
make audit-unsafe     # every unsafe block has a // SAFETY: comment
make audit-panic      # reject intentional panic paths
make kani             # the proofs; an absent toolchain fails
make kani-mutations   # injected defects must fail their owning proofs
make ci               # all mandatory local gates; never dispatches hosted CI
```

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.
- `verification/kani.rs` is compiled under `cfg(kani)`. Stable Clippy does not type-check that
  configuration; rustfmt and `make kani` are the controls for this boundary.

## Layout

```
src/lib.rs             # crate root
verification/kani.rs   # proof harnesses, compiled only under cfg(kani)
measurement/footprint/ # the linked-footprint population
tests/                 # integration, operator, proptest and release tests
spec/                  # requirements, test cases, matrix, assurance
plan/                  # PLAN-001 runtime v0.1, PLAN-002 shared assurance migration
scripts/               # feature matrix, footprint, Kani mutation and audit scripts
```
