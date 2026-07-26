import com.google.protobuf.Any
import io.lifechronicle.events.v1.AppForeground
import io.lifechronicle.events.v1.EventEnvelope
import io.lifechronicle.events.v1.copy
import java.io.ByteArrayOutputStream
import java.io.OutputStreamWriter
import java.nio.charset.StandardCharsets
import java.nio.file.Files
import java.nio.file.Path
import java.security.MessageDigest
import java.util.Locale
import java.util.Properties

/**
 * Independent Kotlin implementation of the Phase 0 golden-vector framing.
 *
 * The generated Kotlin API augments the generated Java message classes, so the
 * no-op copy DSL calls below intentionally force both generated source sets to
 * compile and link while parseFrom performs the Protobuf wire decode.
 */
private const val VECTOR_NAME = "phase0-v1.properties"

fun main(args: Array<String>) {
    val vectorPath = locateVector(args)
    val input = loadProperties(vectorPath)
    emit(calculate(input))
}

private fun calculate(input: Properties): LinkedHashMap<String, String> {
    val compressedItems = decodeHex(required(input, "batch.compressed_items_hex"))
    val batchPayloadHash = sha256(compressedItems)
    requireEqualHex(
        "batch.payload_sha256_hex",
        required(input, "batch.payload_sha256_hex"),
        batchPayloadHash,
    )

    val lcb1 = buildLcb1(input, batchPayloadHash)
    val lce1 = buildLce1(input)
    val submittedHash = sha256(lce1)

    val canonicalUserId = required(input, "kafka_key_fields.0")
    val lcc1 = buildIdentityFrame("LCC1", canonicalUserId, submittedHash)
    val canonicalHash = sha256(lcc1)

    val rawSeriesPayload = decodeHex(required(input, "series.raw_payload_hex"))
    val seriesChecksum = sha256(rawSeriesPayload)
    requireEqualHex(
        "series.checksum_hex",
        required(input, "series.checksum_hex"),
        seriesChecksum,
    )

    val submittedSeriesWire =
        decodeHex(required(input, "series.submitted_wire_hex"))
    val lcs1 = buildFramedPayload("LCS1", submittedSeriesWire)
    val seriesSubmittedHash = sha256(lcs1)
    val lcr1 =
        buildIdentityFrame(
            "LCR1",
            required(input, "series.user_id"),
            seriesSubmittedHash,
        )
    val seriesCanonicalHash = sha256(lcr1)

    val compressedSeries =
        decodeHex(required(input, "series.compressed_payload_hex"))
    val compressedHash = sha256(compressedSeries)
    val objectVersion = encodeHex(compressedHash)
    val objectKey =
        "private/${required(input, "series.user_id")}/series/" +
            "${required(input, "series.chunk_id")}/$objectVersion.zst"
    val kafkaKey = buildKafkaKey(input)

    val output = linkedMapOf<String, String>()
    output["language"] = "kotlin"
    output["lcb1_hex"] = encodeHex(lcb1)
    output["lce1_hex"] = encodeHex(lce1)
    output["lcc1_hex"] = encodeHex(lcc1)
    output["lcs1_hex"] = encodeHex(lcs1)
    output["lcr1_hex"] = encodeHex(lcr1)
    output["submitted_sha256_hex"] = encodeHex(submittedHash)
    output["canonical_sha256_hex"] = encodeHex(canonicalHash)
    output["series_submitted_sha256_hex"] = encodeHex(seriesSubmittedHash)
    output["series_canonical_sha256_hex"] = encodeHex(seriesCanonicalHash)
    output["compressed_sha256_hex"] = objectVersion
    output["kafka_key_hex"] = encodeHex(kafkaKey)
    output["compressed_size"] = compressedSeries.size.toString()
    output["object_version"] = objectVersion
    output["object_key"] = objectKey

    decodeEvent(input, output)
    return output
}

