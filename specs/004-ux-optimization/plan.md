# Implementation Plan: 用户体验优化与高级功能

**Branch**: `004-ux-optimization` | **Date**: 2026-03-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-ux-optimization/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

为 AgentSwitch 添加四项用户体验优化功能：交互式配置向导、自动工具发现、Shell 自动补全和 Git 配置同步。这些功能将显著降低新用户使用门槛，提升日常使用效率，并支持多机器配置共享。

技术方法包括：使用 Rust 终端交互库实现向导，扩展现有 AgentAdapter 系统支持自动检测，生成 Shell 补全脚本，以及集成 Git 进行配置版本控制和加密同步。

## Technical Context

**Language/Version**: Rust 最新稳定版 (edition = "2024")
**Primary Dependencies**: clap 4.x, serde, serde_json, toml, anyhow, dirs, colored, inquire (交互式输入), dialoguer (对话框), git2 (Git 操作)
**Storage**: 配置文件 (~/.agentswitch/config.toml) + 系统密钥链 + Git 仓库（可选）
**Testing**: cargo test (单元 + 集成测试)
**Target Platform**: Linux, macOS, Windows (跨平台 CLI 工具)
**Project Type**: cli (命令行工具)
**Performance Goals**: 向导响应 < 100ms, 工具检测 < 3s, 补全生成 < 200ms, Git 同步 < 10s
**Constraints**: 交互式向导需要 TTY 终端，Git 同步需要 Git 已安装
**Scale/Scope**: 单用户工具，支持 10+ 个 Code Agent 工具，三种 Shell（Bash/Zsh/Fish）

## 宪章合规性检查 (Constitution Check)

*GATE: 在 Phase 0 研究前必须通过。Phase 1 设计后再次检查。*

### 强制要求（来自 .specify/memory/constitution.md）

- [x] **Rust 2024 标准**: 代码使用 edition = "2024"，通过 `cargo clippy` 无警告
  - 现有项目已使用 edition = "2024"
  - 新增代码将遵循相同标准

- [x] **中文文档**: plan.md, spec.md, tasks.md 全部使用中文编写
  - spec.md 已使用中文编写
  - 本 plan.md 使用中文编写
  - 后续 tasks.md 将使用中文编写

- [x] **测试驱动**: 单元测试覆盖率 ≥ 80%
  - 每个新模块都将编写对应的单元测试
  - 交互式向导、工具检测、补全生成、Git 同步均需测试覆盖

- [x] **集成测试**: 每个功能完成后执行完整集成测试
  - 向导集成测试：完整配置流程
  - 工具检测集成测试：真实工具环境检测
  - 补全脚本集成测试：生成的脚本在各 Shell 中验证
  - Git 同步集成测试：完整的初始化、推送、拉取流程

- [x] **安全优先**: API Key 加密存储，配置文件权限 600
  - Git 同步时 API Key 必须加密（使用 git-crypt 或类似方案）
  - 向导中输入的 API Key 采用掩码显示
  - 临时向导状态文件权限设置为 600

- [x] **向后兼容**: 配置变更提供迁移脚本
  - 新增功能不修改现有配置格式
  - Git 同步为可选功能，不影响现有用户
  - 补全脚本为新增功能，无破坏性变更

### Gate 结果: ✅ 通过 (Phase 0 前检查)

所有强制要求均已满足或已规划相应的实施方案。无违规项需要论证。

### Gate 结果: ✅ 通过 (Phase 1 设计后检查)

Phase 1 设计完成后，所有强制要求继续保持合规：
- 数据模型设计遵循 Rust 2024 标准
- 所有设计文档使用中文编写
- 测试策略已包含在数据模型中
- 安全性考虑已纳入加密模块设计
- 向后兼容性通过可选功能实现

---

## Phase 0: 研究完成

### 研究文档

完整的可行性研究和技术选型已完成，详见: [research.md](./research.md)

### 关键技术决策

