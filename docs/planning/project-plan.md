# LifeChronicle 多设备日常数据记录与公开展示平台项目规划书

**文档版本：** v1.0
**状态：** 已接受规划基线
**项目名称：** LifeChronicle
**项目类型：** 以开源为目标、自托管、隐私优先的个人数据平台
**架构原则：** 目标架构优先、契约优先、事件驱动、原始数据不可变
**目标读者：** 项目开发者、维护者及未来贡献者

---

# 1. 文档目的

本文档是 LifeChronicle 的总体项目规划和实施基线，用于回答以下问题：

1. 项目要解决什么问题；
2. 第一阶段和长期阶段分别支持哪些数据；
3. 系统的最终架构是什么；
4. 设备数据如何采集、缓存、上传和确认；
5. 普通事件与高频传感器数据如何统一管理；
6. 原始数据、派生数据和公开数据如何隔离；
7. 应参考哪些开源项目以及分别参考什么；
8. 各服务、存储和处理组件如何分工；
9. 插件、安全、权限和隐私系统如何设计；
10. 从空仓库开始应按照什么顺序实施；
11. 每个阶段如何验收；
12. 后续新增设备、数据类型和算法时需要修改哪些模块。

本文档描述系统级目标。以下内容在实施时还应分别形成独立设计文档：

* [事件协议、Stream 与 Schema Registry 规范](../protocol/event-stream-spec.md)；
* [参考项目调研](../research/reference-project-survey.md)；
* [总体架构说明](../architecture/overall-architecture.md)；
* [项目契约](../contract/project-contract.md)；
* 设备身份与签名规范；
* 流处理规范；
* 数据湖规范；
* 查询 API 规范；
* 隐私和公开投影规范；
* 插件 SDK 规范；
* 数据保留与删除规范；
* [基础设施部署规范](../operations/infrastructure-deployment-spec.md)；
* 部署与灾难恢复手册；
* [阶段 0–12 开发路线图](development-roadmap.md)；
* [聚合开发任务清单](tasks/README.md)。

---

# 2. 项目概述

## 2.1 项目背景

个人每天通过手机、电脑、手表、手环、浏览器、智能家居和其他设备产生大量数据，包括：

* 手机和电脑的前台应用；
* 窗口、浏览器标签和用户空闲状态；
* 屏幕、锁屏、在线、电池和网络状态；
* 步数、睡眠、心率和运动记录；
* 心率变异性、血氧和体温；
* 位置、移动轨迹和地点访问；
* 媒体播放；
* 日历和用户手动标注；
* 环境温度、湿度和空气质量；
* 加速度计、陀螺仪等高频传感器。

这些数据通常存在以下问题：

* 分散在不同应用和厂商服务中；
* 格式不统一；
* 数据来源和记录方式不透明；
* 多个设备可能重复记录同一指标；
* 无法形成统一的跨设备时间线；
* 历史数据不容易完整导出；
* 难以接入自定义设备；
* 厂商停止服务后存在数据丢失风险；
* 很难安全地选择部分数据对外展示。

LifeChronicle 通过统一协议、事件流、数据湖、流式处理和公开投影系统，将这些数据变成可长期控制的个人数字档案。

## 2.2 产品定位

LifeChronicle 的核心定位是：

> 一个面向个人的多设备数据采集、同步、归档、处理、分析和受控发布平台。

它不是单纯的屏幕时间工具，也不是单纯的健康统计工具或在线状态页。

完整产品由以下能力组成：

```text
多设备采集
+ 本地可靠缓存
+ 统一事件接入
+ 不可变原始档案
+ 实时流处理
+ 历史回放
+ 时间序列分析
+ 私有时间线
+ 受控公开窗口
+ 可扩展插件平台
```

## 2.3 核心业务目标

LifeChronicle 应能够：

1. 从不同设备和系统持续采集数据；
2. 在设备离线时本地保存数据；
3. 在网络恢复后补传数天或数月的历史；
4. 保证重复上传不会生成重复数据；
5. 正确处理事件乱序和迟到；
6. 保存设备真实发生时间而非只保存服务器接收时间；
7. 长期保留不可变原始记录；
8. 根据原始记录生成会话、统计和时间线；
9. 在算法升级后重新处理历史数据；
10. 支持普通状态、区间、采样和高频序列；
11. 支持多个设备和多个数据来源；
12. 对重复和冲突来源进行显式处理；
13. 提供完整私有控制台；
14. 提供与私有数据物理隔离的公开窗口；
15. 支持导入、导出、删除、备份和恢复；
16. 支持新增采集器、处理器、算法和展示卡片；
17. 支持从单节点部署扩展到多节点集群。

## 2.4 非目标

LifeChronicle 不以以下内容为核心目标：

* 医疗诊断；
* 自动给出医疗建议；
* 未经允许采集聊天、通知正文或剪贴板；
* 默认持续录制音频或摄像头；
* 默认公开精确位置和健康数据；
* 依赖单一厂商云服务；
* 使用某一个数据模型强行表示所有数据；
* 将所有数据逐条保存为 JSON 数据库行；
* 为实现形式上的分布式而引入没有实际价值的组件。

---

# 3. 总体要求汇总

## 3.1 项目命名

项目统一使用：

```text
产品名：LifeChronicle
仓库名：lifechronicle
服务前缀：lifechronicle-
Token 前缀：lch_
Kafka Topic 前缀：lc.
Protobuf 包前缀：lifechronicle.
```

## 3.2 架构要求

项目从第一版开始直接按照长期目标架构实施，不采用后续必须整体替换的临时核心方案。

应当从第一版建立：

* 统一 Protobuf 契约；
* 本地 WAL 和 Outbox；
* 设备密钥和批次签名；
* 持久事件总线；
* 事件时间流处理；
* 事务型控制数据库；
* 热分析数据库；
* 对象存储数据湖；
* 可回放工作流；
* 独立权限与隐私策略；
* 严格隔离的公开快照；
* 全链路可观测性；
* 插件权限与沙箱。

## 3.3 实施要求

开发路径应满足：

* 每一步具有明确输入、输出和验收条件；
* 协议和 Schema 优先于业务实现；
* 能由契约自动生成多语言代码；
* 所有处理器具有确定性回放测试；
* 所有数据库和 Topic 变更可版本化；
* 不依赖隐含约定；
* 不使用无法自动验证的人工流程作为核心保障；
* 先完成最小业务纵向切片，再扩大数据类型；
* MVP 使用最终数据通路，而不是简化替代通路。

---

# 4. 设计原则

## 4.1 原始数据不可变

服务端接收到的原始事实原则上不直接修改。

修正通过以下方式表达：

```text
Correction
Supersedes
Tombstone
Annotation
```

例如，某条睡眠记录被更新时，不直接覆盖旧数据，而是保存新记录以及它替代的原记录 ID。

这样能够：

* 保留数据来源；
* 审计历史修改；
* 支持不同算法重新处理；
* 避免错误修改无法恢复；
* 比较不同处理器版本结果。

## 4.2 设备端本地优先

采集端的核心流程必须是：

```text
Platform Collector
    ↓
Local Normalizer
    ↓
Append-only Local WAL
    ↓
Local Event Index
    ↓
Outbox
    ↓
Batch Builder
    ↓
Compress and Sign
    ↓
Transport
    ↓
Server Acknowledgement
    ↓
Outbox Cleanup
```

采集器不能直接依赖网络请求。

服务端不可用时：

* 数据继续采集；
* 写入本地 WAL；
* 等待后续上传；
* 不因重复重试产生新 Event ID。

## 4.3 事件时间优先

所有时间相关处理默认使用：

```text
observed_at
```

而不是：

```text
received_at
processed_at
```

每条数据至少区分：

| 字段             | 含义                           |
| ---------------- | ------------------------------ |
| `observed_at`  | 数据实际发生或被设备观测的时间 |
| `ended_at`     | 区间结束时间                   |
| `received_at`  | 服务端收到时间                 |
| `ingested_at`  | 进入事件日志的时间             |
| `processed_at` | 派生处理完成时间               |

