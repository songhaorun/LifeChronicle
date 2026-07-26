---
adr: ADR-013
title: 阶段 0 机器契约与工具使用独立顶层边界
status: accepted
date: 2026-07-27
owners:
  - architecture
reviewers:
  - bootstrap-maintainer
supersedes: []
superseded_by: null
related:
  - ../contract/project-contract.md
  - ../protocol/event-stream-spec.md
  - ../planning/tasks/foundation-and-contracts.md
---

# ADR-013：阶段 0 机器契约与工具使用独立顶层边界

## 状态

Accepted。该决策建立阶段 0 的 Registry、生成配置、生成产物、跨语言验收包和仓库门禁
边界；它不改变后续业务模块的所有权。

## 上下文

原始顶层模块表已经区分 `proto/`、业务模块、基础设施、通用测试和文档，却没有为阶段 0
实际产生的机器 Registry、手写生成配置、可发布生成产物、跨语言黄金验收包和根门禁脚本
分配稳定位置。将这些内容塞入 `proto/` 会混淆权威输入、构建逻辑与生成输出；放入
`tests/` 又会让发布物和通用测试职责交叉。

## 决策

- `registry/` 保存 Stream/Metric Registry 元 Schema、登记项、模板和合法/非法夹具；
- `codegen/` 保存各语言手写生成与编译配置、依赖锁和最小构建入口；
- `generated/` 只保存从同一 Proto 契约生成并由重复生成检查保护的五语言产物；
- `contract-tests/` 保存 breaking 基线、黄金向量、语义测试和五语言 runner；
- `scripts/` 保存仓库级 lint、生成、兼容、扫描和阶段门实现；
- `tests/` 继续作为阶段 1 以后跨模块集成、性能、隐私、故障和恢复测试边界；
- 权威依赖方向固定为 `proto/` 与 `registry/` → `generated/` → 各语言 runner/业务模块，
  `codegen/` 和 `scripts/` 只能编排，不得成为运行时业务依赖。

## 备选方案

1. **全部放入 `proto/`。** 会混合手写权威输入、工具配置、测试和生成输出，拒绝。
2. **全部放入 `tests/`。** Registry 和生成 SDK 是可发布契约，不是测试私有实现，拒绝。
3. **建立单一 `contracts/` 聚合目录。** 目录层级更整齐，但会同时迁移现有权威路径并
   扩大阶段 0 变更面；保留为未来有明确迁移收益时的新 ADR 候选。

## 后果

正面后果：

- 权威输入、手写构建配置、生成产物和验收证据可以分别审查；
- `CODEOWNERS`、CI 缓存和发布规则能按职责精确限定；
- 通用 `tests/` 不需要承载生成 SDK 或 Registry 发布语义。

负面后果：

- 顶层目录数量增加，文档和所有权表必须同步维护；
- 跨语言契约变更通常会同时触及多个边界；
- 必须用自动化防止手工修改 `generated/` 或把本地缓存提交进仓库。

## 迁移

1. 在项目契约和 `CODEOWNERS` 中登记五个边界；
2. 将阶段 0 Registry、生成配置、产物、验收包和门禁放入对应目录；
3. 在统一阶段门中验证目录、依赖方向、重复生成和缓存忽略规则；
4. 后续若聚合为 `contracts/`，必须提供兼容路径、发布坐标迁移和回滚方案。

## 回滚

若边界导致构建或所有权不可维护，先保留旧发布坐标和 CI 入口，再以新 ADR 选择目标
结构并逐步移动；迁移期间同时验证旧、新路径。不得通过删除 breaking 基线、黄金向量
或生成历史来简化回滚。

## 测试

- `scripts/check_governance.py` 检查项目契约与 `CODEOWNERS` 的顶层边界；
- `scripts/verify_codegen.ps1` 验证五语言生成、编译和重复生成无差异；
- `scripts/run_cross_language_contract_tests.py` 从 `contract-tests/` 运行五语言黄金包；
- `scripts/scan_repository.py` 拒绝纳入本地工具、缓存、Secret 和未登记顶层目录；
- `make phase-0-gate` 聚合上述检查，任一失败返回非零。

## 退出条件

- 五个目录在项目契约、ADR 和 `CODEOWNERS` 中一致登记；
- 生成产物可追溯到 `proto/` 和锁定工具链，重复生成无差异；
- Registry 和黄金验收包通过机器校验，五语言输出一致；
- 本地缓存、工具下载和构建目录不进入版本控制；
- 统一阶段门从仓库根目录可重复运行。
