import com.google.protobuf.Any;
import io.lifechronicle.events.v1.AppForeground;
import io.lifechronicle.events.v1.EventEnvelope;
import io.lifechronicle.events.v1.Origin;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.OutputStreamWriter;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.LinkedHashMap;
import java.util.Locale;
import java.util.Map;
import java.util.Properties;

/**
 * Independent Java implementation of the Phase 0 golden-vector framing rules.
 *
 * <p>The expected.* properties are deliberately never read. The runner derives
 * every emitted digest and frame from the input fields, then uses the generated
 * Java Protobuf bindings for the decode half of the contract.
 */
public final class Phase0Runner {
  private static final String VECTOR_NAME = "phase0-v1.properties";

  private Phase0Runner() {}

  public static void main(String[] args) throws Exception {
    Path vectorPath = locateVector(args);
    Properties input = loadProperties(vectorPath);
    LinkedHashMap<String, String> output = calculate(input);
    emit(output);
  }

  private static LinkedHashMap<String, String> calculate(Properties input)
      throws Exception {
    byte[] compressedItems = decodeHex(required(input, "batch.compressed_items_hex"));
    byte[] batchPayloadHash = sha256(compressedItems);
    requireEqualHex(
        "batch.payload_sha256_hex",
        required(input, "batch.payload_sha256_hex"),
        batchPayloadHash);

    byte[] lcb1 = buildLcb1(input, batchPayloadHash);
    byte[] lce1 = buildLce1(input);
    byte[] submittedHash = sha256(lce1);

    String canonicalUserId = required(input, "kafka_key_fields.0");
    byte[] lcc1 = buildIdentityFrame("LCC1", canonicalUserId, submittedHash);
    byte[] canonicalHash = sha256(lcc1);

    byte[] rawSeriesPayload = decodeHex(required(input, "series.raw_payload_hex"));
    byte[] seriesChecksum = sha256(rawSeriesPayload);
    requireEqualHex(
        "series.checksum_hex",
        required(input, "series.checksum_hex"),
        seriesChecksum);

    byte[] submittedSeriesWire =
        decodeHex(required(input, "series.submitted_wire_hex"));
    byte[] lcs1 = buildFramedPayload("LCS1", submittedSeriesWire);
    byte[] seriesSubmittedHash = sha256(lcs1);
    byte[] lcr1 =
        buildIdentityFrame(
            "LCR1",
            required(input, "series.user_id"),
            seriesSubmittedHash);
    byte[] seriesCanonicalHash = sha256(lcr1);

    byte[] compressedSeries =
        decodeHex(required(input, "series.compressed_payload_hex"));
    byte[] compressedHash = sha256(compressedSeries);
    String objectVersion = encodeHex(compressedHash);
    String objectKey =
        "private/"
            + required(input, "series.user_id")
            + "/series/"
            + required(input, "series.chunk_id")
            + "/"
            + objectVersion
            + ".zst";

    byte[] kafkaKey = buildKafkaKey(input);

    LinkedHashMap<String, String> output = new LinkedHashMap<>();
    output.put("language", "java");
    output.put("lcb1_hex", encodeHex(lcb1));
    output.put("lce1_hex", encodeHex(lce1));
    output.put("lcc1_hex", encodeHex(lcc1));
    output.put("lcs1_hex", encodeHex(lcs1));
    output.put("lcr1_hex", encodeHex(lcr1));
    output.put("submitted_sha256_hex", encodeHex(submittedHash));
    output.put("canonical_sha256_hex", encodeHex(canonicalHash));
    output.put(
        "series_submitted_sha256_hex", encodeHex(seriesSubmittedHash));
    output.put(
        "series_canonical_sha256_hex", encodeHex(seriesCanonicalHash));
    output.put("compressed_sha256_hex", objectVersion);
    output.put("kafka_key_hex", encodeHex(kafkaKey));
    output.put("compressed_size", Integer.toString(compressedSeries.length));
    output.put("object_version", objectVersion);
    output.put("object_key", objectKey);

    decodeEvent(input, output);
    return output;
  }

  private static byte[] buildLcb1(Properties input, byte[] payloadHash) {
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    appendAscii(output, "LCB1");
    appendFramedString(output, required(input, "batch.batch_id"));
    appendFramedString(output, required(input, "batch.device_id"));
    appendFramedString(
        output, required(input, "batch.collector_instance_id"));
    appendU64(output, parseU64(input, "batch.sequence_start"));
    appendU64(output, parseU64(input, "batch.sequence_end"));
    appendI64(output, parseI64(input, "batch.created_at.seconds"));
    appendU32(output, parseU32(input, "batch.created_at.nanos"));
    appendFramedBytes(output, decodeHex(required(input, "batch.nonce_hex")));
    appendU32(output, parseU32(input, "batch.compression"));
    appendFramedString(output, required(input, "batch.source"));
    append(output, payloadHash);
    return output.toByteArray();
  }

