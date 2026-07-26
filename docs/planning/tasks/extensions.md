# 方向 C：扩展功能

覆盖阶段 9–12，共 30 个工作包。在基本可用 MVP 上增加健康/高频数据、公开窗口、
插件平台和外部数据源。

## 阶段 9：健康与高频序列

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | E9-01 | Series Registry、通用编解码与提交 ADR | 心率、RR、GPS、IMU Stream；固定/不规则 Chunk、Channel、缺失值、Clock、zstd、checksum；在 E9-02 实现前接受 Series 对象→Kafka metadata→ACK ADR，冻结对象 Key/布局、幂等协调状态、故障窗口和安全期孤儿回收 | 五语言黄金 Chunk round-trip 与 `LCS1/LCR1`/对象引用向量通过；单位/轴/时间一致；损坏摘要失败；ADR 明确对象先持久、metadata 后持久、最后 ACK，并覆盖崩溃恢复与回滚；`ES-C018` 通过 | M8-08 |
| [ ] | E9-02 | Agent Chunk Outbox 与双传输接入 | Chunk Builder/Outbox、gRPC/HTTPS、multipart；Ingestion 校验后幂等写对象、持久记录协调状态、发布 Kafka metadata，双持久后逐项 ACK；exact retry、Chunk ID 冲突、ACK 结果未知恢复、故障窗口、对账/安全期孤儿回收和普通 Topic 隔离 | 对象写入前后、metadata 发送前后、Kafka ACK 未知及 ACK 状态持久化前崩溃均不错误清理；重试不换 ID/对象；同 ID 异内容冲突；普通 Event Topic 零 Series；`ES-C015/ES-C018` 通过 | E9-01 |
| [ ] | E9-03 | Series 下游归档、Parquet 和热采样 | Series Archiver 消费 `RawSeriesRecord`，校验 metadata/object 引用与 checksum，提交 Iceberg metadata，执行损坏隔离、Parquet 转换/统计和 ClickHouse 心率/位置热表 | 已确认对象与 metadata 一一对应；缺失/损坏引用进入隔离且可重放；下游重启不漏不重；Parquet round-trip 和热表幂等 | E9-02 |
| [ ] | E9-04 | 高频特征、聚合和保留 | 心率分钟/小时、IMU 秒/分钟/小时、GPS 降采样、缺失检测、时钟校正派生、TTL、原始删除保留特征 | 聚合与参考实现一致；原始时间不改；删除原始后批准特征仍可查询 | E9-03 |
| [ ] | E9-05 | Series 范围查询与图表 | 时间/Stream 分区裁剪、按显示宽度降采样、心率/GPS/IMU 图表 | 只扫描命中 Parquet；UI 不加载全量样本；缺口不错误连线 | M6-07、E9-04 |
| [ ] | E9-06 | 普通健康数据处理 | steps/sleep session/stage Registry、Health Normalization、来源融合、睡眠重叠/修订、ClickHouse/Iceberg Sink | 多来源步数不直接相加；睡眠无非法重叠；重放无重复 | M8-05、E9-03 |
| [ ] | E9-07 | 健康指标、API 和私有页面 | 步数小时/每日、睡眠总量/阶段、Health API、multiple-step/sleep-overlap 回放、今日健康摘要 | 一天/月/年含单位/来源/覆盖率/版本；到达顺序不影响结果 | E9-04–E9-06 |
| [ ] | E9-08 | 高频健康性能与故障验收 | 一天 50Hz 三轴 IMU、重复/损坏 Chunk、裁剪、TTL、删除/保留、迟到回放 | `make series-test`；吞吐/内存/存储在预算内且所有正确性场景通过 | E9-01–E9-07 |

阶段 9 完成命令：`make phase-9-gate`

## 阶段 10：公开投影

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | E10-01 | Public ADR、Card Schema 与 OPA 隐私规则 | 以“实现开始前已接受”为入口门完成 Public Snapshot Store 与 Projection Topic ADR/契约；Card/Projection Schema、默认私有、字段 allowlist、规则版本和失败关闭 | ADR 冻结物理隔离、强 Schema、原子发布、撤销、延迟队列及 Topic Key/Value/Producer/Consumer/保留/迁移（或明确复用现有 Topic）；新 Stream/新 Card 默认无输出，未知字段和非法规则被拒绝 | M8-08 |
| [ ] | E10-02 | Projection Pipeline 与隐私转换 | Kafka Consumer、字段删除、类别替换、取整、时间模糊、地点降精度、扰动、最小样本、时段/频率、Delay Queue | 固定输入/规则可重放；未到延迟不可读；少样本不发布 | E10-01 |
| [ ] | E10-03 | Public Snapshot、API 和分享 | 物理隔离 Snapshot Store、强 Schema 写入、Public API、Share Token、Profile/Card 开关 | API 镜像无私有驱动/凭据；只读有效 Snapshot；撤销立即失败 | R1-07、E10-02 |
| [ ] | E10-04 | 公开页面、CDN 和首批卡片 | SvelteKit、CDN；在线、活动类别/分布、设备、媒体、自定义文本；为后续步骤/睡眠卡片保留同一强 Schema 集成点 | 卡片默认关闭；页面只调 Public API；缓存按到期/撤销清除；基础卡片不等待阶段 9 | E10-03 |
| [ ] | E10-05 | 公开运维与安全响应 | 紧急暂停、访问审计、API 限流、CDN purge 和可观测性 | 暂停/撤销在目标时间内覆盖 API/CDN；审计无响应敏感内容 | E10-03、E10-04 |
| [ ] | E10-06 | 物理隔离与隐私攻击测试 | Public Namespace 网络扫描、未声明字段 fuzz、组合恢复精确位置、跨 profile 缓存、默认私有回归 | 私有 PG/CH/Kafka/Iceberg/MinIO 全不可达；不能恢复禁止精度 | E10-01–E10-05 |
| [ ] | E10-07 | 公开投影端到端与负载验收 | 私有事实→OPA→转换/延迟→Snapshot→API/CDN→页面；热门/冷门卡片流量 | `make public-projection-test`；功能、隔离、撤销、暂停、负载和威胁模型通过 | E10-01–E10-06 |

