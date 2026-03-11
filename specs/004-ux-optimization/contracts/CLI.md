# CLI 命令接口契约: 用户体验优化与高级功能

**功能分支**: `004-ux-optimization`
**创建日期**: 2026-03-10
**状态**: 完成

## 概述

本文档定义 Spec 004 中新增的 CLI 命令接口，包括命令结构、参数、行为和输出格式。

---

## 1. 交互式配置向导命令

### 1.1 asw init

初始化配置（交互式向导）。

**命令结构**:
```bash
asw init [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--resume` | 恢复之前的向导进度 | flag | false |
| | `--reset` | 重新开始（清除进度） | flag | false |
| | `--non-interactive` | 尝试非交互式模式（实验性） | flag | false |

**行为**:
1. 检查是否存在配置文件
   - 如果已存在，提示用户是否重新配置
2. 检查是否为交互式终端
   - 如果不是，显示错误并退出（除非 `--non-interactive`）
3. 检查是否有未完成的向导状态
   - 如果有且未指定 `--reset`，询问是否恢复
4. 启动交互式向导，依次收集：
   - 模型配置名称
   - Base URL
   - API Key（掩码显示）
   - Model ID
5. 验证所有输入
6. 保存配置文件
7. 显示成功消息和后续操作建议

**退出码**:
- `0`: 成功
- `1`: 用户取消
- `2`: 验证失败
- `3`: 非交互式环境

**输出示例**:
```
Welcome to AgentSwitch configuration wizard!

This wizard will guide you through setting up your first model configuration.

Press Ctrl+C at any time to exit (progress will be saved).

? Model configuration name glm
? Base URL https://open.bigmodel.cn/api/v1
? API Key ****************************
? Model ID glm-4

Configuration summary:
  Name: glm
  Base URL: https://open.bigmodel.cn/api/v1
  API Key: sk-***abc123
  Model ID: glm-4

? Save this configuration? Yes

✓ Configuration saved successfully!

Next steps:
  - Run 'asw model list' to see all configured models
  - Run 'asw doctor' to detect installed tools
  - Run 'asw switch <tool> <model>' to apply a model to a tool
```

### 1.2 asw wizard

添加模型配置（交互式向导）。

**命令结构**:
```bash
asw wizard [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--name` `<NAME>` | 预设模型名称（跳过输入） | string | - |
| | `--resume` | 恢复之前的向导进度 | flag | false |

**行为**:
与 `asw init` 类似，但用于添加额外模型配置，不检查是否首次运行。

---

## 2. 工具检测和诊断命令

### 2.1 asw doctor

检测已安装工具并检查配置健康状态。

