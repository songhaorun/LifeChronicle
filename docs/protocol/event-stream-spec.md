# LifeChronicle 事件与 Stream 规范

**文档版本：** v1.0
**状态：** 阶段 0 已接受协议基线
**依据：** [项目工程契约](../contract/project-contract.md)、
[总体架构](../architecture/overall-architecture.md)、
[项目计划书](../planning/project-plan.md)

## 1. 目的与范围

本文档定义 LifeChronicle 普通事件、高频序列、上传批次、接收确认、
Stream Registry 和 Kafka Topic 的稳定契约。凡使用“必须”“不得”“仅”
的条款均为规范性要求。

本文档不定义设备注册 UI、OIDC 流程、具体数据库 DDL、公开投影字段或插件
WIT；这些能力只能引用本文档中的事件和 Stream 标识，不得另建平行数据模型。

## 2. 核心不变量

1. 原始事实只追加，不原地更新；修正、替代和删除均以新记录表达。
2. 普通事件与高频序列使用不同载荷和数据通路。
3. 业务时间使用 `observed_at`，接收时间不得替代业务时间。
4. 重试必须复用原 `event_id`、`chunk_id` 和 `batch_id`。
5. 设备清理 Outbox 的可靠终态仅包括 `ACCEPTED_TO_LOG`，以及服务端已证明
   同 ID、同内容满足相同持久条件的 `DUPLICATE`。
6. 普通 Event 的 `ACCEPTED_TO_LOG` 只能在 Kafka 已持久确认后返回；Series
   还必须等待对象块持久写入且其 Kafka metadata 已确认。
7. 新 Stream 默认 `PRIVATE`，且不会自动进入公开 API。
8. Stream、Schema、Topic 和处理器版本均必须显式登记。
9. 同一输入快照、同一处理器版本和同一规则版本必须产生确定性相同结果。

## 3. 标识、枚举和时间

### 3.1 标识格式

| 标识 | 格式 | 生成方 | 稳定性要求 |
| --- | --- | --- | --- |
| `event_id` | 小写规范形式 UUIDv7 | Agent | 写入 WAL 后永不改变 |
| `chunk_id` | 小写规范形式 UUIDv7 | Agent | 封块后永不改变 |
| `batch_id` | 小写规范形式 UUIDv7 | Agent | 同一批重试不改变 |
| `device_id` | UUID | Identity Service | 设备撤销后不复用 |
| `collector_instance_id` | UUID | Agent | 安装、重置或身份重建时重新生成 |
| `user_id` | UUID | 服务端 | 设备提交值不得被信任 |
| `processor_run_id` | UUIDv7 | 处理器 | 每次实时部署或回放运行唯一 |

所有 UUID 字符串必须使用 36 字符小写连字符形式。解析端可以接受大小写，
但规范化输出必须为小写。不得使用设备名称、邮箱或序列号作为任何主键。

### 3.2 基础枚举

```protobuf
enum RecordKind {
  RECORD_KIND_UNSPECIFIED = 0;
  STATE = 1;
  INTERVAL = 2;
  SAMPLE = 3;
  DELTA = 4;
  SERIES = 5;
  ANNOTATION = 6;
}

enum PrivacyClass {
  PRIVACY_CLASS_UNSPECIFIED = 0;
  PRIVATE = 1;
  SENSITIVE = 2;
  RESTRICTED = 3;
}

enum RetentionClass {
  RETENTION_CLASS_UNSPECIFIED = 0;
  STANDARD = 1;
  LONG_TERM = 2;
  EPHEMERAL = 3;
  USER_MANAGED = 4;
}
```

`UNSPECIFIED` 只用于检测缺失值，不得通过接入校验。公开可见性不是
`PrivacyClass` 的枚举值；公开数据必须由独立投影产生。

### 3.3 时间语义

| 字段 | 来源 | 含义 | 参与事件时间计算 |
| --- | --- | --- | --- |
| `observed_at` | Agent | 事实发生或首次被观测的时刻 | 是 |
| `ended_at` | Agent | 区间结束时刻 | 仅区间 |
| `received_at` | Ingestion | 服务端完整收到请求的时刻 | 否 |
| `ingested_at` | Ingestion | 最终成功 Kafka Record 的服务端写入时间；send 前生成，ACK 后才生效 | 否 |
| `processed_at` | Processor | 当前派生结果完成的时刻 | 否 |

时间戳必须是 UTC `google.protobuf.Timestamp`。`timezone` 使用 IANA TZDB
名称，例如 `Asia/Shanghai`；它描述观测时设备采用的民用时区，不改变
时间戳本身。

校验规则：

