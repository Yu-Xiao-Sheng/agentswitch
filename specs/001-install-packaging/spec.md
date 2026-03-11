# 功能规格说明: 便捷安装与分发系统

**功能分支**: `001-install-packaging`
**创建日期**: 2026-03-11
**状态**: 草稿
**输入**: 用户描述: "改进 AgentSwitch 的安装体验，为不同平台提供便捷的安装方式。优先支持 Linux 平台，提供 DEB 包和 Shell 安装脚本，同时为其他平台预留扩展接口。"

**⚠️ 重要提示**: 根据 AgentSwitch 项目宪章，本文档必须使用中文编写。

## 用户场景与测试 *(mandatory)*

### 用户故事 1 - 一键脚本安装 (优先级: P1)

**场景**: 一位 Linux 用户想要快速安装 AgentSwitch，但没有 Rust 开发环境，也不想手动编译源代码。用户期望像安装其他主流 CLI 工具（如 kubectl、helm、gh）一样，通过一条简单的命令就能完成安装。

**用户旅程**:
1. 用户在项目文档中看到安装命令
2. 用户复制并执行 `curl https://.../install.sh | bash` 命令
3. 脚本自动检测用户的系统架构和操作系统
4. 脚本下载对应平台的预编译二进制文件
5. 脚本将二进制安装到系统路径
6. 脚本自动配置 Shell 自动补全功能
7. 用户立即可以使用 `asw` 命令

**为什么是此优先级**: 这是降低安装门槛的最直接方式，能够让最多用户快速上手。一旦实现，用户无需了解 Rust、编译、依赖管理等技术细节，极大提升首次使用体验。

**独立测试**: 可以通过在干净的 Linux 虚拟机中执行安装脚本完全测试。安装完成后，用户能够：
- 直接运行 `asw --version` 查看版本信息
- 使用 `asw model list` 等基础命令
- 通过 `which asw` 找到安装路径
- 在新启动的 Shell 中使用自动补全功能

**验收场景**:

1. **给定** 一台运行 Debian/Ubuntu 的 x86_64 计算机，**当** 用户执行 `curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash`，**则** 系统自动下载并安装 AgentSwitch 到 `/usr/local/bin/asw`，安装成功后用户可以立即执行 `asw --version` 命令

2. **给定** 一台运行 Linux 的 ARM64 计算机（如树莓派），**当** 用户执行安装脚本，**则** 脚本自动检测到 ARM64 架构并下载对应的二进制文件

3. **给定** 一台 macOS 计算机，**当** 用户执行安装脚本，**则** 脚本识别操作系统为 macOS 并下载对应的 Darwin 二进制文件

4. **给定** 用户已安装旧版本的 AgentSwitch，**当** 用户再次执行安装脚本，**则** 脚本提示是否覆盖现有版本，用户确认后更新到最新版本

5. **给定** 用户想要卸载 AgentSwitch，**当** 用户执行 `bash scripts/install.sh --uninstall`，**则** 脚本删除二进制文件并清理相关配置文件

6. **给定** 用户没有 `/usr/local/bin` 的写入权限，**当** 用户执行安装脚本，**则** 脚本提示需要使用 sudo 或提供 `INSTALL_DIR` 环境变量指定安装目录

7. **给定** 用户的操作系统架构不被支持（如 PowerPC），**当** 用户执行安装脚本，**则** 脚本显示友好的错误消息，列出支持的架构并建议从源码编译

---

### 用户故事 2 - DEB 包安装 (优先级: P2)

**场景**: 一位 Debian 或 Ubuntu 用户习惯使用系统的包管理器来安装软件，希望可以通过 `apt install agentswitch` 或 `dpkg -i agentswitch_x.x.x_amd64.deb` 来安装 AgentSwitch。这样用户可以通过包管理器统一管理软件的安装、更新和卸载。

**用户旅程**:
1. 用户从 GitHub Releases 页面下载 `.deb` 文件
2. 用户执行 `sudo dpkg -i agentswitch_x.x.x_amd64.deb` 安装
3. 系统自动处理依赖关系（如果有）
4. 系统将二进制安装到 `/usr/bin/asw`
5. 系统自动注册 man 手册页
6. 系统自动安装 bash 补全脚本
7. 用户可以通过 `apt list --upgradeable` 检查更新
8. 用户可以通过 `apt upgrade agentswitch` 更新软件

