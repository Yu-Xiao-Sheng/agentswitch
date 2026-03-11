# Shell 安装脚本接口契约

**功能**: Spec 001 - 便捷安装与分发系统
**契约类型**: Shell 脚本命令行接口
**版本**: 1.0
**日期**: 2026-03-11

## 概述

本契约定义 AgentSwitch Shell 安装脚本的外部接口，包括命令行参数、环境变量、行为规范和退出码。

## 命令行接口

### 基本用法

```bash
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
```

### 交互式使用

```bash
# 下载后执行
curl -O https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh
bash install.sh

# 带参数执行
bash install.sh --uninstall
bash install.sh --version
bash install.sh --help
```

## 命令行参数

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `--help, -h` | flag | - | 显示帮助信息并退出 |
| `--version, -V` | flag | - | 显示脚本版本并退出 |
| `--uninstall` | flag | false | 卸载 AgentSwitch |
| `--dry-run` | flag | false | 模拟运行，不实际安装 |
| `--force, -f` | flag | false | 强制覆盖已安装的版本 |
| `--yes, -y` | flag | false | 自动确认所有提示 |
| `--verbose, -v` | flag | false | 显示详细的安装日志 |

## 环境变量

| 变量名 | 类型 | 默认值 | 描述 |
|--------|------|--------|------|
| `INSTALL_DIR` | path | `/usr/local/bin` | 二进制文件安装目录 |
| `NO_MODIFY_PATH` | bool | `false` | 不修改 Shell 配置文件中的 PATH |
| `ASW_VERSION` | string | `latest` | 指定安装版本（默认最新） |
| `GITHUB_TOKEN` | string | - | GitHub API 令牌（用于私有仓库） |

## 行为规范

### 1. 系统检测

脚本 MUST 自动检测以下信息：

| 检测项 | 方法 | 用途 |
|--------|------|------|
| 操作系统 | `uname -s` | 选择对应的二进制文件 |
| 架构 | `uname -m` | 选择对应的二进制文件 |
| Shell 类型 | `$SHELL` | 配置对应的补全脚本 |
| 安装状态 | `command -v asw` | 检测是否已安装 |

**架构映射**:
- `x86_64`, `amd64`, `x64` → `x86_64`
- `aarch64`, `arm64` → `aarch64`
- `armv7l`, `armv6l` → `armv7` (未来支持)
- 其他 → 不支持，显示错误

**操作系统映射**:
- `Linux` → `unknown-linux-gnu`
- `Darwin` → `apple-darwin`
- `MINGW*`, `MSYS*`, `CYGWIN*` → `pc-windows-msvc` (未来支持)
- 其他 → 不支持，显示错误

### 2. 安装流程

```text
1. 解析命令行参数和环境变量
2. 检测操作系统和架构
3. 检查是否已安装 AgentSwitch
   - 如果已安装且未指定 --force，提示用户确认
4. 下载对应的二进制文件
   - 使用 GitHub API 获取最新版本
   - 下载到临时目录（/tmp/asw-install）
   - 验证文件大小和权限
5. 验证二进制文件
   - 检查文件是否可执行
   - 运行 `asw --version` 验证版本
6. 安装二进制文件
   - 检查 INSTALL_DIR 是否存在，不存在则创建
   - 检查写入权限，无权限时提示使用 sudo
   - 复制二进制到 INSTALL_DIR
   - 设置权限为 0755
7. 配置 Shell 补全
   - 检测用户 Shell（bash/zsh/fish）
   - 下载对应的补全脚本
   - 安装到对应的补全目录
   - 修改 Shell 配置文件（如果需要）
8. 显示安装成功消息
   - 显示安装路径
   - 显示版本信息
   - 显示快速开始命令
```

### 3. 下载行为

- 下载 URL 格式:
  ```
  https://github.com/Yu-Xiao-Sheng/agentswitch/releases/download/v{version}/agentswitch-{arch}-{os}.tar.gz
  ```

- 下载失败 MUST 自动重试，最多 3 次
- 重试间隔: 2 秒
- 最终失败 MUST 显示错误消息和手动下载链接

- 下载进度:
  - 默认: 不显示进度（静默模式）
  - `--verbose`: 显示下载进度

### 4. 卸载流程

```text
1. 检查 AgentSwitch 是否已安装
   - 如果未安装，显示错误并退出
2. 显示将要删除的文件列表
   - 二进制文件路径
   - 补全脚本路径
3. 提示用户确认（除非指定 --yes）
4. 删除二进制文件
5. 删除补全脚本
6. 清理 Shell 配置文件中的补全配置
7. 提示是否删除配置文件（~/.agentswitch/）
8. 显示卸载完成消息
```

### 5. 错误处理

| 错误场景 | 退出码 | 行为 |
|----------|--------|------|
| 用户中断 (Ctrl+C) | 130 | 清理临时文件，退出 |
| 不支持的操作系统 | 1 | 显示错误和支持的 OS 列表 |
| 不支持的架构 | 1 | 显示错误和支持的架构列表 |
| 下载失败 | 1 | 显示错误和手动下载链接，退出 |
| 无写入权限 | 1 | 提示使用 sudo 或设置 INSTALL_DIR |
| 二进制损坏 | 1 | 显示错误，建议重新安装 |
| 网络不可达 | 1 | 显示错误和故障排除建议 |

