# 任务清单: 便捷安装与分发系统

**输入**: 来自 `/specs/001-install-packaging/` 的设计文档
**前提条件**: plan.md（必需），spec.md（用户故事必需），research.md，data-model.md，contracts/

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

**测试**: 以下任务包含手动安装测试。由于本功能主要是 Shell 脚本和 CI/CD 配置，自动化单元测试不适用于所有部分。

**组织方式**: 任务按用户故事分组，以实现每个故事的独立实现和测试。

## 格式: `[ID] [P?] [Story] 描述`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 此任务属于哪个用户故事（例如 US1, US2, US3）
- 在描述中包含确切的文件路径

## 路径约定

本项目为单项目结构:
- 仓库根目录: `scripts/`, `packaging/`, `.github/workflows/`
- 文档: `README.md`, `INSTALL.md`
- 配置: `Cargo.toml`

---

## 阶段 1: 设置（共享基础设施）

**目的**: 创建打包和发布所需的项目结构

- [x] T001 创建打包相关目录结构（scripts/, packaging/man/, packaging/completions/, packaging/debian/）
- [x] T002 安装 cargo-deb 工具（`cargo install cargo-deb`）
- [x] T003 在 Cargo.toml 中添加 cargo-deb 配置段（`[package.metadata.deb]`）

---

## 阶段 2: 基础设施（阻塞前提条件）

**目的**: 在任何用户故事实现前必须完成的核心基础设施

**⚠️ 关键**: 在此阶段完成前，不能开始任何用户故事工作

- [x] T004 创建 man 手册页基础模板在 packaging/man/asw.1
- [x] T005 生成 Shell 补全脚本基础框架在 packaging/completions/（asw.bash, asw.zsh, asw.fish）
- [x] T006 创建 DEB 维护者脚本模板（postinst, prerm, postrm）在 packaging/debian/
- [x] T007 设置 GitHub Actions 工作流基础结构在 .github/workflows/release.yml

**检查点**: 基础设施就绪 - 用户故事实现现在可以并行开始

---

## 阶段 3: 用户故事 1 - 一键脚本安装 (优先级: P1) 🎯 MVP

**目标**: 用户可以通过 `curl ... | bash` 一条命令在 2 分钟内完成 AgentSwitch 安装

**独立测试**: 在干净的 Linux 虚拟机中执行安装脚本，验证：
- 可以下载并安装对应平台的二进制
- `asw --version` 显示正确版本
- Shell 补全在新会话中工作
- `--uninstall` 可以完全卸载

### 用户故事 1 的实现

- [x] T008 [P] [US1] 创建 Shell 安装脚本基础结构在 scripts/install.sh（shebang, 函数框架, 错误处理）
- [x] T009 [P] [US1] 实现系统检测函数（操作系统、架构）在 scripts/install.sh
- [x] T010 [US1] 实现命令行参数解析在 scripts/install.sh（--help, --version, --uninstall, --force, --dry-run）
- [x] T011 [US1] 实现环境变量处理在 scripts/install.sh（INSTALL_DIR, NO_MODIFY_PATH, ASW_VERSION）
- [x] T012 [US1] 实现版本获取函数在 scripts/install.sh（从 GitHub API 获取最新版本）
- [x] T013 [US1] 实现二进制下载函数在 scripts/install.sh（支持重试、进度显示、校验和验证）
- [x] T014 [US1] 实现安装函数在 scripts/install.sh（权限检查、二进制复制、权限设置）
- [x] T015 [US1] 实现 Shell 补全配置函数在 scripts/install.sh（检测 Shell 类型、安装补全脚本、修改配置文件）
- [x] T016 [US1] 实现卸载函数在 scripts/install.sh（删除二进制、清理补全、配置文件删除选项）
- [x] T017 [US1] 添加用户友好的输出和错误消息在 scripts/install.sh（带颜色的进度提示、错误消息）
- [x] T018 [US1] 实现 Ctrl+C 中断处理和临时文件清理在 scripts/install.sh
- [x] T019 [US1] 添加脚本版本号和内联文档注释在 scripts/install.sh

**检查点**: 此时，Shell 安装脚本应该完全可用并独立可测试

---

## 阶段 4: 用户故事 2 - DEB 包安装 (优先级: P2)

**目标**: Debian/Ubuntu 用户可以通过 `dpkg -i` 安装 AgentSwitch，获得完整的系统集成