**为什么是此优先级**: DEB 包是 Linux 用户，特别是 Debian/Ubuntu 系列发行版用户最熟悉的安装方式。它提供了更好的系统集成、依赖管理和版本控制。虽然优先级略低于一键脚本（因为适用范围更窄），但仍然是提升用户体验的重要方式。

**独立测试**: 可以通过在 Debian 或 Ubuntu 虚拟机中安装 `.deb` 包完全测试。安装完成后：
- `dpkg -L agentswitch` 显示所有安装的文件
- `man asw` 可以查看手册页
- `apt-cache show agentswitch` 显示包信息
- `apt remove agentswitch` 可以完全卸载

**验收场景**:

1. **给定** 一台运行 Ubuntu 22.04 的计算机，**当** 用户从 GitHub Releases 下载 `agentswitch_0.4.0_amd64.deb` 并执行 `sudo dpkg -i agentswitch_0.4.0_amd64.deb`，**则** 系统成功安装 AgentSwitch，`asw` 命令可以在任何路径下执行

2. **给定** DEB 包已安装，**当** 用户执行 `man asw`，**则** 系统显示完整的命令行工具使用手册

3. **给定** DEB 包已安装，**当** 用户在 bash 中输入 `asw ` 并按 Tab 键，**则** 系统显示可用的子命令和参数补全

4. **给定** DEB 包已安装，**当** 用户执行 `dpkg -r agentswitch`，**则** 系统删除所有安装的文件，但保留用户的配置文件（`~/.agentswitch/`）

5. **给定** DEB 包已安装，**当** 用户执行 `dpkg -P agentswitch`（purge），**则** 系统删除所有文件包括配置文件

6. **给定** 用户尝试安装比系统版本更旧的 DEB 包，**则** 安装程序警告版本降级并要求用户确认

---

### 用户故事 3 - 自动化发布流程 (优先级: P3)

**场景**: 作为项目维护者，当发布新版本时，希望系统能够自动构建多平台的二进制文件和安装包，并上传到 GitHub Releases，避免手动操作和人为错误。

**用户旅程**:
1. 开发者在本地完成新版本开发
2. 开发者创建 Git tag（如 `v0.5.0`）
3. 开发者推送 tag 到 GitHub
4. GitHub Actions 自动触发构建流程
5. 构建流程为多个平台编译二进制（Linux x86_64、Linux ARM64、macOS x86_64、macOS ARM64）
6. 构建流程为 Linux 平台生成 DEB 包
7. 构建流程自动创建 GitHub Release
8. 构建流程上传所有构建产物到 Release
9. 开发者和用户可以从 Release 页面下载对应平台的安装包

**为什么是此优先级**: 这是开发者体验的改进，不影响最终用户的功能。虽然重要，但只有在实现了 P1 和 P2 之后才有意义（因为需要有可以发布的产物）。自动化发布可以减少维护成本，确保每次发布都包含所有平台的包。

**独立测试**: 可以通过在测试分支创建 tag 来验证。预期结果：
- GitHub Actions 工作流自动触发
- 构建日志显示所有平台成功编译
- GitHub Release 自动创建并包含所有构建产物
- 每个平台的二进制文件可以独立运行
- DEB 包可以在对应系统上安装

**验收场景**:

1. **给定** 代码仓库中创建了新的 tag `v0.5.0`，**当** tag 被推送到 GitHub，**则** GitHub Actions 自动触发 release 工作流

2. **给定** Release 工作流正在运行，**当** 构建完成，**则** GitHub Release 页面包含以下文件：
   - `agentswitch-x86_64-unknown-linux-gnu.tar.gz` (Linux x86_64)
   - `agentswitch-aarch64-unknown-linux-gnu.tar.gz` (Linux ARM64)
   - `agentswitch-x86_64-apple-darwin.tar.gz` (macOS Intel)
   - `agentswitch-aarch64-apple-darwin.tar.gz` (macOS Apple Silicon)
   - `agentswitch_0.5.0_amd64.deb` (DEB 包)
   - `install.sh` (安装脚本)

