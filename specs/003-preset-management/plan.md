# Implementation Plan: 配置预设与批量管理

**Branch**: `003-preset-management` | **Date**: 2026-03-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-preset-management/spec.md`

## Summary

为 AgentSwitch 添加配置预设和批量管理功能，支持保存命名配置组合、批量操作工具配置，以及配置的导入导出。

**技术方案**:
- 使用 TOML 格式存储预设配置（~/.agentswitch/presets.toml）
- 使用 JSON 格式导出配置（便于跨平台分享）
- 使用 rayon 库实现并发批量操作
- 基于现有的 AgentAdapter 和 ModelConfig 系统扩展
- 实现事务模式确保原子性操作和自动回滚

**主要组件**:
1. 预设管理模块（src/presets/）
2. 批量操作模块（src/batch/）
3. 导入导出模块（src/io/）
4. CLI 命令扩展（src/cli/commands.rs）

## Technical Context

**Language/Version**: Rust 最新稳定版 (edition = "2024")
**Primary Dependencies**: clap 4.x, serde, serde_json, toml, anyhow, dirs, colored, **rayon 1.10**（新增）
**Storage**: 配置文件 (~/.agentswitch/presets.toml) + 现有配置系统
**Testing**: cargo test (单元 + 集成测试)
**Target Platform**: Linux, macOS, Windows (跨平台 CLI 工具)
**Project Type**: cli (命令行工具)
**Performance Goals**:
- 创建并应用预设（3 个工具）< 30 秒
- 批量切换（5 个工具）< 10 秒
- 导出预设（10 个预设）< 5 秒
- 导入预设（5 个预设）< 15 秒
**Constraints**:
- 无网络依赖（本地操作）
- 离线可用
- API Key 必须脱敏后导出
**Scale/Scope**:
- 单用户工具
- 支持 10+ 个 Code Agent 工具
- 支持 100+ 个预设

## 宪章合规性检查 (Constitution Check)

*GATE: 在 Phase 0 研究前必须通过。Phase 1 设计后再次检查。*

### 强制要求（来自 .specify/memory/constitution.md）

- [x] **Rust 2024 标准**: 代码使用 edition = "2024"，通过 `cargo clippy` 无警告
  - **计划**: 所有新代码遵循 Rust 2024 Edition 规范
  - **验证**: CI/CD 流程包含 `cargo clippy` 检查

- [x] **中文文档**: plan.md, spec.md, tasks.md 全部使用中文编写
  - **已完成**: spec.md 使用中文
  - **进行中**: plan.md 使用中文（本文档）
  - **待完成**: tasks.md 将使用中文

- [x] **测试驱动**: 单元测试覆盖率 ≥ 80%
  - **计划**: 每个模块编写单元测试
  - **验证**: 使用 tarpaulin 或 cargo-llvm-cov 测量覆盖率

- [x] **集成测试**: 每个功能完成后执行完整集成测试
  - **计划**: 编写集成测试覆盖：
    - 预设创建、应用、删除流程
    - 批量操作并发执行
    - 导入导出完整流程
    - 错误处理和回滚机制

- [x] **安全优先**: API Key 加密存储，配置文件权限 600
  - **计划**:
    - 导出时自动脱敏 API Key
    - 导入文件验证权限
    - 预设文件权限设置为 600

- [x] **向后兼容**: 配置变更提供迁移脚本
  - **计划**:
    - 预设文件格式变更时提供迁移工具
    - 保留旧版预设文件的导入支持

## Project Structure

### Documentation (this feature)

```text
specs/003-preset-management/
├── plan.md              # 本文件（实现计划）
├── research.md          # Phase 0 输出（技术研究）
├── data-model.md        # Phase 1 输出（数据模型）
├── quickstart.md        # Phase 1 输出（快速入门）
├── contracts/           # Phase 1 输出（接口契约）
│   └── CLI.md          # CLI 命令接口
├── spec.md             # 功能规格说明
└── tasks.md            # Phase 2 输出（任务分解 - 待生成）
```

### Source Code (repository root)

```text
src/
├── main.rs              # 程序入口
├── cli/                 # CLI 命令定义
│   ├── mod.rs
│   ├── commands.rs      # 扩展：添加 preset 和 batch 命令
│   └── args.rs          # 扩展：添加命令行参数
├── config/              # 配置管理（现有）
│   ├── mod.rs
│   ├── store.rs
│   └── models.rs
├── agents/              # Agent 适配器（现有）
│   ├── mod.rs
│   ├── adapter.rs
│   └── ...
├── backup/              # 备份管理（现有）
│   ├── mod.rs
│   └── manager.rs
├── presets/             # [新增] 预设管理
│   ├── mod.rs
│   ├── store.rs         # 预设存储（TOML 读写）
│   ├── preset.rs        # 预设数据结构
│   └── validator.rs     # 预设验证
├── batch/               # [新增] 批量操作
│   ├── mod.rs
│   ├── switch.rs        # 批量切换
│   ├── validate.rs      # 批量验证
│   └── status.rs        # 批量状态
├── io/                  # [新增] 导入导出
│   ├── mod.rs
│   ├── export.rs        # 导出功能
│   ├── import.rs        # 导入功能
│   └── sanitizer.rs     # API Key 脱敏
└── utils/               # 工具函数（现有）
    ├── mod.rs
    └── ...

