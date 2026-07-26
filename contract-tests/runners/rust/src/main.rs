use lifechronicle_events_v1::{AppForeground, EventEnvelope};
use protobuf::Parse;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

type RunnerResult<T> = Result<T, String>;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rust golden runner: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> RunnerResult<()> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        return Err("usage: contract-runner-rust <phase0-v1.properties>".to_owned());
    }
    let properties = Properties::load(Path::new(&arguments[1]))?;

    let event_wire = properties.hex_bytes("protobuf_event_hex")?;
    let event = EventEnvelope::parse(&event_wire)
        .map_err(|error| format!("decode EventEnvelope with generated binding: {error}"))?;
    if !event.has_observed_at() {
        return Err("decoded EventEnvelope lacks observed_at".to_owned());
    }
    if !event.has_origin() {
        return Err("decoded EventEnvelope lacks origin".to_owned());
    }
    if !event.has_payload() {
        return Err("decoded EventEnvelope lacks payload Any".to_owned());
    }
    let payload_value = event.payload().value();
    let app = AppForeground::parse(payload_value)
        .map_err(|error| format!("decode Any.value as generated AppForeground: {error}"))?;

    let lcb1 = build_lcb1(&properties)?;
    let lce1 = build_lce1(&event)?;
    let submitted = sha256(&lce1);
    let trusted_user = properties.require("series.user_id")?;
    let lcc1 = build_identity_frame(b"LCC1", trusted_user, &submitted)?;
    let canonical = sha256(&lcc1);

    let series_wire = properties.hex_bytes("series.submitted_wire_hex")?;
    let lcs1 = build_bytes_frame(b"LCS1", &series_wire)?;
    let series_submitted = sha256(&lcs1);
    let lcr1 = build_identity_frame(b"LCR1", trusted_user, &series_submitted)?;
    let series_canonical = sha256(&lcr1);

    let kafka_key = build_kafka_key(&properties)?;
    let compressed_payload = properties.hex_bytes("series.compressed_payload_hex")?;
    let compressed_digest = sha256(&compressed_payload);
    let compressed_hex = encode_hex(&compressed_digest);
    let chunk_id = properties.require("series.chunk_id")?;
    let object_key = format!(
        "private/{trusted_user}/series/{chunk_id}/{compressed_hex}.zst"
    );

    let observed_at = event.observed_at();
    let origin = event.origin();
    let payload = event.payload();
    let results = vec![
        output("language", "rust"),
        output("lcb1_hex", encode_hex(&lcb1)),
        output("lce1_hex", encode_hex(&lce1)),
        output("submitted_sha256_hex", encode_hex(&submitted)),
        output("lcc1_hex", encode_hex(&lcc1)),
        output("canonical_sha256_hex", encode_hex(&canonical)),
        output("lcs1_hex", encode_hex(&lcs1)),
        output(
            "series_submitted_sha256_hex",
            encode_hex(&series_submitted),
        ),
        output("lcr1_hex", encode_hex(&lcr1)),
        output(
            "series_canonical_sha256_hex",
            encode_hex(&series_canonical),
        ),
        output("kafka_key_hex", encode_hex(&kafka_key)),
        output("compressed_sha256_hex", compressed_hex.clone()),
        output("compressed_size", compressed_payload.len().to_string()),
        output("object_key", object_key),
        output("object_version", compressed_hex),
        output("decoded.event_id", checked_string(event.event_id())?),
        output("decoded.stream", checked_string(event.stream())?),
        output("decoded.event_type", checked_string(event.event_type())?),
        output("decoded.kind", i32::from(event.kind()).to_string()),
        output("decoded.user_id", checked_string(event.user_id())?),
        output("decoded.device_id", checked_string(event.device_id())?),
        output(
            "decoded.collector_instance_id",
            checked_string(event.collector_instance_id())?,
        ),
        output("decoded.source", checked_string(event.source())?),
        output("decoded.schema_version", event.schema_version().to_string()),
        output("decoded.sequence", event.sequence().to_string()),
        output(
            "decoded.observed_at.seconds",
            observed_at.seconds().to_string(),
        ),
        output("decoded.observed_at.nanos", observed_at.nanos().to_string()),
        output(
            "decoded.ended_at.present",
            event.has_ended_at().to_string(),
        ),
        output("decoded.timezone", checked_string(event.timezone())?),
        output(
            "decoded.privacy_class",
            i32::from(event.privacy_class()).to_string(),
        ),
        output(
            "decoded.retention_class",
            i32::from(event.retention_class()).to_string(),
        ),
        output(
            "decoded.origin.provider",
            checked_string(origin.provider())?,
        ),
        output(
            "decoded.origin.provider_record_id",
            checked_string(origin.provider_record_id())?,
        ),
        output(
            "decoded.origin.import_id",
            checked_string(origin.import_id())?,
        ),
        output(
            "decoded.origin.parent_event_id",
            checked_string(origin.parent_event_id())?,
        ),
        output(
            "decoded.origin.collection_method",
            checked_string(origin.collection_method())?,
        ),
        output(
            "decoded.payload_type_url",
            checked_string(payload.type_url())?,
        ),
        output("decoded.payload_value_hex", encode_hex(payload.value())),
        output(
            "decoded.payload.application_id",
            checked_string(app.application_id())?,
        ),
        output(
            "decoded.payload.application_name",
            checked_string(app.application_name())?,
        ),
        output(
            "decoded.payload.executable_name",
            checked_string(app.executable_name())?,
        ),
        output(
            "decoded.payload.window_title_utf8_hex",
            encode_hex(app.window_title().as_bytes()),
        ),
        output(
            "decoded.payload.process_id",
            app.process_id().to_string(),
        ),
        output(
            "decoded.payload.window_id",
            checked_string(app.window_id())?,
        ),
        output(
            "decoded.payload.fullscreen",
            app.fullscreen().to_string(),
        ),
    ];

    for (key, value) in results {
        if key.contains(['=', '\r', '\n']) || value.contains(['\r', '\n']) {
            return Err(format!("output {key:?} contains a forbidden delimiter"));
        }
        println!("{key}={value}");
    }
    Ok(())
}

