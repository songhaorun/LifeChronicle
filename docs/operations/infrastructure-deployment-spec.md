# LifeChronicle 基础设施部署规范

**文档版本：** v1.0-draft
**状态：** 阶段 1 候选实施基线（未采纳）
**依据：** [项目工程契约](../contract/project-contract.md)、
[总体架构](../architecture/overall-architecture.md)、
[项目计划书](../planning/project-plan.md)

## 1. 目标

本规范定义 LifeChronicle 从本地 `kind` 到生产 Kubernetes 的同构部署边界。
目标不是要求所有环境拥有相同容量，而是要求它们使用相同 Chart、对象命名、
网络边界、密钥接口、可观测性字段和恢复流程。

## 2. 环境和支持等级

| 环境 | 集群 | 用途 | 数据持久性 | 高可用 |
| --- | --- | --- | --- | --- |
| `local` | kind | 开发、契约和集成测试 | 可选 hostPath | 否 |
| `development` | 单节点 k3s 或 K8s | 共享开发 | 必须 | 否 |
| `staging` | 多节点 K8s | 升级、性能、恢复演练 | 必须 | 与生产拓扑一致 |
| `production` | 多节点 K8s | 正式服务 | 必须 | 是 |

`local` 可以降低副本数和资源，但不得替换 Kafka、Flink、PostgreSQL、
ClickHouse、对象存储、Iceberg Catalog、Temporal 或 OPA 的核心通路。

## 3. 版本和制品策略

所有部署制品必须固定版本，禁止在 Git 中使用 `latest`、浮动 Chart 范围或
未固定 Git 引用。

版本唯一来源：

```text
infrastructure/versions.yaml
```

该文件至少固定：

- Kubernetes 最低和最高已验证版本；
- Helm 和 Argo CD 版本；
- 每个 Operator/Chart 版本；
- 每个容器镜像的 tag 与 digest；
- CRD API 版本；
- PostgreSQL、Kafka、Flink、ClickHouse、Iceberg、Temporal 的数据格式主版本。

依赖升级必须由单独变更完成，包含 release note 审阅、staging 升级、回滚或
前向修复方案以及备份恢复验证。

## 4. GitOps 目录和所有权

```text
infrastructure/
├── versions.yaml
├── bootstrap/
│   ├── namespaces/
│   ├── argocd/
│   └── root-application/
├── helm/
│   └── lifechronicle/
├── operators/
├── components/
│   ├── kafka/
│   ├── flink/
│   ├── postgresql/
│   ├── clickhouse/
│   ├── object-storage/
│   ├── iceberg-catalog/
│   ├── temporal/
│   ├── opa/
│   ├── valkey/
│   └── observability/
├── environments/
│   ├── local/
│   ├── development/
│   ├── staging/
│   └── production/
├── policies/
├── backup/
├── scripts/
└── tests/
```

- `bootstrap` 只包含建立 GitOps 控制面所需对象；
- `operators` 管理 CRD 和 Operator；
- `components` 保存环境无关基线；
- `environments` 只保存差异化 Values/Patches；
- 应用团队不得在环境目录复制完整基线；
- 集群内手工修改只能用于止血，且必须在 24 小时内回写 Git 或撤销。

## 5. Namespace 和网络区域

| Namespace | 区域 | Namespace 级候选目标 |
| --- | --- | --- |
| `lc-edge` | edge | Ingestion、Control/Identity API、OTel、DNS |
| `lc-control` | control | PostgreSQL、OPA、Valkey、OTel、DNS |
| `lc-streaming` | streaming | Control/Identity/Registry API、OPA、接入幂等库、Kafka、Series 对象前缀、Valkey、OTel、DNS |
| `lc-processing` | processing | Kafka、ClickHouse、对象存储、Catalog、OPA、Public Snapshot 写入口、OTel、DNS |
| `lc-storage` | storage | 仅组件复制、备份端点、OTel、DNS |
| `lc-private-api` | private-api | Control/Identity API、ClickHouse、Cold Query Worker、Valkey、OPA、OTel、DNS |
| `lc-public-api` | public-api | Public Snapshot Store、OPA、OTel、DNS |
| `lc-plugins` | plugins | Capability Broker、OPA、OTel、DNS |
| `lc-observability` | observability | 采集端点和受控管理端点 |
| `lc-gitops` | management | Kubernetes API 和声明的 Git/Registry |

