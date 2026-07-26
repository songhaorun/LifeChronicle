"""Executable reference semantics for LifeChronicle phase-0 contracts.

This module is deliberately dependency-free.  It is not production ingestion
code; it is a small executable model of the protocol invariants that must remain
true in every implementation.
"""

from __future__ import annotations

import copy
import hashlib
import json
import struct
from dataclasses import dataclass
from enum import Enum
from typing import Any, Iterable, Mapping, Sequence


TIMESTAMP_MIN_SECONDS = -62_135_596_800
TIMESTAMP_MAX_SECONDS = 253_402_300_799

RAW_EVENT_TOPIC = "lc.raw.events.v1"
RAW_SERIES_METADATA_TOPIC = "lc.raw.series-metadata.v1"
NORMALIZED_EVENT_TOPIC = "lc.normalized.events.v1"
LATE_EVENT_TOPIC = "lc.processing.late-events.v1"


@dataclass(frozen=True, order=True)
class Timestamp:
    seconds: int
    nanos: int


@dataclass(frozen=True)
class EventCandidate:
    stream: str
    event_type: str
    kind: str
    schema_version: int
    type_url: str
    observed_at: Timestamp
    ended_at: Timestamp | None = None


def validate_timestamp(value: Timestamp, path: str) -> list[str]:
    errors: list[str] = []
    if not TIMESTAMP_MIN_SECONDS <= value.seconds <= TIMESTAMP_MAX_SECONDS:
        errors.append(f"{path}.seconds is outside google.protobuf.Timestamp range")
    if not 0 <= value.nanos <= 999_999_999:
        errors.append(f"{path}.nanos must be in [0, 999999999]")
    return errors


def validate_event_against_stream(
    event: EventCandidate, stream_definition: Mapping[str, Any]
) -> list[str]:
    """Validate the envelope fields controlled by Stream Registry v1."""

    errors = validate_timestamp(event.observed_at, "observed_at")
    if event.ended_at is not None:
        errors.extend(validate_timestamp(event.ended_at, "ended_at"))

    if event.stream != stream_definition["name"]:
        errors.append("stream is not the selected Registry definition")
    if event.event_type != stream_definition["schema"]["event_type"]:
        errors.append("event_type does not match Registry")
    if event.kind != stream_definition["record_kind"]:
        errors.append("record kind does not match Registry")
    if event.schema_version != stream_definition["schema"]["version"]:
        errors.append("schema_version is not accepted by Registry")

    expected_type_url = (
        "type.googleapis.com/" + stream_definition["schema"]["payload_type"]
    )
    if event.type_url != expected_type_url:
        errors.append(
            f"payload.type_url must be canonical registered type {expected_type_url}"
        )

    if event.ended_at is not None:
        if event.kind != "INTERVAL":
            errors.append("non-INTERVAL record cannot set ended_at")
        elif event.ended_at < event.observed_at:
            errors.append("ended_at cannot precede observed_at")
    return errors


def encode_partition_key(
    ordered_fields: Sequence[str], record: Mapping[str, Any]
) -> bytes:
    """Encode Kafka key fields as u32be length followed by UTF-8 bytes."""

    encoded = bytearray()
    for field in ordered_fields:
        if field not in record or record[field] is None or record[field] == "":
            raise ValueError(f"missing Kafka partition key field: {field}")
        value = record[field]
        if not isinstance(value, str):
            raise TypeError(f"Kafka partition key field {field} must be a string")
        raw = value.encode("utf-8")
        if len(raw) > 0xFFFF_FFFF:
            raise ValueError(f"Kafka partition key field {field} is too large")
        encoded.extend(struct.pack(">I", len(raw)))
        encoded.extend(raw)
    return bytes(encoded)


def ingestion_topic(item_kind: str) -> str:
    if item_kind == "EVENT":
        return RAW_EVENT_TOPIC
    if item_kind == "SERIES":
        return RAW_SERIES_METADATA_TOPIC
    raise ValueError(f"unsupported ingestion item kind: {item_kind}")


class ItemStatus(str, Enum):
    ACCEPTED_TO_LOG = "ACCEPTED_TO_LOG"
    DUPLICATE = "DUPLICATE"
    REJECTED_PERMANENT = "REJECTED_PERMANENT"
    RETRYABLE = "RETRYABLE"


@dataclass
class DurableItemState:
    item_kind: str
    content_digest: bytes
    object_persisted: bool = False
    kafka_persisted: bool = False
    terminal_evidence_persisted: bool = False
    object_put_count: int = 0
    kafka_publish_count: int = 0


