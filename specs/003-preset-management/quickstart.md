# 快速入门指南: 配置预设与批量管理

**功能**: Spec 003 - 配置预设与批量管理
**创建日期**: 2026-03-05
**版本**: 1.0.0

## 概述

本指南帮助开发者快速理解和使用 Spec 003 的核心功能。涵盖配置预设管理、批量操作和导入导出三个主要模块。

---

## 功能概览

### 1. 配置预设管理

保存和管理常用的工具配置组合，快速切换不同环境的配置。

**核心价值**:
- 一次配置，多次使用
- 为不同环境（开发、测试、生产）保存独立配置
- 减少重复操作，提升效率

### 2. 批量操作

一次性对多个工具执行相同操作，提升配置管理效率。

**核心价值**:
- 批量切换所有工具到同一模型
- 批量验证配置状态
- 并发执行，快速完成

### 3. 导入导出

在团队之间共享配置，或在多台机器之间迁移配置。

**核心价值**:
- 配置可分享，促进团队协作
- 跨机器迁移，保持一致性
- 版本控制，跟踪配置变化

---

## 快速开始

### 前置条件

1. AgentSwitch 已安装并配置
2. 至少有一个模型配置（如 `glm`, `gpt-4`）
3. 至少有一个工具已注册（如 `claude-code`, `codex`）

### 场景 1: 创建开发环境预设

**目标**: 为开发环境创建一个预设，使用 GLM 模型。

```bash
# 创建预设
asw preset create development \
  --description "开发环境配置，使用 GLM 模型" \
  --tag dev \
  --tag local \
  --agent claude-code:glm \
  --agent codex:glm

# 输出:
# ✅ 预设创建成功
# 名称: development
# 描述: 开发环境配置，使用 GLM 模型
# 标签: dev, local
# 映射:
#   - claude-code → glm
#   - codex → glm
# 创建时间: 2026-03-05 10:00:00 UTC
```

**应用预设**:
```bash
asw preset apply development

# 输出:
# 应用预设: development
# 备份配置... ✅
# 应用配置:
#   claude-code → glm ... ✅
#   codex → glm ... ✅
#
# ✅ 预设应用成功（2/2 工具）
# 耗时: 1.234 秒
```

---

### 场景 2: 批量切换所有工具

**目标**: 将所有工具快速切换到 GPT-4 模型。

```bash
asw batch switch gpt-4

# 输出:
# 批量切换到模型: gpt-4
# 备份配置... ✅
# 切换工具 (并发数: 4):
#   claude-code → gpt-4 ... ✅
#   codex → gpt-4 ... ✅
#   qwen → gpt-4 ... ✅
#
# ✅ 批量切换成功（3/3 工具）
# 耗时: 2.345 秒
```

**仅切换部分工具**:
```bash
asw batch switch gpt-4 --agent claude-code --agent codex
```

---

### 场景 3: 导出和导入配置

**目标**: 将预设配置导出并分享给团队成员。

**导出配置**:
```bash
# 导出所有预设（包含模型配置和当前活跃配置）
asw preset export team-presets.json \
  --include-models \
  --include-active

# 输出:
# 导出预设到: team-presets.json
# 导出 3 个预设
# ✅ 导出成功
```

**导入配置**（在其他机器上）:
```bash
# 预览导入内容
asw preset import team-presets.json --dry-run

# 执行导入（合并模式）
asw preset import team-presets.json --strategy merge

# 输出:
# 导入预设从: team-presets.json
# 验证文件... ✅
#
# 导入预览:
#   新增预设: 2 (production, testing)
#   冲突预设: 1 (development)
#   跳过预设: 0
#
# 确认导入? [y/N]: y
#
# ✅ 导入成功（2 个预设）
```

---

## 核心概念

### 预设（Preset）

预设是一个命名的工具配置组合，包含：
- **名称**: 唯一标识符（如 `development`）
- **描述**: 说明预设用途
- **标签**: 便于分类和筛选（如 `dev`, `prod`）
- **映射关系**: 工具到模型的映射（如 `claude-code → glm`）

**示例**:
```toml
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

### 批量操作

批量操作并发执行多个工具的配置操作，提升效率。

**关键特性**:
- 并发执行（使用 rayon）
- 错误隔离（单个工具失败不影响其他）
- 自动备份（失败可回滚）
- 结果汇总（显示成功和失败的工具）

### 导入导出

导入导出使用 JSON 格式，便于跨平台分享。

**安全措施**:
- API Key 自动脱敏（替换为 `***REDACTED***`）
- 文件格式验证
- 依赖检查（模型配置、工具安装）

---

## 常见用例

### 用例 1: 环境切换

**场景**: 开发、测试、生产环境使用不同模型配置。

```bash
# 开发环境
asw preset apply development  # 使用 GLM

# 测试环境
asw preset apply testing      # 使用 GPT-4

# 生产环境
asw preset apply production   # 使用 GPT-4 + 特殊配置
```

### 用例 2: 团队配置同步

**场景**: 新团队成员快速配置开发环境。

**步骤**:
1. 导出配置到版本控制（Git）
2. 新成员克隆仓库
3. 导入配置文件
4. 应用预设

```bash
# 导出配置
asw preset export config/presets.json --include-models