private fun buildLcb1(input: Properties, payloadHash: ByteArray): ByteArray =
    ByteArrayOutputStream().run {
        appendAscii("LCB1")
        appendFramedString(required(input, "batch.batch_id"))
        appendFramedString(required(input, "batch.device_id"))
        appendFramedString(required(input, "batch.collector_instance_id"))
        appendU64(parseU64(input, "batch.sequence_start"))
        appendU64(parseU64(input, "batch.sequence_end"))
        appendI64(parseI64(input, "batch.created_at.seconds"))
        appendU32(parseU32(input, "batch.created_at.nanos"))
        appendFramedBytes(decodeHex(required(input, "batch.nonce_hex")))
        appendU32(parseU32(input, "batch.compression"))
        appendFramedString(required(input, "batch.source"))
        write(payloadHash)
        toByteArray()
    }

private fun buildLce1(input: Properties): ByteArray =
    ByteArrayOutputStream().run {
        appendAscii("LCE1")
        appendFramedString(required(input, "event.event_id"))
        appendFramedString(required(input, "event.stream"))
        appendFramedString(required(input, "event.event_type"))
        appendU32(parseU32(input, "event.kind"))
        appendFramedString(required(input, "event.device_id"))
        appendFramedString(required(input, "event.collector_instance_id"))
        appendFramedString(required(input, "event.source"))
        appendU32(parseU32(input, "event.schema_version"))
        appendU64(parseU64(input, "event.sequence"))
        appendI64(parseI64(input, "event.observed_at.seconds"))
        appendU32(parseU32(input, "event.observed_at.nanos"))

        val endedAtPresent = parseBoolean(input, "event.ended_at.present")
        write(if (endedAtPresent) 1 else 0)
        if (endedAtPresent) {
            appendI64(parseI64(input, "event.ended_at.seconds"))
            appendU32(parseU32(input, "event.ended_at.nanos"))
        }

        appendFramedString(required(input, "event.timezone"))
        appendU32(parseU32(input, "event.privacy_class"))
        appendU32(parseU32(input, "event.retention_class"))
        appendFramedString(required(input, "event.origin.provider"))
        appendFramedString(required(input, "event.origin.provider_record_id"))
        appendFramedString(required(input, "event.origin.import_id"))
        appendFramedString(required(input, "event.origin.parent_event_id"))
        appendFramedString(required(input, "event.origin.collection_method"))
        appendFramedString(required(input, "event.payload_type_url"))
        appendFramedBytes(decodeHex(required(input, "event.payload_value_hex")))
        toByteArray()
    }

private fun buildIdentityFrame(
    magic: String,
    userId: String,
    submittedHash: ByteArray,
): ByteArray =
    ByteArrayOutputStream().run {
        appendAscii(magic)
        appendFramedString(userId)
        write(submittedHash)
        toByteArray()
    }

private fun buildFramedPayload(magic: String, payload: ByteArray): ByteArray =
    ByteArrayOutputStream().run {
        appendAscii(magic)
        appendFramedBytes(payload)
        toByteArray()
    }

private fun buildKafkaKey(input: Properties): ByteArray {
    val count = parseNonNegativeInt(input, "kafka_key_fields.count")
    require(count > 0) { "kafka_key_fields.count must be greater than zero" }
    return ByteArrayOutputStream().run {
        repeat(count) { index ->
            appendFramedString(required(input, "kafka_key_fields.$index"))
        }
        toByteArray()
    }
}