  private static byte[] buildLce1(Properties input) {
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    appendAscii(output, "LCE1");
    appendFramedString(output, required(input, "event.event_id"));
    appendFramedString(output, required(input, "event.stream"));
    appendFramedString(output, required(input, "event.event_type"));
    appendU32(output, parseU32(input, "event.kind"));
    appendFramedString(output, required(input, "event.device_id"));
    appendFramedString(
        output, required(input, "event.collector_instance_id"));
    appendFramedString(output, required(input, "event.source"));
    appendU32(output, parseU32(input, "event.schema_version"));
    appendU64(output, parseU64(input, "event.sequence"));
    appendI64(output, parseI64(input, "event.observed_at.seconds"));
    appendU32(output, parseU32(input, "event.observed_at.nanos"));

    boolean endedAtPresent =
        parseBoolean(input, "event.ended_at.present");
    output.write(endedAtPresent ? 1 : 0);
    if (endedAtPresent) {
      appendI64(output, parseI64(input, "event.ended_at.seconds"));
      appendU32(output, parseU32(input, "event.ended_at.nanos"));
    }

    appendFramedString(output, required(input, "event.timezone"));
    appendU32(output, parseU32(input, "event.privacy_class"));
    appendU32(output, parseU32(input, "event.retention_class"));
    appendFramedString(output, required(input, "event.origin.provider"));
    appendFramedString(
        output, required(input, "event.origin.provider_record_id"));
    appendFramedString(output, required(input, "event.origin.import_id"));
    appendFramedString(
        output, required(input, "event.origin.parent_event_id"));
    appendFramedString(
        output, required(input, "event.origin.collection_method"));
    appendFramedString(output, required(input, "event.payload_type_url"));
    appendFramedBytes(
        output, decodeHex(required(input, "event.payload_value_hex")));
    return output.toByteArray();
  }

  private static byte[] buildIdentityFrame(
      String magic, String userId, byte[] submittedHash) {
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    appendAscii(output, magic);
    appendFramedString(output, userId);
    append(output, submittedHash);
    return output.toByteArray();
  }

  private static byte[] buildFramedPayload(String magic, byte[] payload) {
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    appendAscii(output, magic);
    appendFramedBytes(output, payload);
    return output.toByteArray();
  }

  private static byte[] buildKafkaKey(Properties input) {
    int count = parseNonNegativeInt(input, "kafka_key_fields.count");
    if (count == 0) {
      throw new IllegalArgumentException(
          "kafka_key_fields.count must be greater than zero");
    }
    ByteArrayOutputStream output = new ByteArrayOutputStream();
    for (int index = 0; index < count; index++) {
      appendFramedString(
          output, required(input, "kafka_key_fields." + index));
    }
    return output.toByteArray();
  }

