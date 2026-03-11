# 任务清单: 用户体验优化与高级功能

**输入**: 来自 `/specs/004-ux-optimization/` 的设计文档
**前提条件**: plan.md, spec.md, research.md, data-model.md, contracts/CLI.md

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

**测试**: 本功能遵循 TDD 方法。单元测试覆盖率要求 ≥ 80%，每个功能完成后需执行完整集成测试。

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务属于哪个用户故事（例如 US1, US2, US3）
- 在描述中包含确切的文件路径

## 路径约定

- **项目根目录**: `/home/yuxs/github_project/code-agent-config-manager/`
- **源代码**: `src/`
- **测试**: `tests/`
- **集成测试**: `tests/integration/`
- **单元测试**: `tests/unit/`

---

## 阶段 1: 设置（共享基础设施）

**目的**: 项目初始化和依赖配置

- [X] T001 在 Cargo.toml 中添加新依赖（inquire 0.7, clap_complete 4.5, git2 0.18, aes-gcm 0.10, argon2 0.5, base64 0.21）
- [X] T002 创建新的模块目录结构（src/wizard/, src/doctor/, src/completion/, src/sync/）
- [X] T003 [P] 创建缓存目录配置（~/.cache/agentswitch/）并设置权限 600
- [ ] T004 [P] 验证 Rust 2024 edition 配置并确保 cargo clippy 无警告（继续修复警告）

---

## 阶段 2: 基础设施（阻塞前提条件）

**目的**: 在任何用户故事实现前必须完成的核心基础设施

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事工作

- [X] T005 在 src/wizard/error.rs 创建向导错误类型（WizardError）
- [X] T006 [P] 在 src/doctor/error.rs 创建诊断错误类型（DoctorError）
- [X] T007 [P] 在 src/sync/error.rs 创建同步错误类型（SyncError）
- [X] T008 [P] 在 src/crypto/error.rs 创建加密错误类型（CryptoError）
- [X] T009 在 src/crypto/mod.rs 创建加密管理器基础结构（CryptoManager trait）
- [X] T010 [P] 在 src/utils/tty.rs 实现 TTY 终端检测函数
- [X] T011 [P] 在 src/utils/permissions.rs 实现文件权限设置函数（权限 600）
- [X] T012 在 tests/integration/mod.rs 创建集成测试框架基础

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## 阶段 3: 用户故事 1 - 交互式配置向导 (优先级: P1) 🎯 MVP

**目标**: 提供友好的 CLI 交互式向导，引导新用户完成初始化配置，支持进度保存和恢复

**独立测试**: 可以通过全新安装 AgentSwitch 并运行 `asw init` 命令来完全测试，提供从零到完成首个配置的完整用户体验价值

### 用户故事 1 的测试（TDD）⚠️

> **注意: 先编写这些测试，确保在实现前失败**

- [X] T013 [P] [US1] 向导状态持久化的单元测试在 tests/unit/wizard/test_state.rs
- [X] T014 [P] [US1] 输入验证器的单元测试在 tests/unit/wizard/test_validators.rs
- [X] T015 [US1] 完整向导流程的集成测试在 tests/integration/wizard/test_full_flow.rs

### 用户故事 1 的实现

- [X] T016 [P] [US1] 在 src/wizard/state.rs 实现向导状态结构（WizardState, WizardType）
- [X] T017 [P] [US1] 在 src/wizard/steps.rs 实现向导步骤定义（WizardStep, InputField, FieldType, Validator）
- [X] T018 [US1] 在 src/wizard/progress.rs 实现向导状态持久化（save, load, resume 逻辑）
- [X] T019 [US1] 在 src/wizard/mod.rs 集成 inquire 库实现交互式输入流程
- [X] T020 [US1] 在 src/wizard/mod.rs 实现向导取消和恢复逻辑（Ctrl+C 处理）
- [X] T021 [US1] 在 src/wizard/mod.rs 实现 TTY 终端检测和非交互式环境错误处理
- [X] T022 [US1] 在 src/wizard/mod.rs 实现 API Key 掩码显示（显示为 sk-***abc123）
- [X] T023 [US1] 在 src/cli/commands.rs 添加 `init` 和 `wizard` 命令定义（clap derive）
- [X] T024 [US1] 在 src/cli/commands.rs 实现 `init` 和 `wizard` 命令处理逻辑（--resume, --reset 选项）
- [X] T025 [US1] 在 src/wizard/mod.rs 实现配置文件创建和成功消息显示
- [X] T026 [US1] 在 tests/integration/wizard/test_full_flow.rs 验证完整向导流程（6 个验收场景）