| 功能 | 技术选择 | 版本 | 理由 |
|------|----------|------|------|
| 交互式向导 | inquire | 0.7 | 现代 API、内置验证、TTY 检测 |
| Shell 补全 | clap_complete + 自定义 | 4.5 | 自动同步、维护成本低 |
| Git 集成 | git2 | 0.18 | 类型安全、无外部依赖、性能好 |
| API Key 加密 | AES-GCM + Argon2 | 0.10, 0.5 | 无外部依赖、灵活、跨平台 |
| 工具检测 | 扩展现有 agents 模块 | - | 复用现有架构 |
| 向导状态管理 | TOML + 文件权限 | - | 简单可靠 |

### 新增依赖

```toml
[dependencies]
# 交互式输入
inquire = "0.7"

# Shell 补全
clap_complete = "4.5"

# Git 操作
git2 = "0.18"

# 加密
aes-gcm = "0.10"
argon2 = "0.5"
base64 = "0.21"
```

---

## Phase 1: 设计完成

### 设计文档

Phase 1 设计文档已完成：

1. **数据模型**: [data-model.md](./data-model.md)
   - 向导状态管理 (WizardState)
   - 工具检测结果 (ToolDetection, HealthCheckResult)
   - Shell 补全配置 (CompletionConfig)
   - Git 同步配置 (SyncConfig, SyncState)
   - 加密管理 (CryptoManager, EncryptedValue)

2. **CLI 命令接口**: [contracts/CLI.md](./contracts/CLI.md)
   - 交互式配置向导命令 (init, wizard)
   - 工具诊断命令 (doctor, detect)
   - Shell 补全命令 (completion)
   - Git 同步命令 (sync)
   - 完整的命令层次结构和参数定义

3. **快速开始指南**: [quickstart.md](./quickstart.md)
   - 7 个常见使用场景
   - 完整的工作流程示例
   - 故障排除指南
   - 最佳实践建议

### 架构概览

```
src/
├── wizard/           # 交互式向导（新增）
│   ├── mod.rs
│   ├── state.rs
│   ├── steps.rs
│   └── progress.rs
├── doctor/           # 工具检测和健康检查（新增）
│   ├── mod.rs
│   ├── detector.rs
│   ├── health.rs
│   └── reporter.rs
├── completion/       # Shell 补全（新增）
│   ├── mod.rs
│   ├── static.rs
│   ├── dynamic.rs
│   └── install.rs
├── sync/             # Git 同步（新增）
│   ├── mod.rs
│   ├── git.rs
│   ├── crypto.rs
│   └── conflict.rs
├── cli/              # 扩展现有 CLI
│   └── commands.rs   # 添加新命令
└── agents/           # 现有模块（无需修改）
```

### 数据流

**向导流程**:
```
[开始] → [检查状态] → [恢复/新建] → [收集输入] → [验证] → [保存] → [完成]
```

**工具检测流程**:
```
[扫描] → [检测工具] → [读取配置] → [健康检查] → [生成报告]
```

**Git 同步流程**:
```
[检查 Git] → [加密数据] → [提交] → [推送/拉取] → [解密数据] → [完成]
```

### 关键设计特点

1. **模块化设计**: 每个功能独立模块，降低耦合
2. **复用现有架构**: 基于现有的 agents、config、presets 模块
3. **渐进式实现**: 可以按优先级逐步实现（P1 → P2）
4. **安全性优先**: API Key 加密、文件权限控制
5. **用户体验**: 友好的错误提示、进度保存、TTY 检测

---

## Phase 2: 实施阶段 (待开始)

**注意**: Phase 2 不由 `/speckit.plan` 命令创建。请运行 `/speckit.tasks` 生成 tasks.md。

### 预估任务分解

#### P1 优先级（核心功能）

**交互式配置向导**:
1. 实现向导状态持久化
2. 实现向导步骤定义和验证
3. 集成 inquire 库实现交互式输入
4. 实现向导恢复和取消逻辑
5. 编写向导单元测试和集成测试

**自动工具发现**:
1. 扩展 agents 注册表支持检测方法
2. 实现工具检测器（executable、config path）
3. 实现配置文件健康检查
4. 实现诊断报告生成
5. 编写检测器单元测试

#### P2 优先级（增强功能）

**Shell 自动补全**:
1. 集成 clap_complete 生成静态补全
2. 实现动态补全数据缓存
3. 实现补全脚本安装逻辑
4. 为三种 Shell 生成补全脚本
5. 编写补全功能测试

