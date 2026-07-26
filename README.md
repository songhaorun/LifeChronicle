# LifeChronicle

LifeChronicle 是一个开源目标、自托管、隐私优先的个人数据平台，用于从多设备可靠
采集日常数据，保存不可变原始档案，生成可回放的派生结果，并在严格隔离后按需发布
最小化公开快照。

仓库已经建立阶段 0 的本地 bootstrap 基线：MIT 开源治理、架构决策、统一 Proto、
Stream/Metric Registry、五语言生成和跨语言契约验收均已实现。阶段 1 及生产业务服务
尚未开始。主要入口：

- [参考项目调研](docs/research/reference-project-survey.md)
- [总体架构](docs/architecture/overall-architecture.md)
- [项目工程契约](docs/contract/project-contract.md)
- [事件与 Stream 规范](docs/protocol/event-stream-spec.md)
- [架构决策记录](docs/adr/README.md)
- [契约验收包](contract-tests/README.md)
- [第三方来源登记](THIRD_PARTY_SOURCES.md)
- [基础设施部署规范](docs/operations/infrastructure-deployment-spec.md)
- [开发路线图](docs/planning/development-roadmap.md)
- [阶段任务总览](docs/planning/tasks/README.md)
- [完整项目规划书](docs/planning/project-plan.md)

开始设计或贡献前，先阅读项目工程契约；字段、ACK、Topic 和 Stream 语义以事件规范
为准，系统边界以总体架构为准，阶段范围以开发路线图为准。

## 许可证与验证

LifeChronicle 自有内容采用 [MIT License](LICENSE)。第三方内容仍按各自许可证和
[来源登记](THIRD_PARTY_SOURCES.md)管理；MIT 不会覆盖第三方义务。

从仓库根目录运行：

```powershell
make phase-0-gate
```

该命令执行治理、ADR、文档、来源/Secret 扫描、Buf 兼容、Registry 正反例、五语言
生成编译、重复生成和跨语言黄金向量。仓库当前没有远程地址，因此托管 CI 尚未实跑，
服务端 `main` 分支保护也尚未生效；版本化工作流和目标保护策略已经提交，建立远程时
必须应用并验证。
