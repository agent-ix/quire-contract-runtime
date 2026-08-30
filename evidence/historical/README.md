# Historical evidence

Files here remain immutable records for earlier candidate revisions. They do not establish the state
of a rebased or subsequently amended candidate.

`aac4bee923aef78838b856118cd73aad3728226e-github-ci.txt` preserves what GitHub reported for that
exact pre-reconciliation revision, but its `msrv_1_75=SUCCESS` result is invalidated. The job selected
Rust 1.75 and then allowed `rust-toolchain.toml` to override it; reproducing the intended command as
`cargo +1.75.0 check --lib --no-default-features` at the equivalent source revision fails with four
`E0658` errors in `src/accounting.rs`. The retained `kani_0_67_0` result is historical evidence only
for its own revision. The later runtime head and its manual-CI stack require a fresh, deliberately
dispatched run before review or merge.
