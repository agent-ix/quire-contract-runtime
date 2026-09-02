# Schemas

Six files. Four are frozen and named by digest inside retained evidence; two are
the provenance of one of those four.

| File | Class |
|---|---|
| `runtime-evidence-input-v1.schema.json` | Frozen historical. Read never, written never. |
| `runtime-evidence-manifest-v1.schema.json` | Frozen historical. Read never, written never. |
| `pgm01-derivation-evidence-envelope-v1.schema.json` | Frozen historical. Vendored from `quire-contract-ir@7dac9d8c`. |
| `pgm01-validate-governance.py` | Frozen historical. Vendored governance validator. Never executed. |
| `pgm01-merged-commit.txt` | Provenance for the two files above: the upstream commit object. |
| `pgm01-validator-blob.txt` | Provenance for the validator: its upstream blob id. |

## Nothing validates against these any more

The collector, envelope builder, JSON-schema driver and verifier that used them
were removed by `agent-ix/quire-contract-runtime#8`. Quoin owns the record,
attestation, and receipt shapes now, and it ships them itself — a producer
validates against the same file the sealing code was written against rather than
against a local copy that has drifted. Engineering Assurance owns the read-only
mapping of retained bytes through
`engineering_assurance.verification_semantics.map_pgm01_bytes`.

`pgm01-validate-governance.py` is the only executable in this directory and it is
never run. It has no import site, no Make target, and no test.

## They are not deleted, and the reason is specific rather than sentimental

Every one of the 42 retained envelopes under `evidence/` names the two runtime
schemas, by id and by SHA-256:

```json
"inputs":  [{ "schema": { "id": "quire.runtime-evidence-input",    "version": "v1",
              "digest": { "value": "b7235394…04140e29" } } }],
"outputs": [{ "schema": { "id": "quire.runtime-evidence-manifest", "version": "v1",
              "digest": { "value": "0f8c78c4…3d1b7bdd" } } }]
```

Each envelope also names the PGM-01 envelope schema in its extension block
(`envelopeSchemaDigest: 0946e235…7152256`), and each retained record directory
carries a `pgm01-validator-sha256.txt` naming the validator
(`1c2881d5…3b58df2f1`).

Deleting any of them would not remove a generic evidence family from this
repository. The family was the *collector and the verifier*, and that is what
went. It would instead break a reference inside bytes the migration is required
to leave untouched.

## The freeze is enforced, not described

`tests/shared_assurance.rs::tc_014_no_local_evidence_framework_remains_and_the_frozen_schemas_bind_nothing`
pins all four by digest and asserts that no source file under `scripts/`,
`tests/`, `src/`, `verification/`, `measurement/`, `spec/`, `plan/` or `.github/`
— nor the `Makefile` — mentions any of their names. The census size is asserted
too, so the claim cannot pass by inspecting nothing.
