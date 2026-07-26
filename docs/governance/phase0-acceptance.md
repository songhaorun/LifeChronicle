# 阶段 0 bootstrap 验收记录

> - 验收日期：2026-07-27
> - 验收人：bootstrap maintainer 宋昊润
> - 治理模式：单维护者 bootstrap
> - 结论：`R0-01`–`R0-06` 本地基线接受

## 1. 范围与决策

本记录接受阶段 0 的仓库治理、13 份 ADR、事件与上传 Proto v1、Stream/Metric
Registry v1、Go/Rust/Kotlin/Java/TypeScript 五语言生成，以及跨语言黄金验收包。
LifeChronicle 自有内容使用 MIT License；第三方依赖仍按来源登记和各自许可证管理。

该决定只冻结契约演进边界，不表示阶段 1 基础设施、生产服务、Agent 或公开数据能力
已经实现。

## 2. 可执行证据

仓库根入口为：

```powershell
make phase-0-gate
```

机器报告写入 `artifacts/phase0/latest.json`，其中记录：

- 被测 commit、分支和运行前工作树状态；
- 操作系统、Python 运行时和完整 `toolchain.lock.json`；
- breaking baseline 与 JSON/properties 黄金向量 SHA-256；
- 每个命令、规范/工作包映射、退出码、输出尾部和耗时；
- 自动重试次数（固定为 0）与已知限制。

门禁聚合治理/许可证/Secret 扫描、ADR 与文档检查、Buf lint/build/breaking 及破坏负控、
Registry 正反例、16 个语义契约测试、五语言生成编译、连续生成无差异，以及五语言
44 个字段/帧逐项一致检查。失败不会自动重试或被转换为成功。

## 3. Bootstrap 自审

单维护者已核对：

- 根 MIT 文本、贡献许可关系、行为准则、安全联系、来源登记和所有权一致；
- 参考调研只形成事实性结论和 clean-room 边界，没有复制参考项目实现或资产；
- 生成只使用本地插件，不向远程生成服务上传 Proto；
- 权威输入、Registry、生成配置、生成物和验收包按 ADR-013 分离；
- breaking 负控会拒绝字段删除/重编号，Registry 负例和签名/幂等负例会失败关闭；
- 本地工具、依赖缓存、Gradle/Cargo/Go/TypeScript 构建产物和 Secret 不进入 Git 候选集。

## 4. 已知限制与远程激活条件

当前没有 Git remote，因此以下事项没有被宣称已经发生：

- GitHub Actions 托管运行成功；
- GitHub 服务端 `main` 分支保护已启用；
- 非作者 PR 批准或 CODEOWNERS 远程身份识别已验证。

仓库已经保存精确 SHA 固定的 CI 工作流和
`.github/branch-protection-main.json` 目标策略。创建远程并准备进入多人治理时，必须：

1. 确认托管平台能识别 CODEOWNERS 账号或团队；
2. 推送 `main` 并让 `phase-0-gate` 托管检查成功；
3. 应用目标保护策略，验证强制 PR、CODEOWNERS、对话解决、线性历史和禁止强推/删除；
4. 登记第二名维护者并按 `GOVERNANCE.md` 切换治理模式。

JDK 24 运行 Protobuf JVM 4.35.0 时会打印 `sun.misc.Unsafe` 终止弃用警告；当前编译和
黄金结果通过，但升级 JDK 或 Protobuf 时必须继续验证并消除该兼容风险。
