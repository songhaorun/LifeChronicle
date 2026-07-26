# Stream and Metric Registry v1

This directory is the version-controlled authority for LifeChronicle stream and
metric definitions.

## Layout

- `schemas/` contains the Draft 2020-12 meta-schemas. Every object schema sets
  `additionalProperties: false`; a misspelled field is therefore an error at
  any nesting level.
- `streams/<name>/v<schema.version>.yaml` contains published stream versions.
- `metrics/<name>/v<schema.version>.yaml` contains published metric versions.
- `templates/` is the copy source for new definitions.
- `fixtures/valid/` and `fixtures/invalid/` are executable acceptance examples.

The `.yaml` files use the JSON-compatible profile of YAML 1.2. This keeps the
documents valid YAML while letting the phase-0 validator use only Python's
standard library and avoid parser-dependent coercions.

## Invariants

- Names contain two to five lowercase dot-separated segments. A third-party
  name must use `plugin.<publisher>.<plugin>.<metric>`.
- The file path, registry name, and schema version must agree.
- Lifecycle is monotonic: `draft -> active -> deprecated -> retired`. A
  deprecated or retired definition remains in the registry.
- Every definition names an owner and every processor explicitly declares the
  versions it accepts.
- The current schema version must be accepted by every listed processor.
- Duration values carry one of `ns`, `us`, `ms`, `s`, `m`, `h`, or `d`.
- A stream's default retention class must appear in its accepted set.
- New stream and metric templates default to `PRIVATE`; registry v1 has no
  implicit public path and therefore requires `public_projection: none`.
- A non-Series stream sets `series` to `null`. A Series definition must provide
  its exact format, channel, compression, and chunk reconstruction contract.

## Validation

From the repository root:

```text
python scripts/validate_registry.py
```

The default command validates both meta-schemas, all definitions and templates,
all legal fixtures, and proves that every negative fixture is rejected for its
declared reason. To check one or more files and receive a non-zero status for an
invalid document:

```text
python scripts/validate_registry.py registry/streams/app.foreground/v1.yaml
python scripts/validate_registry.py registry/fixtures/invalid/unknown-field.json
```
