// @ts-nocheck

import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fromBinary } from "@bufbuild/protobuf";
import {
  EventEnvelopeSchema,
} from "../../../../generated/typescript/lifechronicle/events/v1/event_pb.js";
import {
  AppForegroundSchema,
} from "../../../../generated/typescript/lifechronicle/events/v1/payloads_pb.js";

function loadProperties(path: string): Map<string, string> {
  const result = new Map<string, string>();
  const text = readFileSync(path, "utf8");
  for (const rawLine of text.split(/\r?\n/u)) {
    if (rawLine === "" || rawLine.startsWith("#")) {
      continue;
    }
    let separator = -1;
    let escaped = false;
    for (let index = 0; index < rawLine.length; index += 1) {
      const char = rawLine[index];
      if (escaped) {
        escaped = false;
      } else if (char === "\\") {
        escaped = true;
      } else if (char === "=") {
        separator = index;
        break;
      }
    }
    if (separator < 0) {
      throw new Error(`invalid property line: ${rawLine}`);
    }
    const unescape = (value: string): string =>
      value.replace(/\\(.)/gu, (_match, char: string) => {
        if (char === "n") return "\n";
        if (char === "r") return "\r";
        return char;
      });
    result.set(
      unescape(rawLine.slice(0, separator)),
      unescape(rawLine.slice(separator + 1)),
    );
  }
  return result;
}

function required(properties: Map<string, string>, key: string): string {
  const value = properties.get(key);
  if (value === undefined) {
    throw new Error(`missing property ${key}`);
  }
  return value;
}

function hex(value: Uint8Array): string {
  return Buffer.from(value).toString("hex");
}

function fromHex(value: string): Buffer {
  if (value.length % 2 !== 0 || !/^[0-9a-f]*$/u.test(value)) {
    throw new Error(`invalid lowercase hex: ${value}`);
  }
  return Buffer.from(value, "hex");
}

function sha256(value: Uint8Array): Buffer {
  return createHash("sha256").update(value).digest();
}

function u8(value: number): Buffer {
  const result = Buffer.alloc(1);
  result.writeUInt8(value);
  return result;
}

function u32be(value: number): Buffer {
  const result = Buffer.alloc(4);
  result.writeUInt32BE(value);
  return result;
}

function u64be(value: bigint): Buffer {
  const result = Buffer.alloc(8);
  result.writeBigUInt64BE(value);
  return result;
}

function i64be(value: bigint): Buffer {
  const result = Buffer.alloc(8);
  result.writeBigInt64BE(value);
  return result;
}

function framedBytes(value: Uint8Array): Buffer {
  return Buffer.concat([u32be(value.length), Buffer.from(value)]);
}

function framedString(value: string): Buffer {
  return framedBytes(Buffer.from(value, "utf8"));
}

function lcb1(p: Map<string, string>): Buffer {
  return Buffer.concat([
    Buffer.from("LCB1", "ascii"),
    framedString(required(p, "batch.batch_id")),
    framedString(required(p, "batch.device_id")),
    framedString(required(p, "batch.collector_instance_id")),
    u64be(BigInt(required(p, "batch.sequence_start"))),
    u64be(BigInt(required(p, "batch.sequence_end"))),
    i64be(BigInt(required(p, "batch.created_at.seconds"))),
    u32be(Number(required(p, "batch.created_at.nanos"))),
    framedBytes(fromHex(required(p, "batch.nonce_hex"))),
    u32be(Number(required(p, "batch.compression"))),
    framedString(required(p, "batch.source")),
    fromHex(required(p, "batch.payload_sha256_hex")),
  ]);
}

function lce1(p: Map<string, string>): Buffer {
  const endedPresent = required(p, "event.ended_at.present") === "true";
  const parts: Buffer[] = [
    Buffer.from("LCE1", "ascii"),
    framedString(required(p, "event.event_id")),
    framedString(required(p, "event.stream")),
    framedString(required(p, "event.event_type")),
    u32be(Number(required(p, "event.kind"))),
    framedString(required(p, "event.device_id")),
    framedString(required(p, "event.collector_instance_id")),
    framedString(required(p, "event.source")),
    u32be(Number(required(p, "event.schema_version"))),
    u64be(BigInt(required(p, "event.sequence"))),
    i64be(BigInt(required(p, "event.observed_at.seconds"))),
    u32be(Number(required(p, "event.observed_at.nanos"))),
    u8(endedPresent ? 1 : 0),
  ];
  if (endedPresent) {
    parts.push(
      i64be(BigInt(required(p, "event.ended_at.seconds"))),
      u32be(Number(required(p, "event.ended_at.nanos"))),
    );
  }
  parts.push(
    framedString(required(p, "event.timezone")),
    u32be(Number(required(p, "event.privacy_class"))),
    u32be(Number(required(p, "event.retention_class"))),
    framedString(required(p, "event.origin.provider")),
    framedString(required(p, "event.origin.provider_record_id")),
    framedString(required(p, "event.origin.import_id")),
    framedString(required(p, "event.origin.parent_event_id")),
    framedString(required(p, "event.origin.collection_method")),
    framedString(required(p, "event.payload_type_url")),
    framedBytes(fromHex(required(p, "event.payload_value_hex"))),
  );
  return Buffer.concat(parts);
}