fn output(key: &'static str, value: impl Into<String>) -> (&'static str, String) {
    (key, value.into())
}

fn checked_string(value: &protobuf::ProtoStr) -> RunnerResult<String> {
    value
        .to_str()
        .map(str::to_owned)
        .map_err(|error| format!("decoded protobuf string is not UTF-8: {error}"))
}

struct Properties {
    values: BTreeMap<String, String>,
}

impl Properties {
    fn load(path: &Path) -> RunnerResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|error| format!("read properties {}: {error}", path.display()))?;
        let mut values = BTreeMap::new();
        for (index, raw_line) in content.lines().enumerate() {
            let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("properties line {} lacks key=value", index + 1))?;
            if key.is_empty() {
                return Err(format!("properties line {} has an empty key", index + 1));
            }
            if key.starts_with("expected.") {
                continue;
            }
            if values.insert(key.to_owned(), value.to_owned()).is_some() {
                return Err(format!("duplicate property {key:?}"));
            }
        }
        Ok(Self { values })
    }

    fn require(&self, key: &str) -> RunnerResult<&str> {
        self.values
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| format!("missing property {key:?}"))
    }

    fn hex_bytes(&self, key: &str) -> RunnerResult<Vec<u8>> {
        let value = self.require(key)?;
        let decoded = decode_hex(value)
            .map_err(|error| format!("property {key:?} is not lowercase hex: {error}"))?;
        if encode_hex(&decoded) != value {
            return Err(format!("property {key:?} must use canonical lowercase hex"));
        }
        Ok(decoded)
    }

    fn u32(&self, key: &str) -> RunnerResult<u32> {
        self.require(key)?
            .parse::<u32>()
            .map_err(|error| format!("property {key:?} is not uint32: {error}"))
    }

    fn u64(&self, key: &str) -> RunnerResult<u64> {
        self.require(key)?
            .parse::<u64>()
            .map_err(|error| format!("property {key:?} is not uint64: {error}"))
    }

    fn i64(&self, key: &str) -> RunnerResult<i64> {
        self.require(key)?
            .parse::<i64>()
            .map_err(|error| format!("property {key:?} is not int64: {error}"))
    }
}

fn append_u8(destination: &mut Vec<u8>, value: u8) {
    destination.push(value);
}

fn append_u32(destination: &mut Vec<u8>, value: u32) {
    destination.extend_from_slice(&value.to_be_bytes());
}

fn append_u64(destination: &mut Vec<u8>, value: u64) {
    destination.extend_from_slice(&value.to_be_bytes());
}

fn append_i64(destination: &mut Vec<u8>, value: i64) {
    destination.extend_from_slice(&value.to_be_bytes());
}

fn append_length_prefixed(destination: &mut Vec<u8>, value: &[u8]) -> RunnerResult<()> {
    let length = u32::try_from(value.len())
        .map_err(|_| "length-prefixed field exceeds uint32".to_owned())?;
    append_u32(destination, length);
    destination.extend_from_slice(value);
    Ok(())
}

fn append_string(destination: &mut Vec<u8>, value: &str) -> RunnerResult<()> {
    append_length_prefixed(destination, value.as_bytes())
}

fn append_proto_string(
    destination: &mut Vec<u8>,
    value: &protobuf::ProtoStr,
) -> RunnerResult<()> {
    value
        .to_str()
        .map_err(|error| format!("protobuf string is not UTF-8: {error}"))?;
    append_length_prefixed(destination, value.as_bytes())
}

fn append_fixed32(destination: &mut Vec<u8>, value: &[u8]) -> RunnerResult<()> {
    if value.len() != 32 {
        return Err(format!("fixed32 requires 32 bytes, got {}", value.len()));
    }
    destination.extend_from_slice(value);
    Ok(())
}

