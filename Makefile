# =============================================================================
# Quire Contract Runtime Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, Kani for the proofs, binutils for the footprint, quire
# for specification validation.

CARGO ?= cargo
# The QSL agreement package depends on quire-contract-model (rustc 1.98.1). rustup
# resolves the toolchain from the working directory, not the manifest, so name it here.
QSL_AGREEMENT_TOOLCHAIN ?= 1.98.1
PYTHON ?= python3
QUIRE ?= quire

MSRV := 1.75.0
FOOTPRINT_TARGET := thumbv7em-none-eabi
FOOTPRINT_TARGET_DIR := target/footprint-msrv

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test with all features"
	@echo "  make test-features    - test every supported feature set"
	@echo "  make test-ignored     - run #[ignore]d tests, to re-detect a cleared blocker"
	@echo "  make conformance      - exact oracles against the QSL authority"
	@echo "  make doc              - warning-denied docs for runtime and footprint"
	@echo "  make build            - Release build"
	@echo "  make msrv             - Check all targets and features with Rust $(MSRV)"
	@echo "  make size             - Measure the linked $(FOOTPRINT_TARGET) footprint"
	@echo "  make spec             - Validate and cover the specification with Quire"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make kani             - Run the Kani proofs"
	@echo "  make kani-mutations   - Require injected defects to fail their owning proofs"
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

.PHONY: test
test:
	$(CARGO) test --all-features

.PHONY: test-features
test-features:
	$(PYTHON) scripts/run_feature_matrix.py

# `#[ignore]`d tests are evidence blocked on an open defect, not dropped
# coverage: this is the only target that runs them, so a fix that clears the
# blocking defect is re-detected here rather than staying silently ignored.
.PHONY: test-ignored
test-ignored:
	$(CARGO) test --all-features -- --ignored

# Exact-oracle agreement against the quire-spec-language authority. A separate
# package, so the runtime's own dependency graph never contains it.
.PHONY: conformance
conformance:
	$(CARGO) +$(QSL_AGREEMENT_TOOLCHAIN) test --locked --release --manifest-path conformance/qsl-agreement/Cargo.toml

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

# Link the footprint staticlib on the MSRV compiler, then measure it.
.PHONY: size
size:
	$(CARGO) +$(MSRV) build --locked --release --manifest-path measurement/footprint/Cargo.toml \
		--target $(FOOTPRINT_TARGET) --target-dir $(FOOTPRINT_TARGET_DIR)
	bash scripts/check_linked_footprint.sh

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md' \
		'reviews/**/*.md' --summary
	$(QUIRE) coverage --scope . --strict

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check licenses

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

# =============================================================================
# Proofs
#
# `cargo kani` exits non-zero when a proof fails, and cargo itself exits
# non-zero when `cargo-kani` is not installed, so a green `make kani` means the
# proofs ran here.
# =============================================================================

# CBMC segfaults on some harnesses under the field-sensitivity metadata flag
# without an unlimited stack. `ulimit -s unlimited` itself fails where the
# hard limit is finite, so the fallback raises the soft limit to the hard one
# instead of leaving it unset.
KANI_STACK := bash -c 'ulimit -s unlimited 2>/dev/null || ulimit -s "$$(ulimit -H -s)"; exec "$$@"' --

.PHONY: kani
kani:
	$(KANI_STACK) $(CARGO) kani --features exact

.PHONY: kani-mutations
kani-mutations:
	$(KANI_STACK) $(PYTHON) scripts/check_kani_mutations.py

# =============================================================================
# Composite
# =============================================================================

.NOTPARALLEL: ci
.PHONY: ci
ci: fmt-check spec lint test-features conformance doc msrv size deny kani kani-mutations test