每个 Namespace 必须先应用 default-deny ingress/egress，再添加白名单。
`lc-public-api` 不得存在通往 PostgreSQL 私有事实表、ClickHouse 私有库、
Kafka raw Topic、Iceberg 私有仓库或 MinIO 私有 Bucket 的网络路径。

Namespace 表只是可创建策略的上界，不代表其中所有 Pod 彼此共享权限。最终授权必须
同时使用带明确 `ServiceAccount` 的 workload NetworkPolicy、服务端认证、数据库/
Bucket/Topic ACL 和应用层 Scope。不得用一条 Namespace 全放行规则实现表中的并集。

### 5.1 Workload 落点与 egress 契约

| Workload / ServiceAccount | 落点 | 必须允许的主动访问 | 必须拒绝的代表路径 |
| --- | --- | --- | --- |
| API Gateway / `lc-gateway` | `lc-edge` | Ingestion；用户入口所需的 Control/Identity API；OTel、DNS | Kafka、对象存储、Catalog、PostgreSQL、ClickHouse、Public Snapshot |
| Ingestion / `lc-ingestion` | `lc-streaming` | Control/Identity/Registry API、OPA、专用接入幂等数据库、Kafka 登记 Topic、Series 私有对象前缀、专用 Valkey ACL、OTel、DNS | ClickHouse、Iceberg Catalog/数据前缀、Public Snapshot、任意未登记数据库/Topic/Bucket |
| Control/Identity API / `lc-control-api` | `lc-control` | 专用 PostgreSQL 数据库/角色、专用 Valkey ACL、OPA、OTel、DNS | Kafka raw、Iceberg/Series 对象、ClickHouse 私有事实库、Public Snapshot |
| OPA / `lc-opa-*` | 调用方所在信任区或 `lc-control` | 签名 Bundle 分发端点（pull 模式时）、OTel、DNS | PostgreSQL、ClickHouse、Kafka、Catalog、对象存储及其他业务事实端点 |
| Flink 与登记 Sink / 各自 SA | `lc-processing` | 仅各 Job 登记的 Kafka Topic、ClickHouse 数据库、Catalog/对象前缀、Control/Registry 只读 API、OTel、DNS | Public Snapshot；未登记 Topic/数据库/Bucket；控制写接口 |
| Cold Query Worker / `lc-cold-query` | `lc-processing` | 只读 Catalog 表/对象前缀、审计端点、OTel、DNS | PostgreSQL、写/删对象、Bucket 全量列表、实时 Flink 资源端点、Public Snapshot |
| Projection Pipeline / `lc-projection` | `lc-processing` | E10-01 登记的派生投影输入、OPA、Delay Queue、Public Snapshot **写**入口、OTel、DNS | Kafka raw Topic、PostgreSQL、Catalog、私有对象前缀、ClickHouse 任意查询、Public Snapshot 管理/读回接口 |
| Query Service / `lc-query` | `lc-private-api` | Control/Identity API、ClickHouse 只读库、Cold Query Worker API、专用 Valkey 只读前缀、OPA、OTel、DNS | PostgreSQL、Catalog、对象存储、Kafka、Public Snapshot |
| Public API / `lc-public-api` | `lc-public-api` | Public Snapshot **只读**入口、OPA、OTel、DNS | 所有私有 PostgreSQL/ClickHouse/Kafka/Catalog/对象端点和 Snapshot 写/管理入口 |

补充约束：

- 外部流量先到 `lc-edge` 的 Gateway，再由 Gateway 调用 `lc-streaming` 的
  Ingestion；不得让 Gateway 自己持久化事件或持有 Kafka/对象凭据；
