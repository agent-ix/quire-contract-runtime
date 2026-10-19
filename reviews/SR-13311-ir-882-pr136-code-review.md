---
id: SR-13311
title: "IR-882 PR 136 code review (with Rust lane): slice 0 exact kernel lock bump"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@ef89b4f6a69a5a48d6466e75c1708fa11399bbc9 vs origin/main merge-base b4ea971082aa7c7c152af724c8a26755a855e34a; Cargo.lock, src/exact/division.rs, tests/support/charge_points.rs, tests/exact_*.rs"
review_set: subset
---

## Summary

Ticket: IR-882 (slice 0). PR agent-ix/quire-contract-runtime#136. The Rust lane is folded into this file. The diff (12 files, +138/-55) moves `Cargo.lock` to new first-party revisions, adds one test-only extension trait, renames `meter.admitted_charges()` to `meter.admitted_charge_points()` in seven test binaries, extends the TC-024 census test, and adds one item-scoped Clippy suppression in `src/exact/division.rs`.

Measured by the reviewer at the head, in a fresh worktree with its own target dir:

- `Cargo.lock` holds exactly one revision per first-party crate: quire-exact `1c6cbcb6`, quire-semantic-value `9fd13752`, quire-canonical and quire-canonical-derive `8b69bedc`, ix-trace-rs `9378a677` (unchanged). Each equals its repository's `main` by `git ls-remote` at review time. The only crates.io changes are two added packages, `allocator-api2` 0.2.21 and `once_cell` 1.21.4, both new dependencies of quire-exact at `1c6cbcb6` (its own `Cargo.toml`). No other `version` line changes.
- `Cargo.toml` is unchanged; every first-party dependency stays `branch = "main"`. No `[patch]`, path dependency, `.cargo/config.toml`, vendored file, new pin, digest or SHA record is added.
- `make fmt-check lint doc test-features size deny` and `make test` pass. The working tree stayed clean after the build, so the lock is consistent with the manifests. `make deny` (the one-copy check included) passes.
- Without the new `#[allow(clippy::large_enum_variant)]`, `cargo clippy --all-features --lib -D warnings` fails ("the entire enum is at least 288 bytes"), so the suppression is needed (reviewer probe, reverted).
- `tests/support/charge_points.rs` is a test-only trait, `ChargePoints for quire_exact::Meter`, that maps the kernel's `AdmittedCharge` log to its `point` field. It is not compiled into the library, re-exports nothing, and converts no value type, so it is not an AD-004 conversion layer or bridge. No `src/` file gains a `pub use`, `From` impl or value conversion.
- Tags are `#[trace(...)]` only; the diff adds no `/// Trace:` line. No test gains an FR-277-AC-1 tag.
- Kani and mutation lanes were not run (lead's unsandboxed lane).

## Examined criteria

- Cargo.lock first-party sources (examined): one `[[package]]` per agent-ix crate, each at upstream `main`.
- src/exact/division.rs:31-41 (examined): "// The kernel's larger IntegerInterval makes Finite exceed Clippy's size threshold. // Boxing this public variant would change existing consumer construction in a lock-only slice. #[allow(clippy::large_enum_variant)]"
- tests/support/charge_points.rs (examined): "pub trait ChargePoints { fn admitted_charge_points(&self) -> Vec<ChargePoint>; } impl ChargePoints for Meter { ... self.admitted_charges().iter().map(|charge| charge.point).collect() }"
- tests/exact_composite.rs:85-135 `tc_024_p1_charge_point_vocabulary_pins_kernel_census_and_spellings` (examined).
- The `admitted_charges` to `admitted_charge_points` call-site rewrites in tests/exact_collection.rs, exact_equality.rs, exact_function_application.rs, exact_meter_state.rs, exact_outcomes.rs, exact_quantity_conversion.rs (examined): each keeps its assertion; only the projection changes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new lint suppression uses `#[allow(clippy::large_enum_variant)]` with a `//` comment. On the 1.98.1 toolchain, `#[expect(clippy::large_enum_variant, reason = "...")]` records the same reason and fails the lint gate once it is no longer needed, for example after slice 1 or a kernel `IntegerInterval` change. A plain `allow` would then stay behind unnoticed. | src/exact/division.rs:31-33 |

## Verdict

**PASS: mergeable after the lead's Kani and mutation lanes.** The lock move is coherent: there is one revision per first-party crate, each equals upstream `main`, and nothing is patched, vendored, pinned or shimmed. The test adaptation is a projection only and weakens no assertion. The one low finding is optional hardening of the suppression's form, not a defect in behaviour.

## Dispositions

Review pass only; no disposition round has run yet.
