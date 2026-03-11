# Implementation Plan: 便捷安装与分发系统

**Branch**: `001-install-packaging` | **Date**: 2026-03-11 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-install-packaging/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

本功能为 AgentSwitch 添加便捷的安装和分发系统，优先支持 Linux 平台（Shell 脚本 + DEB 包），同时为其他平台预留扩展接口。

**核心目标**:
1. **一键安装脚本** (P1): 用户可通过 `curl ... | bash` 在 2 分钟内完成安装
2. **DEB 包分发** (P2): 为 Debian/Ubuntu 用户提供系统集成安装方式
3. **自动化发布** (P3): 通过 GitHub Actions 自动构建多平台二进制和安装包
4. **可扩展架构** (P4): 模块化设计，便于添加 RPM、Homebrew、Chocolatey 等其他包格式

**技术方案**:
- Shell 安装脚本使用标准 `curl --proto '=https' --tlsv1.2 -sSf` 模式（参考 rustup）
- DEB 包使用 `cargo-deb` 工具自动生成
- GitHub Actions 使用 `cross-rs/cross` 进行交叉编译
- 模块化的 CI/CD 配置便于扩展

## Technical Context

**Language/Version**: Rust 最新稳定版 (edition = "2024")
**Primary Dependencies**:
- 现有: clap 4.x, serde, serde_json, toml, anyhow, dirs, colored
- 新增: cargo-deb ^1.44 (仅构建时依赖)

**Storage**: 配置文件 (~/.agentswitch/config.toml) + 系统密钥链
**Testing**: cargo test (单元 + 集成测试) + 手动安装测试
**Target Platform**:
- Linux: x86_64 (amd64), ARM64 (aarch64) - Ubuntu 20.04+, Debian 11+
- macOS: x86_64 (Intel), ARM64 (Apple Silicon) - macOS 11+

**Project Type**: cli (命令行工具)
**Performance Goals**: 配置切换 < 100ms, 启动时间 < 50ms
**Constraints**:
- 安装脚本需要网络连接下载二进制
- CI/CD 需要 GitHub Actions
- DEB 包构建需要 Ubuntu 环境

**Scale/Scope**: 单用户工具，支持 10+ 个 Code Agent 工具
**Build System**: cargo + cargo-deb + GitHub Actions
**Package Formats**:
- Phase 1: Shell 脚本 (Linux/macOS), DEB 包 (Linux)
- Future: RPM, Homebrew, Chocolatey

## 宪章合规性检查 (Constitution Check)

*GATE: 在 Phase 0 研究前必须通过。Phase 1 设计后再次检查。*

### 强制要求（来自 .specify/memory/constitution.md）

- [x] **Rust 2024 标准**: 代码使用 edition = "2024"，通过 `cargo clippy` 无警告
  - ✅ 现有项目已遵循 Rust 2024 标准
  - ✅ 新增代码将继续使用 edition = "2024"
  - ✅ Shell 脚本使用 POSIX 兼容语法

- [x] **中文文档**: plan.md, spec.md, tasks.md 全部使用中文编写
  - ✅ spec.md 已使用中文编写
  - ✅ 本文档 (plan.md) 使用中文编写
  - ✅ tasks.md 将使用中文编写
  - ✅ Shell 脚本注释使用中文
  - ✅ CI/CD 配置注释使用中文

- [x] **测试驱动**: 单元测试覆盖率 ≥ 80%
  - ✅ 现有代码测试覆盖率达标
  - ⚠️ Shell 脚本需要手动测试（自动化测试单元测试不适用）
  - ✅ 将添加集成测试验证安装流程

- [x] **集成测试**: 每个功能完成后执行完整集成测试
  - ✅ 需要在实际 Linux/macOS 环境中测试安装脚本
  - ✅ 需要在 Docker 容器中测试 DEB 包安装
  - ✅ 需要测试完整的安装 → 使用 → 卸载流程

- [x] **安全优先**: API Key 加密存储，配置文件权限 600
  - ✅ 安装脚本使用 `--proto '=https' --tlsv1.2` 强制 HTTPS
  - ✅ 安装脚本不要求用户输入密码（除了 sudo）
  - ✅ DEB 包构建时记录文件校验和
  - ✅ 不在安装脚本中修改系统关键配置