系统必须能够处理：

* 事件乱序；
* 长期离线补传；
* 设备时钟快慢；
* 时钟回拨；
* 时区变化；
* 夏令时变化；
* 跨零点会话；
* 设备重装；
* Collector 序列号重置。

## 4.4 普通事件与高频序列分离

普通事件适合以独立事件处理：

* 应用切换；
* 屏幕开关；
* 电量变化；
* 网络变化；
* 睡眠会话；
* 用户标注。

高频数据必须使用序列块：

* 每秒心率；
* RR 间期；
* ECG；
* GPS；
* 加速度计；
* 陀螺仪；
* 运动姿态；
* 高频环境传感器。

高频样本不能先转换成数百万个普通事件再进入系统。

## 4.5 原始、派生和公开数据分离

数据分为三个平面。

### 原始数据平面

保存设备提交的事实：

* 原始事件；
* 原始序列块；
* 原始文件；
* 来源元数据；
* 修正和删除事件。

### 派生数据平面

保存可重建结果：

* 规范化事件；
* 应用使用会话；
* 屏幕会话；
* 睡眠统计；
* 小时和每日指标；
* 地点访问；
* 数据质量发现。

### 公开数据平面

只保存经过隐私规则转换后的快照：

* 在线状态；
* 延迟后的活动类别；
* 取整步数；
* 模糊睡眠时长；
* 城市级位置；
* 用户主动填写的文本。

公开服务不能直接读取原始和私有派生数据。

## 4.6 所有派生结果可回放

所有派生结果必须包含：

* 处理器名称；
* 处理器版本；
* 输入 Stream；
* 输入时间范围；
* 输入数据版本；
* 输出 Schema；
* 规则版本；
* 生成时间。

处理器升级时采用：

```text
创建隔离输出版本
  → 重放历史
  → 比较结果
  → 验证
  → 原子切换
  → 保留旧版本回滚
```

## 4.7 默认私有

所有新 Stream 默认：

```text
privacy_class = private
```

新增数据类型不得自动出现在公开 API。

---

# 5. 参考开源项目

LifeChronicle 不从单一项目完整派生，而是分别参考不同项目已经验证的设计。

实施前在以下目录建立研究笔记：

```text
docs/research/
```

每份笔记至少记录：

```text
项目名称
研究版本或提交
许可证
重点目录
重点数据模型
计划借鉴的设计
明确不借鉴的设计
许可证风险
对应 LifeChronicle 模块
```

任何代码复用都必须以复用时仓库中的实际许可证和文件头为准。

---

## 5.1 ActivityWatch

### 参考范围

* Bucket 和 Event；
* Heartbeat；
* Watcher；
* 前台应用检测；
* 用户空闲检测；
* 应用分类；
* 客户端缓存；
* 时间范围查询；
* 数据导出。

### 重点借鉴

```text
Watcher 与服务器解耦
前台应用和 AFK 的关联
相同活动的 Heartbeat 合并
面向时间范围的事件查询
采集器独立命名空间
```

### 需要重新设计

* 增加严格 Schema Registry；
* 增加设备身份和事件签名；
* 增加长期离线补传语义；
* 增加事件总线和数据湖；
* 增加高频序列；
* 增加事件血缘和处理器版本；
* 不把 Bucket 同时作为权限、设备和 Schema 边界。

### 对应 LifeChronicle 模块

```text
desktop-agent
app.foreground
device.idle.state
sessionizer
timeline
```

---

## 5.2 Home Assistant Recorder

### 参考范围

* 当前状态；
* 原始历史；
* 短期统计；
* 长期统计；
* 数据清理；
* 数据库迁移；
* 指标元数据。

### 重点借鉴

```text
当前状态与历史分离
短期历史与长期统计分离
指标单位和统计类型
数据保留和清理任务
历史数据迁移
```

### 需要重新设计

LifeChronicle 不使用智能家居 Entity 作为统一数据模型，也不复制完整的 Home Assistant 事件总线和集成加载体系。

### 对应模块

```text
latest-state
hourly-aggregate
daily-aggregate
retention
metric-registry
```

---

## 5.3 OwnTracks 与 OwnTracks Recorder

### 参考范围

* 移动设备位置上传；
* HTTP 和 MQTT；
* 设备标识；
* 弱网络场景；
* 位置历史；
* GeoJSON；
* 实时位置和历史位置分离。

### 重点借鉴

```text
统一消息结构支持不同传输
移动端离线缓存
轻量位置接收端
位置导入和导出
设备维度的数据组织
```

### 对应模块

```text
location.collector
location.position
mqtt-gateway
geojson-exporter
```

第一版正式传输支持 gRPC 和 HTTPS。MQTT 作为 IoT Gateway 的补充接入方式。

---

## 5.4 OpenTracks

### 参考范围

* Android 长时间记录；
* 前台服务；
* 会话生命周期；
* 本地数据库；
* GPS 和传感器；
* 断线恢复；
* GPX/KML 导入导出。

### 重点借鉴

```text
长时间采集生命周期
设备异常退出后的会话恢复
传感器连接管理
本地可靠写入
采集状态 UI
```

### 对应模块

```text
android-agent
exercise-session
location-collector
sensor-session
export
```

---

## 5.5 Gadgetbridge

### 参考范围

* Device Coordinator；
* 设备能力声明；
* BLE 协议适配；
* 分批同步历史；
* 睡眠、步数和心率解析；
* 固件差异；
* Health Connect 桥接。

### 重点借鉴

```text
每类设备独立 Provider
设备能力表
同步游标
协议差异隔离
健康数据归一化
```

### 实施边界

首批穿戴设备数据优先通过：

```text
手表或手环
  → 厂商应用或 Gadgetbridge
  → Health Connect
  → LifeChronicle Android Agent
```

直接 BLE Provider 在 Health Connect 链路稳定后实现。

Gadgetbridge 类项目的代码复用必须特别检查许可证。设备协议实现应保留明确来源，必要时采用独立协议说明后重新实现。

### 对应模块

```text
wearable-provider
device-capabilities
health-normalizer
sync-cursor
```

---

## 5.6 Sleepy

### 参考范围

* 当前设备状态；
* Android 前台应用检测；
* 简洁的公开状态页面；
* 卡片展示；
* 应用历史统计插件原型。

### 重点借鉴

```text
状态上报体验
简洁公共页面
活动状态卡片
设备状态变化触发扩展
```

### 必须重新设计

Sleepy 历史应用统计原型不适合作为 LifeChronicle 的最终历史系统，主要需要补充：

* 客户端事件时间；
* 设备与用户维度；
* 幂等；
* 持久开放会话；
* 离线补传；
* 乱序处理；
* 通用 Stream；
* 数据血缘；
* 高频数据；
* 公开和私有数据物理隔离。

### 对应模块

```text
public-window
public-card
android-accessibility
application-session
```

---

## 5.7 Traccar

### 参考范围

* 多设备身份；
* 位置协议；
* 地理围栏；
* 位置报告；
* 告警；
* 设备管理。

### 对应模块

```text
device-identity
location
geofence
alert-engine
reporting
```

---

## 5.8 Dawarich

### 参考范围

* 地图时间线；
* 地点停留；
* 行程识别；
* 长期位置历史；
* GPX、GeoJSON 和 OwnTracks 导入。

### 对应模块

```text
location-timeline
visit-detector
trip-detector
map-view
location-import
```

---

## 5.9 CloudEvents

用于参考统一事件上下文：

```text
id
source
type
specversion
subject
time
datacontenttype
```

LifeChronicle 在其基础上增加设备、Stream、记录类型、Sequence、隐私和保留信息。

---

## 5.10 Health Connect

作为 Android 健康数据首选统一入口。

重点支持：

```text
StepsRecord
SleepSessionRecord
SleepStage
HeartRateRecord
ExerciseSessionRecord
DistanceRecord
TotalCaloriesBurnedRecord
```

重点研究：

