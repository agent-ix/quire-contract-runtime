# =============================================================================
# Quire Contract Runtime Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, Kani for the proofs, the footprint measurer for MP-001,
# quire for static export, quoin for evidence. Nothing here computes a verdict,
# attests to its own correctness, or retains evidence of its own.
#
# This file is not a trust root and no longer tries to be one. The gates that
# used to police Make's own execution controls — the ambient-flag guard, the
# recipe-failure-propagation prover, the `override` fence around HOME and CARGO —
# went with the collector they were protecting. Where a producer needs a
# toolchain it can trust it resolves one from the password database itself, which
# is a property of the producer rather than of the build system that invoked it.
#
# READ THIS BEFORE TRUSTING A GREEN `make ci`.
#
# `.IGNORE:` added to this file, a `-` prefix on a recipe line, or
# `SHELL := /bin/true` makes every recipe report success without its exit status
# being consulted. Measured on this repository, not assumed: with a rustfmt
# violation, a failing test and a renamed Kani harness all present, the control
# tree exits 2 at `fmt-check` and a tree with `.IGNORE:` prepended exits 0 from
# `make ci`. Six of the fourteen prerequisites still printed a diagnostic; none
# of them failed the build.
#
# What that does and does not reach. Quoin binds retained inputs by digest and
# `scripts/assurance_chain.py` derives every attested result from the producer's
# own bytes, so a Makefile that lies about running a producer yields an absent or
# unreadable input and the chain errors rather than passing. The gates that feed
# nothing into the chain — `fmt-check`, `lint`, `doc`, `deny`, `audit-unsafe`,
# `audit-panic`, `size` — are simply neutered.
#
# `tests/shared_assurance.rs` asserts this file declares no such directive, which
# protects a reviewer reading a diff. It does not make this file's exit code
# trustworthy on a tree where it has been edited, because under `.IGNORE:` that
# test also runs, also fails, and is also swallowed. Tracked as
# agent-ix/quire-contract-runtime#10.

CARGO ?= cargo
PYTHON ?= python3
QUIRE ?= quire
QUOIN ?= quoin

# The shared-assurance lane runs in its own interpreter. engineering-assurance
# declares jsonschema>=4.23 and this repository's retired Draft 7 lane pinned
# 3.2.0; both were right for their own job, so they get one environment each.
ASSURANCE_VENV ?= .venv-assurance
ASSURANCE_PYTHON ?= $(ASSURANCE_VENV)/bin/python

MSRV := 1.75.0
FOOTPRINT_TARGET := thumbv7em-none-eabi

ASSURANCE_DIR := target/assurance
FEATURE_RESULT := $(ASSURANCE_DIR)/feature-matrix.json
KANI_RESULT := $(ASSURANCE_DIR)/kani-proofs.json
KANI_MUTATION_RESULT := $(ASSURANCE_DIR)/kani-mutations.json
FOOTPRINT_RESULT := $(ASSURANCE_DIR)/footprint.json
QUIRE_EXPORT := $(ASSURANCE_DIR)/quire-static-export.json
COMPAT_RESULT := $(ASSURANCE_DIR)/legacy-compatibility.json
MSRV_RESULT := $(ASSURANCE_DIR)/msrv.jsonl
REVISION ?= $(shell git rev-parse HEAD)

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test plus the shared-assurance tests"
	@echo "  make test-features    - test every supported feature set"
	@echo "  make doc              - warning-denied docs for runtime and footprint"
	@echo "  make build            - Release build"
	@echo "  make msrv             - Check all targets and features with Rust $(MSRV)"
	@echo "  make size             - Measure the linked $(FOOTPRINT_TARGET) footprint"
	@echo "  make spec             - Validate and cover the specification with Quire"
	@echo "  make clean            - cargo clean and drop the assurance environment"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make audit-panic      - Reject intentional panic paths in runtime source"
	@echo "  make kani-census      - Require the declared harness census in source"
	@echo "  make kani             - Run the proofs; an absent toolchain fails closed"
	@echo "  make kani-mutations   - Require injected defects to fail their owning proofs"
	@echo "  make assurance-env    - create the pinned shared-assurance interpreter"
	@echo "  make assurance-inputs - run the producers and write their structured results"
	@echo "  make pins             - classify the toolchain through the shared matrix"
	@echo "  make compat-view      - read retained evidence through the shared mapping"
	@echo "  make assurance-chain  - seal, retain, and verify through Quoin"
	@echo "  make assurance        - pins + compat-view + assurance-chain"
	@echo "  make ci               - All CI gates locally (hosted CI is manual-only)"

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy -p quire-contract-runtime --all-targets --all-features -- -D warnings
	$(CARGO) clippy -p quire-contract-runtime-footprint --lib --release --target $(FOOTPRINT_TARGET) -- -D warnings

