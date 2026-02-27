# 任务清单: AgentSwitch 核心基础功能

**输入**: 来自 `/specs/001-core-foundation/` 的设计文档
**前提条件**: plan.md（✅ 已完成），spec.md（✅ 已完成），research.md（✅ 已完成），data-model.md（✅ 已完成）

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

**测试**: 本项目需要完整的单元测试和集成测试，覆盖率目标 ≥ 80%

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务属于哪个用户故事（例如 US1, US2, US3）
- 包含确切的文件路径

## 路径约定

- **单项目**: 仓库根目录的 `src/`, `tests/`
- 以下显示的路径基于 plan.md 中定义的项目结构

## 阶段 1: 设置（共享基础设施）

**目的**: 项目初始化和基本结构

- [ ] T001 创建 Cargo.toml 配置文件，设置 edition = "2024"，添加依赖项（clap、serde、toml、anyhow、colored、dirs、url、comfy-table）
- [ ] T002 初始化 Rust 项目结构，创建 src/ 目录及所有子模块目录（cli/、config/、output/、utils/）
- [ ] T003 [P] 创建 .gitignore 文件，配置 /target、Cargo.lock 等忽略规则
- [ ] T004 [P] 创建 tests/ 目录结构（integration/、unit/、fixtures/）
- [ ] T005 [P] 配置 pre-commit hook 或 CI 脚本，集成 `cargo clippy` 和 `cargo fmt` 检查

---

## 阶段 2: 基础设施（阻塞前提条件）

**目的**: 在任何用户故事实现前必须完成的核心基础设施

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事工作

### 数据模型与配置存储

- [ ] T006 在 src/config/models.rs 中创建 ModelConfig 结构体（字段：name、base_url、api_key、model_id、extra_params），实现 Serialize/Deserialize trait
- [ ] T007 在 src/config/models.rs 中创建 AppConfig 结构体（字段：models、active_models），实现 Serialize/Deserialize、Default trait
- [ ] T008 在 src/config/models.rs 中为 ModelConfig 实现 new() 方法，为 AppConfig 实现 new()、add_model()、get_model()、remove_model()、edit_model() 方法
- [ ] T009 在 src/config/store.rs 中创建 ConfigStore 结构体，实现 new() 方法用于自动初始化检测和配置目录创建
- [ ] T010 在 src/config/store.rs 中实现 ConfigStore::load() 方法，从 ~/.agentswitch/config.toml 读取配置，文件不存在时返回默认配置
- [ ] T011 在 src/config/store.rs 中实现 ConfigStore::save() 方法，将配置写入 TOML 文件并设置权限为 0600

### 错误处理与验证

- [ ] T012 在 src/utils/validation.rs 中实现 validate_url() 函数，使用 url crate 验证 URL 格式
- [ ] T013 在 src/utils/validation.rs 中实现 validate_model_name() 函数，验证名称非空且不包含特殊字符
- [ ] T014 在 src/utils/permissions.rs 中实现 set_file_permissions() 函数，使用条件编译支持 Unix（0o600）和 Windows 权限设置

### 模块导出

- [ ] T015 创建 src/config/mod.rs，导出 models 和 store 模块
- [ ] T016 创建 src/utils/mod.rs，导出 validation 和 permissions 模块

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## 阶段 3: 用户故事 1 - 配置自动初始化与数据持久化 (优先级: P1) 🎯 MVP

**目标**: 实现首次运行时自动创建配置目录和默认配置文件，并确保数据往返一致性

**独立测试**: 通过首次执行任何 `asw` 命令（如 `asw model list`）测试自动初始化，验证目录创建、文件生成和幂等性

### 用户故事 1 的集成测试 ⚠️

> **注意: 先编写这些测试，确保在实现前失败**

- [ ] T017 [P] [US1] 在 tests/integration/config_init.rs 中编写测试：首次运行时自动创建 ~/.agentswitch/ 目录和 config.toml 文件
- [ ] T018 [P] [US1] 在 tests/integration/config_init.rs 中编写测试：配置目录已存在时跳过创建（幂等操作）
- [ ] T019 [P] [US1] 在 tests/integration/data_integrity.rs 中编写测试：往返一致性 - 序列化后反序列化得到与原始对象等价的结果

### 用户故事 1 的实现

- [ ] T020 [US1] 在 src/config/store.rs 中实现 ConfigStore::ensure_initialized() 方法，检测并创建配置目录（如果不存在）
- [ ] T021 [US1] 在 src/config/store.rs 中实现 ConfigStore::create_default_config() 方法，生成包含空模型列表的默认 TOML 配置
- [ ] T022 [US1] 在 src/config/store.rs 中增强 new() 方法，调用 ensure_initialized() 和 load()，确保自动初始化流程
- [ ] T023 [US1] 在 src/config/store.rs 中添加错误处理，捕获权限不足等错误并返回友好的 anyhow::Error
- [ ] T024 [US1] 在 src/utils/permissions.rs 中完善 set_file_permissions()，权限设置失败时记录警告但不阻塞（使用 anyhow::Context）

