package main

import (
	"bufio"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"

	eventsv1 "github.com/lifechronicle/lifechronicle/gen/go/lifechronicle/events/v1"
	"google.golang.org/protobuf/proto"
)

type properties map[string]string

type output struct {
	key   string
	value string
}

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "go golden runner:", err)
		os.Exit(1)
	}
}

func run() error {
	if len(os.Args) != 2 {
		return errors.New("usage: contract-runner-go <phase0-v1.properties>")
	}
	props, err := loadProperties(os.Args[1])
	if err != nil {
		return err
	}

	eventWire, err := props.hexBytes("protobuf_event_hex")
	if err != nil {
		return err
	}
	var event eventsv1.EventEnvelope
	if err := proto.Unmarshal(eventWire, &event); err != nil {
		return fmt.Errorf("decode EventEnvelope with generated binding: %w", err)
	}
	if event.GetObservedAt() == nil {
		return errors.New("decoded EventEnvelope lacks observed_at")
	}
	if event.GetOrigin() == nil {
		return errors.New("decoded EventEnvelope lacks origin")
	}
	if event.GetPayload() == nil {
		return errors.New("decoded EventEnvelope lacks payload Any")
	}

	var app eventsv1.AppForeground
	if err := proto.Unmarshal(event.GetPayload().GetValue(), &app); err != nil {
		return fmt.Errorf("decode Any.value as generated AppForeground: %w", err)
	}

	lcb1, err := buildLCB1(props)
	if err != nil {
		return err
	}
	lce1, err := buildLCE1(&event)
	if err != nil {
		return err
	}
	submitted := sha256.Sum256(lce1)
	trustedUser, err := props.require("series.user_id")
	if err != nil {
		return err
	}
	lcc1, err := buildIdentityFrame("LCC1", trustedUser, submitted[:])
	if err != nil {
		return err
	}
	canonical := sha256.Sum256(lcc1)

	seriesWire, err := props.hexBytes("series.submitted_wire_hex")
	if err != nil {
		return err
	}
	lcs1, err := buildBytesFrame("LCS1", seriesWire)
	if err != nil {
		return err
	}
	seriesSubmitted := sha256.Sum256(lcs1)
	lcr1, err := buildIdentityFrame("LCR1", trustedUser, seriesSubmitted[:])
	if err != nil {
		return err
	}
	seriesCanonical := sha256.Sum256(lcr1)

	kafkaKey, err := buildKafkaKey(props)
	if err != nil {
		return err
	}
	compressedPayload, err := props.hexBytes("series.compressed_payload_hex")
	if err != nil {
		return err
	}
	compressedDigest := sha256.Sum256(compressedPayload)
	chunkID, err := props.require("series.chunk_id")
	if err != nil {
		return err
	}
	compressedHex := hex.EncodeToString(compressedDigest[:])
	objectKey := fmt.Sprintf(
		"private/%s/series/%s/%s.zst",
		trustedUser,
		chunkID,
		compressedHex,
	)

	endedPresent := event.GetEndedAt() != nil
	results := []output{
		{"language", "go"},
		{"lcb1_hex", hex.EncodeToString(lcb1)},
		{"lce1_hex", hex.EncodeToString(lce1)},
		{"submitted_sha256_hex", hex.EncodeToString(submitted[:])},
		{"lcc1_hex", hex.EncodeToString(lcc1)},
		{"canonical_sha256_hex", hex.EncodeToString(canonical[:])},
		{"lcs1_hex", hex.EncodeToString(lcs1)},
		{"series_submitted_sha256_hex", hex.EncodeToString(seriesSubmitted[:])},
		{"lcr1_hex", hex.EncodeToString(lcr1)},
		{"series_canonical_sha256_hex", hex.EncodeToString(seriesCanonical[:])},
		{"kafka_key_hex", hex.EncodeToString(kafkaKey)},
		{"compressed_sha256_hex", compressedHex},
		{"compressed_size", strconv.Itoa(len(compressedPayload))},
		{"object_key", objectKey},
		{"object_version", compressedHex},
		{"decoded.event_id", event.GetEventId()},
		{"decoded.stream", event.GetStream()},
		{"decoded.event_type", event.GetEventType()},
		{"decoded.kind", strconv.FormatInt(int64(event.GetKind()), 10)},
		{"decoded.user_id", event.GetUserId()},
		{"decoded.device_id", event.GetDeviceId()},
		{"decoded.collector_instance_id", event.GetCollectorInstanceId()},
		{"decoded.source", event.GetSource()},
		{"decoded.schema_version", strconv.FormatUint(uint64(event.GetSchemaVersion()), 10)},
		{"decoded.sequence", strconv.FormatUint(event.GetSequence(), 10)},
		{"decoded.observed_at.seconds", strconv.FormatInt(event.GetObservedAt().GetSeconds(), 10)},
		{"decoded.observed_at.nanos", strconv.FormatInt(int64(event.GetObservedAt().GetNanos()), 10)},
		{"decoded.ended_at.present", strconv.FormatBool(endedPresent)},
		{"decoded.timezone", event.GetTimezone()},
		{"decoded.privacy_class", strconv.FormatInt(int64(event.GetPrivacyClass()), 10)},
		{"decoded.retention_class", strconv.FormatInt(int64(event.GetRetentionClass()), 10)},
		{"decoded.origin.provider", event.GetOrigin().GetProvider()},
		{"decoded.origin.provider_record_id", event.GetOrigin().GetProviderRecordId()},
		{"decoded.origin.import_id", event.GetOrigin().GetImportId()},
		{"decoded.origin.parent_event_id", event.GetOrigin().GetParentEventId()},
		{"decoded.origin.collection_method", event.GetOrigin().GetCollectionMethod()},
		{"decoded.payload_type_url", event.GetPayload().GetTypeUrl()},
		{"decoded.payload_value_hex", hex.EncodeToString(event.GetPayload().GetValue())},
		{"decoded.payload.application_id", app.GetApplicationId()},
		{"decoded.payload.application_name", app.GetApplicationName()},
		{"decoded.payload.executable_name", app.GetExecutableName()},
		{"decoded.payload.window_title_utf8_hex", hex.EncodeToString([]byte(app.GetWindowTitle()))},
		{"decoded.payload.process_id", strconv.FormatUint(app.GetProcessId(), 10)},
		{"decoded.payload.window_id", app.GetWindowId()},
		{"decoded.payload.fullscreen", strconv.FormatBool(app.GetFullscreen())},
	}

	writer := bufio.NewWriter(os.Stdout)
	for _, item := range results {
		if strings.ContainsAny(item.key, "=\r\n") || strings.ContainsAny(item.value, "\r\n") {
			return fmt.Errorf("output %q contains a forbidden delimiter", item.key)
		}
		if _, err := fmt.Fprintf(writer, "%s=%s\n", item.key, item.value); err != nil {
			return fmt.Errorf("write output: %w", err)
		}
	}
	if err := writer.Flush(); err != nil {
		return fmt.Errorf("flush output: %w", err)
	}
	return nil
}

