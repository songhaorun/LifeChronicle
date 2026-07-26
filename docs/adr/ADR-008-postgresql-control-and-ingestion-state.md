---
adr: ADR-008
title: PostgreSQL 保存控制事务和独立接入幂等 ACK 状态
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

# ADR-008：PostgreSQL 保存控制事务和独立接入幂等 ACK 状态

## 状态

Accepted。控制数据库与接入数据库可以由同一 PostgreSQL 平台承载，但必须保持独立
database、owner、角色、迁移、连接池和备份核对。

## 上下文

用户、设备、密钥、Registry 和审计索引需要事务、约束和迁移。Ingestion 还需要在
服务重启、Valkey 丢失和 Kafka ACK 结果未知后恢复 Batch/Nonce、Item 内容身份和
逐项终态，否则 exact retry 可能重复写入或错误清理 Agent Outbox。

把所有状态放进 Valkey 会使可靠 ACK 证据随缓存丢失；把海量事件 Payload 放进
PostgreSQL 又会破坏控制平面的容量和所有权边界。

## 决策

- PostgreSQL 只保存事务型元数据，不保存原始 Event/Series Payload、时间线或海量
  派生数据。
- 控制数据库保存 users、identities、devices、collector instances、device keys、
  Registry、share tokens、Workflow/审计索引等控制事务。
- 接入数据库使用独立 database/owner/role/migration/pool，保存 Batch ID、Nonce、
  内容摘要、逐项 ACK、Series 对象/metadata 提交状态和恢复协调元数据。
- exact retry 所需记录和允许清理 Outbox 的终态证据在声明窗口内必须可恢复；Valkey
  只作缓存、限流和短期加速，清空不得改变事实或授权。
- Ingestion 只获接入库角色；Control/Identity API 只获控制库角色；OPA、Query、
  Public API 和插件无 PostgreSQL 凭据。
- 使用约束、唯一索引和事务原子区分 exact retry、ID 内容冲突、Nonce 重用和新批次。

## 备选方案

1. **所有幂等状态只存 Valkey。** 缓存清空会丢失 ACK 证据，拒绝。
2. **控制与接入共享同一 Schema/角色。** 增加越权和迁移耦合，拒绝。
3. **把原始事件也写 PostgreSQL。** 与 Kafka/Iceberg 的事实边界和容量目标冲突，
   拒绝。

## 后果

正面后果：

- 身份、Registry、Nonce 和 ACK 状态拥有事务与强约束；
- Ingestion 可在进程或缓存故障后安全恢复 exact retry；
- 数据库凭据和迁移 owner 清晰。

负面后果：

- 需要维护两套逻辑数据库、连接池、迁移和恢复核对；
- Kafka/object/接入库之间仍需显式幂等协调，而非分布式事务幻想；
- ACK 状态保留窗口和清理策略必须按审计、离线和容量要求量化。

安全与隐私影响：接入库不得保存 Payload；数据库连接使用 mTLS/短期凭据或等价控制，
日志不记录 nonce、签名或敏感内容；OPA 明确无数据库 egress。

## 迁移

1. 使用 expand/migrate/contract 创建独立接入 database、owner、表和唯一约束；
2. 在旧幂等存储仍可读时双写或一次性导入未过窗口的 Batch/ACK 状态；
3. 对 exact retry、冲突、Nonce 绑定和 Series 提交状态逐项核对；
4. 切换 Ingestion 角色并证明无法读取控制表；
5. 经过恢复和回滚窗口后撤销旧角色与写路径。

## 回滚

数据库迁移失败时停止新接入写入，保留 Agent Outbox，并回滚应用到兼容旧 Schema 的
版本。不可逆迁移必须从预先验证的 PITR 恢复到新 Namespace；禁止覆盖源实例。Kafka
或对象已经持久的 Item 通过稳定 ID 对账，不返回无证据的成功 ACK。

## 测试

- `ES-C008/ES-C016/ES-C017`：ID 内容冲突、exact retry、Nonce 绑定和 Item 内容身份；
- `ES-C015/ES-C018`：Series 双持久与可恢复 metadata/对象证据；
- `INF-C005/INF-C012`：两类数据库持久和 PITR 核对；
- `INF-C017`：Ingestion 只通专用接入库，OPA/Query/Public 禁路和 ACL 生效；
- `R1-04/R2-01/R2-04`：迁移、约束、签名、防重放和故障路径；
- `ARC-014/ARC-018`：安全失败关闭和恢复证据。

## 退出条件

- 控制库和接入库的 owner、角色、迁移、连接池与备份核对相互独立；
- exact retry 在 Ingestion/Valkey 重启后返回既有或等价逐项 ACK；
- 相同 Nonce 绑定不同 Batch、摘要或签名时原子拒绝；
- 接入库抽查和敏感扫描证明不含原始 Payload；
- PITR 到新 Namespace 后，控制表和接入幂等/ACK 表均通过业务不变量核对。
