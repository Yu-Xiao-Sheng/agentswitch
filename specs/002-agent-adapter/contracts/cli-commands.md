# CLI 命令契约: AgentSwitch Agent 工具适配器系统

**功能分支**: `002-agent-adapter` | **日期**: 2026-02-28

## 命令结构

```bash
asw
├── model              # 模型配置管理 (v0.1.0 已实现)
│   ├── add
│   ├── list
│   ├── remove
│   └── edit
├── agent              # Agent 工具管理 (新增 v0.2.0)
│   ├── detect         # 检测已安装的工具
│   └── list           # 列出所有支持的工具
├── switch             # 模型配置切换 (新增 v0.2.0)
│   └── <agent> <model>
├── status             # 显示当前配置状态 (新增 v0.2.0)
├── backup             # 配置备份管理 (新增 v0.2.0)
│   ├── list           # 列出所有备份
│   ├── restore        # 恢复指定备份
│   └── clean          # 清理旧备份
└── help               # 帮助信息
```

---

## 命令详细规格

### 1. `asw agent detect` - 检测已安装的工具

**功能**: 检测系统上已安装的 Code Agent 工具

**用法**:
```bash
asw agent detect
```

**输出格式**:
```
Agent Detection Results:
=======================

Claude Code        ✓ 已安装    ~/.claude/settings.json
Codex              ✗ 未安装    需要运行: npm install -g @openai/codex@0.80.0
Gemini CLI         ✓ 已安装    ~/.gemini/settings.json

提示: 使用 'asw switch <agent> <model>' 切换工具的模型配置
```

**退出码**:
- `0`: 所有检测完成
- `1`: 检测过程中发生错误

---

### 2. `asw switch <agent> <model>` - 切换工具模型配置

**功能**: 将指定的 Agent 工具切换到指定的模型配置

**用法**:
```bash
asw switch claude-code glm
asw switch codex minimax
asw switch gemini-cli deepseek
```

**参数**:
- `<agent>`: 工具名称（claude-code, codex, gemini-cli）
- `<model>`: 模型配置名称（必须已通过 `asw model add` 添加）

**执行流程**:
1. 验证模型配置是否存在
2. 检测工具是否已安装
3. 创建配置文件备份
4. 应用新配置到工具的配置文件
5. 更新 `active_models` 映射
6. 显示成功信息

**输出示例**:
```
正在备份原配置...
已备份到: /home/user/.agentswitch/backups/claude-code-20260228-143022.config.json.bak

正在应用配置...
✓ Claude Code 已切换到 glm 模型

提示: 使用 'claude-code' 命令测试新配置
```

**错误场景**:
```bash
# 模型配置不存在
$ asw switch claude-code unknown-model
✗ 错误: 模型配置 'unknown-model' 不存在
建议: 使用 'asw model list' 查看可用模型

# 工具未安装
$ asw switch codex glm
✗ 错误: 未检测到 Codex 安装
建议: 运行 'npm install -g @openai/codex@0.80.0' 安装工具

# 环境变量覆盖警告
$ asw switch claude-code glm
✓ Claude Code 已切换到 glm 模型
⚠ 警告: 检测到环境变量 ANTHROPIC_BASE_URL 可能覆盖配置文件设置
建议: 取消环境变量或重启终端
```

---

### 3. `asw status` - 显示当前配置状态

**功能**: 显示所有 Agent 工具的当前配置状态

**用法**:
```bash
asw status
```

**输出格式**:
```
Agent Configuration Status:
==========================

Agent            Model         Config Path                              Last Switched
────────────────────────────────────────────────────────────────────────────────
Claude Code       glm           /home/user/.claude/settings.json      2026-02-28 14:30
Codex            -              /home/user/.codex/config.toml          -
Gemini CLI        minimax       /home/user/.gemini/settings.json      2026-02-27 09:15

Legend:
  ✓ = 已配置  ✗ = 未配置  ⚠ = 配置文件不存在

提示: 使用 'asw switch <agent> <model>' 配置工具
```

---

### 4. `asw backup list` - 列出所有备份

**功能**: 列出所有工具的配置备份

**用法**:
```bash
asw backup list
```

**输出格式**:
```
Backup List:
============

Claude Code:
  2026-02-28 14:30:22  /home/user/.agentswitch/backups/claude-code-20260228-143022.config.json.bak
  2026-02-27 10:15:33  /home/user/.agentswitch/backups/claude-code-20260227-101533.config.json.bak

Codex:
  (无备份)

Gemini CLI:
  2026-02-28 13:45:10  /home/user/.agentswitch/backups/gemini-cli-20260228-134510.config.json.bak

Total: 3 backups, 0.5 MB
```

---

### 5. `asw backup restore <agent> --backup <timestamp>` - 恢复备份

**功能**: 从备份恢复工具的配置文件

