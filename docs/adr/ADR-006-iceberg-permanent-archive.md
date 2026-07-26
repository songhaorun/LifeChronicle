---
adr: ADR-006
title: Iceberg 保存永久档案和历史版本
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

# ADR-006：Iceberg 保存永久档案和历史版本

## 状态

Accepted。具体 Catalog 实现、分区演进和文件大小目标由部署与表级 ADR/Schema 决定。

## 上下文

LifeChronicle 需要开放列式格式、Schema/分区演进、固定 snapshot 回放和长期低成本
保存。Kafka 保留窗口和 ClickHouse 热模型都不能承担永久档案；裸 Parquet 目录又缺少
原子表提交、snapshot 和演进元数据。

## 决策

- S3/MinIO + Iceberg + Parquet 是长期数据湖边界。
- Bronze 保存不可变普通原始记录和实际 Kafka topic/partition/offset 范围；提交
  source offset 与 Iceberg snapshot 的进度必须形成可恢复原子边界。
- Series 原始二进制先按 ADR-002 保存；Series Archiver 校验 `RawSeriesRecord` 与
  对象后生成 Iceberg metadata/Parquet。
- Silver/Gold 保存带处理器、规则、输入 snapshot 和 Lineage 的版本化派生结果。
- Cold Query Worker 是交互式私有 API 读取 Iceberg 数据文件的唯一边界；Query
  Service、Public API 和插件不得持有 Catalog/对象凭据。
- Catalog 与对象数据使用一致恢复点；业务 Pod 使用独立表/前缀凭据，禁止 root
  凭据和人工覆盖对象。

## 备选方案

1. **长期只保留 Kafka。** 缺少适合永久列式历史和 snapshot 演进的边界，拒绝。
2. **ClickHouse 同时作为永久真相。** 热查询优化与长期开放档案职责冲突，拒绝。
3. **直接管理 Parquet 目录。** 难以保证原子提交、Schema 演进和可追踪 snapshot，
   拒绝。

## 后果

正面后果：

- 可固定输入 snapshot 进行回放、导出和算法比较；
- 开放 Parquet 格式降低单一引擎锁定；
- ClickHouse 和派生表可从历史重建。

负面后果：

- Catalog、对象、snapshot 和 source offset 的一致恢复更复杂；
- 小文件、compaction、分区裁剪和生命周期需要持续治理；
- 交互式冷查询必须受扫描预算、取消和资源隔离约束。

## 迁移

1. 为源表冻结 Schema、分区、内容摘要和 source offset 映射；
2. 将旧数据写入隔离 Iceberg 表并记录迁移 snapshot；
3. 对比记录数、ID 集合、时间范围、Schema、摘要和查询结果；
4. Consumer 在固定边界切换到新表，旧存储保持只读；
5. 经过恢复演练和回滚窗口后按保留策略退役旧副本。

## 回滚

将读指针切回迁移前 snapshot 或旧表；停止产生新表版本的 Writer。已提交 Iceberg
snapshot 保留用于审计，不手工删除文件。Catalog 损坏时在新 Namespace 恢复与对象
一致的恢复点，再重新开放读写。

## 测试

- `INF-C013`：抽样对象哈希、Parquet Schema 和读取完整性；
- `INF-C016`：Cold Query Worker 最小只读权限、预算、取消和实时资源隔离；
- `M3-05/M3-06`：Kafka offset 与 Bronze snapshot 原子提交和故障恢复；
- `M5-01/M5-02/M5-05`：固定 snapshot 回放、版本切换和隔离恢复；
- `ARC-008/ARC-011/ARC-013/ARC-018`：追踪、回放、冷查询和恢复证据。

## 退出条件

- 能从 Event/Batch 定位实际 source offset、Iceberg snapshot 和文件；
- Sink 在所有声明的提交故障点恢复后不漏不重；
- Catalog 与对象存储可在新 Namespace 恢复到一致点；
- Cold Query Worker 之外的 Query/Public 工作负载无 Catalog 或对象路径；
- compaction、Schema/分区演进、保留和删除均保留 snapshot 与审计证据。