- `observed_at` 必填且必须是有效 Timestamp；
- `INTERVAL` 的 `ended_at` 可暂缺以表示开放区间，存在时不得早于
  `observed_at`；
- 非 `INTERVAL` 记录不得设置 `ended_at`，除非对应 Stream 定义明确允许；
- 接入端不得因为时钟偏差修改原时间，只能附加质量标记；
- 闰秒、时钟回拨和时区变化不得导致 `event_id` 或 `sequence` 重写。

## 4. 普通事件契约

### 4.1 EventEnvelope

```protobuf
message EventEnvelope {
  string event_id = 1;
  string stream = 2;
  string event_type = 3;
  RecordKind kind = 4;

  string user_id = 5;
  string device_id = 6;
  string collector_instance_id = 7;
  string source = 8;

  uint32 schema_version = 9;
  uint64 sequence = 10;

  google.protobuf.Timestamp observed_at = 11;
  optional google.protobuf.Timestamp ended_at = 12;
  string timezone = 13;

  PrivacyClass privacy_class = 14;
  RetentionClass retention_class = 15;

  Origin origin = 16;
  google.protobuf.Any payload = 20;
}
```

保留字段号 `17` 至 `19`，在 v1 中不得使用。

### 4.2 字段约束

| 字段 | 约束 |
| --- | --- |
| `stream` | 必须存在于 Registry，格式见第 7 节 |
| `event_type` | 必须等于 Registry 当前版本声明的事件类型 |
| `kind` | 必须等于 Stream 定义的记录类型 |
| `user_id` | Agent 留空；Ingestion 根据设备身份注入并覆盖外部输入 |
| `device_id` | 必须与认证设备一致 |
| `collector_instance_id` | 必须属于该设备且未撤销 |
| `source` | 小写点分标识，格式为 `<platform>.<collector>` |
| `schema_version` | 必须是 Registry 中可接收的版本 |
| `sequence` | 在 `(device_id, collector_instance_id, source)` 内单调递增 |
| `privacy_class` | 只能等于或严于 Stream 默认值 |
| `retention_class` | 必须是 Stream 允许的保留类别 |
| `origin` | 必须声明采集方式、原始来源和导入关系 |
| `payload` | `type_url` 和二进制内容必须匹配登记的 Protobuf 消息 |

`sequence` 缺口、倒退或重复是数据质量信号，不单独构成拒收理由；重复事实由
`event_id` 判定。若 Collector 丢失持久序列状态，必须生成新的
`collector_instance_id`，不得从旧实例的序列 0 继续。

### 4.3 Origin

```protobuf
message Origin {
  string provider = 1;
  string provider_record_id = 2;
  string import_id = 3;
  string parent_event_id = 4;
  string collection_method = 5;
}
```

- 原生采集至少填写 `provider` 和 `collection_method`；
- 外部导入必须填写 `provider_record_id` 或 `import_id`；
- 派生数据必须填写 `parent_event_id`，或在 `Lineage` 中声明完整输入范围；
- Origin 字段不得包含访问令牌、文件绝对路径或用户敏感正文。

### 4.4 修正、替代和删除

原始记录不得 `UPDATE`。控制记录使用独立 Payload：

```protobuf
message Correction {
  string target_event_id = 1;
  string replacement_event_id = 2;
  string reason_code = 3;
}

message Tombstone {
  string target_id = 1;
  string target_kind = 2; // event | chunk | range
  string reason_code = 3;
}

message Annotation {
  repeated string target_event_ids = 1;
  repeated string labels = 2;
  string note = 3;
}
```

`replacement_event_id` 指向一条独立、已接收的新事实。Tombstone 是逻辑删除
意图；跨存储层物理删除由后续删除工作流完成。

## 5. 高频 Series 契约

### 5.1 SeriesChunk

```protobuf
message SeriesChunk {
  string chunk_id = 1;
  string stream = 2;
  uint32 schema_version = 3;

  int64 start_time_ns = 4;
  int64 end_time_ns = 5;
  double nominal_sample_rate = 6;

  repeated int64 timestamp_delta_ns = 7;
  repeated Channel channels = 8;

  bytes compressed_payload = 9;
  bytes checksum = 10;

  string device_id = 11;
  string collector_instance_id = 12;
  string source = 13;
  string timezone = 14;
  PrivacyClass privacy_class = 15;
  RetentionClass retention_class = 16;
  uint64 sequence = 17;
  ClockMetadata clock = 18;
}
```

### 5.2 分块和校验规则

