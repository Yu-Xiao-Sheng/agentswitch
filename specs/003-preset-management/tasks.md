# 任务清单: 配置预设与批量管理

**输入**: 来自 `/specs/003-preset-management/` 的设计文档
**前提条件**: plan.md（已完成），spec.md（用户故事 P1-P3），data-model.md，contracts/CLI.md，research.md

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

**测试**: 本功能要求测试驱动开发（TDD），包含单元测试和集成测试。

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务属于哪个用户故事（US1, US2, US3）
- 在描述中包含确切的文件路径

## 路径约定

- 项目根目录的 `src/`, `tests/`
- 新增模块: `src/presets/`, `src/batch/`, `src/io/`
- 集成测试: `tests/integration/`

---

## 阶段 1: 设置（共享基础设施）

**目的**: 项目初始化和依赖配置

- [ ] T001 在 Cargo.toml 中添加 rayon 依赖（版本 1.10）
- [ ] T002 验证 Cargo.toml 配置 edition = "2024"
- [ ] T003 [P] 创建 src/presets/ 目录结构（mod.rs, store.rs, preset.rs, validator.rs）
- [ ] T004 [P] 创建 src/batch/ 目录结构（mod.rs, switch.rs, validate.rs, status.rs）
- [ ] T005 [P] 创建 src/io/ 目录结构（mod.rs, export.rs, import.rs, sanitizer.rs）
- [ ] T006 [P] 创建 tests/integration/ 目录结构（preset_test.rs, batch_test.rs, io_test.rs）
- [ ] T007 [P] 创建 tests/fixtures/ 目录并添加示例数据文件

---

## 阶段 2: 基础设施（阻塞前提条件）

**目的**: 核心数据结构和错误处理，所有用户故事依赖

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事工作

### 错误处理

- [ ] T008 在 src/presets/error.rs 创建 PresetError 枚举（包含所有错误类型：PresetNotFound, ModelConfigNotFound, AgentNotInstalled, BackupFailed, ApplyFailed, RollbackFailed, ValidationFailed, ImportFailed, ExportFailed）
- [ ] T009 在 src/presets/mod.rs 导出 PresetError 并实现 anyhow::Context

### 核心数据模型

- [ ] T010 [P] 在 src/presets/preset.rs 创建 Preset 结构体（name, description, tags, created_at, updated_at, mappings 字段）
- [ ] T011 [P] 在 src/presets/preset.rs 为 Preset 实现 Serialize 和 Deserialize trait
- [ ] T012 [US1] 在 src/presets/preset.rs 实现 Preset::validate() 方法（验证名称格式、描述长度、标签、映射关系、模型存在性）
- [ ] T013 [P] 在 src/presets/store.rs 创建 PresetCollection 结构体（version, presets 字段）
- [ ] T014 [P] 在 src/presets/store.rs 为 PresetCollection 实现 Serialize 和 Deserialize trait
- [ ] T015 [P] 在 src/io/export.rs 创建 ExportPackage 结构体（version, exported_at, presets, model_configs, active_config 字段）
- [ ] T016 [P] 在 src/io/export.rs 为 ExportPackage 实现 Serialize 和 Deserialize trait
- [ ] T017 [P] 在 src/io/export.rs 创建 ImportStrategy 枚举（Merge, Overwrite）
- [ ] T018 [P] 在 src/batch/status.rs 创建 BatchOperationResult 结构体（total, succeeded, failed, results, duration_ms）
- [ ] T019 [P] 在 src/batch/status.rs 创建 ToolOperationResult 结构体（agent_name, success, error_message, backup_path）

### 预设存储基础

- [ ] T020 在 src/presets/store.rs 实现 PresetStore::new() 方法（初始化存储，读取 ~/.agentswitch/presets.toml）
- [ ] T021 在 src/presets/store.rs 实现 PresetStore::load() 方法（从 TOML 文件加载预设集合）
- [ ] T022 在 src/presets/store.rs 实现 PresetStore::save() 方法（原子写入 TOML 文件，使用临时文件 + 重命名）
- [ ] T023 在 src/presets/store.rs 实现 PresetStore::backup() 方法（创建 presets.backup.toml）
- [ ] T024 在 src/presets/validator.rs 实现 is_valid_preset_name() 函数（验证预设名称格式）
- [ ] T025 在 src/presets/validator.rs 实现 is_valid_version() 函数（验证版本号格式）

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## 阶段 3: 用户故事 1 - 配置预设管理 (优先级: P1) 🎯 MVP

