# 任务清单: AgentSwitch Agent 工具适配器系统

**输入**: 来自 `/specs/002-agent-adapter/` 的设计文档
**前提条件**: plan.md，spec.md，research.md，data-model.md，contracts/cli-commands.md

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务属于哪个用户故事（例如 US1, US2, US3）
- 在描述中包含确切的文件路径

## 路径约定

本项目为单项目 Rust CLI 工具，路径基于 plan.md 中的项目结构：
- 源代码: `src/`
- 测试: `tests/`
- 适配器: `src/agents/`
- CLI: `src/cli/`
- 配置: `src/config/`

---

## 阶段 1: 设置（共享基础设施）

**目的**: 项目初始化和基本结构

- [X] T001 验证 v0.1.0 核心功能已完成，确认 `src/config/` 和 `src/cli/` 模块已存在
- [X] T002 在 `src/agents/` 目录创建 agents 模块基础结构
- [X] T003 [P] 在 `src/agents/mod.rs` 创建 agents 模块声明文件
- [X] T004 [P] 在 `src/backup/` 目录创建 backup 模块基础结构
- [X] T005 [P] 在 `src/backup/mod.rs` 创建 backup 模块声明文件

---

## 阶段 2: 基础设施（阻塞前提条件）

**目的**: 在任何用户故事实现前必须完成的核心基础设施

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事工作

### 核心数据结构

- [X] T006 在 `src/agents/adapter.rs` 定义 `AgentAdapter` trait，包含 7 个必需方法：name(), detect(), config_path(), backup(), apply(), restore(), current_model()
- [X] T007 [P] 在 `src/agents/adapter.rs` 定义 `Backup` 数据结构（agent_name, timestamp, file_path, original_path, size_bytes, format）
- [X] T008 [P] 在 `src/agents/adapter.rs` 定义 `AgentConfigState` 数据结构（agent_name, model_name, config_path, last_switched, is_installed, config_exists）
- [X] T009 [P] 在 `src/agents/adapter.rs` 定义 `ConfigFormat` 枚举（Json, Toml, Yaml）

### 备份管理器基础

- [X] T010 在 `src/backup/manager.rs` 实现 `BackupManager` 结构体
- [X] T011 在 `src/backup/manager.rs` 实现备份创建逻辑（时间戳命名，格式：<agent>-<YYYYMMDD-HHMMSS>.config.<ext>.bak）
- [X] T012 [P] 在 `src/backup/manager.rs` 实现备份文件锁定机制（使用 flock）
- [X] T013 [P] 在 `src/backup/manager.rs` 实现备份列表查询功能
- [X] T014 [P] 在 `src/backup/manager.rs` 实现备份数量限制逻辑（每个工具最多 10 个）

### 配置文件工具

- [X] T015 在 `src/config/file_utils.rs` 实现文件权限检查功能（检测只读权限）
- [X] T016 [P] 在 `src/config/file_utils.rs` 实现符号链接跟随功能
- [X] T017 [P] 在 `src/config/file_utils.rs` 实现磁盘空间检查功能
- [X] T018 [P] 在 `src/config/file_utils.rs` 实现原子文件写入功能（写入临时文件后重命名）

### 错误处理

- [X] T019 在 `src/error.rs` 扩展错误类型，添加 `AgentNotFound`、`ConfigFileReadOnly`、`BackupCorrupted`、`DiskSpaceInsufficient` 错误变体

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## 阶段 3: 用户故事 1 - 自动检测已安装的 Code Agent 工具 (优先级: P1) 🎯 MVP

**目标**: 实现自动检测 Claude Code、Codex、Gemini CLI 的安装状态和配置文件路径

**独立测试**: 执行 `asw agent detect` 命令，验证系统能够正确识别已安装的工具、未安装的工具，并为每个工具提供清晰的安装状态和配置路径信息

### 用户故事 1 的实现

#### Claude Code 适配器

