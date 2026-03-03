# 实现计划: AgentSwitch Agent 工具适配器系统

**分支**: `002-agent-adapter` | **日期**: 2026-02-28 | **规格**: [spec.md](spec.md)
**输入**: 来自 `/specs/002-agent-adapter/spec.md` 的功能规格说明

## Summary

本阶段实现 AgentSwitch 的 Agent 工具适配器系统，支持 Claude Code、Codex、Gemini CLI 三个主流 Code Agent 工具的配置集成与一键切换功能。

**核心功能**:
1. 自动检测已安装的 Code Agent 工具
2. 配置文件自动备份与恢复
3. 模型配置一键切换
4. 当前配置状态查看
5. Agent 工具适配器扩展接口

**技术方法**: 使用 Rust 2024 Edition 构建适配器系统，基于网络调研获取的真实配置文件信息实现三个工具的配置适配器，支持 JSON 和 TOML 格式。

## Technical Context

**Language/Version**: Rust 最新稳定版 (edition = "2024")
**Primary Dependencies**:
- clap 4.x (CLI 框架，derive 特性)
- serde + serde_json (JSON 序列化)
- toml (TOML 序列化)
- anyhow (错误处理)
- dirs (跨平台路径)
- colored (彩色输出)
- chrono (时间处理，用于备份时间戳)

**Storage**:
- 配置文件: `~/.agentswitch/config.toml`
- 备份目录: `~/.agentswitch/backups/`
- 工具配置:
  - Claude Code: `~/.claude/settings.json` (JSON)
  - Codex: `~/.codex/config.toml` (TOML) + `~/.codex/auth.json` (JSON)
  - Gemini CLI: `~/.gemini/settings.json` (JSON) + `~/.gemini/.env` (环境变量)

**Testing**: cargo test (单元 + 集成测试)
**Target Platform**: Linux, macOS (优先), Windows (后续支持)
**Project Type**: CLI (命令行工具)
**Performance Goals**: 配置切换 < 10s，备份操作 < 500ms
**Constraints**: 无网络依赖（本地操作），离线可用
**Scale/Scope**: 单用户工具，支持 3+ 个 Code Agent 工具

**重要发现** (基于网络调研):
1. ✅ Gemini CLI 使用 JSON 而非 YAML（规格已更正）
2. ✅ Codex 需要使用 v0.80.0 才支持自定义 API
3. ✅ 所有工具都支持环境变量覆盖配置文件

## 宪章合规性检查 (Constitution Check)

### 强制要求（来自 .specify/memory/constitution.md）

- [x] **Rust 2024 标准**: 代码使用 edition = "2024"，通过 `cargo clippy` 无警告
  - **验证方式**: CI/CD 中集成 `cargo clippy` 和 `cargo fmt --check`
- [x] **中文文档**: plan.md, spec.md, tasks.md 全部使用中文编写
  - **当前状态**: ✅ 所有文档使用中文
- [x] **测试驱动**: 单元测试覆盖率 ≥ 80%
  - **计划**: 每个适配器和命令都有对应的单元测试
- [x] **集成测试**: 每个功能完成后执行完整集成测试
  - **计划**: tests/integration/ 目录下包含完整切换流程测试
- [x] **安全优先**: API Key 文件权限 0600，配置文件权限保护
  - **实现**: 使用 `std::fs::set_permissions` 设置权限
  - **注意**: 工具本身的配置文件使用明文存储，AgentSwitch 仅管理权限
- [x] **向后兼容**: 配置变更提供迁移脚本
  - **当前阶段**: v1.0 初始版本，配置格式稳定；未来变更将通过版本号和迁移函数处理

**合规性状态**: ✅ **全部通过**

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 最新稳定版 (edition = "2024")
**Primary Dependencies**: clap 4.x, serde, serde_json, toml, anyhow, dirs, colored
**Storage**: 配置文件 (~/.agentswitch/config.toml) + 系统密钥链
**Testing**: cargo test (单元 + 集成测试)
**Target Platform**: Linux, macOS, Windows (跨平台 CLI 工具)
**Project Type**: cli (命令行工具)
**Performance Goals**: 配置切换 < 100ms, 启动时间 < 50ms
**Constraints**: 无网络依赖（本地操作），离线可用
**Scale/Scope**: 单用户工具，支持 10+ 个 Code Agent 工具

## 宪章合规性检查 (Constitution Check)

*GATE: 在 Phase 0 研究前必须通过。Phase 1 设计后再次检查。*

### 强制要求（来自 .specify/memory/constitution.md）

- [ ] **Rust 2024 标准**: 代码使用 edition = "2024"，通过 `cargo clippy` 无警告
- [ ] **中文文档**: plan.md, spec.md, tasks.md 全部使用中文编写
- [ ] **测试驱动**: 单元测试覆盖率 ≥ 80%
- [ ] **集成测试**: 每个功能完成后执行完整集成测试
- [ ] **安全优先**: API Key 加密存储，配置文件权限 600
- [ ] **向后兼容**: 配置变更提供迁移脚本

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

