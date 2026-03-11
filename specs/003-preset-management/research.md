# 技术研究文档: 配置预设与批量管理

**功能**: Spec 003 - 配置预设与批量管理
**创建日期**: 2026-03-05
**状态**: 完成

## 研究目标

基于功能规格说明中的需求，研究并确定以下技术决策：
1. 预设配置存储格式和位置
2. 导出配置文件格式
3. 批量操作的并发策略
4. 错误处理和回滚机制
5. 导入导出的安全和验证机制

## 研究发现与决策

### 1. 预设配置存储格式

**决策**: 使用 TOML 格式存储预设配置

**理由**:
- AgentSwitch 项目已在使用 TOML 格式（~/.agentswitch/config.toml）
- TOML 格式对人类友好，易于手动编辑
- Rust 生态系统有成熟的 toml 库支持
- 适合存储层次化的配置数据

**替代方案考虑**:
- JSON: 更通用，但可读性较差，不适合手动编辑
- YAML: 更灵活，但需要额外依赖，解析复杂度高
- SQLite: 过度设计，预设数量有限，不需要数据库

**存储位置**: `~/.agentswitch/presets.toml`

**理由**:
- 与现有配置文件在同一目录，便于管理
- 遵循 XDG Base Directory 规范（通过 dirs crate）
- 用户可直接访问和备份

**数据结构**:
```toml
# presets.toml 示例
[presets.development]
name = "development"
description = "开发环境配置"
tags = ["dev", "local"]
created_at = "2026-03-05T10:00:00Z"
updated_at = "2026-03-05T10:00:00Z"

[presets.development.mappings]
"claude-code" = "glm"
"codex" = "gpt-4"

[presets.production]
name = "production"
description = "生产环境配置"
tags = ["prod", "stable"]
created_at = "2026-03-05T11:00:00Z"
updated_at = "2026-03-05T11:00:00Z"

[presets.production.mappings]
"claude-code" = "gpt-4"
"codex" = "gpt-4"
```

---

### 2. 导出配置文件格式

**决策**: 使用 JSON 格式作为导出配置文件格式

**理由**:
- JSON 是广泛接受的数据交换格式
- 易于在不同平台和语言之间解析
- 支持嵌套数据结构
- 便于版本控制和差异对比
- 业界标准，用户期望使用 JSON

**替代方案考虑**:
- TOML: 导出场景不如 JSON 通用，跨语言支持较差
- YAML: 解析复杂，安全性问题（如执行任意代码）
- CBOR/MessagePack: 二进制格式，不适合人工阅读和版本控制

**导出文件结构**:
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

**安全考虑**:
- API Key 必须脱敏（替换为 "***REDACTED***"）
- 不导出敏感的认证信息
- 文件权限设置为 600（用户仅读写）

---

### 3. 批量操作的并发策略

**决策**: 使用 rayon 库实现并行迭代器进行并发操作

**理由**:
- rayon 是 Rust 生态系统的标准数据并行库
- 提供简单的并行迭代器 API（par_iter()）
- 自动工作窃取（work-stealing）调度器
- 零成本抽象，性能优秀
- 与现有的迭代器代码兼容

**替代方案考虑**:
- std::thread 手动管理线程: 复杂度高，容易出错
- Tokio async: 过度设计，批量操作是 CPU 密集型而非 IO 密集型
- 串行执行: 简单但性能差，无法满足 SC-002（10秒内切换5个工具）

**并发控制**:
- 默认并行度：rayon 自动选择（通常为 CPU 核心数）
- 最大并发数限制：通过环境变量 RAYON_NUM_THREADS 配置
- 错误隔离：每个工具的操作失败不影响其他工具

**实现示例**:
```rust
use rayon::prelude::*;

fn batch_switch_agents(
    agents: Vec<Box<dyn AgentAdapter>>,
    model_config: &ModelConfig
) -> Vec<(String, Result<(), Error>)> {
    agents.par_iter()
        .map(|adapter| {
            let name = adapter.agent_name();
            let result = adapter.apply(model_config);
            (name.to_string(), result)
        })
        .collect()
}
```

---

### 4. 错误处理和回滚机制

**决策**: 使用事务模式实现原子性操作和自动回滚

**理由**:
- 满足 FR-006（应用预设前备份）和 FR-007（失败时回滚）
- 确保批量操作的原子性和一致性
- 符合用户期望（操作要么全部成功，要么全部回滚）

**回滚策略**:
1. **预检查阶段**:
   - 验证所有模型配置存在
   - 检查所有工具已安装
   - 失败则直接返回，不执行任何操作

2. **备份阶段**:
   - 使用现有的 backup::Manager 备份每个工具的当前配置
   - 备份失败则中止操作

3. **应用阶段**:
   - 逐个应用预设配置
   - 失败时记录错误，继续处理其他工具（FR-022）

4. **回滚阶段**:
   - 如果应用失败率超过阈值（如 50%），执行全局回滚
   - 使用 backup::Manager.restore() 恢复备份
   - 汇总显示成功和失败的工具列表（FR-023）