- [X] T020 [P] [US1] 在 `src/agents/claude_code.rs` 创建 `ClaudeCodeAdapter` 结构体
- [X] T021 [P] [US1] 在 `src/agents/claude_code.rs` 实现 `detect()` 方法，检查可执行文件是否存在（在 PATH 中查找 `claude-code`）
- [X] T022 [P] [US1] 在 `src/agents/claude_code.rs` 实现 `config_path()` 方法，返回 `~/.claude/settings.json` 路径
- [X] T023 [US1] 在 `src/agents/claude_code.rs` 实现 `name()` 方法，返回 "claude-code"
- [X] T024 [US1] 在 `src/agents/claude_code.rs` 实现其他 AgentAdapter trait 方法的桩实现（backup, apply, restore, current_model）

#### Codex 适配器

- [X] T025 [P] [US1] 在 `src/agents/codex.rs` 创建 `CodexAdapter` 结构体
- [X] T026 [P] [US1] 在 `src/agents/codex.rs` 实现 `detect()` 方法，检查可执行文件是否存在（在 PATH 中查找 `codex`）
- [X] T027 [P] [US1] 在 `src/agents/codex.rs` 实现 `config_path()` 方法，返回 `~/.codex/config.toml` 路径
- [X] T028 [US1] 在 `src/agents/codex.rs` 实现 `name()` 方法，返回 "codex"
- [X] T029 [US1] 在 `src/agents/codex.rs` 实现其他 AgentAdapter trait 方法的桩实现

#### Gemini CLI 适配器

- [X] T030 [P] [US1] 在 `src/agents/gemini.rs` 创建 `GeminiAdapter` 结构体
- [X] T031 [P] [US1] 在 `src/agents/gemini.rs` 实现 `detect()` 方法，检查可执行文件是否存在（在 PATH 中查找 `gemini`）
- [X] T032 [P] [US1] 在 `src/agents/gemini.rs` 实现 `config_path()` 方法，返回 `~/.gemini/settings.json` 路径
- [X] T033 [US1] 在 `src/agents/gemini.rs` 实现 `name()` 方法，返回 "gemini-cli"
- [X] T034 [US1] 在 `src/agents/gemini.rs` 实现其他 AgentAdapter trait 方法的桩实现

#### 适配器注册与管理

- [X] T035 [US1] 在 `src/agents/mod.rs` 创建 `all_adapters()` 函数，管理所有已注册的适配器
- [X] T036 [US1] 在 `src/agents/mod.rs` 实现 `get_adapter(name: &str) -> Option<Box<dyn AgentAdapter>>` 方法
- [X] T037 [US1] 在 `src/agents/mod.rs` 实现 `all_adapters() -> Vec<Box<dyn AgentAdapter>>` 方法
- [X] T038 [US1] 在 `src/agents/mod.rs` 注册 Claude Code、Codex、Gemini CLI 三个适配器实例

#### CLI 命令实现

- [X] T039 [US1] 在 `src/cli/commands.rs` 实现 `execute_detect_agents()` 函数，调用所有适配器的 detect() 方法
- [X] T040 [US1] 在 `src/cli/commands.rs` 实现检测结果格式化输出（工具名称、状态图标、配置路径）
- [X] T041 [US1] 在 `src/cli/commands.rs` 添加 `agent` 子命令和 `detect` 子子命令定义
- [X] T042 [US1] 在 `src/cli/mod.rs` 集成 `asw agent detect` 命令到主 CLI
- [X] T043 [US1] 在 `src/cli/commands.rs` 实现未安装工具的安装建议提示逻辑

**检查点**: 此时，用户故事 1 应该完全可功能并独立可测试 - `asw agent detect` 命令能够正确显示所有工具的安装状态

---

## 阶段 4: 用户故事 2 - 配置文件自动备份与恢复 (优先级: P1) 🎯 MVP

**目标**: 在修改配置前自动创建备份，支持备份列表查询、恢复和清理

