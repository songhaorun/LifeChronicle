# 第三方来源与许可证登记

> - 登记基线：2026-07-27
> - LifeChronicle 自有内容许可证：MIT

## 1. 目的与范围

本登记覆盖：

- 直接和传递依赖、容器、Chart、插件和工具链；
- 复制、改写或生成的源码、测试、schema、迁移和配置；
- 图标、字体、图片、地图、固件、样例数据和文档；
- 公开协议、设备材料和 clean-room 研究参考；
- 构建时使用但不随产品分发的第三方工具。

项目 MIT License 不会改变第三方内容的许可证。技术审计、能成功构建或许可证宽松，
都不自动等于批准纳入或分发。

## 2. 状态定义

| 状态 | 含义 |
| --- | --- |
| `RESEARCH_ONLY` | 仅观察公开行为或接口；没有代码、资产、制品进入产品 |
| `CANDIDATE` | 正在进行许可证、来源、安全和架构审查；尚未纳入 |
| `APPROVED_DEPENDENCY` | 已批准按锁定版本作为依赖使用，义务和退出方案齐全 |
| `VENDORED` | 经专项批准将第三方内容保存在仓库中，逐文件来源和修改清楚 |
| `REJECTED` | 不允许纳入；登记保留以防重复评估 |
| `REMOVED` | 曾经纳入、现已移除；保留历史版本和清理证据 |

从 `RESEARCH_ONLY` 或 `CANDIDATE` 进入产品必须有独立登记记录和维护者批准。

## 3. 已纳入产品的第三方内容

| ID | 名称 | 精确版本 | 用途 | 状态 | 许可证/通知 | 记录 |
| --- | --- | --- | --- | --- | --- | --- |
| TOOL-001 | Buf CLI | `1.72.0` | 本地 Proto lint/build/generate 编排 | `APPROVED_DEPENDENCY` | Apache-2.0；不随仓库分发二进制 | `toolchain.lock.json` 固定官方 release 与 Windows SHA-256 |
| TOOL-002 | Protocol Buffers compiler | `35.0` | Java/Kotlin/Rust 内置生成与 descriptor 构建 | `APPROVED_DEPENDENCY` | BSD-3-Clause；保留上游许可证 | `toolchain.lock.json` 固定官方 release 与 Windows ZIP SHA-256 |
| TOOL-003 | Go toolchain | `1.26.5` | Go 生成器构建、生成 SDK/runner 编译 | `APPROVED_DEPENDENCY` | BSD-3-Clause；仅构建工具 | `toolchain.lock.json` 固定官方 ZIP SHA-256 |
| TOOL-004 | Rust toolchain | `1.97.1` | Rust SDK/runner 编译 | `APPROVED_DEPENDENCY` | MIT OR Apache-2.0；仅构建工具 | `toolchain.lock.json` 固定版本；由 rustup 官方通道安装 |
| TOOL-005 | JDK / Gradle / Kotlin Gradle Plugin | JDK `24.0.2`、Gradle `9.0.0`、Kotlin `2.2.0` | Java/Kotlin SDK 与 runner 编译，目标字节码 17 | `APPROVED_DEPENDENCY` | JDK 发行版按安装来源；Gradle/Kotlin Apache-2.0；不提交工具二进制 | `toolchain.lock.json` 与固定 build script；CI 使用 Temurin |
| TOOL-006 | Node.js / pnpm / TypeScript | Node `22.12.0`、pnpm `9.15.4`、TypeScript `6.0.3` | TypeScript SDK 生成与类型检查 | `APPROVED_DEPENDENCY` | Node MIT、pnpm MIT、TypeScript Apache-2.0；仅构建工具 | `toolchain.lock.json`、`codegen/typescript/pnpm-lock.yaml` |
| TOOL-007 | `protoc-gen-go` | `1.36.11` | Go SDK 本地生成 | `APPROVED_DEPENDENCY` | BSD-3-Clause；保留许可证和 PATENTS | 官方 Go module source，由锁定 Go 工具链构建 |
| TOOL-008 | `@bufbuild/protoc-gen-es` | `2.13.0` | TypeScript SDK 本地生成 | `APPROVED_DEPENDENCY` | Apache-2.0 | `codegen/typescript/pnpm-lock.yaml` 固定完整性 |
| DEP-001 | `google.golang.org/protobuf` | `1.36.11` | Go 生成 SDK 与 runner 运行时 | `APPROVED_DEPENDENCY` | BSD-3-Clause；保留许可证和 PATENTS | `generated/go/go.mod`、`go.sum` |
| DEP-002 | Rust Protobuf runtime | `protobuf`/`protobuf-well-known-types` `4.35.0-release` | Rust 生成 SDK 与 runner 运行时 | `APPROVED_DEPENDENCY` | BSD-3-Clause | `codegen/rust/Cargo.lock`；宏/构建传递依赖见下表 |
| DEP-003 | JVM Protobuf runtime | `protobuf-java`/`protobuf-kotlin` `4.35.0` | Java/Kotlin 生成 SDK 与 runner 运行时 | `APPROVED_DEPENDENCY` | BSD-3-Clause | `codegen/java`、`codegen/kotlin` 固定依赖坐标 |
| DEP-004 | `@bufbuild/protobuf` | `2.13.0` | TypeScript 生成 SDK 与 runner 运行时 | `APPROVED_DEPENDENCY` | Apache-2.0 AND BSD-3-Clause | `codegen/typescript/pnpm-lock.yaml` 固定完整性 |

