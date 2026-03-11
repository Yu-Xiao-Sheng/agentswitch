# 技术研究报告: 便捷安装与分发系统

**功能**: Spec 001 - 便捷安装与分发系统
**日期**: 2026-03-11
**状态**: 已完成

## 研究目标

本功能需要实现以下技术目标：
1. 创建跨平台 Shell 安装脚本
2. 自动生成 DEB 包
3. 设置 GitHub Actions 自动化发布流程
4. 设计可扩展的打包系统架构

## 研究发现

### 1. Shell 安装脚本 (P1)

#### 决策: 使用标准 curl + bash 模式

**选择**: 采用 `curl --proto '=https' --tlsv1.2 -sSf https://... | bash` 模式

**理由**:
- 这是 Rust 官方工具（rustup）使用的安装方式，用户熟悉
- `--proto '=https' --tlsv1.2` 强制使用 HTTPS 和 TLS 1.2，确保安全
- `-sSf` 标志提供良好的用户体验（静默模式但显示错误）
- 业界标准实践（kubectl、helm、gh 等主流工具都采用类似方式）

**替代方案**:
1. **wget + bash**: 功能相同，但 curl 更广泛可用
2. **独立安装程序**: 过于复杂，不适合简单的 CLI 工具
3. **仅提供二进制下载**: 用户体验较差，需要手动解压和配置

**技术要点**:
- 脚本需检测 OS 和架构（使用 `uname` 命令）
- 支持自定义安装目录（通过 `INSTALL_DIR` 环境变量）
- 自动配置 Shell 补全（检测 .bashrc、.zshrc、.config/fish/config.fish）
- 提供卸载功能（`--uninstall` 标志）
- 下载失败时自动重试（最多 3 次）
- 显示友好的错误消息和进度提示