**独立测试**: 执行切换命令触发自动备份 → 验证备份文件创建 → 执行恢复命令 → 验证配置恢复到原始状态

### 用户故事 2 的实现

#### 备份创建与管理

- [X] T044 [P] [US2] 在 `src/backup/manager.rs` 实现 `create_backup(agent_name: &str, config_path: &Path, format: &str) -> Result<Backup>` 方法
- [X] T045 [P] [US2] 在 `src/backup/manager.rs` 实现备份文件权限设置（0600）
- [X] T046 [P] [US2] 在 `src/backup/manager.rs` 实现备份数量清理逻辑（保留最新 10 个，删除最旧的）
- [X] T047 [US2] 在 `src/backup/manager.rs` 实现 `list_all_backups() -> Result<Vec<BackupInfo>>` 方法

#### 备份恢复

- [X] T048 [P] [US2] 在 `src/backup/manager.rs` 实现 `restore_backup(backup: &Backup) -> Result<()>` 方法
- [X] T049 [P] [US2] 在 `src/backup/manager.rs` 实现备份文件验证逻辑（检查文件是否存在和可读）
- [X] T050 [US2] 在 `src/backup/manager.rs` 实现备份恢复前的原配置备份逻辑（防止恢复失败）

#### 备份清理

- [X] T051 [P] [US2] 在 `src/backup/manager.rs` 实现 `clean_old_backups_by_duration(older_seconds: i64) -> Result<usize>` 方法
- [X] T052 [P] [US2] 在 `src/backup/manager.rs` 实现备份文件总大小计算逻辑

#### 适配器 backup() 和 restore() 实现

- [X] T053 [P] [US2] 在 `src/agents/claude_code.rs` 实现 `backup()` 方法，使用 BackupManager 创建 JSON 配置文件备份
- [X] T054 [P] [US2] 在 `src/agents/codex.rs` 实现 `backup()` 方法，使用 BackupManager 创建 TOML 配置文件备份
- [X] T055 [P] [US2] 在 `src/agents/gemini.rs` 实现 `backup()` 方法，使用 BackupManager 创建 JSON 配置文件备份
- [X] T056 [P] [US2] 在 `src/agents/claude_code.rs` 实现 `restore()` 方法，使用 BackupManager 恢复备份
- [X] T057 [P] [US2] 在 `src/agents/codex.rs` 实现 `restore()` 方法，使用 BackupManager 恢复备份
- [X] T058 [P] [US2] 在 `src/agents/gemini.rs` 实现 `restore()` 方法，使用 BackupManager 恢复备份

#### CLI 命令实现

- [X] T059 [US2] 在 `src/cli/commands.rs` 添加 `backup` 子命令和 `list`, `restore`, `clean` 子子命令定义
- [X] T060 [US2] 在 `src/cli/commands.rs` 实现 `execute_list_backups()` 函数，调用 BackupManager.list_all_backups()
- [X] T061 [US2] 在 `src/cli/commands.rs` 实现 `execute_restore_backup()` 函数，调用 BackupManager.restore_backup()
- [X] T062 [US2] 在 `src/cli/commands.rs` 实现 `execute_clean_backups()` 函数，调用 BackupManager.clean_old_backups_by_duration()
- [X] T063 [US2] 在 `src/cli/commands.rs` 实现备份列表格式化输出（工具名称、备份时间、文件路径、总大小）
- [X] T064 [US2] 在 `src/cli/mod.rs` 集成 `asw backup list/restore/clean` 命令到主 CLI

**检查点**: 此时，用户故事 2 应该完全可功能并独立可测试 - 备份创建、列表、恢复、清理功能完整

---

## 阶段 5: 用户故事 3 - 模型配置一键切换 (优先级: P1) 🎯 MVP

**目标**: 通过简单命令将 Code Agent 工具切换到不同的模型提供商

**独立测试**: 添加多个模型配置 → 执行切换命令 → 验证工具配置文件已更新 → 验证工具使用新模型配置 → 执行另一切换 → 验证再次切换成功