# 新成员导入
asw preset import config/presets.json --strategy merge
asw preset apply development
```

### 用例 3: 应急切换

**场景**: 某个模型服务不可用，需要快速切换所有工具。

```bash
# 快速切换所有工具到备用模型
asw batch switch backup-model --parallel 8
```

---

## 错误处理

### 预设应用失败

**场景**: 应用预设时部分工具失败。

```bash
asw preset apply production

# 输出:
# 应用预设: production
# 备份配置... ✅
# 应用配置:
#   claude-code → gpt-4 ... ✅
#   codex → gpt-4 ... ❌（配置文件损坏）
#
# ⚠️ 预设应用部分失败（1/2 工具）
# 失败工具: codex
# 建议: 检查工具配置或手动配置
#
# 已备份: ~/.agentswitch/backups/codex-20260305-100000.toml
```

**处理方式**:
1. 查看失败原因
2. 修复配置（或跳过该工具）
3. 使用备份恢复（如需要）

### 导入配置冲突

**场景**: 导入时预设名称冲突。

```bash
asw preset import presets.json --strategy overwrite

# 输出:
# 导入预设从: presets.json
# 验证文件... ✅
#
# 导入预览:
#   新增预设: 2 (testing, staging)
#   冲突预设: 1 (development)
#   跳过预设: 0
#
# 警告: 覆盖模式将替换现有预设 'development'
# 确认导入? [y/N]: y
```

**处理方式**:
1. 使用 `merge` 模式（保留现有预设）
2. 使用 `overwrite` 模式（替换现有预设）
3. 手动重命名冲突预设

---

## 最佳实践

### 1. 预设命名规范

推荐使用环境名称：
- `development` / `dev`: 开发环境
- `testing` / `test`: 测试环境
- `staging`: 预生产环境
- `production` / `prod`: 生产环境

### 2. 标签使用

使用标签分类预设：
- 按环境: `dev`, `test`, `prod`
- 按用途: `local`, `remote`, `team`
- 按稳定性: `stable`, `experimental`

```bash
asw preset create production \
  --tag prod --tag stable --tag team \
  --agent claude-code:gpt-4
```

### 3. 定期备份

定期导出预设到版本控制：
```bash
# 每周导出一次
asw preset export backups/presets-$(date +%Y%m%d).json \
  --include-models \
  --include-active
```

### 4. 验证预设

应用预设前验证：
```bash
asw preset validate production

# 输出:
# 验证预设: production
# ✅ 所有模型配置存在
# ✅ 所有工具已安装
#
# ✅ 预设验证通过
```

### 5. 使用批量操作提升效率

批量切换比逐个切换快 3-5 倍：
```bash
# 慢速方式
asw switch claude-code glm
asw switch codex glm
asw switch qwen glm

# 快速方式
asw batch switch glm
```

---

## 故障排除

### 问题 1: 预设文件损坏

**症状**: `asw preset list` 报错

**解决**:
```bash
# 查看备份文件
ls -la ~/.agentswitch/presets.backup*.toml

# 恢复备份
cp ~/.agentswitch/presets.backup.20260305-100000.toml \
   ~/.agentswitch/presets.toml
```

### 问题 2: 模型配置不存在

**症状**: `错误: 模型配置 'unknown-model' 不存在`

**解决**:
```bash
# 查看可用模型
asw model list

# 创建缺失的模型配置
asw model add glm \
  --api-key YOUR_API_KEY \
  --base-url https://api.example.com
```

### 问题 3: 工具未安装

**症状**: `错误: 工具 'unknown-tool' 未注册`

**解决**:
```bash
# 查看已注册工具
asw agent list

# 检查工具是否已安装
which claude-code
```

---

## 性能指标

根据 Spec 003 的成功标准：

- **创建并应用预设**（3 个工具）: < 30 秒
- **批量切换**（5 个工具）: < 10 秒
- **导出预设**（10 个预设）: < 5 秒
- **导入预设**（5 个预设）: < 15 秒

---

## 进阶使用

### 自定义并发数

根据机器性能调整并发数：
```bash
# 高性能机器
asw batch switch glm --parallel 16

# 低性能机器
asw batch switch glm --parallel 2
```

### 组合命令

使用 shell 脚本组合命令：
```bash
#!/bin/bash
# switch-to-prod.sh

echo "切换到生产环境..."

# 1. 验证预设
asw preset validate production

# 2. 应用预设
asw preset apply production

# 3. 验证配置
asw batch validate

echo "生产环境切换完成！"
```

---

## 相关文档

- [CLI 命令契约](contracts/CLI.md) - 完整的命令行接口文档
- [数据模型](data-model.md) - 数据结构和验证规则
- [技术研究](research.md) - 技术决策和实现细节
- [功能规格说明](spec.md) - 完整的功能需求

---

## 获取帮助

```bash
# 查看帮助
asw preset --help
asw preset create --help

# 查看版本
asw --version
```

---

**文档版本**: 1.0.0
**最后更新**: 2026-03-05