- `start_time_ns` 和 `end_time_ns` 为 Unix Epoch UTC 纳秒；
- `end_time_ns >= start_time_ns`；
- `timestamp_delta_ns` 相对 `start_time_ns` 严格非递减；
- 固定采样率可省略每点 delta，但 Registry 必须声明重建规则；
- 每个 Channel 必须声明名称、数据类型、单位、缩放和缺失值编码；
- `checksum` 为 `compressed_payload` 经 zstd 解压后所得**原始载荷字节**的
  SHA-256；不得把解码后的 Channel/Sample 再序列化后计算；
- Registry 必须按 `(stream, schema_version)` 声明 `series_payload_format` 及其
  精确字节布局、端序、数值编码和版本；格式变更必须发布新 Schema 版本；
- 压缩格式 v1 仅允许 `zstd`；
- 单 Chunk 默认目标时长、最大未压缩字节数和最大样本数由 Stream 定义；
- Chunk 不得作为数百万个普通 Event 展开后再进入 `lc.raw.events.v1`；
- 元数据进入 Kafka，二进制块进入对象存储；二者以 `chunk_id` 关联；
- 对象键必须由租户隔离前缀、`chunk_id` 和内容摘要确定；同一键同一摘要的 PUT
  必须幂等，同一键不同摘要必须拒绝；
- Series 接入顺序为“校验摘要 → 持久写对象 → 写 Kafka metadata → 返回
  `ACCEPTED_TO_LOG`”。Kafka 写入失败后留下的未引用对象由带安全期的孤儿回收
  任务清理；重试必须复用原对象和 metadata，不得生成新 `chunk_id`；
- 阶段 0–4 只冻结和测试契约，不要求实现生产级序列接入。

## 6. Batch、签名和确认

### 6.1 上传批次

```protobuf
message UploadBatch {
  string batch_id = 1;
  string device_id = 2;
  string collector_instance_id = 3;
  uint64 sequence_start = 4;
  uint64 sequence_end = 5;
  google.protobuf.Timestamp created_at = 6;
  bytes nonce = 7;
  Compression compression = 8;
  bytes compressed_items = 9;
  bytes payload_sha256 = 10;
  bytes signature = 11;
  string source = 12;
}

message BatchItems {
  repeated EventEnvelope events = 1;
  repeated SeriesChunk series_chunks = 2;
}
```

批次只能包含一个 `device_id`、一个 `collector_instance_id` 和一个
`source`；每个条目的这三个字段必须与批次头一致。普通事件单批上限为
10,000 条；压缩后字节上限由部署配置声明并由 gRPC、HTTPS 保持一致。若同时
携带事件和 Chunk，`sequence_start/end` 必须覆盖两类条目的统一序列范围。

`payload_sha256` 必须是设备实际发送的 `compressed_items` 字节的 SHA-256，
不得对解压或重新序列化后的消息计算。重试同一批次时必须复用完全相同的
`compressed_items`、摘要和签名。

签名输入不得直接使用 Protobuf 序列化结果。Protobuf 的 deterministic
serialization 不是跨语言、跨构建版本稳定的 canonical encoding。v1 使用下列
独立规范帧；`||` 表示字节连接：

```text
ASCII("LCB1")
|| string(batch_id)
|| string(device_id)
|| string(collector_instance_id)
|| u64be(sequence_start)
|| u64be(sequence_end)
|| i64be(created_at.seconds)
|| u32be(created_at.nanos)
|| bytes(nonce)
|| u32be(compression)
|| string(source)
|| fixed32(payload_sha256)
```

- `string(x)` 为 `u32be(UTF-8 字节数) || UTF-8(x)`；字符串必须先通过本规范
  的小写 ASCII/UUID 规范校验，不做语言相关格式化；
- `bytes(x)` 为 `u32be(字节数) || x`；
- `u32be`、`u64be` 和 `i64be` 使用固定宽度大端二进制；
- `fixed32` 要求输入恰好 32 字节，不再附加长度；
- `compression` 使用 Proto 枚举的无符号数值；
- 新的签名帧只能用新魔数和版本发布，不得原地改变 `LCB1`。

设备提交的每个 Event 的 `user_id` 必须为空；服务端只有在签名和哈希验证通过
后才向服务端副本注入 `user_id`。具体密钥注册、轮换、Nonce 有效期和防重放
存储由设备身份规范定义。

### 6.2 确认模型

```protobuf
enum ItemStatus {
  ITEM_STATUS_UNSPECIFIED = 0;
  ACCEPTED_TO_LOG = 1;
  DUPLICATE = 2;
  REJECTED_PERMANENT = 3;
  RETRYABLE = 4;
}

message ItemAcknowledgement {
  string item_id = 1;
  ItemStatus status = 2;
  string error_code = 3;
  string error_detail_id = 4;
}

message BatchAcknowledgement {
  string batch_id = 1;
  repeated ItemAcknowledgement items = 2;
  google.protobuf.Timestamp acknowledged_at = 3;
}
```