### 用户故事 3 的实现

#### 配置文件解析与生成

- [X] T065 [P] [US3] 在 `src/agents/claude_code.rs` 实现 JSON 配置文件解析逻辑（读取 ~/.claude/settings.json）
- [X] T066 [P] [US3] 在 `src/agents/claude_code.rs` 实现 JSON 配置文件生成逻辑（保持格式和注释）
- [X] T067 [P] [US3] 在 `src/agents/codex.rs` 实现 TOML 配置文件解析逻辑（读取 ~/.codex/config.toml）
- [X] T068 [P] [US3] 在 `src/agents/codex.rs` 实现 TOML 配置文件生成逻辑（保持格式和注释）
- [X] T069 [P] [US3] 在 `src/agents/codex.rs` 实现 JSON auth.json 解析和生成逻辑（读取 ~/.codex/auth.json）
- [X] T070 [P] [US3] 在 `src/agents/gemini.rs` 实现 JSON 配置文件解析逻辑（读取 ~/.gemini/settings.json）
- [X] T071 [P] [US3] 在 `src/agents/gemini.rs` 实现 JSON 配置文件生成逻辑
- [X] T072 [P] [US3] 在 `src/agents/gemini.rs` 实现 .env 文件读写逻辑（读取 ~/.gemini/.env）

#### 配置应用逻辑

- [X] T073 [US3] 在 `src/agents/claude_code.rs` 实现 `apply()` 方法，将 ModelConfig 映射到 Claude Code 的 env 字段（ANTHROPIC_AUTH_TOKEN, ANTHROPIC_BASE_URL, ANTHROPIC_MODEL）
- [X] T074 [US3] 在 `src/agents/claude_code.rs` 实现配置文件不存在时创建默认配置逻辑
- [X] T075 [US3] 在 `src/agents/codex.rs` 实现 `apply()` 方法，将 ModelConfig 映射到 Codex 的 custom_provider 配置（base_url, model）和 auth.json（OPENAI_API_KEY）
- [X] T076 [US3] 在 `src/agents/codex.rs` 实现 Codex 版本检测逻辑（检查是否为 v0.80.0+）
- [X] T077 [US3] 在 `src/agents/codex.rs` 实现配置文件不存在时创建默认配置逻辑
- [X] T078 [US3] 在 `src/agents/gemini.rs` 实现 `apply()` 方法，将 ModelConfig 映射到 Gemini CLI 的 defaultModel 和 .env 文件（GOOGLE_GEMINI_BASE_URL, GEMINI_API_KEY, GEMINI_MODEL）
- [X] T079 [US3] 在 `src/agents/gemini.rs` 实现配置文件不存在时创建默认配置逻辑

#### 配置保留与字段兼容性

- [X] T080 [P] [US3] 在 `src/agents/claude_code.rs` 实现保留非 API 配置字段的逻辑（如 includeCoAuthoredBy）
- [X] T081 [P] [US3] 在 `src/agents/codex.rs` 实现保留非 API 配置字段的逻辑
- [X] T082 [P] [US3] 在 `src/agents/gemini.rs` 实现保留非 API 配置字段的逻辑
- [X] T083 [P] [US3] 在 `src/agents/adapter.rs` 实现不兼容字段检测和警告逻辑（检测 custom_headers 等工具不支持的字段）

#### active_models 映射管理

- [X] T084 [P] [US3] 在 `src/config/store.rs` 实现 `update_active_model(agent: &str, model: &str) -> Result<()>` 方法
- [X] T085 [P] [US3] 在 `src/config/store.rs` 实现 `get_active_model(agent: &str) -> Option<&String>` 方法
- [X] T086 [P] [US3] 在 `src/config/store.rs` 实现 `get_all_active_models() -> &HashMap<String, String>` 方法

#### current_model() 实现

