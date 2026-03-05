# CLI 命令契约: 配置预设与批量管理

**功能**: Spec 003 - 配置预设与批量管理
**创建日期**: 2026-03-05
**版本**: 1.0.0

## 概述

本文档定义 AgentSwitch 配置预设与批量管理功能的 CLI 命令接口。

## 命令结构

```
asw preset <subcommand>
asw batch <subcommand>
```

---

## 1. 预设管理命令（asw preset）

### 1.1 创建预设

**命令**:
```bash
asw preset create <NAME> [OPTIONS]
```

**参数**:
- `<NAME>`: 预设名称（必需）
  - 格式: 1-64 字符，仅含字母、数字、连字符、下划线
  - 示例: `development`, `production`, `test-env`

**选项**:
- `--description <TEXT>`: 预设描述（可选）
  - 最大长度: 512 字符
  - 示例: `--description "开发环境配置"`

- `--tag <TAG>`: 预设标签，可多次使用（可选）
  - 最多 10 个标签
  - 每个标签最大 32 字符
  - 示例: `--tag dev --tag local`

- `--agent <AGENT>:<MODEL>`: 工具到模型的映射（必需，至少一个）
  - 可多次使用
  - 格式: `工具名:模型名`
  - 示例: `--agent claude-code:glm --agent codex:gpt-4`

- `--agent-list <FILE>`: 从文件读取映射关系（可选）
  - 文件格式: JSON/TOML
  - 示例: `--agent-list mappings.json`

**行为**:
1. 验证预设名称格式
2. 验证所有模型配置存在
3. 验证所有工具已注册
4. 创建预设并保存到 `~/.agentswitch/presets.toml`
5. 返回创建成功确认

**输出示例**:
```
✅ 预设创建成功
名称: development
描述: 开发环境配置
标签: dev, local
映射:
  - claude-code → glm
  - codex → gpt-4
创建时间: 2026-03-05 10:00:00 UTC
```

**错误情况**:
- 预设名称已存在: `错误: 预设名称 'development' 已存在`
- 模型配置不存在: `错误: 模型配置 'unknown-model' 不存在`
- 工具未注册: `错误: 工具 'unknown-tool' 未注册`
- 映射为空: `错误: 预设必须包含至少一个工具映射`

---

### 1.2 列出预设

**命令**:
```bash
asw preset list [OPTIONS]
```

**选项**:
- `--tag <TAG>`: 按标签筛选（可选）
  - 可多次使用
  - 示例: `--tag dev --tag local`

- `--format <FORMAT>`: 输出格式（可选，默认: `table`）
  - 支持: `table`, `json`, `toml`
  - 示例: `--format json`

**行为**:
1. 读取所有预设
2. 按标签筛选（如指定）
3. 按指定格式输出

**输出示例**（table 格式）:
```
可用的预设 (3):

NAME          DESCRIPTION         TAGS           UPDATED
development   开发环境配置        dev, local     2026-03-05 10:00
production    生产环境配置        prod, stable   2026-03-05 11:00
testing       测试环境配置        test           2026-03-05 12:00
```

**输出示例**（JSON 格式）:
```json
{
  "presets": [
    {
      "name": "development",
      "description": "开发环境配置",
      "tags": ["dev", "local"],
      "updated_at": "2026-03-05T10:00:00Z",
      "mappings": {
        "claude-code": "glm",
        "codex": "gpt-4"
      }
    }
  ],
  "total": 3
}
```

---

### 1.3 显示预设详情

**命令**:
```bash
asw preset show <NAME>
```

**参数**:
- `<NAME>`: 预设名称（必需）

**行为**:
1. 读取指定预设
2. 显示完整信息

**输出示例**:
```
预设: development
描述: 开发环境配置
标签: dev, local
创建时间: 2026-03-05 10:00:00 UTC
更新时间: 2026-03-05 10:00:00 UTC

工具映射:
  claude-code → glm
  codex → gpt-4
```

**错误情况**:
- 预设不存在: `错误: 预设 'unknown' 不存在`

---

### 1.4 应用预设

**命令**:
```bash
asw preset apply <NAME> [OPTIONS]
```

**参数**:
- `<NAME>`: 预设名称（必需）

**选项**:
- `--agent <AGENT>`: 仅应用到指定工具（可选）
  - 可多次使用
  - 示例: `--agent claude-code --agent codex`

