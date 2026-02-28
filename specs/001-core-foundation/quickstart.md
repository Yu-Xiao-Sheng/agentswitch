# 快速开始指南: AgentSwitch 核心基础功能

**功能**: 核心基础功能
**日期**: 2026-02-27
**目标用户**: 开发者
**预计时间**: 10 分钟

## 概述

本指南将帮助您快速上手 AgentSwitch CLI 工具的核心基础功能。通过本指南，您将学会：
- 自动初始化配置
- 添加模型配置
- 查看已配置的模型
- 编辑和删除模型配置

## 前提条件

- Rust 2024 Edition（最新稳定版）
- 类 Unix 操作系统（Linux、macOS）

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/Yu-Xiao-Sheng/agentswitch.git
cd agentswitch

# 编译安装
cargo install --path .

# 验证安装
asw --version
```

### 使用预编译二进制（未来支持）

```bash
# 下载最新版本
curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/releases/latest/download/asw-x86_64-unknown-linux-gnu -o asw
chmod +x asw
mv asw ~/.local/bin/
```

## 首次使用

### 自动初始化

**重要**: AgentSwitch 会在首次运行任何命令时自动初始化配置，无需手动执行 `init` 命令。

```bash
# 首次运行任何命令，自动创建配置
$ asw model list

# ⚠️  首次使用，正在自动创建配置...
# ✓ 配置目录已创建: /home/user/.agentswitch/
# ✓ 配置文件已创建: /home/user/.agentswitch/config.toml
# ✓ 文件权限已设置: 0600（仅所有者可读写）
#
# 当前没有配置任何模型
# 💡 提示: 使用 'asw model add <name>' 添加模型配置
```

### 配置文件结构

自动创建的配置文件位于 `~/.agentswitch/config.toml`：

```toml
# AgentSwitch 配置文件
# 此文件存储所有模型配置

# 模型配置列表
[[models]]
#name = "example"
#base_url = "https://api.example.com/v1"
#api_key = "your-api-key"
#model_id = "model-name"

# 活跃模型映射
[active_models]
# 格式: <agent-name> = <model-name>
```

## 核心功能

### 1. 添加模型配置

添加 GLM 模型配置：

```bash
$ asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-abc123def456..." \
  --model "glm-4"

✓ 模型配置已添加: glm
```

添加 MiniMax 模型配置：

```bash
$ asw model add minimax \
  --base-url "https://api.minimax.chat/v1" \
  --api-key "sk-xyz789..." \
  --model "abab6.5s-chat"

✓ 模型配置已添加: minimax
```

**参数说明**:

| 参数 | 必填 | 描述 | 示例 |
|------|------|------|------|
| `<name>` | ✅ | 模型名称（唯一标识符） | `glm` |
| `--base-url` | ✅ | API 基础地址 | `https://open.bigmodel.cn/api/v1` |
| `--api-key` | ✅ | API 认证密钥 | `sk-abc123...` |
| `--model` | ✅ | 模型标识符 | `glm-4` |

**错误处理**:

```bash
# 添加同名模型
$ asw model add glm --base-url "..." --api-key "..." --model "..."
✗ 错误: 模型名称 'glm' 已存在

  建议: 使用不同的名称或删除现有配置后重试
```

---

### 2. 列出模型配置

查看所有已配置的模型：

```bash
$ asw model list

┌──────────┬─────────────────────────────────┬─────────────┬─────────┐
│ Name     │ Base URL                        │ Model ID    │ API Key │
├──────────┼─────────────────────────────────┼─────────────┼─────────┤
│ glm      │ https://open.bigmodel.cn/api/v1 │ glm-4       │ sk12**** │
│ minimax  │ https://api.minimax.chat/v1     │ abab6.5s... │ sk78**** │
└──────────┴─────────────────────────────────┴─────────────┴─────────┘
```

**空状态提示**：

```bash
$ asw model list

💡 当前没有配置任何模型
  提示: 使用 'asw model add <name>' 添加模型配置
```

---

### 3. 编辑模型配置

更新 GLM 的 API Key：

```bash
$ asw model edit glm --api-key "sk-new-key-456..."

✓ 模型配置已更新: glm
```

**可选参数**（可以组合使用）：

```bash
# 更新多个字段
$ asw model edit glm \
  --base-url "https://new-url.com/api/v1" \
  --api-key "sk-new-key..." \
  --model "glm-4-turbo"

✓ 模型配置已更新: glm
```