func loadProperties(path string) (properties, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open properties: %w", err)
	}
	defer file.Close()

	result := properties{}
	scanner := bufio.NewScanner(file)
	lineNumber := 0
	for scanner.Scan() {
		lineNumber++
		line := strings.TrimSuffix(scanner.Text(), "\r")
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		key, value, found := strings.Cut(line, "=")
		if !found || key == "" {
			return nil, fmt.Errorf("properties line %d lacks key=value", lineNumber)
		}
		if strings.HasPrefix(key, "expected.") {
			continue
		}
		if _, exists := result[key]; exists {
			return nil, fmt.Errorf("duplicate property %q", key)
		}
		result[key] = value
	}
	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("read properties: %w", err)
	}
	return result, nil
}

func (props properties) require(key string) (string, error) {
	value, ok := props[key]
	if !ok {
		return "", fmt.Errorf("missing property %q", key)
	}
	return value, nil
}

func (props properties) hexBytes(key string) ([]byte, error) {
	value, err := props.require(key)
	if err != nil {
		return nil, err
	}
	decoded, err := hex.DecodeString(value)
	if err != nil {
		return nil, fmt.Errorf("property %q is not lowercase hex: %w", key, err)
	}
	if hex.EncodeToString(decoded) != value {
		return nil, fmt.Errorf("property %q must use canonical lowercase hex", key)
	}
	return decoded, nil
}

func (props properties) uint64(key string) (uint64, error) {
	value, err := props.require(key)
	if err != nil {
		return 0, err
	}
	parsed, err := strconv.ParseUint(value, 10, 64)
	if err != nil {
		return 0, fmt.Errorf("property %q is not uint64: %w", key, err)
	}
	return parsed, nil
}

func (props properties) uint32(key string) (uint32, error) {
	value, err := props.uint64(key)
	if err != nil {
		return 0, err
	}
	if value > uint64(^uint32(0)) {
		return 0, fmt.Errorf("property %q exceeds uint32", key)
	}
	return uint32(value), nil
}

func (props properties) int64(key string) (int64, error) {
	value, err := props.require(key)
	if err != nil {
		return 0, err
	}
	parsed, err := strconv.ParseInt(value, 10, 64)
	if err != nil {
		return 0, fmt.Errorf("property %q is not int64: %w", key, err)
	}
	return parsed, nil
}

func appendU8(destination []byte, value byte) []byte {
	return append(destination, value)
}

func appendU32(destination []byte, value uint32) []byte {
	return binary.BigEndian.AppendUint32(destination, value)
}

func appendU64(destination []byte, value uint64) []byte {
	return binary.BigEndian.AppendUint64(destination, value)
}

func appendI64(destination []byte, value int64) []byte {
	return binary.BigEndian.AppendUint64(destination, uint64(value))
}

func appendLengthPrefixed(destination []byte, value []byte) ([]byte, error) {
	if uint64(len(value)) > uint64(^uint32(0)) {
		return nil, errors.New("length-prefixed field exceeds uint32")
	}
	destination = appendU32(destination, uint32(len(value)))
	return append(destination, value...), nil
}

