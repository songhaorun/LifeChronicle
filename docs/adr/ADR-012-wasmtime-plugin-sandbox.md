---
adr: ADR-012
title: Wasmtime Component Model 作为第三方服务端插件边界
status: accepted
date: 2026-07-27
owners:
  - architecture
reviewers:
  - bootstrap-maintainer
supersedes: []
superseded_by: null
related:
  - ../architecture/overall-architecture.md
  - ../contract/project-contract.md
  - ../planning/development-roadmap.md
---

# ADR-012：Wasmtime Component Model 作为第三方服务端插件边界

## 状态

Accepted as the server-side untrusted plugin boundary。插件平台在阶段 11 启用；WIT、
Manifest 和包格式需在 E11-01 冻结。

## 上下文

LifeChronicle 需要允许 Transformer、Analyzer、Importer/Exporter、Card 和 Notification
扩展，但插件可能不可信、崩溃、耗尽资源或尝试读取未授权数据。把第三方代码作为宿主
动态库或进程内原生插件运行，会继承宿主文件、网络、环境变量和数据库权限。

## 决策

- 第三方服务端插件以 WebAssembly Component 运行，Rust Plugin Host 使用 Wasmtime
  Component Model。
- WIT 定义稳定 Host/Guest 接口；Manifest 声明插件版本、API 版本、所需 Stream、
  操作、资源预算和输出 Schema。
- 插件默认无网络、文件系统、环境变量、时钟自由度或数据库权限。所有外部交互只能
  通过 Capability Broker 发放的不可伪造句柄。
- Capability 同时受 Manifest 声明、用户授权、OPA 决策和资源所有权约束；插件只能
  读写获准 Stream，输出必须通过 Registry/Schema、隐私和 Lineage 校验。
- Host 对每实例设置内存、Fuel/CPU、超时、取消和并发限制；插件崩溃不得影响 Host
  或其他实例。
- 插件包签名、信任、安装、启停、撤销和升级有审计；输出携带插件/处理器版本并可
  隔离回填、比较、切换和回滚。

## 备选方案

1. **原生动态库插件。** 与宿主同权限且 ABI/崩溃边界脆弱，拒绝。
2. **任意容器插件。** 隔离更重且接口、数据授权和本地部署复杂，暂不作为默认；
   特殊高信任工作负载需新 ADR。
3. **脚本解释器嵌入。** 沙箱、资源限制和供应链边界不够稳定，拒绝作为第三方边界。

## 后果

正面后果：

- 跨语言组件共享 WIT 契约且默认最小能力；
- 资源耗尽和崩溃可在实例边界隔离；
- 权限、输出 Schema 和 Lineage 可统一审计。

负面后果：

- WIT、Component Model 和 Wasmtime 升级形成兼容承诺；
- 插件不能直接复用任意 OS/网络库，需要 Host Capability；
- 沙箱不能代替宿主漏洞修复、供应链签名和输出验证。

## 迁移

1. 在 E11-01 冻结 WIT、Manifest、包和 API 版本规则；
2. 为可信内置扩展先实现等价 Component，比较输入、输出和资源；
3. 以默认零 Capability 安装，逐项授予测试 Stream 权限；
4. 对历史回填写隔离输出，审批后切换版本；
5. 旧原生/脚本插件排空后撤销宿主权限和加载路径。

## 回滚

停用或撤销新插件版本，阻止新实例启动，取消在途执行并切回上一签名版本。保留旧
输出和 Lineage 以供比较，不删除历史证据。Host 升级失败时回滚 Wasmtime/SDK 版本，
不得为兼容旧插件临时开放网络、文件或数据库。

## 测试

- `E11-01`：WIT/Manifest lint、breaking、路径穿越和 API 版本拒绝；
- `E11-02/E11-03`：签名、信任、不可伪造 Capability、Stream 授权和输出 Schema；
- `E11-04`：网络/文件/env/数据库禁路、内存/Fuel/超时和崩溃隔离；
- `E11-06/E11-07`：升级回放、版本切换、逃逸、资源耗尽和供应链威胁模型；
- `ARC-016`：插件默认无外部能力且故障不扩散。

## 退出条件

- WIT、Manifest 和包格式有版本、breaking 检查和生成 SDK；
- 无 Capability 的恶意 Component 无法访问网络、文件、环境变量、数据库或 Stream；
- 未授权句柄、伪造句柄和非法输出均在进入主干前失败；
- 内存/Fuel/超时/崩溃测试不影响 Host 和其他插件；
- 插件签名、授权、运行、输出 Lineage、停用、撤销和升级均可审计与回滚。