**目标**: 实现预设的创建、列出、应用、更新、删除和验证功能

**独立测试**: 可以通过创建预设、列出预设、应用预设、更新预设、删除预设等操作完全测试，提供完整的配置管理价值

**验收场景**:
1. 给定用户已配置多个工具使用不同模型，当用户创建名为 "development" 的预设并包含当前工具配置，则系统保存预设并返回成功确认
2. 给定用户已创建多个预设，当用户执行列出预设命令，则系统显示所有预设名称、描述和包含的工具配置映射
3. 给定用户已存在 "development" 预设，当用户应用该预设，则系统将预设中的模型配置应用到对应工具并显示应用结果
4. 给定用户需要调整预设，当用户更新预设中某个工具的模型配置，则系统保存更新后的预设并返回成功确认
5. 给定用户不再需要某个预设，当用户删除该预设，则系统移除预设并返回成功确认

### 用户故事 1 的单元测试

> **注意: 先编写这些测试，确保在实现前失败**

- [ ] T026 [P] [US1] Preset::validate() 的单元测试在 tests/presets/preset_test.rs（测试有效/无效预设名称、描述长度、标签数量、空映射）
- [ ] T027 [P] [US1] PresetStore::load() 和 save() 的单元测试在 tests/presets/store_test.rs（测试 TOML 读写、原子写入、备份创建）
- [ ] T028 [P] [US1] add_preset() 的单元测试在 tests/presets/store_test.rs（测试添加预设、名称冲突检测）
- [ ] T029 [P] [US1] apply_preset() 的单元测试在 tests/presets/apply_test.rs（测试应用预设、备份创建、失败回滚）

### 用户故事 1 的实现

#### 预设 CRUD 操作

- [ ] T030 [P] [US1] 在 src/presets/store.rs 实现 PresetStore::add_preset() 方法（验证名称唯一性，添加到集合）
- [ ] T031 [P] [US1] 在 src/presets/store.rs 实现 PresetStore::get_preset() 方法（按名称获取预设）
- [ ] T032 [P] [US1] 在 src/presets/store.rs 实现 PresetStore::list_presets() 方法（返回所有预设）
- [ ] T033 [P] [US1] 在 src/presets/store.rs 实现 PresetStore::find_by_tag() 方法（按标签筛选预设）
- [ ] T034 [US1] 在 src/presets/store.rs 实现 PresetStore::update_preset() 方法（更新预设，自动更新 updated_at 时间戳）
- [ ] T035 [US1] 在 src/presets/store.rs 实现 PresetStore::remove_preset() 方法（删除预设）

#### 预设应用功能

- [ ] T036 [US1] 在 src/presets/apply.rs 创建 PresetAppplier 结构体
- [ ] T037 [US1] 在 src/presets/apply.rs 实现 PresetAppplier::apply() 方法（应用预设到所有工具）
- [ ] T038 [US1] 在 src/presets/apply.rs 实现 PresetAppplier::apply_to_agents() 方法（应用预设到指定工具列表）
- [ ] T039 [US1] 在 src/presets/apply.rs 实现 PresetAppplier::backup_before_apply() 方法（使用 backup::Manager 备份配置）
- [ ] T040 [US1] 在 src/presets/apply.rs 实现 PresetAppplier::rollback_on_failure() 方法（失败时回滚到备份）
- [ ] T041 [US1] 在 src/presets/apply.rs 实现 PresetAppplier::validate_before_apply() 方法（验证模型配置存在性、工具安装状态）

#### 预设验证功能

- [ ] T042 [US1] 在 src/presets/validator.rs 实现 validate_preset() 函数（检查模型配置存在性）
- [ ] T043 [US1] 在 src/presets/validator.rs 实现 validate_preset_agents() 函数（检查工具安装状态）

#### CLI 命令 - preset