  private static void decodeEvent(
      Properties input, LinkedHashMap<String, String> output)
      throws Exception {
    byte[] wire = decodeHex(required(input, "protobuf_event_hex"));
    EventEnvelope event = EventEnvelope.parseFrom(wire);
    if (!event.hasObservedAt()) {
      throw new IllegalArgumentException(
          "protobuf_event_hex has no observed_at");
    }
    if (!event.hasOrigin()) {
      throw new IllegalArgumentException("protobuf_event_hex has no origin");
    }
    if (!event.hasPayload()) {
      throw new IllegalArgumentException("protobuf_event_hex has no payload");
    }

    Any anyPayload = event.getPayload();
    if (!anyPayload.is(AppForeground.class)) {
      throw new IllegalArgumentException(
          "protobuf_event_hex payload is not AppForeground: "
              + anyPayload.getTypeUrl());
    }
    AppForeground payload = anyPayload.unpack(AppForeground.class);
    Origin origin = event.getOrigin();

    output.put("decoded.event_id", event.getEventId());
    output.put("decoded.stream", event.getStream());
    output.put("decoded.event_type", event.getEventType());
    output.put("decoded.kind", Integer.toString(event.getKindValue()));
    output.put("decoded.user_id", event.getUserId());
    output.put("decoded.device_id", event.getDeviceId());
    output.put(
        "decoded.collector_instance_id", event.getCollectorInstanceId());
    output.put("decoded.source", event.getSource());
    output.put(
        "decoded.schema_version",
        Integer.toUnsignedString(event.getSchemaVersion()));
    output.put(
        "decoded.sequence", Long.toUnsignedString(event.getSequence()));
    output.put(
        "decoded.observed_at.seconds",
        Long.toString(event.getObservedAt().getSeconds()));
    output.put(
        "decoded.observed_at.nanos",
        Integer.toString(event.getObservedAt().getNanos()));
    output.put("decoded.ended_at.present", Boolean.toString(event.hasEndedAt()));
    output.put("decoded.timezone", event.getTimezone());
    output.put(
        "decoded.privacy_class",
        Integer.toString(event.getPrivacyClassValue()));
    output.put(
        "decoded.retention_class",
        Integer.toString(event.getRetentionClassValue()));
    output.put("decoded.origin.provider", origin.getProvider());
    output.put(
        "decoded.origin.provider_record_id", origin.getProviderRecordId());
    output.put("decoded.origin.import_id", origin.getImportId());
    output.put(
        "decoded.origin.parent_event_id", origin.getParentEventId());
    output.put(
        "decoded.origin.collection_method", origin.getCollectionMethod());
    output.put("decoded.payload_type_url", anyPayload.getTypeUrl());
    output.put(
        "decoded.payload_value_hex",
        encodeHex(anyPayload.getValue().toByteArray()));
    output.put(
        "decoded.payload.application_id", payload.getApplicationId());
    output.put(
        "decoded.payload.application_name", payload.getApplicationName());
    output.put(
        "decoded.payload.executable_name", payload.getExecutableName());
    output.put(
        "decoded.payload.window_title_utf8_hex",
        encodeHex(payload.getWindowTitle().getBytes(StandardCharsets.UTF_8)));
    output.put(
        "decoded.payload.process_id",
        Long.toUnsignedString(payload.getProcessId()));
    output.put("decoded.payload.window_id", payload.getWindowId());
    output.put(
        "decoded.payload.fullscreen",
        Boolean.toString(payload.getFullscreen()));
  }

  private static Properties loadProperties(Path path) throws IOException {
    Properties properties = new Properties();
    try (var reader = Files.newBufferedReader(path, StandardCharsets.UTF_8)) {
      properties.load(reader);
    }
    return properties;
  }

  private static Path locateVector(String[] args) {
    if (args.length > 1) {
      throw new IllegalArgumentException(
          "usage: Phase0Runner [path/to/" + VECTOR_NAME + "]");
    }
    if (args.length == 1) {
      Path explicit = Path.of(args[0]).toAbsolutePath().normalize();
      if (!Files.isRegularFile(explicit)) {
        throw new IllegalArgumentException(
            "vector file does not exist: " + explicit);
      }
      return explicit;
    }

    Path cursor = Path.of("").toAbsolutePath().normalize();
    while (cursor != null) {
      Path repositoryCandidate =
          cursor.resolve("contract-tests").resolve("vectors").resolve(VECTOR_NAME);
      if (Files.isRegularFile(repositoryCandidate)) {
        return repositoryCandidate;
      }
      Path contractTestsCandidate =
          cursor.resolve("vectors").resolve(VECTOR_NAME);
      if (Files.isRegularFile(contractTestsCandidate)) {
        return contractTestsCandidate;
      }
      cursor = cursor.getParent();
    }
    throw new IllegalArgumentException(
        "cannot locate contract-tests/vectors/" + VECTOR_NAME);
  }

  private static String required(Properties input, String key) {
    if (key.startsWith("expected.")) {
      throw new IllegalArgumentException(
          "runner code must not read expected.* properties: " + key);
    }
    String value = input.getProperty(key);
    if (value == null) {
      throw new IllegalArgumentException("missing vector property: " + key);
    }
    return value;
  }

  private static int parseNonNegativeInt(Properties input, String key) {
    String raw = required(input, key);
    try {
      int value = Integer.parseInt(raw);
      if (value < 0) {
        throw new IllegalArgumentException(
            key + " must be non-negative: " + raw);
      }
      return value;
    } catch (NumberFormatException exc) {
      throw new IllegalArgumentException(key + " is not an integer: " + raw, exc);
    }
  }

  private static long parseU32(Properties input, String key) {
    String raw = required(input, key);
    try {
      long value = Long.parseLong(raw);
      if (value < 0 || value > 0xffff_ffffL) {
        throw new IllegalArgumentException(
            key + " is outside uint32: " + raw);
      }
      return value;
    } catch (NumberFormatException exc) {
      throw new IllegalArgumentException(key + " is not uint32: " + raw, exc);
    }
  }

