---
adr: ADR-003
title: Agent 使用 append-only WAL 与事务 Outbox
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

# ADR-003：Agent 使用 append-only WAL 与事务 Outbox

## 状态

Accepted。Desktop 与 Android 的具体数据库和加密方案由阶段 7/8 的实现 ADR 选择。

## 上下文

设备可能离线数天、进程可能在任意写入点被强杀，Provider 权限和网络也会独立失败。
如果 Collector 直接上传或只在内存排队，网络、崩溃和升级会丢失事实；如果重试重新
生成 ID 或 Batch，会造成重复、Nonce 冲突和不可证明的 ACK 清理。

## 决策

- 本地隐私过滤完成后，事实先以稳定 ID 和 Sequence 写入 append-only WAL；本地事务
  成功后才算已采集。
- WAL 索引和 Outbox 状态在同一事务边界内推进。Provider 不得自行实现上传栈或绕过
  Agent Core。
- Batch/Chunk Builder 从持久状态构建发送项。同一 Batch 的 exact retry 必须复用
  `batch_id`、nonce、`compressed_items`、摘要和签名的完全相同字节。
- 只有 `ACCEPTED_TO_LOG`，或服务端证明同 ID、同内容满足相同持久条件的
  `DUPLICATE`，才能清理对应 Outbox 项。
- `RETRYABLE` 保留原发送材料并退避；永久拒绝进入本地隔离队列，不静默删除。
- WAL、Outbox、游标、Schema Cache 和迁移必须支持强杀恢复、损坏尾隔离和可重建索引。

## 备选方案

1. **Provider 直接上传。** 会产生多套身份、重试和 ACK 语义，拒绝。
2. **仅内存队列。** 无法承受离线、重启和 OS 回收，拒绝。
3. **收到 HTTP/gRPC 响应即删除整批。** 忽略逐项状态和不可靠响应，拒绝。

## 后果

正面后果：

- 采集与网络解耦，可可靠补传；
- exact retry 和逐项 ACK 可跨重启保持；
- 单个 Provider 崩溃不会破坏其他采集器或上传状态。

负面后果：

- 本地数据库需要迁移、压缩、容量水位和损坏恢复；
- WAL 与 Outbox 生命周期不同，不能用一个简单“已发送”布尔值表达；
- 私密设备上的本地存储需要平台密钥、访问控制和诊断脱敏。

## 迁移

1. 为既有本地记录补稳定 ID、来源、Sequence 和迁移版本；
2. 以事务方式建立 WAL、索引、Outbox 和隔离队列表；
3. 旧上传器进入只读/排空模式，新 Core 双读并核对待发送集合；
4. 完成断网、强杀和降级传输验证后删除旧上传路径。

## 回滚

应用升级失败时回滚到仍能读取旧 Schema 的前一版本，保留新旧数据库备份和未确认
Outbox。不得通过清空数据库解决迁移失败。无法降级 Schema 时停止上传、继续安全
采集到兼容 WAL，并提供前向修复。

## 测试

- `ES-C004/ES-C016/ES-C017`：签名帧、exact retry、Nonce 绑定和 Item 内容身份；
- `M7-01/M7-02/M7-07`：Desktop 强杀恢复、完整 Batch 重试和长期离线；
- `M8-01/M8-02/M8-07`：Android 进程死亡、受限网络和重试恢复；
- `ARC-004`：Agent 强杀、离线和重试不丢不重；
- 传输降级测试必须证明 gRPC 与 HTTPS 共用相同身份、校验、幂等和 ACK 管线。

## 退出条件

- 任意事务提交点强杀后，已采集事实仍在且未确认项仍可发送；
- 重复启动和索引重建不改变逻辑 WAL/Outbox 集合；
- exact retry 的所有签名字节跨重启完全一致；
- 只有可靠逐项终态清理 Outbox，永久拒绝可在隔离队列中诊断和重试修复；
- 容量水位、最长离线窗口、损坏恢复和加密方案均有阶段 ADR 与自动化证据。