---

## Phase 2: 任务分解（待执行）

### 核心实现任务

#### 1. AgentAdapter trait 定义
**文件**: `src/agents/adapter.rs`

**任务**:
- 定义 `AgentAdapter` trait 包含所有必需方法
- 定义 `Backup` 和 `AgentConfigState` 数据结构
- 定义 `ConfigFormat` 枚举（Json、Toml）

**验收标准**:
- Trait 定义包含 7 个方法：`name()`, `detect()`, `config_path()`, `backup()`, `apply()`, `restore()`, `current_model()`
- 数据结构支持序列化/反序列化
- 编译通过，无 clippy 警告

#### 2. Claude Code 适配器实现
**文件**: `src/agents/claude_code.rs`

**任务**:
- 实现 `AgentAdapter` trait
- 支持 JSON 格式的 `settings.json`
- 实现版本检测（检测 `env` 字段）
- 实现 API 字段映射逻辑

**配置映射**:
- `base_url` → `env.ANTHROPIC_BASE_URL`
- `api_key` → `env.ANTHROPIC_AUTH_TOKEN`
- `model_id` → `env.ANTHROPIC_MODEL`

**验收标准**:
- 能够正确解析 `~/.claude/settings.json`
- 能够应用新配置并保持格式
- 单元测试覆盖率 ≥ 80%

#### 3. Codex 适配器实现
**文件**: `src/agents/codex.rs`

**任务**:
- 实现 `AgentAdapter` trait
- 支持 TOML 格式的 `config.toml`
- 支持 JSON 格式的 `auth.json`
- 实现版本检测（检查是否为 v0.80.0）
- 实现 API 字段映射逻辑

**配置映射**:
- `base_url` → `model_providers.custom_provider.base_url`
- `api_key` → `auth.json.OPENAI_API_KEY`
- `model_id` → `model`

**验收标准**:
- 能够正确解析 `~/.codex/config.toml`
- 能够正确读取和修改 `~/.codex/auth.json`
- 版本检测功能正常
- 单元测试覆盖率 ≥ 80%

#### 4. Gemini CLI 适配器实现
**文件**: `src/agents/gemini.rs`

**任务**:
- 实现 `AgentAdapter` trait
- 支持 JSON 格式的 `settings.json`
- 支持 `.env` 文件的环境变量读写
- 实现配置优先级处理

**配置映射**:
- `base_url` → `.env.GOOGLE_GEMINI_BASE_URL`
- `api_key` → `.env.GEMINI_API_KEY`
- `model_id` → `defaultModel` 或 `.env.GEMINI_MODEL`

**验收标准**:
- 能够正确解析 `~/.gemini/settings.json`
- 能够正确读写 `~/.gemini/.env`
- 单元测试覆盖率 ≥ 80%

#### 5. 备份管理器实现
**文件**: `src/backup/mod.rs`, `src/backup/manager.rs`

**任务**:
- 实现 `BackupManager` 结构
- 实现备份创建逻辑（时间戳命名）
- 实现备份清理逻辑（每个工具最多10个）
- 实现备份列表查询
- 实现备份恢复功能

**验收标准**:
- 备份文件命名正确：`<agent>-<timestamp>.config.<ext>.bak`
- 备份数量限制生效
- 备份恢复功能正常
- 单元测试覆盖率 ≥ 80%

#### 6. CLI 命令实现
**文件**: `src/cli/commands.rs`（扩展）

**新增命令**:
- `asw agent detect`
- `asw switch <agent> <model>`
- `asw status`
- `asw backup list`
- `asw backup restore <agent> --backup <timestamp>`
- `asw backup clean --older-than <duration>`

**验收标准**:
- 所有命令正常工作
- 错误处理完善
- 彩色输出正确
- 集成测试通过

---

## 测试策略

### 单元测试

**覆盖范围**:
- 适配器接口方法测试（detect、backup、apply、restore、current_model）
- 配置文件解析和序列化测试（JSON/TOML）
- 备份管理器测试
- 文件权限测试

### 集成测试

**测试场景**:
- 完整的切换流程（添加模型 → 检测工具 → 切换配置 → 验证状态）
- 备份恢复流程（切换 → 备份 → 恢复 → 验证）
- 错误场景测试（工具未安装、配置文件损坏、权限不足）

---

## 技术债务与未来改进

1. **Windows 支持**: 当前优先 Linux/macOS，Windows 路径和权限处理需要补充
2. **加密存储**: 当前使用明文存储 + 权限保护，未来可考虑操作系统密钥链
3. **项目级配置**: Claude Code 支持项目级配置，可考虑支持
4. **配置验证**: 切换后验证配置是否生效（如测试 API 连接）

---

## 参考来源

**规格文档**: [spec.md](spec.md)
**技术研究**: [research.md](research.md)
**数据模型**: [data-model.md](data-model.md)
**快速开始**: [quickstart.md](quickstart.md)
**命令契约**: [contracts/cli-commands.md](contracts/cli-commands.md)
