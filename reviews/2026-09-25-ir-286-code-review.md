---
id: SR-010
title: "Code review (rust-review lane) — IR-286 concrete-path representation fixes and Kani metadata flag"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-runtime@9f0071a12904081c7175730071d1c4bf65eb9c18; PR #79 (Linear IR-286) diff against origin/main 23fbb13: Cargo.toml, Makefile, src/exact/{accounting,composite,decimal,enumeration,expression,integer,numeric,outcome,quantity,rational,reference,text,unit}.rs, tests/exact_semantics.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: reviews
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/TC-023
    type: references
  - target: ix://agent-ix/quire-contract-runtime/NFR-002
    type: references
---

# SR-010: Code review (rust-review lane) — IR-286 concrete-path representation fixes

## Summary

Ticket: IR-286. PR agent-ix/quire-contract-runtime#79, reviewed at `9f0071a`, 12 commits, 16 files,
+687/-293. The change ports the IR-286 Kani spike's representation fixes to RT main: `Integer` held
as `{ small: i64, big: Option<Box<BigInt>> }` with `i128` fast paths for `+ - *` and unary `-`, a
sign-only mixed compare, `#[repr(u64)]` tags on `Value`, `ValueType` and `Outcome`, the fields of
nine value/type structs and `CheckedPackage` moved behind one `Box`, `CheckedPackage::enter` returning
`Option`, `evaluate_integer_arithmetic` and the call path building `Outcome` directly, the Kani
`--max-field-sensitivity-array-size 1024` metadata flag, and `make kani` under `ulimit -s unlimited`.

What was checked independently, not taken from the PR body:

- **Integer's canonical invariant.** Every construction site was read: `small`, `big`, `from_big`,
  `From<i64>`, `From<i128>`, `From<u64>`, `Default`, `FromStr` (via `from_big`). `big()` is reached
  only from branches where the value failed `i64::try_from`, so a value that fits `i64` is never
  promoted and `small` is always zero when `big` is `Some`. There is no serde on `Integer` (serde is
  used only by `snapshot_json.rs`, which never names `Integer`), so no deserialization path exists.
  A reviewer probe (temporary, not committed) built 16 boundary values (`0`, `±1`, `i64::MAX`,
  `i64::MAX±1`, `i64::MIN`, `i64::MIN±1`, `±2^64`, `2^64-1`, `±2^100`, `MIN*MIN`, `MIN*MAX`) along
  four independent paths — `From<i128>`, decimal parse, `Add`/`Subtract` across the boundary with six
  offsets, `Negate` — and a `Rational` reduction, and asserted equal values and equal `Hash`. It
  passed. `cmp`, `==`, `is_zero`, `is_negative`, `is_even`, `to_u64`, `magnitude_bits` and
  `to_string` matched an `i128` oracle over all 16×16 pairs.
- **i64 boundaries.** `i128` holds every `+ - *` of two `i64` operands and `-i64::MIN`, so the
  `wrapping_*` calls never wrap. `magnitude_bits(i64::MIN)` is 64. `abs(i64::MIN)` promotes. The
  mixed compare is correct given the invariant: a promoted value lies outside `i64`, so its sign
  alone orders it.
- **Public API and serialization.** No `pub` signature, variant or field changed. `Debug` was
  compared byte for byte against `origin/main`: the same probe rendered `{:?}` and `{:#?}` of all 16
  integers plus `Rational`, `Decimal`, `IntegerInterval`, `RationalDomain`, `Outcome<Integer>` and
  `Integer::default()` on both trees, 76 lines, identical. The nine hand-written `Debug` impls list
  the same fields in the same order as their `*Fields` structs. Layout changes (`#[repr(u64)]` on
  three public enums, `Integer` 40 → 16 bytes) and changed `Hash` values are disclosed in the PR body.
- **Semantics of the `Result` → `Outcome` rewrites.** `Outcome::from_stop(Err(Stop::X))` maps to
  `Outcome::X` one to one, and `charge_call` / `meter.charge` return `Incomplete` wrapped by
  `From<Incomplete> for Stop`, so `run_call`, `run_evaluate`, `Frame::call` and
  `evaluate_integer_arithmetic` keep their check order and their outcomes. The `accounting.rs` loop
  now borrows and clones the amount on the refusal path only.