- [ ] T044 [P] [US1] 在 src/cli/args.rs 添加 PresetCommands 枚举（Create, List, Show, Apply, Update, Delete, Validate, Export）
- [ ] T045 [P] [US1] 在 src/cli/args.rs 添加 PresetCreateSubcommand 结构体（name, description, tags, agents 参数）
- [ ] T046 [P] [US1] 在 src/cli/args.rs 添加 PresetListSubcommand 结构体（tag, format 参数）
- [ ] T047 [P] [US1] 在 src/cli/args.rs 添加 PresetApplySubcommand 结构体（name, agents, dry_run, no_backup 参数）
- [ ] T048 [P] [US1] 在 src/cli/args.rs 添加 PresetUpdateSubcommand 结构体（name, description, tag, remove_tag, agent, remove_agent 参数）
- [ ] T049 [P] [US1] 在 src/cli/args.rs 添加 PresetDeleteSubcommand 结构体（name, force 参数）
- [ ] T050 [P] [US1] 在 src/cli/args.rs 添加 PresetValidateSubcommand 结构体（name 参数）
- [ ] T051 [US1] 在 src/cli/commands.rs 实现 execute_preset_create() 函数
- [ ] T052 [US1] 在 src/cli/commands.rs 实现 execute_preset_list() 函数（支持 table/json/toml 格式）
- [ ] T053 [US1] 在 src/cli/commands.rs 实现 execute_preset_show() 函数
- [ ] T054 [US1] 在 src/cli/commands.rs 实现 execute_preset_apply() 函数（集成备份、应用、回滚）
- [ ] T055 [US1] 在 src/cli/commands.rs 实现 execute_preset_update() 函数
- [ ] T056 [US1] 在 src/cli/commands.rs 实现 execute_preset_delete() 函数（带确认提示）
- [ ] T057 [US1] 在 src/cli/commands.rs 实现 execute_preset_validate() 函数
- [ ] T058 [US1] 在 src/cli/mod.rs 注册 preset 命令到 CLI 主命令

**检查点**: 此时，用户故事 1 应该完全可功能并独立可测试

---

## 阶段 4: 用户故事 2 - 批量操作 (优先级: P2)

**目标**: 实现批量切换、批量验证和批量状态查看功能

**独立测试**: 可以通过批量切换、批量验证、批量状态查看等操作完全测试，提供批量管理工具配置的价值

**验收场景**:
1. 给定用户有多个工具正在使用不同模型，当用户执行批量切换命令指定目标模型，则系统将所有工具切换到该模型并显示每个工具的切换结果
2. 给定用户只想切换部分工具，当用户执行批量切换命令并指定工具列表和目标模型，则系统仅切换指定工具并显示结果
3. 给定用户已配置多个工具，当用户执行批量验证命令，则系统检查每个工具的配置状态并显示验证结果汇总
4. 给定用户需要查看所有工具的当前配置，当用户执行批量状态命令，则系统显示所有工具及其当前使用的模型

### 用户故事 2 的单元测试

- [ ] T059 [P] [US2] batch_switch_agents() 的单元测试在 tests/batch/switch_test.rs（测试并发执行、错误隔离）
- [ ] T060 [P] [US2] batch_validate_agents() 的单元测试在 tests/batch/validate_test.rs（测试批量验证）
- [ ] T061 [P] [US2] batch_get_status() 的单元测试在 tests/batch/status_test.rs（测试状态汇总）

### 用户故事 2 的实现

#### 批量切换功能

- [ ] T062 [P] [US2] 在 src/batch/switch.rs 实现 batch_switch_agents() 函数（使用 rayon::par_iter 并发执行）
- [ ] T063 [US2] 在 src/batch/switch.rs 实现 batch_switch_selected_agents() 函数（切换指定工具列表）
- [ ] T064 [US2] 在 src/batch/switch.rs 实现 collect_switch_results() 函数（汇总成功和失败的工具列表）
- [ ] T065 [US2] 在 src/batch/switch.rs 实现 backup_all_before_switch() 函数（批量备份配置）

#### 批量验证功能