步骤/睡眠公开卡片是 `E10-04` 的后续细粒度集成项，单独依赖 `E9-07` 的健康指标/API
输出；它不阻塞阶段 10 基础公开投影门，也不允许绕过 Projection/Snapshot 通路。

阶段 10 完成命令：`make phase-10-gate`

## 阶段 11：插件平台

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | E11-01 | WIT、Manifest 与插件包契约 | 生命周期、Stream Reader/Writer、Transformer、Analyzer、Importer/Exporter、Card、Notification WIT；Manifest Schema 和包格式 | WIT/Manifest lint/breaking；路径穿越、未知权限、非法资源和错 API 版本失败 | M8-08 |
| [ ] | E11-02 | 插件签名、信任和生命周期管理 | 签名/信任库、安装、启用、停用、卸载、权限 Grant 和审计 | 篡改/未知/撤销包不能安装；停用立即阻止新运行；历史血缘保留 | E11-01 |
| [ ] | E11-03 | Wasmtime Host 与 Capability Broker | Rust Host、不可伪造 handle、Stream 读写授权、输出 Schema、结果 Lineage | 插件只能读写 Manifest 与用户共同授权的 Stream；非法输出不入主干 | E11-01、E11-02 |
| [ ] | E11-04 | 插件沙箱与故障隔离 | 内存、Fuel/CPU、超时/取消、网络/文件/env/数据库默认关闭、崩溃隔离、脱敏日志 | 恶意 Component 无外部访问；超限/崩溃不影响 Host 和其他插件 | E11-03 |
| [ ] | E11-05 | SDK、脚手架和示例插件 | Rust SDK、项目生成/构建/测试/打包；Transformer、Analyzer、流式 Exporter 示例 | 一条命令生成可签名包；示例输出确定、Schema 合法、内存有界 | E11-01–E11-04 |
| [ ] | E11-06 | 插件回填、升级和公共卡片集成 | Temporal Backfill、隔离输出/比较/切换、版本迁移/回滚；受限前端 Card 协议和示例通过阶段 10 的 Public Snapshot/API 集成 | 升级可重放；版本差异可追踪；前端无任意脚本/私有 API 权限；插件 Card 不绕过 Projection/Snapshot | M5-06、E10-03、E11-03、E11-05 |
| [ ] | E11-07 | 插件安全与端到端验收 | 未授权读取、逃逸、资源耗尽、崩溃、plugin-version-change 回放和供应链威胁模型 | `make plugin-test`；所有越权失败，Host 稳定，升级回放确定 | E11-01–E11-06 |

阶段 11 完成命令：`make phase-11-gate`

## 阶段 12：位置、穿戴和外部数据源

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | E12-01 | Health Connect 穿戴融合 | 步数、睡眠、心率、HRV、血氧、运动映射；物理设备关联和跨来源去重 | 厂商/Gadgetbridge/Health Connect 同事实不重复且原始来源全保留 | E9-08 |
| [ ] | E12-02 | Gadgetbridge 与厂商 API 接入 | Bridge 契约/增量游标、一个参考厂商 OAuth/限流/分页连接器、更新/删除 | 断连/429/Token 失效后游标不越页，恢复无漏无重 | E12-01 |
| [ ] | E12-03 | Wearable Provider API 与 BLE | capability、配对、历史游标、固件差异、标准映射、扫描/连接；强制宿主 WAL/Outbox | Provider 无独立上传/文件/凭据能力；断连续传；未知固件安全拒绝 | E11-07、E12-01 |
| [ ] | E12-04 | Location Collector 与本地隐私 | 权限/精度/频率/后台状态、Android 位置采集、围栏排除/降精度/暂停 | 精确点含来源/精度/时间；被过滤点不进入 WAL/Outbox | M8-08 |
| [ ] | E12-05 | Visit、Trip、Geofence 与历史回放 | 规则/状态机、漂移/缺口/乱序/跨午夜、Lineage、迟到 Range Replay | 原始点不修改；派生可确定性重建、比较、切换和回滚 | M5-06、E12-04 |
| [ ] | E12-06 | 导入器与外部 Gateway | OwnTracks、GPX、GeoJSON、Traccar、智能家居、MQTT 身份/转换/QoS 重投递 | 所有源转统一事件并保留 Origin；伪造身份/坏 Schema 失败；重复不生新事实 | E12-04 |
| [ ] | E12-07 | 位置存储、查询、地图和导出 | ClickHouse/Iceberg point/visit/trip/geofence、Sink、Location API、私有地图/时间线、GeoJSON/GPX 导出 | 热/历史无重；只查可视范围；原始点与派生区分；导出可恢复且含血缘 | M6-07、E12-05、E12-06 |
| [ ] | E12-08 | 外部源、位置隐私和删除验收 | 多来源穿戴/位置去重、BLE 宿主隔离、派生回放、API 限流恢复、MQTT 安全、精确位置保留/删除、Public 隔离 | `make integration-source-test`；来源可追踪、行程可重建、删除覆盖全层且私有位置不可恢复 | E12-01–E12-07 |

阶段 12 完成命令：`make phase-12-gate`

阶段 12 完成即达到完整扩展平台目标。