**检查点**: 此时，配置自动初始化功能应完全可功能并独立可测试

---

## 阶段 4: 用户故事 2 - 模型配置管理命令 (优先级: P1) 🎯 MVP

**目标**: 实现完整的 CRUD 操作（add/list/remove/edit），提供用户与系统交互的主要方式

**独立测试**: 通过执行完整的 CRUD 流程测试：add → list → edit → remove，每个步骤都有明确的输出和验证点

### 用户故事 2 的集成测试 ⚠️

> **注意: 先编写这些测试，确保在实现前失败**

- [ ] T025 [P] [US2] 在 tests/integration/model_crud.rs 中编写测试：添加模型配置成功并保存到文件
- [ ] T026 [P] [US2] 在 tests/integration/model_crud.rs 中编写测试：列出所有模型配置，验证表格输出
- [ ] T027 [P] [US2] 在 tests/integration/model_crud.rs 中编写测试：编辑模型配置，验证更新后的值
- [ ] T028 [P] [US2] 在 tests/integration/model_crud.rs 中编写测试：删除模型配置，验证文件中已移除
- [ ] T029 [P] [US2] 在 tests/integration/model_crud.rs 中编写测试：添加同名模型时返回错误

### CLI 框架搭建

- [ ] T030 [P] [US2] 在 src/cli/args.rs 中使用 clap derive 创建 Cli 结构体和 ModelSubcommand 枚举（Add、List、Remove、Edit）
- [ ] T031 [P] [US2] 在 src/cli/args.rs 中定义 ModelAddArgs 结构体（name、base_url、api_key、model 字段）
- [ ] T032 [P] [US2] 在 src/cli/args.rs 中定义 ModelEditArgs 结构体（name、base_url、api_key、model 可选字段）
- [ ] T033 [P] [US2] 在 src/cli/args.rs 中定义 ModelRemoveArgs 和 ModelListArgs 结构体

### 用户故事 2 的实现

- [ ] T034 [US2] 在 src/cli/commands.rs 中实现 execute_add_model() 函数，验证输入、调用 ConfigStore::add_model()、保存配置并输出成功信息
- [ ] T035 [US2] 在 src/cli/commands.rs 中实现 execute_list_models() 函数，调用 ConfigStore::list_models()、格式化为表格输出
- [ ] T036 [US2] 在 src/cli/commands.rs 中实现 execute_remove_model() 函数，调用 ConfigStore::remove_model()、保存配置并输出成功信息
- [ ] T037 [US2] 在 src/cli/commands.rs 中实现 execute_edit_model() 函数，验证输入、调用 ConfigStore::edit_model()、保存配置并输出成功信息
- [ ] T038 [US2] 在 src/cli/commands.rs 中添加输入验证逻辑，检查模型名称唯一性、URL 格式、参数非空等
- [ ] T039 [US2] 在 src/cli/commands.rs 中增强错误处理，为不同错误场景提供友好的错误消息（名称重复、模型不存在、URL 无效等）
- [ ] T040 [US2] 在 src/config/store.rs 中实现 ConfigStore::add_model() 方法，检查名称重复、添加到 AppConfig.models
- [ ] T041 [US2] 在 src/config/store.rs 中实现 ConfigStore::list_models() 方法，返回 AppConfig.models 列表
- [ ] T042 [US2] 在 src/config/store.rs 中实现 ConfigStore::remove_model() 方法，删除模型配置并清除 active_models 映射
- [ ] T043 [US2] 在 src/config/store.rs 中实现 ConfigStore::edit_model() 方法，使用闭包更新模型配置的可选字段

### 程序入口与命令路由

- [ ] T044 [US2] 创建 src/cli/mod.rs，导出 args 和 commands 模块
- [ ] T045 [US2] 在 src/main.rs 中实现 main() 函数，初始化 ConfigStore（自动初始化）、解析命令行参数、路由到对应的命令执行函数

**检查点**: 此时，用户故事 1 和 2 都应该完全可功能并独立可测试

---

## 阶段 5: 用户故事 3 - 安全性与用户体验增强 (优先级: P2)

**目标**: 实现彩色输出、API Key 掩码显示和文件权限保护，提供清晰的用户反馈

**独立测试**: 通过验证错误场景、警告信息和 API Key 掩码显示来测试

### 用户故事 3 的集成测试 ⚠️

> **注意: 先编写这些测试，确保在实现前失败**