- **Excluded items.** No `Outcome` payload boxing, no `verification/` change (see FND-008), no spike harness
  module, no `cfg(kani)` accessor, no `dbg!`/`println!`/`TODO`/`#[allow]`/`unsafe` in added lines.
- **Mutation testing.** Eleven single-point mutants of `src/exact/integer.rs`, each run against the
  exact-feature integration suites and the lib tests (log `mutations.log`). Nine were killed, two
  survived — see FND-001 and FND-003.

| Mutant | Result |
|---|---|
| add / sub / mul / neg fast path wraps in `i64` instead of `i128` | killed (tc_023 i128 oracle, 1–6 tests) |
| swap `Less`/`Greater` in the `(Small, Big)` compare arm | killed (7 tests) |
| swap `Less`/`Greater` in the `(Big, Small)` compare arm | killed (8 tests) |
| `from_big` always promotes (non-canonical) | killed (11 tests) |
| `big_from_i128` drops the top 32-bit digit | killed (8 tests) |
| `magnitude_bits` small path off by one | killed (11 tests) |
| `big_from_i128` drops the sign | **survived** |
| `abs(i64::MIN)` returns `i64::MIN` | **survived** |

## Verdict

**APPROVE with non-blocking findings.** The Integer representation is canonical on every path, the
fast paths are exact at the `i64` boundaries, `Debug` is unchanged, and no public signature or
serialization form moved. FND-001 is a test-oracle weakness, not a live defect: the reviewer probe
checked `big_from_i128` against an independent decimal-parse oracle, and it is correct today. Kani
(8/8 above floor) and `make lint` are green at the reviewed head.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | The tc_023 `i128` oracle is not independent for results outside `i64`. Its expected value is `int(expected)` = `Integer::from(i128)`, which runs the same `big_from_i128` the fast path uses. Mutating `big_from_i128` to drop the sign (`let sign = Sign::Plus;`) survives every exact-feature suite. Failure scenario: `i64::MIN + i64::MIN` then returns `+2^64`, and tc_023's expected `int(-2^64)` is also `+2^64`, so the test stays green. Build the expected value from an independent source, e.g. `expected.to_string().parse::<Integer>()`. | tests/exact_arithmetic.rs:46, tests/exact_arithmetic.rs:557, src/exact/integer.rs:464 |
| FND-002 | medium | The Kani-motivated layout rules exist only as comments, and no in-tree check depends on them: explicit `u64` tags, no inline `Value` payload larger than `Integer`, no inline `ValueType` payload larger than `IntegerInterval`. None of the 8 declared harnesses reaches `Value`/`Frame::call`, and the spike harness is excluded by design. Failure scenario: a later PR adds a 24-byte inline variant payload, or drops the `Box` from `Rational`. `make ci` stays green, and the next harness through `Frame::call` again fails to fold. A `const _: () = assert!(size_of::<T>() <= size_of::<Integer>())` per boxed type, or one landed harness, would guard it. | src/exact/composite.rs:43-49, src/exact/composite.rs:144-150, src/exact/rational.rs:12-16, Cargo.toml:50-55 |
| FND-003 | low | The `abs(i64::MIN)` promotion branch is untested. Mutating it to `Self::small(*value)` (which returns `i64::MIN` as its own magnitude) survives every exact-feature suite. Failure scenario: a regression there gives a negative magnitude to the `decimal.rs:452` and `quantity.rs:480` callers, and no test notices. | src/exact/integer.rs:156-160 |
| FND-004 | low | `ulimit -s unlimited` wraps only `make kani`. `make assurance-inputs` runs the same `run_kani_gate.py` to produce the attested `kani-proofs.json`, and it runs `check_kani_mutations.py`, both without it. `make kani-mutations` also runs without it. Failure scenario: a harness that needs the raised stack passes `make kani` but produces a `fail` row in the attested document, or passes locally and fails in the chain. Separately, `ulimit -s unlimited` exits non-zero where the hard limit is finite, which fails `make kani` before Kani runs. | Makefile:203, Makefile:207, Makefile:230-231 |
| FND-005 | low | Nine derived `Debug` impls became hand-written: `Rational`, `RationalDomain`, `Decimal`, `DecimalType`, `Quantity`, `Text`, `EnumValue`, `ObjectReference`, `CompoundUnit`. Debug rendering is FR-007-AC-6's conformance oracle, and the conformance crate does not compile on `origin/main` or here. Failure scenario: a field added to a `*Fields` struct but not to its `Debug` impl disappears from the rendering, and no gate notices. All nine match today; this was checked field by field. | src/exact/rational.rs:25, src/exact/rational.rs:276, src/exact/decimal.rs:84, src/exact/decimal.rs:351, src/exact/quantity.rs:94, src/exact/text.rs:328, src/exact/enumeration.rs:110, src/exact/reference.rs:86, src/exact/unit.rs:436 |
| FND-006 | low | Integer's canonical form is kept by construction discipline alone. `Integer::big` states its precondition in a doc comment and does not check it. Derived `Eq`/`Hash` and the sign-only `Ord` are silently wrong for any value that breaks it. Failure scenario: a future in-module helper calls `Self::big(BigInt::from(5))`. `Integer::from(5) != that`, the two hash differently, and `cmp` against `6` says `Greater`. A `#[cfg(test)]` unit test of the representation at the boundaries, or routing every promotion through `from_big`, would close it. | src/exact/integer.rs:29-56, src/exact/integer.rs:483 |
| FND-008 | low | The PR leaves a stale claim in a file it did not touch. `verification/kani.rs`'s module comment still says `exact::Integer` "is an unbounded `BigInt`": a harness built from a symbolic operand, even one narrowed to `i8`, drives every step into `num-bigint`'s digit vectors and does not discharge. This PR exists to make that false. An operand that fits `i64` is now held inline, and the fast paths never touch `BigInt`. Failure scenario: the next harness author reads the comment and rejects the `Integer` harness this PR enables, or a reviewer takes the comment as the reason the eight harnesses avoid `Integer`. | verification/kani.rs:15-24, verification/kani.rs:266-268 |
| FND-007 | low | Style note, no failure scenario. The small-path bit-length expression (`unsigned_abs().checked_ilog2().map_or(1, …+1)`) is written twice, once in `magnitude_bits` and once in `magnitude_bits_integer`. | src/exact/integer.rs:92-114 |