def durable_status(state: DurableItemState) -> ItemStatus:
    has_required_payload = (
        state.object_persisted if state.item_kind == "SERIES" else True
    )
    if (
        has_required_payload
        and state.kafka_persisted
        and state.terminal_evidence_persisted
    ):
        return ItemStatus.ACCEPTED_TO_LOG
    return ItemStatus.RETRYABLE


class ReliableIngestion:
    """Fault-injectable durable ACK and idempotency reference state machine."""

    def __init__(self) -> None:
        self._states: dict[tuple[str, str], DurableItemState] = {}

    def state(self, item_kind: str, item_id: str) -> DurableItemState:
        return self._states[(item_kind, item_id)]

    def can_clear_outbox(self, item_kind: str, item_id: str) -> bool:
        return durable_status(self.state(item_kind, item_id)) in {
            ItemStatus.ACCEPTED_TO_LOG,
            ItemStatus.DUPLICATE,
        }

    @staticmethod
    def _check_digest(content_digest: bytes) -> None:
        if not isinstance(content_digest, bytes) or len(content_digest) != 32:
            raise ValueError("content digest must be exactly 32 bytes")

    def _get_or_create(
        self, item_kind: str, item_id: str, content_digest: bytes
    ) -> tuple[DurableItemState, ItemStatus | None]:
        self._check_digest(content_digest)
        key = (item_kind, item_id)
        existing = self._states.get(key)
        if existing is None:
            existing = DurableItemState(
                item_kind=item_kind, content_digest=bytes(content_digest)
            )
            self._states[key] = existing
            return existing, None
        if existing.content_digest != content_digest:
            return existing, ItemStatus.REJECTED_PERMANENT
        if durable_status(existing) == ItemStatus.ACCEPTED_TO_LOG:
            return existing, ItemStatus.DUPLICATE
        return existing, None

    def process_event(
        self, event_id: str, content_digest: bytes, fault: str | None = None
    ) -> ItemStatus:
        state, decision = self._get_or_create("EVENT", event_id, content_digest)
        if decision is not None:
            return decision
        if fault == "before_kafka":
            return ItemStatus.RETRYABLE
        if not state.kafka_persisted:
            state.kafka_publish_count += 1
            state.kafka_persisted = True
        if fault == "after_kafka_before_evidence":
            return ItemStatus.RETRYABLE
        state.terminal_evidence_persisted = True
        return ItemStatus.ACCEPTED_TO_LOG

    def process_series(
        self, chunk_id: str, content_digest: bytes, fault: str | None = None
    ) -> ItemStatus:
        state, decision = self._get_or_create("SERIES", chunk_id, content_digest)
        if decision is not None:
            return decision
        if fault == "before_object":
            return ItemStatus.RETRYABLE
        if not state.object_persisted:
            state.object_put_count += 1
            state.object_persisted = True
        if fault == "after_object_before_kafka":
            return ItemStatus.RETRYABLE
        if not state.kafka_persisted:
            state.kafka_publish_count += 1
            state.kafka_persisted = True
        if fault == "after_kafka_before_evidence":
            return ItemStatus.RETRYABLE
        state.terminal_evidence_persisted = True
        return ItemStatus.ACCEPTED_TO_LOG


@dataclass(frozen=True)
class RoutedEvent:
    topic: str
    is_late: bool
    event: dict[str, Any]


def route_by_event_time(
    event: Mapping[str, Any], watermark_ns: int, allowed_lateness_ns: int
) -> RoutedEvent:
    if allowed_lateness_ns < 0:
        raise ValueError("allowed lateness cannot be negative")
    if "observed_at_ns" not in event:
        raise ValueError("event lacks observed_at_ns")
    observed_at_ns = event["observed_at_ns"]
    if not isinstance(observed_at_ns, int) or isinstance(observed_at_ns, bool):
        raise TypeError("observed_at_ns must be an integer")
    is_late = observed_at_ns < watermark_ns - allowed_lateness_ns
    return RoutedEvent(
        topic=LATE_EVENT_TOPIC if is_late else NORMALIZED_EVENT_TOPIC,
        is_late=is_late,
        event=copy.deepcopy(dict(event)),
    )


@dataclass(frozen=True)
class ForegroundObservation:
    event_id: str
    user_id: str
    device_id: str
    application_id: str
    observed_at_ns: int