tests/
├── integration/         # 集成测试
│   ├── preset_test.rs   # 预设管理测试
│   ├── batch_test.rs    # 批量操作测试
│   └── io_test.rs       # 导入导出测试
└── fixtures/            # 测试数据
    ├── presets.json     # 示例导出文件
    └── presets.toml     # 示例预设文件
```

**Structure Decision**: 选择单项目结构（Option 1: Single project），因为：
1. AgentSwitch 是一个独立的 CLI 工具，不需要 workspace
2. 所有功能在同一代码库中，便于维护
3. 符合现有项目结构（Spec 002 已建立）

## 复杂度跟踪 (Complexity Tracking)

> **本功能没有宪章违规，无需论证复杂度**

所有设计遵循 Rust 2024 标准和项目宪章，没有引入额外的复杂性。

## Phase 0: 研究与技术决策

**输出**: [research.md](./research.md)

**研究问题**:
1. ✅ 预设配置存储格式和位置 → **TOML 格式，~/.agentswitch/presets.toml**
2. ✅ 导出配置文件格式 → **JSON 格式**
3. ✅ 批量操作的并发策略 → **rayon 库，并行迭代器**
4. ✅ 错误处理和回滚机制 → **事务模式，自动备份和回滚**
5. ✅ 导入导出的安全和验证机制 → **多层验证，API Key 脱敏**

**关键决策**:
- 使用 rayon 实现并发批量操作（性能要求 SC-002）
- 使用事务模式确保原子性（FR-006, FR-007）
- 导出时自动脱敏 API Key（安全要求）
- 支持合并和覆盖两种导入策略（FR-020）

**新增依赖**:
- `rayon = "1.10"` - 数据并行库

**现有依赖复用**:
- `serde`, `serde_json`, `toml` - 序列化
- `anyhow` - 错误处理
- `dirs` - 路径操作
- `colored` - 终端输出

## Phase 1: 设计与契约

**输出**:
- [data-model.md](./data-model.md) - 数据模型和验证规则
- [contracts/CLI.md](./contracts/CLI.md) - CLI 命令接口
- [quickstart.md](./quickstart.md) - 快速入门指南

### 数据模型

**核心实体**:
1. **Preset**: 配置预设
   - 字段: name, description, tags, created_at, updated_at, mappings
   - 验证: 名称唯一性，模型配置存在性

2. **PresetCollection**: 预设集合
   - 字段: version, presets

3. **ExportPackage**: 导出包
   - 字段: version, exported_at, presets, model_configs（脱敏）, active_config

4. **BatchOperationResult**: 批量操作结果
   - 字段: total, succeeded, failed, results, duration_ms

**存储格式**:
- 预设文件: TOML (~/.agentswitch/presets.toml)
- 导出文件: JSON

### CLI 契约

**新增命令组**:
1. `asw preset <subcommand>` - 预设管理
   - `create` - 创建预设
   - `list` - 列出预设
   - `show` - 显示预设详情
   - `apply` - 应用预设
   - `update` - 更新预设
   - `delete` - 删除预设
   - `validate` - 验证预设
   - `export` - 导出预设

2. `asw batch <subcommand>` - 批量操作
   - `switch` - 批量切换
   - `validate` - 批量验证
   - `status` - 批量状态

3. `asw preset import` - 导入预设

**详细接口定义**: 见 [contracts/CLI.md](./contracts/CLI.md)

### Agent 上下文更新

**已完成**: 更新 CLAUDE.md 文件，添加新技术：
- 语言: Rust 最新稳定版 (edition = "2024")
- 框架: clap 4.x, serde, serde_json, toml, anyhow, dirs, colored, rayon
- 存储: 配置文件 + 系统密钥链

## Phase 2: 实现计划

**输出**: tasks.md（待通过 `/speckit.tasks` 生成）

**任务分解原则**:
1. 按优先级实现（P1 → P2 → P3）
2. 每个任务独立可测试
3. 遵循 TDD 流程（测试先行）
4. 集成测试覆盖所有用户场景

**预期任务类别**:
1. **核心数据结构**（P1）
   - Preset 数据结构
   - PresetCollection
   - ExportPackage
   - 错误类型定义

2. **预设存储**（P1）
   - TOML 读写
   - 文件原子更新
   - 备份机制

3. **预设管理功能**（P1）
   - 创建预设
   - 列出预设
   - 应用预设
   - 更新预设
   - 删除预设
   - 验证预设

4. **批量操作**（P2）
   - 批量切换（并发）
   - 批量验证
   - 批量状态
   - 错误隔离和汇总

5. **导入导出**（P3）
   - 导出预设（JSON）
   - 导入预设（JSON）
   - API Key 脱敏
   - 格式验证

6. **CLI 命令**（P1-P3）
   - preset 命令组
   - batch 命令组
   - 参数解析和验证

7. **测试**（全部）
   - 单元测试（每个模块）
   - 集成测试（端到端场景）
   - 性能测试（验证成功标准）

8. **文档**（全部）
   - 代码注释（中文）
   - 使用示例
   - 故障排除指南

## 质量保证

### 单元测试

**覆盖率目标**: ≥ 80%

**测试内容**:
- 数据结构验证
- 存储层（读写、原子更新）
- 业务逻辑（创建、应用、更新、删除）
- 错误处理（所有错误路径）

### 集成测试

**测试场景**:
1. 预设生命周期（创建 → 应用 → 删除）
2. 批量操作并发执行
3. 导入导出完整流程
4. 错误恢复（备份、回滚）
5. 边界情况（缺失模型、未安装工具）

**测试数据**:
- 模拟工具适配器
- 测试预设文件
- 测试导出文件

### 性能测试

**验证成功标准**:
- SC-001: 创建并应用预设（3 个工具）< 30 秒
- SC-002: 批量切换（5 个工具）< 10 秒
- SC-003: 导出预设（10 个预设）< 5 秒
- SC-004: 导入预设（5 个预设）< 15 秒

**测试方法**:
- 使用 criterion.rs 进行基准测试
- 模拟不同规模（1, 10, 100 个预设）

### 安全测试

**测试内容**:
- API Key 脱敏验证
- 文件权限检查（600）
- 路径遍历防护
- 恶意文件检测

## 风险与缓解

### 技术风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| rayon 并发控制复杂度高 | 中 | 低 | 使用默认并行度，限制并发数 |
| TOML 格式兼容性 | 低 | 中 | 严格验证，提供迁移工具 |
| 大规模预设性能 | 中 | 低 | 延迟加载，缓存优化 |

### 项目风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| 与现有系统集成问题 | 高 | 低 | 充分利用现有 AgentAdapter，最小化修改 |
| 测试覆盖不足 | 中 | 中 | TDD 流程，强制集成测试 |
| 文档不完整 | 低 | 低 | 宪章要求中文文档，审查清单 |

## 实现时间估算

| 阶段 | 任务 | 预估时间 |
|------|------|----------|
| Phase 0 | 研究与技术决策 | ✅ 已完成 |
| Phase 1 | 设计与契约 | ✅ 已完成 |
| Phase 2 | 核心数据结构 | 2-3 天 |
| Phase 2 | 预设存储 | 1-2 天 |
| Phase 2 | 预设管理功能（P1） | 3-4 天 |
| Phase 2 | 批量操作（P2） | 2-3 天 |
| Phase 2 | 导入导出（P3） | 2-3 天 |
| Phase 2 | CLI 命令 | 2-3 天 |
| Phase 2 | 测试（单元 + 集成） | 3-4 天 |
| Phase 2 | 文档和审查 | 1-2 天 |
| **总计** | | **19-26 天** |

## 后续步骤

1. ✅ 运行 `/speckit.plan` - **已完成**（本文档）
2. ⏭️ 运行 `/speckit.tasks` - 生成 tasks.md
3. ⏭️ 开始实现任务（按 P1 → P2 → P3 优先级）
4. ⏭️ 持续集成测试和代码审查
5. ⏭️ 发布 v0.3.0

## 相关文档

- [功能规格说明](spec.md) - 完整的功能需求
- [技术研究](research.md) - 技术决策和实现细节
- [数据模型](data-model.md) - 数据结构和验证规则
- [CLI 契约](contracts/CLI.md) - 命令行接口文档
- [快速入门](quickstart.md) - 用户指南
- [项目宪章](../../.specify/memory/constitution.md) - 开发规范

---

**文档版本**: 1.0.0
**最后更新**: 2026-03-05
**状态**: Phase 1 完成，等待 Phase 2（tasks.md）