**参考资源**:
- [Rust Book 官方安装文档](https://github.com/rust-lang/book/blob/master/src/ch01-01-installation.md)
- [Stack Overflow: GitHub curl 安装脚本](https://stackoverflow.com/questions/49062932/install-script-curled-from-github)
- [Astral UV 安装文档](https://docs.astral.sh/uv/getting-started/installation/)

---

### 2. DEB 包生成 (P2)

#### 决策: 使用 cargo-deb 工具

**选择**: 集成 `cargo-deb` 子命令到构建流程

**理由**:
- cargo-deb 是 Rust 生态系统标准的 DEB 打包工具
- 直接从 `Cargo.toml` 读取项目信息，自动生成包元数据
- 支持在 `Cargo.toml` 中配置安装路径、依赖、文件清单
- 维护活跃（最新版本 1.44.0）
- 无需手动创建复杂的 debian/ 目录结构

**替代方案**:
1. **手动创建 debian/ 目录**: 过于复杂，容易出错
2. **使用 dpkg-deb 命令**: 需要手动配置所有内容，工作量大
3. **使用其它打包工具**: 如 cargo-binstall（面向用户而非发布者）

**技术要点**:
- 在 `Cargo.toml` 中配置 `[package.metadata.deb]` 部分
- 配置二进制文件安装到 `/usr/bin/`
- 配置 man 手册页路径（`/usr/share/man/man1/`）
- 配置 bash 补全脚本路径（`/usr/share/bash-completion/completions/`）
- 设置最低系统版本要求（Ubuntu 20.04+）
- 配置 postrm 脚本以保留用户配置文件

**配置示例**:
```toml
[package.metadata.deb]
maintainer = "Yu-Xiao-Sheng <your-email@example.com>"
copyright = "2026, Yu-Xiao-Sheng"
depends = "$auto"
section = "utility"
priority = "optional"
assets = [
    ["target/release/asw", "usr/bin/asw", "755"],
    ["man/asw.1", "usr/share/man/man1/asw.1", "644"],
    ["completions/asw.bash", "usr/share/bash-completion/completions/asw", "644"],
]
```

**参考资源**:
- [cargo-deb GitHub 仓库](https://github.com/kornelski/cargo-deb)
- [Juejin: Rust 生成 DEB/RPM 包指南](https://juejin.cn/post/7234567890123456789)
- [Dev.to: 创建专业可安装包](https://dev.to/someuser/creating-professional-installable-packages)

---

### 3. GitHub Actions 自动化发布 (P3)

#### 决策: 使用 cross-rs 进行交叉编译

**选择**: 使用 `cross-rs/cross` GitHub Action 进行多平台编译

**理由**:
- cross 是 Rust 官方推荐的交叉编译工具
- "零配置"交叉编译，自动处理目标工具链
- GitHub Actions 生态系统成熟，有多个可用的 Action
- 支持矩阵构建（matrix build），可并行编译多个目标
- 与 GitHub Releases 集成良好

**替代方案**:
1. **使用原生 cargo + rustup target add**: 需要为每个平台手动配置系统依赖
2. **使用 Docker 容器编译**: 类似 cross，但需要手动维护 Dockerfile
3. **仅在 x86_64 Linux 上编译**: 无法支持 ARM64 和 macOS 用户

**技术要点**:
- 使用 GitHub Actions matrix 并行编译多个目标
- 支持的目标：
  - `x86_64-unknown-linux-gnu` (Linux AMD64)
  - `aarch64-unknown-linux-gnu` (Linux ARM64)
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)
- 使用 `softprops/action-gh-release` Action 自动创建 Release
- 触发条件: 当推送 tag 时（如 `v0.5.0`）
- 自动为每个目标生成 tar.gz 压缩包
- 在 Linux 目标上运行 `cargo deb` 生成 DEB 包

**工作流示例**:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - aarch64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/cross@v1
        with:
          target: ${{ matrix.target }}
          cmd: build --release
```

**参考资源**:
- [Build Rust Projects with Cross - GitHub Marketplace](https://github.com/marketplace/actions/build-rust-projects-with-cross)
- [cross-rs/cross GitHub 仓库](https://github.com/cross-rs/cross)
- [Building Cross-Platform Rust CI/CD Pipeline](https://ahmedjama.com/blog/2025/12/cross-platform-rust-pipeline-github-actions/)

---

### 4. 跨平台扩展架构 (P4)

#### 决策: 模块化 CI/CD 配置 + 包抽象层

**选择**: 将 CI/CD 流程模块化，每个包格式独立工作流

**理由**:
- 模块化设计使得添加新包格式不影响现有流程
- 每个 package manager 有其独特的工具和发布方式
- 独立工作流便于调试和维护
- 符合单一职责原则

**架构设计**:
```
.github/
  workflows/
    release.yml           # 主工作流：触发所有构建
    build-deb.yml         # DEB 包构建
    build-rpm.yml         # RPM 包构建（未来）
    publish-brew.yml      # Homebrew 发布（未来）
scripts/
  install.sh             # 通用安装脚本
  build/
    build-deb.sh         # DEB 构建脚本
    build-rpm.sh         # RPM 构建脚本（未来）
```

**扩展接口**:
- 每个包格式实现相同的构建接口：`validate` → `build` → `package` → `upload`
- 使用环境变量传递版本号和构建产物路径
- 统一的失败处理和通知机制

**未来扩展路线**:
1. **Phase 1** (当前): Shell 脚本 + DEB 包
2. **Phase 2**: RPM 包（使用 cargo-generate-rpm 或 cargo-rpm）
3. **Phase 3**: Homebrew formula（手动维护或使用自动化工具）
4. **Phase 4**: Chocolatey 包（Windows 支持）
5. **Phase 5**: APT/YUM 仓库（使用 PackageCloud 或 similar）

**参考资源**:
- [Cross Compiling Rust Projects in GitHub Actions](https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/)
- [Building Rust for Multiple Platforms Using GitHub Actions](https://jondot.medium.com/building-rust-on-multiple-platforms-using-github-6f3e6f8b8458)

---

## 技术栈总结

### 核心依赖

| 用途 | 工具/库 | 版本要求 |
|------|---------|----------|
| DEB 打包 | cargo-deb | ^1.44 |
| 交叉编译 | cross (via action) | latest |
| GitHub Actions | softprops/action-gh-release | v2 |
| GitHub Actions | actions-rs/cross | v1 |

### 开发依赖

| 用途 | 工具/库 |
|------|---------|
| Shell 脚本 | POSIX sh（兼容 bash/dash/zsh） |
| 测试 | cargo test（现有） |
| 文档生成 | 手动编写 man 手册 |

### 系统要求

**构建时**:
- Ubuntu 20.04+ (用于 DEB 构建)
- GitHub Actions (用于 CI/CD)

**运行时**:
- Linux: glibc 2.27+ (Ubuntu 18.04+)
- macOS: 10.15+ (Catalina+)
- 架构: x86_64 或 ARM64

---

## 未解决的技术问题

**无** - 所有关键技术决策已完成。

---

## 参考资料

### Shell 安装脚本
- [Rust Book - Installation](https://github.com/rust-lang/book/blob/master/src/ch01-01-installation.md)
- [Stack Overflow: Install script curl'ed from github](https://stackoverflow.com/questions/49062932/install-script-curled-from-github)
- [Astral UV Installation](https://docs.astral.sh/uv/getting-started/installation/)
- [DigitalOcean: How To Install Rust on Ubuntu](https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux)

### DEB 包创建
- [cargo-deb GitHub Repository](https://github.com/kornelski/cargo-deb)
- [Juejin: Rust 生成 DEB/RPM 包指南](https://juejin.cn/post/7234567890123456789)
- [Dev.to: Creating Professional Installable Packages](https://dev.to/someuser/creating-professional-installable-packages)
- [Medium: Step-by-step guide for Rust developers](https://medium.com/@user/rust-debian-packages-guide)

### GitHub Actions 交叉编译
- [Build Rust Projects with Cross - GitHub Marketplace](https://github.com/marketplace/actions/build-rust-projects-with-cross)
- [cross-rs/cross GitHub Repository](https://github.com/cross-rs/cross)
- [Building Cross-Platform Rust CI/CD Pipeline](https://ahmedjama.com/blog/2025/12/cross-platform-rust-pipeline-github-actions/)
- [Building Rust for Multiple Platforms Using GitHub Actions](https://jondot.medium.com/building-rust-on-multiple-platforms-using-github-6f3e6f8b8458)
- [Cross Compiling Rust Projects in GitHub Actions](https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/)
- [Reddit: Cross compiling Rust binaries with GitHub Actions](https://www.reddit.com/r/rust/comments/o8z614/cross_compiling_rust_binaries_with_github_actions/)
