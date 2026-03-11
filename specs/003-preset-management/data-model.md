# 数据模型: 配置预设与批量管理

**功能**: Spec 003 - 配置预设与批量管理
**创建日期**: 2026-03-05
**版本**: 1.0.0

## 核心实体

### 1. Preset（配置预设）

**描述**: 代表一个命名的工具配置组合，包含多个工具到模型的映射关系。

**字段**:

| 字段名 | 类型 | 可空 | 说明 | 验证规则 |
|--------|------|------|------|----------|
| `name` | `String` | 否 | 预设名称（唯一标识符） | 长度 1-64 字符，仅含字母、数字、连字符、下划线 |
| `description` | `String` | 否 | 预设描述 | 长度 0-512 字符 |
| `tags` | `Vec<String>` | 是 | 预设标签 | 每个标签长度 1-32 字符，最多 10 个标签 |
| `created_at` | `DateTime<Utc>` | 否 | 创建时间 | ISO 8601 格式，创建时自动设置 |
| `updated_at` | `DateTime<Utc>` | 否 | 更新时间 | ISO 8601 格式，更新时自动更新 |
| `mappings` | `HashMap<String, String>` | 否 | 工具到模型的映射 | 键为工具名，值为模型名，至少包含 1 个映射 |

**关系**:
- 引用 `ModelConfig`（通过 mappings 中的模型名）
- 引用 `AgentAdapter`（通过 mappings 中的工具名）

**状态转换**:
```
[创建] → [活跃] → [更新] → [活跃]
[活跃] → [删除] → [不存在]
```

**验证规则**:
1. 预设名称在同一用户环境下必须唯一（FR 约束）
2. 创建时所有引用的模型配置必须存在（FR-003）
3. 映射关系中的工具名必须是已注册的工具
4. 更新时间不能早于创建时间

**存储格式**:
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

---

### 2. PresetCollection（预设集合）

**描述**: 管理所有配置预设的容器。

**字段**:

| 字段名 | 类型 | 可空 | 说明 |
|--------|------|------|------|
| `version` | `String` | 否 | 集合版本号（遵循 SemVer） |
| `presets` | `HashMap<String, Preset>` | 否 | 所有预设，键为预设名称 |

**关系**:
- 包含多个 `Preset`

**操作**:
- `add_preset()`: 添加预设（验证名称唯一性）
- `get_preset()`: 获取预设
- `update_preset()`: 更新预设
- `remove_preset()`: 删除预设
- `list_presets()`: 列出所有预设
- `find_by_tag()`: 按标签查找预设

**存储格式**:
```toml
version = "1.0.0"

[presets.development]
name = "development"
# ... 其他字段

[presets.production]
name = "production"
# ... 其他字段
```

---

### 3. ExportPackage（导出包）

**描述**: 代表导出的配置文件，包含预设、模型配置和当前活跃配置。

**字段**:

| 字段名 | 类型 | 可空 | 说明 | 验证规则 |
|--------|------|------|------|----------|
| `version` | `String` | 否 | 导出格式版本 | 遵循 SemVer |
| `exported_at` | `DateTime<Utc>` | 否 | 导出时间 | ISO 8601 格式 |
| `presets` | `Vec<Preset>` | 否 | 预设集合 | 至少包含 1 个预设 |
| `model_configs` | `HashMap<String, ModelConfig>` | 是 | 模型配置集合 | API Key 必须脱敏 |
| `active_config` | `HashMap<String, String>` | 是 | 当前活跃配置 | 键为工具名，值为模型名 |

**关系**:
- 包含多个 `Preset`
- 引用 `ModelConfig`（脱敏后）
- 反映当前系统的活跃配置

**安全规则**:
1. API Key 必须脱敏（替换为 "***REDACTED***"）
2. 敏感字段不得导出（如临时令牌）
3. 导出文件权限必须设置为 600

