# LifeChronicle 架构决策记录

本目录保存已进入 LifeChronicle 阶段 0 架构基线的长期决策。ADR 解释“为什么这样
设计、哪些边界不可弱化、如何迁移和退出”；字段、Topic、网络和阶段范围仍分别以
对应领域权威规范为准。

## ADR 索引

| ADR | 决策 | 状态 |
| --- | --- | --- |
| [ADR-001](ADR-001-immutable-raw-facts.md) | 原始事实不可变，修正和删除以新记录表达 | accepted |
| [ADR-002](ADR-002-event-series-separation.md) | 普通 Event 与高频 Series 分离 | accepted |
| [ADR-003](ADR-003-agent-wal-outbox.md) | Agent 使用 append-only WAL 与事务 Outbox | accepted |
| [ADR-004](ADR-004-kafka-event-backbone.md) | Kafka 作为服务端近期持久事件主干 | accepted |
| [ADR-005](ADR-005-flink-event-time-processing.md) | Flink 负责 Event Time 有状态流处理 | accepted |
| [ADR-006](ADR-006-iceberg-permanent-archive.md) | Iceberg 保存永久档案和历史版本 | accepted |
| [ADR-007](ADR-007-clickhouse-hot-read-model.md) | ClickHouse 只承载可重建的私有热读模型 | accepted |
| [ADR-008](ADR-008-postgresql-control-and-ingestion-state.md) | PostgreSQL 保存控制事务和独立接入幂等/ACK 状态 | accepted |
| [ADR-009](ADR-009-temporal-durable-workflows.md) | Temporal 负责长期可靠工作流 | accepted |
| [ADR-010](ADR-010-public-snapshot-isolation.md) | Public API 只读物理隔离的公开快照 | accepted |
| [ADR-011](ADR-011-opa-policy-decisions.md) | OPA 负责授权和隐私决策，不执行数据转换 | accepted |
| [ADR-012](ADR-012-wasmtime-plugin-sandbox.md) | Wasmtime Component Model 作为第三方服务端插件边界 | accepted |
| [ADR-013](ADR-013-phase-zero-contract-tooling-boundaries.md) | 阶段 0 机器契约与工具使用独立顶层边界 | accepted |

## 稳定文件结构

ADR 文件名必须匹配：

```text
ADR-[0-9]{3}-[a-z0-9-]+.md
```

每份 ADR 必须以 YAML Front Matter 开始，并按以下顺序且各出现一次：

```text
adr
title
status
date
owners
reviewers
supersedes
superseded_by
related

# ADR-NNN：标题
## 状态
## 上下文
## 决策
## 备选方案
## 后果
## 迁移
## 回滚
## 测试
## 退出条件
```

字段规则：

- `adr` 必须与文件名编号一致；
- `status` 只能是 `proposed`、`accepted`、`deprecated` 或 `superseded`；
- `date` 使用 `YYYY-MM-DD`；
- `owners`、`reviewers`、`supersedes` 和 `related` 必须是 YAML 列表；
- `superseded_by` 使用 ADR 编号或 `null`；
- `related` 只使用仓库内相对路径；
- “测试”和“退出条件”不得为空，也不得以无法验证的“人工确认正常”代替证据。

`adr-lint` 应检查文件名、Front Matter 字段、状态枚举、日期、标题编号、章节完整性和
内部链接。内容检查还应拒绝空章节、重复编号以及未建立双向链接的取代关系。

## 生命周期与修改规则

生命周期为 `proposed → accepted → superseded/deprecated`。已接受 ADR 只允许修正
拼写、链接和不改变结论的澄清；改变结论必须创建新 ADR，并在旧 ADR 的
`superseded_by` 与新 ADR 的 `supersedes` 中建立双向关系。

ADR 接受不代表实现或测试已经完成。“退出条件”描述该决策在对应阶段可被宣称落地
前必须取得的证据；实际完成状态仍由任务清单、CI 和阶段门管理。
