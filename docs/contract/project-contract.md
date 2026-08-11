# LifeChronicle 项目工程契约

**文档版本：** v1.0
**状态：** 阶段 0 已接受工程契约
**适用对象：** 维护者、开发者、评审者、发布与运维人员、插件和 Agent 贡献者
**性质：** 本文是可执行的工程协作契约，不是法律合同
**依据：** [参考项目调研](../research/reference-project-survey.md)、
[总体架构](../architecture/overall-architecture.md)、
[项目计划书](../planning/project-plan.md)、
[开发路线图](../planning/development-roadmap.md)、
[事件与 Stream 规范](../protocol/event-stream-spec.md)、
[基础设施部署规范](../operations/infrastructure-deployment-spec.md)

## 1. 目的和规范性用语

本文定义 LifeChronicle 的共同工程边界：哪些能力属于项目、模块如何协作、契约
如何先于实现演进、什么证据才能合并和发布，以及发生冲突或例外时由谁决定。

本契约随阶段 0 本地基线由 bootstrap maintainer 采纳。根目录 MIT `LICENSE`、治理
角色、版本化 CI 定义和本地阶段门已经建立；GitHub 远程、托管 CI、有效 CODEOWNERS
和服务端 `main` 保护于 2026-08-11 激活。仓库仍处于单维护者模式，第二名维护者尚未
登记；模式切换规则见第 18 节。

本文中的“必须”“不得”“仅”是强制要求；“应”表示除非有记录在案的例外，否则
必须执行；“可以”表示允许但不要求。口头约定、聊天记录和未合并的设计稿不能替代
本文、领域规范、ADR 或版本化仓库声明。

本契约不复制事件字段、Topic 参数、基础设施测试通过条件等领域细节。领域规范中的
定义和测试 ID 是这些细节的唯一来源；本文只规定它们何时必须被采用和如何进入工程
门禁。

## 2. 范围基线

### 2.1 项目范围

LifeChronicle 以开源、自托管、隐私优先的个人数据平台为产品目标。项目范围包括：

- Android、Desktop、浏览器、穿戴设备和 IoT 等多设备采集；
- 设备端本地可靠写入、离线缓存、签名、补传和逐项确认；
- 统一事件与高频序列契约、设备身份、接入、幂等和事件日志；
- 原始档案、流式处理、历史回放、热冷查询、导入、导出、删除和恢复；
- 私有控制台，以及后续经物理隔离和隐私转换产生的公开投影；
- 在显式能力授权和资源隔离下运行的插件平台；
- 从本地 kind 到生产 Kubernetes 的同构、GitOps 化部署。

LifeChronicle 自有内容按根目录 `LICENSE` 使用 MIT License。该选择不改变第三方内容
的许可证、NOTICE、署名、源码提供或其他义务；第 7.3 节和第三方来源登记规定采用
第三方内容的独立审批边界。

以下内容不属于核心目标：

- 医疗诊断或医疗建议；
- 未经明确选择采集聊天、通知正文、剪贴板、音视频或精确位置；
- 将任一厂商云、数据库、设备 Entity 或 JSON 行模型作为唯一核心模型；
- 以不能回放、不能迁移或不能自动验收的临时通路替代目标架构；
- 为形式上的分布式引入没有可验证价值的组件。

新增范围必须先说明数据来源、用户价值、隐私等级、保留和删除语义、资源预算、
故障方式及退出方案；不能回答这些问题的提案不得进入实现。

### 2.2 阶段和发布范围

阶段边界以 v2 开发路线图为当前权威计划：

| 方向 | 阶段 | 发布结果 |
| --- | --- | --- |
| 基础调研与协议约定 | 0–2 | 契约、基础设施、身份和安全底座冻结 |
| 基本可用 | 3–8 | Windows/Android 到私有控制台的纵向 MVP |
| 扩展功能 | 9–12 | 高频健康、公开投影、插件、位置和外部源 |

阶段号是质量门，不禁止满足依赖后的并行开发；未通过前置阶段门的成果不得被描述为
后续阶段已完成。

项目计划书与 v2 开发路线图已经按“阶段 3–8 私有 MVP、阶段 10 公开投影”收敛。
以下边界是当前契约：

- 阶段 3–8 的 MVP 只提供私有查询和私有控制台；
- 阶段 3–8 的所有数据必须保持私有，公共功能不得启用；
- 阶段 1 可以建立 `lc-public-api` Namespace 和隔离测试基座，但不得建立可读取用户
  数据的公共路由、Snapshot 或默认开启的卡片；
- 公开投影、Public API、CDN 和公开卡片只能在阶段 10 的隔离、隐私、撤销和暂停门禁
  全部通过后启用。

任何把公开功能提前到阶段 3–8 的提案均属于范围变更，必须走 ADR、威胁模型和本契约
第 16 节的高风险审批，不能以“已有基础页面”为理由绕过阶段 10。

## 3. 术语和权威文档

### 3.1 核心术语

| 术语 | 工程含义 |
| --- | --- |
| 原始事实 | 设备或外部来源提交且只追加保存的 Event、Series、文件及控制记录 |
| 派生结果 | 可从固定输入快照、处理器和规则版本确定性重建的会话、指标或状态 |
| 公开快照 | 经授权、隐私转换和延迟后写入物理隔离存储的只读数据 |
| Event | 适合独立表达的状态、区间、采样、增量或标注记录 |
| Series | 以 Chunk 表达的高频或连续样本，不得展开进入普通 Event 主通路 |
| Stream | Registry 中具有稳定业务语义、Schema、时间、隐私和保留规则的数据流 |
| Registry | Stream/Metric 定义和生命周期的版本化权威声明 |
| WAL | 设备端只追加的本地事实日志 |
| Outbox | 等待取得服务端可靠终态确认的发送队列 |
| 契约 | 跨模块可机器验证的 Proto、Registry、Topic、API、WIT、DDL 或策略接口 |
| 黄金向量 | 固定输入及逐字节或逐字段预期输出，用于跨语言一致性验证 |
| 阶段门 | `make phase-<n>-gate` 所代表的可重复、非零失败验收集合 |
| ADR | 对重要、长期或难以回退决策的版本化架构决策记录 |

本文没有定义的 Event、Batch、ACK、时间、Stream 和 Topic 术语，以事件与 Stream
规范为准；部署对象和环境术语以基础设施部署规范为准。

### 3.2 按问题域确定权威来源

权威不是简单的“最后修改者获胜”，而是按问题域确定：

| 问题域 | 权威来源 | 次级说明 |
| --- | --- | --- |
| 工程协作、门禁、审批、跨领域不变量 | 本契约 | ADR 不能静默覆盖本契约 |
| 系统边界、组件职责、数据所有权和信任边界 | 总体架构 | 本契约将其转化为协作和门禁要求 |
| Event、Series、Batch、ACK、Registry、Topic | 事件与 Stream 规范 | 项目计划书只提供背景 |
| 环境、GitOps、网络、存储、恢复、基础设施测试 | 基础设施部署规范 | 环境 Values 只能细化容量 |
| 阶段顺序、MVP 和发布范围 | v2 开发路线图 | 聚合任务清单用于执行跟踪 |
| 产品愿景、长期目标和设计背景 | 项目计划书 | 不覆盖较新的专项规范 |
| 参考项目事实、版本快照和借鉴边界 | 参考项目调研 | 采用上游前按当时状态重新核实 |
| 单项重要决策及其理由 | 已接受 ADR | 必须同步更新受影响的权威规范 |
| 工作包状态、负责人和证据 | `docs/planning/tasks/` 与项目管理记录 | 不能修改规范语义 |

法律义务、许可证条款和已发布的安全响应要求始终优先。任何专项规范若与本契约的
隐私默认值、可靠确认、物理隔离或变更门禁冲突，合并必须暂停，由维护者在同一变更
中修正文档和测试；不得选择对当前实现最方便的一项。

文档冲突按以下流程处理：

1. 在 PR 中列出冲突条款、影响数据和当前运行行为；
2. 根据上表确认问题域和权威来源；
3. 若改变长期决策，先接受 ADR；
4. 在同一 PR 或有明确先后关系的迁移 PR 中同步修改规范、黄金向量和门禁；
5. 冲突未消除前不得发布 Producer 或开启数据迁移。