- [ ] T066 [P] [US2] 在 src/batch/validate.rs 实现 batch_validate_agents() 函数（并发验证所有工具）
- [ ] T067 [US2] 在 src/batch/validate.rs 实现 batch_validate_selected_agents() 函数（验证指定工具列表）
- [ ] T068 [US2] 在 src/batch/validate.rs 实现 collect_validation_results() 函数（汇总验证结果）

#### 批量状态功能

- [ ] T069 [P] [US2] 在 src/batch/status.rs 实现 batch_get_status() 函数（获取所有工具的当前配置）
- [ ] T070 [US2] 在 src/batch/status.rs 实现 format_status_table() 函数（格式化为 table 输出）
- [ ] T071 [US2] 在 src/batch/status.rs 实现 format_status_json() 函数（格式化为 JSON 输出）

#### CLI 命令 - batch

- [ ] T072 [P] [US2] 在 src/cli/args.rs 添加 BatchCommands 枚举（Switch, Validate, Status）
- [ ] T073 [P] [US2] 在 src/cli/args.rs 添加 BatchSwitchSubcommand 结构体（model, agents, parallel, dry_run 参数）
- [ ] T074 [P] [US2] 在 src/cli/args.rs 添加 BatchValidateSubcommand 结构体（agents 参数）
- [ ] T075 [P] [US2] 在 src/cli/args.rs 添加 BatchStatusSubcommand 结构体（format 参数）
- [ ] T076 [US2] 在 src/cli/commands.rs 实现 execute_batch_switch() 函数（集成并发执行、错误隔离）
- [ ] T077 [US2] 在 src/cli/commands.rs 实现 execute_batch_validate() 函数
- [ ] T078 [US2] 在 src/cli/commands.rs 实现 execute_batch_status() 函数
- [ ] T079 [US2] 在 src/cli/mod.rs 注册 batch 命令到 CLI 主命令

**检查点**: 此时，用户故事 1 和 2 都应该独立工作

---

## 阶段 5: 用户故事 3 - 配置导入导出 (优先级: P3)

**目标**: 实现预设的导出和导入功能，支持团队分享和多机器迁移

**独立测试**: 可以通过导出预设、导出当前配置、导入预设、导入当前配置等操作完全测试，提供配置分享和迁移的价值

**验收场景**:
1. 给定用户已创建多个预设，当用户导出单个预设到文件，则系统生成包含预设配置的标准格式文件
2. 给定用户已创建多个预设，当用户导出所有预设到文件，则系统生成包含所有预设和模型配置的标准格式文件
3. 给定用户已配置多个工具，当用户导出当前活跃配置，则系统生成包含当前所有工具模型映射的标准格式文件
4. 给定用户有一个导出的预设文件，当用户导入该文件，则系统解析文件、验证格式、检查模型依赖并创建预设
5. 给定用户导入预设时已存在同名预设，当用户使用合并选项导入，则系统合并配置而不覆盖现有预设
6. 给定用户导入预设时已存在同名预设，当用户使用覆盖选项导入，则系统替换现有预设

### 用户故事 3 的单元测试

- [ ] T080 [P] [US3] export_presets() 的单元测试在 tests/io/export_test.rs（测试导出单个/所有预设）
- [ ] T081 [P] [US3] import_presets() 的单元测试在 tests/io/import_test.rs（测试合并/覆盖策略）
- [ ] T082 [P] [US3] sanitize_api_key() 的单元测试在 tests/io/sanitizer_test.rs（测试 API Key 脱敏）
- [ ] T083 [P] [US3] validate_export_package() 的单元测试在 tests/io/validation_test.rs（测试格式验证、依赖检查）

### 用户故事 3 的实现

#### API Key 脱敏

- [ ] T084 [P] [US3] 在 src/io/sanitizer.rs 实现 sanitize_api_key() 函数（替换为 "***REDACTED***"）
- [ ] T085 [US3] 在 src/io/sanitizer.rs 实现 sanitize_model_config() 函数（脱敏单个模型配置）
- [ ] T086 [US3] 在 src/io/sanitizer.rs 实现 sanitize_model_configs() 函数（脱敏所有模型配置）

#### 导出功能