- Ingestion 的“专用接入幂等数据库”是 PostgreSQL 中独立数据库/角色和 owner
  migration，只保存 Batch/Nonce/摘要、逐项 ACK 与恢复协调元数据，不保存原始
  Payload；其可靠记录满足 exact retry/审计窗口，Valkey 只能作可重建加速层；
- NetworkPolicy 只能限制 Kafka/对象服务端点；Topic、数据库、对象前缀和
  Snapshot 读写边界还必须由独立凭据和 ACL 强制；
- Projection 是从私有数据平面到公开平面的唯一写桥。E10-01 必须冻结其输入
  Topic 或版本化 API、Delay Queue 和 Snapshot 写协议；Public API 不得反向调用
  Projection 或任何私有数据源；
- 每个 workload 使用独立 ServiceAccount 和凭据，禁止把 `lc-processing` 的
  Namespace 能力并集授予任一 Pod。

## 6. 组件部署契约

### 6.1 Kafka

- 使用 Operator 管理，Broker 配置进入 Git；
- production 至少 3 个 Broker，跨节点反亲和；
- raw Topic 使用复制因子 3、`min.insync.replicas=2`、`acks=all`；
- 禁止业务服务自动建 Topic；
- Topic 由版本化清单创建并包含分区、复制、压缩和保留策略；
- 数据卷使用支持快照且具备明确 IOPS 的 StorageClass；
- 暴露 broker unavailable、under-replicated partitions、ISR、consumer lag、
  disk usage 指标；
- 备份以配置、Topic 契约和可从 Bronze 恢复的边界为准，不把 Kafka 视为永久档案。

### 6.2 Flink

- 使用 Flink Kubernetes Operator；
- JobManager 高可用元数据使用持久存储；
- Checkpoint 和 Savepoint 写入专用对象存储前缀；
- 每个 Job 声明并发度、最大并发度、状态后端、Checkpoint 周期和超时；
- production 禁止把 Checkpoint 写入 Pod 本地盘；
- Job 升级先创建 Savepoint，再部署新版本，失败时恢复旧镜像和 Savepoint；
- 指标覆盖 checkpoint age/failure、backpressure、watermark、late events、
  restart count 和 state size。

### 6.3 PostgreSQL

- 使用声明式 Operator 管理主从、备份和恢复；
- 只承载事务型控制元数据：控制数据库保存身份/Registry/策略/工作流等，
  独立接入数据库保存 Batch/Nonce/摘要、逐项 ACK 与恢复协调；后者使用独立
  owner、角色、迁移和连接池，禁止保存原始 Payload；
- production 至少 3 实例或等价的故障切换能力；
- 启用 TLS、连接池、最小权限数据库角色和审计；
- 备份采用连续 WAL + 周期性基础备份；
- 恢复测试必须在新 Namespace 创建新实例，不得覆盖源实例；
- 应用迁移必须支持 expand/migrate/contract，禁止启动时无审查自动改表。

### 6.4 ClickHouse

- 使用 Operator 管理 topology 和配置；
- ClickHouse 只作为私有热读模型是当前基线。Public Snapshot Store 的具体引擎由
  E10-01 ADR 决定；若选择 ClickHouse，必须使用不同集群或网络隔离实例、独立用户
  和独立凭据，本条不预先裁决其实现；
- 表 DDL 和 migration 进入 Git；
- 热数据表必须定义排序键、分区键、TTL 和业务去重键；
- 分布式表不得掩盖副本写入失败；
- 暴露 query latency、merge backlog、replication queue、disk usage、
  rejected query 和 mutation 指标；
- 冷数据可从 Iceberg 重建，恢复演练必须验证实际查询结果。

### 6.5 对象存储和 Iceberg Catalog

- local 使用 MinIO；production 可使用 MinIO 或 S3 兼容托管服务；
- Bucket/前缀至少隔离 `bronze`、`silver`、`gold`、`flink-state`、
  `temporal-export`、`backup`；
- 应用凭据按前缀最小授权，不共享 root 凭据；
- 启用版本控制、服务端加密和生命周期规则；
- Iceberg Catalog 元数据必须备份，并与对象数据保持一致恢复点；
- Bronze 写入记录 Kafka topic/partition/offset 和 Iceberg snapshot ID；
- 完整性测试必须抽样读取 Parquet 并校验记录数、Schema 和内容哈希。