批准范围仅限阶段 0 的契约生成、编译和测试，不代表批准任何产品服务、容器或部署依赖。
仓库不保存上述工具二进制；本地下载位于忽略的 `.tools/`。版本、来源和下载 hash 以
[`toolchain.lock.json`](toolchain.lock.json) 为机器权威，语言依赖树以各生态锁文件为准。

Rust 运行时当前锁定的传递包如下；同名不同版本分别列出：

| 包 | 精确版本 | 许可证 |
| --- | --- | --- |
| `cc` | `1.4.0` | MIT OR Apache-2.0 |
| `find-msvc-tools` | `0.1.9` | MIT OR Apache-2.0 |
| `linkme` / `linkme-impl` | `0.3.37` | MIT OR Apache-2.0 |
| `paste-complete` | `1.0.15` | MIT OR Apache-2.0 |
| `proc-macro2` | `1.0.107` | MIT OR Apache-2.0 |
| `protobuf-codegen` / `protobuf-macros` | `4.35.0-release` | BSD-3-Clause |
| `quote` | `1.0.47` | MIT OR Apache-2.0 |
| `shlex` | `2.0.1` | MIT OR Apache-2.0 |
| `syn` | `2.0.119`、`3.0.3` | MIT OR Apache-2.0 |
| `unicode-ident` | `1.0.24` | (MIT OR Apache-2.0) AND Unicode-3.0 |

批准人：bootstrap maintainer 宋昊润；批准日期：2026-07-27。正式发布前仍须从锁文件生成
完整 SBOM/许可证报告并复核当时的安全公告；本登记不替代发布审计。

## 4. Clean-room 研究参考

下表全部为 `RESEARCH_ONLY`。精确证据、借鉴/不借鉴边界和风险见
[参考项目调研](docs/research/reference-project-survey.md)。没有从这些项目复制源码、
测试、迁移、schema、文档正文、图标或其他资产。