- [ ] T087 [P] [US3] 在 src/io/export.rs 实现 export_presets() 函数（导出预设集合到 JSON）
- [ ] T088 [US3] 在 src/io/export.rs 实现 export_single_preset() 函数（导出单个预设）
- [ ] T089 [US3] 在 src/io/export.rs 实现 export_with_model_configs() 函数（包含脱敏的模型配置）
- [ ] T090 [US3] 在 src/io/export.rs 实现 export_with_active_config() 函数（包含当前活跃配置）
- [ ] T091 [US3] 在 src/io/export.rs 实现 set_export_file_permissions() 函数（设置文件权限为 600）

#### 导入功能

- [ ] T092 [P] [US3] 在 src/io/import.rs 实现 import_presets() 函数（从 JSON 导入预设）
- [ ] T093 [US3] 在 src/io/import.rs 实现 import_with_merge_strategy() 函数（合并模式：保留现有预设）
- [ ] T094 [US3] 在 src/io/import.rs 实现 import_with_overwrite_strategy() 函数（覆盖模式：替换现有预设）
- [ ] T095 [US3] 在 src/io/import.rs 实现 validate_import_file() 函数（格式验证、路径验证、权限验证）
- [ ] T096 [US3] 在 src/io/import.rs 实现 check_import_dependencies() 函数（检查模型配置依赖）
- [ ] T097 [US3] 在 src/io/import.rs 实现 preview_import_changes() 函数（显示导入预览：新增、冲突、跳过的预设）

#### CLI 命令 - import/export

- [ ] T098 [P] [US3] 在 src/cli/args.rs 扩展 PresetCommands 添加 Import 枚举（input, strategy, dry_run 参数）
- [ ] T099 [P] [US3] 在 src/cli/args.rs 扩展 PresetCommands 添加 Export 枚举（output, preset, include_models, include_active 参数）
- [ ] T100 [US3] 在 src/cli/commands.rs 实现 execute_preset_export() 函数
- [ ] T101 [US3] 在 src/cli/commands.rs 实现 execute_preset_import() 函数（集成验证、预览、确认）
- [ ] T102 [US3] 在 src/cli/commands.rs 实现 display_import_preview() 函数（显示差异预览并请求确认）

**检查点**: 所有用户故事现在应该独立可功能

---

## 阶段 6: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进

### 单元测试补充

- [ ] T103 [P] 为所有数据模型添加单元测试（Preset, PresetCollection, ExportPackage, BatchOperationResult）
- [ ] T104 [P] 为错误处理添加单元测试（所有 PresetError 变体的错误路径）
- [ ] T105 [P] 为验证逻辑添加单元测试（is_valid_preset_name, is_valid_version）

### 集成测试

- [ ] T106 [P] 在 tests/integration/preset_lifecycle_test.rs 创建预设生命周期集成测试（创建 → 应用 → 删除完整流程）
- [ ] T107 [P] 在 tests/integration/batch_concurrent_test.rs 创建批量操作并发测试（验证并发执行和错误隔离）
- [ ] T108 [P] 在 tests/integration/io_roundtrip_test.rs 创建导入导出集成测试（导出 → 导入 → 验证）
- [ ] T109 [P] 在 tests/integration/error_recovery_test.rs 创建错误恢复测试（备份、回滚、边界情况）
- [ ] T110 在 tests/integration/ 执行所有集成测试并验证通过

### 性能测试

- [ ] T111 [P] 在 tests/benchmark/preset_bench.rs 创建预设创建和应用基准测试（验证 SC-001: 30秒内）
- [ ] T112 [P] 在 tests/benchmark/batch_bench.rs 创建批量切换基准测试（验证 SC-002: 10秒内）
- [ ] T113 [P] 在 tests/benchmark/export_bench.rs 创建导出基准测试（验证 SC-003: 5秒内）
- [ ] T114 [P] 在 tests/benchmark/import_bench.rs 创建导入基准测试（验证 SC-004: 15秒内）
- [ ] T115 运行所有性能测试并验证符合成功标准

### 安全测试