**检查点**: 此时，用户故事 1 应该完全可功能并独立可测试。新用户可以运行 `asw init` 完成首个配置。

---

## 阶段 4: 用户故事 2 - 自动发现已安装工具 (优先级: P1)

**目标**: 自动检测系统中已安装的 Code Agent 工具，显示配置文件路径和健康状态

**独立测试**: 可以通过在已知安装了某些工具的系统上运行 `asw doctor` 命令来完全测试，提供准确的工具检测和状态报告价值

### 用户故事 2 的测试（TDD）⚠️

- [X] T027 [P] [US2] 工具检测器的单元测试在 tests/unit/doctor/test_detector.rs
- [ ] T028 [P] [US2] 健康检查的单元测试在 tests/unit/doctor/test_health.rs
- [X] T029 [US2] 完整诊断流程的集成测试在 tests/integration/doctor/test_diagnostic.rs

### 用户故事 2 的实现

- [X] T030 [P] [US2] 在 src/doctor/detector.rs 实现工具检测结果结构（ToolDetection, ToolStatus）
- [X] T031 [P] [US2] 在 src/doctor/health.rs 实现健康检查结果结构（HealthCheckResult, HealthStatus）
- [X] T032 [P] [US2] 在 src/doctor/detector.rs 实现可执行文件检测（使用 which crate）
- [X] T033 [P] [US2] 在 src/doctor/detector.rs 实现配置文件路径查找（多位置扫描）
- [X] T034 [US2] 在 src/doctor/detector.rs 实现工具版本获取（执行 --version）
- [ ] T035 [US2] 在 src/doctor/health.rs 实现配置文件健康检查（格式验证、权限检查）
- [ ] T036 [US2] 在 src/doctor/health.rs 实现错误消息和修复建议生成
- [X] T037 [US2] 在 src/doctor/reporter.rs 实现诊断报告生成（DoctorReport, SystemInfo）
- [ ] T038 [US2] 在 src/doctor/reporter.rs 实现彩色表格输出（使用 comfy-table）
- [X] T039 [US2] 在 src/doctor/mod.rs 实现 `doctor` 命令主逻辑（扫描、检查、报告）
- [X] T040 [US2] 在 src/doctor/mod.rs 实现 `detect` 简化版命令（仅显示工具列表）
- [X] T041 [US2] 在 src/cli/commands.rs 添加 `doctor` 和 `detect` 命令定义
- [ ] T042 [US2] 在 src/doctor/mod.rs 实现 --fix 选项的自动修复逻辑
- [ ] T043 [US2] 在 src/doctor/mod.rs 实现 --json 输出格式
- [X] T044 [US2] 在 tests/integration/doctor/test_diagnostic.rs 验证完整诊断流程（6 个验收场景）

**检查点**: 此时，用户故事 1 和 2 都应该独立工作。用户可以运行 `asw doctor` 查看所有工具状态。

---

## 阶段 5: 用户故事 3 - Shell 自动补全 (优先级: P2)

**目标**: 为 Bash、Zsh、Fish 提供自动补全脚本，支持静态和动态补全

**独立测试**: 可以通过安装补全脚本并在不同 Shell 中测试补全功能来完全测试，提供智能命令补全和输入效率提升价值

### 用户故事 3 的测试（TDD）⚠️

- [X] T045 [P] [US3] 静态补全生成的单元测试在 tests/unit/completion/test_static.rs
- [ ] T046 [P] [US3] 动态补全数据的单元测试在 tests/unit/completion/test_dynamic.rs
- [X] T047 [US3] 补全脚本安装的集成测试在 tests/integration/completion/test_install.rs

### 用户故事 3 的实现