- [x] **向后兼容**: 配置变更提供迁移脚本
  - ✅ 本功能不涉及配置文件格式变更
  - ✅ 安装过程保留现有用户配置（`~/.agentswitch/`）
  - ✅ 卸载时提供选项是否删除配置文件

**总体评估**: ✅ 通过 - 所有宪章要求均已满足或已规划

## Project Structure

### Documentation (this feature)

```text
specs/001-install-packaging/
├── plan.md              # 本文件 (/speckit.plan 命令输出)
├── research.md          # Phase 0 输出 - 技术研究报告
├── data-model.md        # Phase 1 输出 - 数据模型
├── quickstart.md        # Phase 1 输出 - 快速开始指南
├── contracts/           # Phase 1 输出 - 接口契约
│   ├── install-script.md # Shell 脚本接口规范
│   └── deb-package.md    # DEB 包规范
└── tasks.md             # Phase 2 输出 - 任务列表 (/speckit.tasks 命令)
```

### Source Code (repository root)

```text
# Option 1: 单项目结构 (DEFAULT - Rust CLI)
agentswitch/
├── src/
│   ├── main.rs              # 程序入口
│   ├── cli/                 # CLI 命令定义
│   │   ├── mod.rs
│   │   └── commands.rs      # 命令实现
│   ├── config/              # 配置管理
│   │   ├── mod.rs
│   │   ├── store.rs         # 配置存储
│   │   └── models.rs        # 数据模型
│   └── agents/              # Agent 适配器
│       ├── mod.rs
│       ├── adapter.rs       # 适配器 trait
│       └── *.rs             # 各工具适配器
├── scripts/                 # 新增：安装和构建脚本
│   ├── install.sh           # Shell 安装脚本 (P1)
│   └── build/               # 构建辅助脚本
│       ├── build-deb.sh     # DEB 构建脚本
│       └── test-install.sh  # 安装测试脚本
├── packaging/               # 新增：打包配置
│   ├── debian/              # DEB 包配置（cargo-deb 使用）
│   │   └── postinst         # 安装后脚本（如需要）
│   ├── man/                 # man 手册页
│   │   └── asw.1            # 主手册页
│   └── completions/         # Shell 补全脚本
│       ├── asw.bash         # Bash 补全
│       ├── asw.zsh          # Zsh 补全
│       └── asw.fish         # Fish 补全
├── .github/
│   └── workflows/           # GitHub Actions 工作流
│       ├── ci.yml           # 现有：CI 测试
│       └── release.yml      # 新增：自动发布 (P3)
├── Cargo.toml               # 更新：添加 cargo-deb 配置
├── Cargo.lock               # 依赖锁定文件
├── README.md                # 更新：添加安装说明
├── INSTALL.md               # 新增：详细安装指南
└── LICENSE                  # MIT 许可证

tests/
├── integration/             # 集成测试
│   ├── install_test.rs      # 新增：安装流程测试
│   └── packaging_test.rs    # 新增：打包测试
├── unit/                    # 单元测试
└── fixtures/                # 测试数据
```

**Structure Decision**: 选择 Option 1（单项目结构），因为：
1. 现有项目已经是单项目 CLI 工具
2. 无需复杂的架构调整
3. 新增文件主要集中在 `scripts/` 和 `packaging/` 目录
4. CI/CD 配置在 `.github/workflows/` 中，符合 GitHub 标准

## 复杂度跟踪 (Complexity Tracking)

> **本项目无需填写此部分**
>
> 所有功能实现符合项目宪章要求，没有额外的复杂度增加。

## Phase 0: 研究完成 ✅

**输出**: [research.md](./research.md)

### 关键技术决策

1. **Shell 安装脚本**: 使用标准 `curl --proto '=https' --tlsv1.2 -sSf` 模式
   - 参考 rustup、kubectl 等主流工具
   - 强制 HTTPS/TLS 1.2 确保安全
   - 支持自定义安装目录和卸载

2. **DEB 包生成**: 使用 `cargo-deb` 工具
   - Rust 生态系统标准
   - 从 Cargo.toml 自动生成包元数据
   - 支持配置文件、man 手册、补全脚本

