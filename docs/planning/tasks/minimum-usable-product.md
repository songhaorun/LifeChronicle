# 方向 B：基本可用

覆盖阶段 3–8，共 40 个工作包。完成后形成真正可使用的纵向闭环：

```text
Windows / Android
→ WAL / Outbox / 签名批次
→ gRPC / HTTPS
→ Kafka / Iceberg Bronze
→ Flink 会话
→ ClickHouse
→ 私有查询和时间线
```

## 阶段 3：接入与原始归档

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M3-01 | Ingestion Service 与双传输入口 | Go 服务、配置、gRPC 双向流、HTTPS 批量端点、请求大小和超时；阶段 3 只启用普通 Event 接入，Series 功能门关闭并失败关闭 | 两种传输使用同一处理管线；断连重试保持 Batch/Event ID；任一传输提交 Series 均在 Kafka/对象存储前得到稳定拒绝且不产生 ACK 持久证据 | R2-06 |
| [ ] | M3-02 | 安全解码与契约校验管线 | Token/撤销、hash/签名、Nonce、zstd 安全解压、Proto、Registry、Payload、服务端 user 注入 | 错认证、压缩炸弹、篡改和错 Schema 在 Kafka 前失败且不 panic | M3-01 |
| [ ] | M3-03 | 幂等、顺序、时钟和逐项确认 | 普通 Event 的 Batch/Event ID 幂等和内容冲突、Sequence、Clock quality、持久逐 Item ACK 证据 | 同 ID 同内容不重复，不同内容冲突；只有 Kafka 持久 ACK 且逐项终态证据已可靠落库，或能证明同内容已满足相同持久边界的可靠 Duplicate，才返回可清理终态 | M3-02 |
| [ ] | M3-04 | Kafka 原始事件发布与运行保护 | 规范 Key、RawEventRecord、幂等 Producer、错误 Topic、限流、Metric、Trace | `acks=all`；Kafka/ISR 故障时零错误确认；日志/Trace 无原 Payload | R1-03、M3-03 |
| [ ] | M3-05 | Iceberg Bronze 原始归档 | Bronze Schema、Kafka Source、offset/snapshot 原子提交、Sink 恢复、Event→文件追踪 | Sink 任意提交点崩溃后不漏不重；可从 event/batch 定位 offset、snapshot 和文件 | R1-04、M3-04 |
| [ ] | M3-06 | 接入性能与故障验收 | 模拟 Agent、10k 单批、30 天补传、Kafka 故障、Bronze 故障和双传输一致性 | `make ingestion-test`；30 天 observed_at 保真，最终 Bronze ID 集合完整无重 | M3-01–M3-05 |

阶段 3 完成命令：`make phase-3-gate`

## 阶段 4：实时处理

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M4-01 | Flink 公共事件时间运行库 | Raw 反序列化、Registry 快照、observed_at、Watermark、idleness、Event ID 去重、迟到侧输出、Lineage | 坏记录不崩 Job；空闲分区不阻塞；超窗事件零静默丢失 | M3-06 |
| [ ] | M4-02 | 应用规范化与最新状态 | Windows/Android/未知平台应用规范化、Normalization Job、Latest State Job/Topic | 原始标识保留；相同应用稳定映射；latest 只按 observed_at/revision 更新 | M4-01 |
| [ ] | M4-03 | Application Session 处理器 | 状态机、新应用、屏幕关闭、Idle、关机/结束、Heartbeat 超时、稳定 session ID/revision | 正常、乱序、重复和跨零点数据得到同一无重叠会话集合 | M4-01、M4-02 |
| [ ] | M4-04 | Idle、Screen 与 Presence 会话 | Idle/Screen Session、Presence 融合优先级和三个输出 Job | 乱序输入仍生成稳定区间；不同设备不误合并；完整 Lineage | M4-01、M4-03 |
| [ ] | M4-05 | 数据质量与热/历史 Sink | Sequence/Clock/开放会话质量规则；ClickHouse DDL/Sink；Iceberg Silver Schema/Sink | Sink 重启和重复回放后逻辑 ID/revision 集合一致；finding 可查询 | R1-04、M4-02–M4-04 |
| [ ] | M4-06 | 状态恢复、升级与确定性回放 | Checkpoint、故障恢复、Savepoint、升级/回滚；normal/duplicate/out-of-order/clock/timezone/reset/late 数据集 | `make streaming-test`；同输入重复回放逐字段一致，升级失败可恢复旧 Job | M4-01–M4-05 |

阶段 4 完成命令：`make phase-4-gate`

