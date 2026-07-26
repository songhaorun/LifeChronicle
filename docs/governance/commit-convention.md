# LifeChronicle 提交规范

> - 状态：R0-01 治理基线
> - 生效日期：2026-07-27

## 1. 目标

提交历史必须能说明意图、关联工作、验证证据和兼容影响，并支持可审计发布与回滚。
本规范适用于 commit、分支和 PR 标题。

## 2. 提交标题

格式：

```text
type(scope): summary
```

示例：

```text
docs(governance): establish R0-01 source registry
feat(ingestion): validate canonical batch hash
fix(agent): preserve outbox item on retryable ack
refactor(query): isolate cold plan construction
```

规则：

- `type` 和 `scope` 使用小写 ASCII；
- `summary` 简洁描述结果，不以句号结尾；
- 标题只表达一个意图，不包含 issue 讨论全文；
- 推荐标题不超过 100 个字符；
- breaking change 仍使用正常类型，并在正文中加入 `BREAKING CHANGE:`。

## 3. 类型

| 类型 | 用途 |
| --- | --- |
| `feat` | 新的用户或调用者能力 |
| `fix` | 错误、回归或不变量修复 |
| `docs` | 文档、契约说明或研究记录 |
| `refactor` | 不改变对外行为的结构调整 |
| `test` | 新增或修正测试与 fixture |
| `build` | 构建系统、生成器和依赖构建 |
| `ci` | 自动化检查与发布流水线 |
| `chore` | 其他可追溯维护工作 |
| `revert` | 撤销既有提交 |

依赖升级不得伪装为 `refactor`；schema/数据迁移应在标题或 scope 中明确。

## 4. Scope

Scope 使用稳定模块或治理边界，例如：

```text
agent
android
desktop
ingestion
identity
registry
proto
processing
query
public
plugin
infra
security
governance
research
docs
```

跨多个模块时选择主要契约边界；无法选择通常表示提交范围过大，应拆分。ADR 可使用
`adr` 作为 scope。

## 5. 正文

微小且意图完全清楚的提交可省略正文。其他提交应说明：

- 为什么改变；
- 行为和不变量如何变化；
- 兼容、迁移、回滚和发布顺序；
- 数据、安全、隐私与第三方来源影响；
- 实际验证命令、结果和已知限制。

正文每行建议不超过 100 个字符。不要粘贴 Secret、真实个人数据或大量生成日志。

## 6. Trailers

关联工作和验证使用以下 trailers；有对应信息时必须填写：

```text
Work-Package: R0-01
Refs: #123
Test: git diff --check
Risk: none
Third-Party: RES-013
```

规则：

- `Work-Package`：计划工作包 ID；
- `Refs`：issue、ADR 或外部追踪链接；
- `Test`：实际运行的检查及简洁结果，可重复多行；
- `Risk`：剩余风险或 `none`；不能用它隐藏高风险登记；
- `Third-Party`：`THIRD_PARTY_SOURCES.md` 的稳定 ID。

涉及不兼容变化时在正文末尾使用：

```text
BREAKING CHANGE: describe the old contract, new version, migration and rollback.
```

`Signed-off-by` 当前不是强制项。若未来采用 DCO，必须通过治理变更统一启用，不能在
单个 PR 中临时要求。

## 7. 原子性与生成内容

- 一个提交只处理一个可评审目标；
- 不混入无关格式化、用户配置或工作区输出；
- 生成器、输入、生成结果和对应测试形成同一可追溯变更序列；
- 数据迁移、依赖升级和安全变化不能隐藏在普通重构中；
- 提交之间应尽量保持当前实际存在的检查可运行；
- 不得通过 amend/rebase 重写已发布 tag 的历史。

## 8. 分支与 PR

分支格式：

```text
feat/<issue>-<slug>
fix/<issue>-<slug>
docs/<issue>-<slug>
chore/<issue>-<slug>
adr/<issue>-<slug>
```

PR 标题使用与 commit 相同的 `type(scope): summary` 格式。默认 squash merge，
squash 后的提交消息必须保留工作包、breaking change 和必要来源信息。确需保留多
提交迁移历史时，由维护者在评审记录中说明理由。

## 9. Bootstrap 说明

单维护者模式可以自审普通可逆提交，但提交/PR 仍要记录实际检查和缺失门禁。不得写：

```text
CI passed
phase-0-gate passed
approved by CODEOWNERS
```

除非对应远程检查、门禁和批准真实存在且可验证。尚未实现时明确写：

```text
Test: phase-0-gate 尚未实现，未运行
```

## 10. Revert

Revert 标题使用：

```text
revert(scope): revert <original summary>
```

正文记录原 commit、原因、数据影响、恢复动作以及是否需要新的前向修复。回退代码
不等于回退已写入的数据或线上协议。