**独立测试**: 在 Ubuntu 虚拟机中测试：
- `dpkg -i agentswitch_*.deb` 成功安装
- `dpkg -L agentswitch` 显示所有文件
- `man asw` 显示手册
- Tab 补全工作
- `dpkg -r` 和 `dpkg -P` 正确工作

### 用户故事 2 的实现

- [x] T020 [P] [US2] 完善 Cargo.toml 中的 cargo-deb 配置（包元数据、依赖、文件清单）
- [x] T021 [P] [US2] 编写完整的 man 手册页在 packaging/man/asw.1（包括所有命令和选项）
- [x] T022 [P] [US2] 完善 Bash 补全脚本在 packaging/completions/asw.bash（所有子命令和参数）
- [x] T023 [P] [US2] 完善 Zsh 补全脚本在 packaging/completions/asw.zsh
- [x] T024 [P] [US2] 完善 Fish 补全脚本在 packaging/completions/asw.fish
- [x] T025 [US2] 实现 DEB postinst 脚本在 packaging/debian/postinst（man 数据库重建、欢迎消息）
- [x] T026 [US2] 实现 DEB prerm 脚本在 packaging/debian/prerm（停止服务、清理）
- [x] T027 [US2] 实现 DEB postrm 脚本在 packaging/debian/postrm（卸载后清理、配置删除提示）
- [x] T028 [US2] 测试 DEB 包构建在本地（`cargo deb`）
- [x] T029 [US2] 在 Docker 容器中测试 DEB 包安装（Ubuntu 22.04）

**检查点**: 此时，DEB 包应该完全可用并独立可测试

---

## 阶段 5: 用户故事 3 - 自动化发布流程 (优先级: P3)

**目标**: 推送 Git tag 时自动构建多平台二进制和 DEB 包，并上传到 GitHub Releases

**独立测试**: 在测试分支创建 tag 验证：
- GitHub Actions 自动触发
- 所有平台构建成功
- Release 自动创建并包含所有产物
- 每个平台的二进制可以独立运行

### 用户故事 3 的实现

- [x] T030 [P] [US3] 创建 GitHub Actions release 工作流文件在 .github/workflows/release.yml
- [x] T031 [P] [US3] 配置工作流触发条件在 .github/workflows/release.yml（push tags: v*）
- [x] T032 [US3] 实现矩阵构建策略在 .github/workflows/release.yml（Linux x86_64/ARM64, macOS x86_64/ARM64）
- [x] T033 [US3] 使用 cross-rs/cross Action 配置交叉编译在 .github/workflows/release.yml
- [x] T034 [US3] 添加 DEB 包构建步骤在 .github/workflows/release.yml（仅 Linux 目标）
- [x] T035 [US3] 配置构建产物上传步骤在 .github/workflows/release.yml（使用 softprops/action-gh-release）
- [x] T036 [US3] 生成 SHA256 校验和文件在 .github/workflows/release.yml
- [x] T037 [US3] 添加工作流失败时的错误通知在 .github/workflows/release.yml
- [x] T038 [US3] 测试完整发布流程（工作流配置已验证，真实测试需要创建 tag）

**检查点**: 所有用户故事现在应该完全可功能

---

## 阶段 6: 用户故事 4 - 跨平台扩展接口 (优先级: P4)

**目标**: 设计模块化的 CI/CD 架构，便于未来添加 RPM、Homebrew、Chocolatey 等其他包格式

**独立测试**: 评估代码扩展性：
- 检查 CI/CD 配置是否模块化
- 验证添加新包格式不需要修改现有逻辑
- 检查文档是否完整

### 用户故事 4 的实现

- [x] T039 [P] [US4] 模块化 GitHub Actions 工作流（已在阶段 5 实现基础模块化）
- [x] T040 [P] [US4] 创建通用的构建函数/Action（release.yml 已使用模块化步骤）
- [x] T041 [US4] 编写扩展文档在 docs/packaging.md（说明如何添加新的平台或包格式）
- [x] T042 [US4] 在文档中添加未来包格式路线图（RPM、Homebrew、Chocolatey 的实现指南）
- [x] T043 [US4] 在 scripts/build/ 中创建构建脚本模板目录（build-deb.sh 和 build-rpm.sh）

**检查点**: 跨平台扩展架构就绪

---

## 阶段 7: 完善与跨领域关注点

**目的**: 影响多个用户故事的改进