func appendString(destination []byte, value string) ([]byte, error) {
	return appendLengthPrefixed(destination, []byte(value))
}

func appendFixed32(destination []byte, value []byte) ([]byte, error) {
	if len(value) != sha256.Size {
		return nil, fmt.Errorf("fixed32 requires %d bytes, got %d", sha256.Size, len(value))
	}
	return append(destination, value...), nil
}

func buildLCB1(props properties) ([]byte, error) {
	result := []byte("LCB1")
	var err error
	for _, key := range []string{
		"batch.batch_id",
		"batch.device_id",
		"batch.collector_instance_id",
	} {
		value, readErr := props.require(key)
		if readErr != nil {
			return nil, readErr
		}
		result, err = appendString(result, value)
		if err != nil {
			return nil, err
		}
	}
	sequenceStart, err := props.uint64("batch.sequence_start")
	if err != nil {
		return nil, err
	}
	sequenceEnd, err := props.uint64("batch.sequence_end")
	if err != nil {
		return nil, err
	}
	createdSeconds, err := props.int64("batch.created_at.seconds")
	if err != nil {
		return nil, err
	}
	createdNanos, err := props.uint32("batch.created_at.nanos")
	if err != nil {
		return nil, err
	}
	nonce, err := props.hexBytes("batch.nonce_hex")
	if err != nil {
		return nil, err
	}
	compression, err := props.uint32("batch.compression")
	if err != nil {
		return nil, err
	}
	source, err := props.require("batch.source")
	if err != nil {
		return nil, err
	}
	payloadDigest, err := props.hexBytes("batch.payload_sha256_hex")
	if err != nil {
		return nil, err
	}

	result = appendU64(result, sequenceStart)
	result = appendU64(result, sequenceEnd)
	result = appendI64(result, createdSeconds)
	result = appendU32(result, createdNanos)
	result, err = appendLengthPrefixed(result, nonce)
	if err != nil {
		return nil, err
	}
	result = appendU32(result, compression)
	result, err = appendString(result, source)
	if err != nil {
		return nil, err
	}
	return appendFixed32(result, payloadDigest)
}

func buildLCE1(event *eventsv1.EventEnvelope) ([]byte, error) {
	if event.GetObservedAt() == nil || event.GetOrigin() == nil || event.GetPayload() == nil {
		return nil, errors.New("LCE1 requires observed_at, origin and payload")
	}
	result := []byte("LCE1")
	var err error
	for _, value := range []string{
		event.GetEventId(),
		event.GetStream(),
		event.GetEventType(),
	} {
		result, err = appendString(result, value)
		if err != nil {
			return nil, err
		}
	}
	result = appendU32(result, uint32(event.GetKind()))
	for _, value := range []string{
		event.GetDeviceId(),
		event.GetCollectorInstanceId(),
		event.GetSource(),
	} {
		result, err = appendString(result, value)
		if err != nil {
			return nil, err
		}
	}
	result = appendU32(result, event.GetSchemaVersion())
	result = appendU64(result, event.GetSequence())
	result = appendI64(result, event.GetObservedAt().GetSeconds())
	result = appendU32(result, uint32(event.GetObservedAt().GetNanos()))
	if event.GetEndedAt() == nil {
		result = appendU8(result, 0)
	} else {
		result = appendU8(result, 1)
		result = appendI64(result, event.GetEndedAt().GetSeconds())
		result = appendU32(result, uint32(event.GetEndedAt().GetNanos()))
	}
	result, err = appendString(result, event.GetTimezone())
	if err != nil {
		return nil, err
	}
	result = appendU32(result, uint32(event.GetPrivacyClass()))
	result = appendU32(result, uint32(event.GetRetentionClass()))
	origin := event.GetOrigin()
	for _, value := range []string{
		origin.GetProvider(),
		origin.GetProviderRecordId(),
		origin.GetImportId(),
		origin.GetParentEventId(),
		origin.GetCollectionMethod(),
		event.GetPayload().GetTypeUrl(),
	} {
		result, err = appendString(result, value)
		if err != nil {
			return nil, err
		}
	}
	return appendLengthPrefixed(result, event.GetPayload().GetValue())
}

func buildBytesFrame(magic string, value []byte) ([]byte, error) {
	return appendLengthPrefixed([]byte(magic), value)
}

func buildIdentityFrame(magic, userID string, digest []byte) ([]byte, error) {
	result, err := appendString([]byte(magic), userID)
	if err != nil {
		return nil, err
	}
	return appendFixed32(result, digest)
}

func buildKafkaKey(props properties) ([]byte, error) {
	count, err := props.uint32("kafka_key_fields.count")
	if err != nil {
		return nil, err
	}
	result := []byte{}
	for index := uint32(0); index < count; index++ {
		value, err := props.require(fmt.Sprintf("kafka_key_fields.%d", index))
		if err != nil {
			return nil, err
		}
		result, err = appendString(result, value)
		if err != nil {
			return nil, err
		}
	}
	return result, nil
}