### 6.6 Temporal

- Server 与 Worker 分离部署；
- production 的持久库必须高可用且独立备份；
- Namespace 明确 retention；
- Workflow ID 承载业务幂等键；
- Worker 部署不得在不兼容代码上线时破坏已有 Workflow replay；
- 至少提供一个跨 Worker 重启的持久 Workflow 烟雾测试；
- 暴露 schedule-to-start latency、task failure、workflow failure 和
  worker availability。

### 6.7 OPA

- 策略 Bundle 由 Git 生成、签名和版本化；
- fail-open/fail-closed 必须按决策点显式配置，授权和公开投影默认
  fail-closed；
- 决策日志不得包含 Token 或原始 Payload；
- 策略版本写入审计记录；
- OPA 使用独立 ServiceAccount，不注入 PostgreSQL、ClickHouse、Kafka、Catalog
  或对象存储凭据；NetworkPolicy 明确拒绝这些业务端点；
- 策略输入由调用方按版本化 Schema 提供。OPA 不直连业务数据库补充决策；Bundle
  使用 GitOps 挂载/推送，或只允许访问签名 Bundle 分发端点；
- 发布前运行单元测试、覆盖率门槛和 public/private 隔离测试。

### 6.8 Valkey

- 仅用于缓存、限流、短期 Nonce 和易失协调状态；
- 不得成为用户、设备、Stream 或审计事实的唯一存储；
- Key 必须有前缀和 TTL 策略；
- Query Service 只授予独立只读 Key 前缀和 ACL，不得读取 Nonce、限流、会话或
  其他服务的缓存空间；
- production 配置持久性与故障切换，但业务必须能从权威存储重建；
- 清空缓存测试不得导致事实丢失或权限放宽。

### 6.9 可观测性

OpenTelemetry Collector 接收 OTLP，并将：

- Metric 发送到 Prometheus 或 Mimir；
- Log 发送到 Loki；
- Trace 发送到 Tempo；
- Grafana 只读取上述后端。

所有服务必须传播 W3C Trace Context。最小资源属性：

```text
service.name
service.version
deployment.environment
k8s.namespace.name
k8s.pod.name
```

禁止记录 Token、完整窗口标题、精确位置、健康原始值和完整 Payload。测试环境
必须通过敏感字段扫描器验证这一约束。

### 6.10 Cold Query Worker

- Cold Query Worker 部署在 `lc-processing`，是交互式私有 API 读取 Iceberg
  数据文件的唯一执行边界；`lc-private-api` 不直接连接 PostgreSQL、Iceberg
  Catalog 或对象存储；
- Query Service 只提交经过授权、带用户和 Scope、资源预算、截止时间及查询 ID
  的受限查询计划，不得提交任意 SQL、对象键或文件路径；
- Worker 使用独立只读身份，Catalog 和对象权限限制到允许查询的 Iceberg 表和
  对象前缀；写入、删除、Bucket 列表和未声明前缀必须拒绝；
- 热查询仍走 ClickHouse；冷查询使用独立队列、并发池和资源配额，不得占用实时
  Flink Job 的 TaskManager；
- 取消、超时和客户端断开必须传播到实际执行任务；查询计划、范围、扫描字节、
  策略版本和结果摘要进入无敏感 Payload 的审计记录；
- 具体执行引擎由 ADR 选择，但不得改变上述网络、授权、预算和取消契约。

## 7. Ingress、TLS 和身份

- 外部入口只暴露 API Gateway 上的用户/设备 Ingestion 路由，以及阶段 10 的
  Public API；`lc-streaming` 内的 Ingestion Service 不直接暴露；
- 使用 cert-manager 签发和轮换证书；
- 集群内服务身份使用独立 ServiceAccount，禁止共享 default ServiceAccount；
- 服务间通信至少使用网络身份 + 短期应用凭据；production 应启用 mTLS；
- 管理端点不得通过公共 Ingress 暴露；
- NetworkPolicy、RBAC、Pod Security 和 OPA 授权是互补层，不得互相替代。