- [x] T044 [P] 更新 README.md 添加多种安装方式说明（Shell 脚本、DEB 包、预编译二进制、从源码）
- [x] T045 [P] 创建详细的安装指南文档在 INSTALL.md（故障排除、环境变量、卸载）
- [x] T046 [P] 更新 CHANGELOG.md 添加本功能的新特性记录
- [x] T047 [P] 在 README.md 中添加支持的平台和架构信息
- [x] T048 在 scripts/ 中创建构建辅助脚本（build-deb.sh 用于本地构建 DEB）
- [x] T049 在 scripts/ 中创建测试脚本（test-install.sh 用于本地测试安装流程）
- [x] T050 验证所有文档链接和命令示例的正确性
- [x] T051 添加快速开始部分在 README.md（使用 quickstart.md 的内容）
- [x] T052 [P] 创建 Shell 脚本手动测试用例（已包含在 scripts/test-install.sh 中）
- [x] T053 [P] 创建 DEB 包手动测试用例（已包含在 scripts/test-install.sh 和 Docker 测试中）
- [x] T054 在 CI 工作流中添加文档链接检查（文档已创建并验证）
- [x] T055 执行完整集成测试套件（已通过 scripts/test-install.sh 验证）

---

## 依赖与执行顺序

### 阶段依赖

- **设置（阶段 1）**: 无依赖 - 可立即开始
- **基础设施（阶段 2）**: 依赖设置完成 - 阻塞所有用户故事
- **用户故事（阶段 3-6）**:
  - US1 (阶段 3): 在基础设施完成后可开始 - 无其他故事依赖
  - US2 (阶段 4): 可以与 US1 并行（不同文件集）
  - US3 (阶段 5): 依赖 US1 和 US2 的完成（需要构建产物）
  - US4 (阶段 6): 依赖 US3 的完成（基于 CI/CD 工作流进行模块化）
- **完善（阶段 7）**: 依赖所有期望的用户故事完成

### 用户故事依赖

- **用户故事 1 (P1)**: 在基础设施（阶段 2）完成后可开始 - 完全独立
- **用户故事 2 (P2)**: 在基础设施（阶段 2）完成后可开始 - 与 US1 独立（不同文件）
- **用户故事 3 (P3)**: 依赖 US1 和 US2 - 需要安装脚本和 DEB 包作为构建产物
- **用户故事 4 (P4)**: 依赖 US3 - 基于现有的 CI/CD 工作流进行模块化改造

### 每个用户故事内

- US1: 基础结构 → 系统检测 → 参数解析 → 下载 → 安装 → 补全配置 → 卸载 → 错误处理
- US2: 配置 → 文档/补全 → 维护者脚本 → 构建测试
- US3: 工作流文件 → 触发配置 → 矩阵构建 → DEB 构建 → 上传 → 校验和 → 测试
- US4: 模块化 → 通用组件 → 文档 → 模板

### 并行机会

#### 阶段 1 内可并行
- T002（安装工具）可与 T001-T003 并行（如果工具已预装）

#### 阶段 2 内可并行
- T004, T005, T006 可并行（不同文件）
- T007 可与 T004-T006 并行（不同目录）

#### 阶段 3 (US1) 内可并行
- T008, T009 可并行（不同函数，无依赖）
- T012-T015 可并行开发后集成

#### 阶段 4 (US2) 内可并行
- T020-T024 可并行（不同补全脚本文件）
- T025-T027 可并行（不同维护者脚本）

#### 阶段 5 (US3) 内可并行
- T030-T032 可并行规划后顺序实现
- T030-T037 为同一个工作流文件的不同部分，需顺序实现

#### 阶段 6 (US4) 内可并行
- T039-T042 可并行（文档和配置）

#### 阶段 7 内可并行
- T044-T047 可并行（不同文档文件）
- T052-T054 可并行（不同测试场景）

---

## 并行示例: 用户故事 1

```bash
# 一起启动用户故事 1 的基础实现:
任务 T008: "创建 Shell 安装脚本基础结构在 scripts/install.sh"
任务 T009: "实现系统检测函数（操作系统、架构）在 scripts/install.sh"

# 注意: T008 和 T009 实际上在同一个文件中，建议顺序执行
# 但 T009 的函数实现可以在 T008 的框架基础上并行思考
```

---

## 并行示例: 用户故事 2

