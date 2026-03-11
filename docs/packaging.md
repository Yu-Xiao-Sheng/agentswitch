# AgentSwitch 打包系统架构文档

**版本**: 1.0.0
**日期**: 2026-03-11
**状态**: v0.4.0

## 概述

AgentSwitch 使用模块化的打包系统架构，支持为不同平台和包管理器生成安装包。本文档说明如何添加新的平台或包格式支持。

## 架构设计

### 核心组件

```
agentswitch/
├── scripts/                    # 安装和构建脚本
│   ├── install.sh            # 通用安装脚本（Shell 脚本）
│   └── build/                # 构建脚本目录（未来扩展）
│       ├── build-deb.sh      # DEB 构建辅助脚本
│       └── build-rpm.sh      # RPM 构建辅助脚本（未来）
├── packaging/                 # 打包资源和配置
│   ├── man/                  # man 手册页
│   ├── completions/          # Shell 补全脚本
│   └── debian/               # DEB 包维护者脚本
├── .github/workflows/        # CI/CD 工作流
│   ├── ci.yml               # 持续集成
│   └── release.yml          # 自动发布
└── Cargo.toml                # Rust 项目配置
```

### 包格式抽象

每种包格式都实现以下接口：

1. **构建接口**: 从源码生成分发包
2. **依赖接口**: 声明包依赖关系
3. **安装接口**: 安装到目标系统
4. **卸载接口**: 从目标系统移除
5. **元数据接口**: 包描述、版本、维护者信息

## 支持的平台

### 当前支持

| 平台 | 包格式 | 状态 |
|------|--------|------|
| Linux | Shell 脚本 | ✅ 完成 (v0.4.0) |
| Linux | DEB 包 | ✅ 完成 (v0.4.0) |
| macOS | Shell 脚本 | ✅ 完成 (v0.4.0) |

### 计划支持

| 平台 | 包格式 | 优先级 | 预计版本 |
|------|--------|--------|----------|
| Linux | RPM 包 | P4 | v0.5.0 |
| macOS | Homebrew | P4 | v0.5.0 |
| Windows | Chocolatey | P4 | v0.5.0 |
| Linux | APT 仓库 | P4 | v0.6.0 |
| Linux | Snap | P4 | v0.6.0 |

## 添加新的包格式支持

### 步骤 1: 创建构建脚本

在 `scripts/build/` 中创建新的构建脚本：

```bash
# 示例：scripts/build/build-rpm.sh
#!/bin/bash
set -e

VERSION="${1:-latest}"
OUTPUT_DIR="target/rpm"

echo "Building RPM package for version $VERSION..."

# 构建 Rust 项目
cargo build --release

# 使用 cargo-generate-rpm 或其他工具生成 RPM
cargo generate-rpm

echo "RPM package built successfully!"
ls -lh "$OUTPUT_DIR"
```

### 步骤 2: 添加 CI/CD 工作流

在 `.github/workflows/` 中创建新的工作流文件：

```yaml
# .github/workflows/build-rpm.yml
name: Build RPM

on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:

jobs:
  build-rpm:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Build RPM
        run: |
          chmod +x scripts/build/build-rpm.sh
          ./scripts/build/build-rpm.sh
```

### 步骤 3: 更新 Cargo.toml

如果需要 Rust 工具集成，在 `Cargo.toml` 中添加配置：

```toml
# 示例：RPM 包配置（如果使用 cargo-generate-rpm）
[package.metadata.generate-rpm]
assets = [
    ["target/release/agentswitch", "/usr/bin/asw", "0755"],
]
```

### 步骤 4: 创建维护者脚本

在 `packaging/` 中创建对应的脚本目录：

```
packaging/
├── rpm/                     # RPM 包配置
│   └── postinst
├── homebrew/                # Homebrew 配置
│   └── agentswitch.rb
└── chocolatey/              # Chocolatey 配置
    └── tools/chocolateyinstall.ps1
```

### 步骤 5: 更新文档

在以下文档中添加新包格式的安装说明：

- `README.md` - 更新"安装"部分
- `INSTALL.md` - 添加详细的安装步骤
- `quickstart.md` - 更新快速开始指南
- `CHANGELOG.md` - 记录新功能

## 添加新的平台支持

### 支持新的操作系统

1. **更新 `scripts/install.sh`**:
   - 在 `detect_system()` 函数中添加新的 OS 检测逻辑
   - 在 `download_binary()` 函数中添加对应的下载 URL 映射
   - 测试安装脚本在新平台上的功能

2. **更新 `.github/workflows/release.yml`**:
   - 在 `matrix.include` 中添加新的 target 配置
   - 确保 GitHub Actions 支持该平台的构建环境

### 支持新的架构

1. **更新 `scripts/install.sh`**:
   - 在 `detect_system()` 函数中添加新的架构检测
   - 确保二进制下载 URL 正确

2. **更新 CI/CD 配置**:
   - 添加新的构建目标到矩阵
   - 配置交叉编译工具链

## 包格式实现指南

### DEB 包（已完成）

**工具**: `cargo-deb`
**配置文件**: `Cargo.toml` `[package.metadata.deb]`
**维护者脚本**: `packaging/debian/{postinst,prerm,postrm}`
**文档**: [contracts/deb-package.md](../specs/001-install-packaging/contracts/deb-package.md)

### RPM 包（计划中）

**工具**: `cargo-generate-rpm` 或 `rpmbuild`
**配置文件**: `Cargo.toml` `[package.metadata.generate-rpm]` (计划)
**Spec 文件**: `packaging/rpm/agentswitch.spec` (计划)

