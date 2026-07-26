---
adr: ADR-004
title: Kafka 作为服务端近期持久事件主干
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

# ADR-004：Kafka 作为服务端近期持久事件主干

## 状态

Accepted。具体分区数和保留期由版本化 Topic 清单和容量评审决定。

## 上下文

接入、实时处理、归档和回放需要一个具备分区顺序、持久确认、消费位点和背压的近期
日志。同步调用每个下游会把 ACK 绑定到最慢组件，数据库队列表又难以提供可扩展的
多 Consumer 回放。Kafka 能承担近期事件主干，但不能被误写成永久档案或跨所有存储
的 exactly-once 保证。

## 决策

- 普通 Event 经完整校验后写入登记的 raw Topic；Kafka `acks=all` 成功且逐项终态
  证据可靠落库后，Ingestion 才能返回可清理 ACK。
- Series 先写对象，再将 `RawSeriesRecord` metadata 写入 Kafka，并持久逐项终态
  证据；Kafka ACK 对 Series 是必要但不充分条件。
- Producer 启用幂等；production raw Topic 使用复制因子 3、
  `min.insync.replicas=2` 和 `acks=all`。
- Topic、Key、Value、Producer、Consumer、分区、保留和清理策略由版本化清单创建；
  业务服务禁止自动建 Topic。
- Key 使用事件规范的长度前缀编码；Key 或 Value 不兼容变化创建新 Topic 主版本。
- raw Topic 使用 `delete` 而非会覆盖历史的 compaction；实际 partition/offset 由
  Bronze Sink 在 Iceberg 提交时记录，不预写进 Kafka Value。
- Kafka 只保存近期可回放日志；Iceberg 是长期档案，控制事务进入 PostgreSQL。

## 备选方案

1. **PostgreSQL Outbox 作为全局总线。** 适合局部事务发布，不适合作为所有高吞吐
   Consumer 的长期主干，拒绝。
2. **同步 HTTP 串联 Sink。** 放大故障并使接入 ACK 依赖下游，拒绝。
3. **Kafka 作为永久唯一档案。** 保留、Schema 演进和历史列式查询不合适，拒绝。

## 后果

正面后果：

- Ingestion、Flink、Bronze Sink 和运维隔离队列可以独立扩缩与重放；
- 分区内局部顺序和消费 offset 可观测；
- 下游短时故障不会要求 Agent 等待完整派生。

负面后果：

- 端到端不重复仍依赖稳定业务 ID、Flink 状态、Sink 键和 Workflow 幂等；
- Topic Key 变更是状态迁移，不是普通配置修改；
- Kafka 容量、ISR、lag 和磁盘必须持续运维。

安全与隐私影响：Topic ACL 绑定独立 ServiceAccount；Public API、插件和未登记
Consumer 不得读取 raw Topic；日志和监控不得把 Payload 作为标签。

## 迁移

不兼容变更使用新 Topic 主版本：

1. 先部署能读取新旧 Value/Key 的 Consumer；
2. 在受控窗口双写或从 Bronze 回放到新 Topic；
3. 比较 offset 范围、业务 ID、状态和 Sink 结果；
4. 原子切换 Consumer，监控 lag 和拒绝率；
5. 经过保留和回滚窗口后停止旧写入并按清单退役旧 Topic。

## 回滚

保留旧 Topic、旧 Consumer 状态和切换前 Savepoint。新版本异常时停止新写入，将
Producer/Consumer 切回旧 Topic，并从已确认 offset 或 Bronze 重放缺口。不得原地
改回已被新 Consumer 读取的 Key/Value 语义。

## 测试

- `ES-C009`：跨语言 Kafka Key 黄金向量；
- `ES-C011`：Kafka 未确认时不存在普通 Event 的 `ACCEPTED_TO_LOG`；
- `ES-C015/ES-C018`：Series metadata、对象和终态证据的确认边界；
- `INF-C014`：ISR 不满足时 Producer 失败且 Ingestion 不确认；
- `M3-04/M3-06`：幂等 Producer、10k Batch、30 天补传和故障场景；
- `ARC-005/ARC-006`：Event 与 Series 的持久确认证据。

## 退出条件

- 所有 v1 Topic 均有版本化 Key/Value/ACL/保留清单且禁止自动创建；
- Kafka/ISR 故障测试证明零错误成功 ACK；
- 普通 Event 可从 Batch 追踪到 Topic/partition/offset 和 Bronze snapshot；
- Series metadata 不引用缺失或摘要不匹配的对象；
- 新旧 Topic 迁移模板包含 Consumer 先行、双写/回放、状态迁移、回滚和退役步骤。
