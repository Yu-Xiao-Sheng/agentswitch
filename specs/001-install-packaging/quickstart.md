# 快速开始指南: AgentSwitch 安装

**功能**: Spec 001 - 便捷安装与分发系统
**日期**: 2026-03-11
**目标用户**: 想要快速安装和使用 AgentSwitch 的用户

## 概述

本指南提供多种安装 AgentSwitch 的方式，选择最适合你的环境：

| 方式 | 适用场景 | 时间 | 难度 |
|------|----------|------|------|
| **Shell 脚本** | Linux/macOS 快速安装 | 2 分钟 | ⭐ 简单 |
| **DEB 包** | Debian/Ubuntu 系统集成 | 3 分钟 | ⭐ 简单 |
| **预编译二进制** | 任何 Linux/macOS 系统 | 5 分钟 | ⭐⭐ 中等 |
| **从源码编译** | 开发者或自定义需求 | 10 分钟 | ⭐⭐⭐ 复杂 |

## 方式 1: Shell 脚本安装（推荐）

### Linux / macOS

一条命令完成安装：

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

### Windows (WSL)

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

### 自定义安装目录

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash -s -- --install-dir ~/bin
```

### 查看安装选项

```bash
bash -c "$(curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh)" -- --help
```

### 卸载

```bash
bash -c "$(curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh)" -- --uninstall
```

### 安装后验证

```bash
# 检查版本
asw --version

# 查看帮助
asw --help

# 列出已配置的模型（如果没有，会引导初始化）
asw model list
```

## 方式 2: DEB 包安装（Debian/Ubuntu）

### 下载 DEB 包

从 [GitHub Releases](https://github.com/Yu-Xiao-Sheng/agentswitch/releases) 下载最新的 `.deb` 文件：

```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch_0.4.0_amd64.deb
```

### 安装

```bash
sudo dpkg -i agentswitch_0.4.0_amd64.deb

# 如果有依赖问题，运行：
sudo apt-get install -f
```

### 从 APT 仓库安装（未来）

```bash
# 添加仓库
echo "deb [signed-by=/usr/share/keyrings/agentswitch-archive-keyring.gpg] https://repo.example.com stable main" | sudo tee /etc/apt/sources.list.d/agentswitch.list

# 安装
sudo apt update
sudo apt install agentswitch
```

### 卸载

```bash
# 卸载但保留配置
sudo apt remove agentswitch

# 完全卸载（包括配置）
sudo apt purge agentswitch
```

## 方式 3: 预编译二进制

### 下载

从 [GitHub Releases](https://github.com/Yu-Xiao-Sheng/agentswitch/releases) 下载对应的压缩包：

**Linux x86_64 (AMD64)**:
```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch-x86_64-unknown-linux-gnu.tar.gz
```

**Linux ARM64**:
```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch-aarch64-unknown-linux-gnu.tar.gz
```

**macOS Intel**:
```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch-x86_64-apple-darwin.tar.gz
```

**macOS Apple Silicon**:
```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch-aarch64-apple-darwin.tar.gz
```

### 解压和安装

```bash
# 解压
tar -xzf agentswitch-*-*.tar.gz

# 安装
sudo cp asw /usr/local/bin/

# 设置可执行权限
sudo chmod +x /usr/local/bin/asw
```

### 验证安装

```bash
asw --version
```

## 方式 4: 从源码编译

### 前置要求

- Rust 1.70+ (使用 [rustup](https://rustup.rs/) 安装)
- Git

### 克隆和编译

```bash
# 克隆仓库
git clone https://github.com/Yu-Xiao-Sheng/agentswitch.git
cd agentswitch

# 编译
cargo build --release

# 安装
sudo cp target/release/asw /usr/local/bin/
sudo chmod +x /usr/local/bin/asw
```

### 安装 Shell 补全

```bash
# 生成补全脚本
asw completion generate bash > /tmp/asw.bash
sudo cp /tmp/asw.bash /usr/share/bash-completion/completions/asw