private fun decodeEvent(
    input: Properties,
    output: LinkedHashMap<String, String>,
) {
    val wire = decodeHex(required(input, "protobuf_event_hex"))
    val event =
        EventEnvelope.parseFrom(wire).copy {
            // Generated Kotlin copy DSL intentionally participates in decode.
        }
    require(event.hasObservedAt()) { "protobuf_event_hex has no observed_at" }
    require(event.hasOrigin()) { "protobuf_event_hex has no origin" }
    require(event.hasPayload()) { "protobuf_event_hex has no payload" }

    val anyPayload: Any = event.payload
    require(anyPayload.`is`(AppForeground::class.java)) {
        "protobuf_event_hex payload is not AppForeground: ${anyPayload.typeUrl}"
    }
    val payload =
        anyPayload.unpack(AppForeground::class.java).copy {
            // Generated Kotlin copy DSL intentionally participates in decode.
        }
    val origin = event.origin

    output["decoded.event_id"] = event.eventId
    output["decoded.stream"] = event.stream
    output["decoded.event_type"] = event.eventType
    output["decoded.kind"] = event.kindValue.toString()
    output["decoded.user_id"] = event.userId
    output["decoded.device_id"] = event.deviceId
    output["decoded.collector_instance_id"] = event.collectorInstanceId
    output["decoded.source"] = event.source
    output["decoded.schema_version"] =
        Integer.toUnsignedString(event.schemaVersion)
    output["decoded.sequence"] = java.lang.Long.toUnsignedString(event.sequence)
    output["decoded.observed_at.seconds"] = event.observedAt.seconds.toString()
    output["decoded.observed_at.nanos"] = event.observedAt.nanos.toString()
    output["decoded.ended_at.present"] = event.hasEndedAt().toString()
    output["decoded.timezone"] = event.timezone
    output["decoded.privacy_class"] = event.privacyClassValue.toString()
    output["decoded.retention_class"] = event.retentionClassValue.toString()
    output["decoded.origin.provider"] = origin.provider
    output["decoded.origin.provider_record_id"] = origin.providerRecordId
    output["decoded.origin.import_id"] = origin.importId
    output["decoded.origin.parent_event_id"] = origin.parentEventId
    output["decoded.origin.collection_method"] = origin.collectionMethod
    output["decoded.payload_type_url"] = anyPayload.typeUrl
    output["decoded.payload_value_hex"] =
        encodeHex(anyPayload.value.toByteArray())
    output["decoded.payload.application_id"] = payload.applicationId
    output["decoded.payload.application_name"] = payload.applicationName
    output["decoded.payload.executable_name"] = payload.executableName
    output["decoded.payload.window_title_utf8_hex"] =
        encodeHex(payload.windowTitle.toByteArray(StandardCharsets.UTF_8))
    output["decoded.payload.process_id"] =
        java.lang.Long.toUnsignedString(payload.processId)
    output["decoded.payload.window_id"] = payload.windowId
    output["decoded.payload.fullscreen"] = payload.fullscreen.toString()
}

private fun loadProperties(path: Path): Properties =
    Properties().also { properties ->
        Files.newBufferedReader(path, StandardCharsets.UTF_8).use {
            properties.load(it)
        }
    }

private fun locateVector(args: Array<String>): Path {
    require(args.size <= 1) { "usage: Phase0RunnerKt [path/to/$VECTOR_NAME]" }
    if (args.size == 1) {
        val explicit = Path.of(args[0]).toAbsolutePath().normalize()
        require(Files.isRegularFile(explicit)) {
            "vector file does not exist: $explicit"
        }
        return explicit
    }

    var cursor: Path? = Path.of("").toAbsolutePath().normalize()
    while (cursor != null) {
        val repositoryCandidate =
            cursor.resolve("contract-tests").resolve("vectors").resolve(VECTOR_NAME)
        if (Files.isRegularFile(repositoryCandidate)) {
            return repositoryCandidate
        }
        val contractTestsCandidate = cursor.resolve("vectors").resolve(VECTOR_NAME)
        if (Files.isRegularFile(contractTestsCandidate)) {
            return contractTestsCandidate
        }
        cursor = cursor.parent
    }
    throw IllegalArgumentException(
        "cannot locate contract-tests/vectors/$VECTOR_NAME",
    )
}

private fun required(input: Properties, key: String): String {
    require(!key.startsWith("expected.")) {
        "runner code must not read expected.* properties: $key"
    }
    return input.getProperty(key)
        ?: throw IllegalArgumentException("missing vector property: $key")
}