| 状态 | Agent 行为 | 服务端前置条件 |
| --- | --- | --- |
| `ACCEPTED_TO_LOG` | 可清理对应 Outbox 条目 | Event 已获 Kafka 持久确认；Series 的对象和 Kafka metadata 均已持久确认；对应逐项终态及证据已进入可恢复幂等记录 |
| `DUPLICATE` | 可清理，保留审计计数 | 可恢复记录或权威存储已证明同 ID、同内容满足相同持久边界，并已持久化本次可返回终态 |
| `REJECTED_PERMANENT` | 移入本地隔离队列 | 错误不可通过原批重试修复 |
| `RETRYABLE` | 指数退避并复用全部 ID | 尚未形成可靠持久确认 |

同 ID 不同内容不得返回 `DUPLICATE`，必须返回
`REJECTED_PERMANENT/ID_CONTENT_CONFLICT`。批次整体失败时不得为未实际写入
Kafka 的条目返回 `ACCEPTED_TO_LOG`。Kafka/对象已成功但逐项终态证据尚未可靠
持久时也不得返回可清理 ACK；后续 exact retry 必须通过权威存储核验并安全补齐
该证据。

Nonce 的唯一性约束针对“新构造的批次”。同一设备密钥下，完全相同的
`(batch_id, nonce, payload_sha256, signature)` 是合法原样重试，服务端必须返回
已持久化的确认结果，或以相同幂等规则安全重算结果；相同 Nonce 绑定到不同
Batch ID、摘要或签名时才返回 `NONCE_REPLAYED`。

### 6.3 错误码基线

| 错误码 | 类型 | 含义 |
| --- | --- | --- |
| `AUTH_INVALID` | 永久 | 认证材料无效 |
| `DEVICE_REVOKED` | 永久 | 设备已撤销 |
| `SIGNATURE_INVALID` | 永久 | 签名不匹配 |
| `NONCE_REPLAYED` | 永久 | Nonce 已绑定到不同 Batch ID、摘要或签名 |
| `PAYLOAD_HASH_MISMATCH` | 永久 | `compressed_items` 原始字节与摘要不匹配 |
| `PROTO_DECODE_FAILED` | 永久 | Protobuf 无法解析 |
| `STREAM_UNKNOWN` | 永久 | Stream 未登记 |
| `SCHEMA_VERSION_UNSUPPORTED` | 永久 | Schema 版本不可接收 |
| `SCHEMA_VALIDATION_FAILED` | 永久 | Payload 不符合 Schema |
| `ID_CONTENT_CONFLICT` | 永久 | 相同 ID 对应不同内容 |
| `BATCH_LIMIT_EXCEEDED` | 永久 | 条数或字节数超过限制 |
| `RATE_LIMITED` | 重试 | 当前超过配额 |
| `KAFKA_UNAVAILABLE` | 重试 | 无法取得 Kafka 持久确认 |
| `OBJECT_STORAGE_UNAVAILABLE` | 重试 | Series 对象无法取得持久写入确认 |
| `INTERNAL_RETRYABLE` | 重试 | 未产生可靠确认的内部故障 |

对外响应只包含安全的错误码和 `error_detail_id`；完整错误写入受限日志，不得
回显原始 Payload。

## 7. Stream Registry

### 7.1 命名规则

内建 Stream 使用：

```text
<domain>.<subject>[.<measurement-or-state>]
```

要求：

- 仅允许小写 ASCII 字母、数字和下划线，段之间用点分隔；
- 至少两段，最多五段；每段以字母开头；
- 名称表达业务事实，不包含版本、环境、数据库或设备名；
- Schema 版本只放在 `schema_version`；
- 第三方名称固定为
  `plugin.<publisher>.<plugin>.<metric>`；
- 已发布名称不得改义；重命名通过新 Stream 和迁移关系完成。

### 7.2 定义文件格式

每个 Stream 对应一个版本化 YAML 文件：

```yaml
api_version: lifechronicle.io/stream/v1
name: app.foreground
status: active
schema:
  version: 1
  payload_type: lifechronicle.events.v1.AppForeground
  event_type: app.foreground.observed
record_kind: STATE
unit: null
defaults:
  privacy_class: PRIVATE
  retention_class: LONG_TERM
accepted_retention_classes: [STANDARD, LONG_TERM, USER_MANAGED]
event_time:
  field: observed_at
  max_clock_skew: 24h
  max_out_of_order: 15m
  realtime_allowed_lateness: 24h
  late_event_action: topic_and_workflow_replay
partitioning:
  semantic: device_order
  fields: [user_id, device_id, collector_instance_id]
source_policy:
  strategy: preserve_all
  priorities: []
aggregation: []
processors:
  - id: normalization
    accepted_schema_versions: [1]
privacy:
  public_projection: none
series: null
```

