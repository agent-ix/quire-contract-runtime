---
id: PLAN-015
title: "Bounded runtime snapshot implementation and qualification"
type: Plan
---
# Bounded runtime snapshot implementation and qualification

Approved direction: REV-008 at `5a1a221af7b4bdaf0cb65af15095de1f872ef9fc`.

## Sequence

1. Commit owning requirements/interface/feature/test matrix and independent native test bank
   before production implementation. Initial tests must fail because the API is absent.
2. Add allocation-free immutable snapshot; add optional fixed-shape JSON visitors and bounded
   encoder. Keep all mutable report constructors and recorded-event semantics unchanged.
3. Update exact public-surface guard and explicit feature-producer population without weakening
   bypass probes. Execute stable, exact Rust 1.75, isolated no_std-target, docs, licenses,
   panic/unsafe and governed default footprint controls.
4. Independently review immutable commit before downstream pin changes. Preserve unavailable
   Kani/shared-release gates and do not turn transport completion into codegen issue closure.

## Dependency and resource review before incorporation

The existing lock contains serde 1.0.229, serde_core 1.0.229 and serde_json 1.0.151.
Their inspected published Cargo manifests declare respectively Rust 1.56/1.56/1.71 and
MIT OR Apache-2.0. serde_json's locked numeric formatter zmij 1.0.23 declares Rust 1.71
and MIT. Exact optional normal dependencies will select the already locked serde and
serde_json versions with default-features disabled and alloc enabled. No derive dependency
or parser/template source is copied. Cargo.lock retains registry checksums; license/MSRV
execution remains required rather than inferred from declarations.

Pinned serde_json SliceRead parses unescaped strings by borrowing input and escaped strings
through one reusable Vec scratch buffer. Its read.rs parse_str_bytes and parse_escape paths
append decoded bytes from the capped input; fixed visitors will not recurse into arbitrary
unknown values or build a Value tree. Owned identity copies are checked at 4096 bytes first.
This bounds data volume but not allocator metadata, allocation growth policy or recoverable
OOM. The encoder will use a capped Vec-backed fmt::Write sink, with conservative preflight;
it will not call unbounded serde_json::to_vec before checking output size.

## Open integration gates

Generated terminal transport, exhaustive package/run/candidate joins, codegen pin promotion,
native execution authentication and shared release sufficiency belong to later owners.
No parser proof or codec footprint is attributed to the old fixed-population Kani/size runs.
