---
adr: ADR-011
title: OPA 负责授权和隐私决策
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

# ADR-011：OPA 负责授权和隐私决策

## 状态

Accepted。OPA 是决策点，不是身份事实源、数据转换器或事实存储。

## 上下文

上传、私有查询、插件 Capability 和公开投影都需要一致、版本化、可测试且默认拒绝的
决策。把规则散落在服务代码中会产生不同的默认值和升级节奏；让策略引擎直接查询
业务数据库又会扩大凭据和网络边界，并使同一输入无法稳定重放。

## 决策

- OPA 执行授权与隐私决策，返回 allow/deny、策略版本和有界决策元数据。
- Policy Bundle 由 Git 通路生成、签名、版本化和测试，通过挂载/推送或唯一允许的
  签名 Bundle 端点分发。
- 调用方按版本化 Schema 提供身份、Scope、资源、Card/Profile 和必要事实摘要；
  OPA 不直连 PostgreSQL、ClickHouse、Kafka、Catalog 或对象存储补充输入。
- OPA 使用独立 ServiceAccount，不注入业务数据库/事实存储凭据，NetworkPolicy
  明确拒绝这些端点。
- 上传、私有查询、插件和公开投影默认 fail-closed；依赖不可用或输入未知时不放行。
- 字段删除、类别替换、取整、扰动、延迟和数据保存由业务组件执行，OPA 不变换事实。

## 备选方案

1. **每个服务硬编码规则。** 默认值和审计版本容易分叉，拒绝。
2. **OPA 直接查询数据库。** 扩大权限、破坏可重放输入和故障边界，拒绝。
3. **策略失败时沿用最近 allow。** 可能在撤销或规则变化后放宽权限，拒绝。

## 后果

正面后果：

- 策略可以单元测试、签名、审计和按版本回放；
- 多个决策点共享默认拒绝和输入 Schema；
- OPA 被攻破时没有直接事实存储凭据。

负面后果：

- 调用方必须准备完整且最小化的决策输入；
- Bundle 发布、缓存和版本传播成为运行依赖；
- OPA 的 allow 不代替 NetworkPolicy、数据库 ACL、Topic ACL 和业务验证。

## 迁移

1. 盘点服务内授权分支并定义版本化 OPA 输入/输出 Schema；
2. 用相同黄金请求比较旧逻辑与新 Bundle，记录有意差异；
3. 先以 shadow decision 观测，再在 fail-closed 模式切换；
4. 删除旧旁路并撤销 OPA 的任何业务数据库凭据；
5. 保留上一签名 Bundle 作为有审计的回滚版本。

## 回滚

策略回归时回滚到上一已签名、仍受支持的 Bundle，不得切换为 fail-open。若 OPA
服务不可用，停止受影响的上传/查询/投影/插件操作并保留原始事实，不使用数据库直查
作为临时旁路。

## 测试

- `R2-05/R2-06`：Scope、撤销、Share Token、失败关闭和审计；
- `INF-C011/INF-C017`：默认拒绝、OPA 到业务端点禁路和调用方正向路径；
- `E10-01/E10-02/E10-06`：公开规则、未知字段、最小样本和隐私攻击；
- `E11-03/E11-07`：插件 Stream/Capability 授权和越权拒绝；
- `ARC-014/ARC-015/ARC-016`：身份、Public 和插件边界。

## 退出条件

- 每个决策点都有版本化输入/输出 Schema、owner、默认值和失败模式；
- Bundle 签名、单元测试和回滚可自动执行，策略版本进入审计；
- OPA Pod 凭据和 egress 扫描证明无法访问业务数据库或事实存储；
- OPA 不执行数据转换，Projection/业务服务对输出执行强 Schema 校验；
- OPA/Bundle 不可用测试证明所有安全决策失败关闭。
