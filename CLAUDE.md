# quire-contract-runtime

Small no_std runtime support for generated contract oracles and harness verdicts.

## Commands

```bash
make fmt            # format with rustfmt
make fmt-check      # verify formatting (CI gate)
make lint           # clippy with -D warnings
make test           # core/no-default-feature tests
make test-features  # every supported feature combination
make doc            # warning-denied docs for runtime and footprint
make msrv           # exact Rust 1.75 all-target compatibility check
make size           # linked thumbv7em footprint and panic-reference gate
make spec           # Quire specification validation
make build          # release build
make clean          # cargo clean
make deny           # cargo deny check licenses
make audit-unsafe   # check that every unsafe block has a // SAFETY: comment
make audit-panic    # reject intentional panic paths
make coverage       # repository-owned traceability integrity gate
make kani           # mandatory seven-harness Kani proof gate
make evidence-tool  # local evidence-control regression tests
make verify-evidence # verify the authoritative retained record
make assurance-anchor # bind AA-001 claims to the authoritative evidence
make ci-guard       # prove recipe failure propagation and tool identities
make ci             # all mandatory local gates above; never dispatches hosted CI
```

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
tests/integration.rs   # end-to-end tests
benches/               # criterion benchmarks (opt-in; add criterion to dev-deps)
spec/                  # requirements artifacts (from /spec-create-spec)
scripts/               # local tooling
```