**实现步骤**:
1. 安装 `cargo-generate-rpm`: `cargo install cargo-generate-rpm`
2. 创建 `.spec` 文件
3. 在 `scripts/build/build-rpm.sh` 中添加构建逻辑
4. 在 CI/CD 中添加 RPM 构建步骤

### Homebrew（计划中）

**工具**: Homebrew Formula
**配置文件**: `packaging/homebrew/agentswitch.rb`
**仓库**: Homebrew Tap (单独的 GitHub 仓库)

**实现步骤**:
1. 创建 Homebrew Tap 仓库
2. 编写 Formula 文件
3. 提交到 Tap 仓库
4. 用户可以通过 `brew install` 安装

**Formula 示例**:
```ruby
# agentswitch.rb
class Agentswitch < Formula
  desc "Universal code agent tool configuration switcher"
  homepage "https://github.com/Yu-Xiao-Sheng/agentswitch"
  url "https://github.com/Yu-Xiao-Sheng/agentswitch/archive/v#{version}.tar.gz"
  sha256 ""

  depends_on "rust"

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", libexec
    bin.install libexec/"agentswitch"
  end

  test do
    system "#{bin}/agentswitch", "--version"
  end
end
```

### Chocolatey（计划中）

**工具**: Chocolatey
**配置文件**: `packaging/chocolatey/agentswitch.nuspec`
**脚本**: `packaging/chocolatey/tools/chocolateyInstall.ps1`

**实现步骤**:
1. 创建 `.nuspec` 文件
2. 创建 PowerShell 安装脚本
3. 提交到 Chocolatey 社区仓库
4. 用户可以通过 `choco install` 安装

## CI/CD 模块化设计

### 工作流分离

目前的工作流结构：

```
.github/workflows/
├── ci.yml                  # 持续集成（测试、lint）
└── release.yml            # 自动发布（多平台构建、DEB 包）
```

### 未来扩展

计划中的工作流：

```
.github/workflows/
├── ci.yml
├── release.yml
├── build-rpm.yml          # RPM 包构建（独立工作流）
├── publish-homebrew.yml   # Homebrew 发布（独立工作流）
└── publish-chocolatey.yml # Chocolatey 发布（独立工作流）
```

### 通用构建函数

创建可复用的构建步骤：

```yaml
# .github/workflows/reusable-build.yml
on:
  workflow_call:
    inputs:
      target:
        required: true
        type: string
      package_type:
        required: true
        type: string

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Build package
        run: |
          if [ "${{ inputs.package_type }}" = "deb" ]; then
            cargo deb
          elif [ "${{ inputs.package_type }}" = "rpm" ]; then
            ./scripts/build/build-rpm.sh
          fi
```

## 测试和验证

### 本地测试

每种包格式都应提供本地测试方法：

```bash
# DEB 包测试
cargo deb
sudo dpkg -i target/debian/agentswitch_*.deb

# RPM 包测试
./scripts/build/build-rpm.sh
sudo rpm -i target/rpm/*.rpm

# Homebrew 测试
brew install ./packaging/homebrew/agentswitch.rb
```

### Docker 测试

使用 Docker 容器测试包安装：

```bash
# DEB 包测试
docker run --rm -v target/debian/*.deb:/tmp/pkg ubuntu:22.04 \
  dpkg -i /tmp/pkg

# RPM 包测试
docker run --rm -v target/rpm/*.rpm:/tmp/rpm fedora:latest \
  rpm -i /tmp/rpm
```

## 发布流程

### 自动发布流程

1. 开发者创建 Git tag: `git tag v0.5.0 && git push origin v0.5.0`
2. GitHub Actions 自动触发 `release.yml` 工作流
3. 构建所有平台的二进制和包
4. 创建 GitHub Release 并上传所有产物
5. 用户可以从 Releases 页面下载

### 手动发布流程

如果自动发布失败，可以手动发布：

1. 本地构建所有平台和包
2. 创建 GitHub Release
3. 手动上传构建产物
4. 发布 Release

## 维护和更新

### 更新现有包格式

当需要更新现有包格式的配置时：

1. 更新对应的配置文件
2. 更新构建脚本
3. 在本地测试构建
4. 提交 PR 测试 CI/CD
5. 合并 PR

### 添加新的包格式

遵循"添加新的包格式支持"部分的步骤。

## 故障排除

### 常见问题

**Q: DEB 包构建失败**
A: 检查 `Cargo.toml` 中的 `[package.metadata.deb]` 配置，确保所有路径正确

**Q: CI/CD 工作流失败**
A: 查看 GitHub Actions 日志，确认构建环境是否支持目标平台

**Q: 交叉编译失败**
A: 某些平台需要特定的工具链，确保 CI/CD 环境中已安装

## 参考资料

- [cargo-deb GitHub](https://github.com/kornelski/cargo-deb)
- [cross-rs GitHub](https://github.com/cross-rs/cross)
- [GitHub Actions Docs](https://docs.github.com/en/actions)
- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Chocolatey Docs](https://docs.chocolatey.org/)

## 贡献指南

如果您想为 AgentSwitch 添加新的包格式或平台支持：

1. 查看 [issue tracker](https://github.com/Yu-Xiao-Sheng/agentswitch/issues) 确认没有人正在进行
2. Fork 项目仓库
3. 创建功能分支：`git checkout -b feature/add-rpm-support`
4. 按照本文档指南实现
5. 测试您的更改
6. 提交 PR 并描述您的更改
7. 等待代码审查和合并

---

**最后更新**: 2026-03-11
**维护者**: Yu-Xiao-Sheng
**许可证**: MIT