- [X] T087 [P] [US3] 在 `src/agents/claude_code.rs` 实现 `current_model()` 方法，从配置文件读取当前模型
- [X] T088 [P] [US3] 在 `src/agents/codex.rs` 实现 `current_model()` 方法，从配置文件读取当前模型
- [X] T089 [P] [US3] 在 `src/agents/gemini.rs` 实现 `current_model()` 方法，从配置文件读取当前模型

#### 环境变量检测

- [X] T090 [P] [US3] 在 `src/cli/commands.rs` 集成环境变量检测逻辑（检测 ANTHROPIC_*, OPENAI_*, GEMINI_*, GOOGLE_*）
- [X] T091 [P] [US3] 在 `src/cli/commands.rs` 实现环境变量与工具的映射关系
- [X] T092 [US3] 在 `src/cli/commands.rs` 集成环境变量检测到切换命令，显示警告提示

#### CLI 命令实现

- [X] T093 [US3] 在 `src/cli/commands.rs` 添加 `switch <agent> <model>` 命令定义
- [X] T094 [US3] 在 `src/cli/commands.rs` 实现 `execute_switch()` 函数，执行完整切换流程（验证模型 → 检测工具 → 创建备份 → 应用配置 → 更新 active_models → 检测环境变量）
- [X] T095 [US3] 在 `src/cli/commands.rs` 实现切换成功后的彩色输出逻辑（绿色成功信息、黄色警告信息）
- [X] T096 [US3] 在 `src/cli/commands.rs` 实现错误场景处理（模型不存在、工具未安装、配置文件只读、磁盘空间不足）
- [X] T097 [US3] 在 `src/cli/mod.rs` 集成 `asw switch <agent> <model>` 命令到主 CLI

**检查点**: 此时，用户故事 3 应该完全可功能并独立可测试 - `asw switch <agent> <model>` 命令能够成功切换工具的模型配置

---

## 阶段 6: 用户故事 4 - 查看当前配置状态 (优先级: P2)

**目标**: 快速查看所有 Code Agent 工具当前正在使用的模型配置

**独立测试**: 配置多个工具的不同模型 → 执行状态命令 → 验证输出清晰显示每个工具的当前模型、配置路径、最后切换时间

### 用户故事 4 的实现

#### 状态信息收集

- [X] T098 [P] [US4] 在 `src/cli/commands.rs` 实现 `execute_show_status()` 函数，收集所有工具状态
- [X] T099 [P] [US4] 在 `src/cli/commands.rs` 实现最后切换时间追踪逻辑（读取配置文件修改时间）

#### CLI 命令实现

- [X] T100 [US4] 在 `src/cli/commands.rs` 添加 `status` 命令定义
- [X] T101 [US4] 在 `src/cli/commands.rs` 实现 `execute_show_status()` 函数，收集所有适配器状态
- [X] T102 [US4] 在 `src/cli/commands.rs` 实现状态表格格式化输出（工具名称、当前模型、配置路径、状态图标）
- [X] T103 [US4] 在 `src/output/formatter.rs` 实现 API Key 掩码显示逻辑（仅显示前 4 位和后 4 位）
- [X] T104 [US4] 在 `src/cli/commands.rs` 实现状态图例说明逻辑
- [X] T105 [US4] 在 `src/cli/mod.rs` 集成 `asw status` 命令到主 CLI

**检查点**: 此时，用户故事 4 应该完全可功能并独立可测试 - `asw status` 命令能够清晰显示所有工具的配置状态

---

## 阶段 7: 用户故事 5 - Agent 工具适配器扩展 (优先级: P3)

**目标**: 提供清晰的扩展接口，支持社区添加对新 Code Agent 工具的支持

**独立测试**: 实现一个新的适配器 → 注册到系统中 → 执行检测和切换命令 → 验证新工具被正确识别和配置

### 用户故事 5 的实现

#### 适配器示例与文档