## 8. 密钥管理

- Git 只保存 ExternalSecret 引用或加密后的声明，不保存明文 Secret；
- production 使用外部密钥管理系统；
- local 使用独立的、可丢弃的开发 Secret，名称与 production 接口一致；
- root、管理员、数据库超级用户和对象存储主密钥不得注入业务 Pod；
- 每个凭据声明 owner、consumer、scope、rotation_period 和 revoke_runbook；
- 轮换测试必须证明新旧凭据重叠窗口可用且旧凭据最终失效。

## 9. 持久卷和容量

每个有状态组件必须在 Values 中声明：

```yaml
persistence:
  enabled: true
  storageClass: "<environment-defined>"
  size: "<explicit>"
  accessModes: ["ReadWriteOnce"]
  snapshotClass: "<environment-defined>"
```

production 上线前必须记录：

- 每日普通事件数和平均/峰值字节；
- 高频序列每日压缩字节；
- Kafka 保留窗口；
- ClickHouse 热保留窗口；
- Iceberg 年增长量；
- Checkpoint、Savepoint 和备份增长量；
- 70%、80%、90% 容量告警和扩容提前量。

不得依赖自动扩容来弥补未定义的数据保留策略。

## 10. 备份与恢复

| 对象 | 备份方法 | 最低恢复验证 |
| --- | --- | --- |
| PostgreSQL | 基础备份 + WAL | 新实例 PITR 后分别核对控制表和接入幂等/ACK 表 |
| ClickHouse | 原生备份/快照 | 新实例查询指定时间段 |
| Iceberg Catalog | Catalog 数据库备份 | 能列出并读取指定 snapshot |
| 对象存储 | 版本化 + 复制/备份 | 抽样对象哈希和 Parquet 读取 |
| Temporal | 持久库备份 | 未完成 Workflow 可继续 |
| OPA | Git Bundle + 签名制品 | 相同策略测试结果 |
| Kubernetes | GitOps 声明 + CRD 清单 | 空集群可重建 |
| 密钥系统 | 提供商备份机制 | 隔离环境完成受控恢复 |

初始目标：

- local：不承诺 RPO/RTO；
- development：RPO 24h，RTO 8h；
- staging：RPO 1h，RTO 4h；
- production：在容量评审前暂定 RPO 15m、RTO 4h。

“备份成功”只表示作业完成是不够的；必须按季度从空 Namespace 恢复并保存
测试证据。

## 11. 资源、健康检查和中断

- 所有容器必须声明 requests 和 limits；
- JVM 组件的堆上限必须低于容器内存限制并预留 off-heap；
- 存储组件必须设置 PodDisruptionBudget 和反亲和；
- readiness 只表示可接流量，liveness 不得因下游暂时故障反复杀进程；
- startup probe 必须覆盖数据库恢复和 Flink 状态恢复时长；
- 优雅终止时间必须允许 Producer flush、Checkpoint 或连接排空；
- local 的最小资源档由实际集成测试测量后写入 Values，不在规范中伪造。

## 12. GitOps 部署顺序

```text
集群与 StorageClass
→ Namespace、RBAC、Pod Security、default-deny
→ cert-manager、External Secrets、Argo CD
→ 各 Operator 和 CRD
→ PostgreSQL、对象存储、Kafka、ClickHouse
→ Iceberg Catalog、Flink、Temporal、Valkey、OPA
→ OpenTelemetry 和观测后端
→ Topic、Bucket、数据库和策略初始化 Job
→ LifeChronicle 服务
→ Smoke、隔离和恢复测试
```

Argo CD Application 必须使用 sync wave 或等价依赖表达。不得用固定
`sleep` 代替 CRD Established、Deployment Available、Job Complete 或组件
业务健康检查。

## 13. 环境重建

`infrastructure/scripts/rebuild-local.ps1` 必须只执行编排，不内嵌未版本化
YAML。脚本最低行为：