3. **给定** Release 已创建，**当** 用户下载并解压 Linux x86_64 的 tar.gz 文件，**则** 得到的二进制文件可以在 Linux x86_64 系统上直接运行

4. **给定** 构建过程中任何一个平台失败，**则** 整个 Release 失败，维护者收到通知

---

### 用户故事 4 - 跨平台扩展接口 (优先级: P4)

**场景**: 作为项目维护者，希望当前的打包和分发系统能够方便地扩展到其他平台和包格式（如 macOS Homebrew、Windows Chocolatey、RPM 包等），而不需要重构整个系统。

**用户旅程**:
1. 未来需要为 macOS 添加 Homebrew 支持
2. 开发者基于现有的打包接口实现 Homebrew formula
3. 开发者添加新的构建步骤到 CI/CD 流程
4. 系统自动生成 Homebrew 安装文件
5. 用户可以通过 `brew install agentswitch` 安装

**为什么是此优先级**: 这是面向未来的架构设计，不是当前 MVP（最小可行产品）的必要部分。在完成 P1-P3 之后，再考虑其他平台的扩展。良好的架构设计可以让未来的扩展更加简单。

**独立测试**: 可以通过评估现有代码的扩展性来测试。检查点：
- 是否存在抽象的打包接口
- 添加新的包格式是否不需要修改核心逻辑
- CI/CD 配置是否模块化、易于添加新步骤

**验收场景**:

1. **给定** 需要添加 RPM 包支持，**当** 开发者查看打包系统架构，**则** 发现存在清晰的扩展点，只需要实现 RPM 特定的打包逻辑

2. **给定** 需要添加 Homebrew 支持，**当** 开发者实现 Homebrew formula，**则** 不需要修改现有的 DEB 包构建逻辑

3. **给定** 新增一种包格式，**当** 在 CI/CD 中添加对应的构建步骤，**则** 不会影响现有包的构建流程

---

### 边界情况

- 当用户下载二进制文件时网络中断或服务器无响应时会发生什么？
  - 安装脚本应检测下载失败，重试 3 次，如果仍失败则显示错误消息和手动下载链接

- 当用户没有 `/usr/local/bin` 或 `/usr/bin` 的写入权限时会发生什么？
  - 安装脚本应提示使用 sudo 权限或提供 `INSTALL_DIR` 环境变量指定用户目录（如 `~/bin`）

- 当用户的操作系统架构不被支持（如 RISC-V、PowerPC、s390x）时会发生什么？
  - 安装脚本应显示友好的错误消息，列出支持的架构（x86_64、ARM64），并提供从源码编译的指南链接

- 当用户的操作系统版本过旧（如 Ubuntu 16.04）时会发生什么？
  - DEB 包应明确最低系统版本要求，安装时检查版本，如果不符合则拒绝安装并显示原因

- 当 DEB 包安装过程中依赖关系无法满足时会发生什么？
  - `dpkg` 应显示依赖错误并退出，提示用户运行 `apt-get install -f` 修复

- 当用户覆盖安装时现有配置文件如何处理？
  - 安装脚本和 DEB 包应保留用户的配置文件（`~/.agentswitch/`），只更新二进制文件

- 当安装脚本被中断（Ctrl+C）时会发生什么？
  - 应清理已下载的临时文件，不留下不完整的安装

- 当 GitHub API 速率限制时会发生什么？
  - 安装脚本应使用备用下载 URL（如直接 CDN 链接），或提示用户稍后重试

- 当 DEB 包的文件损坏时会发生什么？
  - `dpkg` 应检测到校验和不匹配并拒绝安装，显示校验和错误

- 当用户在同一台机器上多次安装时会发生什么？
  - 安装脚本应检测到已安装的版本，提示用户是否覆盖

## 需求 *(mandatory)*

### 功能性需求

#### 安装脚本相关

