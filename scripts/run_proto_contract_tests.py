#!/usr/bin/env python3
"""Run ES-C001 and ES-C002 against the accepted Protobuf v1 baseline."""

from __future__ import annotations

import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROTO = ROOT / "proto"
BASELINE = (
    ROOT
    / "contract-tests"
    / "breaking-baseline"
    / "lifechronicle-v1.binpb"
)
BASELINE_SHA256 = (
    "f160ec95b89b4b4e2c7236d9ba35e1cf016dbc81e938c0fb116ac91b8c6221f9"
)


def find_buf() -> str:
    configured = os.environ.get("BUF")
    if configured:
        return configured
    local = ROOT / ".tools" / ("buf.exe" if os.name == "nt" else "buf")
    if local.is_file():
        return str(local)
    discovered = shutil.which("buf")
    if discovered:
        return discovered
    raise FileNotFoundError(
        "Buf 1.72.0 is required (set BUF or bootstrap .tools/buf)."
    )


def run(
    command: list[str],
    *,
    cwd: Path = ROOT,
    expect_success: bool = True,
) -> subprocess.CompletedProcess[str]:
    cache = ROOT / ".tools" / "buf-cache"
    cache.mkdir(parents=True, exist_ok=True)
    completed = subprocess.run(
        command,
        cwd=cwd,
        env={**os.environ, "BUF_CACHE_DIR": str(cache)},
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=False,
    )
    if expect_success and completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(command)}\n"
            f"{completed.stdout}{completed.stderr}"
        )
    if not expect_success and completed.returncode == 0:
        raise AssertionError(
            "breaking-change negative control unexpectedly passed: "
            + " ".join(command)
        )
    return completed


def main() -> None:
    buf = find_buf()
    if not BASELINE.is_file():
        raise FileNotFoundError(f"missing breaking baseline: {BASELINE}")
    actual_hash = hashlib.sha256(BASELINE.read_bytes()).hexdigest()
    if actual_hash != BASELINE_SHA256:
        raise AssertionError(
            f"breaking baseline SHA-256 mismatch: {actual_hash}"
        )

    version = run([buf, "--version"]).stdout.strip()
    if version != "1.72.0":
        raise AssertionError(f"Buf version drift: expected 1.72.0, got {version}")
    run([buf, "lint", str(PROTO)])
    run([buf, "build", str(PROTO)])
    run([buf, "breaking", str(PROTO), "--against", str(BASELINE)])
    print("ES-C001: Buf STANDARD lint/build passed.")
    print("ES-C002: current Proto is FILE-compatible with accepted v1 image.")

    temporary_root = ROOT / ".tools" / "tmp"
    temporary_root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(
        prefix="lifechronicle-breaking-", dir=temporary_root
    ) as temp:
        mutated = Path(temp) / "proto"
        shutil.copytree(PROTO, mutated)
        event_file = (
            mutated / "lifechronicle" / "events" / "v1" / "event.proto"
        )
        original = event_file.read_text(encoding="utf-8")
        changed = original.replace(
            "  string event_id = 1;",
            "  string event_id = 101;",
            1,
        )
        if changed == original:
            raise AssertionError("negative control could not mutate event_id")
        event_file.write_text(changed, encoding="utf-8", newline="\n")
        rejected = run(
            [buf, "breaking", str(mutated), "--against", str(BASELINE)],
            expect_success=False,
        )
        diagnostic = (rejected.stdout + rejected.stderr).lower()
        if "field" not in diagnostic and "deleted" not in diagnostic:
            raise AssertionError(
                "breaking negative control failed without a field diagnostic:\n"
                + rejected.stdout
                + rejected.stderr
            )
    print("ES-C002: deletion/renumbering negative control was rejected.")


if __name__ == "__main__":
    try:
        main()
    except Exception as error:
        print(f"Proto contract tests failed: {error}", file=sys.stderr)
        raise
