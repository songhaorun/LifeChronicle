#!/usr/bin/env python3
"""Run and compare the five Phase 0 golden-vector implementations."""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path
from types import ModuleType


ROOT = Path(__file__).resolve().parents[1]
VECTOR_JSON = ROOT / "contract-tests" / "vectors" / "phase0-v1.json"
VECTOR_PROPERTIES = (
    ROOT / "contract-tests" / "vectors" / "phase0-v1.properties"
)
RUNNERS = {
    language: (
        ROOT / "contract-tests" / "runners" / language / "run.ps1"
    )
    for language in ("go", "rust", "kotlin", "java", "typescript")
}


def load_reference_module() -> ModuleType:
    source = ROOT / "contract-tests" / "generate_vectors.py"
    spec = importlib.util.spec_from_file_location(
        "lifechronicle_vector_reference", source
    )
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load reference implementation: {source}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def parse_output(output: str, language: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for number, line in enumerate(output.splitlines(), start=1):
        if not line:
            continue
        if "=" not in line:
            raise AssertionError(
                f"{language} emitted non-property stdout at line {number}: "
                f"{line!r}"
            )
        key, value = line.split("=", 1)
        if not key or key in values:
            raise AssertionError(
                f"{language} emitted invalid/duplicate key {key!r}"
            )
        values[key] = value
    if values.get("language") != language:
        raise AssertionError(
            f"{language} runner identified as {values.get('language')!r}"
        )
    return values


def expected_results(vector: dict[str, object]) -> dict[str, str]:
    expected = {
        key: str(value)
        for key, value in vector["expected"].items()
    }
    event = vector["decoded_event"]
    origin = event["origin"]
    payload = event["payload"]
    ended = event["ended_at"]
    expected.update(
        {
            "decoded.event_id": str(event["event_id"]),
            "decoded.stream": str(event["stream"]),
            "decoded.event_type": str(event["event_type"]),
            "decoded.kind": str(event["kind"]),
            "decoded.user_id": str(event["user_id"]),
            "decoded.device_id": str(event["device_id"]),
            "decoded.collector_instance_id": str(
                event["collector_instance_id"]
            ),
            "decoded.source": str(event["source"]),
            "decoded.schema_version": str(event["schema_version"]),
            "decoded.sequence": str(event["sequence"]),
            "decoded.observed_at.seconds": str(
                event["observed_at"]["seconds"]
            ),
            "decoded.observed_at.nanos": str(
                event["observed_at"]["nanos"]
            ),
            "decoded.ended_at.present": (
                "true" if ended is not None else "false"
            ),
            "decoded.timezone": str(event["timezone"]),
            "decoded.privacy_class": str(event["privacy_class"]),
            "decoded.retention_class": str(event["retention_class"]),
            "decoded.origin.provider": str(origin["provider"]),
            "decoded.origin.provider_record_id": str(
                origin["provider_record_id"]
            ),
            "decoded.origin.import_id": str(origin["import_id"]),
            "decoded.origin.parent_event_id": str(
                origin["parent_event_id"]
            ),
            "decoded.origin.collection_method": str(
                origin["collection_method"]
            ),
            "decoded.payload_type_url": str(event["payload_type_url"]),
            "decoded.payload_value_hex": str(event["payload_value_hex"]),
            "decoded.payload.application_id": str(
                payload["application_id"]
            ),
            "decoded.payload.application_name": str(
                payload["application_name"]
            ),
            "decoded.payload.executable_name": str(
                payload["executable_name"]
            ),
            "decoded.payload.window_title_utf8_hex": str(
                payload["window_title"]
            ).encode("utf-8").hex(),
            "decoded.payload.process_id": str(payload["process_id"]),
            "decoded.payload.window_id": str(payload["window_id"]),
            "decoded.payload.fullscreen": (
                "true" if payload["fullscreen"] else "false"
            ),
        }
    )
    return expected


def run_runner(language: str) -> dict[str, str]:
    script = RUNNERS[language]
    if not script.is_file():
        raise FileNotFoundError(f"missing {language} runner: {script}")
    completed = subprocess.run(
        [
            "pwsh",
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-File",
            str(script),
            str(VECTOR_PROPERTIES),
        ],
        cwd=ROOT,
        env={**os.environ, "NO_COLOR": "1"},
        capture_output=True,
        text=True,
        encoding="utf-8",
        timeout=300,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"{language} runner failed ({completed.returncode})\n"
            f"stdout:\n{completed.stdout}\n"
            f"stderr:\n{completed.stderr}"
        )
    if completed.stderr.strip():
        print(
            f"[{language} build]\n{completed.stderr.rstrip()}",
            file=sys.stderr,
        )
    return parse_output(completed.stdout, language)


def assert_expected(
    language: str, result: dict[str, str], expected: dict[str, str]
) -> None:
    actual_keys = set(result) - {"language"}
    expected_keys = set(expected)
    missing = sorted(expected_keys - actual_keys)
    extra = sorted(actual_keys - expected_keys)
    mismatches = sorted(
        key
        for key in expected_keys & actual_keys
        if result[key] != expected[key]
    )
    if missing or extra or mismatches:
        details = []
        if missing:
            details.append(f"missing={missing}")
        if extra:
            details.append(f"extra={extra}")
        for key in mismatches:
            details.append(
                f"{key}: expected={expected[key]!r} "
                f"actual={result[key]!r}"
            )
        raise AssertionError(
            f"{language} golden output mismatch: " + "; ".join(details)
        )


def verify_mutation_coverage(
    reference: ModuleType, vector: dict[str, object]
) -> None:
    event = vector["event"]
    base_hash = hashlib.sha256(reference.lce1(event)).digest()
    event_mutations = {
        "event_id": lambda value: value + "x",
        "stream": lambda value: "app.other",
        "event_type": lambda value: "app.foreground.changed",
        "kind": lambda value: 2,
        "device_id": lambda value: value[:-1] + "1",
        "collector_instance_id": lambda value: value[:-1] + "2",
        "source": lambda value: "windows.other",
        "schema_version": lambda value: int(value) + 1,
        "sequence": lambda value: int(value) + 1,
        "timezone": lambda value: "UTC",
        "privacy_class": lambda value: 2,
        "retention_class": lambda value: 1,
        "payload_type_url": lambda value: value + "V2",
        "payload_value_hex": lambda value: value + "00",
    }
    for field, mutate in event_mutations.items():
        changed = copy.deepcopy(event)
        changed[field] = mutate(changed[field])
        if hashlib.sha256(reference.lce1(changed)).digest() == base_hash:
            raise AssertionError(f"ES-C017 mutation not covered: {field}")
    for field in ("seconds", "nanos"):
        changed = copy.deepcopy(event)
        changed["observed_at"][field] += 1
        if hashlib.sha256(reference.lce1(changed)).digest() == base_hash:
            raise AssertionError(
                f"ES-C017 mutation not covered: observed_at.{field}"
            )
    changed = copy.deepcopy(event)
    changed["ended_at"] = {"seconds": 1_735_689_601, "nanos": 0}
    if hashlib.sha256(reference.lce1(changed)).digest() == base_hash:
        raise AssertionError("ES-C017 ended_at presence is not covered")
    for field in (
        "provider",
        "provider_record_id",
        "import_id",
        "parent_event_id",
        "collection_method",
    ):
        changed = copy.deepcopy(event)
        changed["origin"][field] += "x"
        if hashlib.sha256(reference.lce1(changed)).digest() == base_hash:
            raise AssertionError(
                f"ES-C017 mutation not covered: origin.{field}"
            )

    canonical_user = vector["series"]["user_id"]
    canonical = reference.identity_frame(
        b"LCC1", canonical_user, base_hash
    )
    changed_canonical = reference.identity_frame(
        b"LCC1", canonical_user[:-1] + "3", base_hash
    )
    if hashlib.sha256(canonical).digest() == hashlib.sha256(
        changed_canonical
    ).digest():
        raise AssertionError("ES-C017 trusted user is not identity-bound")

    series = vector["series"]
    compressed = bytes.fromhex(series["compressed_payload_hex"])
    checksum = bytes.fromhex(series["checksum_hex"])
    wire = reference.series_wire(series, compressed, checksum)
    if wire.hex() != series["submitted_wire_hex"]:
        raise AssertionError("ES-C018 reference Series wire drift")
    base_series_hash = hashlib.sha256(
        b"LCS1" + reference.framed_bytes(wire)
    ).digest()
    series_mutations = {
        "chunk_id": lambda value: value[:-1] + "e",
        "stream": lambda value: value + ".changed",
        "schema_version": lambda value: int(value) + 1,
        "start_time_ns": lambda value: int(value) + 1,
        "end_time_ns": lambda value: int(value) + 1,
        "nominal_sample_rate": lambda value: float(value) + 1.0,
        "timestamp_delta_ns": lambda value: [0, 20_000_001],
        "device_id": lambda value: value[:-1] + "1",
        "collector_instance_id": lambda value: value[:-1] + "2",
        "source": lambda value: value + ".changed",
        "timezone": lambda value: "UTC",
        "privacy_class": lambda value: 3,
        "retention_class": lambda value: 2,
        "sequence": lambda value: int(value) + 1,
    }
    for field, mutate in series_mutations.items():
        changed = copy.deepcopy(series)
        changed[field] = mutate(changed[field])
        changed_wire = reference.series_wire(changed, compressed, checksum)
        changed_hash = hashlib.sha256(
            b"LCS1" + reference.framed_bytes(changed_wire)
        ).digest()
        if changed_hash == base_series_hash:
            raise AssertionError(
                f"ES-C018 metadata mutation not covered: {field}"
            )
    for label, changed_compressed, changed_checksum in (
        ("compressed_payload", compressed + b"\x00", checksum),
        ("checksum", compressed, bytes([checksum[0] ^ 1]) + checksum[1:]),
    ):
        changed_wire = reference.series_wire(
            series, changed_compressed, changed_checksum
        )
        changed_hash = hashlib.sha256(
            b"LCS1" + reference.framed_bytes(changed_wire)
        ).digest()
        if changed_hash == base_series_hash:
            raise AssertionError(f"ES-C018 mutation not covered: {label}")
    changed_lcr = reference.identity_frame(
        b"LCR1", canonical_user[:-1] + "3", base_series_hash
    )
    base_lcr = reference.identity_frame(
        b"LCR1", canonical_user, base_series_hash
    )
    if hashlib.sha256(changed_lcr).digest() == hashlib.sha256(
        base_lcr
    ).digest():
        raise AssertionError("ES-C018 trusted user is not identity-bound")

    expected = vector["expected"]
    if not expected["object_version"] or int(expected["compressed_size"]) < 1:
        raise AssertionError("ES-C018 object reference is incomplete")
    if (
        hashlib.sha256(bytes.fromhex(series["raw_payload_hex"])).hexdigest()
        != series["checksum_hex"]
    ):
        raise AssertionError("ES-C018 uncompressed checksum drift")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--languages",
        default="go,rust,kotlin,java,typescript",
        help="comma-separated runner subset (default: all five)",
    )
    args = parser.parse_args()
    languages = [item.strip() for item in args.languages.split(",") if item]
    unknown = sorted(set(languages) - set(RUNNERS))
    if unknown:
        raise SystemExit(f"unknown languages: {', '.join(unknown)}")
    if not languages:
        raise SystemExit("at least one language is required")

    vector = json.loads(VECTOR_JSON.read_text(encoding="utf-8"))
    reference = load_reference_module()
    regenerated = reference.build_vector()
    if vector != regenerated:
        raise AssertionError("committed JSON vector differs from generator")
    expected = expected_results(vector)
    verify_mutation_coverage(reference, vector)

    all_results: dict[str, dict[str, str]] = {}
    for language in languages:
        result = run_runner(language)
        assert_expected(language, result, expected)
        all_results[language] = result
        print(f"{language}: {len(expected)} fields/frames match")

    baseline = all_results[languages[0]]
    for language in languages[1:]:
        for key in expected:
            if all_results[language][key] != baseline[key]:
                raise AssertionError(
                    f"cross-language mismatch for {key}: "
                    f"{languages[0]}={baseline[key]!r}, "
                    f"{language}={all_results[language][key]!r}"
                )
    print(
        "Cross-language contract coverage passed: "
        "ES-C003, ES-C004, ES-C009, ES-C017, ES-C018"
    )


if __name__ == "__main__":
    main()