- **FR-001**: 系统 MUST 提供一个可通过 `curl` 或 `wget` 直接执行的 Shell 安装脚本
- **FR-002**: 安装脚本 MUST 自动检测操作系统（Linux/macOS）和系统架构（x86_64/arm64/aarch64）
- **FR-003**: 安装脚本 MUST 从 GitHub Releases 下载对应平台的预编译二进制文件
- **FR-004**: 安装脚本 MUST 将二进制文件安装到系统路径（默认 `/usr/local/bin`，可通过环境变量 `INSTALL_DIR` 自定义）
- **FR-005**: 安装脚本 MUST 自动配置 Shell 自动补全（bash/zsh/fish）
- **FR-006**: 安装脚本 MUST 支持 `--uninstall` 选项以完全卸载 AgentSwitch
- **FR-007**: 安装脚本 MUST 在安装前检测已安装的版本并提示用户确认覆盖
- **FR-008**: 安装脚本 MUST 在下载失败时自动重试最多 3 次
- **FR-009**: 安装脚本 MUST 显示清晰的安装进度和错误消息
- **FR-010**: 安装脚本 MUST 验证下载的二进制文件的校验和（如果提供）

#### DEB 包相关

- **FR-011**: 系统 MUST 为每个 Release 自动生成 `.deb` 安装包
- **FR-012**: DEB 包 MUST 包含可执行的二进制文件（安装到 `/usr/bin/asw`）
- **FR-013**: DEB 包 MUST 包含 man 手册页（安装到 `/usr/share/man/man1/asw.1`）
- **FR-014**: DEB 包 MUST 包含 bash 自动补全脚本（安装到 `/usr/share/bash-completion/completions/asw`）
- **FR-015**: DEB 包 MUST 正确设置依赖关系（如果有）
- **FR-016**: DEB 包 MUST 设置正确的文件权限（二进制文件 0755，文档文件 0644）
- **FR-017**: DEB 包 MUST 指定最低系统版本要求（如 Ubuntu 20.04+）
- **FR-018**: DEB 包 MUST 在卸载时（`dpkg -r`）保留用户配置文件（`~/.agentswitch/`）
- **FR-019**: DEB 包 MUST 在完全卸载时（`dpkg -P`）删除配置文件
- **FR-020**: DEB 包 MUST 包含包描述、版本号、维护者信息等元数据

#### CI/CD 自动化相关

- **FR-021**: 系统 MUST 在创建 Git tag 并推送到 GitHub 时自动触发构建流程
- **FR-022**: 构建流程 MUST 为 Linux x86_64、Linux ARM64、macOS Intel、macOS Apple Silicon 编译二进制文件
- **FR-023**: 构建流程 MUST 为 Linux 平台生成 DEB 包
- **FR-024**: 构建流程 MUST 自动创建 GitHub Release
- **FR-025**: 构建流程 MUST 将所有构建产物（二进制、DEB 包、安装脚本）上传到 GitHub Release
- **FR-026**: 构建流程 MUST 在任何平台构建失败时标记整个 Release 为失败
- **FR-027**: 构建流程 MUST 为每个构建产物生成校验和文件（如 SHA256SUMS）

#### 扩展性相关

- **FR-028**: 打包系统 MUST 提供统一的接口以支持未来添加其他包格式（RPM、Homebrew、Chocolatey）
- **FR-029**: CI/CD 配置 MUST 模块化，允许独立添加新的平台构建步骤
- **FR-030**: 系统 MUST 提供清晰的文档说明如何添加新的平台或包格式支持

#### 安全性相关

- **FR-031**: 安装脚本 MUST 使用 HTTPS 下载二进制文件
- **FR-032**: 安装脚本 MUST 验证下载的文件完整性（如果提供校验和）
- **FR-033**: DEB 包 MUST 在元数据中声明源代码 URL
- **FR-034**: 所有下载的二进制文件 MUST 在发布前进行安全扫描

### 关键实体 *(如果功能涉及数据则包含)*

- **安装包**: 包含二进制文件和相关资源的分发包，有不同的格式（DEB、tar.gz、未来的 RPM 等）
- **构建产物**: 由编译流程生成的文件，包括各平台的可执行文件
- **发布版本**: 由 Git tag 标识的软件版本，包含完整的构建产物集合
- **安装脚本**: 自动化安装过程的 Shell 脚本，负责检测环境、下载文件、配置系统
- **CI/CD 工作流**: 定义构建、打包、发布流程的自动化脚本

## 成功标准 *(mandatory)*

### 可衡量的结果