## 4. 仓库和模块边界

### 4.1 Monorepo 顶层所有权

阶段 0 应建立以下稳定边界：

| 目录 | 唯一职责 | 不得承担 |
| --- | --- | --- |
| `proto/` | 跨语言 Proto、Buf 配置和生成入口 | 业务实现、手写生成产物 |
| `registry/` | Stream/Metric Registry 元 Schema、登记项、模板和校验夹具 | 运行时私有状态、绕过 Proto 的任意 Payload |
| `codegen/` | 各语言手写生成/编译配置和依赖锁 | 生成源码、业务实现 |
| `generated/` | 从统一契约生成并受重复生成检查保护的五语言产物 | 手工业务逻辑、未追溯的源码 |
| `contract-tests/` | breaking 基线、黄金向量、语义测试和五语言 runner | 生产运行时实现、真实个人数据 |
| `scripts/` | 仓库级 lint、生成、扫描和阶段门实现 | 在线业务逻辑、隐藏本机依赖 |
| `agents/` | 设备采集、本地 WAL/Outbox、签名和同步 | 直连 Kafka、私有数据库或公开存储 |
| `services/` | 身份、接入、查询、Cold Query Worker、投影和插件宿主服务 | 导入其他服务的私有实现包 |
| `streaming/` | Flink 公共运行库及确定性实时处理 Job | 修改原始事实或绕过 Registry |
| `workflows/` | Temporal 长任务、补偿、审批和版本切换 | 在单次 HTTP 请求内模拟长工作流 |
| `lakehouse/` | Iceberg/Parquet Schema、迁移、压缩和表治理 | 控制平面事务业务 |
| `web/` | 私有控制台、后续公开窗口和共享展示组件 | 直连数据库、Kafka 或对象存储 |
| `plugins/` | WIT、SDK、示例和插件包契约 | 绕过 Capability Broker |
| `infrastructure/` | 版本锁、GitOps、Chart、环境差异、策略和恢复编排 | 未版本化手工对象、业务分支逻辑 |
| `tests/` | 跨模块契约、回放、集成、性能、隐私、故障和恢复测试 | 只在维护者本机可运行的隐藏夹具 |
| `docs/` | 架构、ADR、协议、运维、安全、研究和契约 | 与代码行为无关联的口头承诺 |

如需新增顶层目录，必须说明现有边界为何不能承载该职责，并触发 ADR。目录移动不得
改变模块所有权而不更新 `CODEOWNERS`、构建入口、文档链接和发布流水线。

### 4.2 依赖方向

允许的依赖必须保持以下方向：

```text
权威契约 → 生成代码/SDK → Agent、Service、Job、Workflow、Web
设备事实 → Ingestion → Kafka/Bronze → Processing → Query/Private Web
Query Service → 内部受限查询计划 → Cold Query Worker → Iceberg 只读表/前缀
私有事实 → Projection/OPA → Public Snapshot → Public API/Web
Workflow → 版本化 Activity/Client → 被编排组件
Plugin → Capability Broker/WIT → 已授权 Stream
```

强制边界如下：

- 跨语言和跨服务调用只使用版本化 Proto、Topic Schema、OpenAPI 或 WIT；
- 服务不得导入另一个服务的数据库模型或内部包形成隐式 RPC；
- Agent 只能通过统一 gRPC/HTTPS 接入，设备插件不得实现平行上传协议；
- Web 只能调用对应 API；公开 Web 只能调用 Public API；
- `lc-private-api` 中的 Query Service 只能通过内部版本化 API 请求 Cold Query Worker
  执行 Iceberg 冷读，不得持有对象存储或 Iceberg Catalog 凭据，也不得直连其端点；
- Query Service 不得直连 PostgreSQL；用户、设备、Registry、策略和其他控制元数据
  只能经版本化 Control/Identity API 获取；
- Query Service 可以读取 Valkey 中为其分配的独立只读前缀，但必须使用专用 ACL，
  且其中只能是可从权威来源重建的实时缓存；缓存缺失不能放宽权限或改变事实语义；
- Cold Query Worker 部署在 `lc-processing`，只接收受限查询计划，使用独立
  ServiceAccount 和只读表/前缀凭据；它与实时 Flink 使用不同资源池、队列或等价隔离，
  冷查询不得抢占实时处理的 worker、Checkpoint 或状态存储预算；
- Public API 只能读取 Public Snapshot Store，不能拥有私有事实存储驱动或凭据；
- 插件默认无网络、文件、环境变量和数据库能力，只能使用不可伪造的 Host Capability；
- Valkey 只能保存缓存、限流和可重建的短期协调状态，不能成为事实或授权的唯一来源；
- 原始、派生和公开数据的存储、凭据和网络路径必须分离。

### 4.3 数据存储所有权

| 存储 | 权威数据 | 写入边界 |
| --- | --- | --- |
| PostgreSQL | 用户、设备、密钥元数据、Registry、策略、工作流元数据、审计索引，以及独立接入数据库中的 Batch/Nonce/ACK 协调元数据 | 控制库仅由控制 owner 迁移；接入库仅由 Ingestion owner 以专用角色迁移/访问且不得保存原始 Payload；Query 等模块不得直连 |
| Kafka | 近期持久事件日志和处理主干 | 仅登记的 Producer 写入登记的 Topic |
| Iceberg/对象存储 | Bronze 原始档案及可版本化 Silver/Gold 历史 | Sink/Workflow 版本化写入；Cold Query Worker 按独立只读表/前缀读取；不得人工覆盖对象 |
| ClickHouse | 可从原始档案重建的热查询和分析数据 | 登记的 Sink/迁移；不得成为唯一原始事实源 |
| Temporal 持久库 | Workflow 历史和状态 | 仅 Temporal Server 管理 |
| Public Snapshot Store | 已批准、已转换的公开快照 | 仅 Projection Pipeline 写入 |
| Valkey | 可重建缓存、限流和短期协调状态 | 按 consumer 分前缀和 ACL；Query 只读其专用实时缓存前缀 |
| Agent 本地存储 | 本地事实、索引、Outbox、游标和诊断状态 | Agent Core 事务管理，Provider 不得绕过 |

一个模块不得直接写入另一个 owner 的表、Bucket 或 Topic。确需共享读取时必须发布
稳定接口或只读视图，声明 Schema、权限、资源预算和弃用期。

## 5. 技术栈和语言边界

默认技术栈如下。替换核心组件或让新语言进入主数据通路必须有 ADR、迁移和回滚方案。

| 模块 | 基线技术 |
| --- | --- |
| Android Agent | Kotlin |
| Desktop Agent、IoT Gateway | Rust |
| API Gateway、Identity、Ingestion、Query、Projection、Temporal Worker | Go |
| Cold Query Worker | `lc-processing` 内部组件；执行引擎由单独 ADR 选定 |
| Flink Job | Java 或 Kotlin；单个 Job 模块选定一种主语言 |
| Web、Browser Extension | TypeScript；Web 使用 SvelteKit |
| 离线研究与分析 | Python，不得成为在线事实通路的隐式依赖 |
| Plugin Host | Rust + Wasmtime |
| 跨语言协议 | Protobuf + Buf |
| 服务端运行环境 | Kubernetes + Helm + Argo CD |
| 数据主干和存储 | Kafka、PostgreSQL、ClickHouse、Iceberg/Parquet、S3/MinIO |
| 长工作流和策略 | Temporal、OPA |
| 可观测性 | OpenTelemetry + Prometheus/Mimir、Loki、Tempo、Grafana |

语言边界必须满足：

- 业务对象不能通过复制结构体或手写 JSON 在语言间同步；
- 生成代码来自同一已发布契约，生成器和插件版本固定；
- 同一消息在 Go、Rust、Kotlin、Java 和 TypeScript 的黄金向量必须逐字段或逐字节一致；
- HTTP JSON 表示若存在，必须从同一领域契约映射，并有独立版本和兼容测试；
- Flink Java/Kotlin Job 共享的只能是明确发布的 JVM 公共库，不能依赖其他 Job 的私有状态；
- Python 产出的模型或规则进入生产前必须版本化、可重放，并由生产语言实现受控加载接口；
- 基础镜像、工具链、Operator、Chart 和生成器必须固定版本或 digest。