  private static long parseU64(Properties input, String key) {
    String raw = required(input, key);
    try {
      return Long.parseUnsignedLong(raw);
    } catch (NumberFormatException exc) {
      throw new IllegalArgumentException(key + " is not uint64: " + raw, exc);
    }
  }

  private static long parseI64(Properties input, String key) {
    String raw = required(input, key);
    try {
      return Long.parseLong(raw);
    } catch (NumberFormatException exc) {
      throw new IllegalArgumentException(key + " is not int64: " + raw, exc);
    }
  }

  private static boolean parseBoolean(Properties input, String key) {
    String raw = required(input, key);
    if (raw.equals("true")) {
      return true;
    }
    if (raw.equals("false")) {
      return false;
    }
    throw new IllegalArgumentException(key + " is not true or false: " + raw);
  }

  private static void requireEqualHex(
      String key, String declaredHex, byte[] calculated) {
    String normalized = declaredHex.toLowerCase(Locale.ROOT);
    String actual = encodeHex(calculated);
    if (!normalized.equals(actual)) {
      throw new IllegalArgumentException(
          key + " does not match independently calculated SHA-256");
    }
  }

  private static byte[] sha256(byte[] value) {
    try {
      return MessageDigest.getInstance("SHA-256").digest(value);
    } catch (NoSuchAlgorithmException exc) {
      throw new AssertionError("JVM has no SHA-256 provider", exc);
    }
  }

  private static byte[] decodeHex(String value) {
    if ((value.length() & 1) != 0) {
      throw new IllegalArgumentException("hex value has odd length");
    }
    byte[] output = new byte[value.length() / 2];
    for (int index = 0; index < output.length; index++) {
      int high = Character.digit(value.charAt(index * 2), 16);
      int low = Character.digit(value.charAt(index * 2 + 1), 16);
      if (high < 0 || low < 0) {
        throw new IllegalArgumentException(
            "hex value contains a non-hexadecimal character");
      }
      output[index] = (byte) ((high << 4) | low);
    }
    return output;
  }

  private static String encodeHex(byte[] value) {
    char[] digits = "0123456789abcdef".toCharArray();
    char[] output = new char[value.length * 2];
    for (int index = 0; index < value.length; index++) {
      int unsigned = value[index] & 0xff;
      output[index * 2] = digits[unsigned >>> 4];
      output[index * 2 + 1] = digits[unsigned & 0x0f];
    }
    return new String(output);
  }

  private static void appendAscii(ByteArrayOutputStream output, String value) {
    append(output, value.getBytes(StandardCharsets.US_ASCII));
  }

  private static void appendFramedString(
      ByteArrayOutputStream output, String value) {
    appendFramedBytes(output, value.getBytes(StandardCharsets.UTF_8));
  }

  private static void appendFramedBytes(
      ByteArrayOutputStream output, byte[] value) {
    appendU32(output, value.length);
    append(output, value);
  }

  private static void appendU32(ByteArrayOutputStream output, long value) {
    if (value < 0 || value > 0xffff_ffffL) {
      throw new IllegalArgumentException("value is outside uint32: " + value);
    }
    output.write((int) (value >>> 24) & 0xff);
    output.write((int) (value >>> 16) & 0xff);
    output.write((int) (value >>> 8) & 0xff);
    output.write((int) value & 0xff);
  }

  private static void appendU64(ByteArrayOutputStream output, long value) {
    appendI64(output, value);
  }

  private static void appendI64(ByteArrayOutputStream output, long value) {
    for (int shift = 56; shift >= 0; shift -= 8) {
      output.write((int) (value >>> shift) & 0xff);
    }
  }

  private static void append(ByteArrayOutputStream output, byte[] value) {
    output.writeBytes(value);
  }

  private static void emit(LinkedHashMap<String, String> output) {
    try (PrintWriter writer =
        new PrintWriter(
            new OutputStreamWriter(System.out, StandardCharsets.UTF_8), false)) {
      for (Map.Entry<String, String> entry : output.entrySet()) {
        String key = entry.getKey();
        String value = entry.getValue();
        if (key.indexOf('=') >= 0
            || key.indexOf('\n') >= 0
            || key.indexOf('\r') >= 0
            || value.indexOf('\n') >= 0
            || value.indexOf('\r') >= 0) {
          throw new IllegalArgumentException(
              "stdout key/value contains an unsupported delimiter");
        }
        writer.print(key);
        writer.print('=');
        writer.println(value);
      }
      writer.flush();
    }
  }
}