### 6. 安全要求

- MUST 使用 HTTPS 下载所有文件
- MUST 不在脚本中硬编码敏感信息
- MUST 不修改系统关键配置（除了 Shell 配置文件的补全部分）
- MUST 不要求用户输入密码（除了 sudo 提示）
- SHOULD 在脚本开头添加安全警告注释

### 7. 用户体验要求

#### 输出格式

**成功安装**:
```
✓ Detected system: Linux x86_64
✓ Detected shell: bash
✓ Downloading agentswitch v0.4.0...
✓ Installing to /usr/local/bin...
✓ Configuring bash completion...
✓ Installation complete!

AgentSwitch v0.4.0 has been installed successfully.
Binary: /usr/local/bin/asw
Config: ~/.agentswitch/

Quick start:
  asw model list
  asw --help

For more information, visit: https://github.com/Yu-Xiao-Sheng/agentswitch
```

**错误示例**:
```
✗ Error: Unsupported architecture: ppc64le

Supported architectures:
  - x86_64 (AMD64/Intel 64-bit)
  - aarch64 (ARM64/Apple Silicon)

Please install from source: https://github.com/Yu-Xiao-Sheng/agentswitch#building-from-source
```

#### 交互提示

**覆盖确认**:
```
AgentSwitch v0.3.0 is already installed at /usr/local/bin/asw.
Do you want to upgrade to v0.4.0? [Y/n]
```

**卸载确认**:
```
This will remove AgentSwitch from your system:
  - Binary: /usr/local/bin/asw
  - Bash completion: /usr/share/bash-completion/completions/asw

Do you want to continue? [Y/n]
```

**配置文件删除**:
```
Do you want to remove your configuration directory (~/.agentswitch/)? [y/N]
This will delete all your model configurations and presets.
```

## Shell 补全配置

### Bash

**补全脚本安装路径**: `/usr/share/bash-completion/completions/asw`

**配置文件修改**:
- 如果 `~/.bash_completion` 或 `/etc/bash_completion.d/` 可用，不修改 `.bashrc`
- 否则，在 `~/.bashrc` 中添加:
  ```bash
  # AgentSwitch bash completion
  source ~/.agentswitch/completion.bash 2>/dev/null || true
  ```

### Zsh

**补全脚本安装路径**: `/usr/share/zsh/vendor-completions/_asw` 或 `~/.zsh/completion/_asw`

**配置文件修改**:
- 在 `~/.zshrc` 中添加:
  ```zsh
  # AgentSwitch zsh completion
  fpath=(~/.agentswitch/completion.zsh $fpath)
  autoload -U compinit && compinit
  ```

### Fish

**补全脚本安装路径**: `~/.config/fish/completions/asw.fish`

**配置文件修改**:
- Fish 自动加载 `~/.config/fish/completions/` 下的补全脚本
- 无需修改配置文件

## 测试要求

### 必须测试的场景

1. **全新安装** (Fresh install)
   - 干净的 Linux 系统
   - 验证二进制可以执行
   - 验证补全功能正常

2. **覆盖安装** (Upgrade install)
   - 已安装旧版本
   - 验证配置文件保留
   - 验证版本更新成功

3. **自定义安装路径** (Custom install dir)
   - 设置 `INSTALL_DIR=~/bin`
   - 验证安装到指定目录
   - 验证 PATH 配置正确

4. **卸载** (Uninstall)
   - 验证二进制删除
   - 验证补全脚本删除
   - 验证配置文件可选删除

5. **错误处理** (Error handling)
   - 不支持的架构
   - 无写入权限
   - 下载失败
   - 网络不可达

6. **跨平台** (Cross-platform)
   - Linux x86_64
   - Linux ARM64
   - macOS Intel
   - macOS Apple Silicon

## 版本控制

脚本 SHOULD 在帮助信息中显示版本号：

```bash
$ bash install.sh --version
AgentSwitch Installer v1.0.0
```

版本格式遵循语义化版本：`MAJOR.MINOR.PATCH`

## 文档要求

脚本 MUST 包含以下内联文档：

1. **文件头部注释**:
   ```bash
   #!/bin/sh
   # AgentSwitch Installer
   # Usage: curl -sSL https://.../install.sh | bash
   # Environment variables:
   #   INSTALL_DIR - Installation directory (default: /usr/local/bin)
   #   NO_MODIFY_PATH - Don't modify PATH (default: false)
   ```

2. **函数注释**:
   ```bash
   # Detect operating system and architecture
   # Returns: OS and architecture string
   detect_system() {
       ...
   }
   ```

## 参考资料

- [Rustup Installer](https://github.com/rust-lang/rustup/blob/master/rustup-init.sh)
- [kubectl Installer](https://github.com/kubernetes/kubernetes/blob/master/cluster/get-kube-binaries.sh)
- [Helm Installer](https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3)