**命令结构**:
```bash
asw doctor [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| `-v` | `--verbose` | 显示详细信息 | flag | false |
| `-j` | `--json` | 以 JSON 格式输出 | flag | false |
| | `--fix` | 尝试自动修复问题 | flag | false |

**行为**:
1. 遍历所有已注册的工具适配器
2. 对每个工具执行：
   - 检查可执行文件是否存在
   - 获取工具版本
   - 查找配置文件位置
   - 读取并验证配置
3. 生成健康检查报告
4. 如果指定 `--fix`，尝试自动修复常见问题
5. 输出报告

**退出码**:
- `0`: 所有工具健康
- `1`: 有警告
- `2`: 有错误
- `3`: 检测失败

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

**JSON 输出格式**:
```json
{
  "system_info": {
    "os": "Linux",
    "arch": "x86_64",
    "shell": "/bin/zsh",
    "git_version": "2.43.0"
  },
  "tools": [
    {
      "name": "claude-code",
      "display_name": "Claude Code",
      "status": "installed",
      "healthy": true,
      "version": "1.2.3",
      "executable_path": "/usr/bin/claude",
      "config_path": "/home/user/.claude/config.json",
      "message": "Configuration valid"
    }
  ],
  "summary": {
    "installed": 3,
    "healthy": 2,
    "warnings": 1,
    "errors": 0
  }
}
```

### 2.2 asw detect

检测已安装工具（简化版，仅显示工具列表）。

**命令结构**:
```bash
asw detect [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| `-j` | `--json` | 以 JSON 格式输出 | flag | false |

**输出示例**:
```
Installed tools:
  - Claude Code (v1.2.3)
  - Codex (v2.1.0)
  - Qwen CLI (v1.0.5)
```

---

## 3. Shell 补全命令

### 3.1 asw completion install

安装 Shell 补全脚本。

**命令结构**:
```bash
asw completion install <SHELL> [OPTIONS]
```

**参数**:
| 位置 | 描述 | 类型 |
|------|------|------|
| `SHELL` | Shell 类型（bash/zsh/fish） | string |

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--path` `<PATH>` | 自定义安装路径 | path | 自动检测 |
| | `--no-modify-config` | 不修改 Shell 配置文件 | flag | false |

**行为**:
1. 检测 Shell 类型（如果未指定）
2. 生成补全脚本
3. 创建安装目录
4. 写入补全脚本
5. 修改 Shell 配置文件（除非 `--no-modify-config`）
6. 显示安装成功消息

**退出码**:
- `0`: 成功
- `1`: 不支持的 Shell
- `2`: 文件写入失败

**输出示例**:
```
✓ Bash completion script installed to: ~/.local/share/bash-completion/completions/asw.bash
✓ Added source line to ~/.bashrc

Restart your shell or run: source ~/.bashrc
```

### 3.2 asw completion uninstall

卸载 Shell 补全脚本。

**命令结构**:
```bash
asw completion uninstall <SHELL> [OPTIONS]
```

**参数**:
| 位置 | 描述 | 类型 |
|------|------|------|
| `SHELL` | Shell 类型（bash/zsh/fish） | string |

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--path` `<PATH>` | 自定义安装路径 | path | 自动检测 |

**输出示例**:
```
✓ Removed Bash completion script
✓ Removed source line from ~/.bashrc
```

### 3.3 asw completion generate

生成补全脚本到标准输出（用于管道）。

**命令结构**:
```bash
asw completion generate <SHELL>
```

**参数**:
| 位置 | 描述 | 类型 |
|------|------|------|
| `SHELL` | Shell 类型（bash/zsh/fish） | string |

**用法示例**:
```bash
# 生成并输出到文件
asw completion generate bash > /usr/share/bash-completion/completions/asw

# 直接 eval（Zsh）
eval "$(asw completion generate zsh)"
```

---

## 4. Git 同步命令

### 4.1 asw sync init

初始化 Git 仓库。

**命令结构**:
```bash
asw sync init [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--encrypt` | 启用加密 | flag | false |
| | `--encryption-method` `<METHOD>` | 加密方法（aes-gcm/git-crypt） | string | aes-gcm |
| | `--no-encrypt` | 禁用加密 | flag | false |

**行为**:
1. 检查 Git 是否安装
2. 检查是否已是 Git 仓库
   - 如果是，提示用户
3. 初始化 Git 仓库
4. 创建 .gitignore（排除敏感临时文件）
5. 如果启用加密：
   - 询问用户设置密码或使用 GPG 密钥
   - 配置加密
6. 创建初始提交
7. 保存同步配置

**退出码**:
- `0`: 成功
- `1`: Git 未安装
- `2`: 已经是 Git 仓库
- `3`: 加密配置失败

**输出示例**:
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

### 4.2 asw sync remote

管理远程仓库。

**命令结构**:
```bash
asw sync remote <SUBCOMMAND> [OPTIONS]
```

**子命令**:
| 子命令 | 描述 |
|--------|------|
| `add` `<URL>` | 添加远程仓库 |
| `remove` `<NAME>` | 删除远程仓库 |
| `list` | 列出远程仓库 |
| `set-url` `<NAME> <URL>` | 修改远程仓库 URL |

**示例**:
```bash
# 添加远程仓库
asw sync remote add https://github.com/user/agentswitch-config.git

# 列出远程仓库
asw sync remote list

# 修改 URL
asw sync remote set-url origin https://github.com/user/new-config.git
```

### 4.3 asw sync push

推送配置到远程仓库。

**命令结构**:
```bash
asw sync push [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--remote` `<NAME>` | 远程仓库名称 | string | origin |
| | `--branch` `<BRANCH>` | 分支名称 | string | main |
| | `--no-encrypt` | 跳过加密（不推荐） | flag | false |

**行为**:
1. 检查是否是 Git 仓库
2. 检查远程仓库是否配置
3. 检查是否有未提交的更改
   - 如果有，自动创建提交
4. 加密敏感字段（除非 `--no-encrypt`）
5. 推送到远程
6. 显示结果

**退出码**:
- `0`: 成功
- `1`: 推送失败
- `2`: 网络错误
- `3`: 加密失败

**输出示例**:
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

### 4.4 asw sync pull

从远程仓库拉取配置。

**命令结构**:
```bash
asw sync pull [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| | `--remote` `<NAME>` | 远程仓库名称 | string | origin |
| | `--branch` `<BRANCH>` | 分支名称 | string | main |
| | `--strategy` `<STRATEGY>` | 合并策略（merge/rebase） | string | merge |

**行为**:
1. 检查是否是 Git 仓库
2. 获取远程更改
3. 检查是否有冲突
   - 如果有冲突，提供解决选项
4. 拉取并合并
5. 解密敏感字段
6. 显示结果

**退出码**:
- `0`: 成功
- `1`: 拉取失败
- `2`: 有冲突未解决
- `3`: 解密失败

**输出示例**:
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

### 4.5 asw sync status

显示同步状态。

**命令结构**:
```bash
asw sync status [OPTIONS]
```

**参数**:
- 无位置参数

**选项**:
| 短选项 | 长选项 | 描述 | 类型 | 默认值 |
|--------|--------|------|------|--------|
| `-v` | `--verbose` | 显示详细信息 | flag | false |

**输出示例**:
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

---

## 5. 命令层次结构

### 5.1 完整命令树

```
asw
├── init                    # 初始化配置（向导）
├── wizard                  # 添加模型配置（向导）
├── doctor                  # 工具诊断
├── detect                  # 检测工具
├── completion              # Shell 补全
│   ├── install <SHELL>     # 安装补全
│   ├── uninstall <SHELL>   # 卸载补全
│   └── generate <SHELL>    # 生成补全脚本
├── sync                    # Git 同步
│   ├── init                # 初始化仓库
│   ├── remote              # 管理远程仓库
│   │   ├── add <URL>
│   │   ├── remove <NAME>
│   │   ├── list
│   │   └── set-url <NAME> <URL>
│   ├── push                # 推送到远程
│   ├── pull                # 从远程拉取
│   └── status              # 同步状态
├── model                   # 模型配置（现有）
│   ├── add
│   ├── list
│   ├── remove
│   └── edit
├── agent                   # 工具管理（现有）
├── switch                  # 切换配置（现有）
├── preset                  # 预设管理（现有）
└── backup                  # 备份管理（现有）
```

### 5.2 命令分类

**新增命令**（Spec 004）:
- `init` - 交互式配置向导
- `wizard` - 添加模型向导
- `doctor` - 工具诊断
- `detect` - 工具检测
- `completion` - Shell 补全
- `sync` - Git 同步

**现有命令**（保持兼容）:
- `model` - 模型配置管理
- `agent` - 工具管理
- `switch` - 配置切换
- `preset` - 预设管理
- `backup` - 备份管理

---

## 6. 补全脚本行为

### 6.1 静态补全

由 `clap_complete` 自动生成，包括：

**命令补全**:
```bash
asw <TAB>
# 显示: init, wizard, doctor, detect, completion, sync, model, agent, switch, preset, backup
```

**子命令补全**:
```bash
asw model <TAB>
# 显示: add, list, remove, edit
```

**选项补全**:
```bash
asw doctor <TAB>
# 显示: --verbose, --json, --fix
```

### 6.2 动态补全

自定义脚本实现：

**模型名称补全**:
```bash
asw switch claude-code <TAB>
# 显示: glm, gpt-4, minimax（从配置文件读取）
```

**工具名称补全**:
```bash
asw switch <TAB>
# 显示: claude-code, codex, gemini-cli, qwen-cli（从检测结果读取）
```

**预设名称补全**:
```bash
asw preset apply <TAB>
# 显示: development, production（从预设文件读取）
```

**远程仓库名称补全**:
```bash
asw sync remote remove <TAB>
# 显示: origin, upstream（从 Git 配置读取）
```

---

## 7. 输出格式规范

### 7.1 成功消息

格式：
```
✓ [操作描述]
```

示例：
```
✓ Configuration saved successfully
✓ Bash completion script installed
```

### 7.2 错误消息

格式：
```
✗ [错误描述]

Details: [详细信息]
Suggestion: [修复建议]
```

示例：
```
✗ Failed to detect tool

Details: Cannot execute 'codex': No such file or directory
Suggestion: Install Codex via 'npm install -g @openai/codex-cli'
```

### 7.3 警告消息

格式：
```
⚠ [警告描述]
```

示例：
```
⚠ API Key will be stored unencrypted
Consider running 'asw sync init --encrypt' to enable encryption
```

### 7.4 信息提示

格式：
```
ℹ [信息内容]
```

示例：
```
ℹ No tools detected. Run 'asw doctor' after installing tools.
```

### 7.5 进度指示

格式：
```
[操作描述]...
  ✓ [步骤 1]
  ✓ [步骤 2]
  ⠠ [步骤 3] (进行中)
```

示例：
```
Pushing to remote...
  ✓ Encrypted sensitive data
  ✓ Created commit
  ⠠ Uploading... (45%)
```

---

## 8. 交互式提示规范

### 8.1 文本输入

```
? [提示文本] [默认值]
```

示例：
```
? Model configuration name glm
```

### 8.2 密码输入

```
? [提示文本]
[输入显示为 ******]
```

示例：
```
? API Key ****************************
```

### 8.3 确认提示

```
? [提示文本] (Y/n)
```

示例：
```
? Save this configuration? Yes
```

### 8.4 选择提示

```
? [提示文本]
> [选项 1]
  [选项 2]
  [选项 3]
```

示例：
```
? Choose encryption method
  > AES-GCM (password)
    git-crypt (requires git-crypt installation)
    None (not recommended)
```

### 8.5 多选提示

```
? [提示文本] (Space to select, Enter to confirm)
>[x] [选项 1]
 [ ] [选项 2]
[x] [选项 3]
```

示例：
```
? Select tools to configure
>[x] Claude Code
 [ ] Codex
[x] Gemini CLI
```

---

## 9. 退出码规范

| 代码 | 含义 | 使用场景 |
|------|------|----------|
| 0 | 成功 | 操作成功完成 |
| 1 | 一般错误 | 用户取消、验证失败、操作失败 |
| 2 | 依赖错误 | Git 未安装、不是 Git 仓库 |
| 3 | 环境错误 | 非交互式环境、不支持的操作 |
| 4 | 网络错误 | 远程仓库不可访问 |
| 5 | 冲突错误 | 合并冲突、配置冲突 |
| 64 | 使用错误 | 无效参数、命令使用错误 |
| 65 | 数据错误 | 配置文件损坏、数据格式错误 |
| 70 | 软件内部错误 | 未预期的错误、panic |

---

## 10. 环境变量

### 10.1 支持的环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `ASW_CONFIG_DIR` | 配置目录路径 | `~/.agentswitch` |
| `ASW_CACHE_DIR` | 缓存目录路径 | `~/.cache/agentswitch` |
| `ASW_NO_COLOR` | 禁用彩色输出 | - |
| `ASW_VERBOSE` | 详细输出模式 | - |
| `ASW_NON_INTERACTIVE` | 非交互式模式 | - |

### 10.2 使用示例

```bash
# 自定义配置目录
export ASW_CONFIG_DIR=/custom/path/agentswitch
asw model list

# 禁用彩色输出
export ASW_NO_COLOR=1
asw doctor

# 非交互式模式（脚本中）
export ASW_NON_INTERACTIVE=1
asw init || echo "Init failed"
```

---

## 11. 配置文件

### 11.1 sync.toml

Git 同步配置文件。

**位置**: `~/.agentswitch/sync.toml`

**格式**:
```toml
[remote]
url = "https://github.com/user/agentswitch-config.git"
name = "origin"
branch = "main"

[encryption]
enabled = true
method = "aes-gcm-password"
# password_hash 和 salt 存储在系统密钥链中

[user]
name = "AgentSwitch User"
email = "user@example.com"

[behavior]
auto_encrypt = true
auto_push = false
```

---

## 12. 总结

### 新增命令汇总

| 命令 | 分类 | 优先级 |
|------|------|--------|
| `init` | 向导 | P1 |
| `wizard` | 向导 | P1 |
| `doctor` | 诊断 | P1 |
| `detect` | 诊断 | P1 |
| `completion install` | 补全 | P2 |
| `completion uninstall` | 补全 | P2 |
| `completion generate` | 补全 | P2 |
| `sync init` | 同步 | P2 |
| `sync remote` | 同步 | P2 |
| `sync push` | 同步 | P2 |
| `sync pull` | 同步 | P2 |
| `sync status` | 同步 | P2 |

### 关键设计决策

1. **向后兼容**: 所有新命令不影响现有命令
2. **渐进式增强**: 向导命令是可选的，现有命令仍然可用
3. **安全性优先**: API Key 默认加密存储
4. **用户友好**: 清晰的输出格式和错误提示
5. **自动化友好**: 支持 JSON 输出和非交互式模式

---

**文档完成日期**: 2026-03-10
**状态**: ✅ CLI 命令接口定义完成