- **SC-001**: 新用户可以在 2 分钟内完成 AgentSwitch 安装（从开始执行安装命令到可以运行 `asw --help`）
- **SC-002**: 安装脚本在 99% 的情况下成功完成安装（排除用户主动取消、网络完全不可达等极端情况）
- **SC-003**: 用户在安装过程中遇到错误时，有 95% 的情况可以通过错误消息自行解决问题或找到解决方向
- **SC-004**: DEB 包可以在所有支持的 Debian/Ubuntu 版本（Ubuntu 20.04+）上成功安装
- **SC-005**: 从创建 Git tag 到 GitHub Release 完成发布的时间不超过 15 分钟
- **SC-006**: 添加新的包格式支持（如从 DEB 到 RPM）所需的代码改动不超过 200 行
- **SC-007**: 用户反馈显示安装体验满意度提升 50%（相比从源码编译的方式）
- **SC-008**: 与安装相关的支持工单（如"如何安装"、"安装失败"）减少 70%

## 范围与假设 *(mandatory)*

### 功能范围

**包含**:
- Linux 平台的一键脚本安装（支持 x86_64 和 ARM64）
- macOS 平台的一键脚本安装（支持 Intel 和 Apple Silicon）
- Linux 平台的 DEB 包构建和分发（Debian 11+, Ubuntu 20.04+）
- GitHub Actions 自动化构建和发布流程
- Shell 自动补全的自动配置
- 卸载功能

**不包含**:
- RPM 包支持（Red Hat、CentOS、Fedora）
- Homebrew 支持（macOS）
- Chocolatey 支持（Windows）
- APT 仓库的搭建和托管
- 自动更新检测和提示（需要时手动运行安装脚本更新）
- GUI 安装程序
- 签名验证（GPG/代码签名）
- 安装前的依赖检查和自动安装（如 curl、wget 等基础工具假设用户已具备）

### 依赖与假设

**外部依赖**:
- GitHub Actions 提供 CI/CD 能力
- GitHub Releases 承载构建产物
- 用户有基本的 Unix/Linux 终端使用能力
- 用户的系统已安装 `curl` 或 `wget`（用于下载安装脚本）
- Debian/Ubuntu 系统已安装 `dpkg` 包管理器

**假设**:
- 用户有互联网连接以下载安装包
- 用户有 sudo 权限或愿意安装到用户目录
- 用户使用的是 64 位操作系统（x86_64 或 ARM64）
- 用户使用的 Ubuntu 版本不低于 20.04（发布于 2020 年）
- 用户使用的 Debian 版本不低于 11（发布于 2021 年）
- macOS 用户使用的是 macOS 11+ (Big Sur)
- 用户理解通过管道执行脚本的潜在风险（在文档中提供安全说明）

**技术约束**:
- DEB 包不需要复杂的依赖关系（AgentSwitch 是静态链接的二进制）
- 安装脚本需要是 POSIX 兼容的 Shell 脚本（兼容 bash、sh、zsh）
- 构建过程必须使用 GitHub Actions（项目已有的 CI/CD 基础设施）
- Release 版本号遵循语义化版本规范（Semantic Versioning）

## 安全考虑

**安装脚本安全**:
- 安装脚本必须使用 HTTPS 下载所有文件
- 文档中应提示用户可以先下载脚本查看内容再执行
- 提供脚本文件的 SHA256 校验和
- 安装脚本不应要求用户输入密码（除了 sudo）
- 安装脚本不应在执行过程中修改系统关键配置

**DEB 包安全**:
- DEB 包应在构建时记录所有文件的校验和
- DEB 包不应包含任何未声明的依赖
- DEB 包不应在安装时执行未经声明的脚本

**发布安全**:
- GitHub Releases 应标记为预发布（pre-release）直到经过测试
- 构建产物的完整性应通过校验和验证
- 发布流程应有审核机制（至少需要维护者手动批准）

## 文档需求

作为本功能的一部分，需要更新或创建以下文档：

**必需的文档**:
- `README.md` - 更新安装部分，添加一键安装和 DEB 包安装说明
- `INSTALL.md` - 详细的安装指南，包括各种安装方法和故障排除
- `scripts/install.sh` - 安装脚本本身应包含内联注释
- `.github/workflows/release.yml` - CI/CD 流程的文档

**可选的文档**:
- `docs/packaging.md` - 打包系统架构文档，说明如何添加新的平台支持
- `DEVELOPMENT.md` - 开发者指南，说明如何手动构建和测试包