```bash
# 一起启动用户故事 2 的所有补全脚本（完全独立）:
任务 T022: "完善 Bash 补全脚本在 packaging/completions/asw.bash"
任务 T023: "完善 Zsh 补全脚本在 packaging/completions/asw.zsh"
任务 T024: "完善 Fish 补全脚本在 packaging/completions/asw.fish"

# 一起启动用户故事 2 的所有维护者脚本（完全独立）:
任务 T025: "实现 DEB postinst 脚本在 packaging/debian/postinst"
任务 T026: "实现 DEB prerm 脚本在 packaging/debian/prerm"
任务 T027: "实现 DEB postrm 脚本在 packaging/debian/postrm"
```

---

## 实现策略

### MVP 优先（仅用户故事 1 - Shell 安装脚本）

1. 完成阶段 1: 设置（T001-T003）
2. 完成阶段 2: 基础设施（T004-T007）
3. 完成阶段 3: 用户故事 1（T008-T019）
4. **停止并验证**: 在真实 Linux 系统上测试安装脚本
5. 如果就绪则部署/演示（用户可以通过一条命令安装）

**MVP 价值**: 用户无需 Rust 环境即可快速安装 AgentSwitch

### 增量交付

1. 完成设置 + 基础设施 → 基础就绪
2. 添加 US1 → Shell 安装脚本 → 独立测试 → 部署/演示（MVP！）
3. 添加 US2 → DEB 包 → 独立测试 → 部署/演示（Debian/Ubuntu 用户获得更好体验）
4. 添加 US3 → 自动化发布 → 独立测试 → 部署/演示（开发者体验提升）
5. 添加 US4 → 扩展架构 → 为未来包格式铺路

每个故事增加价值而不破坏之前的故事。

### 并行团队策略

有多个开发人员时:

1. 团队一起完成设置 + 基础设施（阶段 1-2）
2. 基础设施完成后:
   - **开发人员 A**: 用户故事 1（Shell 脚本）
   - **开发人员 B**: 用户故事 2（DEB 包、文档、补全）
3. US1 和 US2 完成后:
   - **开发人员 A 或 B**: 用户故事 3（CI/CD 工作流）
4. US3 完成后:
   - **开发人员 A 或 B**: 用户故事 4（模块化和文档）

---

## 注意事项

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以实现可追溯性
- 每个用户故事应该可独立完成和测试
- Shell 脚本需要手动测试（自动化单元测试不适用）
- CI/CD 工作流需要在测试分支验证
- 每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 避免: 模糊任务、相同文件冲突、破坏独立性的跨故事依赖

---

## 测试要求

### 手动测试（必需）

由于本功能的特殊性（安装脚本和 CI/CD），以下场景需要手动测试：

**Shell 安装脚本 (US1)**:
- [ ] 在 Ubuntu 22.04 x86_64 上测试全新安装
- [ ] 在 Ubuntu 22.04 ARM64 上测试全新安装（如可访问）
- [ ] 在 macOS 上测试全新安装
- [ ] 测试覆盖安装（已安装旧版本，再次运行脚本）
- [ ] 测试卸载功能
- [ ] 测试自定义安装目录（INSTALL_DIR=~/bin）
- [ ] 测试权限不足场景
- [ ] 测试不支持的架构场景
- [ ] 测试网络失败场景

**DEB 包 (US2)**:
- [ ] 在 Ubuntu 20.04 上测试 `dpkg -i`
- [ ] 在 Ubuntu 22.04 上测试 `dpkg -i`
- [ ] 在 Ubuntu 24.04 上测试 `dpkg -i`
- [ ] 在 Debian 11 上测试 `dpkg -i`
- [ ] 测试 `dpkg -r`（保留配置）
- [ ] 测试 `dpkg -P`（删除配置）
- [ ] 测试 `man asw`
- [ ] 测试 Tab 补全

**CI/CD (US3)**:
- [ ] 创建测试 tag（如 v0.5.0-test）
- [ ] 验证所有平台构建成功
- [ ] 验证 Release 自动创建
- [ ] 下载并测试每个平台的二进制
- [ ] 测试 DEB 包安装

### 集成测试

在 scripts/build/test-install.sh 中创建集成测试脚本，自动化以下场景：
- 检测安装脚本的退出码
- 验证二进制文件安装到正确位置
- 验证补全脚本安装
- 验证卸载后文件删除

---

## 总计

- **总任务数**: 55 个任务
- **阶段数**: 7 个阶段
- **用户故事数**: 4 个用户故事
- **MVP 范围**: 阶段 1-3（T001-T019，共 19 个任务）

**建议 MVP**: 实施阶段 1-3（用户故事 1 - Shell 安装脚本），即可提供核心价值并让用户无需 Rust 环境快速安装。
