# AgentSwitch 包格式扩展路线图

**版本**: 1.0.0
**日期**: 2026-03-11

## 概述

本文档描述 AgentSwitch 包格式支持的扩展路线图，包括计划中的包格式、实现优先级和预计时间表。

## 当前状态 (v0.4.0)

### 已完成

✅ **Shell 安装脚本** (Linux/macOS)
- 支持平台: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon)
- 安装方式: `curl ... | bash`
- 状态: 生产就绪

✅ **DEB 包** (Debian/Ubuntu)
- 支持发行版: Debian 11+, Ubuntu 20.04+
- 安装方式: `dpkg -i agentswitch_*.deb`
- 状态: 生产就绪

## Phase 1: Linux 增强支持 (v0.5.0)

### RPM 包 (Red Hat/CentOS/Fedora) - P4

**优先级**: 高
**用户需求**: 许多企业级 Linux 发行版使用 RPM 包管理器

**实现方案**:
- 工具选择: `cargo-generate-rpm` 或 `rpm-build`
- 支持发行版: Fedora 38+, RHEL 9+, CentOS Stream 9+
- 配置文件: `packaging/rpm/agentswitch.spec`
- 构建脚本: `scripts/build/build-rpm.sh`

**技术挑战**:
- RPM spec 文件语法复杂
- 需要处理不同发行版的依赖差异
- GPG 签名要求

**预估工作量**: 3-5 天

**用户价值**:
- 企业用户更容易采用
- 与现有 RPM 仓库集成
- 统一的包管理体验

### Snap 包 - P4

**优先级**: 中
**用户需求**: Ubuntu 用户习惯使用 Snap

**实现方案**:
- 工具: Snapcraft
- 配置文件: `snap/snapcraft.yaml`
- 自动发布到 Snap Store

**技术挑战**:
- Snap 严格的沙盒限制
- 需要处理 Classic 模式要求

**预估工作量**: 2-3 天

## Phase 2: macOS 增强支持 (v0.5.0)

### Homebrew Formula - P4

**优先级**: 高
**用户需求**: macOS 用户最常用的包管理方式

**实现方案**:
- 创建 Homebrew Tap 仓库
- Formula 文件: `agentswitch.rb`
- 支持 `brew install agentswitch`
- 自动更新机制

**技术挑战**:
- 需要维护单独的 Tap 仓库
- Homebrew 代码审查要求严格
- 需要处理 Bottle（预编译二进制）

**预估工作量**: 3-4 天

**实现步骤**:

1. **创建 Tap 仓库**
   ```bash
   gh repo create homebrew-tap --public
   ```

2. **编写 Formula**
   ```ruby
   class Agentswitch < Formula
     desc "Universal code agent tool configuration switcher"
     homepage "https://github.com/Yu-Xiao-Sheng/agentswitch"
     url "https://github.com/Yu-Xiao-Sheng/agentswitch/archive/v#{version}.tar.gz"
     sha256 ""
     license "MIT"

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

3. **提交到 Tap 仓库**
   ```bash
   git add .
   git commit -m "Add agentswitch formula"
   git push origin main
   ```

4. **用户安装**
   ```bash
   brew tap Yu-Xiao-Sheng/agentswitch
   brew install agentswitch
   ```

## Phase 3: Windows 支持 (v0.6.0)

### Chocolatey Package - P4

**优先级**: 中
**用户需求**: Windows 用户需要便捷的安装方式

**实现方案**:
- 工具: Chocolatey
- 配置文件: `packaging/chocolatey/agentswitch.nuspec`
- 安装脚本: `tools/chocolateyInstall.ps1`
- 发布到 Chocolatey Community Repository

**技术挑战**:
- Windows 特定的路径问题
- 需要处理 PATH 环境变量
- 代码签名证书（可选但推荐）

**预估工作量**: 4-5 天

### Windows 安装程序 - P4

**优先级**: 低
**用户需求**: 部分 Windows 用户更喜欢图形化安装程序

**实现方案**:
- 使用 NSIS 或 Inno Setup
- 创建图形化安装向导
- 包含可选组件（Shell 补全、文档）

**预估工作量**: 5-7 天

## Phase 4: 仓库和分发 (v0.6.0)

### APT 仓库 - P4

**优先级**: 中
**用户需求**: 企业用户需要私有 APT 仓库

**实现方案**:
- 使用 reprepro 或 Aptly
- 托管在服务器或 GitHub Pages
- 支持 `apt install agentswitch`

**技术挑战**:
- 需要维护服务器
- GPG 签名管理
- 带宽成本

**预估工作量**: 5-7 天

### YUM/DNF 仓库 - P4

**优先级**: 中
**用户需求**: RHEL 系列用户

**实现方案**:
- 使用 createrepo
- 与 RPM 包集成
- 支持企业级部署

**预估工作量**: 3-5 天

## Phase 5: 高级功能 (v0.7.0+)

### 自动更新检测 - P4

**优先级**: 低
**用户需求**: 用户希望自动获取更新通知

**实现方案**:
- 在 CLI 中添加 `asw update` 命令
- 检查 GitHub Releases API
- 提示用户更新或自动下载

**技术挑战**:
- 需要处理更新策略（仅通知 vs 自动安装）
- 安全性考虑（验证签名）

**预估工作量**: 3-4 天

### 包格式自动转换 - P4

**优先级**: 低
**用户需求**: 开发者希望从一种格式转换为另一种

**实现方案**:
- 命令行工具：`asw packaging convert --to rpm`
- 使用 alien 或类似工具

**预估工作量**: 4-5 天

## 时间表

| 版本 | 功能 | 目标日期 | 状态 |
|------|------|----------|------|
| v0.4.0 | Shell 脚本 + DEB 包 | 2026-03-11 | ✅ 完成 |
| v0.5.0 | RPM + Homebrew + Snap | 2026-04-30 | 📅 计划中 |
| v0.6.0 | Chocolatey + APT/YUM 仓库 | 2026-06-30 | 📅 计划中 |
| v0.7.0+ | 自动更新 + 包转换 | 待定 | 📅 未来 |

## 贡献指南

我们欢迎社区贡献！如果您想实现某个包格式支持：

1. 查看 [issues](https://github.com/Yu-Xiao-Sheng/agentswitch/issues) 确认未被认领
2. 查看 [打包架构文档](packaging.md) 了解实现指南
3. Fork 仓库并创建功能分支
4. 实现并测试
5. 提交 PR

## 参与讨论

如果您对路线图有建议或想要讨论实现细节：

- GitHub Issues: [创建新 issue](https://github.com/Yu-Xiao-Sheng/agentswitch/issues/new)
- GitHub Discussions: [发起讨论](https://github.com/Yu-Xiao-Sheng/agentswitch/discussions)

---

**最后更新**: 2026-03-11
**维护者**: Yu-Xiao-Sheng