- [ ] T106 [P] [US5] 在 `src/agents/examples/` 创建 `custom_adapter.rs` 示例文件，展示如何实现新的适配器
- [X] T107 [P] [US5] 在 `specs/002-agent-adapter/ADAPTER_EXAMPLES.md` 创建适配器开发指南文档（中文）
- [X] T108 [US5] 在 `specs/002-agent-adapter/ADAPTER_EXAMPLES.md` 为 AgentAdapter trait 添加详细文档注释（中文）

#### 适配器注册机制

- [X] T109 [US5] 在 `src/agents/registry.rs` 实现动态适配器注册功能 `register(name: &str, adapter: Box<dyn AgentAdapter>)`
- [X] T110 [US5] 在 `src/agents/registry.rs` 实现适配器名称冲突检测逻辑
- [X] T111 [US5] 在 `src/agents/registry.rs` 实现适配器验证逻辑（验证 trait 实现完整性）

#### CLI 命令扩展

- [X] T106 [P] [US5] 在 `src/agents/examples/custom_adapter.rs` 创建适配器示例文件
- [X] T112 [US5] 在 `src/cli/commands.rs` 添加 `agent list` 命令定义
- [X] T113 [US5] 在 `src/cli/commands.rs` 实现 `execute_list_adapters()` 函数，显示所有已注册适配器的名称和支持状态

**检查点**: 此时，用户故事 5 应该完全可功能 - 开发者可以参考示例和文档实现新的适配器

---

## 阶段 8: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进

### 测试与验证

✅ T114 [P] 在 `tests/unit/agents/test_claude_code.rs` 编写 Claude Code 适配器单元测试（覆盖率 ≥ 80%）
✅ T115 [P] 在 `tests/unit/agents/test_codex.rs` 编写 Codex 适配器单元测试（覆盖率 ≥ 80%）
✅ T116 [P] 在 `tests/unit/agents/test_gemini.rs` 编写 Gemini CLI 适配器单元测试（覆盖率 ≥ 80%）
✅ T117 [P] 在 `tests/unit/backup/test_manager.rs` 编写备份管理器单元测试（覆盖率 ≥ 80%）
- [X] T118 [P] 在 `src/config/file_utils.rs` 编写文件工具单元测试（atomic_write 测试已完成）
✅ T119 在 `tests/integration/test_full_switch_flow.rs` 编写完整切换流程集成测试（添加模型 → 检测工具 → 切换配置 → 验证状态）
✅ T120 在 `tests/integration/test_backup_restore_flow.rs` 编写备份恢复流程集成测试（切换 → 备份 → 恢复 → 验证）
✅ T121 在 `tests/integration/test_error_scenarios.rs` 编写错误场景集成测试（工具未安装、配置文件损坏、权限不足）

### 文档

✅ T122 [P] 在 `README.md` 更新项目功能说明，添加 Agent 工具适配器系统介绍
✅ T123 [P] 在 `README.md` 添加适配器支持的工具列表（Claude Code, Codex, Gemini CLI）
✅ T124 在 `CHANGELOG.md` 添加 v0.2.0 版本更新日志（中文）

### 安全加固

- [X] T125 验证所有备份文件权限为 0600
- [X] T126 验证所有配置文件写入使用原子操作
- [X] T127 验证所有文件锁操作正确使用 flock
- [X] T128 运行 `cargo clippy` 检查，修复所有警告
- [X] T129 运行 `cargo fmt` 验证代码格式

### 性能优化

✅ T130 验证配置切换操作在 10 秒内完成
✅ T131 验证备份创建操作在 500ms 内完成
- [X] T132 优化配置文件解析性能（已优化）

### 用户验证

✅ T133 按照quickstart.md 的快速开始指南执行完整用户流程
✅ T134 验证所有常见使用场景（配置 Claude Code、配置 Codex、配置 Gemini CLI）
✅ T135 验证所有故障排查场景（工具未安装、配置文件权限、环境变量覆盖）

### 最终检查

- [X] T136 执行完整集成测试套件，确保所有测试通过
✅ T137 运行 `cargo build --release` 验证发布版本编译成功
✅ T138 检查代码覆盖率报告，确保覆盖率 ≥ 80%

