# 快速开始指南: 用户体验优化与高级功能

**功能分支**: `004-ux-optimization`
**创建日期**: 2026-03-10
**文档版本**: 1.0.0

## 概述

本文档为 Spec 004 的用户提供快速上手指南，涵盖交互式配置向导、工具检测、Shell 补全和 Git 同步的常见使用场景。

---

## 前置要求

### 系统要求

- **操作系统**: Linux、macOS 或 Windows
- **Rust**: 最新稳定版（如需从源码编译）
- **Git**: 2.0+（仅用于配置同步功能）
- **Shell**: Bash、Zsh 或 Fish（用于自动补全）

### 安装 AgentSwitch

```bash
# 从源码编译
cargo install --path .

# 或使用预编译二进制
wget https://github.com/Yu-Xiao-Sheng/agentswitch/releases/latest/download/agentswitch-linux
chmod +x agentswitch-linux
sudo mv agentswitch-linux /usr/local/bin/asw
```

---

## 场景 1: 首次使用 - 交互式配置向导

### 1.1 启动向导

首次安装 AgentSwitch 后，运行以下命令启动配置向导：

```bash
asw init
```

### 1.2 向导流程

向导将引导你完成以下步骤：

#### 步骤 1: 欢迎和说明

```
Welcome to AgentSwitch configuration wizard!

This wizard will guide you through setting up your first model configuration.

Press Ctrl+C at any time to exit (progress will be saved).
```

#### 步骤 2: 输入模型配置名称

```
? Model configuration name glm
```

**提示**:
- 选择一个易记的名称（如 `glm`、`gpt-4`、`minimax`）
- 名称将用于后续的 `asw switch` 命令

#### 步骤 3: 输入 Base URL

```
? Base URL https://open.bigmodel.cn/api/v1
```

**常见 Base URL**:
- 智谱 GLM: `https://open.bigmodel.cn/api/v1`
- OpenAI: `https://api.openai.com/v1`
- MiniMax: `https://api.minimax.chat/v1`
- 本地模型: `http://localhost:11434/v1` (Ollama)

#### 步骤 4: 输入 API Key

```
? API Key ****************************
```

**提示**:
- API Key 会以掩码形式显示
- 输入长度至少 32 个字符

#### 步骤 5: 输入 Model ID

```
? Model ID glm-4
```

**常见 Model ID**:
- 智谱: `glm-4`、`glm-4-turbo`
- OpenAI: `gpt-4`、`gpt-3.5-turbo`
- MiniMax: `abab6.5s-chat`

#### 步骤 6: 确认并保存

```
Configuration summary:
  Name: glm
  Base URL: https://open.bigmodel.cn/api/v1
  API Key: sk-***abc123
  Model ID: glm-4

? Save this configuration? Yes
```

### 1.3 完成配置

```
✓ Configuration saved successfully!

Next steps:
  - Run 'asw model list' to see all configured models
  - Run 'asw doctor' to detect installed tools
  - Run 'asw switch <tool> <model>' to apply a model to a tool
```

---

## 场景 2: 添加更多模型配置

### 2.1 使用向导添加

```bash
asw wizard
```

向导流程与首次配置相同。

### 2.2 查看已配置的模型

```bash
asw model list
```

**输出示例**:
```
Configured Models:
==================

glm
  Base URL: https://open.bigmodel.cn/api/v1
  API Key: sk-***abc123
  Model ID: glm-4

gpt-4
  Base URL: https://api.openai.com/v1
  API Key: sk-***xyz789
  Model ID: gpt-4
```

---

## 场景 3: 工具检测和诊断

### 3.1 检测已安装工具

```bash
asw detect
```

**输出示例**:
```
Installed tools:
  - Claude Code (v1.2.3)
  - Codex (v2.1.0)
  - Qwen CLI (v1.0.5)
```

### 3.2 运行完整诊断

```bash
asw doctor
```

**输出示例**:
```
AgentSwitch Tool Diagnostic Report
==================================

System Information:
  OS: Linux 6.8.0-101-generic
  Arch: x86_64
  Shell: /bin/zsh
  Git: git version 2.43.0

Tool Status:
-------------
✓ Claude Code    v1.2.3    Installed (Healthy)
  Config: ~/.claude/config.json
  Status: Configuration valid

✓ Codex          v2.1.0    Installed (Healthy)
  Config: ~/.codex/config.toml
  Status: Configuration valid

✗ Gemini CLI     -         Not Installed
  Suggestion: Install via 'npm install -g gemini-cli'

⚠ Qwen CLI       v1.0.5    Installed (Warning)
  Config: ~/.qwen/config.env
  Issue: API Key not set
  Suggestion: Run 'asw switch qwen-cli <model>' to configure

Summary:
  Installed: 3
  Healthy: 2
  Warnings: 1
  Errors: 0

Run 'asw doctor --fix' to attempt automatic repairs.
```

