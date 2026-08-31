# =============================================================================
# Quire Contract Runtime Makefile
# =============================================================================

override CARGO := cargo
override MSRV := 1.75.0
override PYTHON := python3
FOOTPRINT_TARGET := thumbv7em-none-eabi
FOOTPRINT_TARGET_DIR := target/footprint-msrv

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make test-features    - test every supported feature set"
	@echo "  make doc              - warning-denied docs for runtime and footprint"
	@echo "  make build            - Release build"
	@echo "  make msrv             - Check all targets and features with Rust $(MSRV)"
	@echo "  make size             - Enforce the 4 KiB linked footprint on $(FOOTPRINT_TARGET)"
	@echo "  make spec             - Quire-validate the specification"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo deny check licenses"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make audit-panic      - Reject intentional panic paths in runtime source"
	@echo "  make evidence-tool    - Test the local evidence toolchain and PGM-01 pin"
	@echo "  make verify-evidence  - Verify the anchored retained evidence set"
	@echo "  make coverage         - Run strict repository-owned traceability classification"
	@echo "  make kani             - Run the complete declared Kani harness set (required)"
	@echo "  make update-evidence-anchors - Regenerate evidence/ANCHORS for review"
	@echo "  make ci               - All local CI gates"

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
	$(CARGO) test --no-default-features

.PHONY: test-features
test-features:
	$(CARGO) test --no-default-features
	$(CARGO) test --features alloc
	$(CARGO) test --features std
	$(CARGO) test --all-features
	$(CARGO) test -p quire-contract-runtime-footprint

.PHONY: doc
doc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime --all-features --no-deps
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime-footprint --no-deps --target $(FOOTPRINT_TARGET)

.PHONY: build
build:
	$(CARGO) build --release --no-default-features

.PHONY: msrv
msrv:
	$(CARGO) +$(MSRV) check --all-targets --all-features

.PHONY: size
size:
	CARGO_TARGET_DIR=$(FOOTPRINT_TARGET_DIR) $(CARGO) +$(MSRV) build --locked --release --manifest-path measurement/footprint/Cargo.toml --target $(FOOTPRINT_TARGET)
	bash scripts/check_linked_footprint.sh $(FOOTPRINT_TARGET_DIR)/$(FOOTPRINT_TARGET)/release/libquire_contract_runtime_footprint.a

.PHONY: spec
spec:
	quire validate --scope . 'spec/**/*.md' 'planning/**/*.md' 'plan/**/*.md'

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

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

.PHONY: audit-panic
audit-panic:
	bash scripts/check_panic_surface.sh

.PHONY: evidence-tool
evidence-tool:
	$(PYTHON) -m py_compile scripts/build_evidence_envelope.py scripts/check_coverage_status.py scripts/check_kani_harnesses.py scripts/update_evidence_anchors.py scripts/validate_json_schema.py scripts/verify_evidence.py
	$(PYTHON) -m unittest discover -s tests -p '*.py'

.PHONY: verify-evidence
verify-evidence:
	$(PYTHON) scripts/verify_evidence.py

.PHONY: coverage
coverage:
	$(PYTHON) scripts/check_coverage_status.py

.PHONY: kani-census
kani-census:
	$(PYTHON) scripts/check_kani_harnesses.py

.PHONY: kani
kani: kani-census
	@if command -v cargo-kani >/dev/null 2>&1; then \
		$(CARGO) kani; \
	else \
		echo "KANI_STATUS=unavailable; cargo-kani is required for this gate" >&2; \
		exit 2; \
	fi

.PHONY: update-evidence-anchors
update-evidence-anchors:
	$(PYTHON) scripts/update_evidence_anchors.py

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check spec lint test-features doc msrv size deny audit-unsafe audit-panic coverage kani evidence-tool verify-evidence