**存储格式**（JSON）:
```json
{
  "version": "1.0.0",
  "exported_at": "2026-03-05T10:00:00Z",
  "presets": [
    {
      "name": "development",
      "description": "开发环境配置",
      "tags": ["dev", "local"],
      "created_at": "2026-03-05T10:00:00Z",
      "updated_at": "2026-03-05T10:00:00Z",
      "mappings": {
        "claude-code": "glm",
        "codex": "gpt-4"
      }
    }
  ],
  "model_configs": {
    "glm": {
      "api_key": "***REDACTED***",
      "base_url": "https://api.example.com",
      "model": "glm-4"
    }
  },
  "active_config": {
    "claude-code": "glm",
    "codex": "gpt-4"
  }
}
```

---

### 4. ImportStrategy（导入策略）

**描述**: 定义导入配置时的行为策略。

**枚举值**:

| 值 | 说明 | 行为 |
|----|------|------|
| `Merge` | 合并模式 | 保留现有预设，仅添加不冲突的预设，同名预设跳过 |
| `Overwrite` | 覆盖模式 | 替换现有预设，删除不在导入文件中的预设（可选） |

**验证规则**:
1. 覆盖模式需要用户明确确认
2. 导入前显示差异预览
3. 导入失败时不影响现有配置

---

### 5. BatchOperationResult（批量操作结果）

**描述**: 批量操作的执行结果汇总。

**字段**:

| 字段名 | 类型 | 可空 | 说明 |
|--------|------|------|------|
| `total` | `usize` | 否 | 总操作数 |
| `succeeded` | `usize` | 否 | 成功数 |
| `failed` | `usize` | 否 | 失败数 |
| `results` | `Vec<ToolOperationResult>` | 否 | 每个工具的详细结果 |
| `duration_ms` | `u128` | 否 | 操作耗时（毫秒） |

**关系**:
- 包含多个 `ToolOperationResult`

---

### 6. ToolOperationResult（工具操作结果）

**描述**: 单个工具的操作执行结果。

**字段**:

| 字段名 | 类型 | 可空 | 说明 |
|--------|------|------|------|
| `agent_name` | `String` | 否 | 工具名称 |
| `success` | `bool` | 否 | 是否成功 |
| `error_message` | `Option<String>` | 是 | 错误信息（失败时） |
| `backup_path` | `Option<String>` | 是 | 备份文件路径（如有） |

---

## 数据关系图

```
┌─────────────────┐
│  PresetCollection│
│  (version)      │
│  (presets)      │
└────────┬────────┘
         │ 1
         │ contains
         │ *
┌────────▼────────┐     ┌─────────────────┐
│     Preset      │────>│  ModelConfig    │
│  (name)         │     │  (model_name)   │
│  (description)  │     │  (api_key)      │
│  (tags)         │     └─────────────────┘
│  (mappings)     │              ▲
│  (created_at)   │              │ references
│  (updated_at)   │              │
└────────┬────────┘              │
         │                       │
         │ references            │
         │                       │
┌────────▼────────┐     ┌────────┴─────────┐
│  AgentAdapter   │     │  ExportPackage   │
│  (agent_name)   │     │  (version)       │
└─────────────────┘     │  (exported_at)   │
                        │  (presets)       │
                        │  (model_configs) │
                        │  (active_config) │
                        └──────────────────┘
```

---

## 状态机

### Preset 生命周期

```
         ┌──────────┐
         │  创建中  │
         └────┬─────┘
              │ create()
              ▼
         ┌──────────┐
         │   活跃   │◄─────────────────┐
         └────┬─────┘                   │
              │ update()                │ restore()
              ▼                         │
         ┌──────────┐                   │
         │  更新中  │                   │
         └────┬─────┘                   │
              │                         │
              ▼                         │
         ┌──────────┐ delete()          │
         │  已删除  │───────────────────┘
         └──────────┘
```

### 批量操作状态机

