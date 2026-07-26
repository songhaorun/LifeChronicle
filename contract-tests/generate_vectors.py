#!/usr/bin/env python3
"""Generate the immutable Phase 0 cross-language contract vector.

This generator intentionally implements the small amount of wire encoding used
by the fixture without importing a Protobuf runtime.  It is therefore an
independent reference for language-runtime decode checks.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
from pathlib import Path


ROOT = Path(__file__).resolve().parent
OUTPUT = ROOT / "vectors" / "phase0-v1.json"
PROPERTIES_OUTPUT = ROOT / "vectors" / "phase0-v1.properties"


def sha256(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def u8(value: int) -> bytes:
    return struct.pack(">B", value)


def u32be(value: int) -> bytes:
    return struct.pack(">I", value)


def u64be(value: int) -> bytes:
    return struct.pack(">Q", value)


def i64be(value: int) -> bytes:
    return struct.pack(">q", value)


def framed_bytes(value: bytes) -> bytes:
    return u32be(len(value)) + value


def framed_string(value: str) -> bytes:
    return framed_bytes(value.encode("utf-8"))


def varint(value: int) -> bytes:
    if value < 0:
        value &= (1 << 64) - 1
    out = bytearray()
    while value > 0x7F:
        out.append((value & 0x7F) | 0x80)
        value >>= 7
    out.append(value)
    return bytes(out)


def field_varint(number: int, value: int) -> bytes:
    return varint(number << 3) + varint(value)


def field_fixed64(number: int, value: float) -> bytes:
    return varint((number << 3) | 1) + struct.pack("<d", value)


def field_bytes(number: int, value: bytes) -> bytes:
    return varint((number << 3) | 2) + varint(len(value)) + value


def field_string(number: int, value: str) -> bytes:
    return field_bytes(number, value.encode("utf-8"))


def field_message(number: int, value: bytes) -> bytes:
    return field_bytes(number, value)


def timestamp_wire(seconds: int, nanos: int) -> bytes:
    return field_varint(1, seconds) + field_varint(2, nanos)


def payload_wire() -> tuple[bytes, dict[str, object]]:
    payload = {
        "application_id": "com.example.editor",
        "application_name": "Example Editor",
        "executable_name": "editor.exe",
        "window_title": "阶段 0 – LifeChronicle",
        "process_id": 4242,
        "window_id": "0x001122",
        "fullscreen": False,
    }
    wire = b"".join(
        [
            field_string(1, str(payload["application_id"])),
            field_string(2, str(payload["application_name"])),
            field_string(3, str(payload["executable_name"])),
            field_string(4, str(payload["window_title"])),
            field_varint(5, int(payload["process_id"])),
            field_string(6, str(payload["window_id"])),
        ]
    )
    return wire, payload


def origin_wire(origin: dict[str, str]) -> bytes:
    result = b""
    for number, name in enumerate(
        (
            "provider",
            "provider_record_id",
            "import_id",
            "parent_event_id",
            "collection_method",
        ),
        start=1,
    ):
        if origin[name]:
            result += field_string(number, origin[name])
    return result


def event_wire(event: dict[str, object], payload_value: bytes) -> bytes:
    any_wire = (
        field_string(1, str(event["payload_type_url"]))
        + field_bytes(2, payload_value)
    )
    parts = [
        field_string(1, str(event["event_id"])),
        field_string(2, str(event["stream"])),
        field_string(3, str(event["event_type"])),
        field_varint(4, int(event["kind"])),
        field_string(6, str(event["device_id"])),
        field_string(7, str(event["collector_instance_id"])),
        field_string(8, str(event["source"])),
        field_varint(9, int(event["schema_version"])),
        field_varint(10, int(event["sequence"])),
        field_message(
            11,
            timestamp_wire(
                int(event["observed_at"]["seconds"]),
                int(event["observed_at"]["nanos"]),
            ),
        ),
        field_string(13, str(event["timezone"])),
        field_varint(14, int(event["privacy_class"])),
        field_varint(15, int(event["retention_class"])),
        field_message(16, origin_wire(event["origin"])),
        field_message(20, any_wire),
    ]
    return b"".join(parts)


def lcb1(batch: dict[str, object]) -> bytes:
    return b"".join(
        [
            b"LCB1",
            framed_string(str(batch["batch_id"])),
            framed_string(str(batch["device_id"])),
            framed_string(str(batch["collector_instance_id"])),
            u64be(int(batch["sequence_start"])),
            u64be(int(batch["sequence_end"])),
            i64be(int(batch["created_at"]["seconds"])),
            u32be(int(batch["created_at"]["nanos"])),
            framed_bytes(bytes.fromhex(str(batch["nonce_hex"]))),
            u32be(int(batch["compression"])),
            framed_string(str(batch["source"])),
            bytes.fromhex(str(batch["payload_sha256_hex"])),
        ]
    )


def lce1(event: dict[str, object]) -> bytes:
    observed = event["observed_at"]
    ended = event["ended_at"]
    origin = event["origin"]
    parts = [
        b"LCE1",
        framed_string(str(event["event_id"])),
        framed_string(str(event["stream"])),
        framed_string(str(event["event_type"])),
        u32be(int(event["kind"])),
        framed_string(str(event["device_id"])),
        framed_string(str(event["collector_instance_id"])),
        framed_string(str(event["source"])),
        u32be(int(event["schema_version"])),
        u64be(int(event["sequence"])),
        i64be(int(observed["seconds"])),
        u32be(int(observed["nanos"])),
        u8(1 if ended is not None else 0),
    ]
    if ended is not None:
        parts.extend([i64be(int(ended["seconds"])), u32be(int(ended["nanos"]))])
    parts.extend(
        [
            framed_string(str(event["timezone"])),
            u32be(int(event["privacy_class"])),
            u32be(int(event["retention_class"])),
            framed_string(str(origin["provider"])),
            framed_string(str(origin["provider_record_id"])),
            framed_string(str(origin["import_id"])),
            framed_string(str(origin["parent_event_id"])),
            framed_string(str(origin["collection_method"])),
            framed_string(str(event["payload_type_url"])),
            framed_bytes(bytes.fromhex(str(event["payload_value_hex"]))),
        ]
    )
    return b"".join(parts)


def identity_frame(magic: bytes, user_id: str, submitted_hash: bytes) -> bytes:
    return magic + framed_string(user_id) + submitted_hash


def kafka_key(fields: list[str]) -> bytes:
    return b"".join(framed_string(field) for field in fields)


def channel_wire() -> bytes:
    return b"".join(
        [
            field_string(1, "x"),
            field_varint(2, 10),
            field_string(3, "m/s2"),
            field_fixed64(4, 1.0),
            field_varint(6, 1),
        ]
    )


def clock_wire() -> bytes:
    return b"".join(
        [
            field_varint(1, 2),
            field_string(2, "monotonic-0"),
            field_varint(4, 1_000_000),
            field_fixed64(5, 0.5),
        ]
    )


def series_wire(series: dict[str, object], compressed: bytes, checksum: bytes) -> bytes:
    deltas = b"".join(varint(value) for value in series["timestamp_delta_ns"])
    return b"".join(
        [
            field_string(1, str(series["chunk_id"])),
            field_string(2, str(series["stream"])),
            field_varint(3, int(series["schema_version"])),
            field_varint(4, int(series["start_time_ns"])),
            field_varint(5, int(series["end_time_ns"])),
            field_fixed64(6, float(series["nominal_sample_rate"])),
            field_bytes(7, deltas),
            field_message(8, channel_wire()),
            field_bytes(9, compressed),
            field_bytes(10, checksum),
            field_string(11, str(series["device_id"])),
            field_string(12, str(series["collector_instance_id"])),
            field_string(13, str(series["source"])),
            field_string(14, str(series["timezone"])),
            field_varint(15, int(series["privacy_class"])),
            field_varint(16, int(series["retention_class"])),
            field_varint(17, int(series["sequence"])),
            field_message(18, clock_wire()),
        ]
    )


def build_vector() -> dict[str, object]:
    payload_value, decoded_payload = payload_wire()
    event: dict[str, object] = {
        "event_id": "018f1e2d-3c4b-7abc-8def-0123456789ab",
        "stream": "app.foreground",
        "event_type": "app.foreground.observed",
        "kind": 1,
        "user_id": "",
        "device_id": "123e4567-e89b-12d3-a456-426614174000",
        "collector_instance_id": "223e4567-e89b-12d3-a456-426614174001",
        "source": "windows.foreground",
        "schema_version": 1,
        "sequence": 42,
        "observed_at": {"seconds": 1_735_689_600, "nanos": 123_456_789},
        "ended_at": None,
        "timezone": "Asia/Shanghai",
        "privacy_class": 1,
        "retention_class": 2,
        "origin": {
            "provider": "windows",
            "provider_record_id": "",
            "import_id": "",
            "parent_event_id": "",
            "collection_method": "accessibility",
        },
        "payload_type_url": (
            "type.googleapis.com/lifechronicle.events.v1.AppForeground"
        ),
        "payload_value_hex": payload_value.hex(),
    }
    protobuf_event = event_wire(event, payload_value)

    compressed_items = bytes.fromhex("28b52ffd20082100004576656e742d7631")
    payload_hash = sha256(compressed_items)
    batch: dict[str, object] = {
        "batch_id": "018f1e2d-3c4b-7abc-8def-0123456789ac",
        "device_id": event["device_id"],
        "collector_instance_id": event["collector_instance_id"],
        "sequence_start": 42,
        "sequence_end": 42,
        "created_at": {"seconds": 1_735_689_605, "nanos": 987_654_321},
        "nonce_hex": "000102030405060708090a0b0c0d0e0f",
        "compression": 1,
        "source": event["source"],
        "compressed_items_hex": compressed_items.hex(),
        "payload_sha256_hex": payload_hash.hex(),
    }

    raw_series_payload = bytes.fromhex(
        "3ff0000000000000bff0000000000000"
    )
    compressed_series = bytes.fromhex(
        "28b52ffd20108100003ff0000000000000bff0000000000000"
    )
    checksum = sha256(raw_series_payload)
    series: dict[str, object] = {
        "chunk_id": "018f1e2d-3c4b-7abc-8def-0123456789ad",
        "stream": "sensor.imu.acceleration",
        "schema_version": 1,
        "start_time_ns": 1_735_689_600_000_000_000,
        "end_time_ns": 1_735_689_600_020_000_000,
        "nominal_sample_rate": 50.0,
        "timestamp_delta_ns": [0, 20_000_000],
        "device_id": event["device_id"],
        "collector_instance_id": event["collector_instance_id"],
        "source": "android.imu",
        "timezone": "Asia/Shanghai",
        "privacy_class": 2,
        "retention_class": 1,
        "sequence": 43,
        "user_id": "323e4567-e89b-12d3-a456-426614174002",
        "raw_payload_hex": raw_series_payload.hex(),
        "compressed_payload_hex": compressed_series.hex(),
        "checksum_hex": checksum.hex(),
    }
    submitted_series = series_wire(series, compressed_series, checksum)
    series["submitted_wire_hex"] = submitted_series.hex()

    lcb = lcb1(batch)
    lce = lce1(event)
    lce_hash = sha256(lce)
    lcc = identity_frame(
        b"LCC1", "323e4567-e89b-12d3-a456-426614174002", lce_hash
    )
    lcs = b"LCS1" + framed_bytes(submitted_series)
    lcs_hash = sha256(lcs)
    lcr = identity_frame(b"LCR1", str(series["user_id"]), lcs_hash)
    compressed_hash = sha256(compressed_series)
    object_key = (
        f"private/{series['user_id']}/series/{series['chunk_id']}/"
        f"{compressed_hash.hex()}.zst"
    )
    key_fields = [
        str(event["user_id"] or "323e4567-e89b-12d3-a456-426614174002"),
        str(event["device_id"]),
        str(event["collector_instance_id"]),
    ]

    return {
        "format": "lifechronicle.contract-vector/v1",
        "description": (
            "Phase 0 ES-C003/004/009/017/018 cross-language golden vector"
        ),
        "batch": batch,
        "event": event,
        "series": series,
        "kafka_key_fields": key_fields,
        "protobuf_event_hex": protobuf_event.hex(),
        "decoded_event": {
            **event,
            "payload": decoded_payload,
        },
        "expected": {
            "lcb1_hex": lcb.hex(),
            "lce1_hex": lce.hex(),
            "lcc1_hex": lcc.hex(),
            "lcs1_hex": lcs.hex(),
            "lcr1_hex": lcr.hex(),
            "submitted_sha256_hex": lce_hash.hex(),
            "canonical_sha256_hex": sha256(lcc).hex(),
            "series_submitted_sha256_hex": lcs_hash.hex(),
            "series_canonical_sha256_hex": sha256(lcr).hex(),
            "kafka_key_hex": kafka_key(key_fields).hex(),
            "compressed_size": len(compressed_series),
            "compressed_sha256_hex": compressed_hash.hex(),
            "object_version": compressed_hash.hex(),
            "object_key": object_key,
        },
    }


def flatten_properties(
    prefix: str, value: object, output: dict[str, str]
) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            name = f"{prefix}.{key}" if prefix else str(key)
            flatten_properties(name, child, output)
        return
    if isinstance(value, list):
        output[f"{prefix}.count"] = str(len(value))
        for index, child in enumerate(value):
            flatten_properties(f"{prefix}.{index}", child, output)
        return
    if value is None:
        output[f"{prefix}.present"] = "false"
        return
    if isinstance(value, bool):
        output[prefix] = "true" if value else "false"
        return
    output[prefix] = str(value)


def escape_property(value: str) -> str:
    return (
        value.replace("\\", "\\\\")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("=", "\\=")
    )


def render_vector() -> tuple[str, str]:
    vector = build_vector()
    content = json.dumps(vector, ensure_ascii=False, indent=2, sort_keys=True) + "\n"
    flat: dict[str, str] = {}
    for section in (
        "batch",
        "event",
        "series",
        "kafka_key_fields",
        "protobuf_event_hex",
        "expected",
    ):
        flatten_properties(section, vector[section], flat)
    properties = "".join(
        f"{escape_property(key)}={escape_property(flat[key])}\n"
        for key in sorted(flat)
    )
    return content, properties


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail if committed vector files differ from regenerated content",
    )
    args = parser.parse_args()
    content, properties = render_vector()
    digest = hashlib.sha256(content.encode("utf-8")).hexdigest()
    properties_digest = hashlib.sha256(properties.encode("utf-8")).hexdigest()
    if args.check:
        failures: list[str] = []
        if not OUTPUT.is_file() or OUTPUT.read_text(encoding="utf-8") != content:
            failures.append(str(OUTPUT.relative_to(ROOT.parent)))
        if (
            not PROPERTIES_OUTPUT.is_file()
            or PROPERTIES_OUTPUT.read_text(encoding="utf-8") != properties
        ):
            failures.append(str(PROPERTIES_OUTPUT.relative_to(ROOT.parent)))
        if failures:
            raise SystemExit(
                "Generated vectors are stale: " + ", ".join(failures)
            )
        print(
            "Vector generation check passed: "
            f"json={digest} properties={properties_digest}"
        )
        return

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(content, encoding="utf-8", newline="\n")
    print(f"Wrote {OUTPUT.relative_to(ROOT.parent)} sha256={digest}")
    PROPERTIES_OUTPUT.write_text(properties, encoding="utf-8", newline="\n")
    print(
        f"Wrote {PROPERTIES_OUTPUT.relative_to(ROOT.parent)} "
        f"sha256={properties_digest}"
    )


if __name__ == "__main__":
    main()
