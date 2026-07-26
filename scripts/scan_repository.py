#!/usr/bin/env python3
"""Phase 0 repository, license, and strong-secret checks.

The scanner intentionally examines the Git candidate set (tracked files plus
untracked, non-ignored files), not every local file. This keeps developer tool
installations and caches out of scope while still rejecting them if they would
be committed.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path, PurePosixPath


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]

REQUIRED_DIRECTORIES = {
    ".github",
    "agents",
    "codegen",
    "contract-tests",
    "docs",
    "generated",
    "infrastructure",
    "lakehouse",
    "plugins",
    "proto",
    "registry",
    "scripts",
    "services",
    "streaming",
    "tests",
    "web",
    "workflows",
}

# These are either repository boundaries or explicitly local-only directories.
# "artifacts" is optional because the phase gate may create evidence there.
ALLOWED_TOP_LEVEL_DIRECTORIES = REQUIRED_DIRECTORIES | {
    ".agents",
    ".cache",
    ".codex",
    ".git",
    ".openai",
    ".pnpm-store",
    ".tools",
    ".vscode",
    "artifacts",
    "build",
    "node_modules",
    "out",
    "target",
    "temp",
    "tmp",
}

FORBIDDEN_GIT_DIRECTORY_NAMES = {
    ".cache",
    ".pnpm-store",
    ".tools",
    "build",
    "cache",
    "node_modules",
    "out",
    "target",
    "temp",
    "tmp",
}

REQUIRED_IGNORES = (
    "/.tools/",
    ".cache/",
    "**/build/",
    "node_modules/",
    ".pnpm-store/",
)

MIT_MARKERS = (
    "MIT License",
    "Copyright (c) 2026 宋昊润",
    "Permission is hereby granted, free of charge",
    'THE SOFTWARE IS PROVIDED "AS IS"',
)

PINNED_TOOL_PATHS = (
    ("tools", "buf", "version"),
    ("tools", "protoc", "version"),
    ("tools", "protoc-gen-go", "version"),
    ("tools", "protoc-gen-es", "version"),
    ("tools", "go", "version"),
    ("tools", "rust", "rustc_version"),
    ("tools", "rust", "cargo_version"),
    ("tools", "java", "jdk_version"),
    ("tools", "gradle", "version"),
    ("tools", "kotlin", "gradle_plugin_version"),
    ("tools", "node", "version"),
    ("tools", "pnpm", "version"),
    ("tools", "typescript", "version"),
)

PINNED_DIGEST_PATHS = (
    ("tools", "buf", "windows_x86_64_sha256"),
    ("tools", "protoc", "windows_x86_64_zip_sha256"),
    ("tools", "go", "windows_x86_64_zip_sha256"),
    ("tools", "gradle", "bin_zip_sha256"),
)

STRONG_SECRET_PATTERNS = (
    (
        "private key",
        re.compile(
            r"-----BEGIN (?:OPENSSH |RSA |EC |DSA |PGP )?PRIVATE KEY-----"
        ),
    ),
    ("AWS access key", re.compile(r"\b(?:AKIA|ASIA)[A-Z0-9]{16}\b")),
    (
        "GitHub token",
        re.compile(r"\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,255}\b"),
    ),
    (
        "GitHub fine-grained token",
        re.compile(r"\bgithub_pat_[A-Za-z0-9_]{70,255}\b"),
    ),
    (
        "Google API key",
        re.compile(r"\bAIza[0-9A-Za-z_-]{35}\b"),
    ),
    (
        "Slack token",
        re.compile(r"\bxox[baprs]-[0-9A-Za-z-]{10,255}\b"),
    ),
    (
        "Stripe live secret",
        re.compile(r"\bsk_live_[0-9A-Za-z]{16,255}\b"),
    ),
)

ASSIGNED_SECRET_PATTERN = re.compile(
    r"""(?ix)
    \b(?:api[_-]?key|access[_-]?token|auth[_-]?token|client[_-]?secret|
        password|passwd|private[_-]?key|secret)\b
    \s*(?::|=)\s*
    ["']([^"' \t\r\n]{12,})["']
    """
)

SAFE_SECRET_VALUE_FRAGMENTS = (
    "${{",
    "${",
    "<",
    "changeme",
    "example",
    "placeholder",
    "redacted",
    "replace-me",
    "replace_me",
    "test-only",
    "xxxx",
)


def git_candidate_files() -> list[PurePosixPath]:
    process = subprocess.run(
        [
            "git",
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
        cwd=REPOSITORY_ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if process.returncode != 0:
        detail = process.stderr.decode("utf-8", errors="replace").strip()
        raise RuntimeError(f"cannot enumerate Git candidate files: {detail}")

    return [
        PurePosixPath(raw.decode("utf-8", errors="surrogateescape"))
        for raw in process.stdout.split(b"\0")
        if raw
    ]


def nested_value(document: dict[str, object], keys: tuple[str, ...]) -> object:
    value: object = document
    for key in keys:
        if not isinstance(value, dict) or key not in value:
            raise KeyError(".".join(keys))
        value = value[key]
    return value


def decode_text(path: Path) -> str | None:
    data = path.read_bytes()
    if b"\0" in data:
        return None
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        return None


def main() -> int:
    failures: list[str] = []

    missing_directories = sorted(
        name
        for name in REQUIRED_DIRECTORIES
        if not (REPOSITORY_ROOT / name).is_dir()
    )
    if missing_directories:
        failures.append(
            "missing required top-level directories: "
            + ", ".join(missing_directories)
        )

    unexpected_directories = sorted(
        entry.name
        for entry in REPOSITORY_ROOT.iterdir()
        if entry.is_dir() and entry.name not in ALLOWED_TOP_LEVEL_DIRECTORIES
    )
    if unexpected_directories:
        failures.append(
            "unregistered top-level directories: "
            + ", ".join(unexpected_directories)
        )

    license_path = REPOSITORY_ROOT / "LICENSE"
    if not license_path.is_file():
        failures.append("LICENSE is missing")
    else:
        license_text = license_path.read_text(encoding="utf-8")
        for marker in MIT_MARKERS:
            if marker not in license_text:
                failures.append(f"LICENSE is not the approved MIT text: missing {marker!r}")

    third_party_path = REPOSITORY_ROOT / "THIRD_PARTY_SOURCES.md"
    if not third_party_path.is_file():
        failures.append("THIRD_PARTY_SOURCES.md is missing")
    else:
        third_party_text = third_party_path.read_text(encoding="utf-8")
        approved_entry = any(
            re.match(r"^\|\s*[A-Z]+-\d{3,}\s*\|", line)
            and re.search(r"\|\s*`?APPROVED_DEPENDENCY`?\s*\|", line)
            for line in third_party_text.splitlines()
        )
        if not approved_entry:
            failures.append(
                "THIRD_PARTY_SOURCES.md has no registered "
                "APPROVED_DEPENDENCY table entry"
            )

    ignore_path = REPOSITORY_ROOT / ".gitignore"
    if not ignore_path.is_file():
        failures.append(".gitignore is missing")
    else:
        ignore_text = ignore_path.read_text(encoding="utf-8")
        for required_ignore in REQUIRED_IGNORES:
            if required_ignore not in ignore_text:
                failures.append(
                    f".gitignore does not exclude required local path {required_ignore!r}"
                )

    lock_path = REPOSITORY_ROOT / "toolchain.lock.json"
    if not lock_path.is_file():
        failures.append("toolchain.lock.json is missing")
    else:
        try:
            lock = json.loads(lock_path.read_text(encoding="utf-8"))
            if lock.get("policy") != "exact versions; local generation only":
                failures.append(
                    "toolchain.lock.json must require exact versions and local generation"
                )
            for path in PINNED_TOOL_PATHS:
                value = nested_value(lock, path)
                if not isinstance(value, str) or not value.strip():
                    failures.append(
                        f"toolchain.lock.json has an empty pin at {'.'.join(path)}"
                    )
            for path in PINNED_DIGEST_PATHS:
                digest = nested_value(lock, path)
                if not isinstance(digest, str) or not re.fullmatch(
                    r"[0-9a-f]{64}", digest
                ):
                    failures.append(
                        "toolchain.lock.json has an invalid SHA-256 at "
                        + ".".join(path)
                    )
        except (json.JSONDecodeError, KeyError, TypeError) as error:
            failures.append(f"invalid toolchain.lock.json: {error}")

    try:
        candidates = git_candidate_files()
    except RuntimeError as error:
        failures.append(str(error))
        candidates = []

    forbidden_candidates = sorted(
        path.as_posix()
        for path in candidates
        if any(
            part.casefold() in FORBIDDEN_GIT_DIRECTORY_NAMES
            for part in path.parts[:-1]
        )
    )
    if forbidden_candidates:
        failures.append(
            "local tool/cache/build directories are Git candidates: "
            + ", ".join(forbidden_candidates[:20])
        )

    text_file_count = 0
    for relative_path in candidates:
        absolute_path = REPOSITORY_ROOT.joinpath(*relative_path.parts)
        if not absolute_path.is_file():
            continue
        text = decode_text(absolute_path)
        if text is None:
            continue
        text_file_count += 1

        for label, pattern in STRONG_SECRET_PATTERNS:
            match = pattern.search(text)
            if match:
                line_number = text.count("\n", 0, match.start()) + 1
                failures.append(
                    f"{relative_path.as_posix()}:{line_number}: possible {label}"
                )

        for match in ASSIGNED_SECRET_PATTERN.finditer(text):
            value = match.group(1).casefold()
            if any(fragment.casefold() in value for fragment in SAFE_SECRET_VALUE_FRAGMENTS):
                continue
            line_number = text.count("\n", 0, match.start()) + 1
            failures.append(
                f"{relative_path.as_posix()}:{line_number}: possible assigned secret"
            )

    if failures:
        print("Repository scan failed:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print(
        "Repository scan passed: "
        f"{len(candidates)} Git candidate files, {text_file_count} text files, "
        "MIT license, dependency registry, boundaries, ignores, pins, and "
        "strong-secret patterns verified."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
