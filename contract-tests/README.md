# Phase 0 contract acceptance

This directory is the executable acceptance package for the event contract test
IDs defined in
[`docs/protocol/event-stream-spec.md`](../docs/protocol/event-stream-spec.md).

- `vectors/phase0-v1.json` is the immutable human-readable five-language input
  and expected output for `ES-C003`, `ES-C004`, `ES-C009`, `ES-C017`, and
  `ES-C018`. `phase0-v1.properties` is a deterministic flattened view generated
  from the same object so runners can use only their standard-library parsers.
- `generate_vectors.py` is an independent, standard-library reference
  implementation. It includes the minimal Protobuf wire encoder needed to
  create the decode fixture and does not depend on generated bindings.
- `python/` contains the semantic, rejection, idempotency, routing, replay, and
  acknowledgement tests for `ES-C005` through `ES-C016`.
- `runners/` contains the Go, Rust, Kotlin, Java, and TypeScript implementations
  that consume the same vector and emit the same normalized JSON result.

Run the complete suite through `make contract-test`. Regenerating the vector
must produce byte-identical JSON; a changed vector hash is a contract change
and requires review of the affected `ES-C*`, ADR, compatibility, and migration
requirements.