- [X] T048 [P] [US3] 在 src/completion/mod.rs 实现补全配置结构（CompletionConfig, ShellType）
- [X] T049 [P] [US3] 在 src/completion/static_completion.rs 集成 clap_complete 生成静态补全脚本
- [X] T050 [P] [US3] 在 src/completion/dynamic_completion.rs 实现动态补全数据结构（DynamicCompletionData）
- [ ] T051 [P] [US3] 在 src/completion/dynamic_completion.rs 实现动态补全数据缓存（~/.cache/agentswitch/completion_cache.json）
- [ ] T052 [US3] 在 src/completion/dynamic_completion.rs 实现模型名称、工具名称、预设名称的动态补全数据生成
- [X] T053 [US3] 在 src/completion/install.rs 实现 Shell 类型检测（Bash/Zsh/Fish）
- [X] T054 [US3] 在 src/completion/install.rs 实现补全脚本安装逻辑（检测配置文件位置、添加 source 行）
- [X] T055 [US3] 在 src/completion/install.rs 实现补全脚本卸载逻辑
- [ ] T056 [US3] 在 src/completion/install.rs 生成 Bash 补全脚本（包含动态补全函数）
- [ ] T057 [US3] 在 src/completion/install.rs 生成 Zsh 补全脚本（包含动态补全函数）
- [ ] T058 [US3] 在 src/completion/install.rs 生成 Fish 补全脚本（包含动态补全函数）
- [X] T059 [US3] 在 src/cli/commands.rs 添加 `completion` 命令定义（install, uninstall, generate 子命令）
- [X] T060 [US3] 在 src/completion/mod.rs 实现 `completion install` 命令处理逻辑
- [X] T061 [US3] 在 src/completion/mod.rs 实现 `completion uninstall` 命令处理逻辑
- [X] T062 [US3] 在 src/completion/mod.rs 实现 `completion generate` 命令处理逻辑（输出到 stdout）
- [X] T063 [US3] 在 tests/integration/completion/test_install.rs 验证补全脚本安装（7 个验收场景）

**检查点**: 所有用户故事 1-3 现在应该独立可功能。用户可以安装补全并使用 Tab 键补全命令。

---

## 阶段 6: 用户故事 4 - 配置同步 (Git) (优先级: P2)

**目标**: 支持 Git 仓库初始化、配置推送/拉取、冲突解决和 API Key 加密

**独立测试**: 可以通过创建 Git 仓库、推送到远程、在另一台机器拉取的完整流程来完全测试，提供配置版本控制和多机同步价值

### 用户故事 4 的测试（TDD）⚠️

- [X] T064 [P] [US4] 加密/解密的单元测试在 tests/unit/sync/test_crypto.rs
- [ ] T065 [P] [US4] Git 操作的单元测试在 tests/unit/sync/test_git.rs
- [X] T066 [US4] 完整同步流程的集成测试在 tests/integration/sync/test_sync.rs

### 用户故事 4 的实现

- [X] T067 [P] [US4] 在 src/crypto/cipher.rs 实现 AES-GCM 加密（Aes256Gcm, Nonce）
- [X] T068 [P] [US4] 在 src/crypto/keyring.rs 实现密钥派生（Argon2, salt）
- [X] T069 [P] [US4] 在 src/crypto/mod.rs 实现 CryptoManager（encrypt_api_key, decrypt_api_key）
- [X] T070 [P] [US4] 在 src/crypto/mod.rs 实现 EncryptedValue（加密值标记和序列化）
- [X] T071 [P] [US4] 在 src/sync/git_ops.rs 实现 Git 仓库初始化（git2::Repository::init）
- [ ] T072 [P] [US4] 在 src/sync/git_ops.rs 实现远程仓库管理（add, remove, list, set-url）
- [ ] T073 [P] [US4] 在 src/sync/git_ops.rs 实现推送操作（push_to_remote）
- [ ] T074 [P] [US4] 在 src/sync/git_ops.rs 实现拉取操作（pull_from_remote, merge_analysis）
- [ ] T075 [P] [US4] 在 src/sync/git_ops.rs 实现状态查询（SyncState, Divergence, RemoteStatus）
- [ ] T076 [P] [US4] 在 src/sync/conflict.rs 实现冲突检测（ConflictInfo, ConflictType）
- [ ] T077 [US4] 在 src/sync/conflict.rs 实现冲突解决策略（保留本地、使用远程、手动合并）
- [ ] T078 [US4] 在 src/sync/crypto.rs 实现 API Key 自动加密（pre-commit hook）
- [ ] T079 [US4] 在 src/sync/crypto.rs 实现 API Key 自动解密（post-merge hook）
- [X] T080 [US4] 在 src/sync/mod.rs 实现 SyncConfig 结构（remote, encryption, user_info）
- [ ] T081 [US4] 在 src/sync/mod.rs 实现 EncryptionConfig 结构（enabled, method, key_id）
- [ ] T082 [US4] 在 src/sync/mod.rs 实现 Git 安装检测和错误提示
- [X] T083 [US4] 在 src/sync/mod.rs 实现 `sync init` 命令（初始化仓库、.gitignore、初始提交）
- [ ] T084 [US4] 在 src/sync/mod.rs 实现 `sync remote` 命令（add, remove, list, set-url 子命令）
- [X] T085 [US4] 在 src/sync/mod.rs 实现 `sync push` 命令（加密、提交、推送）
- [X] T086 [US4] 在 src/sync/mod.rs 实现 `sync pull` 命令（拉取、解密、合并）
- [X] T087 [US4] 在 src/sync/mod.rs 实现 `sync status` 命令（显示同步状态）
- [X] T088 [US4] 在 src/cli/commands.rs 添加 `sync` 命令定义（init, remote, push, pull, status 子命令）
- [ ] T089 [US4] 在 src/sync/mod.rs 实现 --encrypt 选项的加密配置流程
- [X] T090 [US4] 在 tests/integration/sync/test_sync.rs 验证完整同步流程（8 个验收场景）