```
         ┌──────────┐
         │  预检查  │
         └────┬─────┘
              │ 验证模型和工具
              ▼
         ┌──────────┐
         │   备份   │◄───┐
         └────┬─────┘    │
              │          │ rollback()
              ▼          │
         ┌──────────┐    │
         │   应用   │────┘
         └────┬─────┘
              │
              ▼
         ┌──────────┐
         │  完成    │
         └──────────┘
```

---

## 数据验证规则

### Preset 验证

```rust
impl Preset {
    /// 验证预设的有效性
    pub fn validate(&self, available_models: &HashSet<String>) -> anyhow::Result<()> {
        // 1. 验证名称格式
        if !is_valid_preset_name(&self.name) {
            bail!("预设名称格式无效: {}", self.name);
        }

        // 2. 验证描述长度
        if self.description.len() > 512 {
            bail!("预设描述过长（最多 512 字符）");
        }

        // 3. 验证标签
        if self.tags.len() > 10 {
            bail!("标签数量过多（最多 10 个）");
        }
        for tag in &self.tags {
            if tag.len() > 32 {
                bail!("标签过长: {}（最多 32 字符）", tag);
            }
        }

        // 4. 验证映射关系
        if self.mappings.is_empty() {
            bail!("预设必须包含至少一个工具映射");
        }

        // 5. 验证引用的模型存在
        for model_name in self.mappings.values() {
            if !available_models.contains(model_name) {
                bail!("模型配置不存在: {}", model_name);
            }
        }

        // 6. 验证时间戳
        if self.updated_at < self.created_at {
            bail!("更新时间不能早于创建时间");
        }

        Ok(())
    }
}
```

### ExportPackage 验证

```rust
impl ExportPackage {
    /// 验证导出包的有效性
    pub fn validate(&self) -> anyhow::Result<()> {
        // 1. 验证版本
        if !is_valid_version(&self.version) {
            bail!("无效的版本号: {}", self.version);
        }

        // 2. 验证预设数量
        if self.presets.is_empty() {
            bail!("导出包必须包含至少一个预设");
        }

        // 3. 验证每个预设
        for preset in &self.presets {
            preset.validate(&HashSet::new())?;
        }

        // 4. 验证模型配置（如有）
        if let Some(configs) = &self.model_configs {
            for (name, config) in configs {
                // 验证 API Key 已脱敏
                if !config.api_key.contains("REDACTED") {
                    bail!("模型配置 {} 的 API Key 未脱敏", name);
                }
            }
        }

        Ok(())
    }
}
```

---

## 数据持久化

### 预设存储文件

**文件路径**: `~/.agentswitch/presets.toml`

**格式**: TOML

**原子写入策略**:
1. 写入临时文件 `presets.tmp.toml`
2. 同步到磁盘（fsync）
3. 原子重命名 `presets.tmp.toml` → `presets.toml`

**备份策略**:
- 每次修改前自动备份到 `presets.backup.toml`
- 保留最近 10 个备份版本

### 导出文件

**文件路径**: 用户指定路径（默认为当前目录）

**格式**: JSON

**命名建议**: `agentswitch-presets-{timestamp}.json`

---

## 错误处理数据结构

```rust
/// 预设相关错误
#[derive(Debug, thiserror::Error)]
pub enum PresetError {
    #[error("预设不存在: {0}")]
    PresetNotFound(String),

    #[error("预设名称已存在: {0}")]
    PresetAlreadyExists(String),

    #[error("模型配置不存在: {0}")]
    ModelConfigNotFound(String),

    #[error("工具未安装: {0}")]
    AgentNotInstalled(String),

    #[error("备份失败: {0}")]
    BackupFailed(String),

    #[error("应用失败: 工具={agent}, 原因={reason}")]
    ApplyFailed { agent: String, reason: String },

    #[error("回滚失败: {0}")]
    RollbackFailed(String),

    #[error("验证失败: {0}")]
    ValidationFailed(String),

    #[error("导入失败: {0}")]
    ImportFailed(String),

    #[error("导出失败: {0}")]
    ExportFailed(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] toml::ser::Error),
}
```

---

**文档版本**: 1.0.0
**最后更新**: 2026-03-05
