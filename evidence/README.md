# Retained evidence

Run `scripts/collect_evidence.sh` from the repository root. Each output preserves stdout and stderr
separately, along with source/tool identities and SHA-256 digests. A missing optional Kani installation
is recorded as `skipped-unavailable`; it is never represented as successful proof evidence.

Candidate output supports the human release decision described by `spec/assurance/MP-001`; it is not
itself a release approval.