Registry Schema 必须拒绝未知顶层字段，避免拼写错误被静默忽略。时长统一使用
`ns`、`us`、`ms`、`s`、`m`、`h`、`d` 后缀；单位采用 UCUM 可表达形式，
无单位时为 `null`。

### 7.3 生命周期

`draft → active → deprecated → retired`

- `draft`：只允许测试环境；
- `active`：允许生产写入；
- `deprecated`：允许既有 Producer 写入，但新 Producer 不得采用；
- `retired`：接入拒绝新写入，历史仍可读取和回放。

状态变更必须经代码评审、兼容检查和 Registry 契约测试。删除 Registry 文件
不得用于退役 Stream。

### 7.4 首批 Stream

| Stream | 类型 | 原始/派生 | Payload | 默认迟到 | 阶段 4 输出 |
| --- | --- | --- | --- | --- | --- |
| `app.foreground` | `STATE` | 原始 | `AppForeground` | 24h | 应用会话 |
| `device.idle.state` | `STATE` | 原始 | `IdleState` | 24h | Idle 会话 |
| `device.screen.state` | `STATE` | 原始 | `ScreenState` | 24h | Screen 会话 |
| `device.power.state` | `STATE` | 原始 | `PowerState` | 24h | 最新状态 |
| `device.network.type` | `STATE` | 原始 | `NetworkType` | 24h | 最新状态 |
| `user.presence` | `INTERVAL` | 派生 | `PresenceInterval` | 不适用 | Presence 会话 |

阶段 0 必须登记前三个原始 Stream；后两个可作为扩展契约；`user.presence`
只能由处理器输出，设备不得直接写入。

## 8. Kafka Topic 契约

### 8.1 Topic 基线

| Topic | Key 语义 | Value | Producer | Consumer |
| --- | --- | --- | --- | --- |
| `lc.raw.events.v1` | device order | `RawEventRecord` | Ingestion | Bronze Sink、Flink |
| `lc.raw.series-metadata.v1` | series order | `RawSeriesRecord` | Ingestion | Series Archiver |
| `lc.corrections.v1` | target ID | `CorrectionRecord` | Ingestion | Flink、Lake Sink |
| `lc.tombstones.v1` | target ID | `TombstoneRecord` | Ingestion | 删除工作流 |
| `lc.normalized.events.v1` | device order | `DerivedEventRecord` | Flink | Silver Sink、下游 Job |
| `lc.device.latest-state.v1` | user + device + stream | `LatestState` | Flink | ClickHouse Sink |
| `lc.sessions.application.v1` | user + device | `ApplicationSession` | Flink | Gold/ClickHouse Sink |
| `lc.sessions.presence.v1` | user | `PresenceSession` | Flink | Gold/ClickHouse Sink |
| `lc.processing.late-events.v1` | user + stream | `LateEventRecord` | Flink | Temporal Trigger |
| `lc.processing.errors.v1` | source topic + partition | `ProcessingError` | 各处理器 | 运维/隔离 |
| `lc.data-quality.v1` | user + device | `QualityFinding` | Ingestion/Flink | 质量服务 |
| `lc.audit.events.v1` | actor | `AuditEvent` | 控制平面 | 审计归档 |

Topic 名中的 `v1` 是 Topic Value 契约主版本，不等同于具体 Stream Schema
版本。兼容演进保留 Topic；不兼容 Value 变更创建新 Topic 主版本。

### 8.2 Key 编码

Key 必须使用 Registry 声明字段的 UTF-8 值，按顺序进行“4 字节大端长度 +
字节内容”编码。不得使用简单字符串拼接或未转义分隔符。缺失任一 Key 字段
必须在进入 Kafka 前拒绝。

### 8.3 RawEventRecord

Kafka 中的原始 Value 必须保留设备事实和服务端接入元数据：

```protobuf
message RawEventRecord {
  EventEnvelope event = 1;
  string batch_id = 2;
  google.protobuf.Timestamp received_at = 3;
  google.protobuf.Timestamp ingested_at = 4;
  bytes submitted_sha256 = 5;
  bytes canonical_sha256 = 6;
  string authenticated_principal = 7;
  repeated QualitySignal quality_signals = 8;
}
```

两个摘要都必须是 32 字节 SHA-256，不得通过重新序列化 `EventEnvelope` 或
`Any` 计算。v1 使用下面两个独立内容帧。