## Gates

Run by this reviewer at `9f0071a`, with `CARGO_BUILD_JOBS=4`. Each Kani harness ran alone under
`systemd-run --user --scope -p MemoryMax=16G` and `ulimit -s unlimited`.

| Gate | Result |
|---|---|
| `make lint` | exit 0 |
| `cargo kani --features exact --harness kani_proofs::<h> --exact` ×8 | all exit 0, all at or above their obligation floors (140, 136, 264, 59, 2561−110 unreachable = 2451 ≥ 2417, 43, 52, 24) |
| `make assurance-inputs` (capped, `ulimit -s unlimited`) | exit 0; `kani-proofs.json` 8/8 `pass` plus `suite-census` `pass` |
| `cargo test --all-features` | exit 101. Every suite passes except `shared_assurance` `tc_009_every_shared_pin_is_classified_by_the_packaged_matrix`, which hits pin drift (installed quire-cli 0.33.0 against the pinned 0.31.0), and `tc_010_the_chain_never_executes_a_producer_and_the_probe_can_prove_it` (`stubbing quire produced no invocation`, `shared_assurance.rs:584`). Both fail identically at the same lines on `origin/main` 23fbb13 after its own `make assurance-inputs` (pre-existing) |
| `make spec` | exit 2, identical on `origin/main` 23fbb13 (pre-existing) |
| `make conformance` | exit 2. Built with `--keep-going`, the same 9 test binaries fail to compile on the same 4 `E0004` sites on `origin/main` and here (pre-existing) |
