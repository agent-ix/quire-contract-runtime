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

MSRV := 1.82.0
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
	@echo "  make deny-mutations   - Require each banned QSL crate to fail cargo-deny in a scratch copy"
	@echo "  make use-local        - patch first-party git deps to sibling checkouts"
	@echo "  make use-remote       - Remove the local patch file and restore Cargo.lock; build from GitHub"
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
# `use-local` writes a gitignored .cargo/config.toml that patches each
# first-party git dependency to its working tree at $(SIBLINGS)/<repo>, uncommitted
# edits included. `use-local` first snapshots Cargo.lock to the gitignored
# .cargo/Cargo.lock.pre-local; `use-remote` deletes the config and restores the
# lock from that snapshot (and does nothing to the lock if there is none). So the
# lock returns to its state before the first `use-local`; lock changes made while
# a patch is active are discarded, and a `cargo update -p` made without a patch is kept.
# Format: <repo>:<crate>:<crate-dir>; entries are grouped by repo here, in any
# order, so each repo gets exactly one [patch] table.
# SIBLINGS is the directory holding the sibling clones: the parent of the main
# checkout, so it is also right from a linked worktree. Override to relocate.
# =============================================================================

SIBLINGS ?= $(abspath $(shell git rev-parse --path-format=absolute --git-common-dir)/../..)
LOCAL_PATCHES ?=

.PHONY: use-local
use-local:
	@set -e; mkdir -p .cargo; \
	for spec in $(LOCAL_PATCHES); do \
	  if [ "$$(printf '%s' "$$spec" | tr -cd ':' | wc -c)" != 2 ] || printf '%s' "$$spec" | grep -q '::\|^:\|:$$'; then \
	    echo "use-local: malformed LOCAL_PATCHES entry '$$spec' (want repo:crate:dir)" >&2; exit 1; \
	  fi; \
	  repo=$${spec%%:*}; rest=$${spec#*:}; dir=$${rest#*:}; \
	  if [ ! -f "$(SIBLINGS)/$$repo/$$dir/Cargo.toml" ]; then \
	    echo "use-local: $(SIBLINGS)/$$repo is not cloned (no Cargo.toml at $(SIBLINGS)/$$repo/$$dir); clone agent-ix/$$repo next to this repo" >&2; exit 1; \
	  fi; \
	done; \
	[ -f .cargo/Cargo.lock.pre-local ] || cp Cargo.lock .cargo/Cargo.lock.pre-local; \
	: > .cargo/config.toml; \
	repos=$$(for spec in $(LOCAL_PATCHES); do printf '%s\n' "$${spec%%:*}"; done | awk '!seen[$$0]++'); \
	first=1; \
	for repo in $$repos; do \
	  [ "$$first" = 1 ] || printf '\n' >> .cargo/config.toml; first=0; \
	  printf '[patch."https://github.com/agent-ix/%s"]\n' "$$repo" >> .cargo/config.toml; \
	  for spec in $(LOCAL_PATCHES); do \
	    [ "$${spec%%:*}" = "$$repo" ] || continue; \
	    rest=$${spec#*:}; crate=$${rest%%:*}; dir=$${rest#*:}; \
	    printf '%s = { path = "%s/%s/%s" }\n' "$$crate" "$(SIBLINGS)" "$$repo" "$$dir" >> .cargo/config.toml; \
	  done; \
	done; echo "wrote .cargo/config.toml"; \
	meta=$$(mktemp); \
	if ! $(CARGO) metadata --format-version 1 >/dev/null 2>"$$meta"; then \
	  cat "$$meta" >&2; rm -f "$$meta" .cargo/config.toml; \
	  [ ! -f .cargo/Cargo.lock.pre-local ] || { cp .cargo/Cargo.lock.pre-local Cargo.lock; rm -f .cargo/Cargo.lock.pre-local; }; \
	  echo "use-local: cargo metadata failed under the patch" >&2; exit 1; \
	fi; \
	if grep -q 'patch .* was not used' "$$meta"; then \
	  cat "$$meta" >&2; rm -f "$$meta" .cargo/config.toml; \
	  [ ! -f .cargo/Cargo.lock.pre-local ] || { cp .cargo/Cargo.lock.pre-local Cargo.lock; rm -f .cargo/Cargo.lock.pre-local; }; \
	  echo "use-local: a patch was not used; the sibling's version does not satisfy the requirement" >&2; exit 1; \
	fi; \
	rm -f "$$meta"

.PHONY: use-remote
use-remote:
	rm -f .cargo/config.toml
	@if [ -f .cargo/Cargo.lock.pre-local ]; then mv .cargo/Cargo.lock.pre-local Cargo.lock; echo "restored Cargo.lock from the pre-local snapshot"; fi

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny --workspace check licenses bans sources
	@set -e; for f in $$(git ls-files -- 'Cargo.lock' '*/Cargo.lock'); do awk -f scripts/check_one_copy.awk $$f; done

# Mutation check of the QSL-crate bans (FR-275-AC-8, TC-198): each guarded crate added in a
# scratch copy must fail `cargo deny` with the `banned` diagnostic.
.PHONY: deny-mutations
deny-mutations:
	bash scripts/check_deny_bans.sh

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
ci: fmt-check spec lint test-features doc msrv size deny deny-mutations kani kani-mutations test