## 6. 契约优先开发流程

任何跨模块、跨语言、跨存储或影响线上数据的工作必须按以下顺序推进：

1. **建立问题和范围。** 写清输入、输出、owner、非目标、依赖、隐私、数据量和验收。
2. **完成必要研究。** 记录参考版本/commit、许可证、借鉴点、不借鉴点和风险。
3. **判断 ADR。** 命中第 15 节触发条件时，ADR 接受前不得实现不可逆部分。
4. **先修改机器契约。** 提交 Proto、Registry、Topic、API、WIT、DDL、策略或配置声明。
5. **定义兼容和迁移。** 写明旧 Producer/Consumer、双读/双写、回填、切换、回滚和退役日期。
6. **添加失败样例和黄金向量。** 至少覆盖正常、边界、非法、重复、乱序及相关拒绝路径。
7. **生成并验证 Consumer。** 先发布能读取新旧版本的 Consumer，再激活 Registry，最后发布 Producer。
8. **实现业务。** 实现不得引入契约外字段、隐式默认值或未登记 Topic/表。
9. **运行分层测试和阶段门。** 测试报告必须关联 commit 和不可变制品。
10. **发布和观察。** 使用同一制品逐环境晋级；满足回滚条件时停止写入、投影或迁移。

不兼容变更不能通过一个“大爆炸”PR 同时替换全部 Producer 和 Consumer。必须采用新
主版本、expand/migrate/contract、双读/双写或隔离输出切换等可回退过程。

探索性原型可以先于契约，但必须满足以下全部条件：不接触真实用户数据、不进入默认
构建和部署、不产生持久兼容承诺、有删除日期和 owner。原型进入主干通路前仍须完成
上述流程。

## 7. 编码、配置和依赖规则

### 7.1 通用编码规则

- 每种语言必须使用仓库固定版本的格式化器、lint 和静态检查器；CI 输出是权威结果；
- 生成代码不得手工编辑；契约和生成器变更后重复生成必须无额外 diff；
- 公开函数、协议字段、错误码和配置必须说明单位、时间、空值和失败语义；
- 错误不得静默吞掉；可重试、永久失败和数据质量问题必须可机器区分；
- 所有外部调用必须设置超时、取消传播和有界重试；重试副作用必须幂等；
- 集合、批次、查询、解压、内存和并发必须有显式上限，不能依赖默认值；
- 时间、随机数、ID 和外部 I/O 必须可注入或可固定，以支持确定性测试；
- `TODO`/`FIXME` 若影响正确性、安全或迁移，必须关联 issue、owner 和截止里程碑；
- 日志、Metric 和 Trace 使用稳定字段名，不得把高基数原始 ID 或 Payload 当标签；
- 任何降级路径必须复用相同身份、校验、幂等和 ACK 管线。

### 7.2 配置规则

- 配置必须有 Schema、默认值、范围和启动时校验；未知关键配置必须失败；
- 安全、授权、公开投影和数据持久性配置默认失败关闭；
- 环境目录只保存差异，不复制组件基线；
- 生产配置不得以源代码条件分支替代版本化 Values/Policy；
- Secret 只通过声明的密钥接口注入，不得进入 Git、测试夹具、日志或诊断包；
- 动态配置必须带版本，并记录生效者、生效时间和回滚版本。

### 7.3 依赖和供应链

- 依赖必须固定可复现版本；容器同时固定 tag 和 digest；
- 新依赖需记录用途、维护状态、许可证、已知漏洞和替代方案；
- 禁止 `latest`、浮动 Chart 范围和未固定 Git 引用；
- 发布必须生成依赖清单或 SBOM，并运行许可证、Secret 和漏洞扫描；
- 高危漏洞无缓解措施时不得发布；例外必须按第 16 节审批且有到期日；
- 复制或改写参考项目代码时，以实际文件头和所用 commit 的许可证为准，并登记来源。

项目自有内容已于 2026-07-27 选择 MIT。第三方内容仍默认只允许基于公开思想、协议和
行为进行 clean-room 研究与独立设计，除非精确版本已经在来源登记中批准：

- 不得复制、翻译、改写或拼接第三方源代码、测试夹具、图标、文案或其他受版权保护制品；
- 外部贡献按 `CONTRIBUTING.md` 明确以 MIT 许可，并须通过来源与贡献权利检查；
- 不得把“GitHub 可见”“开源项目参考”或“无许可证头”解释为允许复用；
- 研究记录必须保存来源、版本/commit、许可证和仅借鉴的设计思想；
- 维护者必须维护根目录 `LICENSE` 和第三方来源清单，并按实际第三方义务决定是否同时
  提交 `NOTICE`、版权声明或其他通知。

许可证选择已由版权持有者明确批准为 MIT；未来变更必须由版权持有者和治理流程另行
批准，不能由普通依赖或代码变更隐式完成。

## 8. Schema、Topic、API、数据库和迁移契约

### 8.1 Protobuf 和 Registry

具体字段以事件与 Stream 规范为准，所有贡献必须遵守以下演进边界：

- 已使用字段号和枚举号不得复用；删除时同时保留名称和编号；
- 新字段必须可选并定义缺省语义；改变单位、符号性、时间或业务含义视为不兼容；
- 不兼容 Payload 使用新 Schema 主版本；Consumer 明确声明接受版本范围；
- `Any.type_url`、Stream、`event_type`、记录类型和 Registry 版本必须一致；
- Stream 名称发布后不得改义；退役通过生命周期状态而不是删除文件；
- Registry 顶层未知字段必须拒绝；新 Stream 模板默认 `PRIVATE`；
- 普通 Event 和 Series 使用不同契约与通路，不能为了复用 Sink 而互相展开；
- Buf lint、breaking 和跨语言生成必须在契约 PR 内通过。

### 8.2 签名输入