function identityFrame(
  magic: string,
  userId: string,
  submittedHash: Uint8Array,
): Buffer {
  return Buffer.concat([
    Buffer.from(magic, "ascii"),
    framedString(userId),
    Buffer.from(submittedHash),
  ]);
}

function main(): void {
  const vectorPath =
    process.argv[2] ?? "contract-tests/vectors/phase0-v1.properties";
  const properties = loadProperties(vectorPath);
  const results = new Map<string, string>();

  const lcb = lcb1(properties);
  const lce = lce1(properties);
  const submittedHash = sha256(lce);
  const userId = required(properties, "series.user_id");
  const lcc = identityFrame("LCC1", userId, submittedHash);
  const submittedSeries = fromHex(
    required(properties, "series.submitted_wire_hex"),
  );
  const lcs = Buffer.concat([
    Buffer.from("LCS1", "ascii"),
    framedBytes(submittedSeries),
  ]);
  const seriesSubmittedHash = sha256(lcs);
  const lcr = identityFrame("LCR1", userId, seriesSubmittedHash);
  const compressed = fromHex(
    required(properties, "series.compressed_payload_hex"),
  );
  const compressedHash = sha256(compressed);
  const kafkaFieldCount = Number(
    required(properties, "kafka_key_fields.count"),
  );
  const kafkaFields = Array.from(
    { length: kafkaFieldCount },
    (_unused, index) =>
      framedString(required(properties, `kafka_key_fields.${index}`)),
  );
  const objectVersion = hex(compressedHash);
  const objectKey =
    `private/${userId}/series/` +
    `${required(properties, "series.chunk_id")}/${objectVersion}.zst`;

  results.set("language", "typescript");
  results.set("lcb1_hex", hex(lcb));
  results.set("lce1_hex", hex(lce));
  results.set("lcc1_hex", hex(lcc));
  results.set("lcs1_hex", hex(lcs));
  results.set("lcr1_hex", hex(lcr));
  results.set("submitted_sha256_hex", hex(submittedHash));
  results.set("canonical_sha256_hex", hex(sha256(lcc)));
  results.set("series_submitted_sha256_hex", hex(seriesSubmittedHash));
  results.set("series_canonical_sha256_hex", hex(sha256(lcr)));
  results.set("kafka_key_hex", hex(Buffer.concat(kafkaFields)));
  results.set("compressed_size", String(compressed.length));
  results.set("compressed_sha256_hex", objectVersion);
  results.set("object_version", objectVersion);
  results.set("object_key", objectKey);

  const event = fromBinary(
    EventEnvelopeSchema,
    fromHex(required(properties, "protobuf_event_hex")),
  );
  if (event.observedAt === undefined) {
    throw new Error("decoded event is missing observed_at");
  }
  if (event.origin === undefined || event.payload === undefined) {
    throw new Error("decoded event is missing origin or payload");
  }
  const payload = fromBinary(AppForegroundSchema, event.payload.value);
  results.set("decoded.event_id", event.eventId);
  results.set("decoded.stream", event.stream);
  results.set("decoded.event_type", event.eventType);
  results.set("decoded.kind", String(event.kind));
  results.set("decoded.user_id", event.userId);
  results.set("decoded.device_id", event.deviceId);
  results.set(
    "decoded.collector_instance_id",
    event.collectorInstanceId,
  );
  results.set("decoded.source", event.source);
  results.set("decoded.schema_version", String(event.schemaVersion));
  results.set("decoded.sequence", String(event.sequence));
  results.set(
    "decoded.observed_at.seconds",
    String(event.observedAt.seconds),
  );
  results.set(
    "decoded.observed_at.nanos",
    String(event.observedAt.nanos),
  );
  results.set("decoded.ended_at.present", String(event.endedAt !== undefined));
  results.set("decoded.timezone", event.timezone);
  results.set("decoded.privacy_class", String(event.privacyClass));
  results.set("decoded.retention_class", String(event.retentionClass));
  results.set("decoded.origin.provider", event.origin.provider);
  results.set(
    "decoded.origin.provider_record_id",
    event.origin.providerRecordId,
  );
  results.set("decoded.origin.import_id", event.origin.importId);
  results.set(
    "decoded.origin.parent_event_id",
    event.origin.parentEventId,
  );
  results.set(
    "decoded.origin.collection_method",
    event.origin.collectionMethod,
  );
  results.set("decoded.payload_type_url", event.payload.typeUrl);
  results.set("decoded.payload_value_hex", hex(event.payload.value));
  results.set(
    "decoded.payload.application_id",
    payload.applicationId,
  );
  results.set(
    "decoded.payload.application_name",
    payload.applicationName,
  );
  results.set(
    "decoded.payload.executable_name",
    payload.executableName,
  );
  results.set(
    "decoded.payload.window_title_utf8_hex",
    hex(Buffer.from(payload.windowTitle, "utf8")),
  );
  results.set("decoded.payload.process_id", String(payload.processId));
  results.set("decoded.payload.window_id", payload.windowId);
  results.set("decoded.payload.fullscreen", String(payload.fullscreen));

  const ordered = [
    ["language", required(results, "language")],
    ...[...results.entries()]
      .filter(([key]) => key !== "language")
      .sort(([left], [right]) => left.localeCompare(right)),
  ];
  process.stdout.write(
    ordered.map(([key, value]) => `${key}=${value}\n`).join(""),
  );
}

main();
