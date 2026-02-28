# 数据模型: AgentSwitch 核心基础功能

**功能**: 核心基础功能
**日期**: 2026-02-27
**状态**: ✅ 完成

## 概述

本文档定义 AgentSwitch 核心功能所需的所有数据结构，包括配置模型、验证规则和序列化规范。

## 核心实体

### 1. ModelConfig（模型配置）

**描述**: 代表一个模型提供商的完整配置信息

**字段**:

| 字段名 | 类型 | 必填 | 描述 | 示例 |
|--------|------|------|------|------|
| `name` | `String` | ✅ | 唯一标识符，用于引用该配置 | `"glm"` |
| `base_url` | `String` | ✅ | API 基础地址 | `"https://open.bigmodel.cn/api/v1"` |
| `api_key` | `String` | ✅ | 认证密钥（敏感信息） | `"sk-abc123def456..."` |
| `model_id` | `String` | ✅ | 模型标识符 | `"glm-4"` |
| `extra_params` | `HashMap<String, Value>` | ❌ | 额外的键值对参数 | `{"temperature": 0.7}` |

**验证规则**:

1. **name**:
   - 不能为空字符串
   - 不能包含特殊字符（建议仅使用字母、数字、连字符）
   - 必须唯一（不能与已存在的模型名称重复）

2. **base_url**:
   - 必须是合法的 URL 格式
   - 必须包含 scheme（http 或 https）
   - 不能为空

3. **api_key**:
   - 不能为空
   - 长度建议 ≥ 8 字符

4. **model_id**:
   - 不能为空

**Rust 定义**:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelConfig {
    /// 模型配置的唯一标识符
    pub name: String,

    /// API 基础地址
    pub base_url: String,

    /// API 认证密钥（敏感信息）
    pub api_key: String,

    /// 模型标识符
    pub model_id: String,

    /// 额外参数（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, Value>>,
}

impl ModelConfig {
    /// 创建新的模型配置
    pub fn new(
        name: String,
        base_url: String,
        api_key: String,
        model_id: String,
    ) -> Self {
        Self {
            name,
            base_url,
            api_key,
            model_id,
            extra_params: None,
        }
    }

    /// 添加额外参数
    pub fn with_extra_params(mut self, params: HashMap<String, Value>) -> Self {
        self.extra_params = Some(params);
        self
    }
}
```

**TOML 示例**:

```toml
name = "glm"
base_url = "https://open.bigmodel.cn/api/v1"
api_key = "sk-abc123def456..."
model_id = "glm-4"

[extra_params]
temperature = 0.7
max_tokens = 4096
```

---

### 2. AppConfig（应用配置）

**描述**: 代表应用的完整配置状态，包含所有模型配置和活跃模型映射

**字段**:

| 字段名 | 类型 | 描述 | 示例 |
|--------|------|------|------|
| `models` | `Vec<ModelConfig>` | 所有模型配置的列表 | `[ModelConfig {...}, ...]` |
| `active_models` | `HashMap<String, String>` | 从 Agent 名称到模型名称的映射 | `{"claude-code": "glm"}` |

**字段说明**:

- **models**: 存储所有已添加的模型配置
- **active_models**: 记录每个 Agent 工具当前使用的模型名称
  - Key: Agent 工具名称（如 "claude-code", "codex"）
  - Value: 模型名称（对应 `ModelConfig.name`）

**Rust 定义**:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// 所有模型配置的列表
    #[serde(default)]
    pub models: Vec<ModelConfig>,

    /// 从 Agent 名称到模型名称的映射
    #[serde(default)]
    pub active_models: HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            models: Vec::new(),
            active_models: HashMap::new(),
        }
    }
}

impl AppConfig {
    /// 创建新的空配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加模型配置
    pub fn add_model(&mut self, model: ModelConfig) -> anyhow::Result<()> {
        // 检查名称是否已存在
        if self.models.iter().any(|m| m.name == model.name) {
            anyhow::bail!("模型名称 '{}' 已存在", model.name);
        }
        self.models.push(model);
        Ok(())
    }

    /// 获取模型配置
    pub fn get_model(&self, name: &str) -> Option<&ModelConfig> {
        self.models.iter().find(|m| m.name == name)
    }

    /// 删除模型配置
    pub fn remove_model(&mut self, name: &str) -> anyhow::Result<()> {
        let index = self.models
            .iter()
            .position(|m| m.name == name)
            .ok_or_else(|| anyhow::anyhow!("模型 '{}' 不存在", name))?;

        self.models.remove(index);

        // 清除活跃模型映射
        self.active_models.retain(|_, model_name| model_name != name);

        Ok(())
    }

    /// 编辑模型配置
    pub fn edit_model<F>(&mut self, name: &str, mut updater: F) -> anyhow::Result<()>
    where
        F: FnMut(&mut ModelConfig) -> anyhow::Result<()>,
    {
        let model = self.models
            .iter_mut()
            .find(|m| m.name == name)
            .ok_or_else(|| anyhow::anyhow!("模型 '{}' 不存在", name))?;

        updater(model)?;
        Ok(())
    }

    /// 设置 Agent 的活跃模型
    pub fn set_active_model(&mut self, agent: String, model: String) {
        self.active_models.insert(agent, model);
    }

    /// 获取 Agent 的活跃模型
    pub fn get_active_model(&self, agent: &str) -> Option<&ModelConfig> {
        let model_name = self.active_models.get(agent)?;
        self.get_model(model_name)
    }
}
```