**Git 同步**:
1. 集成 git2 实现仓库操作
2. 实现 API Key 加密/解密
3. 实现推送/拉取逻辑
4. 实现冲突检测和解决
5. 编写同步功能测试

### 预估工作量

| 阶段 | 任务数 | 预估时间 |
|------|--------|----------|
| P1: 向导 | 5 | 3 天 |
| P1: 工具检测 | 5 | 2 天 |
| P2: Shell 补全 | 5 | 2 天 |
| P2: Git 同步 | 5 | 3 天 |
| 集成测试 | 3 | 2 天 |
| 文档和发布 | 2 | 1 天 |
| **总计** | **25** | **13 天** |

---

## Phase 2: 待生成

**下一步**: 运行 `/speckit.tasks` 生成详细的任务分解文档 (tasks.md)

任务文档将包含：
- 按优先级排序的详细任务列表
- 每个任务的验收标准
- 任务依赖关系
- 实施顺序建议

---

## 总结

### 规划完成状态

✅ **Phase 0 (研究)**: 完成
- 技术选型已完成
- 依赖已确定
- 架构方案已验证

✅ **Phase 1 (设计)**: 完成
- 数据模型已定义
- CLI 接口已设计
- 快速开始指南已编写
- Agent 上下文已更新

⏳ **Phase 2 (实施)**: 待开始
- 需运行 `/speckit.tasks` 生成任务列表
- 按优先级逐步实施功能
- 持续测试和验证

### 关键成果

1. **技术方案明确**: 所有关键技术决策已完成，无未知数
2. **设计完整**: 数据模型、CLI 接口、用户指南齐全
3. **可行性验证**: 技术选型合理，风险可控
4. **用户友好**: 交互式向导、自动补全、清晰错误提示
5. **安全性保障**: API Key 加密、文件权限控制

### 风险和缓解

| 风险 | 级别 | 缓解措施 |
|------|------|----------|
| inquire API 稳定性 | 中 | 使用稳定版本 (0.7)，关注更新 |
| Git 操作复杂度 | 中 | 使用 git2 简化操作，详细错误提示 |
| 加密密钥管理 | 低 | 支持多种密钥来源，清晰文档 |
| Shell 配置差异 | 低 | 检测多个位置，提供手动选项 |
| 工具检测准确性 | 低 | 多种检测方法，手动刷新支持 |

### 成功标准

规划阶段完成后，项目将满足以下标准：

1. **技术可行性**: 所有技术选择已验证，无阻塞问题
2. **设计完整性**: 数据模型、接口定义、用户文档齐全
3. **实施清晰度**: 任务分解明确，依赖关系清晰
4. **质量保障**: 测试策略、安全措施已规划
5. **用户体验**: 交互流程、错误处理、文档完善

---

**规划完成日期**: 2026-03-10
**规划状态**: ✅ Phase 0 和 Phase 1 完成，Phase 2 待开始
**下一步**: 运行 `/speckit.tasks` 生成任务列表

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# Option 1: Single project (DEFAULT - Rust CLI)
src/
├── main.rs              # 程序入口
├── cli/                 # CLI 命令定义
│   ├── mod.rs
│   ├── commands.rs
│   └── args.rs
├── config/              # 配置管理
│   ├── mod.rs
│   ├── store.rs         # 配置存储
│   └── models.rs        # 数据模型
└── agents/              # Agent 适配器
    ├── mod.rs
    ├── adapter.rs       # 适配器 trait
    ├── claude_code.rs   # Claude Code 适配器
    ├── codex.rs         # Codex 适配器
    └── others.rs        # 其他工具适配器

tests/
├── integration/         # 集成测试
├── unit/                # 单元测试
└── fixtures/            # 测试数据
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## 复杂度跟踪 (Complexity Tracking)

> **仅在宪章合规性检查存在违规需要论证时填写**

| 违规项 | 为什么需要 | 拒绝更简单方案的原因 |
|-----------|------------|-------------------------------------|
| [例如: 额外的依赖库] | [当前需求] | [为什么更简单的 3 个依赖不足] |
| [例如: 复杂的设计模式] | [特定问题] | [为什么直接访问不足] |
