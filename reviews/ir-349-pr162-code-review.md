---
id: SR-35032
title: "code-review of IR-349 direct scalar dependency removal (quire-contract-runtime PR 162)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-runtime@7da6e84943911d16e537448fea37530395615052; Cargo.toml, Cargo.lock, clippy.toml; base agent-ix/quire-contract-runtime@95a27602cdbec7c70a765520b79a57bc680bebc2; cross-checked src/, tests/, verification/, measurement/footprint, tools/runtime-gates, deny.toml, Makefile, CLAUDE.md, spec/"
---

# Code review of PR 162: remove unused direct scalar dependencies (rust-review lane folded in)

## Summary

Ticket: IR-349. PR agent-ix/quire-contract-runtime#162, frozen head 7da6e849, one commit over
95a27602 (RT #160). The diff removes `num-bigint`, `num-integer`, `num-traits` and
`unicode-normalization` from the `exact` feature and `[dependencies]`, drops their four
`quire-contract-runtime` dependency edges in `Cargo.lock`, and deletes the clippy.toml
`arithmetic-side-effects-allowed = ["num_bigint::BigInt", "num_bigint::BigUint"]` entry with its
comment. No Rust source, test, CI workflow, `deny.toml` or Makefile change.

## Method

- Usage census at head, excluding `target/` and `Cargo.lock`: `num_bigint`, `num-bigint`,
  `num_integer`, `num_traits`, `unicode_normalization`, `BigInt`, `BigUint` across `src/`,
  `tests/`, `verification/`, `measurement/`, `tools/`, `examples/`, docs, `Makefile`, `deny.toml`,
  `CLAUDE.md`, `AGENTS.md`, `README.md`. No code, build script, bench, doctest or feature names
  any of them. Hits are only comments in `verification/kani.rs` (the word `BigInt`), review
  records, a historical plan task, and AD-004 prose. There is no `build.rs` and no `benches/`.
- `deny.toml` names none of the four (no bans list, no skip, no license exception). Licences
  stay covered because the crates remain in the graph through `quire-exact`.
- Lock: `cargo tree --locked` succeeds at head with no lock rewrite (`git status` clean); the
  diff removes only the four edges, no `[[package]]` entry, since `quire-exact` still depends
  on all four. `cargo tree -i num-bigint` / `-i unicode-normalization` show `quire-exact` as
  the only dependent.
- No other dependency changed: `cargo tree --locked -e normal,build -f '{p} {f}'` for
  `--features exact --target thumbv7em-none-eabi`, `--all-features`, and
  `--no-default-features --target thumbv7em-none-eabi` diffed between base and head. The only
  differences are the four removed duplicate `(*)` lines; every package and its resolved
  feature set is identical, so no transitive feature was dropped.
- clippy.toml allow-list removal: `src/lib.rs:21` denies `clippy::arithmetic_side_effects`, and
  no in-crate code names `num_bigint::BigInt`/`BigUint`, so the allowance was dead. The lint
  itself is not weakened (the deny stays; an allowance was removed, which can only make it
  stricter).
- Gates run at head (CARGO_TARGET_DIR in scratchpad, RUST_TEST_THREADS=4): `cargo fmt --all --
  --check` ok; `make lint` (clippy `--all-targets --all-features -D warnings` plus the
  footprint thumbv7em release clippy) exit 0; `make test-features` "verified 14 feature sets",
  including the `+1.98.1 --no-default-features --features exact --target thumbv7em-none-eabi`
  no_std build; `make size` 973 bytes, 0 panic references, footprint graph holds no shared
  exact dependency, identical to base 95a27602 (973 bytes); `make deny` with
  `QUIRE=/home/peter/dev/quire-cli/target/release/quire` (quire-cli 374d508 build): bans,
  licenses, sources ok, `Git lock coherence checked: Cargo.lock`. Aggregate gate and Kani not
  run (lead's single final gate). `verification/kani.rs` is `cfg(kani)` and names none of the
  crates by path, so the removal cannot affect its compilation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The four crates are unused in every target, test, doctest, build script and feature; the
removal is complete and minimal across Cargo.toml, Cargo.lock and clippy.toml; the resolved
graph and feature sets are otherwise byte-identical; the exact no_std thumbv7em build and the
default footprint (973 bytes, 0 panic references) are unchanged; no lint, deny rule, CI lane or
feature gate is weakened. Mergeable.