**参数说明**:

| 参数 | 描述 |
|------|------|
| `--base-url` | 更新 API 基础地址 |
| `--api-key` | 更新 API Key |
| `--model` | 更新模型标识符 |

**错误处理**：

```bash
# 编辑不存在的模型
$ asw model edit unknown --api-key "..."
✗ 错误: 模型 'unknown' 不存在

  建议: 使用 'asw model list' 查看已配置的模型
```

---

### 4. 删除模型配置

删除指定的模型配置：

```bash
$ asw model remove minimax

✓ 模型配置已删除: minimax
```

**错误处理**：

```bash
# 删除不存在的模型
$ asw model remove unknown
✗ 错误: 模型 'unknown' 不存在

  建议: 使用 'asw model list' 查看已配置的模型
```

---

## 输出格式说明

### 颜色标识

AgentSwitch 使用颜色区分不同类型的信息：

| 颜色 | 符号 | 含义 | 示例 |
|------|------|------|------|
| 绿色 | ✓ | 操作成功 | `✓ 模型配置已添加: glm` |
| 红色 | ✗ | 操作失败 | `✗ 错误: 模型名称已存在` |
| 黄色 | 💡 | 警告/提示 | `💡 当前没有配置任何模型` |
| 蓝色 | ℹ | 信息提示 | `ℹ 使用 'asw model add' 添加配置` |

### API Key 掩码

为保护敏感信息，API Key 在输出时会自动掩码：

```bash
$ asw model list

┌──────────┬──────────────┐
│ Name     │ API Key      │
├──────────┼──────────────┤
│ glm      │ sk12****     │  ← 仅显示前 4 位
└──────────┴──────────────┘
```

---

## 常见问题

### Q1: 如何查看配置文件位置？

```bash
# 配置文件固定位置
~/.agentswitch/config.toml
```

### Q2: 如何手动编辑配置文件？

```bash
# 使用文本编辑器打开
vim ~/.agentswitch/config.toml

# 或
nano ~/.agentswitch/config.toml
```

**注意**: 手动编辑后，确保 TOML 格式正确，否则会解析失败。

### Q3: 如何备份配置？

```bash
# 复制配置文件
cp ~/.agentswitch/config.toml ~/.agentswitch/config.toml.backup

# 或使用日期
cp ~/.agentswitch/config.toml ~/.agentswitch/config.toml.$(date +%Y%m%d)
```

### Q4: 配置文件损坏怎么办？

```bash
# 删除损坏的配置文件
rm ~/.agentswitch/config.toml

# 下次运行任何命令时会自动重新创建默认配置
$ asw model list
```

### Q5: 如何完全重置配置？

```bash
# 删除整个配置目录
rm -rf ~/.agentswitch/

# 下次运行会自动初始化
$ asw model list
```

---

## 安全最佳实践

### 1. 文件权限

配置文件自动设置为 `0600`（仅所有者可读写）：

```bash
$ ls -l ~/.agentswitch/config.toml
-rw------- 1 user user 512 Feb 27 10:30 config.toml
```

### 2. API Key 保护

- ✅ API Key 存储在本地配置文件中
- ✅ 文件权限限制为仅所有者访问
- ✅ 命令行输出时自动掩码
- ❌ 不要在版本控制中提交配置文件
- ❌ 不要在公共场合分享配置文件

### 3. 配置文件备份

建议定期备份配置文件到安全位置：

```bash
# 备份到加密的存储位置
cp ~/.agentswitch/config.toml /path/to/secure/backup/
```

---

## 下一步

完成核心基础功能学习后，您可以：

1. **探索高级功能**（Phase 2）:
   - Agent 工具适配器（Claude Code、Codex 等）
   - 配置切换功能
   - 配置预设（Presets）

2. **参与开发**:
   - 报告 Bug
   - 提交功能建议
   - 贡献代码

3. **查看文档**:
   - [完整命令参考](../../README.md#命令参考)
   - [架构设计文档](plan.md)
   - [数据模型文档](data-model.md)

---

## 获取帮助

### 查看命令帮助

```bash
# 查看主帮助
$ asw --help

# 查看子命令帮助
$ asw model --help
$ asw model add --help
```

### 联系方式

- GitHub Issues: [提交问题](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)
- GitHub Discussions: [参与讨论](https://github.com/Yu-Xiao-Sheng/agentswitch/discussions)

---

**快速开始指南状态**: ✅ 完成
**最后更新**: 2026-02-27
