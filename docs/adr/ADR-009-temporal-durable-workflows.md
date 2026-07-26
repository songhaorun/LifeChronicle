---
adr: ADR-009
title: Temporal 负责长期可靠工作流
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

# ADR-009：Temporal 负责长期可靠工作流

## 状态

Accepted。Workflow/Activity 的具体 Task Queue、retention 和版本策略由对应工作包登记。

## 上下文

历史回放、重建、导入、导出、删除、保留、备份验证、插件回填和处理版本切换可能运行
数分钟到数天，涉及重试、取消、人工审批、补偿和跨 Worker 重启。HTTP 请求或定时
脚本无法可靠保存业务进度；Flink 又适合逐条流处理而非长期人工流程。

## 决策

- Temporal Server 保存 Workflow history；独立 Worker 执行业务 Activity。
- 长期操作使用稳定业务幂等 `workflow_id`，Activity 的外部副作用必须幂等并记录
  进度、输入 snapshot、输出版本和审计关联。
- Workflow 代码遵守确定性 replay；不兼容升级使用 Worker Versioning、兼容分支或
  新 Workflow 类型，不让新代码破坏在途 history。
- API 只创建、查询、取消或审批 Workflow，不在 HTTP 请求内执行大规模扫描或删除。
- 大 Payload 留在对象/Iceberg 等事实存储，Workflow history 只保存引用和有界元数据。
- 每条实时事件的 Watermark、会话和聚合继续由 Flink 处理。

## 备选方案

1. **数据库状态表 + cron。** 可实现简单任务，但重试、取消、补偿和代码 replay
   需要重复建设，拒绝作为统一边界。
2. **长 HTTP 请求。** 客户端断开和服务滚动会丢失进度，拒绝。
3. **Flink 承载所有历史工作。** 不适合人工审批和跨小时副作用，拒绝。

## 后果

正面后果：

- Worker 重启和短暂依赖故障后工作可继续；
- 取消、审批、补偿和进度具有统一可观测模型；
- 回放、删除和备份验证可以复用幂等 Activity 规范。

负面后果：

- Workflow 代码需要严格的 replay 兼容纪律；
- Temporal 持久库、retention 和 Task Queue 成为重要运维对象；
- Activity 必须自行处理下游“结果未知”和幂等，而不能依赖 Temporal 自动保证。

## 迁移

1. 为现有长任务定义稳定 Workflow/Activity 接口和业务幂等键；
2. 将未完成任务导入为带原状态引用的新 Workflow，或让旧执行器排空；
3. 新旧实现并行期间禁止对同一副作用使用不同幂等键；
4. 核对进度、输出版本、审计和取消结果后切换调度入口；
5. 保留旧 Worker 直至所有旧 history 完成或迁移。

## 回滚

停止向新 Task Queue 调度，保留 Temporal Server 和 history，恢复兼容旧 history 的
Worker。错误输出保留为隔离版本并通过补偿或重新回放处理；不得删除 history 来
“修复”replay 错误。

## 测试

- `INF-C007`：Worker 重启后 Workflow 继续并完成；
- `M5-01–M5-06`：幂等 Activity、回放、导出、删除、备份、取消和故障恢复；
- `E11-06`：插件回填、版本比较、切换和回滚；
- `ARC-011/ARC-012/ARC-018`：版本化回放、删除和隔离恢复。

## 退出条件

- 至少一个跨 Worker 重启的持久 Workflow 自动完成且副作用不重复；
- 回放、删除和导出 API 只调度 Workflow，并能查询/取消/审批；
- 不兼容 Worker 升级在 staging 被 replay 测试拒绝或有版本迁移；
- Workflow history 不保存大 Payload、Token 或敏感原值；
- schedule-to-start、task/workflow failure、worker availability 和业务进度可观测。