**TOML 示例**:

```toml
# 模型配置列表
[[models]]
name = "glm"
base_url = "https://open.bigmodel.cn/api/v1"
api_key = "sk-abc123..."
model_id = "glm-4"

[[models]]
name = "minimax"
base_url = "https://api.minimax.chat/v1"
api_key = "sk-xyz789..."
model_id = "abab6.5s-chat"

# 活跃模型映射
[active_models]
"claude-code" = "glm"
"codex" = "minimax"
```

---

## 数据关系

```
AppConfig
  │
  ├─ models: Vec<ModelConfig>
  │     │
  │     ├─ ModelConfig (glm)
  │     ├─ ModelConfig (minimax)
  │     └─ ...
  │
  └─ active_models: HashMap<Agent, ModelName>
        │
        ├─ "claude-code" ──▶ "glm" ──▶ ModelConfig
        ├─ "codex" ────────▶ "minimax" ──▶ ModelConfig
        └─ ...
```

## 验证规则总结

### ModelConfig 验证

| 字段 | 规则 | 错误消息 |
|------|------|----------|
| `name` | 非空 | "模型名称不能为空" |
| `name` | 唯一性 | "模型名称 '{name}' 已存在" |
| `base_url` | 合法 URL | "URL 格式无效" |
| `api_key` | 非空 | "API Key 不能为空" |
| `model_id` | 非空 | "Model ID 不能为空" |

### AppConfig 验证

| 操作 | 规则 | 错误消息 |
|------|------|----------|
| `add_model` | 名称唯一 | "模型名称已存在" |
| `remove_model` | 名称存在 | "模型不存在" |
| `edit_model` | 名称存在 | "模型不存在" |

## 序列化/反序列化

### 往返一致性保证

**要求**: 序列化后再反序列化，必须得到与原始对象等价的结果

**测试**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = AppConfig {
            models: vec![
                ModelConfig::new(
                    "glm".to_string(),
                    "https://open.bigmodel.cn/api/v1".to_string(),
                    "sk-abc123".to_string(),
                    "glm-4".to_string(),
                ),
            ],
            active_models: HashMap::new(),
        };

        // 序列化
        let toml_str = toml::to_string_pretty(&original).unwrap();

        // 反序列化
        let restored: AppConfig = toml::from_str(&toml_str).unwrap();

        // 验证
        assert_eq!(original, restored);
    }
}
```

## 安全考虑

### API Key 处理

1. **存储**: 完整存储在配置文件中（明文）
2. **显示**: 掩码处理（仅显示前 4 位 + `****`）
3. **文件权限**: 0600（仅所有者可读写）

### 掩码函数

```rust
/// 掩码 API Key，仅显示前 4 个字符
pub fn mask_api_key(api_key: &str) -> String {
    if api_key.len() <= 4 {
        "****".to_string()
    } else {
        format!("{}****", &api_key[..4])
    }
}
```

## 扩展性

### 额外参数（extra_params）

**用途**: 支持未来扩展，无需修改核心数据结构

**示例**:
```toml
[extra_params]
temperature = 0.7
max_tokens = 4096
top_p = 0.9
```

**反序列化**: 使用 `serde_json::Value` 保留类型信息

---

## 文件格式

### 配置文件路径

- **路径**: `~/.agentswitch/config.toml`
- **权限**: 0600（仅所有者可读写）

### 默认配置

首次运行时自动创建：

```toml
# AgentSwitch 配置文件
# 此文件存储所有模型配置

# 模型配置列表
[[models]]
# 示例配置（已注释）
#name = "example"
#base_url = "https://api.example.com/v1"
#api_key = "your-api-key"
#model_id = "model-name"

# 活跃模型映射
[active_models]
# 格式: <agent-name> = <model-name>
# 示例:
# "claude-code" = "glm"
```

---

**数据模型状态**: ✅ 完成
**下一步**: 创建快速开始指南（quickstart.md）
