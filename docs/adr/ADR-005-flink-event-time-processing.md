---
adr: ADR-005
title: Flink 负责 Event Time 有状态流处理
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
  - ../operations/infrastructure-deployment-spec.md
---

# ADR-005：Flink 负责 Event Time 有状态流处理

## 状态

Accepted。每个 Job 的 Watermark、lateness、状态 TTL 和分区键仍由 Stream/处理器
契约单独登记。

## 上下文

设备会长期离线、乱序、时钟回拨或跨时区，接收时间不能替代真实发生时间。应用会话、
Presence、最新状态和聚合需要可恢复的有状态处理、Watermark、idleness 与迟到旁路。
普通请求服务或定时 SQL 无法统一提供这些语义。

## 决策

- Flink 是实时 Event Time 派生的标准运行时；`observed_at` 是业务事件时间，
  `received_at`、`ingested_at` 和 `processed_at` 仅描述处理过程。
- 公共运行库统一反序列化、Registry 快照、Watermark、idleness、Event ID 去重、
  late side output、Lineage 和安全错误隔离。
- 实时去重状态 TTL 不小于
  `realtime_allowed_lateness + max_out_of_order + safety_margin`；超出 TTL 的重复
  继续由 Sink 业务键、Iceberg 索引和回放流程约束。
- 超过实时窗口的数据进入 `lc.processing.late-events.v1`，不得静默丢弃；大规模
  历史导入和超窗重建交给 Temporal Range Replay。
- Checkpoint/Savepoint 写专用对象前缀。Job 升级前创建 Savepoint，状态 Schema 或
  Key 变化必须有兼容或迁移方案。
- Sink 使用稳定逻辑 ID + revision 幂等；Flink 的 checkpoint 不被表述为跨 Kafka、
  ClickHouse、Iceberg 和外部服务的全局 exactly-once。

## 备选方案

1. **按接收时间处理。** 会扭曲离线补传和时钟异常下的业务事实，拒绝。
2. **每个服务自写状态机。** 状态恢复、Watermark 和升级语义会分叉，拒绝。
3. **Temporal 处理每条实时事件。** Workflow history 和调度开销不适合逐条流，拒绝。

## 后果

正面后果：

- 乱序、迟到和空闲分区有统一、可测试的语义；
- 状态可从 Checkpoint/Savepoint 恢复；
- 同一输入快照和处理版本能产生稳定逻辑结果。

负面后果：

- 状态大小、backpressure、checkpoint age 和升级兼容需要专门运维；
- Key、Watermark 和 TTL 决策变成长期兼容承诺；
- 极晚数据需要额外回放工作流，实时结果可能产生新 revision。

## 迁移

1. 固定旧、新 Job 的输入 snapshot、Key、状态 Schema 和输出版本；
2. 能兼容时从 Savepoint 启动新 Job；不兼容时写隔离输出并从 Kafka/Iceberg 回放；
3. 比较逻辑 ID、revision、Lineage、迟到集合和聚合不变量；
4. 审批后原子切换读版本，保留旧输出和 Savepoint 至回滚窗口结束。

## 回滚

停止新 Job，恢复旧镜像和切换前 Savepoint；将读版本切回旧输出。新 Job 已产生的
输出保留为隔离版本，不原地覆盖或删除。若输入 Topic Key 已变更，按 ADR-004 的
Topic 迁移回滚，而不是强行读取不兼容状态。

## 测试

- `ES-C007`：Timestamp 和区间边界；
- `ES-C013`：超窗事件进入 late Topic；
- `ES-C014`：固定输入确定性回放；
- `INF-C006`：Checkpoint 持久且 Job 可恢复；
- `M4-01–M4-06`：Watermark、idleness、会话、Sink、Savepoint 和升级回滚；
- `ARC-009/ARC-010`：事件时间正确性与派生结果确定性。

## 退出条件

- 公共运行库对坏记录、空闲分区、重复、乱序和超窗数据有非静默行为；
- Job 从 Checkpoint 恢复后逻辑 ID/revision 集合不变；
- 升级失败可恢复旧镜像和 Savepoint；
- 处理输出包含输入范围、snapshot、processor/rule 版本和 run ID；
- backpressure、watermark、late events、restart 和 state size 均可观测。