fn build_lcb1(properties: &Properties) -> RunnerResult<Vec<u8>> {
    let mut result = b"LCB1".to_vec();
    for key in [
        "batch.batch_id",
        "batch.device_id",
        "batch.collector_instance_id",
    ] {
        append_string(&mut result, properties.require(key)?)?;
    }
    append_u64(
        &mut result,
        properties.u64("batch.sequence_start")?,
    );
    append_u64(&mut result, properties.u64("batch.sequence_end")?);
    append_i64(
        &mut result,
        properties.i64("batch.created_at.seconds")?,
    );
    append_u32(
        &mut result,
        properties.u32("batch.created_at.nanos")?,
    );
    append_length_prefixed(&mut result, &properties.hex_bytes("batch.nonce_hex")?)?;
    append_u32(&mut result, properties.u32("batch.compression")?);
    append_string(&mut result, properties.require("batch.source")?)?;
    append_fixed32(
        &mut result,
        &properties.hex_bytes("batch.payload_sha256_hex")?,
    )?;
    Ok(result)
}

fn build_lce1(event: &EventEnvelope) -> RunnerResult<Vec<u8>> {
    if !event.has_observed_at() || !event.has_origin() || !event.has_payload() {
        return Err("LCE1 requires observed_at, origin and payload".to_owned());
    }
    let mut result = b"LCE1".to_vec();
    append_proto_string(&mut result, event.event_id())?;
    append_proto_string(&mut result, event.stream())?;
    append_proto_string(&mut result, event.event_type())?;
    append_u32(&mut result, i32::from(event.kind()) as u32);
    append_proto_string(&mut result, event.device_id())?;
    append_proto_string(&mut result, event.collector_instance_id())?;
    append_proto_string(&mut result, event.source())?;
    append_u32(&mut result, event.schema_version());
    append_u64(&mut result, event.sequence());
    append_i64(&mut result, event.observed_at().seconds());
    append_u32(&mut result, event.observed_at().nanos() as u32);
    if event.has_ended_at() {
        append_u8(&mut result, 1);
        append_i64(&mut result, event.ended_at().seconds());
        append_u32(&mut result, event.ended_at().nanos() as u32);
    } else {
        append_u8(&mut result, 0);
    }
    append_proto_string(&mut result, event.timezone())?;
    append_u32(&mut result, i32::from(event.privacy_class()) as u32);
    append_u32(&mut result, i32::from(event.retention_class()) as u32);
    let origin = event.origin();
    append_proto_string(&mut result, origin.provider())?;
    append_proto_string(&mut result, origin.provider_record_id())?;
    append_proto_string(&mut result, origin.import_id())?;
    append_proto_string(&mut result, origin.parent_event_id())?;
    append_proto_string(&mut result, origin.collection_method())?;
    append_proto_string(&mut result, event.payload().type_url())?;
    append_length_prefixed(&mut result, event.payload().value())?;
    Ok(result)
}

fn build_bytes_frame(magic: &[u8; 4], value: &[u8]) -> RunnerResult<Vec<u8>> {
    let mut result = magic.to_vec();
    append_length_prefixed(&mut result, value)?;
    Ok(result)
}

fn build_identity_frame(
    magic: &[u8; 4],
    user_id: &str,
    digest: &[u8],
) -> RunnerResult<Vec<u8>> {
    let mut result = magic.to_vec();
    append_string(&mut result, user_id)?;
    append_fixed32(&mut result, digest)?;
    Ok(result)
}

fn build_kafka_key(properties: &Properties) -> RunnerResult<Vec<u8>> {
    let count = properties.u32("kafka_key_fields.count")?;
    let mut result = Vec::new();
    for index in 0..count {
        append_string(
            &mut result,
            properties.require(&format!("kafka_key_fields.{index}"))?,
        )?;
    }
    Ok(result)
}

fn decode_hex(value: &str) -> RunnerResult<Vec<u8>> {
    if value.len() % 2 != 0 {
        return Err("hex length must be even".to_owned());
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn hex_nibble(value: u8) -> RunnerResult<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(format!("invalid lowercase hex byte 0x{value:02x}")),
    }
}

fn encode_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(value.len() * 2);
    for byte in value {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const INITIAL: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    const ROUND: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let bit_length = (input.len() as u64).wrapping_mul(8);
    let mut padded = Vec::with_capacity((input.len() + 72) & !63);
    padded.extend_from_slice(input);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_be_bytes());

    let mut state = INITIAL;
    for block in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, word) in block.chunks_exact(4).enumerate() {
            words[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        for index in 0..64 {
            let sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(sigma1)
                .wrapping_add(choice)
                .wrapping_add(ROUND[index])
                .wrapping_add(words[index]);
            let sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sigma0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::{encode_hex, sha256};

    #[test]
    fn sha256_matches_fips_vectors() {
        assert_eq!(
            encode_hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            encode_hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