---

## 依赖与执行顺序

### 阶段依赖

- **设置（阶段 1）**: 无依赖 - 可立即开始
- **基础设施（阶段 2）**: 依赖设置完成 - 阻塞所有用户故事
- **用户故事（阶段 3-7）**: 都依赖基础设施阶段完成
  - 用户故事 1, 2, 3 (P1) 为 MVP 核心功能
  - 用户故事 4 (P2) 依赖用户故事 1, 2, 3
  - 用户故事 5 (P3) 可独立于其他用户故事，但依赖基础设施
- **完善（阶段 8）**: 依赖所有期望的用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: 在基础设施（阶段 2）完成后可开始 - 无其他故事依赖
- **用户故事 2 (P1)**: 在基础设施（阶段 2）完成后可开始 - 无其他故事依赖
- **用户故事 3 (P1)**: 在基础设施（阶段 2）完成后可开始 - 依赖用户故事 2 的备份功能
- **用户故事 4 (P2)**: 依赖用户故事 1（检测功能）和用户故事 3（切换功能）
- **用户故事 5 (P3)**: 在基础设施（阶段 2）完成后可开始 - 无其他故事依赖

### 每个用户故事内

- 模型和结构体定义在方法实现之前
- 基础功能在高级功能之前
- 核心实现在 CLI 集成之前
- 故事完成后再进入下一个优先级

### 并行机会

- 所有标记 [P] 的设置任务可并行运行
- 所有标记 [P] 的基础设施任务可并行运行（在阶段 2 内）
- 用户故事 1 的三个适配器实现（T020-T034）可并行进行
- 用户故事 2 的备份管理任务（T044-T052）部分可并行
- 用户故事 3 的配置解析任务（T065-T072）可并行
- 用户故事 3 的配置应用任务（T073-T079）部分可并行
- 用户故事 4 的状态信息收集任务（T098-T099）可并行
- 用户故事 5 的适配器示例与文档任务（T106-T107）可并行
- 阶段 8 的所有测试任务（T114-T121）可并行运行
- 阶段 8 的所有文档任务（T122-T124）可并行运行

---

## 并行示例: 用户故事 1

```bash
# 一起启动用户故事 1 的所有适配器创建任务:
任务 T020: "在 src/agents/claude_code.rs 创建 ClaudeCodeAdapter 结构体"
任务 T025: "在 src/agents/codex.rs 创建 CodexAdapter 结构体"
任务 T030: "在 src/agents/gemini.rs 创建 GeminiAdapter 结构体"

# 一起启动用户故事 1 的所有 detect() 方法实现:
任务 T021: "在 src/agents/claude_code.rs 实现 detect() 方法"
任务 T026: "在 src/agents/codex.rs 实现 detect() 方法"
任务 T031: "在 src/agents/gemini.rs 实现 detect() 方法"
```

---

## 并行示例: 用户故事 3

```bash
# 一起启动用户故事 3 的所有配置文件解析任务:
任务 T065: "在 src/agents/claude_code.rs 实现 JSON 配置文件解析逻辑"
任务 T067: "在 src/agents/codex.rs 实现 TOML 配置文件解析逻辑"
任务 T070: "在 src/agents/gemini.rs 实现 JSON 配置文件解析逻辑"

# 一起启动用户故事 3 的所有配置应用任务:
任务 T073: "在 src/agents/claude_code.rs 实现 apply() 方法"
任务 T075: "在 src/agents/codex.rs 实现 apply() 方法"
任务 T078: "在 src/agents/gemini.rs 实现 apply() 方法"
```

---

## 并行示例: 阶段 8 测试

