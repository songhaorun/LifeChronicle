from __future__ import annotations

import hashlib
import itertools
import json
import subprocess
import sys
import unittest
from pathlib import Path

from contract_semantics import (
    LATE_EVENT_TOPIC,
    RAW_EVENT_TOPIC,
    RAW_SERIES_METADATA_TOPIC,
    DurableItemState,
    EventCandidate,
    ForegroundObservation,
    ItemStatus,
    NonceDecision,
    NonceLedger,
    ReliableIngestion,
    Timestamp,
    derive_application_sessions,
    durable_status,
    encode_partition_key,
    ingestion_topic,
    route_by_event_time,
    validate_event_against_stream,
)


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
REGISTRY_SCRIPT = REPOSITORY_ROOT / "scripts" / "validate_registry.py"
REGISTRY_ROOT = REPOSITORY_ROOT / "registry"


def load_json_profile(relative_path: str) -> dict:
    with (REPOSITORY_ROOT / relative_path).open("r", encoding="utf-8") as handle:
        return json.load(handle)


def run_registry(*relative_paths: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(REGISTRY_SCRIPT),
            "--quiet",
            *(str(REPOSITORY_ROOT / path) for path in relative_paths),
        ],
        cwd=REPOSITORY_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


class RegistryContractTests(unittest.TestCase):
    def test_ES_C005_registry_schema_accepts_valid_and_rejects_targeted_cases(self):
        valid = run_registry("registry/streams/app.foreground/v1.yaml")
        self.assertEqual(valid.returncode, 0, valid.stderr)

        negative_cases = {
            "registry/fixtures/invalid/unknown-field.json": "unknown field",
            "registry/fixtures/invalid/unknown-nested-field.json": "unknown field",
            "registry/fixtures/invalid/invalid-name.json": "does not match",
            "registry/fixtures/invalid/missing-privacy.json": "privacy_class",
        }
        for path, message in negative_cases.items():
            with self.subTest(path=path):
                rejected = run_registry(path)
                self.assertNotEqual(rejected.returncode, 0)
                self.assertIn(message, rejected.stderr)

    def test_ES_C006_payload_type_url_must_match_stream_registry(self):
        definition = load_json_profile(
            "registry/streams/app.foreground/v1.yaml"
        )
        valid = EventCandidate(
            stream="app.foreground",
            event_type="app.foreground.observed",
            kind="STATE",
            schema_version=1,
            type_url=(
                "type.googleapis.com/lifechronicle.events.v1.AppForeground"
            ),
            observed_at=Timestamp(1_800_000_000, 123),
        )
        self.assertEqual(validate_event_against_stream(valid, definition), [])

        wrong_payload = EventCandidate(
            **{
                **valid.__dict__,
                "type_url": (
                    "type.googleapis.com/lifechronicle.events.v1.ScreenState"
                ),
            }
        )
        errors = validate_event_against_stream(wrong_payload, definition)
        self.assertTrue(any("payload.type_url" in error for error in errors))

    def test_ES_C007_timestamp_bounds_and_interval_order_are_enforced(self):
        definition = load_json_profile(
            "registry/streams/app.foreground/v1.yaml"
        )
        invalid_timestamp = EventCandidate(
            stream="app.foreground",
            event_type="app.foreground.observed",
            kind="STATE",
            schema_version=1,
            type_url=(
                "type.googleapis.com/lifechronicle.events.v1.AppForeground"
            ),
            observed_at=Timestamp(253_402_300_800, 1_000_000_000),
        )
        timestamp_errors = validate_event_against_stream(
            invalid_timestamp, definition
        )
        self.assertTrue(any(".seconds" in error for error in timestamp_errors))
        self.assertTrue(any(".nanos" in error for error in timestamp_errors))

        interval_definition = {
            **definition,
            "name": "user.presence",
            "record_kind": "INTERVAL",
            "schema": {
                "version": 1,
                "payload_type": "lifechronicle.events.v1.PresenceInterval",
                "event_type": "user.presence.observed",
            },
        }
        reverse_interval = EventCandidate(
            stream="user.presence",
            event_type="user.presence.observed",
            kind="INTERVAL",
            schema_version=1,
            type_url=(
                "type.googleapis.com/lifechronicle.events.v1.PresenceInterval"
            ),
            observed_at=Timestamp(100, 1),
            ended_at=Timestamp(99, 999_999_999),
        )
        reverse_errors = validate_event_against_stream(
            reverse_interval, interval_definition
        )
        self.assertIn("ended_at cannot precede observed_at", reverse_errors)

        open_interval = EventCandidate(
            **{**reverse_interval.__dict__, "ended_at": None}
        )
        self.assertEqual(
            validate_event_against_stream(open_interval, interval_definition), []
        )

        state_with_end = EventCandidate(
            stream="app.foreground",
            event_type="app.foreground.observed",
            kind="STATE",
            schema_version=1,
            type_url=(
                "type.googleapis.com/lifechronicle.events.v1.AppForeground"
            ),
            observed_at=Timestamp(100, 0),
            ended_at=Timestamp(101, 0),
        )
        self.assertIn(
            "non-INTERVAL record cannot set ended_at",
            validate_event_against_stream(state_with_end, definition),
        )

    def test_ES_C008_same_id_is_duplicate_only_for_same_durable_content(self):
        ingestion = ReliableIngestion()
        event_id = "018f3e88-6bec-7a24-8000-000000000001"
        digest = hashlib.sha256(b"canonical-event-a").digest()
        different = hashlib.sha256(b"canonical-event-b").digest()

        self.assertEqual(
            ingestion.process_event(event_id, digest),
            ItemStatus.ACCEPTED_TO_LOG,
        )
        self.assertEqual(
            ingestion.process_event(event_id, digest), ItemStatus.DUPLICATE
        )
        self.assertEqual(
            ingestion.process_event(event_id, different),
            ItemStatus.REJECTED_PERMANENT,
        )

        pending = ReliableIngestion()
        self.assertEqual(
            pending.process_event(event_id, digest, fault="before_kafka"),
            ItemStatus.RETRYABLE,
        )
        self.assertEqual(
            pending.process_event(event_id, digest),
            ItemStatus.ACCEPTED_TO_LOG,
        )

    def test_ES_C009_partition_key_is_length_framed_and_requires_every_field(self):
        values = {
            "user_id": "a",
            "device_id": "bc",
            "collector_instance_id": "设备",
        }
        encoded = encode_partition_key(
            ["user_id", "device_id", "collector_instance_id"], values
        )
        self.assertEqual(
            encoded.hex(),
            "000000016100000002626300000006e8aebee5a487",
        )
        self.assertNotEqual(
            encode_partition_key(["user_id", "device_id"], values),
            encode_partition_key(
                ["user_id", "device_id"],
                {"user_id": "ab", "device_id": "c"},
            ),
        )
        with self.assertRaisesRegex(ValueError, "collector_instance_id"):
            encode_partition_key(
                ["user_id", "device_id", "collector_instance_id"],
                {"user_id": "a", "device_id": "bc"},
            )

    def test_ES_C010_event_and_series_use_distinct_ingestion_topics(self):
        self.assertEqual(ingestion_topic("EVENT"), RAW_EVENT_TOPIC)
        self.assertEqual(
            ingestion_topic("SERIES"), RAW_SERIES_METADATA_TOPIC
        )
        self.assertNotEqual(ingestion_topic("SERIES"), RAW_EVENT_TOPIC)
        with self.assertRaises(ValueError):
            ingestion_topic("SERIES_EXPANDED_AS_EVENTS")

    def test_ES_C011_ack_requires_kafka_and_recoverable_terminal_evidence(self):
        digest = hashlib.sha256(b"event-after-crash").digest()
        event_id = "018f3e88-6bec-7a24-8000-000000000002"
        ingestion = ReliableIngestion()

        interrupted = ingestion.process_event(
            event_id, digest, fault="after_kafka_before_evidence"
        )
        state = ingestion.state("EVENT", event_id)
        self.assertEqual(interrupted, ItemStatus.RETRYABLE)
        self.assertTrue(state.kafka_persisted)
        self.assertFalse(state.terminal_evidence_persisted)
        self.assertFalse(ingestion.can_clear_outbox("EVENT", event_id))

        recovered = ingestion.process_event(event_id, digest)
        self.assertEqual(recovered, ItemStatus.ACCEPTED_TO_LOG)
        self.assertEqual(state.kafka_publish_count, 1)
        self.assertTrue(state.terminal_evidence_persisted)
        self.assertTrue(ingestion.can_clear_outbox("EVENT", event_id))

    def test_ES_C011_no_kafka_confirmation_never_yields_accepted(self):
        ingestion = ReliableIngestion()
        digest = hashlib.sha256(b"kafka-unavailable").digest()
        status = ingestion.process_event(
            "018f3e88-6bec-7a24-8000-000000000003",
            digest,
            fault="before_kafka",
        )
        self.assertEqual(status, ItemStatus.RETRYABLE)
        self.assertFalse(
            ingestion.state(
                "EVENT", "018f3e88-6bec-7a24-8000-000000000003"
            ).kafka_persisted
        )

    def test_ES_C012_templates_default_private_and_missing_privacy_is_rejected(self):
        for path in (
            "registry/templates/stream-v1.yaml",
            "registry/templates/metric-v1.yaml",
        ):
            with self.subTest(path=path):
                template = load_json_profile(path)
                self.assertEqual(
                    template["defaults"]["privacy_class"], "PRIVATE"
                )
                self.assertEqual(
                    template["privacy"]["public_projection"], "none"
                )
                accepted = run_registry(path)
                self.assertEqual(accepted.returncode, 0, accepted.stderr)

        missing = run_registry(
            "registry/fixtures/invalid/missing-privacy.json"
        )
        self.assertNotEqual(missing.returncode, 0)
        self.assertIn("privacy_class", missing.stderr)

    def test_ES_C013_over_lateness_window_routes_to_late_topic_without_drop(self):
        event = {
            "event_id": "018f3e88-6bec-7a24-8000-000000000004",
            "observed_at_ns": 99,
            "payload": {"application_id": "editor"},
        }
        routed = route_by_event_time(
            event, watermark_ns=200, allowed_lateness_ns=100
        )
        self.assertTrue(routed.is_late)
        self.assertEqual(routed.topic, LATE_EVENT_TOPIC)
        self.assertEqual(routed.event, event)

        boundary = route_by_event_time(
            {**event, "observed_at_ns": 100},
            watermark_ns=200,
            allowed_lateness_ns=100,
        )
        self.assertFalse(boundary.is_late)

    def test_ES_C014_replay_is_deterministic_across_arrival_order(self):
        observations = [
            ForegroundObservation("event-c", "user", "device", "browser", 30),
            ForegroundObservation("event-a", "user", "device", "editor", 10),
            ForegroundObservation("event-b", "user", "device", "browser", 20),
            ForegroundObservation("event-d", "user", "device", "terminal", 40),
        ]
        arguments = {
            "input_snapshot": "topic=lc.raw.events.v1;p0=100-103",
            "processor_version": "application-sessionization@1.0.0",
            "rule_version": "session-rules@1",
        }
        first = derive_application_sessions(observations, **arguments)
        second = derive_application_sessions(reversed(observations), **arguments)
        third = derive_application_sessions(observations, **arguments)

        self.assertEqual(first, second)
        self.assertEqual(first, third)
        self.assertEqual(
            json.dumps(first, sort_keys=True, separators=(",", ":")),
            json.dumps(second, sort_keys=True, separators=(",", ":")),
        )
        self.assertTrue(all(session["duration_ns"] >= 0 for session in first))

    def test_ES_C015_series_ack_is_atomic_across_all_durable_boundaries(self):
        digest = hashlib.sha256(b"series-wire-bytes").digest()
        for object_ok, kafka_ok, evidence_ok in itertools.product(
            (False, True), repeat=3
        ):
            with self.subTest(
                object=object_ok, kafka=kafka_ok, evidence=evidence_ok
            ):
                state = DurableItemState(
                    item_kind="SERIES",
                    content_digest=digest,
                    object_persisted=object_ok,
                    kafka_persisted=kafka_ok,
                    terminal_evidence_persisted=evidence_ok,
                )
                expected = (
                    ItemStatus.ACCEPTED_TO_LOG
                    if object_ok and kafka_ok and evidence_ok
                    else ItemStatus.RETRYABLE
                )
                self.assertEqual(durable_status(state), expected)

    def test_ES_C015_series_retry_reuses_object_and_metadata_after_crashes(self):
        ingestion = ReliableIngestion()
        chunk_id = "018f3e88-6bec-7a24-8000-000000000005"
        digest = hashlib.sha256(b"series-wire-bytes").digest()

        self.assertEqual(
            ingestion.process_series(
                chunk_id, digest, fault="after_object_before_kafka"
            ),
            ItemStatus.RETRYABLE,
        )
        self.assertFalse(ingestion.can_clear_outbox("SERIES", chunk_id))
        self.assertEqual(
            ingestion.process_series(
                chunk_id, digest, fault="after_kafka_before_evidence"
            ),
            ItemStatus.RETRYABLE,
        )
        self.assertFalse(ingestion.can_clear_outbox("SERIES", chunk_id))
        self.assertEqual(
            ingestion.process_series(chunk_id, digest),
            ItemStatus.ACCEPTED_TO_LOG,
        )
        state = ingestion.state("SERIES", chunk_id)
        self.assertEqual(state.object_put_count, 1)
        self.assertEqual(state.kafka_publish_count, 1)
        self.assertTrue(ingestion.can_clear_outbox("SERIES", chunk_id))

    def test_ES_C016_exact_retry_returns_persisted_ack_or_safe_recompute(self):
        ledger = NonceLedger()
        request = {
            "device_key_id": "device-key-1",
            "batch_id": "018f3e88-6bec-7a24-8000-000000000006",
            "nonce": b"0123456789abcdef",
            "payload_sha256": hashlib.sha256(b"batch-payload").digest(),
            "signature": b"signed-batch",
        }
        decision, ack = ledger.check_and_bind(**request)
        self.assertEqual(decision, NonceDecision.NEW_BATCH)
        self.assertIsNone(ack)

        decision, ack = ledger.check_and_bind(**request)
        self.assertEqual(decision, NonceDecision.SAFE_RECOMPUTE)
        self.assertIsNone(ack)

        persisted_ack = {
            "batch_id": request["batch_id"],
            "items": [{"id": "event-1", "status": "ACCEPTED_TO_LOG"}],
        }
        ledger.persist_acknowledgement(**request, acknowledgement=persisted_ack)
        decision, ack = ledger.check_and_bind(**request)
        self.assertEqual(decision, NonceDecision.EXACT_RETRY)
        self.assertEqual(ack, persisted_ack)

    def test_ES_C016_nonce_conflicts_on_batch_digest_or_signature_changes(self):
        base = {
            "device_key_id": "device-key-2",
            "batch_id": "018f3e88-6bec-7a24-8000-000000000007",
            "nonce": b"same-nonce",
            "payload_sha256": hashlib.sha256(b"payload-a").digest(),
            "signature": b"signature-a",
        }
        mutations = (
            {"batch_id": "018f3e88-6bec-7a24-8000-000000000008"},
            {"payload_sha256": hashlib.sha256(b"payload-b").digest()},
            {"signature": b"signature-b"},
        )
        for mutation in mutations:
            with self.subTest(mutation=mutation):
                ledger = NonceLedger()
                self.assertEqual(
                    ledger.check_and_bind(**base)[0],
                    NonceDecision.NEW_BATCH,
                )
                conflicted = {**base, **mutation}
                self.assertEqual(
                    ledger.check_and_bind(**conflicted)[0],
                    NonceDecision.NONCE_REPLAYED,
                )

    def test_ES_C016_nonce_scope_is_per_device_key(self):
        ledger = NonceLedger()
        common = {
            "batch_id": "018f3e88-6bec-7a24-8000-000000000009",
            "nonce": b"same-nonce",
            "payload_sha256": hashlib.sha256(b"payload").digest(),
            "signature": b"signature",
        }
        self.assertEqual(
            ledger.check_and_bind(device_key_id="key-a", **common)[0],
            NonceDecision.NEW_BATCH,
        )
        self.assertEqual(
            ledger.check_and_bind(device_key_id="key-b", **common)[0],
            NonceDecision.NEW_BATCH,
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
