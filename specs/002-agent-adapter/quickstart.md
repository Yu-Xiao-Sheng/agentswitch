# 快速开始指南: AgentSwitch Agent 工具适配器系统

**功能分支**: `002-agent-adapter` | **日期**: 2026-02-28

## 前提条件

在使用 AgentSwitch v0.2.0 的 Agent 工具适配功能之前，请确保：

1. ✅ 已完成 v0.1.0 核心：配置存储和 CLI 框架
2. ✅ 系统上已安装至少一个 Code Agent 工具
3. ✅ 具有相应工具的配置文件读写权限
4. ✅ 已添加至少一个模型配置

## 快速开始

### 步骤 1: 检测已安装的工具

```bash
# 检测系统上已安装的 Code Agent 工具
asw agent detect
```

**预期输出**:
```
Agent Detection Results:
=======================

Claude Code        ✓ 已安装    ~/.claude/settings.json
Codex              ✗ 未安装    需要运行: npm install -g @openai/codex@0.80.0
Gemini CLI         ✓ 已安装    ~/.gemini/settings.json
```

### 步骤 2: 添加模型配置

如果尚未添加模型配置，请先添加：

```bash
# 添加 GLM 模型配置
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-..." \
  --model "glm-4"

# 添加 MiniMax 模型配置
asw model add minimax \
  --base-url "https://api.minimax.chat/v1" \
  --api-key "your-api-key" \
  --model "abab6.5s-chat"
```

### 步骤 3: 切换工具模型配置

```bash
# 将 Claude Code 切换到 GLM 模型
asw switch claude-code glm

# 将 Gemini CLI 切换到 MiniMax 模型
asw switch gemini-cli minimax
```

**预期输出**:
```
正在备份原配置...
已备份到: /home/user/.agentswitch/backups/claude-code-20260228-143022.config.json.bak

正在应用配置...
✓ Claude Code 已切换到 glm 模型

提示: 使用 'claude-code' 命令测试新配置
```

### 步骤 4: 查看当前配置状态

```bash
asw status
```

**预期输出**:
```
Agent Configuration Status:
==========================

Agent            Model         Config Path                              Last Switched
────────────────────────────────────────────────────────────────────────────────
Claude Code       glm           /home/user/.claude/settings.json      2026-02-28 14:30
Codex            -              /home/user/.codex/config.toml          -
Gemini CLI        minimax       /home/user/.gemini/settings.json      2026-02-28 13:45
```

---

## 常见使用场景

### 场景 1: 配置 Claude Code 使用国内 API

```bash
# 1. 添加 GLM API 配置
asw model add glm \
  --base-url "https://open.bigmodel.cn/api/v1" \
  --api-key "sk-your-glm-key" \
  --model "glm-4"

# 2. 切换 Claude Code 到 GLM
asw switch claude-code glm

# 3. 验证配置
cat ~/.claude/settings.json
```

### 场景 2: 配置 Codex 使用自定义 API

```bash
# 1. 添加自定义 API 配置
asw model add custom-api \
  --base-url "https://your-api.example.com/v1" \
  --api-key "sk-your-key" \
  --model "gpt-5-codex"

# 2. 切换 Codex
asw switch codex custom-api

# 3. 验证配置
cat ~/.codex/config.toml
cat ~/.codex/auth.json
```

### 场景 3: 恢复错误的配置

```bash
# 1. 查看可用备份
asw backup list

# 2. 恢复到之前的配置
asw backup restore claude-code --backup 20260227-101533

# 3. 验证恢复
asw status
```

### 场景 4: 清理旧备份释放空间

```bash
# 清理 7 天前的备份
asw backup clean --older-than 7d

# 清理 1 周前的备份
asw backup clean --older-than 1w
```

---

## 故障排查

### 问题 1: 工具未检测到

**症状**: `asw agent detect` 显示工具未安装

**解决方案**:
```bash
# Claude Code
npm install -g @anthropic-ai/claude-code

# Codex (注意：需要 v0.80.0)
npm install -g @openai/codex@0.80.0

# Gemini CLI
npm install -g @google/gemini-cli
```

### 问题 2: 配置文件权限不足

**症状**: 切换时报错 "配置文件只读"

**解决方案**:
```bash
# 修改配置文件权限
chmod 644 ~/.claude/settings.json
chmod 644 ~/.codex/config.toml
chmod 644 ~/.gemini/settings.json
```

### 问题 3: 环境变量覆盖配置

**症状**: 切换成功但工具仍使用旧配置

**解决方案**:
```bash
# 查看当前环境变量
env | grep -E "(ANTHROPIC_|GEMINI_|OPENAI_)"

# 取消临时设置环境变量
unset ANTHROPIC_BASE_URL
unset GEMINI_API_KEY

# 重启终端使配置生效
```

### 问题 4: 备份文件损坏

**症状**: 恢复备份时报错

**解决方案**:
```bash
# 查看所有备份
asw backup list

# 使用更早的备份
asw backup restore claude-code --backup 20260227-101533
```

---

## 配置文件参考

### Claude Code 配置示例

**文件**: `~/.claude/settings.json`

```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "sk-ant-api03-...",
    "ANTHROPIC_BASE_URL": "https://open.bigmodel.cn/api/v1",
    "ANTHROPIC_MODEL": "glm-4"
  },
  "includeCoAuthoredBy": true
}
```

### Codex 配置示例

**文件**: `~/.codex/config.toml`

```toml
model_provider = "custom_provider"
model = "glm-4"
preferred_auth_method = "apikey"

[model_providers.custom_provider]
name = "GLM API"
base_url = "https://open.bigmodel.cn/api/v1"
wire_api = "responses"
```

**文件**: `~/.codex/auth.json`

```json
{
  "OPENAI_API_KEY": "sk-..."
}
```

### Gemini CLI 配置示例

**文件**: `~/.gemini/settings.json`

```json
{
  "defaultModel": "glm-4"
}
```

**文件**: `~/.gemini/.env`

```bash
GOOGLE_GEMINI_BASE_URL=https://open.bigmodel.cn/api/v1
GEMINI_API_KEY=sk-...
GEMINI_MODEL=glm-4
```

---

## 高级用法

### 批量切换多个工具

```bash
# 切换所有已安装工具到同一模型
asw switch claude-code glm && \
asw switch gemini-cli glm && \
asw switch codex glm
```

### 查看备份历史

```bash
# 查看所有备份
asw backup list

# 统计备份数量
asw backup list | grep "Total:"
```

### 自动清理旧备份

```bash
# 添加到 crontab，每周清理
0 0 * * 0 asw backup clean --older-than 7d
```

---

## 下一步

- 阅读完整文档: `spec.md`
- 查看技术研究: `research.md`
- 了解数据模型: `data-model.md`
- 查看命令契约: `contracts/cli-commands.md`

## 获取帮助

```bash
# 查看帮助信息
asw --help

# 查看具体命令帮助
asw switch --help
asw backup --help
```

## 技术支持

如有问题或建议，请：
- 提交 Issue: [GitHub Issues](https://github.com/Yu-Xiao-Sheng/agentswitch/issues)
- 查看文档: [项目 README](https://github.com/Yu-Xiao-Sheng/agentswitch)