**用法**:
```bash
asw backup restore claude-code --backup 20260228-143022
```

**参数**:
- `<agent>`: 工具名称
- `--backup <timestamp>`: 备份时间戳（YYYYMMDD-HHMMSS）

**执行流程**:
1. 验证备份文件是否存在
2. 验证原配置文件是否存在
3. 将备份内容复制到原配置文件
4. 显示成功信息

**输出示例**:
```
正在恢复配置...
✓ 配置已恢复到 2026-02-28 14:30:22 的版本

提示: 重启相关工具以使配置生效
```

---

### 6. `asw backup clean --older-than <duration>` - 清理旧备份

**功能**: 清理指定时间之前的备份

**用法**:
```bash
asw backup clean --older-than 7d
asw backup clean --older-than 1w
asw backup clean --older-than 1m
```

**参数**:
- `--older-than <duration>`: 时间间隔
  - `d`: 天
  - `w`: 周
  - `m`: 月

**输出示例**:
```
正在清理旧备份...
✓ 已清理 5 个旧备份文件
释放空间: 0.2 MB
```

---

## 环境变量支持

### 可配置的环境变量

| 环境变量 | 说明 | 默认值 |
|---------|------|--------|
| `AGENTSWITCH_CONFIG_DIR` | 配置文件目录 | `~/.agentswitch/` |
| `AGENTSWITCH_BACKUP_DIR` | 备份文件目录 | `~/.agentswitch/backups/` |
| `AGENTSWITCH_MAX_BACKUPS` | 每个工具最大备份数 | `10` |

### 使用示例

```bash
# 自定义配置目录
export AGENTSWITCH_CONFIG_DIR=/custom/path/agentswitch
asw model list

# 自定义备份保留数量
export AGENTSWITCH_MAX_BACKUPS=20
asw switch claude-code glm
```

---

## 错误处理规范

### 错误消息格式

**成功消息**:
```
✓ [操作结果]
```

**错误消息**:
```
✗ 错误: [具体原因]

  详细信息: [上下文]
  建议: [解决方案]

  位置: [相关路径]
```

**警告消息**:
```
⚠ 警告: [警告内容]
```

**信息提示**:
```
ℹ 提示: [信息性提示]
```

### 常见错误场景

| 错误场景 | 错误消息 | 解决建议 |
|---------|---------|---------|
| 模型配置不存在 | `✗ 错误: 模型配置 'xxx' 不存在` | 使用 `asw model list` 查看可用模型 |
| 工具未安装 | `✗ 错误: 未检测到 Claude Code 安装` | 查看官方安装文档 |
| 配置文件权限不足 | `✗ 错误: 配置文件只读，无法修改` | 运行 `chmod 644 <文件路径>` 修改权限 |
| 备份文件损坏 | `✗ 错误: 无法读取备份文件，文件已损坏` | 使用 `asw backup list` 查看其他可用备份 |
| 磁盘空间不足 | `✗ 错误: 磁盘空间不足，无法创建备份` | 运行 `asw backup clean` 清理旧备份或清理磁盘空间 |

---

## 退出码规范

| 退出码 | 含义 | 使用场景 |
|-------|------|---------|
| 0 | 成功 | 所有命令正常执行完毕 |
| 1 | 一般错误 | 参数错误、文件操作失败等 |
| 2 | 用户输入错误 | 模型名称不存在、工具名称错误等 |
| 3 | 系统错误 | 权限不足、磁盘空间不足等 |
| 4 | 网络错误 | API 调用失败（未来功能） |

---

## 配置文件格式

### `~/.agentswitch/config.toml` 结构

```toml
[models]
  [[models.items]]
  name = "glm"
  base_url = "https://open.bigmodel.cn/api/v1"
  api_key = "sk-..."
  model_id = "glm-4"

  [[models.items]]
  name = "minimax"
  base_url = "https://api.minimax.chat/v1"
  api_key = "your-api-key"
  model_id = "abab6.5s-chat"

[active_models]
  claude-code = "glm"
  gemini-cli = "minimax"

[backup]
  max_per_agent = 10
  auto_cleanup = true
```

---

## 版本兼容性

### v0.1.0 → v0.2.0 升级

**配置兼容性**: v0.2.0 完全兼容 v0.1.0 的配置文件

**新增字段**:
- `[active_models]`: 记录每个工具当前使用的模型
- `[backup]`: 备份配置

**迁移策略**: 自动迁移，无需用户干预

---

## 测试契约

### 单元测试契约

每个命令必须包含以下测试：
1. 正常执行路径测试
2. 参数验证测试
3. 错误处理测试
4. 边界条件测试

### 集成测试契约

必须测试以下完整流程：
1. 添加模型 → 检测工具 → 切换配置 → 验证状态
2. 切换配置 → 列出备份 → 恢复备份 → 验证恢复
3. 清理备份 → 验证清理结果
