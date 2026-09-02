# quire-contract-runtime

Small no_std runtime support for generated contract oracles and harness verdicts.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # producers, then the full suite including the assurance gates
make test-features    # the crate's feature matrix, via its own producer
make doc              # warning-denied docs for runtime and footprint
make msrv             # exact Rust 1.75 all-target compatibility check
make size             # governed thumbv7em linked footprint and panic-relocation gate
make spec             # Quire validation and coverage
make build            # release build
make clean            # cargo clean and drop the assurance environment
make deny             # cargo deny check licenses
make audit-unsafe     # every unsafe block has a // SAFETY: comment
make audit-panic      # reject intentional panic paths
make kani-census      # the declared harness census, without running Kani
make kani             # the proofs; an absent toolchain fails closed
make kani-mutations   # injected defects must fail their owning proofs
make assurance-env    # create the pinned shared-assurance interpreter
make assurance-inputs # the ONLY target that runs a producer
make pins             # classify the toolchain through the shared matrix
make assurance-chain  # seal, retain, and verify through Quoin
make assurance        # pins + assurance-chain
make ci               # all mandatory local gates; never dispatches hosted CI
```

## Assurance

This repository does not retain or verify its own evidence. Engineering Assurance
0.2.0, quire-cli 0.31.0 (engine 0.46.0), quoin 0.23.1 and ix-flow 0.0.4 own that,
and `assurance/pins.json` records the release and the digests of what is read
from it. Every component resolves from the public npm registry; `npm.ix` must not
appear in any requirement, pin, lockfile or `.npmrc`, and a gate refuses to find
it written down.

`make assurance-inputs` is the only target that runs a producer. Everything
downstream consumes those files and refuses to create them.

Kani publishes no machine-readable result, so `scripts/run_kani_gate.py` owns the
transcript and publishes `runtime.kani-proof/v1`. It is the only place a
transcript is parsed. An absent `cargo-kani` produces `unavailable` rows and a
non-zero gate — never a skip and never a pass.

This repository retains no evidence of its own and no frozen schema family. The
42 `quire.derivation-evidence/v1` envelopes under `evidence/`, the reader that
read them, their fixtures and the four schemas they named by digest were deleted
under `agent-ix/quire-contract-runtime#11`, by the repository owner's decision on
2026-09-02 to release the evidence-preservation constraint for the pre-stable
phase (`agent-ix/engineering-assurance#7`). Nothing was rewritten to look as
though it still verifies. The constraint re-applies unchanged at the move toward
stable releases.

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- `rustfmt.toml` uses stable rustfmt settings with a 100-char width. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.
- `verification/kani.rs` is compiled under `cfg(kani)`. Stable Clippy does not type-check that
  configuration; rustfmt, the exact harness census, positive proof-obligation counts, and the pinned
  Kani execution are the explicit controls for this boundary.

## Layout

```
src/lib.rs             # crate root
verification/kani.rs   # seven proof harnesses, compiled only under cfg(kani)
measurement/footprint/ # the governed linked-footprint population
tests/                 # integration, operator, proptest, release and assurance tests
spec/                  # requirements, test cases, matrix, suite registry, assurance
plan/                  # PLAN-001 runtime v0.1, PLAN-002 shared assurance migration
assurance/             # the change declaration and the shared release pins
scripts/               # domain producers and the shared-assurance gates
```