#### 8.3.1 `LCE1` 提交内容帧

`submitted_sha256 = SHA-256(LCE1)`。`LCE1` 覆盖设备提交的全部 v1 事件内容，
但明确排除必须为空的 `user_id`：

```text
ASCII("LCE1")
|| string(event_id)
|| string(stream)
|| string(event_type)
|| u32be(kind)
|| string(device_id)
|| string(collector_instance_id)
|| string(source)
|| u32be(schema_version)
|| u64be(sequence)
|| i64be(observed_at.seconds)
|| u32be(observed_at.nanos)
|| u8(ended_at_present)
|| [i64be(ended_at.seconds) || u32be(ended_at.nanos)]  // 仅 present=1
|| string(timezone)
|| u32be(privacy_class)
|| u32be(retention_class)
|| string(origin.provider)
|| string(origin.provider_record_id)
|| string(origin.import_id)
|| string(origin.parent_event_id)
|| string(origin.collection_method)
|| string(payload.type_url)
|| bytes(payload.value)
```

- `string`、`bytes`、`u32be`、`u64be` 和 `i64be` 沿用第 6.1 节的长度和端序；
- `u8` 是单个无符号字节；`ended_at_present` 只能是 `0` 或 `1`；
- 枚举使用 Proto 中登记的无符号数值；
- 字符串先通过各字段自己的校验规则，再按原 UTF-8 字节编码；不做大小写或
  Unicode 归一化；
- `payload.value` 使用 `Any` 中设备实际提交的原始 bytes，不解析后重编码；
- 对 v1 而言，语义等价但 `payload.value` 字节不同的 Payload 是不同内容；
- 新增任何影响事件内容的 Envelope 字段必须发布新内容帧魔数，不能静默沿用
  `LCE1`。

#### 8.3.2 `LCC1` 身份绑定帧

Ingestion 在完成认证、签名/hash、严格 Proto 和 Registry 校验后注入规范小写
`user_id`，并计算：

```text
canonical_sha256 =
  SHA-256(
    ASCII("LCC1")
    || string(user_id)
    || fixed32(submitted_sha256)
  )
```

`canonical_sha256` 绑定可信租户身份与设备提交内容，用于 Event ID 内容冲突和
服务端派生幂等；`received_at`、`ingested_at`、`batch_id`、认证主体和质量信号
不参与内容身份。完全相同内容的重试必须得到相同两个摘要；同 `event_id` 的任一
`LCE1` 字段或绑定 `user_id` 不同，必须判为 `ID_CONTENT_CONFLICT`。

不得将 Kafka partition/offset 预写进 Value；归档 Sink 在提交 Iceberg 时记录
实际 source topic、partition、offset 范围和 snapshot ID。

### 8.4 RawSeriesRecord

Series Topic 只保存可验证 metadata 和对象引用，不复制 `compressed_payload`：

```protobuf
message SeriesChunkMetadata {
  string chunk_id = 1;
  string stream = 2;
  uint32 schema_version = 3;
  int64 start_time_ns = 4;
  int64 end_time_ns = 5;
  double nominal_sample_rate = 6;
  repeated int64 timestamp_delta_ns = 7;
  repeated Channel channels = 8;
  bytes checksum = 9;
  string device_id = 10;
  string collector_instance_id = 11;
  string source = 12;
  string timezone = 13;
  PrivacyClass privacy_class = 14;
  RetentionClass retention_class = 15;
  uint64 sequence = 16;
  ClockMetadata clock = 17;
}

message SeriesObjectReference {
  string object_key = 1;
  string object_version = 2;
  uint64 compressed_size = 3;
  bytes compressed_sha256 = 4;
  Compression compression = 5;
}

message RawSeriesRecord {
  SeriesChunkMetadata chunk = 1;
  string user_id = 2;
  string batch_id = 3;
  google.protobuf.Timestamp received_at = 4;
  google.protobuf.Timestamp ingested_at = 5;
  SeriesObjectReference object = 6;
  bytes series_submitted_sha256 = 7;
  bytes series_canonical_sha256 = 8;
  string authenticated_principal = 9;
  repeated QualitySignal quality_signals = 10;
}
```

字段映射和对象契约：

- `chunk` 是设备 `SeriesChunk` 除 `compressed_payload` 外的逐字段投影；
- `chunk.checksum` 仍是第 5.2 节定义的 zstd 解压后原始载荷摘要；
- `object.compressed_sha256` 是设备实际提交的 `compressed_payload` bytes 的
  SHA-256，`compressed_size` 是这些 bytes 的精确长度；