- `--dry-run`: 模拟运行，不实际应用（可选）
  - 示例: `--dry-run`

- `--no-backup`: 跳过备份（不推荐，可选）
  - 示例: `--no-backup`

**行为**:
1. 验证预设存在
2. 验证所有模型配置存在
3. 验证所有工具已安装
4. 备份当前配置（除非指定 `--no-backup`）
5. 应用预设配置到工具
6. 显示应用结果

**输出示例**:
```
应用预设: development
备份配置... ✅
应用配置:
  claude-code → glm ... ✅
  codex → gpt-4 ... ✅

✅ 预设应用成功（2/2 工具）
耗时: 1.234 秒
```

**输出示例**（部分失败）:
```
应用预设: development
备份配置... ✅
应用配置:
  claude-code → glm ... ✅
  codex → gpt-4 ... ❌（配置文件损坏）

⚠️ 预设应用部分失败（1/2 工具）
失败工具: codex
建议: 检查工具配置或手动配置
```

**错误情况**:
- 预设不存在: `错误: 预设 'unknown' 不存在`
- 模型配置不存在: `错误: 模型配置 'unknown-model' 不存在`
- 备份失败: `错误: 备份失败，已中止操作`
- 所有工具失败: `错误: 所有工具应用失败，已回滚`

---

### 1.5 更新预设

**命令**:
```bash
asw preset update <NAME> [OPTIONS]
```

**参数**:
- `<NAME>`: 预设名称（必需）

**选项**:
- `--description <TEXT>`: 更新描述（可选）
- `--tag <TAG>`: 添加标签，可多次使用（可选）
- `--remove-tag <TAG>`: 删除标签，可多次使用（可选）
- `--agent <AGENT>:<MODEL>`: 更新或添加映射（可选）
- `--remove-agent <AGENT>`: 删除工具映射（可选）

**行为**:
1. 验证预设存在
2. 验证更新的模型配置存在
3. 更新预设
4. 保存到文件

**输出示例**:
```
✅ 预设更新成功
名称: development
变更:
  + 描述: 开发环境配置
  + 标签: local
  - 标签: test
  ~ 映射: codex → gpt-4
```

---

### 1.6 删除预设

**命令**:
```bash
asw preset delete <NAME> [OPTIONS]
```

**参数**:
- `<NAME>`: 预设名称（必需）

**选项**:
- `--force`: 跳过确认（可选）
  - 示例: `--force`

**行为**:
1. 请求确认（除非指定 `--force`）
2. 删除预设
3. 保存到文件

**输出示例**:
```
确认删除预设 'development'? [y/N]: y
✅ 预设删除成功
```

---

### 1.7 验证预设

**命令**:
```bash
asw preset validate <NAME>
```

**参数**:
- `<NAME>`: 预设名称（必需）

**行为**:
1. 验证预设存在
2. 验证所有模型配置存在
3. 验证所有工具已安装
4. 显示验证结果

**输出示例**:
```
验证预设: development
✅ 所有模型配置存在
✅ 所有工具已安装

✅ 预设验证通过
```

**输出示例**（有警告）:
```
验证预设: development
✅ 所有模型配置存在
⚠️  未安装工具: codex

⚠️  预设验证通过（有警告）
```

---

## 2. 批量操作命令（asw batch）

### 2.1 批量切换

**命令**:
```bash
asw batch switch <MODEL> [OPTIONS]
```

**参数**:
- `<MODEL>`: 目标模型名称（必需）

**选项**:
- `--agent <AGENT>`: 仅切换指定工具（可选）
  - 可多次使用
  - 示例: `--agent claude-code --agent codex`
  - 默认: 切换所有工具

- `--parallel <NUM>`: 并发数量（可选）
  - 默认: CPU 核心数
  - 示例: `--parallel 4`

- `--dry-run`: 模拟运行（可选）

**行为**:
1. 验证模型配置存在
2. 备份当前配置
3. 并发切换工具到目标模型
4. 显示切换结果

**输出示例**:
```
批量切换到模型: glm
备份配置... ✅
切换工具 (并发数: 4):
  claude-code → glm ... ✅
  codex → glm ... ✅
  qwen → glm ... ✅

✅ 批量切换成功（3/3 工具）
耗时: 2.345 秒
```

---

### 2.2 批量验证