* UID；
* Client Record ID；
* Recording Method；
* Data Origin；
* Changes Token；
* 聚合读取；
* 更新；
* 删除；
* 权限变化；
* 序列记录。

---

## 5.11 基础设施参考

| 组件              | 用途                              |
| ----------------- | --------------------------------- |
| Apache Kafka      | 持久事件主干和回放                |
| Apache Flink      | Event Time、Watermark、状态和窗口 |
| Temporal          | 长期工作流、回放、删除和导出      |
| ClickHouse        | 热分析和交互式时间查询            |
| Apache Iceberg    | 原始档案和表级版本演进            |
| Apache Parquet    | 高频和历史列式存储                |
| Open Policy Agent | 授权和隐私决策                    |
| Wasmtime          | 插件沙箱                          |
| OpenTelemetry     | Trace、Metric 和 Log              |

---

# 6. 目标总体架构

```text
┌──────────────────────────── 设备层 ────────────────────────────┐
│ Android │ Windows │ macOS │ Linux │ Watch │ Browser │ IoT    │
│ Collector → Local WAL → Outbox → Sign → Batch/Stream          │
└──────────────────────────────┬─────────────────────────────────┘
                               │
                     Protobuf + zstd
                     gRPC / HTTPS / MQTT Gateway
                               │
                               ▼
┌──────────────────────────── 接入平面 ──────────────────────────┐
│ API Gateway                                                   │
│ Device Identity                                               │
│ Signature Verification                                        │
│ Schema Validation                                             │
│ Idempotency                                                   │
│ Clock and Sequence Validation                                 │
│ Rate Limiting                                                 │
└──────────────────────────────┬─────────────────────────────────┘
                               ▼
┌──────────────────────────── 事件主干 ──────────────────────────┐
│ Apache Kafka                                                  │
│ raw │ correction │ tombstone │ normalized │ session │ metric  │
└───────────────┬────────────────────┬────────────────────────────┘
                │                    │
                ▼                    ▼
┌────────────────────────┐  ┌───────────────────────────────────┐
│ Apache Flink           │  │ Temporal                          │
│ 实时流处理             │  │ 历史回放、导出、删除、迁移、归档  │
└──────────────┬─────────┘  └───────────────────────────────────┘
               │
      ┌────────┼──────────────────────────┐
      ▼        ▼                          ▼
┌──────────┐ ┌──────────────┐ ┌────────────────────────────────┐
│PostgreSQL│ │ ClickHouse   │ │ S3/MinIO + Iceberg + Parquet  │
│控制平面  │ │热查询和分析   │ │原始数据湖和历史档案             │
└──────────┘ └──────────────┘ └────────────────────────────────┘
      │               │                       │
      └───────────────┴───────────┬───────────┘
                                  ▼
┌──────────────────────────── 查询平面 ──────────────────────────┐
│ Private Query │ Timeline │ Analytics │ Lineage │ Export       │
│ Public Projection │ Plugin Host │ Data Quality                │
└──────────────────────────────┬─────────────────────────────────┘
                               ▼
┌──────────────────────────── 展示层 ────────────────────────────┐
│ 私有控制台 │ 时间线 │ 图表 │ 数据质量 │ 公开窗口 │ API       │
└────────────────────────────────────────────────────────────────┘
```

---

# 7. 核心组件职责

## 7.1 Kafka：持久事件主干

Kafka 是普通 Event 进入服务端后的第一持久化目标。SeriesChunk 先将二进制块
幂等写入对象存储，再把 metadata 写入 Kafka；只有两者都持久确认后才允许设备
进入可返回终态。普通 Event 与 Series 都还必须把逐项终态及其持久证据可靠落库，
才能允许设备清理对应 Outbox。

建议 Topic：

```text
lc.raw.events.v1
lc.raw.series-metadata.v1
lc.corrections.v1
lc.tombstones.v1

lc.normalized.events.v1
lc.device.latest-state.v1
lc.sessions.application.v1
lc.sessions.presence.v1
lc.metrics.hourly.v1
lc.metrics.daily.v1

lc.public.projection-requests.v1
lc.public.snapshots.v1

lc.processing.errors.v1
lc.data-quality.v1
lc.audit.events.v1
```

以上是长期规划候选名，不构成可创建 Topic 的契约。当前 v1 基线只以
[事件与 Stream 规范第 8 节](../protocol/event-stream-spec.md#8-kafka-topic-契约)
为准；指标和公开投影 Topic 必须在对应阶段先补齐 Key、Value、Producer、
Consumer、保留和迁移规则后才能创建。

分区键应根据处理语义确定：

```text
设备顺序事件：
user_id + device_id + collector_instance_id

用户级聚合：
user_id

高频序列：
user_id + device_id + stream
```

Kafka 保存可回放的近期日志，长期永久档案进入 Iceberg。

## 7.2 Flink：事件时间流处理

Flink 负责：

* Event Time；
* Watermark；
* 乱序；
* 允许迟到；
* 有状态去重；
* 规范化；
* 最新状态；
* 应用会话；
* 在线会话；
* 睡眠处理；
* 分钟、小时和每日统计；
* 数据质量；
* 公开投影触发。

主要 Job：

```text
RawEventValidationJob
EventDeduplicationJob
NormalizationJob
LatestStateJob
ApplicationSessionJob
PresenceSessionJob
HealthNormalizationJob
LocationProcessingJob
HourlyAggregationJob
DailyAggregationJob
PublicProjectionTriggerJob
DataQualityJob
```

每个 Stream 在 Registry 中声明：

```yaml
event_time:
  field: observed_at
  max_out_of_order: 15m
  realtime_allowed_lateness: 24h
  late_event_action: workflow_replay
```

## 7.3 Temporal：长期可靠工作流

Temporal 负责：

```text
Range Replay
Aggregate Rebuild
Account Export
Data Deletion
Retention
Device Migration
Plugin Backfill
Application Reclassification
Backup Verification
Historical Import
```

这些操作不能作为一次普通 HTTP 请求执行。

## 7.4 PostgreSQL：事务型控制元数据

PostgreSQL 保存：

```text
users
identities
devices
device_keys
collector_instances
stream_definitions
schema_versions
metric_definitions
privacy_policies
retention_policies
plugin_manifests
plugin_grants
workflow_metadata
public_profiles
share_tokens
audit_index

# 独立 ingestion coordination 数据库
ingestion_batches
ingestion_nonce_bindings
ingestion_item_ack_state
```

控制库和接入协调库使用不同 owner、角色、迁移和连接池。接入库只保存
Batch/Nonce/摘要、逐项 ACK 与恢复协调，不保存原始 Payload。PostgreSQL 不作为
海量历史事实数据、最新状态或时间线的主存储。

## 7.5 ClickHouse：热分析数据

ClickHouse 保存近期可交互查询的数据：

```text
normalized_events
application_sessions
device_state_intervals
health_samples
health_sessions
location_points
location_visits
hourly_metrics
daily_metrics
data_quality_findings
```

适用于：

* 时间线；
* 图表；
* 快速聚合；
* 设备比较；
* 应用分类统计；
* 健康趋势。

## 7.6 Iceberg 与 Parquet：永久档案

数据湖分三层：

```text
Bronze
  原始事件
  原始序列
  原始导入文件
  修正和删除事件

Silver
  规范化事件
  清理后的健康数据
  标准化位置
  统一设备数据

Gold
  会话
  小时指标
  每日指标
  长期特征
```

Iceberg 负责表版本、Schema 演进和分区演进。

Parquet 负责高效压缩和历史分析。

## 7.7 OPA：权限和隐私决策

OPA 用于：

* 用户授权；
* 设备权限；
* API Scope；
* 插件权限；
* Stream 读取权限；
* 公开规则；
* 导出范围；
* 删除范围。

OPA 只进行决策，不直接执行数据模糊化。

## 7.8 Wasmtime：插件沙箱

第三方服务端插件使用 WebAssembly Component Model。

默认不具有：

* 网络；
* 文件系统；
* 环境变量；
* 数据库连接；
* 任意 Stream 读取；
* 任意 Stream 写入。

权限通过 Manifest 和 Host Capability 明确授予。

## 7.9 OpenTelemetry：可观测性

统一采集：

```text
Trace
Metric
Log
Baggage
```

推荐后端：

```text
Prometheus 或 Mimir
Loki
Tempo
Grafana
```

可观测性数据不得包含：

* Token；
* 完整窗口标题；
* 精确位置；
* 健康原始数据；
* 请求完整 Payload。

---

# 8. 服务与语言划分

| 模块              | 推荐技术               |
| ----------------- | ---------------------- |
| Android Agent     | Kotlin                 |
| Desktop Agent     | Rust                   |
| Browser Extension | TypeScript             |
| IoT Gateway       | Rust                   |
| API Gateway       | Go                     |
| Device Identity   | Go                     |
| Ingestion Service | Go                     |
| Query Service     | Go                     |
| Public Projection | Go                     |
| Temporal Worker   | Go                     |
| Flink Jobs        | Java 或 Kotlin         |
| 离线分析          | Python                 |
| Web               | TypeScript + SvelteKit |
| Plugin Host       | Rust + Wasmtime        |
| 协议              | Protobuf + Buf         |

语言边界通过 Protobuf、Kafka Topic Schema 和 WIT 契约连接，不共享隐式内部对象。

---

# 9. 统一数据协议

## 9.1 Event Envelope

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
  google.protobuf.Timestamp ended_at = 12;
  string timezone = 13;

  PrivacyClass privacy_class = 14;
  RetentionClass retention_class = 15;

  Origin origin = 16;
  google.protobuf.Any payload = 20;
}
```

用户 ID 不一定由设备直接提交。接入服务可根据设备身份在服务端注入。

## 9.2 基础记录类型

| 类型           | 用途                       |
| -------------- | -------------------------- |
| `STATE`      | 屏幕、网络、充电和在线状态 |
| `INTERVAL`   | 应用使用、睡眠、运动       |
| `SAMPLE`     | 电量、心率、体温、位置     |
| `DELTA`      | 步数、距离、流量           |
| `SERIES`     | 高频心率、GPS、IMU         |
| `ANNOTATION` | 用户备注、标签和修正       |

## 9.3 Stream 命名

```text
app.foreground
app.browser.tab
device.screen.state
device.idle.state
device.battery.level
device.power.state
device.network.type
user.presence
health.steps
health.sleep.session
health.sleep.stage
health.heart_rate
health.hrv
health.blood_oxygen
health.exercise.session
location.position
location.visit
media.playback
environment.temperature
sensor.accelerometer
sensor.gyroscope
```

第三方：

```text
plugin.<publisher>.<plugin>.<metric>
```

## 9.4 ID 与序列

### Event ID

使用 UUIDv7。

要求：

* 全局唯一；
* 大致按时间排序；
* 事件写入本地 WAL 后不可改变；
* 重试必须使用相同 ID。

### Collector Instance ID

每次安装、重置或数据身份重建时生成新的 ID。

### Sequence

在以下范围内单调增加：

```text
device_id
+ collector_instance_id
+ source
```

Sequence 用于：

* 发现缺口；
* 发现重复；
* 判断乱序；
* 识别 Collector 重置。

## 9.5 Series Chunk

高频数据使用独立协议：

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
}
```