### 3.3 自动修复问题

```bash
asw doctor --fix
```

**可自动修复的问题**:
- 配置文件权限问题
- 缺失的配置文件（创建空模板）
- 格式错误的配置（尝试修复）

---

## 场景 4: Shell 自动补全

### 4.1 安装 Bash 补全

```bash
asw completion install bash
```

**输出**:
```
✓ Bash completion script installed to: ~/.local/share/bash-completion/completions/asw.bash
✓ Added source line to ~/.bashrc

Restart your shell or run: source ~/.bashrc
```

### 4.2 安装 Zsh 补全

```bash
asw completion install zsh
```

**输出**:
```
✓ Zsh completion script installed to: ~/.zsh/completion/_asw
✓ Added completion setup to ~/.zshrc

Restart your shell or run: source ~/.zshrc
```

### 4.3 安装 Fish 补全

```bash
asw completion install fish
```

**输出**:
```
✓ Fish completion script installed to: ~/.config/fish/completions/asw.fish

Completion will be available automatically in new Fish sessions.
```

### 4.4 使用补全

安装补全后，可以使用 Tab 键补全命令：

```bash
# 补全子命令
asw sw<TAB>  # 补全为 asw switch

# 补全工具名称
asw switch <TAB>  # 显示: claude-code, codex, gemini-cli

# 补全模型名称
asw switch claude-code <TAB>  # 显示: glm, gpt-4, minimax
```

### 4.5 卸载补全

```bash
asw completion uninstall bash
```

---

## 场景 5: 配置同步 (Git)

### 5.1 初始化 Git 仓库

```bash
asw sync init --encrypt
```

**输出**:
```
Initializing Git repository...

✓ Created .gitignore
✓ Initialized repository

Encryption settings:
? Enable encryption for API Keys? Yes
? Choose encryption method:
  > AES-GCM (password)
    git-crypt (requires git-crypt installation)

? Enter encryption password: ********
? Confirm password: ********

✓ Encryption configured (AES-GCM)
✓ Created initial commit

Next steps:
  - Add a remote: asw sync remote add <url>
  - Push to remote: asw sync push
```

### 5.2 添加远程仓库

```bash
asw sync remote add https://github.com/user/agentswitch-config.git
```

### 5.3 推送到远程

```bash
asw sync push
```

**输出**:
```
Preparing to push...

✓ Detected 2 modified files
✓ Encrypted 3 API Keys
✓ Created commit: feat: update models and presets

Pushing to origin/main...
✓ Push successful

Changes pushed:
  - models.toml (encrypted)
  - presets.toml
```

### 5.4 在另一台机器上拉取配置

```bash
# 克隆配置仓库
git clone https://github.com/user/agentswitch-config.git ~/.agentswitch

# 或使用拉取命令（如果已有配置）
asw sync pull
```

**输出**:
```
Pulling from origin/main...

✓ Fetched 2 new commits

Changes:
  - models.toml: Added new model "gpt-4"
  - presets.toml: Updated preset "production"

✓ Pull successful

Models added:
  - gpt-4 (OpenAI GPT-4)
```

### 5.5 查看同步状态

```bash
asw sync status
```

**输出**:
```
Sync Status:
============

Repository: ✓ Initialized (Git 2.43.0)
Remote:    origin → https://github.com/user/agentswitch-config.git
Branch:    main

Divergence:
  Your branch is ahead of 'origin/main' by 1 commit.
  (use "asw sync push" to publish)

Encryption:
  Method: AES-GCM (password)
  Status: ✓ Active
  Encrypted files: 1 (models.toml)

Last sync: 2026-03-10 10:30:00 UTC
```

### 5.6 处理冲突

当本地和远程配置有冲突时：

```bash
asw sync pull
```

**冲突输出**:
```
Pulling from origin/main...
✗ Conflict detected

Conflicting files:
  - models.toml

Resolution options:
  1. Keep local version (discard remote)
  2. Use remote version (discard local)
  3. Manual merge (open in editor)

? Choose resolution strategy: 2

✓ Applied remote version
✓ Resolved conflict

Run 'asw sync push' to push the merged result.
```