| ID | 项目/规范 | 调研锚点 | 已知许可证 | 使用边界 |
| --- | --- | --- | --- | --- |
| RES-001 | ActivityWatch | `master@29d5da0` | MPL-2.0 | watcher/heartbeat 模式研究 |
| RES-002 | Home Assistant Recorder | `dev@ed63e16` | Apache-2.0 | 状态、历史、统计分层研究 |
| RES-003 | OwnTracks Android | `master@434baec` | EPL-1.0 | 公开位置消息与离线行为研究 |
| RES-004 | OwnTracks Recorder | `master@69ccbe3` | GPL-2.0-or-later 及组件矩阵 | MQTT/HTTP 接收行为研究 |
| RES-005 | OpenTracks | Codeberg `main@30ed8d8` | Apache-2.0 | Android 长时采集与 GPX 研究 |
| RES-006 | Gadgetbridge | Codeberg `master@e1e7400` | AGPL-3.0 | capability/coordinator 模式研究 |
| RES-007 | Sleepy | `v5.2`、`main@56237f4` | 核心 MIT；文档 CC-BY-SA-4.0；v6 前端 GPL-3.0-or-later | 状态、公开卡片与插件体验研究 |
| RES-008 | Traccar | `master@ff09f38`、API `6.14.5` | Apache-2.0 | 协议适配与统一位置模型研究 |
| RES-009 | Dawarich | `master@5ea231d`、`1.9.1` | AGPL-3.0 | 位置时间线、visit/trip 研究 |
| RES-010 | CloudEvents | 稳定规范 `1.0.2`、`main@c2845a4` | Apache-2.0 | 外部事件互操作语义研究 |
| RES-011 | Android Health Connect | 稳定客户端 `1.1.0` | artifact/sample/平台条款需按集成版本核实 | 平台 API 和增量同步语义研究 |
| RES-012 | Protocol Buffers | `main@c4be748` | BSD-3-Clause 等，按具体 artifact 核实 | schema/transport 与非 canonical 风险研究 |
| RES-013 | Apache Kafka | `trunk@123356b` | Apache-2.0 | 近期持久事件主干研究 |
| RES-014 | Apache Flink | `master@9485109` | Apache-2.0 | event-time 派生研究 |
| RES-015 | Temporal | `main@91929f4` | MIT | durable workflow 研究 |
| RES-016 | ClickHouse | `master@5df3873` | Apache-2.0 | 热分析读模型研究 |
| RES-017 | Apache Iceberg | `main@8dfc4b5` | Apache-2.0 | 永久历史与 snapshot 研究 |
| RES-018 | Open Policy Agent | `main@fa9cce7` | Apache-2.0 | 授权/隐私决策研究 |
| RES-019 | Wasmtime | `main@4993061` | Apache-2.0 WITH LLVM-exception | capability 插件隔离研究 |
| RES-020 | OpenTelemetry Specification | `main@f62b146` | Apache-2.0 | 可观测语义与隐私边界研究 |

提交实现前必须重新核实上游版本、许可证和安全公告；调研锚点不是依赖版本批准。

## 5. 新增或变更流程

1. 复制
   [第三方来源登记模板](docs/governance/templates/third-party-source-record.md)；
2. 分配稳定 ID，如 `DEP-0001`、`ASSET-0001`、`TOOL-0001`；
3. 固定 tag/commit、下载 URL、hash、artifact 和传递依赖；
4. 核实仓库根许可证、逐文件头、NOTICE、资产和生成物；
5. 说明静态/动态链接、构建时/运行时、网络提供和分发方式；
6. 评估与 MIT、目标平台和发布方式的兼容；
7. 完成安全、维护状态、升级、替换和退出评估；
8. 维护者批准后更新本表、锁文件、SBOM、NOTICE/署名和测试；
9. 版本升级重复审查变化的许可证、依赖树和公告。

许可证不明、来源不可复现、权利不清、已知高危无修复或无法履行义务时，状态必须保持
`CANDIDATE` 或改为 `REJECTED`。

## 6. Clean-room 规则

- 研究者只记录公开可验证的行为、接口、约束和来源；
- 不把上游表达、结构、命名细节或测试向量直接交给独立实现者，除非它们是获准公开
  互操作规范的一部分；
- 设备协议还需记录资料获取权限及专利、商标、商业秘密和反规避风险；
- 研究工具只能在隔离临时环境运行，其 clone、二进制和输出不能进入产品工作区；
- 独立实现必须从 LifeChronicle 自有契约和测试生成，并保留 provenance；
- 发现疑似污染时立即停止合并、隔离内容、记录范围并由维护者审查。

## 7. 审计与发布

本文件是人工来源登记，不替代锁文件、SBOM、许可证扫描或制品 attestations。正式发布
前必须证明：

- 所有随附内容均有登记 ID、精确版本和 hash；
- 许可证、NOTICE、署名、源码提供和修改声明义务已履行；
- 构建制品能追溯到 commit、依赖、工具链和测试；
- 已移除内容不再出现在源码、缓存、镜像、安装包或公开资产中；
- Secret 和真实个人数据扫描为零。