- [ ] T046 [P] [US3] 在 tests/unit/formatter_tests.rs 中编写测试：API Key 掩码显示（仅前 4 位 + ****）
- [ ] T047 [P] [US3] 在 tests/unit/formatter_tests.rs 中编写测试：表格格式化输出验证列对齐和边框
- [ ] T048 [P] [US3] 在 tests/integration/model_crud.rs 中编写测试：配置文件权限验证（0600）

### 用户故事 3 的实现

- [ ] T049 [US3] 在 src/output/theme.rs 中实现 print_success() 函数，使用 colored 输出绿色文本和 ✓ 前缀
- [ ] T050 [US3] 在 src/output/theme.rs 中实现 print_error() 函数，使用 colored 输出红色文本和 ✗ 前缀，包含详细错误信息
- [ ] T051 [US3] 在 src/output/theme.rs 中实现 print_warning() 函数，使用 colored 输出黄色文本和 💡 前缀
- [ ] T052 [US3] 在 src/output/theme.rs 中实现 print_info() 函数，使用 colored 输出蓝色文本和 ℹ 前缀
- [ ] T053 [US3] 在 src/output/formatter.rs 中实现 mask_api_key() 函数，截取前 4 个字符并添加 **** 后缀
- [ ] T054 [US3] 在 src/output/formatter.rs 中实现 format_models_table() 函数，使用 comfy-table 生成表格（列：Name、Base URL、Model ID、API Key）
- [ ] T055 [US3] 在 src/output/formatter.rs 中增强 format_models_table()，对 API Key 列应用 mask_api_key() 处理
- [ ] T056 [US3] 在 src/output/formatter.rs 中实现空状态提示，当没有模型配置时输出黄色建议信息
- [ ] T057 [US3] 在 src/cli/commands.rs 中替换所有 println!/eprintln! 调用为 theme 模块的输出函数（print_success、print_error 等）
- [ ] T058 [US3] 在 src/cli/commands.rs 中的 execute_list_models() 调用 format_models_table() 格式化输出
- [ ] T059 [US3] 在 src/config/store.rs 中的 save() 方法调用 set_file_permissions() 设置文件权限为 0600
- [ ] T060 [US3] 在 src/config/store.rs 中增强 save() 错误处理，权限设置失败时使用 print_warning() 输出警告但继续运行

### 模块导出

- [ ] T061 创建 src/output/mod.rs，导出 formatter 和 theme 模块

**检查点**: 所有用户故事现在应该完全可功能

---

## 阶段 6: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进

### 文档与代码质量

- [ ] T062 [P] 完善 README.md，添加项目介绍、安装说明、快速开始指南
- [ ] T063 [P] 在 src/main.rs 添加命令行帮助信息，确保 `asw --help` 和 `asw model --help` 提供清晰的用法说明
- [ ] T064 运行 `cargo clippy` 修复所有警告，确保代码通过 lint 检查
- [ ] T065 运行 `cargo fmt` 格式化所有代码，确保代码风格一致

### 单元测试增强

- [ ] T066 [P] 在 tests/unit/config_tests.rs 中为 ModelConfig 和 AppConfig 的所有方法编写单元测试
- [ ] T067 [P] 在 tests/unit/validation_tests.rs 中为 URL 验证、参数验证编写单元测试，覆盖边界情况
- [ ] T068 [P] 在 tests/unit/validation_tests.rs 中编写测试：空参数、无效 URL、特殊字符等场景
- [ ] T069 [P] 在 tests/unit/config_tests.rs 中编写测试：往返一致性测试（序列化/反序列化）

### 集成测试完善

- [ ] T070 在 tests/integration/model_crud.rs 中添加端到端测试：完整的 CRUD 流程（add → list → edit → list → remove → list）
- [ ] T071 在 tests/integration/config_init.rs 中添加测试：首次使用场景（从零开始，首次运行任何命令时自动初始化）
- [ ] T072 在 tests/integration/config_init.rs 中添加测试：错误恢复场景（配置文件损坏、权限不足）
- [ ] T073 在 tests/integration/model_crud.rs 中添加测试：边界条件测试（空参数、超长输入、特殊字符）

### 安全加固

- [ ] T074 验证 API Key 在所有命令输出中都已掩码（list、add 成功信息等）
- [ ] T075 验证配置文件权限在所有场景下都是 0600（首次创建、后续保存）
- [ ] T076 审查所有错误消息，确保不泄露敏感信息（完整的 API Key、文件路径等）

### 性能优化（可选）

- [ ] T077 使用 `cargo fmt` 和 `cargo clippy` 验证代码质量和性能
- [ ] T078 运行集成测试，确保配置切换时间 < 100ms、启动时间 < 50ms

### 测试覆盖率验证

- [ ] T079 运行 `cargo test --all-features` 执行所有测试（单元 + 集成）
- [ ] T080 使用 tarpaulin 或 cargo-llvm-cov 测量测试覆盖率，确保 ≥ 80%
- [ ] T081 如果覆盖率不足，补充额外的单元测试