**命令**:
```bash
asw batch validate [OPTIONS]
```

**选项**:
- `--agent <AGENT>`: 仅验证指定工具（可选）
  - 可多次使用

**行为**:
1. 验证所有工具的配置状态
2. 显示验证结果汇总

**输出示例**:
```
批量验证工具配置
验证结果:
  claude-code: ✅ 配置有效
  codex: ❌ 配置无效（API Key 缺失）
  qwen: ✅ 配置有效

验证完成: 2/3 有效
```

---

### 2.3 批量状态

**命令**:
```bash
asw batch status [OPTIONS]
```

**选项**:
- `--format <FORMAT>`: 输出格式（可选，默认: `table`）
  - 支持: `table`, `json`

**行为**:
1. 读取所有工具的当前配置
2. 显示状态汇总

**输出示例**:
```
工具配置状态:

AGENT          MODEL      STATUS      UPDATED
claude-code    glm        ✅ 有效      2026-03-05 10:00
codex          gpt-4      ❌ 无效     2026-03-04 15:30
qwen           qwen-max   ✅ 有效      2026-03-05 09:45

总计: 3 个工具（2 个有效，1 个无效）
```

---

## 3. 导入导出命令

### 3.1 导出预设

**命令**:
```bash
asw preset export [OPTIONS] <OUTPUT>
```

**参数**:
- `<OUTPUT>`: 输出文件路径（必需）
  - 示例: `presets.json`

**选项**:
- `--preset <NAME>`: 导出指定预设（可选）
  - 可多次使用
  - 默认: 导出所有预设

- `--include-models`: 包含模型配置（API Key 脱敏）（可选）
- `--include-active`: 包含当前活跃配置（可选）

**行为**:
1. 读取预设（所有或指定）
2. 可选地包含模型配置和活跃配置
3. 导出为 JSON 文件

**输出示例**:
```
导出预设到: presets.json
导出 3 个预设
✅ 导出成功
```

---

### 3.2 导入预设

**命令**:
```bash
asw preset import <INPUT> [OPTIONS]
```

**参数**:
- `<INPUT>`: 输入文件路径（必需）
  - 示例: `presets.json`

**选项**:
- `--strategy <STRATEGY>`: 导入策略（可选，默认: `merge`）
  - 支持: `merge`, `overwrite`
  - 示例: `--strategy merge`

- `--dry-run`: 模拟运行，显示预览（可选）

**行为**:
1. 读取并验证文件
2. 显示导入预览
3. 请求确认
4. 执行导入

**输出示例**:
```
导入预设从: presets.json
验证文件... ✅

导入预览:
  新增预设: 2 (production, testing)
  冲突预设: 1 (development)
  跳过预设: 0

确认导入? [y/N]: y

✅ 导入成功（2 个预设）
```

---

## 退出码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 参数错误 |
| 3 | 配置错误 |
| 4 | 网络/IO 错误 |

---

## 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `AGENTSWITCH_CONFIG_DIR` | 配置目录 | `~/.agentswitch` |
| `AGENTSWITCH_PRESETS_FILE` | 预设文件名 | `presets.toml` |
| `RAYON_NUM_THREADS` | 并发线程数 | CPU 核心数 |

---

## 配置文件

### 预设文件格式（~/.agentswitch/presets.toml）

```toml
version = "1.0.0"

[presets.development]
name = "development"
description = "开发环境配置"
tags = ["dev", "local"]
created_at = "2026-03-05T10:00:00Z"
updated_at = "2026-03-05T10:00:00Z"

[presets.development.mappings]
"claude-code" = "glm"
"codex" = "gpt-4"
```

---

## 示例工作流

### 工作流 1: 创建并应用预设

```bash
# 创建预设
asw preset create development \
  --description "开发环境配置" \
  --tag dev \
  --agent claude-code:glm \
  --agent codex:gpt-4

# 应用预设
asw preset apply development
```

### 工作流 2: 批量切换

```bash
# 切换所有工具到 glm
asw batch switch glm

# 切换指定工具
asw batch switch gpt-4 --agent claude-code --agent codex
```

### 工作流 3: 导出和导入

```bash
# 导出所有预设
asw preset export all-presets.json \
  --include-models \
  --include-active

# 导入到其他机器
asw preset import all-presets.json --strategy merge
```

---

**文档版本**: 1.0.0
**最后更新**: 2026-03-05