---

## 场景 6: 完整工作流程

### 6.1 新用户设置流程

```bash
# 1. 初始化配置
asw init

# 2. 检测已安装工具
asw doctor

# 3. 安装 Shell 补全（可选）
asw completion install zsh

# 4. 应用模型到工具
asw switch claude-code glm

# 5. 验证配置
asw status
```

### 6.2 多机器同步流程

**机器 A（首次设置）**:
```bash
# 1. 配置模型
asw init

# 2. 初始化 Git 同步
asw sync init --encrypt
asw sync remote add https://github.com/user/agentswitch-config.git

# 3. 推送到远程
asw sync push
```

**机器 B（拉取配置）**:
```bash
# 1. 克隆配置
git clone https://github.com/user/agentswitch-config.git ~/.agentswitch

# 2. 验证配置
asw model list
asw doctor

# 3. 使用配置
asw switch claude-code glm
```

### 6.3 日常更新流程

```bash
# 添加新模型
asw wizard

# 应用新模型到工具
asw switch codex gpt-4

# 同步到远程
asw sync push

# 在其他机器上拉取更新
asw sync pull
```

---

## 场景 7: 高级用法

### 7.1 JSON 输出（脚本使用）

```bash
# 获取工具列表为 JSON
asw detect --json

# 获取诊断报告为 JSON
asw doctor --json
```

### 7.2 非交互式模式（脚本）

```bash
# 设置环境变量
export ASW_NON_INTERACTIVE=1

# 运行命令（如果需要交互会失败）
asw init || echo "Init failed, need interactive mode"
```

### 7.3 自定义配置目录

```bash
# 使用自定义目录
export ASW_CONFIG_DIR=/custom/path/agentswitch

# 初始化配置
asw init
```

### 7.4 生成补全脚本到文件

```bash
# 生成补全脚本
asw completion generate bash > /usr/share/bash-completion/completions/asw
```

---

## 故障排除

### 问题 1: 向导无法启动

**错误信息**:
```
✗ Not running in an interactive terminal
```

**解决方案**:
- 确保在真实的终端中运行（不是脚本或管道）
- 如果在脚本中需要自动化，使用环境变量 `ASW_NON_INTERACTIVE=1`

### 问题 2: Git 未安装

**错误信息**:
```
✗ Git is not installed
```

**解决方案**:
```bash
# Ubuntu/Debian
sudo apt-get install git

# macOS
brew install git

# 验证安装
git --version
```

### 问题 3: 补全不生效

**原因**: Shell 配置文件未重新加载

**解决方案**:
```bash
# 重新加载配置
source ~/.bashrc      # Bash
source ~/.zshrc       # Zsh
source ~/.config/fish/config.fish  # Fish

# 或重启终端
```

### 问题 4: 加密密码忘记

**解决方案**:
```bash
# 重置加密配置
rm ~/.agentswitch/sync.toml
asw sync init --encrypt
```

### 问题 5: 工具检测失败

**错误信息**:
```
✗ Failed to detect tool: Permission denied
```

**解决方案**:
```bash
# 检查文件权限
ls -la ~/.codex/config.toml

# 修复权限
chmod 600 ~/.codex/config.toml
```

---

## 最佳实践

### 1. 定期同步配置

```bash
# 设置定时同步（可选）
# 添加到 crontab
0 */6 * * * /usr/local/bin/asw sync pull
```

### 2. 备份配置

```bash
# 导出配置
asw preset export-all --output backup.json

# 或使用 Git 历史
git log --oneline ~/.agentswitch/
```

### 3. 使用预设管理多环境

```bash
# 开发环境预设
asw preset create development --description "开发环境配置"

# 生产环境预设
asw preset create production --description "生产环境配置"

# 切换环境
asw preset apply development
```

### 4. 定期运行诊断

```bash
# 每周运行一次
asw doctor --fix
```

---

## 下一步

- 查看 [完整命令参考](./contracts/CLI.md)
- 阅读 [数据模型文档](./data-model.md)
- 了解 [项目架构](../../README.md)

---

## 获取帮助

```bash
# 查看命令帮助
asw --help
asw init --help

# 查看版本
asw --version

# 报告问题
https://github.com/Yu-Xiao-Sheng/agentswitch/issues
```

---

**文档完成日期**: 2026-03-10
**文档版本**: 1.0.0