3. **交叉编译**: 使用 `cross-rs/cross` GitHub Action
   - 零配置交叉编译
   - 支持矩阵并行构建
   - 兼容 Linux、macOS 多个目标

4. **CI/CD 架构**: 模块化工作流设计
   - 每个包格式独立工作流
   - 主工作流触发所有构建
   - 便于未来扩展（RPM、Homebrew 等）

### 参考资源

- [cargo-deb GitHub](https://github.com/kornelski/cargo-deb)
- [cross-rs GitHub](https://github.com/cross-rs/cross)
- [Rust Book Installation](https://github.com/rust-lang/book/blob/master/src/ch01-01-installation.md)
- [Building Cross-Platform Rust CI/CD](https://ahmedjama.com/blog/2025/12/cross-platform-rust-pipeline-github-actions/)

## Phase 1: 设计完成 ✅

**输出**:
- [data-model.md](./data-model.md) - 本功能无数据模型（纯安装/打包）
- [contracts/install-script.md](./contracts/install-script.md) - Shell 脚本接口规范
- [contracts/deb-package.md](./contracts/deb-package.md) - DEB 包规范
- [quickstart.md](./quickstart.md) - 安装和使用快速开始指南

### 设计决策

1. **无数据模型**: 本功能是纯安装和打包系统，不涉及新的数据实体

2. **接口契约**:
   - Shell 脚本命令行接口（安装、卸载、帮助）
   - DEB 包文件清单和安装路径规范
   - 环境变量定义（INSTALL_DIR、FORCE 等）

3. **用户体验设计**:
   - 提供多种安装方式（Shell 脚本、DEB 包、手动二进制）
   - 清晰的错误消息和进度提示
   - 完整的文档和故障排除指南

## Phase 2: 任务分解

**输出**: [tasks.md](./tasks.md) - 由 `/speckit.tasks` 命令生成

**注意**: 本文档不包含 tasks.md 的内容。tasks.md 需要运行单独的 `/speckit.tasks` 命令来生成。

## 实施优先级

根据 spec.md 中的用户故事优先级：

1. **P1 - 一键脚本安装** (MVP 核心)
   - 实现 Shell 安装脚本
   - 实现卸载功能
   - 添加系统检测和错误处理

2. **P2 - DEB 包安装** (重要功能)
   - 配置 cargo-deb
   - 创建 man 手册页
   - 配置 bash 补全
   - 生成和测试 DEB 包

3. **P3 - 自动化发布** (开发者体验)
   - 创建 GitHub Actions 工作流
   - 配置矩阵构建
   - 设置自动上传到 Release

4. **P4 - 跨平台扩展** (架构设计)
   - 模块化 CI/CD 配置
   - 编写扩展文档
   - 为未来包格式预留接口

## 下一步

1. ✅ **Phase 0 完成**: 技术研究已完成，所有关键决策已确定
2. ✅ **Phase 1 完成**: 设计文档已创建（data-model.md, contracts/, quickstart.md）
3. ⏭️ **Phase 2 待执行**: 运行 `/speckit.tasks` 生成任务列表
4. ⏭️ **实施**: 运行 `/speckit.implement` 执行任务

## 附录

### 环境变量

| 变量名 | 用途 | 默认值 |
|--------|------|--------|
| `INSTALL_DIR` | 自定义安装目录 | `/usr/local/bin` |
| `FORCE` | 强制覆盖安装 | `false` |
| `NO_MODIFY_PATH` | 不修改 PATH 配置 | `false` |

### 支持的平台

| 平台 | 架构 | 状态 |
|------|------|------|
| Linux | x86_64 (amd64) | ✅ P1 |
| Linux | ARM64 (aarch64) | ✅ P1 |
| macOS | x86_64 (Intel) | ✅ P1 |
| macOS | ARM64 (Apple Silicon) | ✅ P1 |
| Windows | x86_64 | ⏸️ Future (P4) |

### 包格式路线图

| 格式 | 状态 | 优先级 |
|------|------|--------|
| Shell 脚本 | 🚧 开发中 | P1 |
| DEB 包 | 🚧 开发中 | P2 |
| RPM 包 | 📅 已规划 | P4 |
| Homebrew | 📅 已规划 | P4 |
| Chocolatey | 📅 已规划 | P4 |
| APT 仓库 | 📅 已规划 | P4 |
