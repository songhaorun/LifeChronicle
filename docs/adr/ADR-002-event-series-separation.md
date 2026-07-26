---
adr: ADR-002
title: 普通 Event 与高频 Series 分离
status: accepted
date: 2026-07-27
owners:
  - architecture
reviewers:
  - bootstrap-maintainer
supersedes: []
superseded_by: null
related:
  - ../architecture/overall-architecture.md
  - ../contract/project-contract.md
  - ../protocol/event-stream-spec.md
---

# ADR-002：普通 Event 与高频 Series 分离

## 状态

Accepted。协议边界在阶段 0 冻结；生产级 Series 接入属于阶段 9。

## 上下文

前台应用变化、锁屏和步数修正适合独立事件；50 Hz IMU、RR 间期和连续 GPS 若展开
为数百万个 Event，会放大签名、Kafka、对象数、状态和查询成本。反过来，把所有事实
塞入二进制 Chunk 会失去普通事件的清晰幂等、Schema 和流处理语义。

## 决策

- 普通事实使用 `EventEnvelope`；连续或高频样本使用 `SeriesChunk`。
- `SeriesChunk` 以稳定 `chunk_id`、Channel/Clock 元数据、版本化未压缩载荷布局、
  zstd 压缩以及 `LCS1/LCR1` 内容 framing 表达。
- 普通 Event 进入 `lc.raw.events.v1`；Series 二进制进入租户隔离对象前缀，
  `RawSeriesRecord` metadata 进入 `lc.raw.series-metadata.v1`。
- Series 不得展开进入普通 Event Topic；普通 Event 也不得伪装成单样本 Chunk 以
  绕过 Event 契约。
- 普通 Event 在 Kafka durable ACK 且逐项终态证据落库后才能确认；Series 还必须
  先持久对象、再持久 metadata，最后持久逐项终态证据。
- Stream Registry 明确记录类型、Payload/Channel、时间、单位、隐私和保留规则。

## 备选方案

1. **所有样本都是 Event。** 简化类型但成本和小文件问题不可接受，拒绝。
2. **所有记录都是 Chunk。** 破坏普通事实的独立 ID、修正、会话和流处理，拒绝。
3. **仅按载荷大小动态选择。** 同一 Stream 语义会随运行时变化，无法稳定消费，拒绝。

## 后果

正面后果：

- 普通事件保持可读、可分区和易于状态处理；
- 高频数据获得高压缩率、顺序局部性和列式范围读取；
- 两类数据可以独立扩容、保留和故障隔离。

负面后果：

- Agent、Ingestion、Registry、归档和查询必须维护两条明确通路；
- 混合 Batch 必须逐 Item 判定 ACK；
- Series 存在对象已写而 metadata 未确认的故障窗口，需要协调状态、对账和安全期 GC。

安全与隐私影响：对象 Key 不得含敏感业务文本；Series 前缀和 metadata Topic 使用
独立最小权限；普通与序列通路都执行相同身份、Scope 和隐私等级校验。

## 迁移

1. 在 Registry 中把每个既有 Stream 固定为 Event 或 Series；
2. 对误建为 Event 的高频数据保留原记录，并通过版本化迁移任务生成新 Chunk；
3. 新旧 Consumer 在限定窗口双读并按稳定来源 ID 核对；
4. 完成对象/metadata 一致性和查询结果验证后退役旧 Stream，不原地改变其含义。

## 回滚

Series 接入异常时关闭对应 Stream 的 Series 写入并保留 Agent Outbox、对象 staging 和
协调记录；不得把 Chunk 临时改投普通 Topic。恢复旧 Consumer 或对象布局时使用原
metadata 和内容摘要重放。

## 测试

- `ES-C010`：普通 Event Topic 中不存在 Series；
- `ES-C015`：对象、metadata 或终态证据任一步未持久时不得清理 Outbox；
- `ES-C018`：五语言 `LCS1/LCR1`、对象 Key 和 `RawSeriesRecord` 引用一致；
- `E9-01/E9-02/E9-03/E9-08`：Chunk round-trip、故障窗口、归档和 50 Hz 压力；
- `ARC-006/ARC-007`：双持久确认和两条通路隔离。

## 退出条件

- Event、Series、Batch、ACK 和 `RawSeriesRecord` Schema 可由五种语言生成；
- Registry 能拒绝 RecordKind、Payload/Channel 或 Topic 不匹配；
- 对象成功/Kafka 失败、Kafka ACK 未知、终态证据落库前崩溃和 GC 竞争均有测试；
- Event Topic 扫描证明零 Series，Series metadata 引用的对象均存在且摘要匹配；
- 阶段 9 前的 Ingestion 对未启用 Series 失败关闭。