Series Chunk 必须包含：

* 通道定义；
* 单位；
* 时间范围；
* 样本数；
* 采样率；
* 缺失样本；
* 时钟来源；
* 校验和；
* 压缩格式；
* Schema 版本。

## 9.6 Schema Registry

每个 Stream 必须声明：

```text
Stream 名称
Schema 版本
记录类型
Payload 类型
单位
默认隐私等级
默认保留等级
事件时间规则
允许迟到范围
聚合方法
来源优先级规则
支持的处理器
```

兼容规则：

1. 新增可选字段兼容；
2. 新增必填字段必须升级版本；
3. 改变单位必须升级版本；
4. 不允许改变已有字段含义；
5. 不兼容修改必须产生新版本；
6. 服务端允许多个版本共存；
7. 处理器声明支持版本范围；
8. Buf 检查 Protobuf Breaking Change。

---

# 10. 设备身份与安全传输

## 10.1 设备密钥

每台设备生成独立 Ed25519 密钥对。

注册流程：

```text
用户授权设备
  → 设备生成密钥对
  → 上传公钥
  → 服务端创建设备身份
  → 下发短期认证材料
  → 后续批次使用设备私钥签名
```

私钥不得离开设备。

## 10.2 批次签名

签名内容至少包括：

```text
device_id
collector_instance_id
batch_id
sequence_start
sequence_end
payload_hash
created_at
nonce
```

服务端验证：

* 设备状态；
* 公钥；
* 签名；
* 时间窗口；
* Nonce；
* Sequence；
* Batch ID；
* Payload Hash。

## 10.3 传输方式

### gRPC 双向流

适合：

* Desktop Agent；
* 实时状态；
* IoT Gateway；
* 持续传感器；
* 在线设备。

### HTTPS 批量上传

适合：

* Android WorkManager；
* 离线补传；
* 文件导入；
* gRPC 降级；
* 受限网络。

### MQTT Gateway

适合：

* 自定义硬件；
* 家庭局域网传感器；
* 已存在的 MQTT 设备。

MQTT Gateway 负责将 MQTT 消息转换成统一 Protobuf 事件，不建立第二套核心模型。

---

# 11. 设备端实现

## 11.1 通用设备端 Core

所有设备端共享以下逻辑：

```text
Event Model
Local WAL
Outbox
Batching
Compression
Signing
Transport
Acknowledgement
Schema Cache
Clock Diagnostics
Configuration
Diagnostics Export
```

## 11.2 Android Agent

模块建议：

```text
app
collector-api
collector-usage
collector-accessibility
collector-device-state
collector-health-connect
collector-location
collector-sensors
local-storage
sync-engine
identity
permission-center
diagnostics
```

首批实现：

* UsageStats；
* AccessibilityService；
* 屏幕状态；
* 电量和充电；
* 网络；
* Health Connect；
* Room 或 SQLCipher；
* WorkManager；
* 实时通道；
* 权限状态机；
* 厂商后台限制诊断。

### 应用使用融合

```text
AccessibilityService
    → 实时前台应用候选

UsageStatsManager
    → 历史回查和漏报校正
```

两个来源都保留来源信息，由服务端或设备端融合处理，不直接覆盖。

## 11.3 Desktop Agent

使用 Rust Core。

平台 Provider：

```text
Windows Foreground Provider
Windows Idle Provider
Windows Power Provider
Windows Media Provider

macOS Foreground Provider
macOS Idle Provider
macOS Power Provider

X11 Provider
GNOME Provider
KDE Provider
wlroots Provider
```

首批 Windows 数据：

* 前台进程；
* 可选窗口标题；
* 空闲状态；
* 锁定和解锁；
* 电量；
* 电源；
* 网络；
* 媒体播放。

### 本地隐私过滤

必须在写入 Outbox 之前执行：

```text
不记录窗口标题
只记录应用名
忽略指定应用
标题正则替换
忽略隐私浏览
忽略短暂窗口
本地分类后不上传原始标题
```

## 11.4 穿戴设备

接入优先级：

```text
Health Connect
  → Gadgetbridge 或厂商应用已经同步的数据

厂商公开 API

Gadgetbridge 独立桥接

LifeChronicle Wearable Provider

直接 BLE 协议
```

直接 BLE Provider 应实现：

* 设备能力；
* 配对；
* 历史同步；
* 同步游标；
* 固件差异；
* 原始记录；
* 标准 Stream 映射。

## 11.5 Browser Extension

可选采集：

* 当前域名；
* 标签类别；
* 活动与后台状态；
* 页面停留；
* 媒体播放；
* 私密窗口排除。

默认不上传完整 URL 查询参数或页面内容。

---

# 12. 服务端接入语义

## 12.1 接入流程

普通 Event 与 SeriesChunk 复用身份、签名、解压、Schema、Batch 幂等、Sequence
和时钟质量校验，但持久确认边界不同：

