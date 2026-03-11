# AgentSwitch 安装指南

**版本**: v0.5.0
**更新日期**: 2026-03-11

## 目录

- [系统要求](#系统要求)
- [安装方法](#安装方法)
  - [Shell 脚本安装](#shell-脚本安装)
  - [DEB 包安装](#deb-包安装)
  - [从源码编译](#从源码编译)
- [配置](#配置)
- [升级](#升级)
- [卸载](#卸载)
- [故障排除](#故障排除)
- [支持的平台](#支持的平台)

## 系统要求

### Linux

- **发行版**: Ubuntu 20.04+, Debian 11+, 或其他主流发行版
- **架构**: x86_64 (AMD64) 或 ARM64
- **依赖**: `curl` 或 `wget`（用于下载安装脚本）

### macOS

- **版本**: macOS 11+ (Big Sur 或更高版本)
- **架构**: Intel (x86_64) 或 Apple Silicon (ARM64)
- **依赖**: `curl`（系统自带）

### Windows

- **支持方式**: WSL (Windows Subsystem for Linux)
- **推荐**: Ubuntu 22.04 LTS WSL

## 安装方法

### Shell 脚本安装

#### Linux / macOS

**基本安装**:

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

**使用 wget**:

```bash
wget -qO- https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

**自定义选项**:

```bash
# 自定义安装目录
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash -s -- --install-dir ~/bin

# 指定版本
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | ASW_VERSION=0.4.0 bash

# 查看帮助
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash -s -- --help
```

**环境变量**:

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `INSTALL_DIR` | 安装目录 | `/usr/local/bin` |
| `NO_MODIFY_PATH` | 不修改 PATH 配置 | `false` |
| `ASW_VERSION` | 指定版本 | `latest` |

#### Windows (WSL)

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

### DEB 包安装

#### 下载 DEB 包

从 [GitHub Releases](https://github.com/Yu-Xiao-Sheng/agentswitch/releases) 下载最新版本的 `.deb` 文件：

```bash
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.4.0/agentswitch_0.4.0_amd64.deb
```

#### 安装 DEB 包

```bash
sudo dpkg -i agentswitch_0.4.0_amd64.deb
```

如果出现依赖问题：

```bash
sudo apt-get install -f
```

#### 验证安装

```bash
which asw
asw --version
```

#### 查看已安装的文件

```bash
dpkg -L agentswitch
```

#### 卸载

```bash
# 卸载但保留配置
sudo apt remove agentswitch

# 完全卸载（包括配置）
sudo apt purge agentswitch
```

### 从源码编译

#### 前置要求

- Rust 1.70+（使用 [rustup](https://rustup.rs/) 安装）
- Git

#### 克隆和编译

```bash
# 克隆仓库
git clone https://github.com/Yu-Xiao-Sheng/agentswitch.git
cd agentswitch

# 编译安装
cargo install --path .

# 或直接运行
cargo run --release
```

#### 从源码安装 Shell 补全

```bash
# Bash
asw completion generate bash > ~/.local/share/bash-completion/completions/asw

# Zsh
asw completion generate zsh > ~/.local/share/zsh/site-functions/_asw

# Fish
asw completion generate fish > ~/.config/fish/completions/asw.fish
```

## 配置

### 首次运行

安装完成后，运行初始化向导：

```bash
asw wizard init
```

向导会引导你：
1. 检测已安装的 Code Agent 工具
2. 添加第一个模型配置
3. 测试配置是否正常

### 手动添加模型

```bash
# 添加 GLM-4 模型
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-your-api-key" \
  --model "glm-4"
```

### 配置文件位置

- **配置文件**: `~/.agentswitch/config.toml`
- **预设目录**: `~/.agentswitch/presets/`
- **备份目录**: `~/.agentswitch/backups/`

## 升级

### 使用 Shell 脚本升级

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

脚本会自动检测已安装的版本并提示升级。

### 使用 DEB 包升级

```bash
# 下载新版本
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v0.5.0/agentswitch_0.5.0_amd64.deb

# 升级
sudo dpkg -i agentswitch_0.5.0_amd64.deb

# 或使用 APT
sudo apt update
sudo apt install agentswitch
```

### 从源码升级

```bash
cd agentswitch
git pull origin main
cargo install --path
```

## 卸载

### Shell 脚本卸载

```bash
bash scripts/install.sh --uninstall
```

### DEB 包卸载

```bash
# 保留配置
sudo apt remove agentswitch

# 删除配置
sudo apt purge agentswitch
```

### 从源码卸载

```bash
cargo uninstall agentswitch
rm -rf ~/.agentswitch/
```

## 故障排除

### 权限错误

**问题**: `Permission denied` 或 `cannot create directory`

**解决方案 A**: 使用 sudo
```bash
curl -sSL ... | sudo bash
```

**解决方案 B**: 安装到用户目录
```bash
curl -sSL ... | INSTALL_DIR=~/bin bash
```

### 网络问题

**问题**: 下载失败或超时

**解决方案 A**: 手动下载脚本
```bash
curl -O https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh
bash install.sh
```

**解决方案 B**: 设置代理
```bash
export https_proxy=http://proxy.example.com:8080
curl -sSL ... | bash
```

### 架构不支持

**问题**: `Unsupported architecture: xxx`

**解决方案**: 检查系统架构
```bash
uname -m
```

如果不支持，请从源码编译。

### DEB 包依赖问题

**问题**: dpkg 报告依赖错误

**解决方案**:
```bash
sudo apt-get install -f
```

### Shell 补全不工作

**问题**: Tab 补全不生效

**解决方案**: 重新加载 Shell 配置
```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc
```

### PATH 配置

**问题**: 找不到 asw 命令

**解决方案**: 添加到 PATH
```bash
# 临时
export PATH="$PATH:/usr/local/bin"

# 永久（Bash）
echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc
source ~/.bashrc
```

## 支持的平台

### Linux

| 发行版 | 版本 | 架构 | 状态 |
|--------|------|------|------|
| Ubuntu | 20.04+ | x86_64, ARM64 | ✅ 支持 |
| Debian | 11+ | x86_64, ARM64 | ✅ 支持 |
| Fedora | 38+ | x86_64, ARM64 | ⚠️ 计划中 (v0.5.0) |
| CentOS Stream | 9+ | x86_64, ARM64 | ⚠️ 计划中 (v0.5.0) |

### macOS

| 版本 | 架构 | 状态 |
|------|------|------|
| Big Sur+ (11+) | Intel (x86_64) | ✅ 支持 |
| Big Sur+ (11+) | Apple Silicon (ARM64) | ✅ 支持 |

### Windows

| 方式 | 状态 |
|------|------|
| WSL (Ubuntu 22.04) | ✅ 支持 |
| 原生 Windows | ⚠️ 计划中 (v0.6.0) |

## 获取帮助

### 文档

- [README.md](README.md) - 项目概述和快速开始
- [docs/packaging.md](docs/packaging.md) - 打包系统架构
- [docs/roadmap.md](docs/roadmap.md) - 功能路线图

### 社区

- [GitHub Issues](https://github.com/Yu-Xiao-Sheng/agentswitch/issues) - 报告问题
- [GitHub Discussions](https://github.com/Yu-Xiao-Sheng/agentswitch/discussions) - 讨论和交流

### 许可证

本项目采用 [MIT License](LICENSE) 开源协议。