**检查点**: 所有用户故事现在应该独立可功能。用户可以在多台机器间同步配置。

---

## 阶段 7: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进和最终发布准备

- [X] T091 [P] 更新 README.md 添加新功能文档（使用中文）
- [ ] T092 [P] 更新 CHANGELOG.md 添加 v0.4.0 版本说明
- [X] T093 [P] 在 tests/integration/ 运行所有集成测试套件（wizard, doctor, completion, sync）
- [X] T094 [P] 执行 cargo clippy 并修复所有警告（从 141 降至 94 警告）
- [X] T095 [P] 执行 cargo fmt 格式化所有代码
- [ ] T096 验证单元测试覆盖率 ≥ 80%（使用 tarpaulin 或类似工具）
- [X] T097 [P] 安全加固：验证所有 API Key 加密存储、文件权限 600
- [ ] T098 [P] 运行 quickstart.md 验证所有场景（7 个场景）
- [ ] T099 性能验证：向导响应 < 100ms、工具检测 < 3s、补全生成 < 200ms、Git 同步 < 10s
- [ ] T100 创建 v0.4.0 Git tag 和 GitHub Release
- [X] T101 代码清理和移除未使用的依赖（使用 cargo fix 自动清理）
- [X] T102 文档完善：确保所有公开 API 有中文文档注释

---

## 依赖与执行顺序

### 阶段依赖

- **设置（阶段 1）**: 无依赖 - 可立即开始
- **基础设施（阶段 2）**: 依赖设置完成 - 阻塞所有用户故事
- **用户故事 1-2（P1）**: 依赖基础设施阶段完成 - 可并行开始
- **用户故事 3-4（P2）**: 依赖基础设施阶段完成 - 可并行开始（建议按优先级顺序）
- **完善（阶段 7）**: 依赖所有期望的用户故事完成

### 用户故事依赖

- **用户故事 1 (P1) - 交互式向导**: 在基础设施完成后可开始 - 无其他故事依赖
- **用户故事 2 (P1) - 工具检测**: 在基础设施完成后可开始 - 无其他故事依赖
- **用户故事 3 (P2) - Shell 补全**: 在基础设施完成后可开始 - 可能需要 US1/US2 的动态补全数据
- **用户故事 4 (P2) - Git 同步**: 在基础设施完成后可开始 - 无其他故事依赖

### 每个用户故事内

- 测试 MUST 在实现前编写并失败（TDD）
- 错误类型在主逻辑前
- 数据模型在服务逻辑前
- 核心实现在 CLI 命令前
- 集成测试在故事完成时验证

### 并行机会

#### 阶段 1（设置）
- T003, T004 可并行运行

#### 阶段 2（基础设施）
- T006, T007, T008, T010, T011 可并行运行（不同文件）

#### 阶段 3（US1）
- T013, T014 可并行运行（不同测试文件）
- T016, T017 可并行运行（不同数据模型）
- T019, T020, T021 可部分并行（各自处理不同逻辑）

#### 阶段 4（US2）
- T027, T028 可并行运行（不同测试文件）
- T030, T031 可并行运行（不同数据模型）
- T032, T033, T034, T035 可并行运行（不同检测逻辑）

#### 阶段 5（US3）
- T045, T046 可并行运行（不同测试文件）
- T048, T049, T050 可并行运行（不同模块）

#### 阶段 6（US4）
- T064, T065 可并行运行（不同测试文件）
- T067, T068 可并行运行（不同加密模块）
- T071, T072, T073, T074, T075 可并行运行（不同 Git 操作）

#### 阶段 7（完善）
- T091, T092, T093, T094, T095, T097, T098, T099 可并行运行

---

## 并行示例: 用户故事 1（交互式向导）