```text
接收请求
  → 验证设备身份
  → 验证签名
  → 解压
  → 解析 Protobuf
  → 验证 Schema
  → 检查 Batch 幂等
  → 检查 Event / Chunk 幂等与内容冲突
  → 检查 Sequence
  → 检测时钟偏差
  ├─ 普通 Event
  │    → 提交 lc.raw.events.v1
  │    → Kafka durable ACK
  │    → 持久化逐项终态和恢复证据
  │    → 返回 ACCEPTED_TO_LOG
  └─ SeriesChunk
       → 幂等写入对象存储并持久记录协调状态
       → 提交 lc.raw.series-metadata.v1
       → Kafka metadata durable ACK
       → 持久化双持久证据和逐项终态
       → 返回 ACCEPTED_TO_LOG
```

普通 Event 只有取得 Kafka 持久确认并将逐项终态证据可靠落库后才能返回
`ACCEPTED_TO_LOG`。SeriesChunk 必须同时取得对象、Kafka metadata 和终态证据的
持久确认；任一步失败、超时或结果未知均保持 `RETRYABLE`。同 ID、同内容只有在能
证明对应 Event 或 Series 已满足相同持久边界并补齐终态记录时才能返回可靠
`DUPLICATE`；同 ID、不同内容必须永久冲突。

## 12.2 接收状态

进入服务端后的主要生命周期观察点为：

```text
ACCEPTED_TO_LOG
archived_to_lake
normalized
processed
```

设备清理 Outbox 只依赖下列可靠终态之一：

```text
ACCEPTED_TO_LOG
DUPLICATE（同 ID、同内容且有对应持久证据）
```

`REJECTED_PERMANENT` 移入本地隔离队列而不是重试；`RETRYABLE` 保留原条目并使用
原 `batch_id`、nonce、`compressed_items`、hash 和 signature 重试。清理不等待
所有派生处理完成。

## 12.3 幂等层级

```text
设备端：event_id / chunk_id
接入端：batch_id
Kafka：幂等 Producer
Flink：Checkpoint 与状态去重
ClickHouse：业务去重键
Iceberg：Source Offset 和 Commit
Temporal：workflow_id
```

Exactly-once 不能仅依赖单一基础设施组件。

---

# 13. 流式处理

## 13.1 规范化

不同平台的应用标识：

```text
Android package name
Windows executable
macOS bundle ID
Linux desktop entry
Browser domain
```

统一转换为：

```text
application_id
display_name
platform
category
publisher
canonical_identifier
```

原始标识仍保存在原始层。

## 13.2 应用会话

输入：

```text
10:00 VS Code
10:25 Chrome
10:40 VS Code
11:00 Idle
```

输出：

```text
10:00—10:25 VS Code
10:25—10:40 Chrome
10:40—11:00 VS Code
11:00—... Idle
```

会话终止条件：

* 新前台应用；
* 屏幕关闭；
* 用户 Idle；
* 设备关机；
* Collector 明确结束；
* Heartbeat 超时；
* 数据质量规则关闭异常会话。

## 13.3 迟到数据

迟到程度分为：

### 实时允许迟到范围内

由 Flink 更新状态和输出。

### 超出实时范围

进入专用 Late Event Topic，并启动 Temporal Range Replay。

### 大规模历史导入

不通过实时 Watermark 强行处理，直接进入独立 Backfill 流程。

## 13.4 统计

首批指标：

```text
应用使用时长
应用类别时长
屏幕开启时长
空闲时长
在线时长
步数
距离
睡眠时长
睡眠阶段时长
心率 min/max/mean/count
运动时长
地点停留时长
```

统计粒度：

```text
1分钟
5分钟
1小时
1天
1周
1月
```

不同 Stream 按注册的 Metric Definition 选择适用粒度。

---

# 14. 数据存储与保留

## 14.1 PostgreSQL

只保存控制平面事务数据。

## 14.2 ClickHouse

保存热查询数据。

建议初始保留策略：

| 数据           | 热存储时间 |
| -------------- | ---------: |
| 规范化普通事件 |        2年 |
| 应用和设备会话 |  5年或长期 |
| 秒级心率       |       90天 |
| 分钟健康指标   |        1年 |
| 小时指标       |       长期 |
| 每日指标       |       长期 |
| 精确位置       |   用户配置 |
| 数据质量结果   |        1年 |

## 14.3 Iceberg

保存永久原始档案及历史派生数据。

推荐逻辑分区：

```text
days(observed_at)
bucket(user_id)
stream
```

物理分区由 Iceberg 管理，不暴露给业务 API。

## 14.4 高频数据降采样

例如心率：

```text
原始秒级
  → 1分钟 min/max/mean/count
  → 5分钟统计
  → 1小时统计
  → 每日静息和分布指标
```

例如 IMU：

```text
50Hz 原始三轴
  → 每秒均值、方差、能量
  → 每分钟活动强度
  → 每小时活动特征
  → 每日活动统计
```

用户可删除高频原始数据并保留派生特征。

---

# 15. 数据血缘与质量

## 15.1 数据血缘

每个派生结果记录：

```text
processor_id
processor_version
rule_version
input_streams
input_time_range
input_snapshot
output_schema
created_at
```

## 15.2 数据质量检测

检测内容：

* Sequence 缺口；
* 重复批次；
* 设备时钟偏差；
* 设备长时间未同步；
* 异常会话未结束；
* 睡眠区间重叠；
* 多来源步数冲突；
* 心率异常跳变；
* Chunk 损坏；
* 缺失采样；
* 来源突然变化；
* 权限关闭；
* Collector 崩溃。

数据质量问题不能被静默隐藏。

---

# 16. 查询体系

## 16.1 私有查询 API

```text
Timeline API
Raw Event API
Session API
Device State API
Health API
Location API
Statistics API
Lineage API
Data Quality API
Export API
```

## 16.2 查询数据源

近期数据：

```text
ClickHouse
```

完整历史：

```text
Iceberg
```

控制数据：

```text
Control / Identity API（权威数据位于 PostgreSQL）
```

实时状态缓存：

```text
Valkey
```

Valkey 只作为缓存，不作为事实源。
Query Service 不直接连接 PostgreSQL 或 Iceberg Catalog；它通过版本化
Control/Identity API 读取控制元数据，通过受控 Cold Query Worker 读取 Iceberg。
对 Valkey 的访问仅限独立只读 Key 前缀和 ACL。

## 16.3 时间线

时间线支持多个轨道：

```text
应用轨道
设备轨道
屏幕轨道
位置轨道
健康轨道
运动轨道
媒体轨道
用户标注轨道
```

同一时刻可以存在多个并行区间。

---

# 17. 私有控制台

## 17.1 今日页面

显示：

* 当前设备；
* 当前活动；
* 今日应用使用；
* 屏幕和空闲时间；
* 步数；
* 睡眠；
* 心率摘要；
* 最后同步；
* 数据质量警告。

## 17.2 时间线

支持：

* 按天浏览；
* 多轨道；
* 设备筛选；
* Stream 筛选；
* 原始与派生切换；
* 数据来源；
* 处理器版本；
* 手动标注。

## 17.3 设备管理

显示：

* 设备身份；
* 公钥；
* Collector；
* 最后同步；
* Sequence；
* 时钟偏差；
* 权限；
* 撤销和密钥轮换。

## 17.4 数据管理

支持：

* 查询原始事件；
* 查看 Protobuf 或 JSON 表示；
* 导出；
* 创建修正；
* 逻辑删除；
* 创建重放任务；
* 查看血缘。

## 17.5 数据质量

显示：

* 数据缺口；
* 来源冲突；
* 权限失效；
* 同步延迟；
* 设备异常；
* 可执行修复操作。

---

# 18. 公开展示窗口

## 18.1 公开数据流程

```text
私有原始数据
  → 私有派生数据
  → OPA 策略决策
  → Projection Transformer
  → 时间延迟
  → 模糊化
  → Public Snapshot
  → Public API
  → CDN
  → 公开页面
```

