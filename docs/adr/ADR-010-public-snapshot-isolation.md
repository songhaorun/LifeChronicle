---
adr: ADR-010
title: Public API 只读取物理隔离的公开快照
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
  - ../planning/development-roadmap.md
---

# ADR-010：Public API 只读取物理隔离的公开快照

## 状态

Accepted for the isolation boundary。Snapshot Store 引擎、Delay Queue 和投影输入
Topic/API 仍必须在 E10-01 前由阶段 ADR/契约选择。

## 上下文

公开卡片面向匿名访客或 Share Token，威胁面与私有控制台不同。若 Public API 按请求
查询 PostgreSQL、ClickHouse、Kafka、Iceberg 或私有对象，应用漏洞、缓存错误或查询
组合可能暴露完整历史、精确位置和健康数据。字段级过滤本身不能替代物理隔离。

## 决策

- 阶段 3–8 的 MVP 不部署或启用任何读取用户数据的 Public Snapshot、路由或卡片；
  公开能力只在阶段 10 门禁通过后启用。
- Projection Pipeline 是私有数据平面到公开平面的唯一写桥。它只读 E10-01 登记的
  派生输入，经 OPA 决策、字段 allowlist、转换、延迟和最小样本检查后写 Snapshot。
- Projection 只有 Snapshot 写入口，无管理/读回凭据；Public API 只有有效 Snapshot
  只读入口，不能反向调用 Projection。
- `lc-public-api` 与私有事实存储使用独立 Namespace、网络路径、ServiceAccount、
  镜像依赖和凭据；Public API 无私有 PostgreSQL/ClickHouse/Kafka/Catalog/对象路径。
- 暂停和撤销同时覆盖 Projection、Snapshot 可见性、Public API 和 CDN purge。
- Snapshot 使用强 Schema，未知字段失败，不保存原始或未声明字段。

## 备选方案

1. **Public API 直接查询私有读库并在响应时过滤。** 单点过滤失败即可暴露事实，拒绝。
2. **与私有库同实例不同表。** 网络和凭据错误仍可跨表读取，不满足物理隔离，拒绝。
3. **仅依赖 CDN 缓存静态文件。** 不能可靠处理撤销、过期、审计和强 Schema，拒绝。

## 后果

正面后果：

- Public 漏洞的最大可见范围受已发布快照约束；
- 私有 Schema 演进和存储驱动不会进入 Public 镜像；
- 公开输出可重放、延迟、暂停和撤销。

负面后果：

- 公开数据存在预计算延迟和重复存储；
- Projection、Delay、Snapshot、API 和 CDN 的撤销传播需要端到端运维；
- 组合隐私攻击仍需专门测试，物理隔离不能代替数据最小化。

## 迁移

1. E10-01 冻结 Card/Projection Schema、输入契约、Snapshot 引擎和原子发布协议；
2. 在隔离环境创建 write-only Projection 与 read-only Public 角色；
3. 从固定派生输入生成候选 Snapshot，并做字段/组合隐私差异检查；
4. 运行正反向网络、凭据、镜像和缓存测试；
5. 门禁通过后才创建公共路由，默认所有 Card 关闭。

## 回滚

立即暂停 Projection 和公共入口，撤销 Snapshot 可见性并 purge CDN；保留最小化审计
元数据和故障证据。回滚不得让 Public API 临时读取私有源。修复后从固定输入和规则
版本重新生成 Snapshot。

## 测试

- `INF-C010`：Public Namespace 到所有私有事实存储连接失败；
- `INF-C017`：Projection 登记输入/OPA/Delay/Snapshot 写正向成功，Public 只读成功，
  反向禁路与读写 ACL 失败；
- `E10-01–E10-07`：默认关闭、字段 fuzz、组合隐私、撤销、暂停、负载和威胁模型；
- `ARC-015/ARC-021/ARC-022`：物理隔离、MVP 无公开副本和启用前门禁。

## 退出条件

- Public API 镜像不含私有存储驱动或凭据；
- 从 Public ServiceAccount 到每个私有事实端点的网络和应用连接均失败；
- Projection 无 raw Topic、私有数据库/对象或 Snapshot 读回权限；
- Public API 无 Snapshot 写/管理权限，且只读有效、未撤销、未过期的强 Schema 数据；
- 暂停/撤销在目标时间内覆盖 API、Snapshot 和 CDN，并留下无敏感内容审计。
