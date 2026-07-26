# LifeChronicle 参考项目调研

> 状态：架构与项目契约输入
> 调研日期：2026-07-26（Asia/Shanghai）
> 资料范围：项目官方网站、官方文档、官方规范与官方源代码仓库
> 版本口径：应用项目和基础设施以调研日的稳定版或默认分支精确提交为锚点；默认分支提交仅用于可复现调研，不等于生产版本建议

## 1. 结论摘要

本轮调研支持 `docs/planning/project-plan.md` 已确定的总体方向：LifeChronicle 应保持“设备侧可靠采集、不可变原始事件、可重放派生、事件与序列分离、默认私有、受控公开”的产品边界，不应直接复刻任一参考项目的数据模型或代码结构。

约束性结论如下：

1. ActivityWatch 的 watcher、heartbeat 和时间段查询适合桌面活动采集，但其 bucket、任意 JSON 和可合并事件不适合作为全局身份、模式、审计或不可变事实模型。
2. Home Assistant Recorder 的“当前状态、历史、短期统计、长期统计”分层值得采用，但单一关系数据库和 entity/state 模型不能承载 LifeChronicle 的全部原始、高频及多模态数据。
3. OwnTracks、OpenTracks、Traccar 和 Dawarich 共同验证了“离线队列、统一位置模型、导入导出、原始点与行程/停留派生分离”的方向；位置隐私、来源和精度必须进入契约。
4. Gadgetbridge 适合借鉴设备能力、协调器和协议适配器的分层；其协议实现和 Dawarich 服务端代码受 AGPL 约束，不能直接复制。
5. Sleepy 验证了实时设备/打开应用状态、简洁公开页面和卡片扩展的产品体验，但其当前快照、访问统计和进程内插件不能承担 LifeChronicle 的历史事实、隐私隔离或不可信插件边界。
6. Android 健康数据优先通过 Health Connect 接入，并以按数据类型的 changes token、来源、记录方式、设备、更新和删除语义实现增量同步；不能把 Health Connect 当成永久云存储或唯一事实源。
7. CloudEvents 可作为外部互操作语义和网关映射参考，但不能替代 LifeChronicle 自有事件信封、领域字段、隐私元数据和幂等契约。
8. Protobuf 只承担类型化 schema 和传输职责。官方明确说明 deterministic serialization 并非跨语言、跨版本的 canonical serialization，不能直接作为签名规范的唯一字节定义。
9. Kafka、Flink、Temporal、ClickHouse、Iceberg、OPA、Wasmtime 和 OpenTelemetry 的职责应保持窄边界，避免将基础设施能力误写成端到端语义保证。
10. 调研时仓库尚无许可证；维护者已于 2026-07-27 选择 MIT 并建立第三方来源治理。许可证决策不追溯授权复制参考项目源码、测试、迁移、schema、文档正文、图标或其他资产。

## 2. 调研方法与证据口径

本文件采用以下证据优先级：

1. 官方规范和官方产品文档；
2. 官方当前上游仓库中的许可证、README 和实现边界；
3. 官方发布说明；
4. 默认分支精确提交，用于固定调研时点。

未采用第三方博客、聚合站、镜像仓库或二手许可证摘要作为结论依据。版本号和提交只描述 2026-07-26 的调研快照；落地依赖仍须由 ADR 和依赖锁文件单独选定、审计和升级。

### 2.1 应用项目与规范快照