## 18.2 物理隔离

Public API：

* 无权读取 Kafka 原始 Topic；
* 无权读取 Iceberg Bronze；
* 无权读取 ClickHouse 私有表；
* 无权读取 PostgreSQL 设备密钥；
* 只能读取 Public Snapshot Store。

## 18.3 隐私转换

支持：

```text
删除字段
类别替换
时间延迟
数值取整
时间模糊
地点降精度
随机扰动
最小样本阈值
更新频率限制
时间段过滤
紧急暂停
```

示例：

```text
com.microsoft.VSCode
  → 开发

精确经纬度
  → Tokyo

6382 步
  → 约 6500 步

当前活动
  → 延迟 15 分钟
```

## 18.4 首批公开卡片

```text
在线状态
当前活动类别
今日活动分布
今日步数
昨晚睡眠
设备可用状态
最近媒体
自定义文本
```

所有卡片默认关闭。

---

# 19. 插件平台

## 19.1 插件类型

```text
Collector
Transformer
Analyzer
Classifier
Importer
Exporter
Public Card Data Provider
Notification Rule
```

## 19.2 服务端插件

使用 WIT 定义接口，Wasmtime 执行。

Manifest 示例：

```yaml
id: org.example.activity-classifier
version: 1.0.0
api_version: 1

inputs:
  - app.foreground@1

outputs:
  - app.activity.category@1

capabilities:
  - stream.read:app.foreground
  - stream.write:app.activity.category

resources:
  memory_mb: 128
  cpu_ms_per_call: 50
  network: false
  filesystem: false
```

## 19.3 插件安全

插件必须满足：

* 默认无网络；
* 默认无文件系统；
* 无数据库直连；
* 只能读取声明的 Stream；
* 输出经过 Schema 校验；
* 有 CPU、内存和超时限制；
* 插件崩溃不影响主服务；
* 结果可追溯插件版本；
* 升级后可通过 Temporal 回填历史。

## 19.4 设备插件

设备宿主统一提供：

```text
本地 WAL
Outbox
事件 ID
Schema
签名
上传
配置
权限
诊断
```

设备插件不得各自实现独立上传协议。

---

# 20. 部署架构

## 20.1 运行平台

正式环境使用 Kubernetes。

支持：

```text
本地 kind
单节点 k3s
多节点 Kubernetes
云托管 Kubernetes
```

核心组件：

```text
Kafka
Flink
PostgreSQL
ClickHouse
MinIO 或 S3
Iceberg Catalog
Temporal
OPA
Valkey
OpenTelemetry Collector
Prometheus
Loki
Tempo
Grafana
```

## 20.2 部署管理

采用：

```text
Helm
Argo CD
cert-manager
External Secrets
NetworkPolicy
Pod Security
```

环境：

```text
local
development
staging
production
```

所有环境配置进入 Git，敏感凭据通过外部密钥系统管理。

## 20.3 网络区域

```text
edge
control
streaming
processing
storage
private-api
public-api
plugins
observability
```

Public API 所在网络不能连接私有事实存储。

---

# 21. 仓库结构

```text
lifechronicle/
├── proto/
│   ├── events/
│   ├── series/
│   ├── ingestion/
│   ├── query/
│   └── plugins/
│
├── agents/
│   ├── android/
│   ├── desktop/
│   │   ├── core/
│   │   ├── windows/
│   │   ├── macos/
│   │   └── linux/
│   ├── browser/
│   └── iot-gateway/
│
├── services/
│   ├── api-gateway/
│   ├── identity/
│   ├── ingestion/
│   ├── query/
│   ├── cold-query-worker/
│   ├── public-projection/
│   ├── export/
│   └── plugin-host/
│
├── streaming/
│   ├── common/
│   ├── normalization/
│   ├── latest-state/
│   ├── sessions/
│   ├── health/
│   ├── location/
│   ├── aggregation/
│   └── data-quality/
│
├── workflows/
│   ├── replay/
│   ├── deletion/
│   ├── export/
│   ├── retention/
│   ├── backfill/
│   └── migration/
│
├── lakehouse/
│   ├── schemas/
│   ├── iceberg/
│   ├── migrations/
│   └── compaction/
│
├── web/
│   ├── private-console/
│   ├── public-window/
│   └── shared/
│
├── plugins/
│   ├── wit/
│   ├── sdk/
│   └── examples/
│
├── infrastructure/
│   ├── helm/
│   ├── argocd/
│   ├── terraform/
│   ├── kubernetes/
│   └── observability/
│
├── tests/
│   ├── contracts/
│   ├── replay/
│   ├── integration/
│   ├── performance/
│   ├── privacy/
│   ├── chaos/
│   └── disaster-recovery/
│
└── docs/
    ├── architecture/
    ├── adr/
    ├── protocol/
    ├── schemas/
    ├── operations/
    ├── security/
    └── research/
```

---

# 22. 详细实施路线

以下阶段直接构建最终组件，不使用未来必须废弃的核心替代实现。

---

## 阶段 0：项目契约和研究基线

### 目标

确定跨语言、跨设备和跨服务的长期稳定边界。

### 实施内容

1. 初始化仓库；
2. 确定许可证；
3. 建立参考项目研究目录；
4. 建立第三方来源登记；
5. 建立 ADR 目录；
6. 建立 Buf Workspace；
7. 定义 Protobuf 包规范；
8. 定义 Event Envelope；
9. 定义 Series Chunk；
10. 定义 Batch；
11. 定义 Acknowledgement；
12. 定义错误码；
13. 定义 Stream Registry；
14. 定义 Metric Registry；
15. 定义隐私等级；
16. 定义保留等级；
17. 定义事件时间规则；
18. 定义兼容策略；
19. 建立多语言代码生成；
20. 建立 Contract Test。

完成状态：上述阶段 0 内容已于 2026-07-27 进入本地 bootstrap 基线，并由统一阶段门
验收；远程 CI 与服务端分支保护的生效边界单独记录，不作为本地执行结果。

### 首批 ADR

```text
ADR-001 原始事件不可变
ADR-002 普通事件和序列块分离
ADR-003 设备端使用 WAL 和 Outbox
ADR-004 Kafka 作为事件主干
ADR-005 Flink 负责事件时间处理
ADR-006 Iceberg 保存永久档案
ADR-007 ClickHouse 负责热查询
ADR-008 PostgreSQL 只保存事务型控制元数据，接入幂等/ACK 使用独立数据库
ADR-009 Temporal 负责长期工作流
ADR-010 Public API 只读取公开快照
ADR-011 OPA 管理授权和隐私决策
ADR-012 Wasm 作为第三方服务端插件边界
ADR-013 阶段 0 机器契约与工具使用独立顶层边界
```

### 验收条件

* Go、Rust、Kotlin、Java和TypeScript可由同一 Proto 生成类型；
* Breaking Change 检查有效；
* 同一测试事件能被所有语言解析；
* 所有时间字段语义明确；
* 普通事件与序列数据边界明确；
* 新增普通 Stream 不需要修改核心事件信封。

---

## 阶段 1：基础设施平台

### 目标

搭建完整目标运行环境。

### 实施内容

1. 创建 kind 或 k3s；
2. 部署 Kafka；
3. 部署 Flink Operator；
4. 部署 PostgreSQL；
5. 部署 ClickHouse；
6. 部署 MinIO；
7. 配置 Iceberg Catalog；
8. 部署 Temporal；
9. 部署 OPA；
10. 部署 Valkey；
11. 部署 OpenTelemetry Collector；
12. 部署 Prometheus、Loki、Tempo和Grafana；
13. 配置 cert-manager；
14. 配置 NetworkPolicy；
15. 创建 Helm Charts；
16. 配置 Argo CD；
17. 配置开发和生产 Values；
18. 配置持久卷；
19. 配置基础备份；
20. 编写环境重建脚本。

### 验收条件