- [ ] T116 [P] 在 tests/security/api_key_test.rs 创建 API Key 脱敏验证测试
- [ ] T117 [P] 在 tests/security/file_permissions_test.rs 创建文件权限检查测试（验证 600 权限）
- [ ] T118 [P] 在 tests/security/path_traversal_test.rs 创建路径遍历防护测试
- [ ] T119 [P] 在 tests/security/malicious_file_test.rs 创建恶意文件检测测试
- [ ] T120 运行所有安全测试并修复发现的问题

### 文档

- [ ] T121 为所有公开的 Rust 结构体和函数添加中文文档注释（/// 和 //! ）
- [ ] T122 在 src/presets/README.md 创建预设管理模块文档
- [ ] T123 在 src/batch/README.md 创建批量操作模块文档
- [ ] T124 在 src/io/README.md 创建导入导出模块文档
- [ ] T125 验证所有代码注释符合项目宪章要求（中文）

### 代码质量

- [ ] T126 运行 cargo fmt 格式化所有代码
- [ ] T127 运行 cargo clippy 并修复所有警告
- [ ] T128 使用 tarpaulin 或 cargo-llvm-cov 测量代码覆盖率（目标 ≥ 80%）
- [ ] T129 修复代码覆盖率不足的模块
- [ ] T130 运行 cargo test 确保所有测试通过

### 用户体验

- [ ] T131 验证 quickstart.md 中的所有示例命令可以正常工作
- [ ] T132 改进错误消息的可读性（中文，清晰，提供解决建议）
- [ ] T133 改进命令行输出的格式和颜色（使用 colored crate）
- [ ] T134 添加 --help 命令的详细示例

### 兼容性

- [ ] T135 在 Linux 上运行完整测试套件
- [ ] T136 在 macOS 上运行完整测试套件
- [ ] T137 在 Windows 上运行完整测试套件
- [ ] T138 验证跨平台文件路径处理正确（使用 dirs crate）

### 发布准备

- [ ] T139 更新 CHANGELOG.md 添加 v0.3.0 版本变更
- [ ] T140 更新 README.md 添加新功能使用说明
- [ ] T141 创建 Git 提交（遵循项目宪章的提交规范）
- [ ] T142 推送到远程分支并创建 Pull Request

---

## 依赖与执行顺序

### 阶段依赖

- **设置（阶段 1）**: 无依赖 - 可立即开始
- **基础设施（阶段 2）**: 依赖设置完成 - 阻塞所有用户故事
- **用户故事 1（阶段 3）**: 依赖基础设施完成 - 无其他故事依赖
- **用户故事 2（阶段 4）**: 依赖基础设施完成 - 可与 US1 并行开发
- **用户故事 3（阶段 5）**: 依赖基础设施完成 - 可与 US1/US2 并行开发
- **完善（阶段 6）**: 依赖所有期望的用户故事完成

### 用户故事依赖

- **用户故事 1 (P1 - 配置预设管理)**: 在基础设施（阶段 2）完成后可开始 - 无其他故事依赖
- **用户故事 2 (P2 - 批量操作)**: 在基础设施（阶段 2）完成后可开始 - 使用 US1 的 PresetStore，但独立可测试
- **用户故事 3 (P3 - 导入导出)**: 在基础设施（阶段 2）完成后可开始 - 使用 US1 的数据结构，但独立可测试

### 每个用户故事内

- 单元测试必须在实现前编写并失败
- 数据模型在业务逻辑之前
- 业务逻辑在 CLI 命令之前
- 核心实现在集成之前
- 故事完成后再进入下一个优先级

### 并行机会

**阶段 1（设置）**: T003, T004, T005, T006, T007 可并行运行

**阶段 2（基础设施）**:
- T010, T011, T013, T015, T016, T017, T018, T019（数据模型）可并行运行
- T024, T025（验证函数）可并行运行

**阶段 3（用户故事 1）**:
- T026, T027, T028, T029（测试）可并行运行
- T030, T031, T032, T033, T034, T035（CRUD 操作）可并行运行
- T044-T050（CLI 参数）可并行运行

**阶段 4（用户故事 2）**:
- T059, T060, T061（测试）可并行运行
- T062, T066, T069（核心功能）可并行运行
- T072-T075（CLI 参数）可并行运行