- `object.object_key` 固定为
  `private/{user_id}/series/{chunk_id}/{lowerhex(compressed_sha256)}.zst`；
- `object.object_version` 必须标识已持久的不可变对象版本/代；对没有原生版本号的
  内容寻址存储，使用小写 `compressed_sha256` hex；
- Kafka metadata 中的对象字段必须来自成功的持久写结果，不能由客户端指定；
- 对象内容必须是 `SeriesChunk.compressed_payload` 的原始 bytes，不包含 Proto
  外壳，也不得重新压缩。

为避免嵌套 Channel、Clock 和浮点的重序列化歧义，Series Item 的内容身份覆盖设备
实际提交的 wire bytes。安全解压 `BatchItems` 时，Ingestion 必须保留每个
`series_chunks` length-delimited value 的原始 bytes（排除父消息 field tag 和
length prefix），不得由解码对象重序列化：

```text
series_submitted_sha256 =
  SHA-256(ASCII("LCS1") || bytes(submitted_series_chunk_wire_bytes))

series_canonical_sha256 =
  SHA-256(
    ASCII("LCR1")
    || string(user_id)
    || fixed32(series_submitted_sha256)
  )
```

`LCS1` 是有版本和 domain separation 的**精确提交字节摘要帧**，不是语义规范化
编码：语义等价但 wire bytes 不同的 Chunk 视为不同内容。`LCR1` 绑定可信用户。
相同 `chunk_id` 只有在 `series_canonical_sha256` 相同时才能返回 `DUPLICATE`；
任一 metadata、压缩载荷 bytes 或绑定用户不同都必须返回
`ID_CONTENT_CONFLICT`。新的字节域规则必须使用新魔数。

### 8.5 Producer 和保留要求

- Ingestion 启用 Kafka 幂等 Producer 和 `acks=all`；
- `min.insync.replicas` 必须与环境副本数匹配；
- `ingested_at` 在单项 Producer send 前生成并写入候选 Record；只有该次 send
  成功返回后它才生效并允许返回确认。它不是 broker ACK 的精确发生时刻；
  ACK 响应时间使用 `BatchAcknowledgement.acknowledged_at`；
- 普通故障不得自动创建 Topic；
- Topic 分区数、复制因子、压缩、保留和清理策略以版本化清单管理；
- 原始 Topic 只使用 `delete` 保留策略，不使用会覆盖历史事实的 compact；
- latest-state 类 Topic 可使用 compact，但 ClickHouse/Iceberg 仍保留历史；
- 阶段 3 的本地环境至少验证 30 天补传数据量，生产保留期由容量评审确定。

## 9. Schema 兼容与发布

### 9.1 Protobuf 规则

1. 已使用字段号和枚举号不得复用；
2. 删除字段必须同时 `reserved` 名称和编号；
3. 新增字段必须可选并有明确缺省语义；
4. 不得改变字段单位、时间语义、含义或有符号性；
5. `Any.type_url` 的不兼容变化创建新的 Payload 主版本；
6. 新增必填业务语义、改变单位或改变记录类型必须发布新 Schema 版本；
7. 服务端必须允许 Registry 声明的多个版本并存；
8. 处理器必须显式声明其接受的版本范围；
9. Buf breaking 检查以最近发布标签为基线。

### 9.2 发布顺序

```text
提交 Proto 和 Stream draft
→ lint、breaking、Registry 校验
→ 生成五种语言代码
→ 运行跨语言黄金样例
→ 部署能读取新版本的 Consumer
→ 激活 Registry 版本
→ 最后发布 Producer
```

回滚 Producer 不得要求回滚已接受新版本的 Consumer。Registry 激活和退役必须
留下审计记录。

## 10. 流处理语义

### 10.1 Event Time 与 Watermark

- 每个输入 Stream 从 Registry 读取 `max_out_of_order`；
- 多 Stream Job 的 Watermark 取能够保证正确性的最保守值；
- 空闲分区必须配置 idleness，防止全局 Watermark 永久停止；
- Watermark 只决定实时计算时机，不改变原始事件；
- 迟到但仍在 `realtime_allowed_lateness` 内的事件可更新派生结果；
- 超出允许范围的事件写入 `lc.processing.late-events.v1`，不得静默丢弃。

### 10.2 去重

实时去重主键为 `event_id`；状态 TTL 必须不小于
`realtime_allowed_lateness + max_out_of_order + safety_margin`。超出状态 TTL
的重复由 ClickHouse 业务键、Iceberg source offset/ID 索引和回放流程继续防护。

### 10.3 会话

应用会话按 `(user_id, device_id)` 串行处理。会话终止条件包括：

