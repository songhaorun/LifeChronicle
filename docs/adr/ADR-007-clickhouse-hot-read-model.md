---
adr: ADR-007
title: ClickHouse 只承载可重建的私有热读模型
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
  - ../operations/infrastructure-deployment-spec.md
---

# ADR-007：ClickHouse 只承载可重建的私有热读模型

## 状态

Accepted。Public Snapshot Store 的引擎尚未由本 ADR 选择，必须在 E10-01 前另行
决策。

## 上下文

私有控制台需要按用户、设备、Stream 和时间范围低延迟读取会话、最新状态、质量和
聚合。直接对永久 Iceberg 文件执行所有交互查询会增加延迟和扫描成本；PostgreSQL
又不应承载海量时间线。热读模型必须快速，但不能成为无法重建的原始真相。

## 决策

- ClickHouse 承载近期私有交互查询、热聚合和可重建读模型。
- 表必须声明排序键、分区键、TTL、逻辑 ID/revision 去重键和对应 Sink owner。
- 写入来自登记的 Flink Sink、Workflow 或迁移；业务 API 不直接任意写表。
- Iceberg 是长期档案。ClickHouse 丢失或模型变更时，从固定 Iceberg snapshot 和
  处理版本重建。
- Query Service 只使用只读数据库角色；Public API、OPA 和插件无私有 ClickHouse
  凭据。
- 本 ADR 不决定 Public Snapshot Store。若 E10-01 后续选择 ClickHouse，必须使用
  物理隔离实例/集群、独立用户和独立凭据，不得复用私有热库。

## 备选方案

1. **所有查询直接扫 Iceberg。** 冷历史可行，但不能满足 latest-state 和交互时间线，
   拒绝作为唯一路径。
2. **PostgreSQL 保存时间线。** 会扩张控制事务边界并增加迁移/索引压力，拒绝。
3. **ClickHouse 作为唯一历史。** 破坏开放永久档案和可重建原则，拒绝。

## 后果

正面后果：

- 热范围和聚合查询可独立优化与扩容；
- 表可以随查询模型演进并从 Iceberg 重建；
- 热/冷路由为一年历史查询提供明确边界。

负面后果：

- 存在 Iceberg 与 ClickHouse 的派生延迟和双份存储；
- revision、TTL、merge 和 mutation 需要业务级幂等设计；
- 重建期间需要隔离输出、比较和原子切换。

## 迁移

1. 为新模型创建版本化表和只读/写入角色；
2. 从固定 Iceberg snapshot 回填隔离表；
3. 比较逻辑 ID/revision、聚合、时间边界和查询延迟；
4. Query Service 原子切换读模型版本；
5. 保留旧表至回滚窗口结束，再按已批准 TTL/删除计划退役。

## 回滚

将 Query 路由切回旧表或 Cold Query Worker；停止新 Sink 写入但保留隔离表用于差异
分析。不得以回滚为由回写或覆盖 Iceberg 原始档案。

## 测试

- `M4-05/M4-06`：Sink 重启、重复回放和 revision 集合一致；
- `M6-02/M6-07`：热/冷/跨边界路由、去重、取消和一年查询；
- `INF-C005`：Pod 重启后热库数据仍在；
- `R1-07`：在隔离环境恢复 ClickHouse 后核对指定时间段的查询结果；
- `ARC-013/ARC-018`：冷热隔离和恢复后的业务结果核对。

## 退出条件

- 每张热表都有 owner、Schema、排序/分区、TTL、去重和重建说明；
- 固定 snapshot 重建与在线表的逻辑结果一致；
- Query 使用只读角色且 Public/OPA/插件无法连接私有热库；
- 热请求不扫描 Iceberg，冷查询不会占用实时 Flink 资源；
- Public Snapshot 引擎未决状态没有被部署清单或本 ADR提前裁决。