**阶段 5（用户故事 3）**:
- T080, T081, T082, T083（测试）可并行运行
- T084, T085, T086（脱敏）可并行运行
- T087, T088（导出）可并行运行
- T092, T093, T094（导入）可并行运行
- T098, T099（CLI 参数）可并行运行

**阶段 6（完善）**:
- T103-T105（单元测试补充）可并行运行
- T106-T109（集成测试）可并行运行
- T111-T114（性能测试）可并行运行
- T116-T119（安全测试）可并行运行
- T121-T124（文档）可并行运行

---

## 并行示例: 用户故事 1

```bash
# 一起启动用户故事 1 的所有测试:
任务: "T026 [P] [US1] Preset::validate() 的单元测试"
任务: "T027 [P] [US1] PresetStore::load() 和 save() 的单元测试"
任务: "T028 [P] [US1] add_preset() 的单元测试"
任务: "T029 [P] [US1] apply_preset() 的单元测试"

# 一起启动用户故事 1 的 CRUD 操作:
任务: "T030 [P] [US1] 实现 add_preset() 方法"
任务: "T031 [P] [US1] 实现 get_preset() 方法"
任务: "T032 [P] [US1] 实现 list_presets() 方法"
任务: "T033 [P] [US1] 实现 find_by_tag() 方法"
任务: "T034 [US1] 实现 update_preset() 方法"
任务: "T035 [US1] 实现 remove_preset() 方法"

# 一起启动用户故事 1 的 CLI 参数:
任务: "T044 [P] [US1] 添加 PresetCommands 枚举"
任务: "T045 [P] [US1] 添加 PresetCreateSubcommand 结构体"
任务: "T046 [P] [US1] 添加 PresetListSubcommand 结构体"
任务: "T047 [P] [US1] 添加 PresetApplySubcommand 结构体"
任务: "T048 [P] [US1] 添加 PresetUpdateSubcommand 结构体"
任务: "T049 [P] [US1] 添加 PresetDeleteSubcommand 结构体"
任务: "T050 [P] [US1] 添加 PresetValidateSubcommand 结构体"
```

---

## 实现策略

### MVP 优先（仅用户故事 1）

1. 完成阶段 1: 设置（T001-T007）
2. 完成阶段 2: 基础设施（T008-T025）- **关键阻塞点**
3. 完成阶段 3: 用户故事 1（T026-T058）
4. **停止并验证**: 独立测试用户故事 1（创建、列出、应用、更新、删除、验证预设）
5. 如果就绪则部署/演示 v0.3.0 MVP

### 增量交付

1. 完成设置 + 基础设施 → 基础就绪
2. 添加用户故事 1 → 独立测试 → 部署/演示（MVP：配置预设管理）
3. 添加用户故事 2 → 独立测试 → 部署/演示（增强：批量操作）
4. 添加用户故事 3 → 独立测试 → 部署/演示（完整：导入导出）
5. 每个故事增加价值而不破坏之前的故事

### 并行团队策略

有多个开发人员时:

1. 团队一起完成设置 + 基础设施
2. 基础设施完成后:
   - 开发人员 A: 用户故事 1（配置预设管理）
   - 开发人员 B: 用户故事 2（批量操作）
   - 开发人员 C: 用户故事 3（导入导出）
3. 故事独立完成和集成
4. 团队协作完成完善阶段（测试、文档、性能优化）

---

## 注意事项

- [P] 任务 = 不同文件，无依赖，可并行执行
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可独立完成和测试
- 实现前验证测试失败（TDD 红灯）
- 每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 测试覆盖率必须 ≥ 80%（项目宪章要求）
- 所有文档必须使用中文（项目宪章要求）
- 避免模糊任务、相同文件冲突、破坏独立性的跨故事依赖

---

**任务统计**:
- 总任务数: 142
- 阶段 1（设置）: 7 个任务
- 阶段 2（基础设施）: 18 个任务
- 阶段 3（用户故事 1）: 33 个任务
- 阶段 4（用户故事 2）: 21 个任务
- 阶段 5（用户故事 3）: 23 个任务
- 阶段 6（完善）: 40 个任务

**MVP 范围**: 阶段 1-3（58 个任务）- 配置预设管理功能

**并行机会**: 约 60% 的任务可并行执行（标记为 [P]）