v1 跨语言签名输入必须直接采用
[事件与 Stream 规范第 6.1 节](../protocol/event-stream-spec.md#61-上传批次)定义的
`LCB1` 规范帧，包括其字段顺序、长度、端序、Timestamp、字符串、二进制和
`payload_sha256` 规则；本契约不另建平行帧定义。重试同一批次必须复用该规范要求的
完全相同压缩字节、摘要和签名。

Protobuf 的 deterministic serialization 只保证特定实现内的稳定输出，不等于跨语言
或跨版本 canonical encoding。任何实现都不得把 Protobuf 序列化结果当作 `LCB1`
替代品。

签名编码变更属于安全协议变更，必须使用新的魔数/版本，定义验证重叠窗口和退役期，
并通过 ADR、威胁模型和 `ES-C004` 契约门；不得原地改变 `LCB1`。

Item 级内容身份必须直接采用事件规范第 8.3 节定义的 `LCE1/LCC1` 帧和
`submitted_sha256/canonical_sha256`，不得通过重序列化 Event 或 `Any` 计算。
`LCE1/LCC1` 变更同样必须使用新魔数，并通过 `ES-C008` 与 `ES-C017`。

### 8.3 Kafka Topic

- Topic 只能由版本化清单创建，业务服务不得自动创建；
- Topic 名中的主版本表示 Value 契约主版本，不等同于 Stream Schema 版本；
- Key 必须使用事件与 Stream 规范定义的规范编码，不得简单拼接字符串；
- Producer 必须登记 owner、Value Schema、Key、分区、复制、压缩、保留和清理策略；
- 不兼容 Value 或 Key 变更必须创建新 Topic，并提供双写/双读、回放和退役计划；
- Raw Topic 不能使用覆盖历史事实的 compact；latest-state 可以 compact，但历史仍需归档；
- Ingestion 使用幂等 Producer 和持久 ACK；普通 Event 在 Kafka 未确认时不得确认，
  Series 还必须满足第 10.4 节的对象与 metadata 双持久条件。

### 8.4 API 和错误

- 对外及跨服务 API 必须版本化，并由 Proto 或 OpenAPI 生成/校验客户端；
- 认证主体、Scope、资源 owner 和 OPA 决策点必须写入 API 契约；
- 错误使用稳定机器码；安全响应只返回安全细节 ID，不回显 Payload、Token 或内部堆栈；
- 分页 Cursor 必须签名并绑定用户、查询条件和版本，不能跨用户或跨查询复用；
- 列表、范围查询、导出和上传均必须有资源预算、大小限制、超时和取消；
- gRPC 与 HTTPS 上传必须调用同一验证、幂等、Kafka 和 ACK 管线；
- API 弃用必须先发布替代版本、监控使用量、公告退役日期并验证旧客户端失败方式。

冷查询内部 API 还必须满足：

- Query Service 完成用户身份、OPA、Cursor 和范围校验后，只传递签名且绑定用户/Scope/
  Stream/时间范围/投影字段/版本/预算的受限查询计划；
- Query Service 所需用户、设备、Registry 和其他控制元数据必须通过版本化
  Control/Identity API 获取，不得使用 PostgreSQL 驱动、SQL 或数据库只读账号；
- Cold Query Worker 再次验证服务身份、计划签名、允许表/前缀、字段、分区裁剪、扫描
  字节、行数、内存、并发和截止时间，不接受任意 SQL、对象 Key 或文件路径；
- Worker 的对象存储和 Catalog 权限只读且最小化，不能读取未授权表/前缀，不能写入、
  删除、执行 compaction 或取得 root 凭据；
- 超时、用户取消和上游断连必须传播到实际执行引擎并释放扫描资源；
- Query Service 负责热/冷路由及跨边界稳定排序和去重，Worker 不绕过 API 直接返回
  未授权原始字段；
- 请求计划、扫描量、结果量、取消、拒绝原因和数据 snapshot 必须审计，但日志和 Trace
  不记录原始行或敏感 Payload；
- 冷查询执行引擎、文件格式读取库和多租户隔离方案必须在实施前由 ADR 决定。

### 8.5 数据库和表 Schema

- PostgreSQL 只保存事务型控制元数据，包括独立数据库中的接入幂等/ACK 协调，
  不保存原始 Payload；ClickHouse 只保存热查询派生；Iceberg 保存长期档案；
- 表、视图、索引、TTL、排序键、分区键和数据保留必须进入 Git；
- 每张事实表必须声明逻辑主键/去重键、事件时间、revision、来源、Schema 和删除策略；
- ClickHouse 或缓存中的数据必须能从权威来源重建；
- Iceberg 提交必须记录输入 offset/snapshot 关系，原始对象必须可校验；
- 应用不得以启动时自动改表代替经过评审的迁移；
- 数据库超级用户、对象存储 root 凭据不得注入业务 Pod。

### 8.6 迁移规则

所有 Schema、数据库、Topic、策略和处理器迁移必须：

1. 使用单调、不可改写的迁移 ID；
2. 声明前置版本、owner、预计时长、锁和容量影响；
3. 优先采用 expand → migrate/backfill → verify → switch → contract；
4. 保持旧版本在切换窗口内可读写，或明确停止写入的维护窗口；
5. 在 staging 使用生产同拓扑和代表性数据量验证；
6. 提供校验查询、行数/哈希/业务不变量和失败注入；
7. 提供可执行回滚；不可逆时提供恢复点和前向修复方案；
8. 在删除旧字段、Topic 或输出版本前证明所有调用者已迁移；
9. 记录实际开始/完成时间、版本、结果和证据；
10. 数据正确性或隐私检查失败时立即停止迁移和新写入。

## 9. 身份、隐私和安全基线

### 9.1 身份和最小权限

- 用户使用 OIDC；设备使用独立 Ed25519 密钥和短期设备凭据；服务使用独立
  ServiceAccount 和短期身份；
- 用户会话、设备上传、服务间、只读、分享、管理和插件 Capability 凭据不得混用；
- 私钥不得离开设备；撤销必须立即阻止新的受保护操作；
- 所有授权和公开投影决策默认 fail-closed，并记录策略版本和安全审计；
- NetworkPolicy、RBAC、Pod Security、服务身份和 OPA 是互补层，不能互相替代；
- 插件、设备、服务和用户只能访问显式授权的 Stream、API 和资源。
- `lc-private-api` 到 PostgreSQL、对象存储和 Iceberg Catalog 必须 default-deny；
  Query Service 通过 Control/Identity API 获取控制元数据，通过 Cold Query Worker
  冷读，通过专用只读 ACL 读取 Valkey 的独立可重建缓存前缀。Worker 只接受 Query
  Service 的受认证调用，并以独立只读凭据访问显式表/前缀。

### 9.2 数据最小化和默认私有

- 新 Stream、新 Card、新 API 字段和新插件权限默认不可公开；
- 窗口标题、完整 URL、精确位置、健康原值和敏感正文必须有独立、明确的用户选择；
- 本地隐私过滤必须在事实进入任何可上传 WAL/Outbox 之前完成；被排除原值不得进入
  服务端、诊断包或可上传存储；
- 服务端不得记录 Token、完整 Payload、精确位置、健康原值或完整窗口标题；
- 原始数据查看、导出、设备撤销、密钥轮换、分享和删除必须授权、二次确认并审计；
- OPA 只做决策，实际字段删除、取整、模糊、延迟等由受测的 Projection Transformer 执行。

### 9.3 公开隔离

- `lc-public-api` 到 PostgreSQL 私有事实表、ClickHouse 私有库、Kafka raw Topic、
  Iceberg 私有仓库和 MinIO 私有 Bucket 的连接必须在网络层失败；
- Public API 镜像和 Pod 不得包含私有存储驱动、连接串或兜底凭据；
- Public API 只读取通过强 Schema 写入的有效 Snapshot；
- Snapshot、API 缓存和 CDN 必须响应卡片关闭、分享撤销、到期和紧急暂停；
- 未通过阶段 10 威胁模型、组合隐私攻击和 `INF-C010` 隔离验证，不得对外启用。

### 9.4 安全事件

发现以下任一情况必须停止相关写入、发布或公开投影，并保留原始证据：

- 签名或身份验证可绕过；
- 未满足 Event 或 Series 对应的全部持久确认前置条件却返回成功 ACK；
- Public 区域可连接私有事实存储；
- Secret、Token 或敏感 Payload 进入 Git、日志、Trace、制品或公开缓存；
- 删除、迁移或处理器产生不可解释的数据损坏；
- 插件取得未授权 Capability 或逃逸资源限制。

恢复前必须完成影响范围、密钥/Token 撤销、数据和缓存处置、修复验证及事件记录。

## 10. 数据语义

### 10.1 不可变事实、修正和删除

- 原始事实只追加，不原地 `UPDATE`；
- 修正通过新事实及 Correction/Supersedes 关系表达；
- Tombstone 表达逻辑删除意图，不等于跨存储物理删除已经完成；
- 派生结果使用稳定逻辑 ID 和递增 revision，旧版本保留到切换和回滚窗口结束；
- 原始、派生和公开平面必须可区分，查询结果必须显示来源、Schema、处理器和规则版本。

### 10.2 ID、Sequence 和身份注入

- `event_id`、`chunk_id` 和 `batch_id` 由 Agent 持久生成，重试不得更换；
- `collector_instance_id` 在安装、重置或持久序列身份丢失时重新生成；
- Sequence 在 `(device_id, collector_instance_id, source)` 内单调递增；
- Sequence 缺口、倒退和重复是质量信号，不单独证明事实重复，也不应单独拒收；
- Agent 提交的 `user_id` 留空，Ingestion 在身份、签名和哈希验证后注入服务端副本；
- 同 ID、同规范内容是幂等重复；同 ID、不同内容是永久冲突。

### 10.3 批次幂等、防重放和 exact retry

防重放不得破坏合法网络重试。服务端必须在同一原子决策中区分：

- **exact retry：** 同一设备密钥提交的
  `(batch_id, nonce, payload_sha256, signature)` 与既有批次完全一致；
- **ID 内容冲突：** 同一 `batch_id` 对应不同 nonce 或 Payload hash；
- **Nonce 重用：** 同一 nonce 被不同 Batch、Payload hash 或签名使用；
- **新批次：** Batch ID、nonce 和 Payload hash 均未占用。

处理必须遵守
[事件与 Stream 规范第 6.2 节](../protocol/event-stream-spec.md#62-确认模型)：

1. 做有界解码，验证设备绑定、Payload hash 和签名；
2. 在持久幂等记录中原子查询 Batch ID 和 nonce；
3. exact retry 若已有稳定逐项 ACK，必须返回原 ACK，不得重新写入 Kafka，也不得返回
   `NONCE_REPLAYED`；
4. exact retry 若仍有未取得可靠终态的 Item，只重试未决 Item 或按相同幂等规则安全
   重算，并保留既有终态；
5. 同 Batch ID 异内容返回 `ID_CONTENT_CONFLICT`；nonce 被其他内容使用才返回
   `NONCE_REPLAYED`；
6. 新批次原子保留 nonce 后才进入验证和写入管线。

允许设备清理成功 Outbox Item 的终态是 `ACCEPTED_TO_LOG`，或服务端已确认同 ID
同内容曾持久接收后的 `DUPLICATE`。`REJECTED_PERMANENT` 必须移入本地隔离队列；
`RETRYABLE` 必须复用全部 ID、nonce 和原始内容。允许清理的 ACK 和 exact retry
所需记录不能只存在 Valkey；它们必须在声明的重试/审计窗口内可靠恢复。

本节是事件规范 exact retry、ID 幂等和确认时点在工程流程中的直接约束，对应
`ES-C004`、`ES-C008`、`ES-C011` 和 `ES-C016`；实现不得另外定义 Nonce 语义。

### 10.4 普通 Event 与 Series 的持久确认边界

普通 Event 和 Series 的可清理 ACK 前置条件不同：

- 普通 Event 只有在 Kafka 对对应记录完成 durable ACK，且本次可返回逐项终态及
  证据可靠落库后，才能返回允许清理 Outbox 的 `ACCEPTED_TO_LOG` 或等价同内容
  `DUPLICATE`；
- Series Chunk 只有在二进制对象已经 durable 写入对象存储、内容 hash 校验通过，且
  对应 Kafka metadata 已取得 durable ACK，并且双持久证据与逐项终态可靠落库后，
  才能返回允许清理 Outbox 的终态；
- 混合批次必须逐 Item 判定。普通 Event 已持久不代表同批 Series 已持久，反之亦然；
- 任何一个 Series 持久步骤失败、超时或结果未知，都必须返回/保持 `RETRYABLE`，设备
  复用原 `chunk_id`、`batch_id`、nonce、对象内容和 hash；对象存储不可用使用事件
  规范登记的 `OBJECT_STORAGE_UNAVAILABLE`。

Series 对象写入必须满足：

- 对象 Key 按事件规范由租户隔离前缀、稳定 `chunk_id` 和内容摘要生成，不含设备名、
  邮箱、精确位置等敏感业务文本；
- 同 Key、同内容 hash 的重试是幂等成功，不重复创建逻辑对象；
- 同 Key、不同内容 hash 必须拒绝为永久内容冲突，不能覆盖原对象；
- Kafka metadata 必须记录足以验证对象 Key、大小、内容 hash、Schema 和 `chunk_id`
  关联的信息；
- 返回 `DUPLICATE` 前必须同时证明对象和 metadata 均已按同一内容持久存在；
- 允许清理的逐项 ACK、对象提交状态和 metadata 提交状态必须可恢复，不能只存于缓存。

对象先写、metadata 后写必然存在故障窗口。实现必须提供幂等协调和孤儿回收：

1. 对象写入成功但 metadata 未确认时不返回成功 ACK；
2. exact retry 复用相同 Key/hash，继续补写或确认 metadata；
3. 后台对账扫描“有对象无 metadata”和“有 metadata 无对象”两类不一致；
4. 孤儿对象只有在超过声明的 in-flight/重试宽限期、确认没有 ACK/metadata/Workflow
   引用并记录审计后才能删除；
5. metadata 指向缺失或 hash 不匹配对象时必须隔离、告警并触发修复，不能向下游伪装
   为完整 Series；
6. 契约测试和故障测试必须覆盖对象写入前、对象写入后、metadata 发送前、Kafka ACK
   结果未知和 ACK 持久化前的崩溃点。

本节直接采用事件规范第 2、5.2、6.2 和 6.3 节的 Series ACK、对象布局和错误码，
并由 `ES-C015` 验证确认原子性；实现不得降低其对象/metadata 双持久边界，也不得
跳过可恢复逐项终态证据。

### 10.5 时间、乱序和迟到

- `observed_at` 是业务和事件时间；`received_at`、`ingested_at`、`processed_at`
  仅描述系统处理过程；
- `ingested_at` 是最终成功 Kafka Record 在 send 前写入的服务端时间，只有
  durable ACK 后才生效；它不声称等于 broker ACK 的精确时刻；
- 时间统一存储为 UTC；IANA `timezone` 描述观测时民用时区，不改变时间戳；
- 接入端不得修正设备原始时间，只能附加 Clock/Quality 信息；
- 时钟回拨、时区/DST 变化和闰秒不得重写 ID 或 Sequence；
- Watermark 和 allowed lateness 只决定实时输出时机，不修改或丢弃原始事实；
- 允许范围内的迟到事件更新稳定逻辑 ID 的新 revision；
- 超窗事件必须进入 late Topic 并触发可追踪的 Range Replay，不得静默丢弃；
- 大规模历史导入使用独立 Backfill，不得强行推进实时 Watermark。

### 10.6 确定性、血缘和多来源

- 相同输入 snapshot、处理器、规则和 Registry 版本必须得到逐字段相同输出；
- 会话 ID 不得依赖到达顺序或随机数；更新同一逻辑会话时递增 revision；
- 派生数据必须记录处理器、规则、输入 Stream/范围/snapshot、输出 Schema 和 run ID；
- 多来源事实默认全部保留；融合规则必须版本化，不能通过覆盖原始记录隐藏冲突；
- 多来源步数、睡眠、位置或穿戴数据不得未经明确规则直接相加或合并；
- 算法升级先写隔离输出，完成回放和比较后原子切换，并保留可回滚旧版本。

### 10.7 删除、保留和导出

- 删除由可恢复的 Temporal Workflow 执行，覆盖 Kafka 保留范围、ClickHouse、Iceberg、
  对象存储、Valkey、统计、索引、公开快照、导出缓存和备份生命周期；
- 任一步失败不得对外报告全部删除成功；不可立即清除的备份必须返回预计彻底清除时间；
- Retention、TTL 和降采样必须来自版本化策略，不能作为临时运维命令；
- 删除高频原始数据而保留派生特征必须经过用户范围确认并保留血缘和删除依据；
- 导出必须固定输入 snapshot，有界流式生成，并保留 Schema、来源、设备、时间、单位、
  血缘和修正关系；
- “备份成功”只以空环境实际恢复和业务校验成功为准。

## 11. 测试金字塔和质量门禁

### 11.1 测试层级

测试从低成本、高频到高成本、低频分层：

| 层级 | 内容 | 最低要求 |
| --- | --- | --- |
| L0 静态 | format、lint、类型、Schema、生成 diff、Secret/许可证/漏洞扫描 | 每个 PR |
| L1 单元 | 纯函数、状态机、策略、迁移辅助、错误和边界 | 变更代码就近覆盖 |
| L2 契约 | Proto/Registry/Topic/API/WIT/DDL、黄金向量、跨语言 | 契约或边界变更必跑 |
| L3 组件 | 单服务、单 Job、单 Agent、单 Workflow 与真实依赖替身/容器 | 主要成功和拒绝路径 |
| L4 集成/E2E | Agent→接入→Kafka/Bronze→处理→查询/工作流 | 跨组件数据路径 |
| L5 专项 | 故障、回放、隐私、安全、性能、长运行、备份恢复 | 按风险和阶段门执行 |

测试必须确定、隔离、可重复，失败返回非零退出码。Flaky 测试不得通过简单重跑被视为
成功；必须有 owner、原因和限期修复记录。每个生产缺陷修复必须先或同时增加能复现
问题的最低层级测试。

### 11.2 规范测试 ID 的唯一来源

事件契约测试 ID 和通过条件只在
[事件与 Stream 规范第 11 节](../protocol/event-stream-spec.md#11-契约测试基线)
维护，当前为 `ES-C001`–`ES-C018`。基础设施测试 ID 和通过条件只在
[基础设施部署规范第 14 节](../operations/infrastructure-deployment-spec.md#14-最低部署验证)
维护，当前为 `INF-C001`–`INF-C017`。架构级验收 ID 和证据映射只在
[总体架构第 19 节](../architecture/overall-architecture.md#19-架构验收矩阵)维护，
当前为 `ARC-001`–`ARC-022`。

本契约不重新命名或复制这些测试。测试报告必须引用原 ID；若规范新增 ID，阶段门按
该规范的最新已接受版本自动纳入。测试实现和报告不得用本地别名掩盖遗漏。

| 变更/门禁 | 必须引用的规范测试 |
| --- | --- |
| Proto、Batch、Registry、Topic 或流语义变更 | 受影响的 `ES-C*`；阶段 0 运行全部 |
| 签名、Item 内容摘要、幂等、exact retry 或 ACK 变更 | 至少覆盖 `ES-C004`、`ES-C008`、`ES-C011`、`ES-C016`、`ES-C017`；Series 再覆盖 `ES-C015`、`ES-C018` |
| 默认隐私和普通/序列边界 | `ES-C010`、`ES-C012` 及相关拒绝测试 |
| GitOps、版本、网络、持久性或恢复变更 | 受影响的 `INF-C*`；阶段 1 与后续发布按基础设施规范和阶段工作包留证 |
| Public 隔离或默认拒绝 | `INF-C010`、`INF-C011`，并叠加阶段 10 隐私攻击测试 |
| Cold Query 网络、凭据、预算或资源隔离 | `INF-C016`、`ARC-013`，并叠加阶段 6 查询取消、审计和实时负载测试 |
| Ingestion、Projection、OPA 或 Public workload 网络/ACL | `INF-C010`、`INF-C011`、`INF-C017`，并验证正向必需路径与反向禁路 |
| staging/production 发布 | 基础设施规范规定的发布子集及当次变更相关 ID |

### 11.3 合并和阶段门

每个 PR 至少通过：

- 变更文件的 L0 和 L1；
- 受影响契约的兼容、黄金和拒绝路径测试；
- 受影响模块的构建与组件测试；
- 涉及跨服务数据路径时的集成测试；
- 涉及状态、幂等、恢复或迁移时的故障测试；
- 涉及权限、隐私、删除或公开时的失败关闭测试；
- 涉及冷查询时，验证 Query Service 无 PostgreSQL/Catalog/对象存储路径、任意 SQL/
  路径被拒绝、Valkey 仅专用只读前缀、预算和取消能到达执行引擎、Worker 只读权限
  成立，且并发冷查询不影响实时 Flink；
- 文档链接、示例和生成产物一致性检查。

阶段完成必须运行 `make phase-<n>-gate`。对应阶段声明的聚合命令，例如
`make contract-test`、`make infra-smoke`、`make security-test`，必须真实执行测试而
不是打印占位成功。命令、commit、环境、制品 digest、报告路径、耗时、失败重试和
已知限制必须进入验收记录。

阶段 0 已实现 `make contract-test`、`make codegen-test` 和 `make phase-0-gate`：
它们真实执行 `ES-C001`–`ES-C018` 的对应机器检查、五语言编译、重复生成和黄金向量，
任一失败返回非零。`INF-C001`–`INF-C017` 属于阶段 1，不得因阶段 0 通过而宣称执行。
托管 CI 是否通过只能由远程工作流记录证明，本地报告不能替代该证据。

覆盖率百分比不能替代关键不变量测试。签名、ACK、删除、Public 隔离、迁移和恢复即使
代码量很小，也必须覆盖完整成功、拒绝和故障路径。

## 12. CI/CD 和环境晋级

### 12.1 CI 流水线

CI 至少分为以下可追踪阶段：

```text
变更范围识别
→ format/lint/type/schema/secret/license
→ unit
→ contract/codegen/breaking
→ build/package/SBOM
→ component
→ integration/replay/privacy/security
→ 按风险触发 performance/chaos/recovery
→ 不可变制品和测试报告
```

要求：

- PR 必须使用干净环境，不得依赖开发者机器缓存才成功；
- 构建依赖和工具版本来自仓库锁定文件；
- 同 commit 重复构建应得到等价生成输出；
- 必需检查不可由提交者自行跳过；临时豁免按第 16 节记录；
- CI 日志和产物同样受敏感信息扫描约束；
- 阶段门缺少环境、测试数据或报告即视为失败，不能手工勾选代替。

### 12.2 CD 和 GitOps

- 一次构建产生的不可变镜像 digest、包和策略 Bundle按
  `development → staging → production` 晋级，不在各环境重新构建；
- 所有 Kubernetes、Topic、Bucket、数据库和策略声明由 GitOps 管理；
- staging 应与生产保持拓扑和安全边界一致，容量可以缩小；
- Argo CD 使用健康条件和依赖表达，禁止固定 `sleep` 代替就绪检查；
- 集群手工止血变更必须在 24 小时内回写 Git 或撤销；
- 发布记录包含 commit、镜像 digest、Chart、迁移、策略、Savepoint、测试和回滚条件；
- 数据正确性、安全或隐私失败时，自动或人工停止后续环境晋级。

### 12.3 发布和回滚

- 无状态服务滚动或金丝雀发布；
- 数据库使用 expand/migrate/contract；
- Flink 升级前创建 Savepoint，失败恢复旧镜像和 Savepoint；
- Topic 不做破坏性原地修改；
- Projection 和 Public API 必须有独立停止开关；
- 回滚不能假设回退已提交的数据格式；无法向后回滚时必须执行已评审的前向修复；
- 回滚完成后仍需验证数据集合、业务不变量、权限和可观测性，而不只验证 Pod Ready。

## 13. 版本、分支、提交和评审

### 13.1 版本

- 对外软件和 SDK 使用 SemVer；
- Proto 包、API、Topic Value、Stream Schema、签名输入、WIT 和策略 Bundle 分别维护
  自己的显式版本，不能用应用版本隐式代表；
- 兼容新增递增次版本；不兼容变更新增主版本并执行迁移；
- 已发布 tag 不得移动或重写；制品必须能追溯到唯一 commit；
- Draft 契约激活前仍需 breaking 基线；“尚未 v1”不是任意破坏调用者的理由。

### 13.2 分支

- `main` 是目标集成分支；当前未提交的 `master` 必须在首次基线提交前改名为
  `main`。完成 bootstrap 后，`main` 必须受保护、随时可构建并禁止直接推送；
- 开发使用短生命周期分支，命名为
  `feat/<issue>-<slug>`、`fix/<issue>-<slug>`、`docs/<issue>-<slug>`、
  `chore/<issue>-<slug>` 或 `adr/<issue>-<slug>`；
- 一个分支只解决一个可评审目标；长期迁移拆为有顺序、每步可部署的 PR；
- 分支不得混入无关格式化、生成文件或用户本地配置；
- 紧急修复使用相同评审和追溯要求，不创建绕过门禁的永久旁路。

### 13.3 提交

- 提交遵循 `type(scope): summary`，至少支持 `feat`、`fix`、`docs`、`refactor`、
  `test`、`build`、`ci`、`chore` 和 `revert`；
- 每个提交应能说明意图并保持相关测试可运行；不能混入无关修改；
- 契约变更、生成结果和对应测试应在同一可追溯变更序列中；
- 数据迁移、生成器和依赖升级不得隐藏在普通重构提交中；
- PR 默认 squash merge；确需保留多提交迁移历史时由维护者在评审中确认；
- Commit/PR 必须关联 issue 或工作包 ID、测试证据和已知限制。

### 13.4 评审

以下批准人数适用于第 18.1 节定义的多人治理模式；bootstrap/单维护者模式只放宽
批准人数，不放宽测试、证据、兼容、安全、隐私或不可豁免不变量。

- `CODEOWNERS` 必须覆盖顶层模块、契约、安全、数据和基础设施；
- 普通变更至少由一名非作者、具备对应模块所有权或经验的评审者批准；
- Proto/Registry/Topic/API/WIT/DDL 变更还需至少一名受影响 Consumer owner 批准；
- 身份、签名、权限、隐私、公开、删除、密钥和插件沙箱变更需安全/隐私 owner 批准；
- 生产基础设施、不可逆迁移和恢复变更需平台或数据 owner 与维护者共同批准；
- 作者不得批准自己的例外；机器人检查不能替代领域评审；
- 评审重点是语义、失败方式、兼容、迁移、隐私和可运行证据，不只是代码风格。

## 14. Definition of Ready 和 Definition of Done

以下条款只在与工作包产出相关时强制。确实不适用的代码、迁移、运行或测试项必须
在工作包记录中标为 `N/A` 并说明理由，不能静默省略；纯文档和治理工作包不因没有
业务代码而失败，但仍必须完成适用的来源、链接、格式、一致性和评审检查。

### 14.1 Definition of Ready

工作包满足以下全部条件才可进入“进行中”：

- 有唯一 ID、owner、目标用户/调用者和可演示结果；
- 范围、非目标、依赖、输入输出和模块边界清楚；
- 权威规范和受影响契约已列出；
- 隐私等级、授权、保留、删除、导出和审计要求已确认；
- 数据量、延迟、存储、查询和资源预算有可验证假设；
- 兼容、迁移、回滚和发布顺序已设计；
- ADR 触发判断完成，所需 ADR 已接受或工作仅限可丢弃原型；
- 测试层级、相关 `ES-C*`/`INF-C*`、故障和拒绝路径已列出；
- 外部依赖、许可证、测试环境和代表性数据可获得；
- 验收人和完成证据位置已确定。

缺少用户选择、权限、删除范围或不可逆迁移恢复点时，不得以开发中再决定为由开工。

### 14.2 Definition of Done

工作包满足以下全部条件才可标记“已完成”：

- 代码、配置、Schema、迁移和文档均已合并到约定范围；
- 格式、静态、单元、组件及所有相关契约测试通过；
- 跨服务路径的集成/E2E 测试通过；
- 幂等、状态、恢复或迁移至少有一个代表性故障测试；
- 隐私、权限或公开能力至少有一个拒绝路径和失败关闭测试；
- 迁移、回滚、运行、告警和已知限制文档可执行；
- 可观测性足以定位失败，且敏感扫描零命中；
- 安全、性能、容量和长期测试满足该阶段预算；
- 生成制品不可变且可追溯到 commit、依赖和测试报告；
- 阶段门及相关 `ES-C*`/`INF-C*` 有证据；
- 验收人已验证演示或实际恢复结果；
- PR/项目记录包含负责人、commit、命令、报告路径和剩余风险。

“代码已写完”“Pod 已启动”“备份作业显示成功”或“本机测试通过”均不等于 Done。

### 14.3 阶段 0 bootstrap 的适用规则

对 `R0-01` 和 `R0-02`：

- `LICENSE`、贡献规则、角色登记、ADR 和自动化是工作包产出，不是开始该工作包
  之前必须已经存在的输入；
- Ready 至少要求明确 bootstrap owner、文档范围、官方资料来源、禁止复制的第三方
  边界和可复核的完成证据；
- Done 仍以第 17 节的目标退出清单为准；只有研究、架构或契约草稿不代表对应工作包
  完成；
- 单维护者验收按第 18.1 节记录自审和实际检查；无法运行的 CI 或阶段命令必须明确
  标记“尚未实现”，不得以 `N/A` 冒充通过。

## 15. 变更控制和 ADR

### 15.1 必须创建或更新 ADR 的情形

以下变化必须先有 ADR：

- 改变原始事实不可变、Event/Series 分离、WAL/Outbox 或事件时间原则；
- 新增、替换或移除核心数据库、消息总线、处理器、工作流、策略或插件运行时；
- 改变语言边界、顶层模块所有权、跨服务依赖方向或部署形态；
- 选择或更换 Cold Query Worker 执行引擎、查询计划协议或实时/冷查询资源隔离方式；
- 改变 ID、签名编码、设备身份、幂等、ACK、防重放或一致性模型；
- 改变 Stream 业务语义、单位、Topic Key/分区、有状态处理范围或回放模型；
- 改变原始/派生/公开平面、Public 物理隔离或默认私有原则；
- 引入敏感数据采集、外部数据共享、网络 Capability 或新的信任边界；
- 改变保留、删除、备份、恢复或导出的可验证语义；
- 执行不可逆数据库/对象迁移或会造成长时间停止写入的变更；
- 引入会形成长期兼容承诺的外部 API、SDK、WIT 或插件包格式。

简单实现细节、兼容的可选字段、无语义变化的重构通常不需要 ADR，但仍需契约测试。

### 15.2 ADR 内容和生命周期

ADR 至少包含：

```text
状态和日期
问题、上下文和决策驱动因素
选择的方案
至少一个可行备选及不选理由
正面与负面后果
数据、安全、隐私和运维影响
兼容、迁移、回滚和退出方案
测试和可观测证据
Owner、评审者及关联规范/PR
```

生命周期为 `proposed → accepted → superseded/deprecated`。已接受 ADR 不得改写历史结论；
新决策通过新 ADR 取代并双向链接。ADR 接受后必须同步更新权威规范和测试，单独的 ADR
不能让运行行为与规范长期不一致。

### 15.3 契约变更

修改本契约必须：

- 列出受影响角色、模块、阶段门和现有例外；
- 说明是否放宽安全、隐私、可靠性或兼容要求；
- 对破坏性变化提供迁移、回滚和生效日期；
- 多人治理模式由至少两名维护者及相关领域 owner 批准；第 18.1 节单维护者模式
  只能按其自审边界处理普通、可逆调整，高风险或放宽不变量的变更仍需独立评审；
- 在同一变更中更新模板、CI 门禁和引用文档；
- 升级文档版本并留下变更记录。

## 16. 风险、例外和紧急变更

### 16.1 风险登记

会影响数据丢失、重复、隐私、授权、迁移、恢复、兼容、容量或供应链的风险必须登记：

```text
风险 ID、描述和触发条件
影响的数据/用户/环境
概率、影响和严重度
Owner
预防、检测和缓解
应急停止与回滚
验证证据
复审和到期日期
```

高风险不能只写在 PR 评论中；必须进入版本化风险登记或关联 ADR。

### 16.2 例外审批

例外必须是最小范围、限时和可撤销的。申请至少包含未满足条款、原因、影响范围、
补偿控制、监控、验证、owner、到期日和清除 PR。批准规则：

| 例外类型 | 最低批准 |
| --- | --- |
| 一般工程门禁 | 模块 owner + 一名维护者 |
| 契约兼容、数据迁移或基础设施 | 领域 owner + 平台/数据 owner + 一名维护者 |
| 身份、安全、隐私、公开、删除或 Secret | 安全/隐私 owner + 两名维护者 |
| 生产高风险且不可逆 | 两名维护者 + 对应安全/平台/数据 owner，且有恢复演练 |

例外到期后自动失效，未清除时相关发布停止。不得创建“永久临时例外”或让同一例外在
每次发布中复制续期。

以下不变量不得豁免：

- 未经签名和身份验证接受新设备事实；
- 未满足事件规范对应的持久确认前置条件却返回 `ACCEPTED_TO_LOG`；
- Public API/Pod 持有私有事实存储访问路径或凭据；
- 将 Secret、Token 或明确禁止的敏感原值写入 Git、公开制品或观测后端；
- 无恢复点执行已知可能丢失原始事实的不可逆迁移；
- 绕过用户删除、撤销、暂停或 OPA 失败关闭。

### 16.3 紧急变更

紧急变更只授权为止血所需的最小动作，不扩大功能范围。必须：

1. 记录事件 ID、批准人、命令、目标和预计回滚；
2. 优先停止写入、投影或入口，而不是修改原始事实；
3. 保存审计、日志和数据证据，且不传播敏感内容；
4. 变更后立即运行相关安全、数据和健康验证；
5. 集群手工变更在 24 小时内回写 Git 或撤销；
6. 补充根因、长期修复、回归测试和风险登记。

## 17. 阶段 0 目标退出清单

阶段 0 对应 `R0-01`–`R0-06`。以下全部完成并有证据后，才能运行并通过
`make phase-0-gate`，进入“底座冻结”：

当前工作包状态以
[基础调研与协议约定任务表](../planning/tasks/foundation-and-contracts.md)
为准。`R0-01`–`R0-06` 已于 2026-07-27 在 bootstrap 模式完成本地验收，具体命令、
输出、工具链、向量 hash 和限制记录在阶段 0 机器报告。文档状态或生成文件存在本身
不能替代该执行证据。

### 17.1 `R0-01` 仓库与研究治理

- 由维护者完成阻塞性的仓库许可证决策，提交根目录 `LICENSE`，并根据所选许可证及
  第三方义务提交所需 `NOTICE`/版权/来源文件；
- 在上述决策完成前仅允许 clean-room 设计研究，不正式接收外部代码贡献、不复制或
  改写第三方代码和制品；
- 确定贡献规则、行为准则、安全报告方式和发布所有权，并明确贡献许可证关系；
- 建立 `CODEOWNERS`、PR/issue 模板、提交规范和版本化分支保护策略；bootstrap 无远程
  时保存目标策略，创建远程且进入多人治理前必须实际应用并验证；
- 建立第三方来源和许可证登记；
- 对项目计划书列出的 ActivityWatch、Home Assistant Recorder、OwnTracks、
  OpenTracks、Gadgetbridge、Sleepy、Traccar、Dawarich、CloudEvents、Health Connect
  及核心基础设施完成版本化研究记录；
- 每份研究记录包含版本/commit、许可证、重点目录/模型、借鉴、不借鉴、风险和对应模块；
- 仓库结构和许可证/Secret 扫描通过。

### 17.2 `R0-02` 架构与决策基线

- 提交整体架构、数据流、信任边界、部署区和模块依赖图；
- 提交项目计划书要求的首批 ADR：原始不可变、Event/Series、WAL/Outbox、Kafka、
  Flink、Iceberg、ClickHouse、PostgreSQL、Temporal、Public Snapshot、OPA 和 Wasm；
- 每个 ADR 包含后果、迁移、回滚、测试和退出条件；
- 本项目工程契约完成评审，MVP 公开范围已按第 2.2 节固定并与权威计划一致。

### 17.3 `R0-03` 事件和上传协议 v1

- Event、Origin、Correction/Tombstone、Series、Batch、逐项 ACK、错误码和 Lineage
  进入统一 Proto/规范；
- Batch 签名输入使用事件规范的 `LCB1` 独立规范帧，不依赖 Protobuf deterministic
  充当 canonical；
- Item 内容身份使用事件规范的 `LCE1/LCC1`，`Any.value` 保留设备提交原始
  bytes，不通过任一语言的 Proto 运行库重序列化；
- Series Topic Value 使用事件规范的 `RawSeriesRecord`；对象引用包含不可变版本、
  精确大小、压缩 bytes 摘要和解压载荷摘要，`LCS1/LCR1` 对设备提交 wire bytes
  做 domain separation 并绑定可信用户；
- exact retry、Batch ID 冲突和 Nonce 重用的顺序与持久幂等语义符合第 10.3 节；
- Series 只有对象、Kafka metadata 及可恢复逐项终态证据均持久后才能确认，使用
  `OBJECT_STORAGE_UNAVAILABLE` 表达对象存储重试失败，并具备对象幂等和孤儿回收语义；
- 正常、非法、篡改、重复、冲突、乱序、时钟和故障样例齐全；
- 任何允许清理 Outbox 的 ACK 都有可恢复的持久证据。

### 17.4 `R0-04` Registry v1

- Stream 和 Metric Registry 元 Schema、目录、owner 和生命周期建立；
- 命名、Schema、单位、事件时间、隐私、保留、迟到、来源和处理器规则可机器校验；
- 首批 `app.foreground`、`device.idle.state`、`device.screen.state` 及核心 Metric 已登记；
- 未知字段、非法版本、错 Payload、非法单位和缺省隐私测试失败；
- 模板默认 `PRIVATE`，公开投影为 `none`。

### 17.5 `R0-05` 多语言生成和发布

- Buf Workspace、lint、breaking 基线和生成锁定文件建立；
- Go、Rust、Kotlin、Java 和 TypeScript 均能从同一契约生成并编译；
- 重复生成无 diff，生成产物不可手改且可追溯到契约版本；
- 发布顺序、制品版本、Consumer 先行、弃用和回滚流程写入 CI/文档；
- 破坏性变更在 CI 中被拒绝。

### 17.6 `R0-06` 跨语言契约验收包

- `make contract-test` 一次运行事件规范当前全部 `ES-C*`；
- 黄金包覆盖普通事件、Series、规范签名字节、Kafka Key、时间边界、Registry、
  exact retry、Nonce 冲突、ACK 时点和确定性回放；
- 五种语言结果逐字段或逐字节一致；
- 测试报告记录 commit、工具链版本、输入向量 hash 和输出；
- `make phase-0-gate` 汇总治理、ADR、文档链接、生成、兼容和契约测试，任一失败返回非零。

阶段 0 的未决项不能以“后续实现时再确定”进入底座冻结。冻结不表示永不修改，而表示
之后的破坏性变化必须有新版本、ADR、迁移、回滚和旧版本退役计划。

## 18. 契约采纳

### 18.1 Bootstrap 与单维护者模式

在远程仓库、受保护 `main`、`CODEOWNERS`、CI 和至少两名维护者全部建立前，项目
处于 bootstrap/单维护者模式。仓库所有者必须在验收记录中明确登记一名
bootstrap maintainer。该维护者可以：

- 将当前未提交分支改名为 `main`，提交规划文档和可逆仓库脚手架；
- 以版权持有者身份明确选择项目许可证，并提交 `LICENSE` 及所需通知文件；
- 对普通、可逆变更完成有记录的自审，前提是运行所有当前实际存在且适用的检查，
  并如实记录尚未实现的门禁；
- 在 CI 建立后按相同不可变制品和测试证据推进非生产开发。

单维护者模式不得用于：

- 宣称不存在的 PR 批准、CI、`make phase-<n>-gate` 或测试 ID 已通过；
- 自行批准放宽身份、签名、ACK、删除、Public 隔离、Secret 或其他第 16.2 节
  不可豁免不变量；
- 未经独立合格评审执行生产高风险、不可逆迁移或对外公开用户数据；
- 在 `LICENSE` 和 `CONTRIBUTING.md` 生效前接收正式外部代码贡献。

第二名维护者加入且远程、分支保护、`CODEOWNERS` 和 CI 生效后，项目进入多人治理
模式，第 13、15、16 节的非作者和多人批准要求完整生效。切换必须形成版本化记录，
不得用单维护者条款长期绕过已经具备的评审能力。

### 18.2 生效与持续采纳

本契约已于 2026-07-27 通过以下第 1 种 bootstrap 流程生效；后续替代版本仍使用相同
两种采纳路径：

1. bootstrap maintainer 记录自审、实际检查和未满足项，并将本契约纳入首次基线
   commit；或
2. 已存在多人治理时，通过受保护分支的正式 PR 评审合并。

契约生效与阶段完成是独立断言：前者约束后续工作，后者必须由阶段门报告证明。正式
接收的贡献按 `LICENSE`、`CONTRIBUTING.md` 和本契约提供工程证据；本契约不构成
MIT 之外的额外版权许可。

维护者负责让 CI、模板、CODEOWNERS、阶段门和领域规范持续反映本文；若自动化与本文
不一致，应先按更严格且更安全的约束停止发布，再通过评审变更消除差异。

首次 v1.0 发布前必须再次验证签名编码、exact retry、测试 ID 和 MVP 公开范围仍与
各权威规范一致。