1. 检查 Docker、kind、kubectl、helm 版本；
2. 创建命名集群；
3. 安装 bootstrap；
4. 等待 Argo CD 同步完成；
5. 运行 smoke tests；
6. 输出组件版本、端点和测试报告路径；
7. 重复执行不产生额外资源或失败。

删除集群是显式、独立命令，且要求准确集群名；重建脚本不得默认删除现有集群。

## 14. 最低部署验证

| 测试 ID | 验证 | 通过条件 |
| --- | --- | --- |
| `INF-C001` | Manifest/Chart 静态检查 | lint、schema、render 全通过 |
| `INF-C002` | 版本固定 | 无 `latest`、浮动 Chart 或未固定镜像 |
| `INF-C003` | GitOps 收敛 | 两次同步均为 Synced/Healthy 且无额外 diff |
| `INF-C004` | 本地重建 | 空 kind 集群由脚本一次构建成功 |
| `INF-C005` | 持久化 | 重启 Pod 后 Kafka/PostgreSQL/ClickHouse/MinIO 数据仍在 |
| `INF-C006` | Flink 状态 | Checkpoint 写入对象存储且 Job 可恢复 |
| `INF-C007` | Temporal 持久性 | Worker 重启后 Workflow 继续并完成 |
| `INF-C008` | 跨服务 Trace | 同一 trace_id 跨至少两个 LifeChronicle 服务 |
| `INF-C009` | 日志隐私 | 敏感字段扫描零命中 |
| `INF-C010` | Public 隔离 | 从 `lc-public-api` 到所有私有事实存储连接均失败 |
| `INF-C011` | 默认拒绝 | 未声明 NetworkPolicy 放行的测试 Pod 无法出入站 |
| `INF-C012` | PostgreSQL 恢复 | 空 Namespace PITR 成功且核对数据一致 |
| `INF-C013` | 对象完整性 | Bronze 抽样对象哈希和 Parquet 读取通过 |
| `INF-C014` | Kafka 故障语义 | 不满足 ISR 时 Producer 失败且接入端不确认 |
| `INF-C015` | 凭据轮换 | 新凭据生效、旧凭据在窗口后失效 |
| `INF-C016` | 冷查询隔离 | Private API 无 PostgreSQL/Catalog/对象存储路径；Query 仅可读专用 Valkey 前缀；Worker 只读授权表/前缀，预算、取消、审计生效，且并发冷查询不影响实时 Flink |
| `INF-C017` | Workload 连通矩阵 | Gateway→Ingestion、Ingestion→控制/OPA/幂等库/Kafka/Series 前缀、Projection→登记输入/OPA/Delay/Snapshot 写、Public API→Snapshot 只读均成功；表 5.1 的代表禁路、OPA→业务库及 Public→私有源全部失败，Topic/DB/前缀/读写 ACL 同时生效 |

每次 staging/production 发布至少运行 `INF-C001` 至 `INF-C003`、
`INF-C008` 至 `INF-C011`、`INF-C016` 及 `INF-C017`；恢复类测试按计划执行并
保留报告。

## 15. 发布和回滚

发布记录必须包含 Git commit、镜像 digest、Chart 版本、数据库迁移、策略
Bundle 版本、Flink Savepoint 和回滚条件。

- 无状态服务使用滚动或金丝雀发布；
- 数据库先 expand，旧版本兼容后再迁移和 contract；
- Flink 先 Savepoint，再升级；失败恢复旧 Job；
- Kafka Topic 不做破坏性原地变更；
- CRD/Operator 升级必须先在 staging 验证降级限制；
- 发现数据正确性或隐私越权时立即停止新写入/公开投影，并保留原始事实供恢复。

## 16. 完成定义

基础设施平台只有同时满足以下条件才算阶段 1 完成：

1. 所有目标组件均由 GitOps 声明；
2. 空集群能够重建；
3. 有状态数据通过 Pod 重启持久性测试；
4. Flink 和 Temporal 状态恢复通过；
5. 跨服务可观测性通过且无敏感载荷；
6. Public Namespace 隔离由自动化测试证明；
7. 备份作业和至少一次隔离恢复有证据；
8. 所有失败测试均能返回非零退出码供 CI 判定。