* 全部组件可由 GitOps 部署；
* 删除集群后可从配置重建；
* Kafka、PostgreSQL、ClickHouse和MinIO具有持久化；
* Flink Checkpoint 可写入对象存储；
* Temporal Workflow 可在 Worker 重启后继续；
* 一条 Trace 能跨越至少两个服务；
* Public Namespace 无法访问私有数据库。

---

## 阶段 2：身份、安全和控制平面

### 实施内容

1. 实现 OIDC 登录；
2. 创建 Users；
3. 创建设备模型；
4. 创建 Collector Instance；
5. 实现设备公钥注册；
6. 实现设备密钥轮换；
7. 实现短期设备 Token；
8. 实现批次签名；
9. 实现 Nonce；
10. 实现重放防护；
11. 实现 Scope；
12. 实现 OPA API 策略；
13. 实现设备撤销；
14. 实现 Share Token；
15. 实现审计事件；
16. 实现密钥安全存储；
17. 编写安全契约测试。

### 验收条件

* 被撤销设备无法上传；
* 篡改的批次无法通过验证；
* 同一批次不能无限重放；
* 设备只能写入自己的 Scope；
* Public Token 无法访问私有 API；
* 敏感管理操作全部产生审计事件。

---

## 阶段 3：事件接入和原始归档

### 实施内容

1. 实现 Go Ingestion Service；
2. 实现 gRPC 接入；
3. 实现 HTTPS 批量接入；
4. 实现 zstd；
5. 实现 Protobuf 校验；
6. 实现 Schema Registry 校验；
7. 实现 Batch ID 幂等；
8. 实现 Event ID 幂等；
9. 实现 Sequence 检查；
10. 实现时钟偏差检测；
11. 实现 Kafka Producer；
12. 初始化 Topic；
13. 实现 Dead Letter Topic；
14. 实现 Iceberg Bronze Sink；
15. 实现接入指标；
16. 实现限流；
17. 实现逐项确认；
18. 实现批次追踪；
19. 进行负载测试；
20. 进行 Kafka 故障测试。

### 验收条件

* 单批支持至少10000个普通事件；
* 同一批次重试不会重复；
* 同一 Event ID 不重复归档；
* Kafka 故障时不会错误确认；
* Iceberg Sink 恢复后可以继续；
* 30天历史补传正常；
* 事件可从批次追踪到 Iceberg 文件。

---

## 阶段 4：流式处理基础

### 实施内容

1. Flink 通用反序列化；
2. Event Time；
3. Watermark；
4. 状态去重；
5. 规范化；
6. Latest State；
7. Application Session；
8. Idle Session；
9. Screen Session；
10. Presence Session；
11. 迟到事件旁路；
12. 数据质量输出；
13. ClickHouse Sink；
14. Iceberg Silver Sink；
15. Checkpoint；
16. Savepoint；
17. Job 升级流程；
18. 确定性回放测试。

### 验收条件

* 乱序事件生成正确会话；
* Job 重启后状态不丢；
* Job 升级可由 Savepoint 恢复；
* 相同输入重复回放结果一致；
* 超时迟到事件进入专用流；
* 派生数据包含处理器版本。

---

## 阶段 5：Temporal 历史工作流

### 实施内容

1. Range Replay；
2. Aggregate Rebuild；
3. Account Export；
4. Data Deletion；
5. Retention；
6. Application Reclassification；
7. Plugin Backfill；
8. Historical Import；
9. Backup Verification；
10. 工作流状态页面；
11. 幂等 Activity；
12. 补偿逻辑；
13. 人工审批点；
14. 输出版本切换。

### 验收条件

* Worker 重启后 Workflow 继续；
* 新处理器可输出到隔离版本；
* 重放不影响当前线上结果；
* 切换失败可回滚；
* 删除覆盖所有存储层；
* 长时间导出任务可恢复执行。

---

## 阶段 6：查询与私有控制台

### 实施内容

1. Query Service；
2. ClickHouse 查询适配；
3. Iceberg 历史查询规划；
4. Cursor Pagination；
5. Timeline API；
6. Application API；
7. Device API；
8. Health API；
9. Statistics API；
10. Lineage API；
11. Data Quality API；
12. 私有 Web；
13. 今日页面；
14. 多轨道时间线；
15. 原始事件查看；
16. 设备管理；
17. 处理版本比较；
18. 导出入口。

### 验收条件

* 能查询一天、一个月和一年；
* 时间线支持设备和 Stream 筛选；
* 查询结果显示来源；
* 统计能追溯输入范围；
* 冷数据查询不阻塞实时 API；
* 查询有资源和超时限制。

---

## 阶段 7：Desktop Agent

### 实施内容

1. Rust Core；
2. SQLite 或 SQLCipher；
3. Local WAL；
4. Outbox；
5. Batch Builder；
6. Protobuf；
7. zstd；
8. Ed25519；
9. gRPC；
10. HTTPS 降级；
11. Windows 前台应用；
12. Windows Idle；
13. Windows 电源和网络；
14. 本地隐私规则；
15. 自动更新；
16. 诊断导出；
17. macOS Provider；
18. Linux Provider。

### 验收条件

* 每个平台持续运行7天；
* 断网7天后补传；
* 服务不可用不影响采集；
* 本地数据库损坏可以隔离和恢复；
* 隐私过滤在写入 Outbox 前完成；
* 单个 Provider 异常不影响其他 Collector。

---

## 阶段 8：Android Agent

### 实施内容

1. Kotlin 多模块；
2. Room 或 SQLCipher；
3. Outbox；
4. WorkManager；
5. 实时通道；
6. UsageStats；
7. AccessibilityService；
8. 屏幕、电量和网络；
9. Health Connect；
10. 权限状态机；
11. 厂商后台限制诊断；
12. 来源融合；
13. Changes Token；
14. 健康增量同步；
15. 更新和删除；
16. 设备重启恢复；
17. 诊断导出；
18. 自动更新。

### 验收条件

* Accessibility 和 UsageStats 可以互相校正；
* Health Connect 重复同步不重复；
* 多来源步数不直接相加；
* 设备重启后继续同步；
* 后台限制解除后能补传；
* 权限变化形成明确质量状态。

---

## 阶段 9：健康和高频序列

### 实施内容

1. 通用 Series Protocol；
2. 秒级心率；
3. RR 间期；
4. GPS；
5. IMU；
6. Chunk Upload；
7. Checksum；
8. Iceberg Bronze；
9. Parquet 转换；
10. ClickHouse 热采样；
11. 分钟聚合；
12. 小时聚合；
13. Downsampling；
14. TTL；
15. 缺失采样；
16. 时钟校正；
17. 范围查询；
18. 图表。

### 验收条件

* 可处理一天50Hz三轴IMU；
* 高频样本不进入普通 Event Topic；
* 重复 Chunk 不重复归档；
* 损坏 Chunk 能检测；
* 原始序列删除后派生特征保留；
* 只读取时间范围对应的 Parquet 数据。

---

## 阶段 10：公开投影系统

### 实施内容

1. OPA 隐私策略；
2. Projection Service；
3. Delay Queue；
4. Public Snapshot Store；
5. Public API；
6. CDN；
7. Card Schema；
8. Share Token；
9. 紧急暂停；
10. 访问审计；
11. 模糊化；
12. 公开页面；
13. 隔离安全测试。

### 验收条件

* Public API 无法访问私有事实库；
* 新 Stream 不自动公开；
* 延迟和取整可自动测试；
* 分享撤销立即生效；
* Snapshot 不包含未声明字段；
* 精确位置不能由组合字段恢复。

---

## 阶段 11：插件平台

### 实施内容

1. WIT 接口；
2. Wasmtime Host；
3. Capability Broker；
4. Manifest 校验；
5. 插件签名；
6. 内存限制；
7. CPU 限制；
8. 超时；
9. 网络默认关闭；
10. SDK；
11. 示例 Transformer；
12. 示例 Analyzer；
13. 示例 Exporter；
14. 插件回填 Workflow；
15. 版本迁移；
16. 前端卡片插件。

### 验收条件

