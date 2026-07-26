#!/usr/bin/env python3
"""Run the complete phase-0 gate and write a machine-readable evidence report."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_REPORT = ROOT / "artifacts" / "phase0" / "latest.json"
MAX_CAPTURE_CHARACTERS = 12_000


@dataclass(frozen=True)
class GateStep:
    name: str
    requirement: str
    command: tuple[str, ...]


def executable(name: str) -> str:
    resolved = shutil.which(name)
    if resolved is None:
        raise RuntimeError(f"Required executable is not available on PATH: {name}")
    return resolved


def git_output(*arguments: str) -> str | None:
    try:
        completed = subprocess.run(
            ["git", *arguments],
            cwd=ROOT,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
            encoding="utf-8",
            errors="replace",
        )
    except (OSError, subprocess.CalledProcessError):
        return None
    return completed.stdout.strip()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def captured_tail(output: str) -> tuple[str, bool]:
    if len(output) <= MAX_CAPTURE_CHARACTERS:
        return output, False
    return output[-MAX_CAPTURE_CHARACTERS:], True


def run_step(step: GateStep, environment: dict[str, str]) -> dict[str, object]:
    started = time.monotonic()
    try:
        completed = subprocess.run(
            list(step.command),
            cwd=ROOT,
            env=environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
        exit_code = completed.returncode
        output = completed.stdout
    except OSError as exc:
        exit_code = 127
        output = f"{type(exc).__name__}: {exc}"
    duration = round(time.monotonic() - started, 3)
    tail, truncated = captured_tail(output)
    print(f"[{step.name}] exit={exit_code} duration={duration:.3f}s")
    if output:
        print(output, end="" if output.endswith("\n") else "\n")
    return {
        "name": step.name,
        "requirement": step.requirement,
        "command": subprocess.list2cmdline(step.command),
        "exit_code": exit_code,
        "duration_seconds": duration,
        "output_tail": tail,
        "output_truncated": truncated,
        "automatic_retries": 0,
    }


def write_report(path: Path, report: dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    rendered = json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(rendered, encoding="utf-8", newline="\n")
    os.replace(temporary, path)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--report",
        type=Path,
        default=DEFAULT_REPORT,
        help="Evidence report path; relative paths are resolved from the repository root.",
    )
    parser.add_argument(
        "--no-report",
        action="store_true",
        help="Run all checks without updating the evidence report.",
    )
    arguments = parser.parse_args()

    try:
        python = executable("python")
    except RuntimeError:
        python = sys.executable
    powershell = executable("pwsh")

    steps = (
        GateStep(
            "governance",
            "R0-01",
            (python, "scripts/check_governance.py"),
        ),
        GateStep("adr-lint", "R0-02", (python, "scripts/lint_adrs.py")),
        GateStep("documentation", "R0-01/R0-02", (python, "scripts/check_docs.py")),
        GateStep(
            "repository-scan",
            "R0-01",
            (python, "scripts/scan_repository.py"),
        ),
        GateStep(
            "proto-compatibility",
            "R0-03/R0-05",
            (python, "scripts/run_proto_contract_tests.py"),
        ),
        GateStep(
            "registry",
            "R0-04",
            (python, "scripts/validate_registry.py"),
        ),
        GateStep(
            "semantic-contracts",
            "R0-03/R0-04/R0-06",
            (python, "scripts/run_semantic_contract_tests.py"),
        ),
        GateStep(
            "golden-vector-source",
            "R0-06",
            (python, "contract-tests/generate_vectors.py", "--check"),
        ),
        GateStep(
            "five-language-codegen",
            "R0-05",
            (
                powershell,
                "-NoLogo",
                "-NoProfile",
                "-File",
                "scripts/verify_codegen.ps1",
            ),
        ),
        GateStep(
            "cross-language-contracts",
            "R0-06",
            (python, "scripts/run_cross_language_contract_tests.py"),
        ),
    )

    environment = os.environ.copy()
    environment["PYTHONUTF8"] = "1"
    started_at = dt.datetime.now(dt.timezone.utc)
    started_monotonic = time.monotonic()
    results: list[dict[str, object]] = []
    for step in steps:
        result = run_step(step, environment)
        results.append(result)
        if result["exit_code"] != 0:
            break

    passed = len(results) == len(steps) and all(
        result["exit_code"] == 0 for result in results
    )
    finished_at = dt.datetime.now(dt.timezone.utc)
    vector_paths = (
        ROOT / "contract-tests" / "vectors" / "phase0-v1.json",
        ROOT / "contract-tests" / "vectors" / "phase0-v1.properties",
        ROOT / "contract-tests" / "breaking-baseline" / "lifechronicle-v1.binpb",
    )
    vectors = {
        path.relative_to(ROOT).as_posix(): {
            "sha256": sha256(path),
            "size_bytes": path.stat().st_size,
        }
        for path in vector_paths
        if path.is_file()
    }
    lock_path = ROOT / "toolchain.lock.json"
    report_path = arguments.report
    if not report_path.is_absolute():
        report_path = ROOT / report_path
    status_before_report = git_output("status", "--short") or ""
    report: dict[str, object] = {
        "schema_version": 1,
        "gate": "phase-0",
        "status": "passed" if passed else "failed",
        "command": "make phase-0-gate",
        "started_at_utc": started_at.isoformat(),
        "finished_at_utc": finished_at.isoformat(),
        "duration_seconds": round(time.monotonic() - started_monotonic, 3),
        "source": {
            "commit": git_output("rev-parse", "HEAD") or "UNCOMMITTED",
            "branch": git_output("branch", "--show-current") or "DETACHED",
            "worktree_clean_before_report": not bool(status_before_report),
            "worktree_status_before_report": status_before_report.splitlines(),
        },
        "environment": {
            "os": platform.platform(),
            "python": platform.python_version(),
            "machine": platform.machine(),
        },
        "toolchain_lock": {
            "path": lock_path.relative_to(ROOT).as_posix(),
            "sha256": sha256(lock_path),
            "content": json.loads(lock_path.read_text(encoding="utf-8")),
        },
        "vectors": vectors,
        "steps": results,
        "retry_policy": (
            "No automatic retry: a flaky or transient failure remains a failed gate."
        ),
        "limitations": [
            (
                "This report proves the local Windows bootstrap gate only. "
                "No remote repository is configured, so hosted CI execution and "
                "server-side main branch protection are not claimed."
            ),
            (
                "Phase 1 infrastructure and INF-C001 through INF-C017 are outside "
                "the phase-0 scope and were not executed."
            ),
        ],
        "report_path": report_path.relative_to(ROOT).as_posix(),
    }

    if not arguments.no_report:
        write_report(report_path, report)
        print(f"Evidence report: {report_path.relative_to(ROOT).as_posix()}")
    if passed:
        print(
            f"Phase 0 gate passed: {len(results)} steps in "
            f"{report['duration_seconds']:.3f}s."
        )
        return 0
    failed = results[-1] if results else {"name": "initialization"}
    print(f"Phase 0 gate failed at {failed['name']}.", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