def derive_application_sessions(
    observations: Iterable[ForegroundObservation],
    *,
    input_snapshot: str,
    processor_version: str,
    rule_version: str,
) -> list[dict[str, Any]]:
    """Produce deterministic closed sessions from foreground transitions."""

    if not input_snapshot or not processor_version or not rule_version:
        raise ValueError("snapshot, processor version and rule version are required")

    grouped: dict[tuple[str, str], list[ForegroundObservation]] = {}
    for observation in observations:
        grouped.setdefault(
            (observation.user_id, observation.device_id), []
        ).append(observation)

    sessions: list[dict[str, Any]] = []
    for (user_id, device_id), values in sorted(grouped.items()):
        ordered = sorted(values, key=lambda item: (item.observed_at_ns, item.event_id))
        if not ordered:
            continue
        current = ordered[0]
        for transition in ordered[1:]:
            if transition.application_id == current.application_id:
                continue
            identity = {
                "user_id": user_id,
                "device_id": device_id,
                "application_id": current.application_id,
                "start_event_id": current.event_id,
                "end_event_id": transition.event_id,
                "start_time_ns": current.observed_at_ns,
                "end_time_ns": transition.observed_at_ns,
                "input_snapshot": input_snapshot,
                "processor_version": processor_version,
                "rule_version": rule_version,
            }
            identity_bytes = json.dumps(
                identity, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ).encode("utf-8")
            sessions.append(
                {
                    "session_id": hashlib.sha256(identity_bytes).hexdigest(),
                    **identity,
                    "duration_ns": transition.observed_at_ns
                    - current.observed_at_ns,
                }
            )
            current = transition
    return sorted(
        sessions,
        key=lambda item: (
            item["user_id"],
            item["device_id"],
            item["start_time_ns"],
            item["session_id"],
        ),
    )


class NonceDecision(str, Enum):
    NEW_BATCH = "NEW_BATCH"
    EXACT_RETRY = "EXACT_RETRY"
    SAFE_RECOMPUTE = "SAFE_RECOMPUTE"
    NONCE_REPLAYED = "NONCE_REPLAYED"


@dataclass
class NonceBinding:
    batch_id: str
    payload_sha256: bytes
    signature: bytes
    acknowledgement: dict[str, Any] | None = None


class NonceLedger:
    """Bind a nonce to an exact signed batch independently per device key."""

    def __init__(self) -> None:
        self._bindings: dict[tuple[str, bytes], NonceBinding] = {}

    @staticmethod
    def _validate_inputs(
        device_key_id: str,
        batch_id: str,
        nonce: bytes,
        payload_sha256: bytes,
        signature: bytes,
    ) -> None:
        if not device_key_id or not batch_id:
            raise ValueError("device key and batch ID are required")
        if not isinstance(nonce, bytes) or not nonce:
            raise ValueError("nonce must be non-empty bytes")
        if not isinstance(payload_sha256, bytes) or len(payload_sha256) != 32:
            raise ValueError("payload_sha256 must be exactly 32 bytes")
        if not isinstance(signature, bytes) or not signature:
            raise ValueError("signature must be non-empty bytes")

    def check_and_bind(
        self,
        *,
        device_key_id: str,
        batch_id: str,
        nonce: bytes,
        payload_sha256: bytes,
        signature: bytes,
    ) -> tuple[NonceDecision, dict[str, Any] | None]:
        self._validate_inputs(
            device_key_id, batch_id, nonce, payload_sha256, signature
        )
        key = (device_key_id, bytes(nonce))
        existing = self._bindings.get(key)
        if existing is None:
            self._bindings[key] = NonceBinding(
                batch_id=batch_id,
                payload_sha256=bytes(payload_sha256),
                signature=bytes(signature),
            )
            return NonceDecision.NEW_BATCH, None

        exact = (
            existing.batch_id == batch_id
            and existing.payload_sha256 == payload_sha256
            and existing.signature == signature
        )
        if not exact:
            return NonceDecision.NONCE_REPLAYED, None
        if existing.acknowledgement is None:
            return NonceDecision.SAFE_RECOMPUTE, None
        return NonceDecision.EXACT_RETRY, copy.deepcopy(existing.acknowledgement)

    def persist_acknowledgement(
        self,
        *,
        device_key_id: str,
        batch_id: str,
        nonce: bytes,
        payload_sha256: bytes,
        signature: bytes,
        acknowledgement: Mapping[str, Any],
    ) -> None:
        decision, _ = self.check_and_bind(
            device_key_id=device_key_id,
            batch_id=batch_id,
            nonce=nonce,
            payload_sha256=payload_sha256,
            signature=signature,
        )
        if decision == NonceDecision.NONCE_REPLAYED:
            raise ValueError("NONCE_REPLAYED")
        binding = self._bindings[(device_key_id, bytes(nonce))]
        binding.acknowledgement = copy.deepcopy(dict(acknowledgement))