# Zsh
asw completion generate zsh > /tmp/asw.zsh
sudo cp /tmp/asw.zsh /usr/share/zsh/vendor-completions/_asw

# Fish
asw completion generate fish > ~/.config/fish/completions/asw.fish
```

## 首次使用

### 运行初始化向导

```bash
asw wizard init
```

向导会引导你：
1. 检测已安装的 Code Agent 工具
2. 添加第一个模型配置
3. 测试配置是否正常

### 手动添加模型配置

```bash
# 添加 GLM-4 模型
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-your-api-key" \
  --model "glm-4"

# 列出所有模型
asw model list

# 切换 Claude Code 到 GLM-4
asw switch claude-code glm
```

## 故障排除

### 权限错误

**问题**: `Permission denied` 或 `cannot create directory`

**解决**:
```bash
# 使用 sudo
sudo bash install.sh

# 或安装到用户目录
curl -sSL https://.../install.sh | bash -s -- --install-dir ~/bin
```

### 网络问题

**问题**: 下载失败或超时

**解决**:
```bash
# 手动下载安装脚本
curl -O https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh

# 查看并编辑脚本（如需要设置代理）
vim install.sh

# 执行
bash install.sh
```

### 架构不支持

**问题**: `Unsupported architecture: xxx`

**解决**:
- 检查你的架构：`uname -m`
- 从源码编译（参见方式 4）
- 或使用支持的系统（x86_64 / ARM64）

### Shell 补全不工作

**问题**: Tab 补全不生效

**解决**:
```bash
# 重新加载 Shell 配置
source ~/.bashrc   # Bash
source ~/.zshrc    # Zsh

# 或手动安装补全
asw completion generate bash | sudo tee /usr/share/bash-completion/completions/asw
```

## 下一步

安装完成后，查看以下文档：

- **用户指南**: [README.md](https://github.com/Yu-Xiao-Sheng/agentswitch#readme)
- **完整文档**: [docs/](https://github.com/Yu-Xiao-Sheng/agentswitch/tree/main/docs)
- **故障排除**: [docs/troubleshooting.md](https://github.com/Yu-Xiao-Sheng/agentswitch/blob/main/docs/troubleshooting.md)

## 获取帮助

如果遇到问题：

1. 查看 [FAQ](https://github.com/Yu-Xiao-Sheng/agentswitch#faq)
2. 搜索 [Issues](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)
3. 提交 [新 Issue](https://github.com/Yu-Xiao-Sheng/agentswitch/issues/new)
4. 加入讨论：[GitHub Discussions](https://github.com/Yu-Xiao-Sheng/agentswitch/discussions)

## 更新 AgentSwitch

### 使用安装脚本更新

```bash
# 重新运行安装脚本会提示升级
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

### 使用 DEB 包更新

```bash
# 下载新版本并安装
sudo dpkg -i agentswitch_0.5.0_amd64.deb

# 或使用 APT
sudo apt update
sudo apt upgrade agentswitch
```

### 从源码更新

```bash
cd agentswitch
git pull origin main
cargo build --release
sudo cp target/release/asw /usr/local/bin/
```

## 系统要求

### Linux

- **发行版**: Ubuntu 20.04+, Debian 11+, 或其他主流发行版
- **架构**: x86_64 (AMD64) 或 ARM64
- **依赖**: `curl` 或 `wget`（用于安装脚本）

### macOS

- **版本**: macOS 11+ (Big Sur 或更高)
- **架构**: Intel (x86_64) 或 Apple Silicon (ARM64)
- **依赖**: `curl`（系统自带）

### Windows

- **支持**: 通过 WSL (Windows Subsystem for Linux)
- **推荐**: Ubuntu 22.04 LTS WSL
- **参考**: [WSL 安装指南](https://learn.microsoft.com/en-us/windows/wsl/install)

## 许可证

AgentSwitch 使用 [MIT License](LICENSE) 开源协议。

---

**版本**: v0.4.0 | **更新**: 2026-03-11 | **项目**: [AgentSwitch](https://github.com/Yu-Xiao-Sheng/agentswitch)
