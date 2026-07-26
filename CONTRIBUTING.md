# 参与 LifeChronicle

感谢你愿意帮助 LifeChronicle。项目处理位置、健康、设备活动等高敏感个人数据，
因此贡献不仅要能运行，还必须满足来源、隐私、安全、兼容、恢复和可验证性要求。

## 1. 开始之前

提交代码前请先阅读：

- [项目契约](docs/contract/project-contract.md)
- [总体架构](docs/architecture/overall-architecture.md)
- [事件流规范](docs/protocol/event-stream-spec.md)
- [治理规则](GOVERNANCE.md)
- [提交规范](docs/governance/commit-convention.md)
- [行为准则](CODE_OF_CONDUCT.md)
- [安全政策](SECURITY.md)

安全漏洞不要创建公开 issue，请按 `SECURITY.md` 私下报告。

## 2. 贡献许可证与来源声明

LifeChronicle 自有代码和文档采用 MIT License。提交贡献即表示：

1. 你有权提交该内容；
2. 你同意该贡献在项目 MIT License 下发布；
3. 你保留自己贡献的版权；
4. 你已披露所有第三方来源、生成工具和许可证义务。

项目不要求 CLA。MIT License 不会把第三方内容重新许可。不得通过复制、翻译、
机械改写或 AI 重述规避第三方许可证、著作权、专利、商标、服务条款或
clean-room 边界。

以下内容在登记和批准前不得提交：

- 从参考项目复制或派生的源码、测试、迁移、schema、注释和构建脚本；
- 未核实许可的图片、图标、字体、地图、固件、协议文档、样例数据和生成物；
- Secret、Token、私钥、真实个人数据或可重新识别个人的数据；
- 未固定版本或无法追溯来源的二进制、容器、插件和生成代码。

新增或更新第三方内容时，先更新
[第三方来源登记](THIRD_PARTY_SOURCES.md)，并使用
[登记模板](docs/governance/templates/third-party-source-record.md) 留下审计证据。
研究参考使用
[研究记录模板](docs/governance/templates/reference-research-record.md)；研究行为不等于
批准把上游实现加入产品。

## 3. Issue 与工作包

非微小变更应先创建 issue，或关联计划中的工作包 ID。Issue 至少说明：

- 用户或调用者的问题；
- 范围与非目标；
- 受影响契约、模块和 owner；
- 隐私、安全、数据保留与删除影响；
- 兼容、迁移、回滚和验证方式；
- 外部依赖和第三方来源。

工作包进入开发前应满足项目契约的 Definition of Ready。无法确认授权、数据来源、
删除范围或不可逆迁移恢复点时，先停止并请求维护者裁决。

## 4. 分支与提交

目标集成分支为 `main`。开发分支使用以下格式：

```text
feat/<issue>-<slug>
fix/<issue>-<slug>
docs/<issue>-<slug>
chore/<issue>-<slug>
adr/<issue>-<slug>
```

提交标题使用 `type(scope): summary`。支持的类型、正文、breaking change 和 trailer
规则见 `docs/governance/commit-convention.md`。一个分支和提交序列只处理一个
可评审目标，不混入无关格式化或本地配置。

## 5. 开发原则

- 原始事实只追加；修正、替代和删除使用显式新记录；
- Event 与 Series 使用不同通路；
- Collector 先写本地 WAL/Outbox，可靠 ACK 后才能清理；
- 业务时间使用 `observed_at`，处理时间不能冒充业务时间；
- 新 Stream 默认 `PRIVATE`，新 Card 默认关闭；
- Public API 只读物理隔离的公开快照；
- 授权、验签、公开投影和删除失败时必须失败关闭；
- 派生结果携带版本与 lineage，并可从固定输入重放；
- 日志、trace、metric、测试 fixture 和截图不得含敏感原值。

改变上述不变量、核心组件、语言边界或线上契约时，应先提交 ADR。

## 6. Pull Request

PR 必须完整填写模板，包括：

- 关联 issue 或工作包 ID；
- 范围、非目标和可演示结果；
- 契约、ADR、兼容和迁移影响；
- 隐私、安全、删除、公开和第三方来源评估；
- 实际运行的命令、结果和报告路径；
- 回滚方式、已知限制和剩余风险。

默认使用 squash merge。生成器、生成结果和对应测试应形成同一可追溯变更序列；
依赖升级、迁移和安全变化不得隐藏在普通重构中。

## 7. 检查与证据

运行仓库当前实际存在且与变更有关的检查。尚未实现的 CI、`make phase-<n>-gate`
或测试 ID 必须明确写“尚未实现”，不能写成 `N/A` 或声称已通过。纯文档/治理变更
仍需检查：

- Markdown、YAML 和链接格式；
- 来源与许可证登记；
- 敏感信息扫描；
- 与权威契约的一致性；
- diff 中没有无关文件。

## 8. Bootstrap 模式

项目当前处于单维护者 bootstrap 模式。bootstrap maintainer 可以对普通、可逆变更
完成有记录的自审，但不得放宽测试、证据、安全、隐私、兼容或项目契约中的不可豁免
不变量。未建立的远程分支保护、CI 和非作者批准必须如实记录，不能虚构。

多人治理启用条件和所有权规则见 `GOVERNANCE.md`。