private fun parseNonNegativeInt(input: Properties, key: String): Int {
    val raw = required(input, key)
    val value =
        raw.toIntOrNull()
            ?: throw IllegalArgumentException("$key is not an integer: $raw")
    require(value >= 0) { "$key must be non-negative: $raw" }
    return value
}

private fun parseU32(input: Properties, key: String): Long {
    val raw = required(input, key)
    val value =
        raw.toLongOrNull()
            ?: throw IllegalArgumentException("$key is not uint32: $raw")
    require(value in 0..0xffff_ffffL) { "$key is outside uint32: $raw" }
    return value
}

private fun parseU64(input: Properties, key: String): Long {
    val raw = required(input, key)
    return try {
        java.lang.Long.parseUnsignedLong(raw)
    } catch (exception: NumberFormatException) {
        throw IllegalArgumentException("$key is not uint64: $raw", exception)
    }
}

private fun parseI64(input: Properties, key: String): Long {
    val raw = required(input, key)
    return raw.toLongOrNull()
        ?: throw IllegalArgumentException("$key is not int64: $raw")
}

private fun parseBoolean(input: Properties, key: String): Boolean =
    when (val raw = required(input, key)) {
        "true" -> true
        "false" -> false
        else -> throw IllegalArgumentException(
            "$key is not true or false: $raw",
        )
    }

private fun requireEqualHex(
    key: String,
    declaredHex: String,
    calculated: ByteArray,
) {
    require(declaredHex.lowercase(Locale.ROOT) == encodeHex(calculated)) {
        "$key does not match independently calculated SHA-256"
    }
}

private fun sha256(value: ByteArray): ByteArray =
    MessageDigest.getInstance("SHA-256").digest(value)

private fun decodeHex(value: String): ByteArray {
    require(value.length % 2 == 0) { "hex value has odd length" }
    return ByteArray(value.length / 2) { index ->
        val high = Character.digit(value[index * 2], 16)
        val low = Character.digit(value[index * 2 + 1], 16)
        require(high >= 0 && low >= 0) {
            "hex value contains a non-hexadecimal character"
        }
        ((high shl 4) or low).toByte()
    }
}

private fun encodeHex(value: ByteArray): String {
    val digits = "0123456789abcdef"
    return buildString(value.size * 2) {
        value.forEach { byte ->
            val unsigned = byte.toInt() and 0xff
            append(digits[unsigned ushr 4])
            append(digits[unsigned and 0x0f])
        }
    }
}

private fun ByteArrayOutputStream.appendAscii(value: String) {
    write(value.toByteArray(StandardCharsets.US_ASCII))
}

private fun ByteArrayOutputStream.appendFramedString(value: String) {
    appendFramedBytes(value.toByteArray(StandardCharsets.UTF_8))
}

private fun ByteArrayOutputStream.appendFramedBytes(value: ByteArray) {
    appendU32(value.size.toLong())
    write(value)
}

private fun ByteArrayOutputStream.appendU32(value: Long) {
    require(value in 0..0xffff_ffffL) { "value is outside uint32: $value" }
    write(((value ushr 24) and 0xff).toInt())
    write(((value ushr 16) and 0xff).toInt())
    write(((value ushr 8) and 0xff).toInt())
    write((value and 0xff).toInt())
}

private fun ByteArrayOutputStream.appendU64(value: Long) {
    appendI64(value)
}

private fun ByteArrayOutputStream.appendI64(value: Long) {
    for (shift in 56 downTo 0 step 8) {
        write(((value ushr shift) and 0xff).toInt())
    }
}

private fun emit(output: LinkedHashMap<String, String>) {
    val writer = OutputStreamWriter(System.out, StandardCharsets.UTF_8)
    output.forEach { (key, value) ->
        require('=' !in key && '\n' !in key && '\r' !in key) {
            "stdout key contains an unsupported delimiter"
        }
        require('\n' !in value && '\r' !in value) {
            "stdout value contains an unsupported delimiter"
        }
        writer.write(key)
        writer.write("=")
        writer.write(value)
        writer.write("\n")
    }
    writer.flush()
}