## 阶段 5：历史工作流

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M5-01 | Temporal Workflow 公共运行库 | Worker、metadata、幂等 Activity、heartbeat/游标、重试、取消、补偿、人工审批 | Worker 重启从进度继续；副作用不重复；失败可补偿并审计 | M4-06 |
| [ ] | M5-02 | Range Replay、Aggregate Rebuild 与输出版本 | 分片、Iceberg snapshot 固定、隔离输出、结果比较、审批切换、原子回滚、应用重分类 | 回放不影响线上结果；切换只见单版本；失败可回旧版本 | M5-01 |
| [ ] | M5-03 | Account Export 与 Historical Import | JSONL/CSV/Parquet、固定 snapshot、流式/分片上传、续传、Manifest；幂等历史导入框架 | 百万记录有界内存；中断恢复；导出保留 Schema/来源/单位/血缘/修正 | M5-01、M5-02 |
| [ ] | M5-04 | 全存储删除与 Retention | Kafka Tombstone、ClickHouse、Iceberg/对象、缓存/索引、公开快照、导出缓存、备份到期 | 任一步失败不静默部分成功；返回备份彻底清除时间；dry-run 与实际计划一致 | M5-01 |
| [ ] | M5-05 | Backup Verification 与工作流运维 | 隔离恢复 Workflow、状态 API/页面、取消、审批、进度、错误和输出版本 | 能从页面追踪/取消/审批；备份以实际恢复成功为准 | M5-01、M5-03、M5-04 |
| [ ] | M5-06 | 历史工作流故障验收 | 在回放、导出、删除、保留和恢复中重启 Worker/下游 | `make workflow-test`；全部 Workflow 可恢复，幂等且结果可追踪 | M5-01–M5-05 |

阶段 5 完成命令：`make phase-5-gate`

## 阶段 6：私有查询与控制台

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M6-01 | Query Service 公共能力 | Go 服务、公共契约、OIDC/OPA、签名 Cursor、资源预算、超时/取消、来源/血缘元数据 | Cursor 不可伪造/跨用户；超预算和取消释放下游；授权失败关闭 | R2-06、M5-06 |
| [ ] | M6-02 | 热冷数据适配与查询路由 | ClickHouse、Control/Identity API、Valkey 只读前缀适配；受控 Cold Query Worker 读取 Iceberg；热/冷/跨边界路由和去重合并 | Private API 不直连 PostgreSQL、Catalog 或对象存储；热请求不扫 Iceberg；冷查询不占实时 Flink worker；跨边界有序无重 | M6-01 |
| [ ] | M6-03 | 时间线和核心实体 API | Timeline、多轨道、设备/Stream/原始派生筛选；Raw Event、Application、Device API | 一天/月/年结果正确；来源、设备、Schema、处理版本完整 | M6-02 |
| [ ] | M6-04 | 统计、血缘、质量和任务 API | Statistics、Lineage、Data Quality、处理版本比较、Export Workflow API | 统计可追踪输入；导出只调度 Temporal；质量问题有修复类型 | M5-02、M5-03、M6-02 |
| [ ] | M6-05 | 私有 Web 与核心浏览体验 | SvelteKit、OIDC 会话、类型安全 Client、今日页面、多轨道时间线、筛选 | Token 不进 URL/日志；大时间线虚拟滚动无漏项；页面显示来源/质量 | M6-03、M6-04 |
| [ ] | M6-06 | 数据和设备管理体验 | 原始事件安全查看、设备撤销/轮换、版本比较/审批、导出进度 | 敏感操作二次确认且关联审计；原始数据默认脱敏；刷新后长任务状态保留 | M6-04、M6-05 |
| [ ] | M6-07 | 查询性能与端到端验收 | Metric/Trace/Audit、并发冷热查询、权限矩阵、一天/月/年和 UI 主路径 | `make query-test`；一年冷查询不阻塞 latest-state；无 Payload 进入观测后端 | M6-01–M6-06 |

阶段 6 完成命令：`make phase-6-gate`

