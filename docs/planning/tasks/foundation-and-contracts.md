# 方向 A：基础调研与协议约定

覆盖阶段 0–2，共 19 个工作包。目标是让后续业务代码在稳定契约、可复现环境和
可信身份边界上开发。

## 阶段 0：研究与契约基线

阶段产物：仓库治理、架构决策、统一 Proto、Stream/Metric Registry 和跨语言契约
测试全部可用。

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [x] | R0-01 | 仓库与开源治理基线 | Monorepo 目录、许可证、贡献规则、第三方来源登记、研究模板；完成计划书列出的参考项目研究 | 仓库结构和许可证扫描通过；每份研究记录明确借鉴、不借鉴和许可证风险 | — |
| [x] | R0-02 | 核心架构 ADR 集 | 原始不可变、Event/Series 分离、WAL/Outbox、Kafka、Flink、Iceberg、ClickHouse、PostgreSQL、Temporal、Public 隔离、OPA、Wasm | ADR lint 通过；每项决策包含后果、迁移和回滚 | R0-01 |
| [x] | R0-03 | 统一事件与上传协议 v1 | EventEnvelope、Origin、Correction/Tombstone、SeriesChunk、Batch、ACK、错误码、Lineage | Buf lint/breaking 通过；正常、非法、幂等和签名黄金样例通过 | R0-02 |
| [x] | R0-04 | Stream 与 Metric Registry v1 | Registry 元 Schema、命名/版本/时间/隐私/保留规则；首批应用、Idle、屏幕 Stream 和应用时长 Metric | 合法定义通过，未知字段、非法版本和非私有默认值失败 | R0-03 |
| [x] | R0-05 | 多语言契约生成与发布流水线 | Go、Rust、Kotlin、Java、TypeScript 生成；Buf lint/breaking；版本发布顺序 | 五语言可编译；重复生成无 diff；破坏性变更被 CI 拒绝 | R0-03、R0-04 |
| [x] | R0-06 | 跨语言契约验收包 | 普通事件、Series、Batch 签名、Kafka Key、时间边界和 Registry 黄金向量 | `make contract-test` 一次运行全部测试；各语言结果逐字段/逐字节一致 | R0-03–R0-05 |

阶段 0 完成命令：`make phase-0-gate`

阶段 0 于 2026-07-27 在 bootstrap/单维护者模式完成本地验收，并于 2026-08-11 完成
GitHub 远程、托管 CI、有效 CODEOWNERS 和服务端 `main` 保护的激活；本地与托管证据及
长期单维护者治理边界见[阶段 0 验收记录](../../governance/phase0-acceptance.md)。

## 阶段 1：可复现基础设施

阶段产物：同一套声明可运行于本地和正式 Kubernetes 环境，核心数据组件具备
持久性、可观测性和恢复能力。

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | R1-01 | 环境与 GitOps 基座 | 版本锁、kind/k3s/Kubernetes 环境、Helm Values、Argo CD、Namespace 和部署顺序；可选 Compose 仅用于快速开发 | 空环境可重建；二次同步无漂移；所有镜像/Chart 固定版本 | R0-06 |
| [ ] | R1-02 | 集群安全与凭据基线 | Pod Security、RBAC、default-deny NetworkPolicy、cert-manager、External Secrets、逐 workload ServiceAccount/egress/ACL 矩阵 | `INF-C017` 正向必需路径成功且代表禁路失败；未授权网络和 Pod 被拒绝；Git/日志无明文 Secret；证书可轮换 | R1-01 |
| [ ] | R1-03 | 事件与流处理平台 | Kafka Operator/集群/Topic、Flink Operator、Checkpoint/Savepoint 存储 | produce/consume、Topic 契约和 Flink 状态恢复通过；Kafka 无持久 ACK 时接入不得确认 | R1-01、R1-02 |
| [ ] | R1-04 | 控制库、热库与数据湖 | PostgreSQL 控制数据库与独立接入幂等/ACK 数据库、ClickHouse、MinIO/S3、Bucket 策略、Iceberg Catalog、持久卷 | 两类 PostgreSQL 数据库隔离且可 PITR；四类存储 Pod 重启后数据仍在；Iceberg 可提交并读取 snapshot | R1-01、R1-02 |
| [ ] | R1-05 | 工作流、策略和缓存平台 | Temporal、OPA、Valkey 及最小健康配置 | Worker 重启后 Workflow 继续；OPA 默认拒绝；清空 Valkey 不丢事实 | R1-01、R1-02、R1-04 |
| [ ] | R1-06 | 全链路可观测性 | OpenTelemetry、Prometheus/Mimir、Loki、Tempo、Grafana、平台 Dashboard 和告警 | 同一 trace 跨至少两个服务；指标/日志可查；敏感诱饵扫描为零 | R1-03–R1-05 |
| [ ] | R1-07 | 重建、备份恢复与隔离验收 | PostgreSQL/ClickHouse/Iceberg/Temporal 备份恢复，本地重建脚本，Public Namespace 私有存储隔离 | `make infra-smoke`；空集群重建、隔离恢复和 public 网络扫描全部通过 | R1-03–R1-06 |

阶段 1 完成命令：`make phase-1-gate`

## 阶段 2：身份、安全和控制平面

阶段产物：用户、设备和服务均具有明确身份；上传批次可验证、不可重放；敏感操作
能够授权和审计。

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | R2-01 | 控制平面数据模型与 Identity Service | users、identities、devices、collector_instances、device_keys、Stream/Schema、share_tokens、audit_index 迁移和 Go 服务骨架 | 空库/升级/回滚通过；外键、唯一性、状态转换和 append-only 审计约束生效 | R0-06、R1-04 |
| [ ] | R2-02 | OIDC 用户身份与会话 | discovery/JWKS、issuer/audience/算法验证、首次登录映射、短期用户会话 | 有效登录稳定映射用户；过期、错 issuer/audience 和算法降级全部失败 | R2-01 |
| [ ] | R2-03 | 设备与密钥生命周期 | 设备/Collector 注册、Ed25519 公钥注册、短期设备 Token、撤销、密钥轮换和安全存储接口 | 撤销立即阻止上传；轮换重叠窗口和旧 key 失效均可测试 | R2-01、R2-02 |
| [ ] | R2-04 | 批次签名与防重放组件 | `LCB1` 签名输入、`LCE1/LCC1` Item 摘要、Payload hash、Ed25519 验签、Nonce、时间窗口、exact retry 和重放存储 | `ES-C004/ES-C008/ES-C016` 通过；五语言黄金字节/摘要一致；原样重试返回既有/等价 ACK，只有同 Nonce 绑定不同 Batch、摘要或签名，以及过期新批次失败 | R0-03、R2-03 |
| [ ] | R2-05 | Scope、OPA、Share Token 与审计 | 上传/私有 API 策略、失败关闭、Share Token 创建撤销、设备/密钥/分享操作审计 | 设备只能写授权 Stream；Public/Share Token 不能访问私有 API；敏感操作均有无 Secret 审计 | R1-05、R2-02–R2-04 |
| [ ] | R2-06 | 身份安全端到端验收 | OIDC、设备生命周期、签名、篡改、重放、撤销、越 Scope、Share Token 和审计测试套件 | `make security-test` 全部通过；任一安全依赖不可用时系统失败关闭 | R2-01–R2-05 |

阶段 2 完成命令：`make phase-2-gate`