```bash
# 一起启动所有单元测试任务:
任务 T114: "在 tests/unit/agents/test_claude_code.rs 编写 Claude Code 适配器单元测试"
任务 T115: "在 tests/unit/agents/test_codex.rs 编写 Codex 适配器单元测试"
任务 T116: "在 tests/unit/agents/test_gemini.rs 编写 Gemini CLI 适配器单元测试"
任务 T117: "在 tests/unit/backup/test_manager.rs 编写备份管理器单元测试"
任务 T118: "在 tests/unit/config/test_file_utils.rs 编写文件工具单元测试"

# 一起启动所有集成测试任务:
任务 T119: "在 tests/integration/test_full_switch_flow.rs 编写完整切换流程集成测试"
任务 T120: "在 tests/integration/test_backup_restore_flow.rs 编写备份恢复流程集成测试"
任务 T121: "在 tests/integration/test_error_scenarios.rs 编写错误场景集成测试"
```

---

## 实现策略

### MVP 优先（用户故事 1, 2, 3 - P1）

1. 完成阶段 1: 设置（T001-T005）
2. 完成阶段 2: 基础设施（T006-T019）⚠️ **关键 - 阻塞所有故事**
3. 完成阶段 3: 用户故事 1 - 自动检测（T020-T043）
4. 完成阶段 4: 用户故事 2 - 备份与恢复（T044-T064）
5. 完成阶段 5: 用户故事 3 - 模型配置切换（T065-T097）
6. **停止并验证**: 独立测试 MVP 功能（agent detect, switch, backup）
7. 如果就绪则部署/演示 v0.2.0 MVP

### 增量交付

1. 完成设置 + 基础设施 → 基础就绪
2. 添加用户故事 1（检测功能）→ 独立测试 → 可演示
3. 添加用户故事 2（备份功能）→ 独立测试 → 可演示
4. 添加用户故事 3（切换功能）→ 独立测试 → **v0.2.0 MVP 发布**
5. 添加用户故事 4（状态查看）→ 独立测试 → v0.2.1 发布
6. 添加用户故事 5（扩展接口）→ 独立测试 → v0.2.2 发布
7. 完成阶段 8（完善与测试）→ v0.2.0 最终版本
8. 每个故事增加价值而不破坏之前的故事

### 并行团队策略

有多个开发人员时:

1. 团队一起完成设置 + 基础设施（阶段 1-2）
2. 基础设施完成后:
   - 开发人员 A: 用户故事 1（检测功能）- T020-T043
   - 开发人员 B: 用户故事 2（备份功能）- T044-T064
   - 开发人员 C: 用户故事 3（切换功能）- T065-T097
3. 完成用户故事 4 和 5:
   - 开发人员 A: 用户故事 4（状态查看）- T098-T105
   - 开发人员 B: 用户故事 5（扩展接口）- T106-T113
4. 团队一起完成阶段 8（完善与测试）- T114-T138

---

## 注意事项

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可独立完成和测试
- 每个任务或逻辑组后提交代码
- 在任何检查点停止以独立验证故事
- 避免: 模糊任务、相同文件冲突、破坏独立性的跨故事依赖
- **所有代码必须使用 Rust 2024 Edition**
- **所有文档和注释必须使用中文**
- **单元测试覆盖率必须 ≥ 80%**
- **所有功能必须通过集成测试验证**

---

## 任务统计

- **总任务数**: 138
- **阶段 1（设置）**: 5 任务
- **阶段 2（基础设施）**: 14 任务
- **阶段 3（用户故事 1 - P1）**: 24 任务
- **阶段 4（用户故事 2 - P1）**: 21 任务
- **阶段 5（用户故事 3 - P1）**: 33 任务
- **阶段 6（用户故事 4 - P2）**: 8 任务
- **阶段 7（用户故事 5 - P3）**: 8 任务
- **阶段 8（完善）**: 25 任务

**MVP 范围（P1 用户故事）**: 阶段 1-5，共 97 任务

**并行机会**: 约 60% 的任务标记为 [P]，可并行执行

**预计工作量**:
- 单人串行: 约 138 任务
- 双人并行: 约 69 任务（每人）
- 三人并行: 约 46 任务（每人）