## 阶段 7：Desktop Agent

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M7-01 | Rust Core 与可靠本地存储 | Workspace、Provider Trait、SQLite/SQLCipher ADR/迁移、append-only WAL、恢复扫描、索引、Outbox、ID/Sequence | 强杀后已写事实仍在；损坏尾隔离；索引可重建；状态转换事务正确 | R0-06、M3-06 |
| [ ] | M7-02 | 批次签名与可靠同步 | Batch/zstd/Keystore/Ed25519、gRPC、HTTPS 降级、退避、逐项 ACK 和 Outbox 清理 | 黄金签名通过；exact retry 逐字节复用原 `batch_id`、nonce、`compressed_items`、hash 和 signature；仅有可靠持久证据的 Accepted/Duplicate 清理 | R2-06、M7-01 |
| [ ] | M7-03 | 配置、隐私、生命周期和诊断 | Schema Cache、Clock、原子配置、本地隐私规则、Provider 监督、heartbeat/end、服务安装、自启动、损坏恢复、脱敏诊断、签名更新回滚 | 敏感原值不进入可上传存储；单 Provider 崩溃不影响其他采集；坏更新可回滚 | M7-01、M7-02 |
| [ ] | M7-04 | Windows Provider 完整实现 | 前台应用/可选标题、Idle、锁定/解锁、电量/电源、网络、媒体 | 各 Stream 契约通过；标题默认关闭并经隐私过滤；权限不足形成质量状态 | M7-03 |
| [ ] | M7-05 | macOS Provider 基线 | 宿主/权限诊断、前台、Idle、电源和生命周期 | 授权/撤销不阻塞其他 Provider；三类事件时间和来源正确 | M7-03 |
| [ ] | M7-06 | Linux Provider 基线 | X11、GNOME、KDE、wlroots 探测和支持 Provider | 只启动匹配 Provider；不支持时明确诊断；支持环境契约测试通过 | M7-03 |
| [ ] | M7-07 | Desktop 长运行与离线验收 | Windows/macOS/Linux 7 天、断网 7 天、服务不可用、Provider 崩溃、数据库损坏 | `make desktop-agent-test`；持续采集、恢复补传无漏无重，隐私和隔离成立 | M7-01–M7-06 |

阶段 7 完成命令：`make phase-7-gate`

## 阶段 8：Android Agent 与 MVP 闭环

| 状态 | ID | 工作包 | 包含范围 | 验证与验收 | 依赖 |
| --- | --- | --- | --- | --- | --- |
| [ ] | M8-01 | Android Core、本地存储和身份 | Kotlin 多模块、Room/SQLCipher ADR/迁移、WAL、Outbox、ID/Sequence、Batch/zstd、Android Keystore | 进程强杀/迁移后数据保留；签名黄金向量通过；私钥不可导出 | R0-06、R2-06、M3-06 |
| [ ] | M8-02 | Android 同步引擎 | WorkManager、gRPC 实时、HTTPS 降级、退避、逐项 ACK 和清理 | 前后台/受限网络/进程死亡后可续传；exact retry 逐字节复用原 `batch_id`、nonce、`compressed_items`、hash 和 signature | M8-01 |
| [ ] | M8-03 | 应用使用双来源采集与融合 | UsageStats 权限/增量/历史游标；Accessibility 生命周期/实时采集；融合真值表 | 两来源互相校正但原始来源都保留；乱序最终会话符合真值 | M8-01 |
| [ ] | M8-04 | 设备状态、权限和厂商限制 | 屏幕/锁定、电量/充电、网络、统一权限状态机、Doze/后台/自启动诊断 | 权限和限制变化形成质量状态；解除限制自动补传 | M8-02、M8-03 |
| [ ] | M8-05 | Health Connect 增量同步基线 | 可用性/权限、Changes Token、分页增量、来源、更新、删除、步数多来源规则 | Token 与提交原子推进；重复同步无重；更新/删除用修正/Tombstone | M8-01、M8-02 |
| [ ] | M8-06 | Android 生命周期、隐私和维护 | 重启恢复、前台服务、本地隐私、脱敏诊断、签名更新回滚 | 重启恢复 WAL/Outbox/游标；敏感原值不入存储；坏更新可回滚 | M8-02–M8-05 |
| [ ] | M8-07 | Android 稳定性验收 | 重启、后台限制、权限变化、Health 幂等、7 天运行、断网 7 天 | `make android-agent-test`；无漏无重，权限 finding 可闭环 | M8-01–M8-06 |
| [ ] | M8-08 | 双端 MVP 纵向闭环 | Windows/Android→接入→Kafka/Bronze→Flink→ClickHouse→Query→私有时间线；导出和删除冒烟测试 | 一次自动化场景能从设备事实追踪到 UI 和 Iceberg 文件；断网重试、回放、导出、删除成立 | M3-06、M4-06、M5-06、M6-07、M7-07、M8-07 |

阶段 8 完成命令：`make phase-8-gate`

阶段 8 完成即达到“基本可用 MVP”。