- 新前台应用；
- 屏幕关闭；
- 用户 Idle；
- 设备关机；
- Collector 明确结束；
- Heartbeat 超时；
- 质量规则强制关闭。

每次更新输出同一稳定 `session_id` 的新 `revision`，Sink 以
`(session_id, revision)` 判定最新版本。不得依赖到达顺序生成随机会话 ID。

### 10.4 派生血缘

所有派生 Value 必须包含：

```protobuf
message Lineage {
  string processor_id = 1;
  string processor_version = 2;
  string rule_version = 3;
  repeated string input_streams = 4;
  TimeRange input_time_range = 5;
  string input_snapshot = 6;
  string output_schema = 7;
  string processor_run_id = 8;
  google.protobuf.Timestamp processed_at = 9;
}
```

实时处理的 `input_snapshot` 使用 Kafka topic/partition/offset 范围的规范表示；
历史回放使用 Iceberg snapshot ID 和文件范围。

## 11. 契约测试基线

下列测试是本规范完成的最低门槛：

| 测试 ID | 测试 | 通过条件 |
| --- | --- | --- |
| `ES-C001` | Proto lint | Buf lint 零错误 |
| `ES-C002` | Breaking 基线 | 删除/改号测试分支被 CI 拒绝 |
| `ES-C003` | 跨语言黄金事件 | Go、Rust、Kotlin、Java、TypeScript 解码结果等价 |
| `ES-C004` | 规范签名帧 | 同一 Batch 的 `LCB1` 签名输入在所有实现中逐字节一致 |
| `ES-C005` | Registry Schema | 未知字段、非法名、缺省隐私均被拒绝 |
| `ES-C006` | Stream/Payload 匹配 | 错误 `type_url` 被拒绝 |
| `ES-C007` | 时间边界 | 无效 Timestamp、反向区间被拒绝 |
| `ES-C008` | ID 幂等 | 同 ID 同内容为 Duplicate，不同内容为 Conflict |
| `ES-C009` | 分区键黄金向量 | 各语言生成完全相同 Key 字节 |
| `ES-C010` | 普通/序列隔离 | Series 不进入 `lc.raw.events.v1` |
| `ES-C011` | 确认时点与恢复 | Kafka 或可恢复逐项终态证据未确认时不存在 `ACCEPTED_TO_LOG`；Kafka 成功/终态落库前崩溃可在重试中核验并补齐，不重复事实 |
| `ES-C012` | 默认私有 | 新 Stream 未声明隐私时校验失败，模板默认 PRIVATE |
| `ES-C013` | 迟到路由 | 超窗事件进入 late Topic 且不被静默丢弃 |
| `ES-C014` | 回放确定性 | 同一黄金输入两次输出逐字段一致 |
| `ES-C015` | Series 确认原子性 | 对象、metadata 或可恢复逐项终态证据任一未持久确认时不得清理 Outbox |
| `ES-C016` | Exact retry 与 Nonce 绑定 | 原样重试返回既有/等价 ACK；同 Nonce 绑定不同 Batch、摘要或签名时拒绝 |
| `ES-C017` | Item 内容摘要 | Go、Rust、Kotlin、Java、TypeScript 生成相同 `LCE1/LCC1` 字节与摘要；任一覆盖字段、Payload 原始 bytes 或绑定用户变化均改变摘要 |
| `ES-C018` | Series metadata 与对象引用 | 五语言从同一提交提取相同 `LCS1/LCR1` 摘要和对象 Key；Topic Value 含不可变版本、大小及压缩/解压双摘要；同 Chunk 异 metadata、载荷或用户均冲突 |

黄金样例至少包含：正常日、重复批次、乱序、时钟前跳、时钟回拨、时区变化、
Collector 重置、开放区间、超迟到事件、损坏 Chunk、ID 内容冲突、完全相同批次
原样重试、相同 Nonce 绑定不同 Batch ID、摘要或签名，以及 Payload 语义等价但
原始 bytes 不同的 Item 内容冲突；Series 样例还必须覆盖相同 Chunk wire bytes、
语义等价但 wire bytes 不同、metadata 篡改、压缩载荷篡改和对象版本丢失。

## 12. 变更控制

修改本规范时必须同时回答：

1. 是否改变线上的序列化字节；
2. 是否改变 Stream 业务语义或单位；
3. 是否改变分区键及有状态处理范围；
4. 是否改变接收确认或重试行为；
5. 是否需要新 Topic、双读、双写或回放；
6. 是否影响隐私默认值、保留或删除；
7. 哪些黄金样例和兼容测试证明变更安全。

任何不能向后兼容的变更必须有 ADR、迁移计划、回滚路径和旧版本退役日期。