**错误类型**:
```rust
pub enum PresetError {
    /// 预设不存在
    PresetNotFound(String),
    /// 模型配置不存在
    ModelConfigNotFound(String),
    /// 工具未安装
    AgentNotInstalled(String),
    /// 备份失败
    BackupFailed(String),
    /// 应用失败（包含工具名称和原因）
    ApplyFailed { agent: String, reason: String },
    /// 回滚失败
    RollbackFailed(String),
    /// 验证失败
    ValidationFailed(String),
}
```

---

### 5. 导入导出的安全和验证机制

**决策**: 多层验证确保导入配置的安全性

**验证层级**:

1. **格式验证**（JSON Schema）:
   - 验证 JSON 格式正确性
   - 检查必需字段（version, exported_at, presets）
   - 验证数据类型（字符串、数组、对象）

2. **语义验证**:
   - 验证版本兼容性（检查 version 字段）
   - 验证预设名称唯一性
   - 验证映射关系（工具名 → 模型名）

3. **依赖验证**:
   - 检查引用的模型配置是否存在
   - 检查引用的工具是否已安装
   - 缺失依赖时询问用户是否继续（跳过依赖项）

4. **安全验证**:
   - 验证文件路径（防止路径遍历攻击）
   - 验证文件权限（必须是 600 或更严格）
   - 检测可疑内容（如过大的文件、异常的嵌套深度）
   - 不执行任意代码或命令

**导入策略**:

1. **合并模式**（Merge）:
   - 保留现有预设
   - 仅添加不冲突的预设
   - 同名预设跳过并警告

2. **覆盖模式**（Overwrite）:
   - 替换现有预设
   - 删除不在导入文件中的预设（可选）
   - 用户明确确认后执行

**安全措施**:
- 导入前显示差异预览（diff）
- 要求用户确认导入操作
- 导入失败时回滚到导入前状态
- 记录所有导入操作的日志

---

## 技术依赖总结

### 新增依赖

```toml
# Cargo.toml 新增依赖

# 并发处理
rayon = "1.10"

# JSON 处理（serde_json 已存在，无需新增）
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"

# TOML 处理（toml 已存在，无需新增）
# toml = "0.8"
```

### 现有依赖复用

- `serde` / `serde_json` / `toml`: 序列化和反序列化
- `anyhow`: 错误处理
- `dirs`: 路径操作
- `colored`: 终端输出
- `clap`: CLI 命令行解析

---

## 性能考虑

### 目标性能（来自成功标准）

- **SC-001**: 30秒内创建并应用预设（3个工具）
- **SC-002**: 10秒内批量切换5个工具
- **SC-003**: 5秒内导出10个预设
- **SC-004**: 15秒内导入并应用5个预设

### 性能优化策略

1. **并发处理**: 使用 rayon 并行执行批量操作
2. **延迟加载**: 仅在需要时读取完整配置
3. **缓存策略**: 缓存已加载的预设和模型配置
4. **增量操作**: 避免不必要的全量扫描

### 性能测试计划

- 基准测试（benchmark）：预设创建、应用、导入、导出
- 压力测试：100+ 个预设的性能表现
- 内存分析：检查内存泄漏和不必要的克隆

---

## 兼容性保证

### 向后兼容性

- 预设文件格式变更时提供迁移脚本
- 保留旧版预设文件的导入支持
- 版本号遵循语义化版本（SemVer）

### 跨平台兼容性

- 使用 `dirs` crate 处理路径差异
- 文件权限设置在 Windows 上使用 ACL
- JSON/TOML 格式平台无关

---

## 未解决问题

无（所有技术决策已明确）

---

## 附录：相关代码示例

### 预设数据结构

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// 配置预设
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// 预设名称（唯一标识符）
    pub name: String,
    /// 预设描述
    pub description: String,
    /// 预设标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 工具到模型的映射关系
    pub mappings: HashMap<String, String>,
}

/// 预设集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetCollection {
    /// 版本号
    pub version: String,
    /// 所有预设
    pub presets: HashMap<String, Preset>,
}

/// 导出包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPackage {
    /// 版本号
    pub version: String,
    /// 导出时间
    pub exported_at: DateTime<Utc>,
    /// 预设集合
    pub presets: Vec<Preset>,
    /// 模型配置集合（API Key 脱敏）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_configs: Option<HashMap<String, ModelConfig>>,
    /// 当前活跃配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_config: Option<HashMap<String, String>>,
}
```

### 并发批量操作

```rust
use rayon::prelude::*;
use crate::agents::AgentAdapter;
use crate::config::ModelConfig;
use anyhow::Result;

/// 批量切换工具到指定模型
pub fn batch_switch_agents(
    adapters: Vec<Box<dyn AgentAdapter>>,
    model_config: &ModelConfig,
) -> Vec<(String, Result<()>)> {
    adapters.par_iter()
        .map(|adapter| {
            let name = adapter.agent_name().to_string();
            let result = adapter.apply(model_config)
                .with_context(|| format!("切换工具 {} 失败", name));
            (name, result)
        })
        .collect()
}
```

---

**文档版本**: 1.0.0
**最后更新**: 2026-03-05