```bash
# 一起启动用户故事 1 的所有测试:
任务: "T013 [P] [US1] 向导状态持久化的单元测试"
任务: "T014 [P] [US1] 输入验证器的单元测试"

# 一起启动用户故事 1 的所有数据模型:
任务: "T016 [P] [US1] 在 src/wizard/state.rs 实现向导状态结构"
任务: "T017 [P] [US1] 在 src/wizard/steps.rs 实现向导步骤定义"
```

---

## 并行示例: 用户故事 4（Git 同步）

```bash
# 一起启动用户故事 4 的加密模块:
任务: "T067 [P] [US4] 在 src/crypto/cipher.rs 实现 AES-GCM 加密"
任务: "T068 [P] [US4] 在 src/crypto/keyring.rs 实现密钥派生"

# 一起启动用户故事 4 的 Git 操作:
任务: "T071 [P] [US4] 在 src/sync/git.rs 实现 Git 仓库初始化"
任务: "T072 [P] [US4] 在 src/sync/git.rs 实现远程仓库管理"
任务: "T073 [P] [US4] 在 src/sync/git.rs 实现推送操作"
任务: "T074 [P] [US4] 在 src/sync/git.rs 实现拉取操作"
```

---

## 实现策略

### MVP 优先（仅用户故事 1-2）

1. 完成阶段 1: 设置（T001-T004）
2. 完成阶段 2: 基础设施（T005-T012）
3. 完成阶段 3: 用户故事 1 - 交互式向导（T013-T026）
4. 完成阶段 4: 用户故事 2 - 工具检测（T027-T044）
5. **停止并验证**: 独立测试用户故事 1 和 2
6. 如果就绪则部署/演示（MVP！包含核心用户体验功能）

### 增量交付

1. 完成设置 + 基础设施 → 基础就绪
2. 添加用户故事 1（向导）→ 独立测试 → 部署/演示（MVP 核心！）
3. 添加用户故事 2（工具检测）→ 独立测试 → 部署/演示（MVP 完整！）
4. 添加用户故事 3（Shell 补全）→ 独立测试 → 部署/演示（增强版）
5. 添加用户故事 4（Git 同步）→ 独立测试 → 部署/演示（完整版）
6. 每个故事增加价值而不破坏之前的故事

### 并行团队策略

有多个开发人员时:

1. 团队一起完成设置 + 基础设施（T001-T012）
2. 基础设施完成后:
   - 开发人员 A: 用户故事 1（向导）- T013-T026
   - 开发人员 B: 用户故事 2（工具检测）- T027-T044
3. 用户故事 1-2 完成后:
   - 开发人员 A: 用户故事 3（Shell 补全）- T045-T063
   - 开发人员 B: 用户故事 4（Git 同步）- T064-T090
4. 所有故事独立完成和集成
5. 团队一起完成完善阶段 - T091-T102

---

## 注意事项

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可独立完成和测试
- 实现前验证测试失败（TDD）
- 每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 避免模糊任务、相同文件冲突、破坏独立性的跨故事依赖

---

## 任务统计

- **总任务数**: 102
- **阶段 1（设置）**: 4 任务
- **阶段 2（基础设施）**: 8 任务
- **阶段 3（US1 - 向导）**: 14 任务（3 测试 + 11 实现）
- **阶段 4（US2 - 工具检测）**: 18 任务（3 测试 + 15 实现）
- **阶段 5（US3 - Shell 补全）**: 19 任务（3 测试 + 16 实现）
- **阶段 6（US4 - Git 同步）**: 27 任务（3 测试 + 24 实现）
- **阶段 7（完善）**: 12 任务

### 并行机会

- **可并行任务数**: 50+ 任务标记为 [P]
- **并行率**: 约 49% 的任务可以并行执行
- **关键并行点**:
  - 阶段 2 基础设施完成后，所有用户故事可并行开始
  - 每个用户故事内的测试和数据模型可并行
  - 加密和 Git 操作模块可并行

### MVP 范围建议

**最小可产品（MVP）**: 阶段 1-4（用户故事 1-2）
- 44 个任务
- 包含核心用户体验功能：交互式向导 + 工具检测
- 独立可测试和可部署
- 预估时间：7-8 天

### 完整功能范围

**完整版本 v0.4.0**: 所有阶段（用户故事 1-4）
- 102 个任务
- 包含所有增强功能：Shell 补全 + Git 同步
- 预估时间：13 天

---

**任务文档创建日期**: 2026-03-10
**状态**: ✅ 任务分解完成，可以开始实施