* 插件不能读取未授权 Stream；
* 插件不能直接访问宿主文件；
* 插件崩溃不影响主服务；
* 插件升级可重放历史；
* 输出经过 Schema 校验；
* 结果能追踪插件版本。

---

## 阶段 12：位置、穿戴设备和外部数据源

### 实施内容

1. Health Connect 穿戴数据完善；
2. Gadgetbridge Bridge；
3. Wearable Provider API；
4. 厂商公开 API；
5. BLE Provider；
6. Location Collector；
7. Visit Detector；
8. Trip Detector；
9. Geofence；
10. OwnTracks Import；
11. GPX Import；
12. GeoJSON Import；
13. Traccar Adapter；
14. 智能家居 Gateway；
15. MQTT Gateway。

### 验收条件

* 同一穿戴数据不会因多个来源重复；
* 位置原始点与地点访问分离；
* 行程可由原始位置重新生成；
* 导入数据保留来源；
* BLE Provider 不能绕过宿主同步协议。

---

# 23. 测试体系

## 23.1 契约测试

覆盖：

```text
Protobuf Compatibility
Schema Compatibility
Error Codes
Kafka Topic Contract
ClickHouse Schema
Iceberg Schema
Plugin WIT
Public Card Schema
```

## 23.2 回放数据集

```text
normal-day
offline-month
duplicate-batches
out-of-order
clock-forward
clock-backward
timezone-change
dst-change
multiple-step-sources
sleep-overlap
collector-reset
late-health-data
plugin-version-change
```

## 23.3 故障注入

测试：

* Kafka Broker 故障；
* Flink TaskManager 故障；
* PostgreSQL 故障；
* ClickHouse 节点故障；
* 对象存储延迟；
* Temporal Worker 故障；
* 网络分区；
* 重复投递；
* 磁盘不足；
* Agent 突然退出；
* 批次部分损坏。

## 23.4 隐私测试

自动验证：

* Public API 无私有连接；
* 禁用卡片后字段消失；
* 延迟规则有效；
* 插件权限不可绕过；
* 日志不包含敏感字段；
* Trace 不包含原始 Payload；
* 删除 Workflow 覆盖全部存储层；
* 新 Stream 默认为私有。

## 23.5 长时间测试

至少执行：

* Desktop Agent 连续运行30天；
* Android Agent 连续运行30天；
* 服务端连续运行90天；
* 7天断网恢复；
* 90天历史补传；
* 1亿普通事件测试；
* 高频序列批量测试；
* 完整备份恢复；
* 处理器全量回放。

---

# 24. 安全与隐私

## 24.1 凭据类型

```text
用户会话凭据
设备上传凭据
服务间身份
只读 API Token
分享 Token
管理 Token
插件 Capability
```

不同凭据不得混用。

## 24.2 数据删除

删除 Workflow 必须覆盖：

```text
Kafka 保留范围内的数据标记
ClickHouse
Iceberg
对象存储
Valkey
统计
索引
公开快照
导出缓存
备份生命周期
```

不可立即物理删除的备份必须明确显示预计彻底清除时间。

## 24.3 数据导出

支持：

```text
JSON Lines
CSV
Parquet
GeoJSON
GPX
原始序列块
完整账户归档
```

导出必须保留：

* Schema；
* 来源；
* 设备；
* 时间语义；
* 单位；
* 数据血缘；
* 修正关系。

---

# 25. 备份与灾难恢复

## 25.1 备份范围

* PostgreSQL；
* Kafka 关键配置和必要日志；
* ClickHouse；
* Iceberg Catalog；
* 对象存储；
* Temporal；
* OPA 策略；
* Kubernetes 配置；
* 密钥系统；
* 插件包。

## 25.2 恢复目标

应能够：

```text
重建 Kubernetes 集群
恢复控制平面
恢复原始数据湖
重新建立 Kafka 数据流
恢复或重建 ClickHouse
恢复 Temporal
从 Iceberg 重建派生数据
重新生成公开快照
```

## 25.3 验收原则

备份成功不以“文件存在”为标准，而以“能够在空环境恢复”为标准。

---

# 26. MVP 定义

MVP 不表示使用简化技术栈，而表示在最终架构上完成最小业务纵向切片。

MVP 包含：

```text
Protobuf 契约
设备身份
本地 WAL 和 Outbox
签名批次
Kafka
Flink
PostgreSQL 控制平面
ClickHouse
Iceberg
Temporal 基础工作流
OPA 基础策略
OpenTelemetry
Windows Agent
Android Agent
前台应用
屏幕和空闲状态
应用会话
私有时间线
```

MVP 暂时可以只支持少量 Stream，但所有数据必须走最终数据通路。公开投影、
Public API 和公开窗口属于阶段 10；在该阶段完成前，MVP 的全部数据保持私有。

---

# 27. v1 完成标准

v1 在 MVP 基础上增加：

* Health Connect；
* 步数；
* 睡眠；
* 心率；
* 秒级健康序列；
* 小时和每日统计；
* 完整公开窗口；
* 数据导入导出；
* 数据删除；
* 插件平台；
* 备份和灾难恢复；
* Android 和 Desktop 长期稳定运行；
* 完整数据血缘；
* 隐私自动测试。

---

# 28. MVP 首批实施任务

建议最先完成以下任务：

```text
1. 初始化 lifechronicle Monorepo
2. 创建 Buf Workspace
3. 定义 Event Envelope v1
4. 定义 Series Chunk v1
5. 定义 Batch 和 Acknowledgement
6. 配置多语言代码生成
7. 添加 Breaking Change 检查
8. 创建首批 Stream Definition
9. 创建首批 ADR
10. 部署本地 Kubernetes
11. 部署 Kafka
12. 部署 PostgreSQL
13. 部署 ClickHouse
14. 部署 MinIO 和 Iceberg
15. 部署 Flink
16. 部署 Temporal
17. 部署 OPA
18. 部署 OpenTelemetry
19. 实现 Device Identity
20. 实现 Ingestion Service
21. 实现 Kafka 原始事件写入
22. 实现 Iceberg Bronze Sink
23. 实现模拟 Agent
24. 实现 Flink Normalization
25. 实现 Application Session
26. 实现 ClickHouse 查询
27. 实现私有时间线
28. 实现 Windows Agent
29. 实现 Android Agent
```

在完成模拟 Agent、接入、归档、流处理和查询闭环前，不应优先实现复杂 UI 和大量 Collector。
公开状态投影属于阶段 10，须在 MVP 私有闭环和公开隔离门禁完成后实施。

---

# 29. 最终架构结论

LifeChronicle 的核心数据路径为：

```text
设备数据
  → 本地不可丢失 WAL
  → Outbox
  → Protobuf
  → zstd
  → 设备签名
  → gRPC 或 HTTPS
  → Kafka 持久事件日志
  → Iceberg 永久原始档案
  → Flink 事件时间处理
  → ClickHouse 热查询
  → Temporal 历史回放
  → OPA 隐私决策
  → 独立公开快照
```

新增普通数据类型时，主要增加：

```text
一个 Protobuf Payload
一个 Stream Definition
一个 Collector
一个 Flink Normalizer
一个 ClickHouse Projection
一个查询接口
一个展示组件
```

新增高频数据类型时，主要增加：

```text
一个 Series Schema
一个采集和分块器
一个 Iceberg 表
一个序列解码器
一个特征处理器
一个降采样策略
一个查询和展示组件
```

新增算法时，主要增加：

```text
一个 Flink Job 或 Wasm 插件
一个处理器版本
一套血缘信息
一个 Temporal Backfill Workflow
一个输出版本切换过程
```

LifeChronicle 的基础架构必须长期保证：

```text
数据不会因网络异常丢失
重复上传不会产生重复事实
离线补传保持真实时间
来源始终可以追踪
派生算法可以重放
统计结果可以重建
公开窗口不能越权读取
插件不能越权访问
高频数据不会拖垮普通事件系统
系统能够完整备份和恢复
```

当这些基础约束成立后，新增设备、健康指标、环境传感器、算法或展示方式都不需要重新设计核心数据通路。
