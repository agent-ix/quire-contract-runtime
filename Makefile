# =============================================================================
# Quire Contract Runtime Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, Kani for the proofs, binutils for the footprint, quire
# for specification validation.

CARGO ?= cargo
# --locked only when no local patch is active: a patch rewrites the resolution.
LOCKED ?= $(if $(wildcard .cargo/config.toml),,--locked)
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
	@echo "  make doc              - warning-denied docs for runtime and footprint"
	@echo "  make build            - Release build"
	@echo "  make msrv             - Check all targets and features with Rust $(MSRV)"
	@echo "  make size             - Measure the linked $(FOOTPRINT_TARGET) footprint"
	@echo "  make spec             - Validate and cover the specification with Quire"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo-deny licenses, bans and sources; one-copy lockfile check"
	@echo "  make use-local        - patch first-party git deps to sibling checkouts"
	@echo "  make use-remote       - drop the local patch file"
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

.PHONY: doc
doc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime --all-features --no-deps
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc -p quire-contract-runtime-footprint --no-deps --target $(FOOTPRINT_TARGET)

.PHONY: build
build:
	$(CARGO) build --release --no-default-features

.PHONY: msrv
msrv:
	rustup run $(MSRV) $(CARGO) check $(LOCKED) --all-targets --all-features

# Link the footprint staticlib on the MSRV compiler, then measure it.
.PHONY: size
size:
	$(CARGO) +$(MSRV) build $(LOCKED) --release --manifest-path measurement/footprint/Cargo.toml \
		--target $(FOOTPRINT_TARGET) --target-dir $(FOOTPRINT_TARGET_DIR)
	bash scripts/check_linked_footprint.sh

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' 'plan/**/*.md' \
		'reviews/**/*.md' --summary
	$(QUIRE) coverage --scope . --strict

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Local development against sibling checkouts
#
# Every first-party git dependency is `branch = "main"` with no rev or tag, so all
# consumers share one source spec and Cargo builds one copy. `use-local` patches each
# one to its sibling working tree (uncommitted edits included) through a gitignored
# .cargo/config.toml; `use-remote` deletes it. List each dependency as
# `<repo>:<crate>:<crate-dir-in-repo>`. RT currently has none, so the list is empty.
# =============================================================================

FIRST_PARTY_GIT_DEPS ?=
LOCAL_CARGO_CONFIG := .cargo/config.toml

.PHONY: use-local
use-local:
	@set -e; \
	for dep in $(FIRST_PARTY_GIT_DEPS); do \
		if [ "$$(printf '%s' "$$dep" | tr -cd ':' | wc -c)" != 2 ] || printf '%s' "$$dep" | grep -q '::\|^:\|:$$'; then \
			echo "use-local: malformed FIRST_PARTY_GIT_DEPS entry '$$dep' (want repo:crate:dir)" >&2; exit 1; \
		fi; \
		repo=$${dep%%:*}; rest=$${dep#*:}; dir=$${rest#*:}; \
		if [ ! -f "../$$repo/$$dir/Cargo.toml" ]; then \
			echo "use-local: ../$$repo/$$dir/Cargo.toml not found; clone agent-ix/$$repo next to this repo" >&2; \
			exit 1; \
		fi; \
	done; \
	mkdir -p .cargo; \
	{ echo "# Generated by make use-local; gitignored. make use-remote removes it."; \
	  for repo in $$(for dep in $(FIRST_PARTY_GIT_DEPS); do echo $${dep%%:*}; done | sort -u); do \
		echo "[patch.\"https://github.com/agent-ix/$$repo\"]"; \
		for dep in $(FIRST_PARTY_GIT_DEPS); do \
			[ "$${dep%%:*}" = "$$repo" ] || continue; \
			rest=$${dep#*:}; crate=$${rest%%:*}; dir=$${rest#*:}; \
			echo "$$crate = { path = \"../$$repo/$$dir\" }"; \
		done; \
	  done; } > $(LOCAL_CARGO_CONFIG); \
	echo "wrote $(LOCAL_CARGO_CONFIG)"; \
	if $(CARGO) metadata --format-version 1 2>&1 >/dev/null | grep -q 'patch .* was not used'; then \
		rm -f $(LOCAL_CARGO_CONFIG); echo "use-local: a patch was not used; the sibling's version does not satisfy the requirement" >&2; exit 1; \
	fi

.PHONY: use-remote
use-remote:
	rm -f $(LOCAL_CARGO_CONFIG)
	git ls-files -z -- 'Cargo.lock' '*/Cargo.lock' | xargs -0 -r git checkout --

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny --workspace check licenses bans sources
	@set -e; for f in $$(git ls-files -- 'Cargo.lock' '*/Cargo.lock'); do awk -F'"' -f scripts/check_one_copy.awk $$f; done

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
ci: fmt-check spec lint test-features doc msrv size deny kani kani-mutations test