| 项目 | 当前上游/版本锚点 | 许可证证据 | 调研定位 |
|---|---|---|---|
| ActivityWatch | `master` [`29d5da0`](https://github.com/ActivityWatch/activitywatch/commit/29d5da0df6476131c643c2c110ffa05b4f0e6291) | [MPL-2.0](https://github.com/ActivityWatch/activitywatch/blob/master/LICENSE.txt) | 桌面活动采集与时间段查询 |
| Home Assistant Core / Recorder | `dev` [`ed63e16`](https://github.com/home-assistant/core/commit/ed63e16db7b93b2035c142e8561d2cdf355ba1c7) | [Apache-2.0](https://github.com/home-assistant/core/blob/dev/LICENSE.md) | 状态、历史、统计和保留 |
| OwnTracks Android | `master` [`434baec`](https://github.com/owntracks/android/commit/434baec29b24bc1ccbb50b45b7fa3c69964187d4) | [EPL-1.0](https://github.com/owntracks/android/blob/master/LICENSE.md) | 移动位置采集与离线发送 |
| OwnTracks Recorder | `master` [`69ccbe3`](https://github.com/owntracks/recorder/commit/69ccbe32268914852bf54fe7f20e9782277de742) | [GPL-2.0-or-later 及随附组件许可矩阵](https://github.com/owntracks/recorder/blob/master/LICENSE) | MQTT/HTTP 位置接收与轻量归档 |
| OpenTracks | Codeberg `main` [`30ed8d8`](https://codeberg.org/OpenTracksApp/OpenTracks/commit/30ed8d8ba3305a50bbb79acac5f6fd7fcfc66d22) | [Apache-2.0](https://codeberg.org/OpenTracksApp/OpenTracks/src/commit/30ed8d8ba3305a50bbb79acac5f6fd7fcfc66d22/LICENSE) | Android 长时运动与轨迹采集 |
| Gadgetbridge | Codeberg `master` [`e1e7400`](https://codeberg.org/Freeyourgadget/Gadgetbridge/commit/e1e7400f7840fd4d4c303e42d6662b239eefe034) | [AGPL-3.0](https://codeberg.org/Freeyourgadget/Gadgetbridge/src/branch/master/LICENSE) | 可穿戴设备能力与协议适配 |
| Sleepy | 稳定发布 [v5.2](https://github.com/sleepy-project/sleepy/releases/tag/v5.2)；`main` [`56237f4`](https://github.com/sleepy-project/sleepy/commit/56237f42e146f9b63cc48e785a1540fe2c38d5f8)；v6 仍处开发阶段 | [核心 MIT](https://github.com/sleepy-project/sleepy/blob/56237f42e146f9b63cc48e785a1540fe2c38d5f8/LICENSE)；[文档 CC-BY-SA-4.0](https://github.com/sleepy-project/docs/blob/9e005e1b5028199c3d33b8e8ee255592c685726e/LICENSE)；[v6 前端 GPL-3.0-or-later](https://github.com/sleepy-project/sleepy-frontend/blob/912fec317be757039f5916e8c2034ffee0856e1a/LICENSE) | 实时状态、公开卡片与插件体验 |
| Traccar | `master` [`ff09f38`](https://github.com/traccar/traccar/commit/ff09f38e17713471cb526869f99bd5ca5d28b6d7)；官方 OpenAPI 标注 `6.14.5` | [Apache-2.0](https://github.com/traccar/traccar/blob/master/LICENSE.txt) | 多协议定位适配和统一位置模型 |
| Dawarich | `master` [`5ea231d`](https://github.com/Freika/dawarich/commit/5ea231d5977ee0465867315be8267c9f57c85064)；官方发布说明 `1.9.1` | [AGPL-3.0](https://github.com/Freika/dawarich/blob/master/LICENSE) | 位置时间线、导入、行程和停留 |
| CloudEvents | 稳定规范 `1.0.2`；`main` [`c2845a4`](https://github.com/cloudevents/spec/commit/c2845a49bc9831be02f305a4a792401b932d77d4) 为 `1.0.3-wip` | [Apache-2.0](https://github.com/cloudevents/spec/blob/main/LICENSE) | 外部事件互操作语义 |
| Android Health Connect | 稳定客户端 `androidx.health.connect:connect-client:1.1.0`；文档同时列出 `1.2.0-alpha04` | [AndroidX 发布页及源码入口](https://developer.android.com/jetpack/androidx/releases/health-connect) | Android 健康数据聚合与增量同步 |

OpenTracks 的旧 [GitHub 仓库](https://github.com/OpenTracksApp/OpenTracks) 已归档并明确迁移至 Codeberg；因此本文件以 Codeberg `main` 为当前上游，不把旧 GitHub release 或提交当作当前基线。Gadgetbridge 的旧 [GitHub 仓库](https://github.com/Freeyourgadget/Gadgetbridge) 同样已归档并指向 Codeberg。

## 3. 许可证与来源治理

### 3.1 仓库许可证前提

本调研执行时 LifeChronicle 仓库根目录没有 `LICENSE`，因此当时没有推断目标许可，
也没有假设任何 copyleft 代码与未来分发模式兼容。维护者随后于 2026-07-27 明确选择
MIT；当前根许可证与批准依赖以仓库 `LICENSE` 和 `THIRD_PARTY_SOURCES.md` 为准。

在许可证决策完成前，只允许 clean-room 研究设计。研究阶段允许：

- 借鉴公开的产品行为、架构思想和互操作接口；
- 根据公开协议形成独立的接口契约、行为模型、测试计划和来源记录；
- 链接官方文档并记录事实来源；
- 在隔离的临时研究环境中运行官方发布工具或项目，用于观察公开行为、验证互操作
  假设、检查许可证和执行安全审计；不得把研究环境、二进制、容器、生成物或代码
  带入产品工作区、构建链和交付物。

运行研究工具不等于批准复用、链接、嵌入或分发该工具。研究工具本身的下载、运行
和访问仍须遵守其许可证、服务条款、平台政策和所在地法律。

在许可证决策完成前，不允许：

- 把任何第三方库、二进制、容器、插件、sample 或生成代码加入产品依赖、运行时、
  构建链、安装包、镜像或部署清单，即使它已经正式发布或完成技术审计；
- 发布、部署或分发包含第三方依赖的 LifeChronicle 产品组合；
- 复制参考项目的源码、测试、数据库迁移、schema、构建脚本或注释；
- 改写后纳入受限源码，或通过翻译语言规避许可义务；
- 复制 README、文档段落、截图、图标、地图资产或示例数据；
- 假定“协议实现”天然不受著作权、专利、商业秘密、商标或反规避规则影响。

即使目标许可证和分发模式已经确定，任何依赖加入或产品分发仍需单独批准：核实精确版本
与逐文件许可证、依赖树、NOTICE/署名、专利和商标条款、修改与源码提供义务、
网络使用影响、平台政策及升级/退出方案。技术审计通过不能代替许可证兼容审批。

### 3.2 许可证类别与风险

| 类别 | 本轮项目 | 主要风险 | LifeChronicle 默认策略 |
|---|---|---|---|
| AGPL-3.0 | Gadgetbridge、Dawarich | 强 copyleft；修改后通过网络提供功能可能触发对应源代码提供义务，且文件/资产可能另有许可 | 不复制代码；仅研究行为和公开接口；如未来复用，必须专项法律与部署模式审查 |
| GPL-2.0-or-later / GPL-3.0-or-later | OwnTracks Recorder 主程序、Sleepy v6 前端 | 分发组合程序时的强 copyleft；Recorder 还包含多许可证/双许可证组件，Sleepy 前后端也不是同一许可 | 当前仅研究公开协议/API；未来获批后仍优先独立适配，不嵌入或派生 copyleft 实现 |
| MPL-2.0 | ActivityWatch | 文件级 copyleft、修改文件源码和通知义务 | 可研究接口；任何文件级代码复用须先确认未来许可证与分发义务 |
| EPL-1.0 | OwnTracks Android | 弱 copyleft、模块/贡献与兼容性问题，且版本较旧 | 当前只研究公开消息格式并形成 clean-room 设计；目标许可证确定前不实现或复制应用代码 |
| Apache-2.0 | Home Assistant、OpenTracks、Traccar、CloudEvents 及多数基础设施 | 需保留版权、许可、NOTICE 和修改声明；包含专利条款；商标、文档、资产和第三方文件不必然同许可 | 当前仅作研究候选；目标许可证确定后，采用依赖或代码前仍须逐文件审计和批准 |
| MIT | Temporal、Sleepy 核心 | 保留版权和许可文本；依赖、前端、插件、文档和资产需分别审计 | 当前仅作研究候选；不得因许可证宽松直接加入或分发 |
| CC-BY-SA-4.0 | Sleepy 官方文档 | 署名与相同方式共享义务，不等于核心代码许可证 | 仅链接和事实性归纳，不复制文档正文或示例表达 |

“官方仓库使用宽松许可证”不代表仓库内所有协议材料、固件、字体、地图、图标和第三方目录都使用相同许可证。任何将来允许的复用都必须建立来源清单，至少记录：

- 上游 URL、tag/commit、文件路径和文件头；
- SPDX 标识、第三方许可证和 NOTICE；
- 本地修改、分发方式、网络提供方式；
- 许可证兼容性审批和替换/升级策略；
- 逆向工程或设备协议资料的来源与权限。

## 4. 参考项目逐项结论

### 4.1 ActivityWatch

#### 已核实设计

[Buckets and Events](https://docs.activitywatch.net/en/latest/buckets-and-events.html) 将 watcher 产生的数据放入按 watcher、主机和来源区分的 bucket；事件包含时间戳、时长和任意 JSON 数据。heartbeat 会在脉冲窗口内合并相邻且数据相同的活动。

[REST API](https://docs.activitywatch.net/en/latest/api/rest.html) 提供 bucket、event、heartbeat 和 query 操作。官方同时说明 API 仍在演进，安全模型主要依赖本地主机使用，不能把这一假设带到跨设备服务端。

[Python client](https://docs.activitywatch.net/en/latest/api/python.html) 展示了客户端排队与时间范围查询；[数据处理示例](https://docs.activitywatch.net/en/latest/examples/working-with-data.html) 和 [导出功能](https://docs.activitywatch.net/en/latest/features/exporting-data.html) 说明原始时间记录可被重新查询、聚合和导出。

#### 可借鉴

- watcher 与领域采集器分离；
- heartbeat 作为在线状态或连续活动的采样输入；
- 以事件时间和时间范围为核心的查询；
- 桌面前台应用、窗口标题和空闲状态的独立采集；
- watcher 本地失败队列和可恢复发送。

#### 不应继承

- 不把 bucket id 同时承担身份、schema、租户和权限职责；
- 不把任意 JSON 当作全局领域契约；
- 不在不可变 raw 层直接合并、覆盖 heartbeat；
- 不丢弃源时区和设备时钟质量信息；
- 不采用仅依赖 localhost 的安全边界。

#### LifeChronicle 映射

- `desktop-agent`
- `app.foreground`
- `device.idle.state`
- `sessionizer`
- `timeline-query`

实施建议：原始 heartbeat 保持不可变；连续会话由带算法版本和 lineage 的派生处理器生成。

### 4.2 Home Assistant Recorder

#### 已核实设计

[Recorder 集成文档](https://www.home-assistant.io/integrations/recorder/) 说明 Recorder 保存状态和事件，供 history、activity、dashboard 与统计功能使用；提供 purge、保留天数、提交间隔和 include/exclude 策略，并支持 SQLite、MariaDB/MySQL 和 PostgreSQL。官方不支持随意数据库迁移，说明持久化契约和运维迁移需要被显式治理。

Home Assistant 把当前状态、历史记录和统计用途区分开。官方 [Recorder statistics API 变更说明](https://developers.home-assistant.io/blog/2025/10/16/recorder-statistics-api-changes/) 还显示统计元数据和 API 会演进，消费者不能越过公共接口依赖内部表。

#### 可借鉴

- 当前状态、事件历史、短期统计和长期统计分层；
- 指标元数据、单位和统计类型显式化；
- include/exclude、purge 和保留策略；
- 数据库迁移与公共 API 的边界；
- 低频状态和统计的关系型查询体验。

#### 不应继承

- 不用 entity/state 表达所有位置、健康、媒体和序列数据；
- 不让单一 SQL 数据库承担全部 raw、高频序列和长期归档；
- 不复制 Home Assistant 的事件总线、integration loader 或内部表；
- 不允许 purge 破坏审计所需的原始保留契约。

#### LifeChronicle 映射

- `latest-state`
- `metric-registry`
- `aggregate-worker`
- `retention-policy`
- PostgreSQL 元数据与低频业务状态

### 4.3 OwnTracks Android 与 Recorder

#### 已核实设计

[OwnTracks HTTP 文档](https://owntracks.org/booklet/tech/http/) 说明相同 JSON 消息可以经 HTTP 或 MQTT 发送；网络不可达时，移动端可排队并在恢复后提交。官方建议使用 TLS。

[JSON 消息文档](https://owntracks.org/booklet/tech/json/) 使用 `_type` 区分 location、waypoint、transition 等消息；位置包含经纬度、时间、精度和触发原因等字段。[Recorder 文档](https://owntracks.org/booklet/clients/recorder/) 描述了 MQTT/HTTP 输入、文件存储、REST、WebSocket 和 GeoJSON 输出。

[安全文档](https://owntracks.org/booklet/features/security/) 明确 Recorder 不应直接暴露到公网，要求 TLS，并记录了 2026 年应用安全行为的调整。这进一步说明设备身份、远程配置和接收端暴露面不能只沿用轻量示例。

#### 可借鉴

- 传输无关的位置消息语义；
- 网络中断时的本地队列和恢复发送；
- 用户、设备和来源分离；
- 位置点、地理围栏、waypoint 和 transition 的领域区分；
- GeoJSON/GPX 等开放格式导入导出。

#### 不应继承

- 不以 Basic Auth 代替设备身份、密钥轮换和细粒度授权；
- 不以 Recorder 平面文件作为长期核心存储；
- 不将 OwnTracks JSON 直接定义为 LifeChronicle canonical schema；
- 不把 Recorder 直接公开到互联网；
- 不复制 Android 的 EPL-1.0 代码或 Recorder 的 GPL 代码和随附组件。

#### LifeChronicle 映射

- `location-collector`
- `location.position`
- `location.transition`
- `mqtt-gateway`
- `geojson-exporter`
- `location-importer`

接入层可接受 OwnTracks 兼容消息，但必须映射到自有事件信封，并保存原始来源、精度、触发原因和接收时间。

### 4.4 OpenTracks

#### 已核实设计

当前上游位于 [Codeberg](https://codeberg.org/OpenTracksApp/OpenTracks)。[官方网站](https://opentracksapp.com/) 强调无广告、无分析、无云依赖和离线记录；当前仓库说明支持长时间轨迹记录、GPX/KML/KMZ 导入导出、唯一 track id 和外部传感器。

#### 可借鉴

- Android 长时间前台采集服务的生命周期；
- 进程终止、设备重启和网络中断后的恢复；
- 轨迹会话与 BLE 传感器会话边界；
- 本地优先持久化和开放格式导出；
- 明确的轨迹身份和导出关联。

#### 不应继承

- 不把 workout track 作为通用事件模型；
- 不复制应用 UI、本地数据库 schema 或遗留 MyTracks 实现；
- 不假设仓库所有历史文件和资产都与根许可证一致；
- 不以旧 GitHub 归档仓库作为当前实现依据。

#### LifeChronicle 映射

- `android-agent`
- `exercise.session`
- `location.track`
- `sensor.sample`
- `gpx-exporter`

### 4.5 Gadgetbridge

#### 已核实设计

当前上游位于 [Codeberg](https://codeberg.org/Freeyourgadget/Gadgetbridge)。[项目概览](https://gadgetbridge.org/internals/development/project-overview/) 将设备通信服务、设备服务、设备模型、DeviceCoordinator 和具体 DeviceSupport 分开；[新增设备教程](https://gadgetbridge.org/internals/development/new-gadget/) 展示能力声明与协议适配的边界。

[Health Connect 集成文档](https://gadgetbridge.org/basics/integrations/health-connect/) 说明同步是显式启用的，并列出可写入的数据类型。官方 [0.91.0 发布说明](https://gadgetbridge.org/blog/release-0_91_00/) 为 2026-05-13，证明该项目仍在快速支持新设备和平台变更。

#### 可借鉴

- 设备 coordinator、能力描述与协议 support 分离；
- 不同固件/协议版本的隔离；
- 设备发现、配对、同步游标和能力降级；
- wearable → Health Connect → LifeChronicle 的首期聚合路径；
- 对外只暴露规范化能力，不让领域层认识具体蓝牙协议。

#### 不应继承

- 不复制 AGPL 协议实现、测试向量、逆向工程代码或 UI；
- 不假定厂商协议、固件和商标材料可随 AGPL 代码一并自由复用；
- 首期不以 LifeChronicle 自研直连 BLE 覆盖所有设备；
- 不把 capability flag 当作数据质量或授权证据。

#### LifeChronicle 映射

- `wearable-provider`
- `device-capabilities`
- `health-normalizer`
- `sync-cursor`
- 后续可选 `ble-adapter`

直连设备协议必须按 clean-room 流程实施：记录公开资料来源，由未接触受限实现的人员依据独立接口契约实现，并保留法律和安全审批。

### 4.6 Sleepy

#### 已核实版本、状态与设计

[Sleepy 官方文档站](https://sleepy.wss.moe/) 将项目定位为个人在线状态展示：
实时更新手动状态和设备打开应用，提供公开状态页面、管理面板、开放 API 和插件
系统。官方 GitHub organization 已验证对 `sleepy.wss.moe` 域名的控制，当前核心
仓库默认分支为 `main` [`56237f4`](https://github.com/sleepy-project/sleepy/commit/56237f42e146f9b63cc48e785a1540fe2c38d5f8)。

最新正式发布为 [v5.2](https://github.com/sleepy-project/sleepy/releases/tag/v5.2)
（2025-12-13）；官方
[v6 首页](https://github.com/sleepy-project/docs/blob/9e005e1b5028199c3d33b8e8ee255592c685726e/v6/index.md)
明确标记 v6 仍处于开发阶段。因此，已发布行为以 v5.2 为稳定参考，v6 仅作为
未来设计信号，不能作为 LifeChronicle 的稳定兼容目标。

官方
[v5 API 文档](https://github.com/sleepy-project/docs/blob/9e005e1b5028199c3d33b8e8ee255592c685726e/v5/apis.md)
提供当前主状态、设备状态和最后更新时间查询，通过 Server-Sent Events 推送状态
更新，并允许客户端上报设备是否使用及当前状态/应用名。其 metrics 主要服务访问
和状态统计；官方当前文档没有给出可承担通用、不可变应用历史的稳定契约。

官方
[v5 插件文档](https://github.com/sleepy-project/docs/blob/9e005e1b5028199c3d33b8e8ee255592c685726e/v5/plugin-development.md)
支持插件路由、主页卡片、管理卡片、HTML 注入、事件监听和独立 JSON 数据。
[v6 插件文档](https://github.com/sleepy-project/docs/blob/9e005e1b5028199c3d33b8e8ee255592c685726e/v6/plugin-development.md)
进一步引入 `pyproject.toml` 元数据、版本化依赖、生命周期钩子、配置 schema、
路由/ASGI mount、CLI、插件间 hook 和依赖拓扑排序。这些能力证明“状态变化驱动
扩展”和“卡片作为独立展示单元”有产品价值，也同时显示进程内插件拥有很高权限。

Sleepy 是多仓库、多许可证项目：核心为 MIT，官方文档为 CC-BY-SA-4.0，独立
v6 前端声明 GPL-3.0-or-later，插件及其依赖还需逐项核实。
[核心 README](https://github.com/sleepy-project/sleepy/blob/56237f42e146f9b63cc48e785a1540fe2c38d5f8/README.md)
也记录了部分前端和模板的第三方来源，不能依据核心仓库根许可证推断所有文件和
资产。

#### 可借鉴

- 设备状态上报简单、低摩擦，并能实时反映在线/离线和打开应用；
- 当前状态查询与 SSE 更新分离；
- 将公开状态做成简洁页面和组合式卡片；
- 卡片只消费经过选择的公开 projection；
- 状态改变触发扩展，而不是由扩展轮询内部表；
- 插件声明 id、版本、依赖、配置 schema 和生命周期；
- 应用历史统计只作为派生产品原型，由统一事件流重建。

#### 不应继承

- 不把服务端可变 current snapshot、最后更新时间或访问 metrics 当作历史事实；
- 不用单个应用名字符串表达跨设备、跨窗口、跨会话的通用活动模型；
- 不采用 GET 修改状态、URL 中共享 secret 或单一实例密钥作为设备身份；
- 不把前台应用/窗口标题默认公开；Android Accessibility/Usage Access 等高敏权限
  必须显式同意、最小采集，并支持本地过滤和撤权；
- 不让公开页面直接读取 private/raw 数据；公开卡片只能读取独立 public projection，
  并支持延迟、模糊化、过期和撤销；
- 不继承进程内 Python 插件的任意路由覆盖、HTML 注入、文件/网络访问和共享内存；
- 不把仍在开发的 v6 API、插件 ABI 或 frontend 当作稳定契约；
- 不复制 MIT 核心、GPL 前端、CC-BY-SA 文档、插件或第三方模板代码与资产。

#### LifeChronicle 映射

- `public-window`
- `public-card`
- `android-accessibility`
- `application-session`
- `latest-state`
- `public-projection`
- `plugin-runtime`

落地时，原始前台应用变化先形成带客户端事件时间、设备、幂等序列和隐私级别的
不可变事件；开放会话、应用历史和卡片统计由版本化派生生成。实时状态来自可重建
缓存，公开卡片来自物理隔离的 public plane。扩展沿用 LifeChronicle 的 Wasmtime
capability、OPA 决策、资源限额和输出 schema，而不是 Sleepy 的进程内插件执行
模型。

### 4.7 Traccar

#### 已核实设计

[架构文档](https://www.traccar.org/architecture/) 将每种设备协议实现为 Netty pipeline：frame decoder、protocol decoder、统一 `Position`，随后进入通用工具、事件和数据处理器；命令方向采用相反路径。

[REST API 文档](https://www.traccar.org/traccar-api/) 与官方 [OpenAPI](https://www.traccar.org/api-reference/openapi.yaml) 覆盖 devices、positions、events、reports 和 geofences。调研日 OpenAPI 标注版本为 `6.14.5`。[Geofence 文档](https://www.traccar.org/geofences/) 展示了地理围栏作为独立对象和事件来源。

#### 可借鉴

- 协议 adapter → 统一位置模型 → 通用处理器；
- 每个设备的身份、协议和最近状态分离；
- 协议 fixture、边界帧和错误输入测试；
- geofence 定义与进入/离开事件；
- 位置报告与设备命令的双向边界。

#### 不应继承

- 不把数百种协议和完整 Traccar 服务端模型纳入核心；
- 不把 server-side `Position` 当作 LifeChronicle 全局事件模型；
- 不复制设备厂商协议 PDF、受限资料或来源不明的实现；
- 不让协议 adapter 绕过身份、签名、schema 和隐私检查。

#### LifeChronicle 映射

- `device-registry`
- `location-adapter`
- `location.position`
- `geofence-evaluator`
- `location.alert`
- `location-report`

优先通过 Traccar 公共 API 或明确许可的设备协议接入，而不是把 Traccar 核心嵌入 LifeChronicle。

### 4.8 Dawarich

#### 已核实设计

[官方仓库](https://github.com/Freika/dawarich) 提供位置历史地图、导入、访问、行程等功能，并明确项目仍在活跃开发且可能有破坏性变更。[导入文档](https://dawarich.app/docs/features/imports/) 展示了多种位置历史格式接入。[1.9.1 发布说明](https://dawarich.app/blog/dawarich-1-9-1/) 发布于 2026-07-07。

#### 可借鉴

- 私有位置时间线和地图交互；
- OwnTracks、GPX、GeoJSON 等来源导入；
- raw points 与 visit/trip 派生对象分离；
- 导入批次、来源和解析错误可追溯；
- 从时间线进入日、地点、行程的查询体验。

#### 不应继承

- 不复制 AGPL Rails/PostGIS 服务端、schema、查询或算法；
- 不把仍处于 beta/演进中的 visit 推断当作不可变真相；
- 不让地图 UI 直接查询未脱敏的精确原始点；
- 不将导入文件中的来源 id 当成全局可信身份。

#### LifeChronicle 映射

- `location-timeline`
- `visit-detector`
- `trip-detector`
- `map-view`
- `location-import`

visit/trip 必须记录算法、模型和参数版本，可从 raw location 重新生成，并允许人工 correction/supersedes。

### 4.9 CloudEvents

#### 已核实设计

[CloudEvents 规范](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md) 定义 `id`、`source`、`type`、`specversion` 等必需上下文属性，以及 `subject`、`time`、`datacontenttype` 等可选属性；`source` 与 `id` 的组合用于事件唯一性。官方仓库标记稳定规范为 `1.0.2`，默认分支当前为 `1.0.3-wip`。

规范的安全考虑指出上下文属性可能被日志系统记录，不应把敏感信息放入易传播的上下文字段。

#### 可借鉴

- 事件 id、来源、类型、主题和发生时间的清晰语义；
- HTTP、Kafka 等绑定的网关映射；
- 外部 webhook、导入器和插件边界的互操作；
- 上下文与业务 payload 分离。

#### 不应继承

- 不用 CloudEvents 替代 LifeChronicle 的设备身份、序列、签名、隐私级别、schema version、lineage 和 correction；
- 不以 `source + id` 单独承担重放、批次和幂等的全部规则；
- 不在可观察上下文字段放精确位置、健康值或用户文本；
- 不以 WIP 分支规范作为稳定生产契约。

#### LifeChronicle 映射

- `EventEnvelope` 的术语参考；
- `ingestion-gateway`
- `external-event-gateway`
- `plugin-event-adapter`

内部信封可以提供到 CloudEvents `1.0.2` 的无损或显式有损映射，但内部 canonical schema 保持独立。

### 4.10 Android Health Connect

#### 已核实设计

[Health Connect 发布页](https://developer.android.com/jetpack/androidx/releases/health-connect) 在调研日列出的稳定客户端为 `1.1.0`，同时存在 `1.2.0-alpha04`。首期生产基线应采用稳定线，除非 ADR 明确接受 alpha 风险。

[同步数据文档](https://developer.android.com/health-and-fitness/health-connect/sync-data) 要求：

- 使用 `clientRecordId` 和 `clientRecordVersion` 支持写入幂等与更新；
- 为不同数据类型分别保存 changes token；
- 持续读取到 `hasMore == false`；
- 处理 UpsertionChange 和 DeletionChange；
- changes token 长期不使用会失效，文档给出的期限为 30 天；
- token 失效时重新读取并去重；
- 前台限制和后台读取权限影响同步策略。

[数据格式文档](https://developer.android.com/health-and-fitness/health-connect/data-format) 包含 UID、data origin、recording method、device、client id/version、时间和 zone offset。[读取文档](https://developer.android.com/health-and-fitness/health-connect/read-data) 说明未取得历史读取权限时存在 30 天范围限制；2026 年设备步数来源归属也发生变化，不能硬编码 `android` 来源包名。

[数据类型文档](https://developer.android.com/health-and-fitness/health-connect/data-types) 显示记录包含 interval、series 等不同形态，进一步支持 LifeChronicle 的 Event/Series 分离。

#### 可借鉴

- Android 健康数据的首选聚合入口；
- 按数据类型管理增量 token；
- 来源、记录方式、设备和客户端版本元数据；
- 更新映射为 correction/supersedes，删除映射为 tombstone；
- series 与离散 record 分开持久化；
- token 失效后的全量窗口重读和稳定去重。

#### 不应继承

- 不把 Health Connect 当作云端备份或永久事实源；
- 不假设所有 wearable 和所有健康类型都存在；
- 不在未处理来源重叠时直接求和；
- 不硬编码来源包名、平台合成来源或设备归属；
- 不只保留 changes token 而忽略定期校验和恢复扫描；
- 不绕过 Android 权限、后台执行和用户撤权语义。

#### LifeChronicle 映射

- `android-health-collector`
- `health-normalizer`
- `health.sync-cursor`
- `health.record`
- `health.series`
- `data-quality`

AndroidX/AOSP 代码通常采用 Apache-2.0，但具体 artifact、sample、平台 API、Google Play 分发政策和文档内容仍须在集成版本确定时分别核实，不能用一般性结论代替依赖清单审计。

## 5. Protobuf 研究发现与已收敛的签名门禁

### 5.1 官方限制

Protobuf 官方文档 [Proto Serialization Is Not Canonical](https://protobuf.dev/programming-guides/serialization-not-canonical/) 明确指出：

- deterministic serialization 不等于 canonical serialization；
- 字节输出可能随 schema 变化、应用构建、构建选项和库实现变化；
- 不同语言实现之间不保证得到相同字节；
- 将序列化结果直接用于长期 hash 或签名是脆弱的。

[Encoding 文档](https://protobuf.dev/programming-guides/encoding/) 进一步说明 deterministic 只保证特定二进制中的稳定输出，字段顺序并非协议级保证。调研时官方实现默认分支为 `main` [`c4be748`](https://github.com/protocolbuffers/protobuf/commit/c4be7483cbb23b9836b2d9e599f37fa1f2c93bed)。

### 5.2 对 LifeChronicle 的结论

Protobuf 只用于：

- 类型化 schema；
- gRPC/批量传输；
- 兼容性检查；
- 代码生成。

Protobuf deterministic serialization 不得定义为跨语言 canonical signing bytes。本次调研发现该风险后，当前
[`event-stream-spec.md`](../protocol/event-stream-spec.md) 已收敛为独立的 `LCB1`
canonical framing，并将 `ES-C004` 改为验证各语言构造的 `LCB1` 签名输入逐字节
一致。因此该问题已经在当前规范中收敛，后续实现和评审必须把它作为不得回退的
契约门禁。

### 5.3 已收敛方案与必须保持的门禁

`LCB1` 独立于 Protobuf serializer，已经固定：

- `ASCII("LCB1")` domain separator 和 signing format version；
- 字段顺序；
- 字符串先通过小写 ASCII/UUID 规范校验，不做语言相关格式化；
- 字符串和字节串长度前缀；
- 固定宽度大端整数和 timestamp 的 seconds/nanos 表示；
- nonce、compression、source 与固定 32 字节 `payload_sha256` 的编码；
- `payload_sha256` 对设备实际发送的 `compressed_items` 字节计算；
- 新签名帧必须使用新魔数和版本，不能原地改变 `LCB1`。

后续 golden vectors 必须验证 `LCB1` canonical framing builder，而不是各语言生成的
Proto serializer。当前事件规范还以 `LCE1` 固定普通 Event 的字段内容帧，以
`LCC1` 绑定服务端认证身份；其中 `Any.value` 保留设备提交的原始 bytes。Series
以 `LCS1/LCR1` 对精确提交 wire bytes 做 domain separation 和身份绑定，并同时
保留压缩 bytes 与解压载荷摘要。`submitted_sha256`、`canonical_sha256` 和
Series checksum 都具有准确输入域；
不能把消息反序列化后再由某个 Protobuf 运行库重新序列化，并期待跨版本、跨语言
hash 永久相同。

## 6. 基础设施参考与职责边界

### 6.1 当前上游快照

| 项目 | 默认分支精确提交 | 许可证 |
|---|---|---|
| Apache Kafka | `trunk` [`123356b`](https://github.com/apache/kafka/commit/123356bad50da1418f42f2f675c963156d4bb78c) | [Apache-2.0](https://github.com/apache/kafka/blob/trunk/LICENSE) |
| Apache Flink | `master` [`9485109`](https://github.com/apache/flink/commit/9485109a4e1eb5b835b99132c5b29a656bed29dc) | [Apache-2.0](https://github.com/apache/flink/blob/master/LICENSE) |
| Temporal | `main` [`91929f4`](https://github.com/temporalio/temporal/commit/91929f4ed78c9e9644e0a97959a75ec0622d3b3b) | [MIT](https://github.com/temporalio/temporal/blob/main/LICENSE) |
| ClickHouse | `master` [`5df3873`](https://github.com/ClickHouse/ClickHouse/commit/5df38732977dd45e46ea3b84c29490e15127835a) | [Apache-2.0](https://github.com/ClickHouse/ClickHouse/blob/master/LICENSE) |
| Apache Iceberg | `main` [`8dfc4b5`](https://github.com/apache/iceberg/commit/8dfc4b5102eb7c35dac631041dcea7bbe07bbec6) | [Apache-2.0](https://github.com/apache/iceberg/blob/main/LICENSE) |
| Open Policy Agent | `main` [`fa9cce7`](https://github.com/open-policy-agent/opa/commit/fa9cce7d6fd44c9cf809057aaf80dbe2cd2d1f86) | [Apache-2.0](https://github.com/open-policy-agent/opa/blob/main/LICENSE) |
| Wasmtime | `main` [`4993061`](https://github.com/bytecodealliance/wasmtime/commit/499306131f8d306ac29fec2c6da982c456c953d4) | [Apache-2.0 WITH LLVM-exception](https://github.com/bytecodealliance/wasmtime/blob/main/LICENSE) |
| OpenTelemetry Specification | `main` [`f62b146`](https://github.com/open-telemetry/opentelemetry-specification/commit/f62b146ecbcb73af4d5a81a2670ff6102741933c) | [Apache-2.0](https://github.com/open-telemetry/opentelemetry-specification/blob/main/LICENSE) |

默认分支不是生产依赖版本。部署契约必须锁定稳定发行版、镜像 digest、配置 schema 和升级/回滚路径。

### 6.2 Kafka

[官方介绍](https://kafka.apache.org/documentation/) 将 Kafka 定义为事件流平台，支持持久存储、实时处理和回溯读取。

LifeChronicle 用途：

- 已验签事件的近期 durable spine；
- 消费者解耦和 replay；
- 按用户/设备/事件族规划 partition key；
- schema、失败队列和滞后监控。

边界：

- Kafka 不是永久对象归档；
- 不能把 topic retention 当成业务保留政策；
- 不能把 Kafka 事务宣传为跨 PostgreSQL、ClickHouse、Iceberg 和外部 API 的全局 exactly-once；
- 对外副作用仍需幂等 key、outbox 或工作流补偿。

### 6.3 Flink

[Flink 流分析文档](https://nightlies.apache.org/flink/flink-docs-stable/docs/learn-flink/streaming_analytics/) 说明 event time、watermark 和有状态流处理；[checkpoint 文档](https://nightlies.apache.org/flink/flink-docs-stable/docs/dev/datastream/fault-tolerance/checkpointing/) 说明 exactly-once 状态语义的配置条件。

LifeChronicle 用途：

- event-time 窗口、乱序和迟到事件；
- sessionization、聚合、异常和质量标记；
- 可重放的 derived event/series；
- checkpoint 与 savepoint 管理。

边界：

- 端到端 exactly-once 还要求可重放 source 和事务性或幂等 sink；
- watermark 规则、allowed lateness 和时钟质量必须进入作业契约；
- Flink 不承担跨日人工流程、删除审批或导出编排。

### 6.4 Temporal

[Temporal 文档](https://docs.temporal.io/temporal) 以 Event History 实现 durable execution；[Workflow Definition](https://docs.temporal.io/workflow-definition) 要求 workflow 代码可确定重放，外部副作用通过 Activity 执行并进行版本治理。

LifeChronicle 用途：

- 导出、删除、重放、回填和长时导入；
- 多阶段审批与补偿；
- 数据产品重建和失败恢复。

边界：

- 不用于逐条高吞吐流处理；
- 不在 workflow history 存大 payload；
- workflow 代码升级必须满足 deterministic replay 和版本兼容；
- Activity 必须可重试或具备幂等/补偿。

### 6.5 ClickHouse

[ClickHouse 官方说明](https://clickhouse.com/clickhouse) 将其定位为列式分析数据库；官方 [OLTP 与 OLAP 说明](https://clickhouse.com/resources/engineering/oltp-vs-olap) 解释了批量、列式分析与事务工作负载的差异。

LifeChronicle 用途：

- 时间范围分析；
- 聚合、维度过滤和仪表盘；
- 高频事件和派生序列的在线分析副本。

边界：

- 不是身份、授权、工作流和最新状态的 OLTP source of truth；
- raw truth 必须仍可由不可变归档恢复；
- 表引擎、排序键、分区、去重和 TTL 必须按查询契约设计。

### 6.6 Apache Iceberg

[Partitioning 文档](https://iceberg.apache.org/docs/latest/partitioning/) 说明 hidden partitioning 和 partition evolution；[Branching and Tagging](https://iceberg.apache.org/docs/latest/branching/) 说明 snapshot、branch、tag 和 time travel。

LifeChronicle 用途：

- 对象存储上的长期 raw/derived 表；
- schema/partition evolution；
- snapshot、审计和批量重放；
- 分析引擎之间共享大规模历史数据。

边界：

- 不承担低延迟交互 API；
- catalog、对象一致性、快照过期、小文件 compact 和删除文件都需运维契约；
- snapshot retention 不能替代法律/用户级删除流程。

### 6.7 OPA

[OPA 文档](https://www.openpolicyagent.org/docs) 将 policy decision 与 enforcement 解耦，并以 Rego 表达 policy-as-code。

LifeChronicle 用途：

- raw、derived、public 三平面的访问决策；
- 导出、公开分享、精确位置和健康数据策略；
- 可测试、可版本化的 policy bundle。

边界：

- OPA 只返回决策，不执行数据裁剪、加密或审计；
- enforcement point 必须 fail closed；
- 输入需最小化，不向策略日志泄露原始敏感 payload；
- 缓存和 bundle 失效策略必须明确。

### 6.8 Wasmtime

[Wasmtime 仓库](https://github.com/bytecodealliance/wasmtime) 和官方 [安全设计说明](https://bytecodealliance.org/articles/security-and-correctness-in-wasmtime) 强调可配置的资源控制和 capability-oriented 嵌入。

LifeChronicle 用途：

- 第三方派生、转换和导出插件；
- 明确 host API；
- 按能力授予读、写、网络和时钟；
- fuel/epoch、内存、输入输出大小和执行时间限制。

边界：

- Wasm sandbox 不是授权策略本身；
- 默认不授予网络、文件系统、环境变量或高精度时钟；
- 插件输出仍需 schema、lineage、隐私和审计检查；
- host call 必须可撤销、可计量并受 OPA 决策约束。

官方在 2026-04-09 发布了 [Wasmtime security advisories](https://bytecodealliance.org/articles/wasmtime-security-advisories)，涉及多个严重问题并给出修复版本。生产必须固定仍受支持且已修复的稳定版本，订阅公告，并通过镜像 digest 和升级演练治理，不能追随默认分支。

### 6.9 OpenTelemetry

[OpenTelemetry 介绍](https://opentelemetry.io/docs/what-is-opentelemetry/) 将其定义为生成、收集和导出 traces、metrics、logs 的 vendor-neutral 可观测性框架，而不是存储后端。

LifeChronicle 用途：

- ingestion、Kafka、Flink、Temporal 和查询链路追踪；
- SLO 所需 metrics；
- 统一 resource 和 correlation id；
- Collector 中的路由、采样和导出。

边界：

- 不把精确位置、健康值、用户文本、token、签名或原始事件放进 span attribute/log；
- 可观测性数据必须有独立的保留、访问和脱敏策略；
- trace id 不是业务 event id 或幂等 key；
- OTel 不提供业务审计账本。

## 7. 综合决策矩阵

复用等级定义：

- **A — 标准/API 采用**：对稳定公共规范或平台 API 建立正式适配。
- **B — 架构模式借鉴**：独立设计与实现，不复制上游表达。
- **C — clean-room 互操作**：仅依据获准的公开协议/行为实现，并保留来源证据。
- **D — 禁止代码复用**：许可证或来源风险较高，当前只允许产品与架构研究。

| 参考 | 主要输入 | 复用等级 | 许可证/来源风险 | 决策 |
|---|---|---:|---:|---|
| ActivityWatch | watcher、heartbeat、时间查询 | B | 中 | 借鉴桌面采集；raw 不合并 |
| Home Assistant Recorder | current/history/statistics、retention | B | 低至中 | 借鉴分层；不复制内部表和总线 |
| OwnTracks Android | 离线队列、位置消息 | C | 中 | 提供兼容入口；转换为自有信封 |
| OwnTracks Recorder | MQTT/HTTP 接收、GeoJSON | D/C | 高 | 不嵌入 GPL 服务端；只做协议互操作 |
| OpenTracks | Android 长时采集、GPX | B | 低至中 | 借鉴生命周期与开放导出 |
| Gadgetbridge | capability/coordinator/support | D/C | 高 | 首期经 Health Connect；直连协议须 clean-room |
| Sleepy | 实时状态、打开应用、公开卡片、插件钩子 | B/D | 中至高 | 借鉴体验和声明式边界；历史、公开面与插件运行时全部重建 |
| Traccar | adapter→Position→handler | B/C | 中 | 借鉴适配器边界；第三方协议逐项审批 |
| Dawarich | 位置时间线、visit/trip、导入 | D/B | 高 | 借鉴 UX 与派生边界；不复制 AGPL 服务端 |
| CloudEvents 1.0.2 | 外部事件上下文与绑定 | A | 低 | 用于网关映射，不取代内部事件信封 |
| Health Connect 1.1.0 | 健康记录、增量同步 | A | 中 | Android 首选入口；保留来源和恢复扫描 |
| Protobuf | schema 与传输 | A（限定） | 低，设计风险高 | 不作为 canonical signing bytes |
| Kafka | 近期事件流与 replay | A/B | 低，运维中 | 不作永久归档或全局事务 |
| Flink | event-time 派生 | A/B | 低，语义中 | 明确 watermark、late data 和 sink 幂等 |
| Temporal | 长时编排 | A/B | 低，升级中 | 不承载逐条流或大 payload |
| ClickHouse | 在线分析 | A/B | 低，数据建模中 | 只作分析副本 |
| Iceberg | 长期历史表 | A/B | 低，运维中 | 管理 catalog、snapshot、compact |
| OPA | 策略决策 | A/B | 低，集成中 | enforcement fail closed |
| Wasmtime | 插件隔离 | A/B | 中至高 | 固定修复版本并最小能力 |
| OpenTelemetry | 可观测性 | A/B | 低，隐私高 | 严格属性白名单与脱敏 |

## 8. 对整体架构和项目契约的输入

### 8.1 采集与设备层

项目契约应固定：

- 每个 agent 具备本地 WAL/Outbox、稳定 event id、设备序列和 ack checkpoint；
- collector/provider 与领域 normalizer 分离；
- 设备能力、权限状态、来源、固件/应用版本、时钟质量可观测；
- 心跳、位置点、健康记录和序列均先保存 raw，再生成会话、行程、停留和聚合；
- Health Connect 和第三方 API 的 cursor/token 是可恢复状态，不是唯一真相。

### 8.2 事件与时间

项目契约应固定：

- `observed_at`、`ended_at`、`received_at`、`ingested_at`、`processed_at` 语义分离；
- 原始事件不可更新，修正使用 correction/supersedes/tombstone；
- Event 与 Series 分离；
- schema、算法、模型、参数和 lineage 均有版本；
- source、device、subject、precision、quality 和 privacy classification 可追溯；
- Protobuf 只负责 schema/transport，签名使用独立 canonical `LCB1` framing。

### 8.3 存储与派生

项目契约应固定：

- PostgreSQL：用户、设备、Registry、策略、工作流，以及独立数据库中的
  Batch/Nonce/ACK 接入协调等事务型控制元数据；不保存原始 Payload 或最新状态；
- Valkey：可重建的 latest-state/cache；ClickHouse 提供历史与分析查询；
- Kafka：已验证事件的近期 durable spine；
- ClickHouse：可重建的在线分析副本；
- Iceberg/object storage：不可变 raw 和长期 derived 历史；
- Flink：event-time 派生；
- Temporal：跨阶段长时编排；
- 所有派生物可按版本重放，并能追溯到输入集合和代码/配置版本。

### 8.4 查询、公开与插件

项目契约应固定：

- raw、derived、public 三平面分别授权；
- 精确位置、健康值和用户文本默认私有；
- OPA 决策与 API enforcement 分离，但必须 fail closed；
- public projection 是显式生成的数据产品，不是对 raw 的过滤视图；
- Wasmtime 插件默认无网络、无文件系统、无环境变量，仅获显式 host capability；
- 插件输入输出经过 schema、资源、隐私和 lineage 检查；
- OpenTelemetry 属性采用白名单，不能成为敏感数据旁路。

### 8.5 持续实施门禁

以下事项是后续实现和发布必须持续满足的门禁：

1. 保持事件流规范的独立 `LCB1`、`LCE1/LCC1`、`LCS1/LCR1` framing，不得回退到
   deterministic Protobuf 签名或 Item 内容摘要；
2. 在首个跨语言上传实现前发布 Go、Rust、Kotlin、Java、TypeScript 五种语言的
   `LCB1`、`LCE1/LCC1` 与 `LCS1/LCR1` golden vectors，并纳入 CI；
3. 保持 hash 字段的准确字节域和压缩前后关系测试；
4. 持续维护 LifeChronicle MIT License、依赖锁和第三方来源清单；
5. 为 AGPL/GPL/MPL/EPL、设备协议和资产设置复用审批门禁；
6. 为 Health Connect、OwnTracks 和 CloudEvents 分别建立适配 ADR；
7. 用端到端故障测试验证 agent 重试、服务端幂等、Kafka replay、Flink late data 和 sink 去重；
8. 固定基础设施稳定版和镜像 digest，不以本文件的默认分支提交直接部署。

## 9. 最终建议

LifeChronicle 不应成为 ActivityWatch、Home Assistant、OwnTracks、Sleepy、Dawarich
或 Traccar 的功能合集，而应将它们验证过的局部模式组合到一个更严格的契约中：

- 设备侧采用 ActivityWatch/OpenTracks/OwnTracks 验证过的本地优先可靠采集；
- wearable 首期经 Gadgetbridge 或厂商应用聚合到 Health Connect；
- 实时状态和公开卡片借鉴 Sleepy 的简洁体验，但历史、公开 projection 和插件隔离
  使用 LifeChronicle 自有契约；
- 位置协议按 Traccar 的 adapter 边界接入，位置产品借鉴 Dawarich 的时间线体验；
- 状态与统计借鉴 Home Assistant Recorder 的分层；
- 外部互操作借鉴 CloudEvents，内部仍使用自有隐私与审计信封；
- raw 普通事件先进入 Kafka 近期持久日志，再归档到不可变 Iceberg Bronze；Series
  由对象存储二进制与 Kafka metadata 共同承载，二者满足持久条件后才确认；
- derived 由 Flink 生成实时规范化、会话、质量和聚合，由 Temporal 编排历史重放、
  回填、切换与删除；Kafka 承载中间事件，ClickHouse 是可重建热读模型，Iceberg
  Silver/Gold 保存版本化历史，Valkey 只保存可重建 latest-state/cache；
- public 只接收 Projection Pipeline 从获准私有派生结果生成的强 Schema 最小快照：
  OPA 负责决策，业务投影执行字段 allowlist、隐私转换、延迟和最小样本检查，结果
  写入物理隔离的 Public Snapshot Store，再由 Public API/CDN 只读发布；
- Wasmtime 是受 capability、资源和输出 schema 约束的插件执行边界，不是 raw、
  derived 或 public 的事实存储，也不得绕过投影和物理隔离；
- 所有实现受不可变 raw、可重放派生、最小权限和第三方来源治理约束。

本文件是选型和契约的研究输入，不是第三方代码复用许可，也不是生产版本批准。每次采用上游接口、依赖或实现前，必须重新核实当时的稳定版本、许可证、公告和平台政策。