# The traced tests invoke the assurance gates, so the producers must already have
# run. They are a prerequisite rather than something a test creates for itself: a
# test that can produce its own inputs can produce a green run out of nothing.
.PHONY: test
test: assurance-inputs
	$(CARGO) test --all-features

# The feature matrix has one definition, in the producer that publishes it. Two
# copies of a matrix — one in Make and one in the tool that reports on it — is
# two matrices, and the reported one would eventually stop being the run one.
.PHONY: test-features
test-features:
	$(PYTHON) scripts/run_feature_matrix.py

.PHONY: doc
doc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime --all-features --no-deps
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime-footprint --no-deps --target $(FOOTPRINT_TARGET)

.PHONY: build
build:
	$(CARGO) build --release --no-default-features

.PHONY: msrv
msrv:
	rustup run $(MSRV) $(CARGO) check --locked --all-targets --all-features

.PHONY: size
size:
	$(PYTHON) scripts/measure_footprint.py

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' \
		'reviews/**/*.md' --summary
	$(QUIRE) coverage --scope . --strict

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(ASSURANCE_VENV)

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check licenses

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

.PHONY: audit-panic
audit-panic:
	bash scripts/check_panic_surface.sh

# =============================================================================
# Proofs
#
# `kani` fails closed. On a machine without cargo-kani the producer emits
# `unavailable` rows and this target exits non-zero, because `make ci` returning
# 0 must mean the proofs ran here, not that someone else's proofs are still on
# disk. `kani-census` is the cheap static half: it catches a deleted, renamed or
# cfg-ed-out harness without needing the model checker at all.
# =============================================================================

.PHONY: kani-census
kani-census:
	$(PYTHON) scripts/check_kani_harnesses.py

.PHONY: kani
kani: kani-census
	$(PYTHON) scripts/run_kani_gate.py

.PHONY: kani-mutations
kani-mutations:
	$(PYTHON) scripts/check_kani_mutations.py

# =============================================================================
# Shared assurance
# =============================================================================

$(ASSURANCE_PYTHON):
	$(PYTHON) -m venv $(ASSURANCE_VENV)
	$(ASSURANCE_VENV)/bin/pip install --quiet --disable-pip-version-check \
		-r requirements-assurance.txt

.PHONY: assurance-env
assurance-env: $(ASSURANCE_PYTHON)

# The only target that runs a producer. Everything downstream consumes these
# files and refuses to create them. Each command below is the exact argv the
# corresponding proof obligation declares in assurance/change-assurance.json; a
# declared command that is not the executed command is a lie in a sealed
# attestation.
.PHONY: assurance-inputs
assurance-inputs: assurance-env
	mkdir -p $(ASSURANCE_DIR)
	$(PYTHON) scripts/run_feature_matrix.py --json > $(FEATURE_RESULT)
	$(PYTHON) scripts/run_kani_gate.py --json > $(KANI_RESULT)
	$(PYTHON) scripts/check_kani_mutations.py --json > $(KANI_MUTATION_RESULT)
	$(PYTHON) scripts/measure_footprint.py --json > $(FOOTPRINT_RESULT)
	$(QUIRE) coverage --scope . --json > $(QUIRE_EXPORT)
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py --json > $(COMPAT_RESULT)
	rustup run $(MSRV) $(CARGO) check --locked --all-targets --all-features \
		--message-format=json > $(MSRV_RESULT)

.PHONY: pins
pins: assurance-env
	$(ASSURANCE_PYTHON) scripts/check_shared_pins.py

.PHONY: compat-view
compat-view: assurance-env
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py
	$(ASSURANCE_PYTHON) scripts/legacy_evidence_view.py --mutation-probes

.PHONY: assurance-chain
assurance-chain: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --candidate-revision $(REVISION)

.PHONY: assurance
assurance: pins compat-view assurance-chain

# An operator target, not a CI gate. It writes into this repository's own Quoin
# evidence store, which is a reviewed change to spec/evidence/ rather than
# something a gate should do on every run.
.PHONY: assurance-record
assurance-record: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --adapt $(KANI_RESULT) \
		> $(ASSURANCE_DIR)/entries.json
	$(QUOIN) evidence record \
		--repo . \
		--suite SUITE-001 \
		--commit $(REVISION) \
		--tool "quire-contract-runtime-kani-proofs 0.1.0" \
		--adapter entries \
		--kind Analysis \
		--results $(ASSURANCE_DIR)/entries.json

# =============================================================================
# Composite
# =============================================================================

.NOTPARALLEL: ci
.PHONY: ci
ci: fmt-check spec lint test-features doc msrv size deny audit-unsafe audit-panic \
	kani kani-mutations test assurance
