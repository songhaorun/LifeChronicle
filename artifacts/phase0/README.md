# Phase 0 evidence

`latest.json` is written by `make phase-0-gate`. It records the tested source state, exact locked
toolchain, golden-vector hashes, every command, exit code, captured output, duration, retry count and
known limitations. A failed run also overwrites the report with the failing evidence and returns a
non-zero exit code.

The report proves a local bootstrap run. Hosted CI and server-side `main` protection require a remote
repository and are never inferred from this file.