---

## 依赖与执行顺序

### 阶段依赖

- **设置（阶段 1）**: 无依赖 - 可立即开始
- **基础设施（阶段 2）**: 依赖设置完成 - **阻塞所有用户故事**
- **用户故事 1（阶段 3）**: 依赖基础设施完成 - **阻塞用户故事 2 和 3**
- **用户故事 2（阶段 4）**: 依赖基础设施和用户故事 1 完成
- **用户故事 3（阶段 5）**: 依赖用户故事 2 完成（增强输出和安全性）
- **完善（阶段 6）**: 依赖所有用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: 实现配置自动初始化和数据持久化，提供基础数据模型
- **用户故事 2 (P1)**: 依赖用户故事 1 的数据模型和 ConfigStore，实现 CRUD 命令
- **用户故事 3 (P2)**: 依赖用户故事 2 的命令框架，增强输出格式和安全性

### 每个用户故事内

- 集成测试 MUST 在实现前编写并失败
- 数据模型在配置存储之前
- 配置存储在命令实现之前
- 命令实现在程序入口之前
- 输出格式化与命令实现同步进行

### 并行机会

#### 阶段 1（设置）
- T003, T004, T005 可并行运行（不同文件）

#### 阶段 2（基础设施）
- T006, T007 可并行运行（同一文件但不同结构体）
- T009, T010, T011 顺序执行（依赖前面的数据模型）
- T012, T013 可并行运行（不同验证函数）
- T015, T016 可并行运行（不同模块）

#### 阶段 3（用户故事 1）
- T017, T018, T019 可并行运行（不同测试文件）

#### 阶段 4（用户故事 2）
- T025, T026, T027, T028, T029 可并行运行（不同测试）
- T030, T031, T032, T033 可并行运行（不同结构体）

#### 阶段 5（用户故事 3）
- T046, T047, T048 可并行运行（不同测试）
- T049, T050, T051, T052 可并行运行（不同输出函数）

#### 阶段 6（完善）
- T062, T063 可并行运行（不同文档）
- T066, T067, T068, T069 可并行运行（不同测试文件）

---

## 并行示例: 阶段 2（基础设施）

```bash
# 一起启动数据模型测试:
任务: "在 tests/unit/config_tests.rs 为 ModelConfig 和 AppConfig 编写单元测试"

# 一起启动验证逻辑:
任务: "在 src/utils/validation.rs 实现 validate_url() 函数"
任务: "在 src/utils/validation.rs 实现 validate_model_name() 函数"
```

---

## 实现策略

### MVP 优先（用户故事 1 + 2）

1. 完成阶段 1: 设置（T001-T005）
2. 完成阶段 2: 基础设施（T006-T016）**关键 - 阻塞所有故事**
3. 完成阶段 3: 用户故事 1（T017-T024）
4. 完成阶段 4: 用户故事 2（T025-T045）
5. **停止并验证**: 独立测试用户故事 1 和 2 的完整流程
6. 如果就绪则部署/演示（MVP 可用）

### 增量交付

1. 完成设置 + 基础设施 → 基础就绪
2. 添加用户故事 1 → 测试自动初始化 → 核心数据持久化可用
3. 添加用户故事 2 → 测试 CRUD 操作 → MVP 完整可用！
4. 添加用户故事 3 → 测试彩色输出和安全性 → 用户体验增强
5. 每个故事增加价值而不破坏之前的故事

### 完整实现

1. 按顺序完成所有阶段（1-6）
2. 每个阶段内充分利用并行机会
3. 每个用户故事完成后立即验证
4. 最后进行集成测试和覆盖率验证

---

## 注意事项

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可独立完成和测试
- 实现前验证测试失败（TDD）
- 每个任务或逻辑组后提交（使用合理的提交信息）
- 在任何检查点停止以独立验证故事
- 避免: 模糊任务、相同文件冲突、破坏独立性的跨故事依赖
- **集成测试强制要求**: 每个功能完成后必须执行完整的集成测试（T017-T019, T025-T029, T046-T048, T070-T073）
- **中文文档要求**: 所有代码注释使用中文，README 和帮助信息使用中文
- **Rust 2024 标准**: Cargo.toml 中设置 edition = "2024"，通过 clippy 检查

---

## 任务统计

- **总任务数**: 81
- **阶段 1（设置）**: 5 个任务
- **阶段 2（基础设施）**: 11 个任务
- **阶段 3（用户故事 1）**: 8 个任务
- **阶段 4（用户故事 2）**: 20 个任务
- **阶段 5（用户故事 3）**: 16 个任务
- **阶段 6（完善）**: 21 个任务

**预计工作量**: MVP（阶段 1-4）约 49 个任务，完整实现（阶段 1-6）81 个任务
